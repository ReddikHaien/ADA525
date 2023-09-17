use std::{fmt::format, cell::RefCell, collections::HashMap};

use comrak::{Arena, ComrakOptions, arena_tree::Node, nodes::{Ast, NodeHtmlBlock}};
use gloo_net::http::Request;
use log::info;
use yew::{prelude::*, suspense::{use_future, SuspensionResult, Suspension}};

use crate::{components::{title::Title, paragraph::Paragraph, raw_html::RawHtml}, information};

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

    let (attributes, body) = read_attributes(&markdown);

    let arena = Arena::new();

    let ast = comrak::parse_document(&arena, &body, &ComrakOptions::default());

    let title = if let Some(title) = attributes.get("title"){
        title.to_string()
    }
    else{
        props.id.to_string()
    };


    Ok(html! {
    <>
        <Title title={title}/>
        <p><a href="#/">{"Back to home."}</a></p>
        {
            parse_markdown(ast)
        }
        {
            if let Some(timestamp) = attributes.get("timestamp"){
                html!{
                    <p>{timestamp}</p>
                }
            }
            else{
                html!{
                    <></>
                }
            }
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
        comrak::nodes::NodeValue::List(i) => {
            match i.list_type{
                comrak::nodes::ListType::Bullet => {
                    html!{
                        <ul>
                            {for children}
                        </ul>
                    }
                },
                comrak::nodes::ListType::Ordered => {
                    html!{
                        <ol>
                            {for children}
                        </ol>
                    }
                },
            }
        },
        comrak::nodes::NodeValue::Item(i) => {

            html!{
                <li>
                    {for children}
                </li>
            }
        },
        comrak::nodes::NodeValue::DescriptionList => todo!(),
        comrak::nodes::NodeValue::DescriptionItem(_) => todo!(),
        comrak::nodes::NodeValue::DescriptionTerm => todo!(),
        comrak::nodes::NodeValue::DescriptionDetails => todo!(),
        comrak::nodes::NodeValue::CodeBlock(_) => todo!(),
        comrak::nodes::NodeValue::HtmlBlock(i) => {
            html!{
                <RawHtml content={i.literal.clone()} inline={false}/>
            }
        },
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
                <Paragraph>
                {"FootNote: "}{a}
                </Paragraph>
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
        comrak::nodes::NodeValue::HtmlInline(i) => {
            html!{
                <RawHtml content={i.clone()} inline={true}/>
            }
        }
        comrak::nodes::NodeValue::Emph => {
            html! {
                <em>
                { for children }
                </em>
            }
        },
        comrak::nodes::NodeValue::Strong => {
            html!{
                <strong>
                    { for children }
                </strong>
            }
        },
        comrak::nodes::NodeValue::Strikethrough => {
            html!{
                <del>
                    { for children }
                </del>
            }
        },
        comrak::nodes::NodeValue::Superscript => todo!(),
        comrak::nodes::NodeValue::Link(l) => {
            html!{
                <a href={l.url.clone()}>
                    { &l.title } {for children }
                </a>
            }
        },
        comrak::nodes::NodeValue::Image(i) => {
            html!{
                <img src={i.url.clone()} alt={i.title.clone()}/>
            }
        },
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
    let url = format!("postsrenamed/{}.txt",id);
    
    Request::get(&url)
    .send().await?
    .text()
    .await
}

fn read_attributes<'a>(source: &'a str) -> (HashMap<&'a str, &'a str>, &'a str){
    let mut attributes = HashMap::new();

    let mut index = 0;
    let mut reading_attributes = false;
    for line in source.split_inclusive('\n'){
        index += line.len();
        let line = remove_newline(line);
        if line == "---"{
            if reading_attributes{
                break;
            }
            else{
                reading_attributes = true;
                continue;
            }
        }
        if reading_attributes{
            if let Some((key, value)) = line.split_once(':'){
                let key = key.trim();
                let value = value.trim().trim_matches('"');
                attributes.insert(key, value);
            }
        }
    }

    info!("attributes {:?}", attributes);

    (attributes, &source[index..])
}

fn remove_newline<'a>(s: &'a str) -> &'a str{
    let Some(s) = s.strip_suffix('\n') else {return s;};
    let Some(s) = s.strip_suffix('\r') else {return  s;};
    s
}