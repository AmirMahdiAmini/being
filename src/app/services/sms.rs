use crate::app::pkg::sms::Message;
use crate::utils::code::{create_verification_code};
use crate::REDIS_DB;
use redis::Commands;
pub struct SMSAuthentication;

impl SMSAuthentication{
    pub async fn verify_sms(phone:&String,message:String)->Result<(),String>{
        let c:String = match REDIS_DB.lock().unwrap().get(&phone.clone()){
            Ok(d)=>d,
            Err(_)=>{
                String::from("n115")
            }
        };
        if c == "n115"{
        }else{
            return Err(String::from("code just sent to you"))
        }
        let code = create_verification_code();
        let _:() = match REDIS_DB.lock().unwrap().set_ex(phone.clone(),code.clone(),120){
            Ok(d)=>d,
            Err(_)=>return Err(String::from("something went wrong. try later"))
        };
        match Message::new(format!("{} \n کد تایید: {}",message,code.clone()), String::from(phone.clone())).send_message().await{
            Ok(_)=>Ok(()),
            Err(_)=>return Err(String::from("couldn't send a code"))
        }
    }
}