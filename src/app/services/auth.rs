use crate::being::{SignupRequest,VerifyRequest,SigninRequest,ForgotPasswordRequest,ChangePasswordRequest,Default};
use crate::utils::jwt::JWT;
use crate::app::models::user::{User, Gender, Role};
use crate::app::services::checking::Checking;
use crate::utils::code::{create_sid, authorization_code};
use crate::REDIS_DB;
use crate::utils::time;
use super::sms::SMSAuthentication;
use redis::Commands;
use regex::Regex;
use tonic::{Response, Status};
pub struct AuthenticationService;

type DefaultResponse = Result<Response<Default>,Status>;
impl AuthenticationService{
    pub async fn signup(user_collection:&mongodb::sync::Collection<bson::Document>,data:&SignupRequest)->DefaultResponse{
        if data.age < 18{
            return Err(Status::aborted("sorry, you are not old enough to signup"))
        }else if data.age > 70{
            return Err(Status::aborted("sorry, we don't support people older than 70"))
        }
        let regex = Regex::new(r"^[A-Za-z]{3,}[A-Za-z0-9]{2,}$").unwrap();
        if !regex.is_match(&data.username.trim()){
            return Err(Status::aborted("enter a correct username"))
        };
        match Checking::check_user_exists(&user_collection,String::from("username"),&data.username.clone()).await{
            Ok(_)=>{},
            Err(e)=>return Err(Status::already_exists(e))
            };
        match Checking::check_user_exists(&user_collection,String::from("phone"),&data.phone.clone()).await{
            Ok(_)=>{},
            Err(e)=>return Err(Status::already_exists(e))
        };
        let hashed_password = match bcrypt::hash(&data.password.trim(), bcrypt::DEFAULT_COST){
            Ok(p)=>p,
            Err(_)=> return Err(Status::internal("something went wrong #113"))
        };
        let mut notifications :Vec<String> = Vec::new();
        notifications.push(format!("{} ساخت حساب کاربری در سایت",time::now()));
        let user = User{
            sid:create_sid(),
            phone:data.phone.clone(),
            username:data.username.clone(),
            password:hashed_password,
            partner:String::from(""),
            location:String::from(""),
            status:"Free Time".to_string(),
            city:data.city.clone(),
            age:data.age.to_string().clone(),
            gender:Gender::from(data.gender).to_string(),
            is_active:false,
            is_private:false,
            role:Role::Silver.to_string(),
            friends:vec![],
            group:vec![],
            created_at:time::now(),
            updated_at:time::now(),
            received_requests:vec![],
            sent_requests:vec![],
            system_notifications:notifications,
        };
        match user_collection.insert_one(bson::to_document(&user).unwrap(), None){
            Ok(_)=>{
                match SMSAuthentication::verify_sms(&data.phone.clone(),String::from("خوش آمدید")).await{
                    Ok(_)=>{},
                    Err(e)=>{
                        match user_collection.delete_one(bson::doc!{"sid":user.sid.clone()},None){
                            Ok(_)=>{},
                            Err(_)=>return Err(Status::internal("something went wrong #113"))
                        };
                        return Err(Status::aborted(e))
                    }
                };
                drop(user_collection);
                return Ok(Response::new(Default{message:"user created successfully, now verify it".to_string()}))
            },
                Err(_)=>return Err(Status::internal("something went wrong #113"))
        }
    }
    pub async fn signin(user_collection:&mongodb::sync::Collection<bson::Document>,data:&SigninRequest)->DefaultResponse{
        let usr = &data.phone_or_username.clone();
        let mut result:bson::Document;
        result = match user_collection.find_one(bson::doc!{"phone":usr.clone()}, None){
            Ok(data)=>{
                match data{
                Some(d)=>d,
                None=>{
                    result = match user_collection.find_one(bson::doc!{"username":usr.clone()}, None){
                        Ok(data)=>{
                            match data{
                                Some(d)=>d,
                                None=>return Err(Status::not_found("couldn't find your account"))
                            }
                        },
                        Err(_)=>return Err(Status::not_found("couldn't find your account"))
                    };
                    result
                }
            }},
            Err(_)=>return Err(Status::not_found("couldn't find your account"))
        };
        let user:User = match bson::from_bson(bson::Bson::Document(result)){
            Ok(d) =>d ,
            Err(_)=>return Err(Status::internal("something went wrong #123"))
        };
        if user.is_active == false{
            match SMSAuthentication::verify_sms(&user.phone.clone(),String::from("احراز هویت")).await{
                Ok(_)=>return Ok(Response::new(Default{message:String::from("your account wasn't verified, we sent verification code to your phone number")})),
                Err(e)=>return Err(Status::aborted(e))
            }
        }
        match bcrypt::verify(&data.password,&user.password){
            Ok(res)=>{
                if res == false{
                    return Err(Status::aborted("password is wrong"))
                }
            }
            Err(_)=>return Err(Status::internal("something went wrong #123"))
        }
        match user_collection.update_one(
            bson::doc!{"phone":user.phone.clone()},
            bson::doc!{"$push":{
                "system_notifications":format!("{} ورود به حساب کاربری",time::now()),
            }}, None){
                Ok(_)=>{},
            Err(_)=>return Err(Status::internal("something went wrong #123"))
        }
        let jwt = JWT{
            exp:time::timestamp() + 3600,
            sid:user.sid.clone(),
        };
        match jwt.create_token(){
            Ok(t)=>{
                drop(user_collection);
                return Ok(Response::new(Default{message:t}))
            },
            Err(_)=> return Err(Status::aborted("couldn't create your token, signin again"))
        }
    }
    pub async fn verify_account(user_collection:&mongodb::sync::Collection<bson::Document>,data:&VerifyRequest)->DefaultResponse{
        let _:String = match REDIS_DB.lock().unwrap().get(data.phone.clone()){
            Ok(code)=>{
                if code != data.code.clone(){
                    return Err(Status::aborted("the verification code was sent is wrong"))
                }
                code
            },
            Err(_)=>return Err(Status::not_found("not found"))
        };
        let _:() = REDIS_DB.lock().unwrap().del(data.phone.clone()).unwrap();
        match Checking::check_isactive(user_collection, &data.phone.clone()).await{
            Ok(_)=>{
                let sid = authorization_code();
                let jwt = JWT{
                    sid:sid.clone(),
                    exp:time::timestamp()+200,
                };
                let token = match jwt.create_token(){
                    Ok(t)=>t,
                    Err(_)=>return Err(Status::internal("something went wrong #133"))
                };
                let _:() = match REDIS_DB.lock().unwrap().set_ex(sid.clone(),token,220){
                    Ok(d)=>d,
                    Err(_)=>return Err(Status::internal("something went wrong #133"))
                };
                return Ok(Response::new(Default{message:format!("{}",sid)}))
            },
            Err(_)=>{
                match user_collection.update_one(
                    bson::doc!{"phone":data.phone.clone()},
                    bson::doc!{"$set":{
                        "is_active":true,
                        "updated_at":time::now()
                    }}, None){
                    Ok(_)=>{
                        drop(user_collection);
                        return Ok(Response::new(Default{message:String::from("user verified successfully")}))
                    }
                    Err(_)=>return Err(Status::internal("something went wrong #133"))
                }
            }
        }
    }
    pub async fn forgot_password(user_collection:&mongodb::sync::Collection<bson::Document>,data:&ForgotPasswordRequest)->DefaultResponse{
        match user_collection.find_one(bson::doc!{"phone":&data.phone}, None){
            Ok(result)=>match result {
                Some(_)=>{
                    match Checking::check_isactive(&user_collection,&data.phone.clone()).await{
                        Ok(_)=>{},
                        Err(e)=>return Err(Status::aborted(e))
                    };
                },
                None=>return Err(Status::not_found("couldn't find your account"))
            },
            Err(_)=>return Err(Status::not_found("couldn't find your account"))
        };
        match SMSAuthentication::verify_sms(&data.phone.clone(),String::from("تغییر رمز عبور")).await{
            Ok(_)=>{
                drop(user_collection);
                return Ok(Response::new(Default{message:String::from("verify the code")}))
                },
            Err(e)=>return Err(Status::aborted(e))
        }
    }
    pub async fn change_password(user_collection:&mongodb::sync::Collection<bson::Document>,data:&ChangePasswordRequest)->DefaultResponse{
        let token:String = match REDIS_DB.lock().unwrap().get(&data.sid.clone()){
            Ok(t)=>t,
            Err(_)=>return Err(Status::not_found("not found"))
        };
        match JWT::verify_token(token.as_str()){
            Ok(_)=>{},
            Err(_)=>return Err(Status::permission_denied("Unauthorized"))
        }
        match user_collection.find_one(bson::doc!{"phone":&data.phone.clone()}, None){
            Ok(d)=>match d {
                Some(_)=>{},
                None=>return Err(Status::aborted("phone number you entered is wrong"))
        },
            Err(_)=>return Err(Status::aborted("phone number you entered is wrong"))
        };
        let hashed_password = match bcrypt::hash(&data.password.trim(), bcrypt::DEFAULT_COST){
            Ok(p)=>p,
            Err(_)=> return Err(Status::internal("something went wrong #153"))
        };
        match user_collection.update_one(
            bson::doc!{"phone":&data.phone.clone()},
            bson::doc!{"$set":{
                "password":hashed_password,
                "updated_at":time::now(),
            }}, None){
                Ok(_)=>{
                let _:() = REDIS_DB.lock().unwrap().del(&data.sid).unwrap();
            },
            Err(_)=>return Err(Status::internal("something went wrong #153"))
        };
        match user_collection.update_one(
            bson::doc!{"phone":&data.phone.clone()},
            bson::doc!{"$push":{
                "system_notifications":format!("{} تغییر رمز عبور",time::now()),
            }}, None){
                Ok(_)=>{
                drop(user_collection);
                return Ok(Response::new(Default{message:String::from("password changed successfully")}))
            },
            Err(_)=>return Err(Status::internal("something went wrong #153"))
        }
    }
}