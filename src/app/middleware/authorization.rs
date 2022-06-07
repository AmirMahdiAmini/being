use tonic::{Request, Status};
use crate::utils::jwt;

pub fn authorization(mut req: Request<()>) -> Result<Request<()>, Status> {
    match req.metadata().get("authorization") {
        Some(token)=>{
            match jwt::JWT::verify_token(token.to_str().unwrap().to_string().split_once(" ").unwrap().1){
                Ok(t)=>{
                    req.extensions_mut().insert(t);
                    Ok(req)
                },
                Err(_)=>Err(Status::unauthenticated("Unauthorized"))
            }
        },
        _ => Err(Status::unauthenticated("No valid auth token")),
    }
}