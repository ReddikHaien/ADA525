use yew::prelude::*;


#[derive(PartialEq, Properties)]
pub struct RawHtmlProps{
    pub content: String,
    pub inline: bool
}


#[function_component(RawHtml)]
pub fn raw_html(props: &RawHtmlProps) -> Html{

    let name = if props.inline{
        "span"
    }
    else{
        "div"
    };

    let div = gloo_utils::document().create_element(name).unwrap();
    if !props.inline{
        div.set_class_name("paragraph");
    }
    div.set_inner_html(&props.content);
    Html::VRef(div.into())
}