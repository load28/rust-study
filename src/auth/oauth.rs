use crate::auth::traits::Loginable;
use crate::models::user::UserInfo;

// OAuth 공급자 종류
#[derive(Debug)]
pub enum OAuthProvider {
    Google,
    Github,
    Kakao,
}

// OAuth 로그인을 처리하는 확장된 구조체
pub struct OAuthLogin {
    pub user: UserInfo,
    pub provider: OAuthProvider,
    access_token: String,
    refresh_token: String,
}

impl OAuthLogin {
    pub fn new(user: UserInfo, provider: OAuthProvider) -> Self {
        Self {
            user,
            provider,
            access_token: String::new(),
            refresh_token: String::new(),
        }
    }

    pub fn refresh_token(&mut self) {
        println!("{}의 토큰 갱신 중...", self.provider_name());
    }

    pub fn provider_name(&self) -> String {
        format!("{:?} Provider", self.provider)
    }

    pub fn get_profile(&self) {
        println!(
            "{}에서 프로필 정보 조회: {}",
            self.provider_name(),
            self.user.id
        );
    }
}

impl Loginable for OAuthLogin {
    fn login(&self) {
        println!(
            "OAuth 로그인 시도: ID = {}, Provider = {:?}",
            self.user.id, self.provider
        );
    }
}
