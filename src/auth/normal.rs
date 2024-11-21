use crate::auth::traits::Loginable;
use crate::models::user::UserInfo;

// 일반 로그인을 처리하는 래퍼 구조체
pub struct NormalLogin(pub UserInfo);

impl NormalLogin {
    pub fn new(id: String, password: String) -> Self {
        let user_info: UserInfo = UserInfo { id, password };
        Self(user_info)
    }
}

impl Loginable for NormalLogin {
    fn login(&self) {
        let id: &String = &self.0.id;
        println!("일반 로그인 시도: ID = {}", id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_login_creation() {
        let login: NormalLogin = NormalLogin::new("test_user".to_string(), "test_pass".to_string());
        assert_eq!(login.0.id, "test_user");
        assert_eq!(login.0.password, "test_pass");
    }
}
