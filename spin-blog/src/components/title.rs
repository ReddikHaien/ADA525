use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct TitleProps{
    pub title: String
}

#[function_component]
pub fn Title(props: &TitleProps) -> Html{
    html! {
        <div class={classes!("title")}>
        <h1> { &props.title } </h1>
        <div></div>
        </div>
    }
}