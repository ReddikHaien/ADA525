use std::{fmt::format, cell::RefCell};

use comrak::{Arena, ComrakOptions, arena_tree::Node, nodes::Ast};
use gloo_net::http::Request;
use yew::{prelude::*, suspense::{use_future, SuspensionResult, Suspension}};

use crate::{components::{title::Title, paragraph::Paragraph}, information};

#[derive(PartialEq, Properties)]
pub struct PostProps{
    pub id: String
}

#[function_component]
pub fn Posts(props: &PostProps) -> Html{
    let fallback = html! {<div>{"Loading..."}</div>};


    html! {
        <Suspense {fallback}>
            <PostsContent id = { props.id.clone() }/>
        </Suspense>
    }
}

#[function_component]
pub fn PostsContent(props: &PostProps) -> HtmlResult{
    
    let markdown = use_post(props.id.clone())?;

    let arena = Arena::new();

    let ast = comrak::parse_document(&arena, &markdown, &ComrakOptions::default());

    Ok(html! {
    <>
        {
            parse_markdown(ast)
        }
    </>
    })
}

pub fn parse_markdown<'a>(node: &'a Node<'a, RefCell<Ast>>) -> Html{

    let mut children = Vec::new();
    for child in node.children(){
        children.push(parse_markdown(child));
    }

    match &node.data.borrow_mut().value {
        comrak::nodes::NodeValue::Document => {
            html! {
                <>
                    {for children }
                </>
            }
        },
        comrak::nodes::NodeValue::FrontMatter(_) => todo!(),
        comrak::nodes::NodeValue::BlockQuote => todo!(),
        comrak::nodes::NodeValue::List(_) => todo!(),
        comrak::nodes::NodeValue::Item(_) => todo!(),
        comrak::nodes::NodeValue::DescriptionList => todo!(),
        comrak::nodes::NodeValue::DescriptionItem(_) => todo!(),
        comrak::nodes::NodeValue::DescriptionTerm => todo!(),
        comrak::nodes::NodeValue::DescriptionDetails => todo!(),
        comrak::nodes::NodeValue::CodeBlock(_) => todo!(),
        comrak::nodes::NodeValue::HtmlBlock(_) => todo!(),
        comrak::nodes::NodeValue::Paragraph => {
            html!{
                <Paragraph>
                {for children}
                </Paragraph>
            }
        },
        comrak::nodes::NodeValue::Heading(v) => {
            match v.level {
                1 => html!{ <h1>{for children}</h1> },
                2 => html!{ <h2>{for children}</h2> },
                3 => html!{ <h3>{for children}</h3> },
                4 => html!{ <h4>{for children}</h4> },
                5 => html!{ <h5>{for children}</h5> },
                _ => html!{ <h6>{for children}</h6> },
            }
        }
        comrak::nodes::NodeValue::ThematicBreak => {
            html! {
                <hr />
            }
        },
        comrak::nodes::NodeValue::FootnoteDefinition(a) => {
            html! {
                <div>
                {"FootNote: "}{a}
                </div>
            }
        },
        comrak::nodes::NodeValue::Table(_) => todo!(),
        comrak::nodes::NodeValue::TableRow(_) => todo!(),
        comrak::nodes::NodeValue::TableCell => todo!(),
        comrak::nodes::NodeValue::Text(text) => {
            html! {
                <>{text}</>
            }
        },
        comrak::nodes::NodeValue::TaskItem(_) => todo!(),
        comrak::nodes::NodeValue::SoftBreak => html!{
            <>{" "}</>
        },
        comrak::nodes::NodeValue::LineBreak => todo!(),
        comrak::nodes::NodeValue::Code(_) => todo!(),
        comrak::nodes::NodeValue::HtmlInline(_) => todo!(),
        comrak::nodes::NodeValue::Emph => todo!(),
        comrak::nodes::NodeValue::Strong => todo!(),
        comrak::nodes::NodeValue::Strikethrough => todo!(),
        comrak::nodes::NodeValue::Superscript => todo!(),
        comrak::nodes::NodeValue::Link(_) => todo!(),
        comrak::nodes::NodeValue::Image(_) => todo!(),
        comrak::nodes::NodeValue::FootnoteReference(_) => todo!(),
    }
}


#[hook]
fn use_post(id: String) -> SuspensionResult<String>{
    let result = use_future(|| async { load_post(id).await })?;

    match *result {
        Ok(ref x) =>  {
            Ok(x.clone())
        },
        Err(ref e) => {
            panic!("Error: {}",e)
        }
    }
}


async fn load_post(id: String) -> Result<String, gloo_net::Error>{
    let url = format!("/posts/{}.md",id);
    
    Request::get(&url)
    .send().await?
    .text()
    .await
}