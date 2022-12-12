#[macro_use]
mod pool;
mod views;
mod models;
extern crate rocket;


use std::collections::HashMap;

use rocket::fairing::{AdHoc};
use rocket::fs::{FileServer, relative};
use rocket::{Build, fairing, Rocket};
use rocket::figment::map;
use rocket::figment::value::{Map, Value};
use rocket_dyn_templates::Template;
use sea_orm_rocket::Database;


use pool::Db;
use views::*;

pub use entity::post;
pub use entity::post::Entity as Post;
use migration::MigratorTrait;

const DEFAULT_POSTS_PER_PAGE: u64 = 5;

pub async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    let _ = migration::Migrator::up(conn, None).await;
    Ok(rocket)
}

use rocket_okapi::{openapi_get_routes,
                   rapidoc::{
                       GeneralConfig, HideShowConfig, make_rapidoc, RapiDocConfig
                   }
};
use rocket_okapi::settings::UrlObject;
// use rocket::figment::{value::{Map, Value}, util::map};
use rocket::time::format_description::parse;

struct DB {
    db_schema: String,
    db_user: String,
    db_pass: String,
    db_host: String,
    db_port: i32,
    db_db_name: String
}

impl DB {
    fn url(self) -> String {
        /// ### return `postgres://hello_django:hello_django@localhost:5432/hello_django_prod`
        format!(
            "{}://{}:{}@{}:{}/{}",
            self.db_schema,
            self.db_user,
            self.db_pass,
            self.db_host,
            self.db_port,
            self.db_db_name
        )
    }

    fn export_to_env(self) {
        //! ```bash
        //! ROCKET_DATABASES='{my_db={url="db.sqlite"}}'
        //! ```
        let val = format!("{{my_db={{url={}}}}}", self.url() );
        println!("{:?}", &val);
        std::env::set_var("ROCKET_DATABASES", val)

    }
}

fn _get_db() -> String {
    let db = DB {
        db_schema: std::env::var("SCHEMA").unwrap_or("postgres".to_string()),
        db_host: std::env::var("DB_HOST").unwrap_or("localhost".to_string()),
        db_port: std::env::var("DB_PASS").unwrap_or(5432.to_string()).parse().unwrap(),
        db_user: std::env::var("DB_USER").unwrap_or("hello_django".to_string()),
        db_pass: std::env::var("DB_PASS").unwrap_or("hello_django".to_string()),
        db_db_name: std::env::var("DB_NAME").unwrap_or("hello_django_prod".to_string()),
    };
    let url = db.url();
    std::env::set_var("DATABASE_URL", &url);
    url
}

#[rocket::main]
pub async fn main() {

    let db: Map<_, Value> = map! {
    "url" => _get_db().into(),
    "pool_size" => 10.into(),
    "timeout" => 5.into(),
    };

    let figment = rocket::Config::figment()
        .merge(("databases", map!["sea_orm" => db]));

    let launch_result = rocket::custom(figment)
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .mount(
            "/",
            openapi_get_routes![
                views::get_all_users,
                views::get_user,
                views::get_user_by_name,
                views::create_user,
                views::hidden,
                views::create_post_by_query,
            ],
        )
        .mount(
            "/doc/",
            make_rapidoc(&RapiDocConfig {
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
        .launch()
        .await;
    match launch_result {
        Ok(_) => println!("Rocket shut down gracefully."),
        Err(err) => println!("Rocket had an error: {}", err),
    };
}


