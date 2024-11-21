use crate::auth::traits::Loginable;
use crate::models::user::UserInfo;

// 일반 로그인을 처리하는 래퍼 구조체
pub struct NormalLogin(pub UserInfo);

impl NormalLogin {
    pub fn new(id: String, password: String) -> Self {
        Self(UserInfo { id, password })
    }
}

impl Loginable for NormalLogin {
    fn login(&self) {
        println!("일반 로그인 시도: ID = {}", self.0.id);
    }
}
