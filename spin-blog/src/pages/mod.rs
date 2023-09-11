pub mod post;

use yew::prelude::*;

use crate::{components::{title::Title, paragraph::Paragraph}, information};


#[function_component]
pub fn Home() -> Html{
    let posts = information::posts()
    .iter()
    .map(|(name, title)| html! {
        <a href={ format!("posts/{}",name) }> { title } </a>
    });

    html! {
    <>
        <Title title = { "ADA525" }/>
        <Paragraph>
            {"This is my blog for documenting the semester project for ADA 525."}
        </Paragraph>
        {
            for posts
        }
    </>
    }
}
