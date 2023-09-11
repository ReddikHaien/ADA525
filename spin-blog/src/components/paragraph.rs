use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ParagraphProps{
    pub children: Children
}

#[function_component]
pub fn Paragraph(props: &ParagraphProps) -> Html{
    html! {
        <div class={classes!("paragraph")}>
            { props.children.clone() }
        </div>
    }
}