use crate::user;


pub enum Role {
    Admin,
    Moderator,
    User,
}

pub struct User{
    pub username: String,
    pub password: String,
    pub role: Role,

}


#[tauri::command]
pub fn user_list(user: String, password:String) -> bool{

    let mut users = User{username: "user".to_string(),password: "1234".to_string(), role: Role::User};
    let mut moder = User{username: "mod".to_string(),password: "1234".to_string(), role: Role::Moderator};
    let mut adm = User{username: "adm".to_string(),password: "1234".to_string(), role: Role::Admin};

    let mut logick = false;
    if user == users.username && password == users.password{
        logick = true;
    }
    else {
        logick = false;
    }

    logick
}
