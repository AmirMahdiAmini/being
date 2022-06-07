use crate::being::{Default,GetInformationResponse,DeleteRequest,InvitationRequest,AcceptInvitationRequest,Invitation};
use crate::app::models::user::{User,Invitations};
use crate::app::models::from_int;
use crate::utils::time;
use tonic::{Response, Status};
pub struct DashboardService;

type DefaultResponse = Result<Response<Default>,Status>;
impl DashboardService{
    pub async fn get_information(user_collection:&mongodb::sync::Collection<bson::Document>, sid:&str) -> Result<Response<GetInformationResponse>,Status>{
        match user_collection.find_one(bson::doc!{"sid":sid}, None){
            Ok(c) => {
                match c{
                    Some(d)=>{
                        let user:User = match bson::from_bson(bson::Bson::Document(d)){
                            Ok(d) =>d ,
                            Err(_)=>return Err(Status::internal("something went wrong #1213"))
                        };
                        let mut received_requests:Vec<Invitation> = Vec::new();
                        for invitation in user.received_requests{
                            received_requests.push(Invitation{
                                username:invitation.username,
                                message:invitation.message,
                                address:invitation.address,
                            });
                        }
                        let mut sent_requests:Vec<Invitation> = Vec::new();
                        for invitation in user.sent_requests{
                            sent_requests.push(Invitation{
                                username:invitation.username,
                                message:invitation.message,
                                address:invitation.address,
                            });
                        }
                        return Ok(Response::new(GetInformationResponse{
                            age:user.age,
                            is_active:user.is_active,
                            gender:user.gender,
                            phone:user.phone,
                            status:user.status,
                            partner:user.partner,
                            username:user.username,
                            received_requests:received_requests,
                            sent_requests:sent_requests,
                            system_notifications:user.system_notifications,
                        }))
                    }
                    None=>return Err(Status::not_found("couldn't find your account"))
                }
            },
            Err(_)=>return Err(Status::not_found("couldn't find your account"))
        }
    }
    pub async fn delete(user_collection:&mongodb::sync::Collection<bson::Document> ,data:&DeleteRequest,sid:&String)->DefaultResponse{
        let field = from_int(&data.request);
        match user_collection.update_one(bson::doc!{"sid":sid},bson::doc!{"$pull":{
            field:&data.data
        }
        },None){
            Ok(res)=>{
                if res.modified_count == 0{
                    return Err(Status::not_found("not found"))
                }
                if field == "received_requests" || field == "sent_requests"{
                    let user = match user_collection.find_one(bson::doc!{"sid":sid}, None){
                        Ok(c) => {
                            match c{
                                Some(d)=>{
                                    let user:User = match bson::from_bson(bson::Bson::Document(d)){
                                        Ok(d) =>d ,
                                        Err(_)=>return Err(Status::internal("something went wrong #1213"))
                                    };
                                    user
                                }
                                None=>return Err(Status::not_found("couldn't find your account"))
                            }
                        },
                        Err(_)=>return Err(Status::not_found("couldn't find your account"))
                    };
                    let update_field = if field == "sent_requests" {
                        "received_requests"
                    }else {
                        "sent_requests"
                    };
                    match user_collection.update_one(bson::doc!{"username":&data.data},bson::doc!{"$pull":{
                        update_field:&user.username
                    }
                    },None){
                        Ok(c)=>{
                            if c.modified_count == 0{
                                return Err(Status::not_found("not found"))
                            }
                            return Ok(Response::new(Default{message:String::from("successfully deleted")}))
                        }
                        Err(_)=>return Err(Status::internal("something went wrong #1213"))
                    }
                }
                return Ok(Response::new(Default{message:String::from("successfully deleted")}))
            }
            Err(_)=>return Err(Status::internal("something went wrong #1213"))
        }
    }
    pub async fn send_invite(user_collection:&mongodb::sync::Collection<bson::Document> ,data:&InvitationRequest,sid:&String)->DefaultResponse{
        match user_collection.find_one(bson::doc!{"username":&data.username}, None){
            Ok(result)=>match result {
                Some(d)=>{
                    let target_user:User = match bson::from_bson(bson::Bson::Document(d)){
                        Ok(d) =>d ,
                        Err(_)=>return Err(Status::internal("something went wrong #1213"))
                    };
                    if target_user.partner != ""{
                        return Err(Status::internal("you can't send an invitation to the user that already has a partner"))
                    }else if target_user.status != "Free Time"{
                        return Err(Status::internal("you can't send an invitation to the user"))
                    }
                    match user_collection.find_one(bson::doc!{"sid":sid}, None){
                        Ok(c)=>{
                            match c{
                            Some(d)=>{
                                let user:User = match bson::from_bson(bson::Bson::Document(d)){
                                    Ok(d) =>d ,
                                    Err(_)=>return Err(Status::internal("something went wrong #1213"))
                                };
                                match user.sent_requests.iter().find(|&r| {&r.username == &data.username}){
                                    Some(_)=>return Err(Status::internal("you already sent this request")),
                                    None=>{}
                                };
                                match user.received_requests.iter().find(|&r| {&r.username == &data.username}){
                                    Some(_)=>return Err(Status::internal("you already had this request")),
                                    None=>{}
                                };
                                let mut invite = Invitations{
                                    username:data.username.to_string(),
                                    message:data.message.to_string(),
                                    address:data.address.to_string(),
                                };
                                match user_collection.update_one(bson::doc!{"sid":sid}, bson::doc!{"$push":
                                {
                                    "sent_requests":bson::to_document(&invite).unwrap(),
                                    "system_notifications":format!("{} در {} ارسال دعوت به ",time::now(),&data.username),
                                }}, None){
                                    Ok(_)=>{},
                                    Err(_)=>return Err(Status::internal("something went wrong #1213"))
                                };
                                invite.username = user.username.clone();
                                match user_collection.update_one(bson::doc!{"username":&data.username}, bson::doc!{"$push":
                                {
                                    "received_requests":bson::to_document(&invite).unwrap(),
                                    "system_notifications":format!("{} در {} دریافت دعوت از ",time::now(),&user.username),
                                }}, None){
                                    Ok(_)=>{},
                                    Err(_)=>return Err(Status::internal("something went wrong #1213"))
                                };
                                drop(invite);
                                drop(user_collection);
                                return Ok(Response::new(Default { message: String::from("request sent") }))

                            },
                            None=>{
                                return Err(Status::not_found("couldn't find the account #1213"))
                            }
                        }},
                        Err(_)=>return Err(Status::not_found("something went wrong #1213"))
                    };
                },
                None=>return Err(Status::not_found("couldn't find the account #1213"))
            },
            Err(_)=>return Err(Status::not_found("something went wrong #1213"))
        }
    }
    pub async fn accept_invite(user_collection:&mongodb::sync::Collection<bson::Document>,data:&AcceptInvitationRequest,sid:&String)->DefaultResponse{
        let field = from_int(&data.accept_request);
        match user_collection.find_one(bson::doc!{"username":&data.username}, None){
            Ok(result)=>match result {
                Some(d)=>{
                    let target_user:User = match bson::from_bson(bson::Bson::Document(d)){
                        Ok(d) =>d ,
                        Err(_)=>return Err(Status::internal("something went wrong #1213"))
                    };
                    if target_user.partner != ""{
                        return Err(Status::aborted("you can't send an invitation to the user that already has a partner"))
                    }
                    match user_collection.find_one(bson::doc!{"sid":sid}, None){
                        Ok(c)=>{
                            match c{
                            Some(d)=>{
                                let user:User = match bson::from_bson(bson::Bson::Document(d)){
                                    Ok(d) =>d ,
                                    Err(_)=>return Err(Status::internal("something went wrong #1333"))
                                };
                                if user.partner != ""{
                                    return Err(Status::internal("you can't choose another partner right now"))
                                }
                                if field == "sent_requests"{
                                    match user.sent_requests.iter().find(|&r| {&r.username == &data.username}){
                                        Some(_)=>{},
                                        None=>{
                                            return Err(Status::not_found("not found"))
                                        }
                                    }
                                }else if field == "received_requests"{
                                    match user.received_requests.iter().find(|&r| {&r.username == &data.username}){
                                        Some(_)=>{},
                                        None=>{
                                            return Err(Status::not_found("not found"))
                                        }
                                    };
                                }else{
                                    return Err(Status::internal("wrong data"))
                                }
                                let removed_list:Vec<String> = Vec::new();
                                match user_collection.update_one(bson::doc!{"sid":sid},bson::doc!{"$set":{
                                    "sent_requests":removed_list.clone(),
                                    "received_requests":removed_list.clone(),
                                    "partner":&target_user.username,
                                    "location":&data.address,
                                    "status":String::from("BUSY"),
                                }}, None){
                                    Ok(_)=>{},
                                    Err(_)=>return Err(Status::internal("something went wrong #1333"))
                                };
                                match user_collection.update_one(bson::doc!{"sid":target_user.sid},bson::doc!{"$set":{
                                    "sent_requests":removed_list.clone(),
                                    "received_requests":removed_list,
                                    "partner":&user.username,
                                    "location":&data.address,
                                    "status":String::from("BUSY"),
                                }}, None){
                                    Ok(_)=>{},
                                    Err(_)=>return Err(Status::internal("something went wrong #1333"))
                                };
                                return Ok(Response::new(Default { message: String::from("congratulations") }))
                            },
                            None=>return Err(Status::not_found("couldn't find the account #1333"))
                            
                        }},
                        Err(_)=>return Err(Status::internal("something went wrong #1333"))
                    };
                },
                None=>return Err(Status::not_found("couldn't find the account #1333"))
            },
            Err(_)=>return Err(Status::internal("something went wrong #1333"))
        }
    }
    pub async fn cancel_invite(user_collection:&mongodb::sync::Collection<bson::Document>,data:&InvitationRequest,sid:&String)->DefaultResponse{
        match user_collection.find_one(bson::doc!{"username":&data.username}, None){
            Ok(result)=>match result {
                Some(d)=>{
                    let target_user:User = match bson::from_bson(bson::Bson::Document(d)){
                        Ok(d) =>d ,
                        Err(_)=>return Err(Status::internal("something went wrong #1213"))
                    };
                    if target_user.partner == ""{
                        return Err(Status::aborted("he/she has no partner"))
                    }
                    if data.username != target_user.username{
                        return Err(Status::internal("wrong data #1444"))
                    }
                    match user_collection.find_one(bson::doc!{"sid":sid}, None){
                        Ok(c)=>{
                            match c{
                            Some(_)=>{
                                match user_collection.update_one(bson::doc!{"sid":sid},bson::doc!{"$set":{
                                    "partner":String::from(""),
                                    "location":String::from(""),
                                    "status":String::from("Free Time"),
                                }}, None){
                                    Ok(_)=>{},
                                    Err(_)=>{}
                                };
                                match user_collection.update_one(bson::doc!{"sid":target_user.sid},bson::doc!{"$set":{
                                    "partner":String::from(""),
                                    "location":String::from(""),
                                    "status":String::from("Free Time"),
                                }}, None){
                                    Ok(_)=>{},
                                    Err(_)=>{}
                                };
                            },
                            None=>{
                                return Err(Status::not_found("couldn't find the account #1444"))
                            }
                        }},
                        Err(_)=>return Err(Status::internal("something went wrong #1444"))
                    };
                
                },
                None=>return Err(Status::not_found("couldn't find the account #1444"))
            },
            Err(_)=>return Err(Status::internal("something went wrong #1444"))
        }
        Ok(Response::new(Default { message: String::from("") }))
    }
}