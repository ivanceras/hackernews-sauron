#![deny(warnings)]
use common::types::StorySorting;
use client::App;
use client::sauron::Render;
use std::convert::Infallible;
use std::net::SocketAddr;
use warp::{http::Response, Filter};
use common::api;
pub use client::sauron;

mod page;

// path relative to the working directory when you run the server binary
const PKG_DIR: &str = "client/pkg";
const FAVICON_FILE: &str = "client/favicon.ico";
const FAVICON_SVG_FILE: &str = "client/favicon.svg";
const STYLE_CSS_FILE: &str = "client/style.css";
#[cfg(not(feature = "use-port80"))]
const DEFAULT_PORT: u16 = 3030;

#[tokio::main]
async fn main() {
    let pkg_files = warp::path("pkg").and(warp::fs::dir(PKG_DIR));
    let favicon = warp::path("favicon.ico").and(warp::fs::file(FAVICON_FILE));
    let favicon_svg =
        warp::path("favicon.svg").and(warp::fs::file(FAVICON_SVG_FILE));
    let style_css = warp::path("style.css").and(warp::fs::file(STYLE_CSS_FILE));

    let story_item =
        warp::path!("item" / i64).and_then(move |item| render_story_page(item));
    let user = warp::path!("user" / String)
        .and_then(move |username| render_user_page(username));
    let comment = warp::path!("comment" / i64)
        .and_then(move |comment_id| render_comment_permalink(comment_id));

    let stories_path = |sorting: StorySorting| {
        warp::path(sorting.to_str().to_owned())
            .and_then(move || render_stories(sorting))
    };


    let home =
        warp::path::end().and_then(move || render_stories(StorySorting::Top));
    let top = stories_path(StorySorting::Top);
    let best = stories_path(StorySorting::Best);
    let new = stories_path(StorySorting::New);
    let show = stories_path(StorySorting::Show);

    let api_stories_path = |sorting: StorySorting| {
        warp::path(sorting.to_str().to_owned()).and_then(move || json_stories(sorting))
    };
    let api_story_item = warp::path!("item" / i64)
        .and_then(move |story_id| json_story_page(story_id));
    let api_comment_permalink = warp::path!("comment" / i64)
        .and_then(move |comment_id| json_comment_permalink(comment_id));
    let api_user_page = warp::path!("user" / String)
        .and_then(move |username| json_user_page(username));

    let api_top_stories = api_stories_path(StorySorting::Top);
    let api_best_stories = api_stories_path(StorySorting::Best);
    let api_new_stories = api_stories_path(StorySorting::New);
    let api_show_stories = api_stories_path(StorySorting::Show);

    // We serve the data from hn-firebase already assembled.
    // (Though this is not needed in our client here)
    // creates this url tree:
    //   /api
    //       /top
    //       /best
    //       /new
    //       /show
    //       /comment
    //       /user
    let api = warp::path("api").and(
        api_story_item
            .or(api_top_stories)
            .or(api_best_stories)
            .or(api_new_stories)
            .or(api_show_stories)
            .or(api_comment_permalink)
            .or(api_user_page)
    );

    // serves the following url:
    //    /
    //      /top
    //      /best
    //      /new
    //      /show
    //      /item
    //      /user
    //      /comment
    //      /api
    //          /top
    //          /best
    //          /new
    //          /comment
    //          /user
    let index = warp::get().and(
        home.or(top)
            .or(best)
            .or(new)
            .or(show)
            .or(story_item)
            .or(user)
            .or(comment)
            .or(api),
    );

    // the overall url looks like:
    //    /
    //      /top
    //      /best
    //      /new
    //      /show
    //      /item
    //      /user
    //      /api
    //          /top
    //          /best
    //          /new
    //          /comment
    //          /user
    //      /pkg
    //      /favicon.ico
    //      /favicon.svg
    //      /style.css
    let routes = index
        .or(warp::get()
            .and(pkg_files.or(favicon).or(favicon_svg).or(style_css)));


    #[cfg(not(feature = "use-port80"))]
    let port = if let Ok(port) = std::env::var("PORT") {
        if let Ok(port) = port.parse::<u16>() {
            port
        } else {
            DEFAULT_PORT
        }
    } else {
        DEFAULT_PORT
    };

    #[cfg(feature = "use-port80")]
    let port = 80;

    #[cfg(feature = "use-ipv6")]
    let socket: SocketAddr = ([0, 0, 0, 0, 0, 0, 0, 0], port).into();

    #[cfg(not(feature = "use-ipv6"))]
    let socket: SocketAddr = ([0, 0, 0, 0], port).into();

    println!("serving at: http://{}", socket);
    warp::serve(routes).run(socket).await;
}

async fn render_stories(
    sorting: StorySorting,
) -> Result<impl warp::Reply, Infallible> {
    let stories = api::get_stories_with_sorting(sorting).await.expect("must not error");
    let app = App::with_stories(stories);
    let index = page::index(&app).render_to_string();
    Ok(Response::builder().body(index))
}

async fn render_story_page(item: i64) -> Result<impl warp::Reply, Infallible> {
    let story_page = api::get_story(item).await.expect("must not error");
    let app = App::with_story(story_page);
    let index = page::index(&app).render_to_string();
    Ok(Response::builder().body(index))
}

async fn render_comment_permalink(
    comment_id: i64,
) -> Result<impl warp::Reply, Infallible> {
    let comment = api::get_comment(comment_id).await.expect("must not error");
    let app = App::with_comment_permalink(comment);
    let index = page::index(&app).render_to_string();
    Ok(Response::builder().body(index))
}

async fn render_user_page(
    username: String,
) -> Result<impl warp::Reply, Infallible> {
    let user_page = api::get_user_page(&username).await.expect("must not error");
    let app = App::with_user_page(user_page);
    let index = page::index(&app).render_to_string();
    Ok(Response::builder().body(index))
}

async fn json_story_page(
    story_id: i64,
) -> Result<impl warp::Reply, Infallible> {
    let story_page = api::get_story(story_id).await.expect("must not error");
    let json = serde_json::to_string(&story_page).expect("must serialize");
    Ok(json)
}

async fn json_user_page(
    username: String,
) -> Result<impl warp::Reply, Infallible> {
    let user_page = api::get_user_page(&username).await.expect("must not error");
    let json = serde_json::to_string(&user_page).expect("must serialize");
    Ok(json)
}

async fn json_comment_permalink(
    comment_id: i64,
) -> Result<impl warp::Reply, Infallible> {
    let comment = api::get_comment(comment_id).await.expect("must not error");
    let json = serde_json::to_string(&comment).expect("must serialize");
    Ok(json)
}

async fn json_stories(
    sorting: StorySorting,
) -> Result<impl warp::Reply, Infallible> {
    let stories = api::get_stories_with_sorting(sorting).await.expect("must not error");
    let json = serde_json::to_string(&stories).expect("must serialize");
    Ok(json)
}
