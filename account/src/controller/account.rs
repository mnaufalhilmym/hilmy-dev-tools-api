use std::str::FromStr;

use argon2::{PasswordHasher, PasswordVerifier};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rand::Rng;
use rdkafka::{message::ToBytes, producer::FutureRecord};
use redis::Commands;
use tonic::{Request, Response, Result, Status};
use uuid::Uuid;

use crate::{
    dto::{
        account_change_email::AccountChangeEmail, account_reset_password::AccountResetPassword,
        account_sign_up::AccountSignUp, jwt_claims,
    },
    helper, model,
    proto::{self, account::AccountService},
    schema,
};

use super::AccountController;

#[tonic::async_trait]
impl AccountService for AccountController {
    async fn sign_up(
        &self,
        req: Request<proto::account::SignUpReq>,
    ) -> Result<Response<proto::account::OpRes>> {
        let db_conn =
            &mut tools_lib_db::pg::connection::get_connection(&self.app_mode, &self.db_pool)
                .map_err(|e| Status::internal(e.to_string()))?;
        let redis_conn =
            &mut tools_lib_db::redis::connection::get_connection(&self.app_mode, &self.redis_pool)
                .map_err(|e| Status::internal(e.to_string()))?;
        let (argon2, salt) = helper::argon2::new(&self.argon2_hash_secret.as_bytes());
        let kafka_producer = &self.kafka_producer;

        // Check if email has been registered
        if schema::account::table
            .filter(schema::account::email.eq(&req.get_ref().email))
            .first::<model::Account>(db_conn)
            .is_ok()
        {
            return Err(Status::aborted("The email has been registered."));
        }

        // Hash the password for security
        let hashed_password = argon2
            .hash_password(req.get_ref().password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        // Create random 6 digit verification code
        let verification_code = rand::thread_rng().gen_range(100000..=999999).to_string();

        // Create data key
        let data_key = format!("sign_up-{}", &req.get_ref().email);

        // Serialize data
        let data = serde_json::to_string(&AccountSignUp {
            email: req.get_ref().email.to_owned(),
            password: hashed_password,
            verify_code: verification_code.to_owned(),
        })
        .map_err(|e| Status::internal(e.to_string()))?;

        // Temporarily save to Redis
        redis_conn
            .set_ex(&data_key, &data, 10 * 60)
            .map_err(|e| Status::internal(e.to_string()))?;

        // Send to mailer service
        kafka_producer
            .send_result(
                FutureRecord::to("mailer").key(&data_key).payload(
                    &serde_json::to_string(&[&tools_mailer::contract::MailReq {
                        to: req.get_ref().email.to_owned(),
                        subject: format!("Sign Up Verification Code - {verification_code}"),
                        body: format!("Your sign up verification code is {verification_code}"),
                    }])
                    .map_err(|e| Status::internal(e.to_string()))?,
                ),
            )
            .map_err(|e| Status::internal(e.0.to_string()))?
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .map_err(|e| Status::internal(e.0.to_string()))?;

        Ok(Response::new(proto::account::OpRes { is_success: true }))
    }

    async fn verify_sign_up(
        &self,
        req: Request<proto::account::VerifySignUpReq>,
    ) -> Result<Response<proto::account::OpRes>> {
        let db_conn =
            &mut tools_lib_db::pg::connection::get_connection(&self.app_mode, &self.db_pool)
                .map_err(|e| Status::internal(e.to_string()))?;
        let redis_conn =
            &mut tools_lib_db::redis::connection::get_connection(&self.app_mode, &self.redis_pool)
                .map_err(|e| Status::internal(e.to_string()))?;
        let kafka_producer = &self.kafka_producer;

        // Create data key
        let data_key = format!("sign_up-{}", &req.get_ref().email);

        // Get temporary data from Redis
        let account_sign_up: String = redis_conn
            .get(&data_key)
            .map_err(|e| Status::internal(e.to_string()))?;
        let account_sign_up: AccountSignUp =
            serde_json::from_str(&account_sign_up).map_err(|e| Status::internal(e.to_string()))?;

        // Check if verification code match
        if req.get_ref().verify_code != account_sign_up.verify_code {
            return Err(Status::aborted(
                "Failed to sign up because of wrong verification code",
            ));
        }

        // Remove existing data from Redis if verification code match
        redis_conn.del::<_, String>(&data_key).ok();

        // Insert new account to database
        diesel::insert_into(schema::account::table)
            .values((
                schema::account::email.eq(account_sign_up.email),
                schema::account::password.eq(account_sign_up.password),
            ))
            .execute(db_conn)
            .map_err(|e| Status::internal(e.to_string()))?;

        // Send to mailer service
        kafka_producer
            .send_result(
                FutureRecord::to("mailer").key(&data_key).payload(
                    &serde_json::to_string(&[&tools_mailer::contract::MailReq {
                        to: req.get_ref().email.to_owned(),
                        subject: "Sign Up Verification Complete".to_string(),
                        body: "Your account is now verified.".to_string(),
                    }])
                    .map_err(|e| Status::internal(e.to_string()))?,
                ),
            )
            .map_err(|e| Status::internal(e.0.to_string()))?
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .map_err(|e| Status::internal(e.0.to_string()))?;

        Ok(Response::new(proto::account::OpRes { is_success: true }))
    }

    async fn sign_in(
        &self,
        req: Request<proto::account::SignInReq>,
    ) -> Result<Response<proto::account::SignInRes>> {
        let db_conn =
            &mut tools_lib_db::pg::connection::get_connection(&self.app_mode, &self.db_pool)
                .map_err(|e| Status::internal(e.to_string()))?;
        let argon2 = helper::argon2::new_argon2(&self.argon2_hash_secret.as_bytes());
        let jwt_secret = &self.jwt_secret;

        // Select user account from database
        let account_data = schema::account::table
            .filter(schema::account::email.eq(&req.get_ref().email))
            .select((schema::account::id, schema::account::password))
            .first::<(Uuid, String)>(db_conn)
            .map_err(|e| Status::internal(e.to_string()))?;

        // Get password hash from account data
        let account_password_hash = argon2::PasswordHash::new(&account_data.1)
            .map_err(|e| Status::internal(e.to_string()))?;

        // Verify user inputted credential
        argon2
            .verify_password(&req.get_ref().password.as_bytes(), &account_password_hash)
            .map_err(|_| Status::aborted("Failed to sign in because of wrong password"))?;

        // Create JWT
        let claims = jwt_claims::Claims {
            id: account_data.0.to_string(),
        };
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(jwt_secret.as_bytes()),
        )
        .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(proto::account::SignInRes { token }))
    }

    async fn change_email(
        &self,
        req: Request<proto::account::ChangeEmailReq>,
    ) -> Result<Response<proto::account::OpRes>> {
        let db_conn =
            &mut tools_lib_db::pg::connection::get_connection(&self.app_mode, &self.db_pool)
                .map_err(|e| Status::internal(e.to_string()))?;
        let redis_conn =
            &mut tools_lib_db::redis::connection::get_connection(&self.app_mode, &self.redis_pool)
                .map_err(|e| Status::internal(e.to_string()))?;
        let kafka_producer = &self.kafka_producer;
        let jwt_secret = &self.jwt_secret;

        // Decode JWT Token
        let mut validation = jsonwebtoken::Validation::default();
        validation.required_spec_claims.remove("exp");
        let account_id = jsonwebtoken::decode::<jwt_claims::Claims>(
            &req.get_ref().token,
            &jsonwebtoken::DecodingKey::from_secret(jwt_secret.to_bytes()),
            &validation,
        )
        .map_err(|e| Status::internal(e.to_string()))?
        .claims
        .id;

        // Get account data
        let account_id = Uuid::from_str(&account_id).map_err(|e| Status::aborted(e.to_string()))?;
        let account_email = schema::account::table
            .find(&account_id)
            .select(schema::account::email)
            .first::<String>(db_conn)
            .map_err(|e| Status::internal(e.to_string()))?;

        // Create random 6 digit verification code
        let verification_code = rand::thread_rng().gen_range(100000..=999999).to_string();

        // Create data key
        let data_key = format!("change_email-{}", &req.get_ref().new_email);

        // Serialize data
        let data = serde_json::to_string(&AccountChangeEmail {
            new_email: req.get_ref().new_email.to_owned(),
            old_email: account_email,
            verify_code: verification_code.to_owned(),
        })
        .map_err(|e| Status::internal(e.to_string()))?;

        // Temporarily save to Redis
        redis_conn
            .set_ex(&data_key, &data, 10 * 60)
            .map_err(|e| Status::internal(e.to_string()))?;

        // Send to mailer service
        kafka_producer
            .send_result(
                FutureRecord::to("mailer").key(&data_key).payload(
                    &serde_json::to_string(&[&tools_mailer::contract::MailReq {
                        to: req.get_ref().new_email.to_owned(),
                        subject: format!("Change Email Verification Code - {verification_code}"),
                        body: format!("Your change email verification code is {verification_code}"),
                    }])
                    .map_err(|e| Status::internal(e.to_string()))?,
                ),
            )
            .map_err(|e| Status::internal(e.0.to_string()))?
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .map_err(|e| Status::internal(e.0.to_string()))?;

        Ok(Response::new(proto::account::OpRes { is_success: true }))
    }

    async fn verify_change_email(
        &self,
        req: Request<proto::account::VerifyChangeEmailReq>,
    ) -> Result<Response<proto::account::OpRes>> {
        let db_conn =
            &mut tools_lib_db::pg::connection::get_connection(&self.app_mode, &self.db_pool)
                .map_err(|e| Status::internal(e.to_string()))?;
        let redis_conn =
            &mut tools_lib_db::redis::connection::get_connection(&self.app_mode, &self.redis_pool)
                .map_err(|e| Status::internal(e.to_string()))?;
        let kafka_producer = &self.kafka_producer;

        // Create data key
        let data_key = format!("change_email-{}", &req.get_ref().new_email);

        // Get temporary data from Redis
        let account_change_email: String = redis_conn
            .get(&data_key)
            .map_err(|e| Status::internal(e.to_string()))?;
        let account_change_email: AccountChangeEmail = serde_json::from_str(&account_change_email)
            .map_err(|e| Status::internal(e.to_string()))?;

        // Check if verification code match
        if req.get_ref().verify_code != account_change_email.verify_code {
            return Err(Status::aborted(
                "Failed to change email because of wrong verification code",
            ));
        }

        // Remove existing data from Redis if verification code match
        redis_conn.del::<_, String>(&data_key).ok();

        // Update account email to database
        diesel::update(
            schema::account::table
                .filter(schema::account::email.eq(&account_change_email.old_email)),
        )
        .set((
            schema::account::email.eq(&account_change_email.new_email),
            schema::account::updated_at.eq(&diesel::dsl::now),
        ))
        .execute(db_conn)
        .map_err(|e| Status::internal(e.to_string()))?;

        // Send email notification change email
        kafka_producer
            .send_result(
                FutureRecord::to("mailer").key(&data_key).payload(
                    &serde_json::to_string(&[
                        &tools_mailer::contract::MailReq {
                            to: account_change_email.old_email.to_owned(),
                            subject: "Change Email Verification Complete".to_string(),
                            body: format!(
                                "Your account email is now changed to {}.",
                                &account_change_email.new_email
                            ),
                        },
                        &tools_mailer::contract::MailReq {
                            to: account_change_email.new_email.to_owned(),
                            subject: "Change Email Verification Complete".to_string(),
                            body: format!(
                                "Your account email is now changed to {}.",
                                &account_change_email.new_email
                            ),
                        },
                    ])
                    .map_err(|e| Status::internal(e.to_string()))?,
                ),
            )
            .map_err(|e| Status::internal(e.0.to_string()))?
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .map_err(|e| Status::internal(e.0.to_string()))?;

        Ok(Response::new(proto::account::OpRes { is_success: true }))
    }

    async fn change_password(
        &self,
        req: Request<proto::account::ChangePasswordReq>,
    ) -> Result<Response<proto::account::OpRes>> {
        let db_conn =
            &mut tools_lib_db::pg::connection::get_connection(&self.app_mode, &self.db_pool)
                .map_err(|e| Status::internal(e.to_string()))?;
        let (argon2, salt) = helper::argon2::new(&self.argon2_hash_secret.as_bytes());
        let kafka_producer = &self.kafka_producer;
        let jwt_secret = &self.jwt_secret;

        // Decode JWT Token
        let mut validation = jsonwebtoken::Validation::default();
        validation.required_spec_claims.remove("exp");
        let account_id = jsonwebtoken::decode::<jwt_claims::Claims>(
            &req.get_ref().token,
            &jsonwebtoken::DecodingKey::from_secret(jwt_secret.to_bytes()),
            &validation,
        )
        .map_err(|e| Status::internal(e.to_string()))?
        .claims
        .id;

        // Get account data
        let account_id = Uuid::from_str(&account_id).map_err(|e| Status::aborted(e.to_string()))?;
        let account_data = schema::account::table
            .find(account_id)
            .select((schema::account::email, schema::account::password))
            .first::<(String, String)>(db_conn)
            .map_err(|e| Status::internal(e.to_string()))?;

        // Get password hash from account data
        let account_password_hash = argon2::PasswordHash::new(&account_data.1)
            .map_err(|e| Status::internal(e.to_string()))?;

        // Check if old password is match
        argon2
            .verify_password(
                &req.get_ref().old_password.as_bytes(),
                &account_password_hash,
            )
            .map_err(|_| {
                Status::aborted("Failed to change password because of old password doesn't match")
            })?;

        // Hash the password for security
        let hashed_password = argon2
            .hash_password(req.get_ref().new_password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        // Update account password to database
        diesel::update(schema::account::table.find(account_id))
            .set((
                schema::account::password.eq(&hashed_password),
                schema::account::updated_at.eq(&diesel::dsl::now),
            ))
            .execute(db_conn)
            .map_err(|e| Status::internal(e.to_string()))?;

        // Send email notification change email
        kafka_producer
            .send_result(
                FutureRecord::to("mailer")
                    .key(&format!("change_password-{}", &account_data.0))
                    .payload(
                        &serde_json::to_string(&[&tools_mailer::contract::MailReq {
                            to: account_data.0.to_owned(),
                            subject: "Change Password Success".to_string(),
                            body: "Your account password is now changed".to_string(),
                        }])
                        .map_err(|e| Status::internal(e.to_string()))?,
                    ),
            )
            .map_err(|e| Status::internal(e.0.to_string()))?
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .map_err(|e| Status::internal(e.0.to_string()))?;

        Ok(Response::new(proto::account::OpRes { is_success: true }))
    }

    async fn request_reset_password(
        &self,
        req: Request<proto::account::RequestResetPasswordReq>,
    ) -> Result<Response<proto::account::OpRes>> {
        let db_conn =
            &mut tools_lib_db::pg::connection::get_connection(&self.app_mode, &self.db_pool)
                .map_err(|e| Status::internal(e.to_string()))?;
        let redis_conn =
            &mut tools_lib_db::redis::connection::get_connection(&self.app_mode, &self.redis_pool)
                .map_err(|e| Status::internal(e.to_string()))?;
        let kafka_producer = &self.kafka_producer;

        // Check if email has been registered
        if !diesel::select(diesel::dsl::exists(
            schema::account::table.filter(schema::account::email.eq(&req.get_ref().email)),
        ))
        .get_result(db_conn)
        .map_err(|e| Status::internal(e.to_string()))?
        {
            return Err(Status::aborted("The email has not been registered."));
        }

        // Create random 6 digit verification code
        let verification_code = rand::thread_rng().gen_range(100000..=999999).to_string();

        // Create data key
        let data_key = format!("reset_password-{}", &req.get_ref().email);

        // Serialize data
        let data = serde_json::to_string(&AccountResetPassword {
            verify_code: verification_code.to_owned(),
        })
        .map_err(|e| Status::internal(e.to_string()))?;

        // Temporarily save to Redis
        redis_conn
            .set_ex(&data_key, &data, 10 * 60)
            .map_err(|e| Status::internal(e.to_string()))?;

        // Send to mailer service
        kafka_producer
            .send_result(
                FutureRecord::to("mailer").key(&data_key).payload(
                    &serde_json::to_string(&[&tools_mailer::contract::MailReq {
                        to: req.get_ref().email.to_owned(),
                        subject: format!("Reset Password Verification Code - {verification_code}"),
                        body: format!(
                            "Your reset password verification code is {verification_code}"
                        ),
                    }])
                    .map_err(|e| Status::internal(e.to_string()))?,
                ),
            )
            .map_err(|e| Status::internal(e.0.to_string()))?
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .map_err(|e| Status::internal(e.0.to_string()))?;

        Ok(Response::new(proto::account::OpRes { is_success: true }))
    }

    async fn verify_request_reset_password(
        &self,
        req: Request<proto::account::VerifyRequestResetPasswordReq>,
    ) -> Result<Response<proto::account::OpRes>> {
        let redis_conn =
            &mut tools_lib_db::redis::connection::get_connection(&self.app_mode, &self.redis_pool)
                .map_err(|e| Status::internal(e.to_string()))?;

        // Get temporary data from Redis
        let account_reset_password: String = redis_conn
            .get(&format!("reset_password-{}", &req.get_ref().email))
            .map_err(|e| Status::internal(e.to_string()))?;
        let account_reset_password: AccountResetPassword =
            serde_json::from_str(&account_reset_password)
                .map_err(|e| Status::internal(e.to_string()))?;

        // Check if verification code match
        if req.get_ref().verify_code != account_reset_password.verify_code {
            return Err(Status::aborted(
                "Failed to change email because of wrong verification code",
            ));
        }

        Ok(Response::new(proto::account::OpRes { is_success: true }))
    }

    async fn reset_password(
        &self,
        req: Request<proto::account::ResetPasswordReq>,
    ) -> Result<Response<proto::account::OpRes>> {
        let db_conn =
            &mut tools_lib_db::pg::connection::get_connection(&self.app_mode, &self.db_pool)
                .map_err(|e| Status::internal(e.to_string()))?;
        let redis_conn =
            &mut tools_lib_db::redis::connection::get_connection(&self.app_mode, &self.redis_pool)
                .map_err(|e| Status::internal(e.to_string()))?;
        let (argon2, salt) = helper::argon2::new(&self.argon2_hash_secret.as_bytes());
        let kafka_producer = &self.kafka_producer;

        // Create data key
        let data_key = format!("reset_password-{}", &req.get_ref().email);

        // Get temporary data from Redis
        let account_reset_password: String = redis_conn
            .get(&data_key)
            .map_err(|e| Status::internal(e.to_string()))?;
        let account_reset_password: AccountResetPassword =
            serde_json::from_str(&account_reset_password)
                .map_err(|e| Status::internal(e.to_string()))?;

        // Check if verification code match
        if req.get_ref().verify_code != account_reset_password.verify_code {
            return Err(Status::aborted(
                "Failed to change email because of wrong verification code",
            ));
        }

        // Remove existing data from Redis if verification code match
        redis_conn.del::<_, String>(&data_key).ok();

        // Hash the password for security
        let hashed_password = argon2
            .hash_password(req.get_ref().new_password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        // Update account password to database
        diesel::update(
            schema::account::table.filter(schema::account::email.eq(&req.get_ref().email)),
        )
        .set((
            schema::account::password.eq(&hashed_password),
            schema::account::updated_at.eq(&diesel::dsl::now),
        ))
        .execute(db_conn)
        .map_err(|e| Status::internal(e.to_string()))?;

        // Send email notification change email
        kafka_producer
            .send_result(
                FutureRecord::to("mailer").key(&data_key).payload(
                    &serde_json::to_string(&[&tools_mailer::contract::MailReq {
                        to: req.get_ref().email.to_owned(),
                        subject: "Reset Password Success".to_string(),
                        body: "Your account password is now changed".to_string(),
                    }])
                    .map_err(|e| Status::internal(e.to_string()))?,
                ),
            )
            .map_err(|e| Status::internal(e.0.to_string()))?
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .map_err(|e| Status::internal(e.0.to_string()))?;

        Ok(Response::new(proto::account::OpRes { is_success: true }))
    }

    async fn get_account(
        &self,
        req: Request<proto::account::GetAccountReq>,
    ) -> Result<Response<proto::account::GetAccountRes>> {
        let db_conn =
            &mut tools_lib_db::pg::connection::get_connection(&self.app_mode, &self.db_pool)
                .map_err(|e| Status::internal(e.to_string()))?;
        let jwt_secret = &self.jwt_secret;

        // Decode JWT Token
        let mut validation = jsonwebtoken::Validation::default();
        validation.required_spec_claims.remove("exp");
        let account_id = jsonwebtoken::decode::<jwt_claims::Claims>(
            &req.get_ref().token,
            &jsonwebtoken::DecodingKey::from_secret(jwt_secret.to_bytes()),
            &validation,
        )
        .map_err(|e| Status::internal(e.to_string()))?
        .claims
        .id;

        // Get account data
        let account_id = Uuid::from_str(&account_id).map_err(|e| Status::aborted(e.to_string()))?;
        let account_data = schema::account::table
            .find(&account_id)
            .first::<model::Account>(db_conn)
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(proto::account::GetAccountRes {
            id: account_data.id.to_string(),
            email: account_data.email,
            created_at: account_data.created_at.to_string(),
            updated_at: account_data.updated_at.to_string(),
        }))
    }

    async fn delete_account(
        &self,
        req: Request<proto::account::DeleteAccountReq>,
    ) -> Result<Response<proto::account::OpRes>> {
        let db_conn =
            &mut tools_lib_db::pg::connection::get_connection(&self.app_mode, &self.db_pool)
                .map_err(|e| Status::internal(e.to_string()))?;
        let jwt_secret = &self.jwt_secret;

        // Decode JWT Token
        let mut validation = jsonwebtoken::Validation::default();
        validation.required_spec_claims.remove("exp");
        let account_id = jsonwebtoken::decode::<jwt_claims::Claims>(
            &req.get_ref().token,
            &jsonwebtoken::DecodingKey::from_secret(jwt_secret.to_bytes()),
            &validation,
        )
        .map_err(|e| Status::internal(e.to_string()))?
        .claims
        .id;

        // Delete account
        let account_id = Uuid::from_str(&account_id).map_err(|e| Status::aborted(e.to_string()))?;
        diesel::delete(schema::account::table.find(account_id))
            .execute(db_conn)
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(proto::account::OpRes { is_success: true }))
    }

    async fn validate_token(
        &self,
        req: Request<proto::account::ValidateTokenReq>,
    ) -> Result<Response<proto::account::ValidateTokenRes>> {
        let db_conn =
            &mut tools_lib_db::pg::connection::get_connection(&self.app_mode, &self.db_pool)
                .map_err(|e| Status::internal(e.to_string()))?;
        let jwt_secret = &self.jwt_secret;

        // Decode JWT Token
        let mut validation = jsonwebtoken::Validation::default();
        validation.required_spec_claims.remove("exp");
        let account_id = jsonwebtoken::decode::<jwt_claims::Claims>(
            &req.get_ref().token,
            &jsonwebtoken::DecodingKey::from_secret(jwt_secret.to_bytes()),
            &validation,
        )
        .map_err(|e| Status::internal(e.to_string()))?
        .claims
        .id;

        // Get account data
        let account_id = Uuid::from_str(&account_id).map_err(|e| Status::aborted(e.to_string()))?;
        let account_data = schema::account::table
            .find(&account_id)
            .select((schema::account::id, schema::account::role))
            .first::<(Uuid, model::AccountRole)>(db_conn)
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(proto::account::ValidateTokenRes {
            id: account_data.0.to_string(),
            role: account_data.1.to_grpc_enum(),
        }))
    }
}
