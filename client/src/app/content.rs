use crate::app;
use common::types::{Comment, StoryItem, StoryPageData, UserData};
use sauron::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Clone, derive_more::From,
)]
pub enum Content {
    Stories(Vec<StoryItem>),
    StoryPage(StoryPageData),
    CommentPermalink(Comment),
    UserPage(UserData),
}

impl Content {
    pub fn view(&self) -> Node<app::Msg> {
        match self {
            Content::Stories(stories) => {
                node! {
                    <div class="index-page">
                       {self.view_story_preview_list(stories)}
                    </div>
                }
            }
            Content::StoryPage(story_page) => {
                node! {
                    <div class="story-page">
                        { self.view_story_page(story_page) }
                    </div>
                }
            }
            Content::UserPage(user_data) => {
                node! {
                    <div class="user-details">
                        <h4>{ text!("{}:",user_data.id) }</h4>
                        <p>{ safe_html(&user_data.about) }</p>
                        <span>{ text!("{} karma", user_data.karma) }</span>
                        <div class="submissions">
                             {self.view_story_preview_list(&user_data.stories)}
                        </div>
                    </div>
                }
            }
            Content::CommentPermalink(comment) => {
                node! {
                    <div class="comment-permalink">
                        {self.view_comment(comment)}
                    </div>
                }
            }
        }
    }

    fn view_story_preview_list(&self, stories: &[StoryItem]) -> Node<app::Msg> {
        node! {
            <ol>
            {
                for (i, story_preview) in stories.iter().enumerate() {
                    node! {
                        <li key=story_preview.id>
                            <div class="item-number">{text!("{}. ",i+1)}</div>
                            {self.view_story_preview(story_preview)}
                        </li>
                    }
                }
            }
            </ol>
        }
    }

    fn view_story_preview(&self, story_preview: &StoryItem) -> Node<app::Msg> {
        // we copy story_preview_id here because it will be moved into the `on_click` event
        // listener in the links to the comments.
        //
        // This is needed since on_click requires an `Fn` where it needs to take variables from
        // it's environment that can last a lifetime of 'static. Therefore we need to create a copy
        // of dynamic variables and move it.
        let story_preview_id = story_preview.id;
        let story_preview_by = story_preview.by.clone();
        node! {
            <div key=story_preview_id class="story-preview">
                <div class="buttons">
                    <a>{safe_html("&#9650;")}</a>
                    <a>{safe_html("&#9660;")}</a>
                </div>
                <div>
                    <h2>
                    {
                        if let Some(url) = &story_preview.url{
                            node!{
                                <a href=url target="_blank" rel="noopener noreferrer">{text(&story_preview.title)}</a>
                            }
                        }else{
                            text(&story_preview.title)
                        }
                    }
                    </h2>
                    <span class="story-details">
                        {  text!("{} points | ",story_preview.score) }
                        <a href=format!("/user/{}", story_preview.by)
                            on_click=move|e|{
                                e.prevent_default();
                                app::Msg::ShowUserPage(story_preview_by.clone())
                            }>
                            { text!(" by {}",story_preview.by) }
                        </a>
                        <span title="time">{ text!(" | {} ago |", crate::util::time_ago(story_preview.time)) }</span>
                        <a href=format!("/item/{}", story_preview.id)
                            on_click=move|e|{
                                e.prevent_default();
                                app::Msg::OpenStory(story_preview_id)
                            }>
                            { text!(" {} comments", story_preview.descendants) }
                        </a>
                    </span>
                </div>
            </div>
        }
    }

    fn view_story_page(&self, story_page: &StoryPageData) -> Node<app::Msg> {
        node! {
            <div key=story_page.id>
                { self.view_story_preview(&story_page.preview()) }
                <ul class="comment-component" key="story-comments">
                {
                    for comment in &story_page.comments{
                        self.view_comment(comment)
                    }
                }
                </ul>
            </div>
        }
    }

    fn view_comment(&self, comment: &Comment) -> Node<app::Msg> {
        let comment_id = comment.id;
        let comment_by = comment.by.clone();
        node! {
            <li key=comment.id class="comment-item">
                <div class="comment-details">
                    <a href=format!("/user/{}",comment.by)
                        on_click=move|e|{
                            e.prevent_default();
                            app::Msg::ShowUserPage(comment_by.clone())
                        }>{text(&comment.by)}
                    </a>
                    <a href=format!("/comment/{}",comment.id)
                        on_click=move|e|{
                            e.prevent_default();
                            app::Msg::ShowCommentPermalink(comment_id)
                        }>{text!(" {} ago", crate::util::time_ago(comment.time))}
                    </a>
                </div>
                <div class="comment">{ safe_html(&comment.text) }</div>
                <ul class="sub-comments">
                {
                    for sub in &comment.sub_comments{
                        node!{
                            <li>
                                {self.view_comment(sub)}
                            </li>
                        }
                    }
                }
                </ul>
            </li>
        }
    }
}
