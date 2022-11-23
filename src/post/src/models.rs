use serde::{Deserialize, Serialize};
use rocket::form::FromForm;
use rocket::{get, post, serde::json::Json};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::settings::UrlObject;
use rocket_okapi::{openapi, openapi_get_routes, rapidoc::{make_rapidoc, HideShowConfig, GeneralConfig, RapiDocConfig}};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub user_id: u64,
    pub username: String,
    #[schemars(example = "example_email")]
    pub email: Option<String>,
}

pub fn example_email() -> &'static str {
    "test@example.com"
}

#[derive(Serialize, Deserialize, JsonSchema, FromForm)]
pub struct Post {
    pub post_id: u64,
    pub title: String,
    pub summary: Option<String>,
}
