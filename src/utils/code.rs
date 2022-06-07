use rand::{thread_rng, distributions::Alphanumeric, Rng};
pub fn create_sid()->String{
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(55)
        .map(char::from)
        .collect()
}
pub fn authorization_code()->String{
    let code1 :String =thread_rng()
        .sample_iter(&Alphanumeric)
        .take(35)
        .map(char::from)
        .collect();
    let code2 :String =thread_rng()
    .sample_iter(&Alphanumeric)
    .take(15)
    .map(char::from)
    .collect();
    format!("{}{}{}",code1,
     thread_rng().gen_range(1531242..9965323),
     code2)
}
pub fn create_verification_code()->i32{
    thread_rng().gen_range(253124..986532)
}