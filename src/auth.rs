use rocket::Outcome;
use rocket::request::{self, Request, FromRequest};
use regex::Regex;

use crate::jwt::JWT;

#[derive(Debug)]
enum ApiError {
    MissingToken
}

// Contains the username
struct RoleUser(String);
struct RoleAdmin { user: RoleUser }
// TODO: voir pour simplement faire :
// type User = ();
// type Admin = ();
// cf:  https://github.com/SergioBenitez/Rocket/blob/52320020bc26387b96a0126a0d49821ad3125e7d/site/guide/4-requests.md#guard-transparency

// Tmp
#[derive(Serialize, Deserialize)]
struct User {
    pub username: String,
    pub roles: Vec<String>
}

fn decode_jwt(request: &Request) -> Option<JWT> {
    // This closure will execute at most once per request, regardless of
    // the number of times the `User` guard is executed.
    let re = Regex::new(r"/^Bearer\s/").unwrap();
    let bearer_token = request.headers().get("Authorization").collect::<String>();
    let token = re.replace_all(&bearer_token, "").to_string();
    match JWT::decode(token) {
        Ok(jwt) => Some(jwt),
        _ => None
    }
}

fn create_jwt(user: User) -> String {
    "Bearer ".to_owned() + &JWT::create(user.username, user.roles).unwrap()
}

impl<'a, 'r> FromRequest<'a, 'r> for RoleUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<RoleUser, ()> {
        let decoded_jwt = decode_jwt(request);

        match decoded_jwt {
            Some(jwt) => {
                if jwt.roles.contains(&"USER".to_owned()) {
                    Outcome::Success(RoleUser(jwt.sub))
                } else {
                    Outcome::Forward(())
                }
            }
            None => Outcome::Forward(())
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for RoleAdmin {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<RoleAdmin, ()> {
        let decoded_jwt = decode_jwt(request);

        match decoded_jwt {
            Some(jwt) => {
                if jwt.roles.contains(&"ADMIN".to_owned()) {
                    Outcome::Success(RoleAdmin { user: RoleUser(jwt.sub) } )
                } else {
                    Outcome::Forward(())
                }
            }
            None => Outcome::Forward(())
        }
    }
}