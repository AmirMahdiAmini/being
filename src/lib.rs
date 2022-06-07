use std::{env, sync::Mutex};
use lazy_static::lazy_static;
pub mod being{
    tonic::include_proto!("being");
}
pub mod app;
pub mod utils;

lazy_static!{
    static ref REDIS_DB:Mutex<redis::Connection> = Mutex::new(redis::Client::open(get_env("REDISDB_URI"))
    .expect("invalid connection URL")
    .get_connection()
    .expect("failed to connect to redis"));
}
pub fn handler_logger(service_name:&str,ip:String) {
    println!("\x1b[41;97m SERVICE: \x1b[0m\x1b[44;97m {} \x1b[0m\x1b[41;97m IP: \x1b[0m\x1b[43;97m {} \x1b[0m",service_name,ip);
}
pub fn get_env(var:&str)->String{
    env::var(var).expect("Environment variable not set")
}
pub async fn mongo_connection() -> Result<mongodb::sync::Database,Box<dyn std::error::Error>>{
    let db_uri= get_env("MONGODB_URI");
    let option = mongodb::options::ClientOptions::parse(db_uri).expect("an error in MONGODB URI");
    let client = mongodb::sync::Client::with_options(option)
    .expect("couldn't connect to the MONGODB");
    logger!(WARN,"Connected to the MONGODB");
    Ok(client.database("being"))
}
#[macro_export]
macro_rules! logger {
    (warn,$e:expr) => {
        println!("\x1b[93m{}\x1b[0m",$e);
    };
    (message,$e:expr) => {
        println!("\x1b[92m{}\x1b[0m",$e);
    };
    (error,$e:expr) => {
        println!("\x1b[91m{}\x1b[0m",$e);
    };
    (info,$e:expr) => {
        println!("\x1b[94m{}\x1b[0m",$e);
    };
    (WARN,$e:expr) => {
        println!("\x1b[43;97m{}\x1b[0m",$e);
    };
    (MESSAGE,$e:expr) => {
        println!("\x1b[42;97m{}\x1b[0m",$e);
    };
    (ERROR,$e:expr) => {
        println!("\x1b[41;97m{}\x1b[0m",$e);
    };  
    (INFO,$e:expr) => {
        println!("\x1b[44;97m{}\x1b[0m",$e);
    };
}