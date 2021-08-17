use async_recursion::async_recursion;
use crate::types::{
    Comment, StoryItem, StoryPageData, StorySorting, UserData,
};
use futures::future::join_all;
use lazy_static::lazy_static;
use lru::LruCache;
use std::sync::Mutex;
use thiserror::Error;

const BASE_URL: &str = "https://hacker-news.firebaseio.com/v0";
const TOP_STORIES: &str = "/topstories.json";
const NEW_STORIES: &str = "/newstories.json";
const BEST_STORIES: &str = "/beststories.json";
const ITEM_API: &str = "/item";
const USER_API: &str = "/user";

const STORIES_COUNT: usize = 20;
const COMMENT_DEPTH: i64 = 3;

lazy_static! {
    static ref STORY_CACHE: Mutex<LruCache<i64, StoryPageData>> =
        Mutex::new(LruCache::new(1000));
    static ref STORY_PREVIEW_CACHE: Mutex<LruCache<i64, StoryItem>> =
        Mutex::new(LruCache::new(1000));
}

pub async fn get_stories() -> Result<Vec<StoryItem>, ServerError>{
    get_stories_with_sorting(StorySorting::default()).await
}

pub async fn get_stories_with_sorting(
    sort: StorySorting,
) -> Result<Vec<StoryItem>, ServerError> {
    let stories_api = match sort {
        StorySorting::Best => BEST_STORIES,
        StorySorting::Top => TOP_STORIES,
        StorySorting::New => NEW_STORIES,
    };

    let url = format!("{}{}", BASE_URL, stories_api);
    let story_ids = make_json_get_request::<Vec<i64>>(&url).await?;
    println!("story_ids:({}) {:#?}", story_ids.len(), story_ids);
    let first_story_ids = &story_ids[..story_ids.len().min(STORIES_COUNT)];
    let story_futures = first_story_ids
        .iter()
        .map(|story_id| get_story_preview(*story_id));

    let stories = join_all(story_futures)
        .await
        .into_iter()
        .filter_map(|c| c.ok())
        .collect();
    Ok(stories)
}

pub async fn get_story(story_id: i64) -> Result<StoryPageData, ServerError> {
    if let Some(cached_story) = STORY_CACHE.lock().unwrap().get(&story_id) {
        return Ok(cached_story.clone());
    }
    let url = format!("{}{}/{}.json", BASE_URL, ITEM_API, story_id);
    let mut story = make_json_get_request::<StoryPageData>(&url).await?;
    let comment_ids = &story.kids[..story.kids.len().min(3)];
    let comments = join_all(
        comment_ids
            .iter()
            .map(|story_id| get_comment_with_depth(*story_id, COMMENT_DEPTH)),
    )
    .await
    .into_iter()
    .filter_map(|c| c.ok())
    .collect();

    story.comments = comments;
    STORY_CACHE.lock().unwrap().put(story_id, story.clone());
    Ok(story)
}

// Same as get_story but does not add comments
pub async fn get_story_preview(story_id: i64) -> Result<StoryItem, ServerError> {
    if let Some(cached_story) =
        STORY_PREVIEW_CACHE.lock().unwrap().get(&story_id)
    {
        return Ok(cached_story.clone());
    }
    let url = format!("{}{}/{}.json", BASE_URL, ITEM_API, story_id);
    let story_preview = make_json_get_request::<StoryItem>(&url).await?;
    STORY_PREVIEW_CACHE
        .lock()
        .unwrap()
        .put(story_id, story_preview.clone());
    Ok(story_preview)
}



#[cfg_attr(target_arch = "wasm32", async_recursion(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_recursion)]
pub async fn get_comment_with_depth(
    story_id: i64,
    depth: i64,
) -> Result<Comment, ServerError> {
    let url = format!("{}{}/{}.json", BASE_URL, ITEM_API, story_id);
    let mut comment = make_json_get_request::<Comment>(&url).await?;
    if depth > 0 {
        let sub_comment_ids = &comment.kids[..comment.kids.len().min(3)];
        let sub_comments = join_all(
            sub_comment_ids
                .iter()
                .map(|story_id| get_comment_with_depth(*story_id, depth - 1)),
        )
        .await
        .into_iter()
        .filter_map(|c| c.ok())
        .collect();

        comment.sub_comments = sub_comments;
    }
    Ok(comment)
}

pub async fn get_comment(comment_id: i64) -> Result<Comment, ServerError> {
    let comment = get_comment_with_depth(comment_id, COMMENT_DEPTH).await?;
    Ok(comment)
}


pub async fn get_user_page(user_id: &String) -> Result<UserData, ServerError> {
    let url = format!("{}{}/{}.json", BASE_URL, USER_API, user_id);
    let mut user = make_json_get_request::<UserData>(&url).await?;
    //submitted could be comments or story post
    let first_story_ids = &user.submitted[..user.submitted.len().min(30)];
    let story_futures = first_story_ids
        .iter()
        .map(|story_id| get_story_preview(*story_id));

    // we only filter the success where other types such as comments fail
    let stories = join_all(story_futures)
        .await
        .into_iter()
        .filter_map(|story| story.ok())
        .collect();

    user.stories = stories;

    dbg!(&user);
    Ok(user)
}

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("json error: {0}")]
    SerdeJson(#[from] serde_json::Error),
}


pub async fn make_json_get_request<T: serde::de::DeserializeOwned>(
    url: &str,
) -> Result<T, ServerError> {
    dbg!(url);
    let response = reqwest::get(url).await?;
    Ok(response.json::<T>().await?)
}
