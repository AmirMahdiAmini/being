use jsonwebtoken::{Header, encode,decode, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
const SECRET:&str = "af29c77f607b07791ce4c68a00647fbeb26b8bddfc185025fbf0e09486ff4505";
#[derive(Debug,Deserialize,Serialize)]
pub struct JWT{
    pub sid:String,
    pub exp:usize,
}
impl JWT{
    pub fn create_token(&self)->Result<String,String>{
       let header = Header{kid:Some("VBW".to_owned()),alg:jsonwebtoken::Algorithm::HS256,..Default::default()};
       match encode(&header, &self, &EncodingKey::from_secret(SECRET.as_bytes())){
           Ok(t)=>Ok(t),
           Err(_)=>return Err(String::from("couldn't create a token"))
       } 
    }
    pub fn verify_token(token:&str)->Result<String,String>{
        match decode::<JWT>(token,&DecodingKey::from_secret(SECRET.as_bytes()),&jsonwebtoken::Validation::default()){
            Ok(cls)=>Ok(cls.claims.sid),
            Err(_)=>return Err(String::from("couldn't verify the token"))
        }
    }   
}