// #[rocket::main]
// async fn main() {
//     let db: Map<_, Value> = map! {
//         "url" => "db.sqlite".into(),
//         "pool_size" => 10.into()
//     };
//
//     let figment = rocket::Config::figment()
//         .merge(("databases", map!["my_db" => db]));
//
//     let launch_result = rocket::custom(figment)
//         // .attach(Db::init())
//         // .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
//         .mount(
//             "/",
//             openapi_get_routes![
//                 post::views::get_all_users,
//                 post::views::get_user,
//                 post::views::get_user_by_name,
//                 post::views::create_user,
//                 post::views::hidden,
//                 post::views::create_post_by_query,
//             ],
//         )
//         .mount(
//             "/doc/",
//             make_rapidoc(&RapiDocConfig {
//                 general: GeneralConfig {
//                     spec_urls: vec![UrlObject::new("General", "../openapi.json")],
//                     ..Default::default()
//                 },
//                 hide_show: HideShowConfig {
//                     allow_spec_url_load: false,
//                     allow_spec_file_load: false,
//                     ..Default::default()
//                 },
//                 ..Default::default()
//             }),
//         )
//         .launch()
//         .await;
//     match launch_result {
//         Ok(_) => println!("Rocket shut down gracefully."),
//         Err(err) => println!("Rocket had an error: {}", err),
//     };
// }

fn main() {
    api::main();
}
