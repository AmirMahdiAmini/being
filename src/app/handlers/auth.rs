use crate::being::auth_service_server::AuthService;
use crate::being::{SignupRequest,VerifyRequest,SigninRequest,ForgotPasswordRequest,ChangePasswordRequest,Default};
use crate::app::services::auth::AuthenticationService;
use crate::handler_logger;
use tonic::{Response, Status, Request};
pub struct Auth{
    pub collection:mongodb::sync::Collection<bson::Document>,
}
#[tonic::async_trait]
impl AuthService for Auth{
    async fn signup(&self, request:Request<SignupRequest>)->Result<Response<Default>,Status>{
        handler_logger("signup",request.remote_addr().unwrap().to_string());
        AuthenticationService::signup(&self.collection,&request.into_inner()).await
    }
    async fn signin(&self, request:Request<SigninRequest>) ->Result<Response<Default>,Status> {
        handler_logger("signin",request.remote_addr().unwrap().to_string());
        AuthenticationService::signin(&self.collection,&request.into_inner()).await
    }
    async fn verify(&self, request:Request<VerifyRequest>)->Result<Response<Default>,Status> {
        handler_logger("verify",request.remote_addr().unwrap().to_string());
        AuthenticationService::verify_account(&self.collection,&request.into_inner()).await
    }
    async fn forgot_password(&self, request:Request<ForgotPasswordRequest>) ->Result<Response<Default>,Status> {
        handler_logger("forgot password",request.remote_addr().unwrap().to_string());
        AuthenticationService::forgot_password(&self.collection,&request.into_inner()).await
    }
    async fn change_password(&self, request:Request<ChangePasswordRequest>) ->Result<Response<Default>,Status> {
        handler_logger("change password",request.remote_addr().unwrap().to_string());
        AuthenticationService::change_password(&self.collection,&request.into_inner()).await
    }
}