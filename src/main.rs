use login_system::{
    handle_github_login_click, handle_google_login_click, handle_kakao_login_click,
    handle_normal_login_click,
};

fn main() {
    handle_normal_login_click();
    handle_google_login_click();
    handle_github_login_click();
    handle_kakao_login_click();
}
