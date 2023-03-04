use argon2::PasswordHasher;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rand::Rng;
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

        let hashed_password = argon2
            .hash_password(req.get_ref().password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        let verification_code = rand::thread_rng().gen_range(100000..=999999).to_string();

        let data = json!({
            "email": &req.get_ref().email,
            "password": &hashed_password,
            "verify_code": &verification_code,
        })
        .to_string();

        redis_conn
            .set_ex(
                format!("sign_up-{}", &req.get_ref().email),
                data,
                1 * 60 * 60,
            )
            .map_err(|e| Status::aborted(e.to_string()))?;

        // TODO: Send mail

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
