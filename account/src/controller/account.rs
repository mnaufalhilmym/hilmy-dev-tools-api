use std::time::Duration;

use argon2::PasswordHasher;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rand::Rng;
use rdkafka::producer::FutureRecord;
use redis::Commands;
use serde_json::json;
use tonic::{Request, Response, Result, Status};

use crate::{
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
        let (argon2, salt) = helper::argon2::new(self.hash_secret.as_bytes());
        let kafka_producer = &self.kafka_producer;

        // Check if email has been registered
        if schema::account::table
            .filter(schema::account::email.eq(&req.get_ref().email))
            .first::<model::Account>(db_conn)
            .is_ok()
        {
            return Err(Status::new(
                tonic::Code::Aborted,
                "The email has been registered.",
            ));
        }

        // Hash the password for safety
        let hashed_password = argon2
            .hash_password(req.get_ref().password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        // Create random 6 digit verification code
        let verification_code = rand::thread_rng().gen_range(100000..=999999).to_string();

        // Serialize data
        let data_key = format!("sign_up-{}", &req.get_ref().email);
        let data = json!({
            "email": &req.get_ref().email,
            "password": &hashed_password,
            "verify_code": &verification_code,
        })
        .to_string();

        // Temporarily save to Redis
        redis_conn
            .set_ex(&data_key, &data, 1 * 60 * 60)
            .map_err(|e| Status::aborted(e.to_string()))?;

        // Send to mailer service
        if let Err(e) = kafka_producer
            .send(
                FutureRecord::to("mailer").key(&data_key).payload(
                    &serde_json::to_string(&tools_mailer::contract::MailReq {
                        to: req.get_ref().email.to_owned(),
                        subject: format!("Sign Up Verification Code - {verification_code}"),
                        body: format!("Your sign up verification code is {verification_code}"),
                    })
                    .map_err(|e| Status::aborted(e.to_string()))?,
                ),
                Duration::from_secs(0),
            )
            .await
        {
            eprintln!("Error send to Kafka: {e:?}");
        }

        Ok(Response::new(proto::account::OpRes { is_success: true }))
    }

    async fn verify_sign_up(
        &self,
        req: Request<proto::account::VerifySignUpReq>,
    ) -> Result<Response<proto::account::OpRes>> {
        println!("{req:?}");
        Ok(Response::new(proto::account::OpRes { is_success: true }))
    }

    async fn sign_in(
        &self,
        req: Request<proto::account::SignInReq>,
    ) -> Result<Response<proto::account::SignInRes>> {
        println!("{req:?}");
        Ok(Response::new(proto::account::SignInRes {
            token: String::new(),
        }))
    }

    async fn change_email(
        &self,
        req: Request<proto::account::ChangeEmailReq>,
    ) -> Result<Response<proto::account::OpRes>> {
        println!("{req:?}");
        Ok(Response::new(proto::account::OpRes { is_success: true }))
    }

    async fn change_password(
        &self,
        req: Request<proto::account::ChangePasswordReq>,
    ) -> Result<Response<proto::account::OpRes>> {
        println!("{req:?}");
        Ok(Response::new(proto::account::OpRes { is_success: true }))
    }

    async fn request_reset_password(
        &self,
        req: Request<proto::account::RequestResetPasswordReq>,
    ) -> Result<Response<proto::account::OpRes>> {
        println!("{req:?}");
        Ok(Response::new(proto::account::OpRes { is_success: true }))
    }

    async fn verify_request_reset_password(
        &self,
        req: Request<proto::account::VerifyRequestResetPasswordReq>,
    ) -> Result<Response<proto::account::OpRes>> {
        println!("{req:?}");
        Ok(Response::new(proto::account::OpRes { is_success: true }))
    }

    async fn reset_password(
        &self,
        req: Request<proto::account::ResetPasswordReq>,
    ) -> Result<Response<proto::account::OpRes>> {
        println!("{req:?}");
        Ok(Response::new(proto::account::OpRes { is_success: true }))
    }
}
