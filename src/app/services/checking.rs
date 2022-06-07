use crate::app::models::user::User;

pub struct Checking;

impl Checking{
    pub async fn check_user_exists(user_collection:&mongodb::sync::Collection<bson::Document>,field:String,data:&String)->Result<(), String>{
        match user_collection.find_one(bson::doc!{field.clone():data}, None){
            Ok(d)=>match d {
                Some(_)=>return Err(format!("{} is already taken",field)),
                None=>{}
        },
            Err(_)=>return Err(format!("{} is already taken",field)),
        };
        Ok(())
    }
    pub async fn check_isactive(user_collection:&mongodb::sync::Collection<bson::Document>,data:&String)->Result<(), String>{
        match user_collection.find_one(bson::doc!{"phone":data}, None){
            Ok(d)=>match d {
                Some(d)=>{
                    let user:User = bson::from_bson(bson::Bson::Document(d)).unwrap();
                    if user.is_active == false{
                        return Err(String::from("user is not active"))
                    }else{
                        return Ok(())
                    }
                },
                None=>{}
        },
            Err(_)=>return Err(String::from("user is not active")),
        };
        Ok(())
    }
}
