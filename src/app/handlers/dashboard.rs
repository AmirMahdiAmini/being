use crate::being::being_service_server::BeingService;
use crate::app::services::dashboard::DashboardService;
use crate::being::{Default,Empty,GetInformationResponse,DeleteRequest,InvitationRequest,AcceptInvitationRequest};
use crate::handler_logger;
use tonic::{Response, Status, Request};
pub struct Dashboard{
    pub collection:mongodb::sync::Collection<bson::Document>,
}
#[tonic::async_trait]
impl BeingService for Dashboard{
    async fn delete(&self,request:Request<DeleteRequest>)->Result<Response<Default>,Status> {
        handler_logger("delete",request.remote_addr().unwrap().to_string());
        let sid = request.extensions().get::<String>().unwrap().clone();
        DashboardService::delete(&self.collection, &request.into_inner(), &sid).await
    }
    async fn get_information(&self,request:Request<Empty>)->Result<Response<GetInformationResponse>,Status> {
        handler_logger("get information",request.remote_addr().unwrap().to_string());
        let sid = request.extensions().get::<String>().unwrap().clone();
        DashboardService::get_information(&self.collection, &sid).await
    }
    async fn send_invite(&self,request:Request<InvitationRequest>)->Result<Response<Default>,Status>{
        handler_logger("send invite",request.remote_addr().unwrap().to_string());
        let sid = request.extensions().get::<String>().unwrap().clone();
        DashboardService::send_invite(&self.collection,&request.into_inner(), &sid).await
    }
    async fn accept_invite(&self,request:Request<AcceptInvitationRequest>)->Result<Response<Default>,Status> {
        handler_logger("accept invite",request.remote_addr().unwrap().to_string());
        let sid = request.extensions().get::<String>().unwrap().clone();
        DashboardService::accept_invite(&self.collection, &request.into_inner(),&sid).await
    }
    async fn cancel_invite(&self,request:Request<InvitationRequest>)->Result<Response<Default>,Status> {
        handler_logger("cancel invite",request.remote_addr().unwrap().to_string());
        let sid = request.extensions().get::<String>().unwrap().clone();
        DashboardService::cancel_invite(&self.collection, &request.into_inner(), &sid).await
    }
}