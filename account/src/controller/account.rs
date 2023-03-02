use tonic::{Request, Response, Result};

use crate::proto::{self, account::AccountService};

use super::AccountController;

#[tonic::async_trait]
impl AccountService for AccountController {
    async fn sign_up(
        &self,
        req: Request<proto::account::SignUpReq>,
    ) -> Result<Response<proto::account::OpRes>> {
        println!("{req:?}");
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
