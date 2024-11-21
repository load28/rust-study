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
        let provider_name: String = self.provider_name();
        println!("{}의 토큰 갱신 중...", provider_name);
    }

    pub fn provider_name(&self) -> String {
        format!("{:?} Provider", self.provider)
    }

    pub fn get_profile(&self) {
        let provider_name: String = self.provider_name();
        let user_id: &String = &self.user.id;
        println!(
            "{}에서 프로필 정보 조회: {}",
            provider_name,
            user_id
        );
    }
}

impl Loginable for OAuthLogin {
    fn login(&self) {
        let user_id: &String = &self.user.id;
        println!(
            "OAuth 로그인 시도: ID = {}, Provider = {:?}",
            user_id, self.provider
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oauth_login_creation() {
        let user: UserInfo = UserInfo {
            id: "test_oauth".to_string(),
            password: "test_token".to_string(),
        };
        let login: OAuthLogin = OAuthLogin::new(user, OAuthProvider::Google);
        
        assert_eq!(login.user.id, "test_oauth");
        assert_eq!(login.provider_name(), "Google Provider");
    }

    #[test]
    fn test_provider_name_formatting() {
        let user: UserInfo = UserInfo {
            id: "test".to_string(),
            password: "test".to_string(),
        };
        let login: OAuthLogin = OAuthLogin::new(user, OAuthProvider::Github);
        
        assert_eq!(login.provider_name(), "Github Provider");
    }
}
