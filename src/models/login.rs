use crate::db::DB;
use rocket::{
    http::Status,
    outcome::try_outcome,
    request::{FromRequest, Outcome, Request},
    serde::{Deserialize, Serialize},
    tokio::sync::RwLock,
    State,
};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(crate = "rocket::serde")]
pub struct UserModel {
    #[serde(default)]
    pub id: u32,
    #[serde(default)]
    pub name: String,
    pub email: String,
    #[serde(default)]
    pub phone: String,
    pub password: String,
    #[serde(default)]
    pub image: String,
    #[serde(default)]
    pub points: i32,
    #[serde(default)]
    pub credit: i32,
    #[serde(default)]
    pub token: String,
}

#[rocket::async_trait()]
impl<'r> FromRequest<'r> for UserModel {
    type Error = &'static str;
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth = req
            .guard::<&State<RwLock<DB<String, String>>>>()
            .await
            .expect("already initialized in main")
            .inner();
        let token = req.headers().get_one("authorization");
        let uri_str = req.uri().to_string();
        if uri_str != "/api/register" && uri_str != "/api/login" {
            match token {
                Some(token) => match auth.read().await.get(token) {
                    Some(token) => {
                        let home_model = req
                            .guard::<&State<RwLock<DB<String, UserModel>>>>()
                            .await
                            .expect("already initialized in main")
                            .inner();
                        Outcome::Success(
                            home_model
                                .read()
                                .await
                                .get(token)
                                .expect("must exists by the the fact of having a token")
                                .clone(),
                        )
                    }
                    None => Outcome::Failure((Status::Unauthorized, "Not Authorized")),
                },
                None => Outcome::Failure((Status::Unauthorized, "authorization is not provided")),
            }
        } else {
            Outcome::Success(try_outcome!(req.guard::<UserModel>().await))
        }
    }
}
