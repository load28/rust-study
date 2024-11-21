pub mod auth;
pub mod models;

use auth::normal::NormalLogin;
use auth::oauth::{OAuthLogin, OAuthProvider};
use auth::traits::Loginable;
use models::user::UserInfo;

pub fn execute_login<T: Loginable>(login_handler: T) {
    println!("----------------------로그인 프로세스 시작----------------------");
    login_handler.login();
    println!("----------------------로그인 프로세스 완료----------------------");
}

pub fn handle_normal_login_click() {
    let normal_login: NormalLogin = NormalLogin::new("user123".to_string(), "pass123".to_string());
    execute_login(normal_login);
}

pub fn handle_oauth_login_click() {
    let oauth_user: UserInfo = UserInfo {
        id: "oauth_user".to_string(),
        password: "oauth_token".to_string(),
    };
    let oauth_login: OAuthLogin = OAuthLogin::new(oauth_user, OAuthProvider::Google);
    execute_login(oauth_login);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_login_flow() {
        handle_normal_login_click();
    }

    #[test]
    fn test_oauth_login_flow() {
        handle_oauth_login_click();
    }
}
