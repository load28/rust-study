pub mod auth;
pub mod models;

use std::fmt;
use std::ops::Add;

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

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}

trait Wizard {
    fn fly(&self) -> &str;
}

trait Pilot {
    fn fly(&self) -> &str;
}

struct Human {}

impl Wizard for Human {
    fn fly(&self) -> &str {
        "Human is flying"
    }
}
impl Pilot for Human {
    fn fly(&self) -> &str {
        "Human is flying"
    }
}

trait Animal {
    fn eat() -> String;
}

struct Dog {}

impl Animal for Dog {
    fn eat() -> String {
        "Animal Dog is eating".to_string()
    }
}

impl Dog {
    fn eat() -> String {
        "Dog is eating".to_string()
    }
}

trait SuperTrait {
    fn super_trait(&self) -> &str;
}

// Display trait를 구현한 타입만 OutlinePrint trait을 구현할 수 있음
// SuperTrait를 구현한 타입만 OutlinePrint trait을 구현할 수 있음
// Super trait은 다중 선언 가능
trait OutlinePrint: fmt::Display + SuperTrait {
    fn outline_print(&self) {
        let output: String = self.to_string();
        let len: usize = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
        println!("{}", self.super_trait());
    }
}

struct Point {
    x: i32,
    y: i32,
}

// Point 타입을 Display trait로 구현
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl SuperTrait for Point {
    fn super_trait(&self) -> &str {
        "SuperTrait"
    }
}

impl OutlinePrint for Point {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_millimeters_and_meters() {
        let millimeters = Millimeters(1000);
        let meters = Meters(1);
        let result = millimeters + meters;
        assert_eq!(result.0, 2000);
    }

    #[test]
    fn test_fly() {
        let human = Human {};
        assert_eq!(Wizard::fly(&human), "Human is flying");
        assert_eq!(Pilot::fly(&human), "Human is flying");
    }

    #[test]
    fn test_animal() {
        assert_eq!(Dog::eat(), "Dog is eating");
        assert_eq!(<Dog as Animal>::eat(), "Animal Dog is eating");
    }

    #[test]
    fn test_outline_print() {
        let point = Point { x: 1, y: 2 };
        point.outline_print();
    }
}
