pub mod user;
pub const fn from_int(data:&i32)->&'static str{
    match data{
        0 => "sent_requests",
        1 => "received_requests",
        2 => "system_notifications",
        _ => "system_notifications",
    }
}