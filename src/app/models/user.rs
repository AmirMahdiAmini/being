use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize)]
pub struct User{
    pub sid:String,
    pub phone:String,
    pub username:String,
    pub password:String,
    pub partner:String,
    pub location:String,
    pub status:String,
    pub city:String,
    pub age:String,
    pub gender:String,
    pub is_active:bool,
    pub role:String,
    pub is_private:bool,
    pub friends:Vec<String>,
    pub group:Vec<String>,
    pub created_at:String,
    pub updated_at:String,
    pub system_notifications:Vec<String>,
    pub sent_requests:Vec<Invitations>,
    pub received_requests:Vec<Invitations>,
}
#[derive(Deserialize,Serialize)]
pub enum Role{
    Gold,
    Silver
}
impl Role{
    pub fn to_string(&self)-> String{
        match &self{
            Role::Gold => String::from("gold"),
            Role::Silver => String::from("silver"),
        }
    }
}
#[derive(Deserialize,Serialize)]
pub enum Gender{
    Male,
    Female,
    Trans,
}
impl From<i32> for Gender{
    fn from(data: i32) -> Self {
        match data{
            0 => Gender::Male,
            1 => Gender::Female,
            2 => Gender::Trans,
            _ => Gender::Trans,
        }
    }
}
impl Gender{
    pub fn to_string(&self)-> String{
        match &self{
            Gender::Male => String::from("male"),
            Gender::Female => String::from("female"),
            Gender::Trans => String::from("trans"),
        }
    }
}
#[derive(Debug,Deserialize,Serialize)]
pub struct Invitations{
    pub username:String,
    pub message:String,
    pub address:String,
}