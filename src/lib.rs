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

pub fn handle_google_login_click() {
    let oauth_user: UserInfo = UserInfo {
        id: "google_user".to_string(),
        password: "google_token".to_string(),
    };
    let oauth_login: OAuthLogin = OAuthLogin::new(oauth_user, OAuthProvider::Google);
    execute_login(oauth_login);
}

pub fn handle_github_login_click() {
    let oauth_user: UserInfo = UserInfo {
        id: "github_user".to_string(),
        password: "github_token".to_string(),
    };
    let oauth_login: OAuthLogin = OAuthLogin::new(oauth_user, OAuthProvider::Github);
    execute_login(oauth_login);
}

pub fn handle_kakao_login_click() {
    let oauth_user: UserInfo = UserInfo {
        id: "kakao_user".to_string(),
        password: "kakao_token".to_string(),
    };
    let oauth_login: OAuthLogin = OAuthLogin::new(oauth_user, OAuthProvider::Kakao);
    execute_login(oauth_login);
}
