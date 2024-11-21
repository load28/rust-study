mod auth;
mod models;

use auth::normal::NormalLogin;
use auth::oauth::{OAuthLogin, OAuthProvider};
use auth::traits::Loginable;
use models::user::UserInfo;

// 공통 로그인 처리 함수
fn execute_login<T: Loginable>(login_handler: T) {
    println!("----------------------로그인 프로세스 시작----------------------");
    login_handler.login();
    println!("----------------------로그인 프로세스 완료----------------------");
}

// 일반 로그인 버튼 클릭 시 호출될 함수
fn handle_normal_login_click() {
    let normal_login: NormalLogin = NormalLogin::new("user123".to_string(), "pass123".to_string());
    execute_login(normal_login);
}

// OAuth 로그인 버튼 클릭 시 호출될 함수
fn handle_oauth_login_click() {
    let oauth_user: UserInfo = UserInfo {
        id: "oauth_user".to_string(),
        password: "oauth_token".to_string(),
    };
    let oauth_login: OAuthLogin = OAuthLogin::new(oauth_user, OAuthProvider::Google);
    execute_login(oauth_login);
}

fn main() {
    // 일반 로그인 버튼 클릭 시뮬레이션
    handle_normal_login_click();

    // OAuth 로그인 버튼 클릭 시뮬레이션
    handle_oauth_login_click();
}
