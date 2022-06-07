use beinglib::app::handlers::dashboard::Dashboard;
use beinglib::being::auth_service_server::AuthServiceServer;
use beinglib::being::being_service_server::BeingServiceServer;

use beinglib::app::handlers::auth::Auth;
use beinglib::{logger,get_env,mongo_connection};
use beinglib::app::middleware::authorization;
use tonic::transport::Server;


#[tokio::main]
async fn main() ->Result<(),Box<dyn std::error::Error>>{
    dotenv::dotenv().ok();
    let db = mongo_connection().await?;
    logger!(WARN,"Server Started...");
    let auth = Auth{
        collection:db.collection("user")
    };
    let dashboard =  Dashboard{
        collection:db.collection("user")
    };
    Server::builder()
    .add_service(AuthServiceServer::new(auth))
    .add_service( BeingServiceServer::with_interceptor(dashboard,authorization::authorization))
    .serve(get_env("address").parse()?).await.expect("Server failed to start");
    Ok(())
}