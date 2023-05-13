mod article;
mod comment;
mod errors;
mod models;
mod user;

use ntex::web::{self, middleware, App, HttpServer};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{env, sync::Arc};

#[derive(Debug, Clone)]
pub struct AppState {
    pub db_pool: Pool<Postgres>,
}

#[ntex::main]
async fn main() {
    dotenvy::dotenv().ok();

    env::set_var("RUST_LOG", "ntex=info");
    env_logger::init();

    let db_url = env::var("DATABASE_URL").expect("Please set `DATABASE_URL`");

    // State
    let app_state = Arc::new(AppState {
        db_pool: PgPoolOptions::new()
            .max_connections(10)
            .connect(&db_url)
            .await
            .unwrap(),
    });

    HttpServer::new(move || {
        App::new()
            .state(Arc::clone(&app_state))
            .wrap(middleware::Logger::default())
            .configure(|cfg| route(Arc::clone(&app_state), cfg))
    })
    .bind("0.0.0.0:12345")
    .unwrap()
    .run()
    .await
    .unwrap();
}

fn route(_state: Arc<AppState>, cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/article")
            .route("/{id}", web::get().to(article::view::get_article))
            .route("", web::post().to(article::new::new_article))
            .route("", web::put().to(article::edit::edit_article))
            .route("/{id}", web::delete().to(article::delete::delete_article))
            .route(
                "/search/{keyword}",
                web::get().to(article::search::search_article),
            ),
    )
    .service(web::scope("/articles").route("", web::get().to(article::view::get_articles_preview)))
    .service(
        web::scope("/user")
            .route("/login", web::post().to(user::login::github_login))
            .route("/info", web::get().to(user::info::get_user_info)),
    )
    .service(
        web::scope("/comment")
            .route(
                "/{article_id}",
                web::get().to(comment::view::get_comments_for_article),
            )
            .route("", web::post().to(comment::new::new_comment))
            .route(
                "/{comment_id}",
                web::delete().to(comment::delete::delete_comment),
            ),
    );
}
