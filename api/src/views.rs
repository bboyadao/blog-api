use sea_orm_rocket::Connection;
use rocket::request::FlashMessage;
use rocket_dyn_templates::Template;
use serde_json::json;
use rocket::{Build, fairing, Request, Rocket};
use rocket::form::{Context, Form};
use entity::post;
use rocket::response::{Flash, Redirect};
use migration::MigratorTrait;
use crate::DEFAULT_POSTS_PER_PAGE;
use crate::pool::Db;
use biz::{Mutation, Query};
use rocket::{catch, delete};


#[get("/new")]
pub async fn new() -> Template {
    Template::render("new", &Context::default())
}

#[post("/", data = "<post_form>")]
pub async fn create(conn: Connection<'_, Db>, post_form: Form<post::Model>) -> Flash<Redirect> {
    let db = conn.into_inner();

    let form = post_form.into_inner();

    Mutation::create_post(db, form)
        .await
        .expect("could not insert post");

    Flash::success(Redirect::to("/"), "Post successfully added.")
}

#[post("/<id>", data = "<post_form>")]
pub async fn update(
    conn: Connection<'_, Db>,
    id: i32,
    post_form: Form<post::Model>,
) -> Flash<Redirect> {
    let db = conn.into_inner();

    let form = post_form.into_inner();

    Mutation::update_post_by_id(db, id, form)
        .await
        .expect("could not update post");

    Flash::success(Redirect::to("/"), "Post successfully edited.")
}

#[get("/?<page>&<posts_per_page>")]
pub async fn list(
    conn: Connection<'_, Db>,
    page: Option<u64>,
    posts_per_page: Option<u64>,
    flash: Option<FlashMessage<'_>>,
) -> Template {
    let db = conn.into_inner();

    // Set page number and items per page
    let page = page.unwrap_or(1);
    let posts_per_page = posts_per_page.unwrap_or(DEFAULT_POSTS_PER_PAGE);
    if page == 0 {
        panic!("Page number cannot be zero");
    }

    let (posts, num_pages) = Query::find_posts_in_page(db, page, posts_per_page)
        .await
        .expect("Cannot find posts in page");

    Template::render(
        "index",
        json! ({
            "page": page,
            "posts_per_page": posts_per_page,
            "num_pages": num_pages,
            "posts": posts,
            "flash": flash.map(FlashMessage::into_inner),
        }),
    )
}

#[get("/<id>")]
pub async fn edit(conn: Connection<'_, Db>, id: i32) -> Template {
    let db = conn.into_inner();

    let post: Option<post::Model> = Query::find_post_by_id(db, id)
        .await
        .expect("could not find post");

    Template::render(
        "edit",
        json! ({
            "post": post,
        }),
    )
}

#[delete("/<id>")]
pub async fn delete(conn: Connection<'_, Db>, id: i32) -> Flash<Redirect> {
    let db = conn.into_inner();

    Mutation::delete_post(db, id)
        .await
        .expect("could not delete post");

    Flash::success(Redirect::to("/"), "Post successfully deleted.")
}

#[delete("/")]
pub async fn destroy(conn: Connection<'_, Db>) -> Result<(), rocket::response::Debug<String>> {
    let db = conn.into_inner();

    Mutation::delete_all_posts(db)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    Template::render(
        "error/404",
        json! ({
            "uri": req.uri()
        }),
    )
}




use rocket::{get, post, serde::json::Json};
use rocket_okapi::{openapi,};
use super::models::{Post, User};

/// # Get all users
///
/// Returns all users in the system.
#[openapi(tag = "Users")]
#[get("/user")]
pub fn get_all_users() -> Json<Vec<User>> {


    Json(vec![User {
        user_id: 42,
        username: "bob".to_owned(),
        email: None,
    }])
}

/// # Get user
///
/// Returns a single user by ID.
#[openapi(tag = "Users")]
#[get("/user/<id>")]
pub fn get_user(id: u64) -> Option<Json<User>> {
    Some(Json(User {
        user_id: id,
        username: "bob".to_owned(),
        email: None,
    }))
}

/// # Get user by name
///
/// Returns a single user by username.
#[openapi(tag = "Users")]
#[get("/user_example?<user_id>&<name>&<email>")]
pub fn get_user_by_name(user_id: u64, name: String, email: Option<String>) -> Option<Json<User>> {
    Some(Json(User {
        user_id,
        username: name,
        email,
    }))
}

/// # Create user
#[openapi(tag = "Users")]
#[post("/user", data = "<user>")]
pub fn create_user(user: Json<User>) -> Json<User> {
    user
}

#[openapi(skip)]
#[get("/hidden")]
pub fn hidden() -> Json<&'static str> {
    Json("Hidden from swagger!")
}


/// # Create post using query params
///
/// Returns the created post.
#[openapi(tag = "Posts")]
#[get("/post_by_query?<post..>")]
pub fn create_post_by_query(post: Post) -> Option<Json<Post>> {
    Some(Json(post))
}
