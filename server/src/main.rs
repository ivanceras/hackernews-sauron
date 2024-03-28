#![deny(warnings)]
use common::types::StorySorting;
use client::App;
use std::net::SocketAddr;
use common::api;
pub use client::sauron;
use axum::{Json, extract::Path,
    http::StatusCode, response::Html,
    http::{header::{HeaderMap, HeaderName, HeaderValue}},
    routing::get, Router,
    response::Response, response::IntoResponse};
use thiserror::Error;

mod page;

#[cfg(not(feature = "use-port-80"))]
const DEFAULT_PORT: u16 = 3030;

#[derive(Error, Debug)]
pub enum ServerError{
    #[error(transparent)]
    Api(#[from] api::ServerError),
    #[error("{0}")]
    Http(#[from] axum::http::Error),
}

impl IntoResponse for ServerError{

    fn into_response(self) -> Response {
        (
           StatusCode::INTERNAL_SERVER_ERROR,
           format!("Server error. Error: {}", self),
        )
        .into_response()
    }
}

#[tokio::main]
async fn main() {

    let route = Router::new()
        .route("/", get(top_stories))
        .route("/favicon.ico", get(favicon_ico))
        .route("/favicon.svg", get(favicon_svg))
        .route("/style.css", get(style_css))
        .route("/pkg/client.js", get(client_js))
        .route("/pkg/client_bg.wasm", get(client_bg_wasm))
        .route("/top", get(top_stories))
        .route("/best", get(best_stories))
        .route("/new", get(new_stories))
        .route("/show", get(show_stories))
        .route("/ask", get(ask_stories))
        .route("/job", get(job_stories))
        .route("/item/:story_id", get(story_item))
        .route("/user/:username", get(user))
        .route("/comment/:comment_id", get(comment))
        .route("/api/top", get(api_top_stories))
        .route("/api/best", get(api_best_stories))
        .route("/api/new", get(api_new_stories))
        .route("/api/show", get(api_show_stories))
        .route("/api/ask", get(api_ask_stories))
        .route("/api/job", get(api_job_stories))
        .route("/api/item/:story_id", get(api_story_item))
        .route("/api/comment/:comment_id", get(api_comment_permalink))
        .route("/api/user/:username", get(api_user_page));

    #[cfg(not(feature = "use-port-80"))]
    let port = if let Ok(port) = std::env::var("PORT") {
        if let Ok(port) = port.parse::<u16>() {
            port
        } else {
            DEFAULT_PORT
        }
    } else {
        DEFAULT_PORT
    };

    #[cfg(feature = "use-port-80")]
    let port = 80;

    #[cfg(feature = "use-ipv6")]
    let socket: SocketAddr = ([0, 0, 0, 0, 0, 0, 0, 0], port).into();

    #[cfg(not(feature = "use-ipv6"))]
    let socket: SocketAddr = ([0, 0, 0, 0], port).into();

    println!("serving at: http://{}", socket);
    axum::Server::bind(&socket)
        .serve(route.into_make_service())
        .await
        .unwrap();

}

async fn favicon_ico() -> (HeaderMap, Vec<u8>){
    let mut headers = HeaderMap::new();
    headers.insert(HeaderName::from_static("content-type"),
        HeaderValue::from_static("image/x-icon"));

    (headers, include_bytes!("../../client/favicon.ico").to_vec())
}

async fn favicon_svg() -> Vec<u8>{
   include_bytes!("../../client/favicon.svg").to_vec()
}

async fn style_css() -> (HeaderMap, String) {
    let mut headers = HeaderMap::new();
    headers.insert(HeaderName::from_static("content-type"),
        HeaderValue::from_static("text/css"));

    (headers, include_str!("../../client/style.css").to_string())
}

async fn client_js() -> (HeaderMap, String) {
    let mut headers = HeaderMap::new();
    headers.insert(HeaderName::from_static("content-type"),
        HeaderValue::from_static("text/javascript"));

    (headers, include_str!("../../client/pkg/client.js").to_string())
}

async fn client_bg_wasm() -> (HeaderMap, Vec<u8>) {
    let mut headers = HeaderMap::new();
    headers.insert(HeaderName::from_static("content-type"),
        HeaderValue::from_static("application/wasm"));

    (headers, include_bytes!("../../client/pkg/client_bg.wasm").to_vec())
}

async fn top_stories() -> Response  {
    render_stories(StorySorting::Top).await
}
async fn best_stories() -> Response {
    render_stories(StorySorting::Best).await
}
async fn new_stories() -> Response {
    render_stories(StorySorting::New).await
}
async fn show_stories() -> Response  {
    render_stories(StorySorting::Show).await
}

async fn ask_stories() -> Response  {
    render_stories(StorySorting::Ask).await
}

async fn job_stories() -> Response  {
    render_stories(StorySorting::Job).await
}


async fn api_top_stories() -> Response  {
    json_stories(StorySorting::Top).await
}
async fn api_best_stories() -> Response {
    json_stories(StorySorting::Best).await
}
async fn api_new_stories() -> Response {
    json_stories(StorySorting::New).await
}
async fn api_show_stories() -> Response  {
    json_stories(StorySorting::Show).await
}

async fn api_ask_stories() -> Response  {
    json_stories(StorySorting::Ask).await
}

async fn api_job_stories() -> Response  {
    json_stories(StorySorting::Job).await
}

async fn story_item(Path(item): Path<i64>) -> Response{
    render_story_page(item).await
}

async fn user(Path(username): Path<String>) -> Response {
    render_user_page(&username).await
}

async fn comment(Path(comment_id): Path<i64>) -> Response {
    render_comment_permalink(comment_id).await
}

async fn api_story_item(Path(story_id): Path<i64>) -> Response {
    json_story_page(story_id).await
}

async fn api_comment_permalink(Path(comment_id): Path<i64>) -> Response {
    json_comment_permalink(comment_id).await
}

 async fn  api_user_page(Path(username): Path<String>) -> Response {
    json_user_page(&username).await
}

async fn render_stories(
    sorting: StorySorting,
) -> Response {
     match api::get_stories_with_sorting(sorting).await{
        Ok(stories) => {
            let app = App::with_stories(stories);
            let index = page::index(&app).render_to_string();
            Html(index).into_response()
        }
        Err(e) => ServerError::from(e).into_response()
    }
}

async fn render_story_page(item: i64) -> Response {
     match api::get_story(item).await{
        Ok(story_page) => {
            let app = App::with_story(story_page);
            let index = page::index(&app).render_to_string();
            Html(index).into_response()
        }
        Err(e) => ServerError::from(e).into_response()
    }
}

async fn render_comment_permalink(
    comment_id: i64,
) -> Response {
    match api::get_comment(comment_id).await{
        Ok(comment) => {
            let app = App::with_comment_permalink(comment);
            let index = page::index(&app).render_to_string();
            Html(index).into_response()
        }
        Err(e) => ServerError::from(e).into_response()
    }
}

async fn render_user_page(
    username: &str,
) -> Response {
    match api::get_user_page(username).await{
        Ok(user_page) => {
            let app = App::with_user_page(user_page);
            let index = page::index(&app).render_to_string();
            Html(index).into_response()
        }
        Err(e) => ServerError::from(e).into_response()
    }
}

async fn json_story_page(
    story_id: i64,
) -> Response {
    match api::get_story(story_id).await{
        Ok(story_page) => {
            let json = serde_json::to_string(&story_page).expect("must serialize");
            Json(json).into_response()
        }
        Err(e) => ServerError::from(e).into_response()
    }
}

async fn json_user_page(
    username: &str,
) -> Response {
    match api::get_user_page(username).await{
        Ok(user_page) => {
            Json(user_page).into_response()
        }
        Err(e) => ServerError::from(e).into_response()
    }
}

async fn json_comment_permalink(
    comment_id: i64,
) -> Response {
    match api::get_comment(comment_id).await{
        Ok(comment) => Json(comment).into_response(),
        Err(e) => ServerError::from(e).into_response(),
    }
}

async fn json_stories(
    sorting: StorySorting,
) -> Response {
     match api::get_stories_with_sorting(sorting).await{
        Ok(stories) => Json(stories).into_response(),
        Err(e) => ServerError::from(e).into_response()
     }
}
