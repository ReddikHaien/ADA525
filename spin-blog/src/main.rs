pub mod components;
pub mod pages;
use pages::{Home, post::Posts};
use yew_router::prelude::*;
use yew::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route{
  #[at("/")]
  Home,
  #[at("/posts/:id")]
  Post{ id: String },
}

fn switch(route: Route) -> Html{
    html! {
        <div class={classes!("main_view")}>
        {
            match route {
                Route::Home => html! { <Home /> },
                Route::Post { id } => html!{ <Posts id={ id } /> }
            }
        }
        </div>
    }
    
}

#[function_component]
fn App () -> Html {
    html! {
        <HashRouter>
            <Switch<Route> render={switch} />
        </HashRouter>
    }
}

fn main(){
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render(); 
}


mod information{
   include!(concat!(env!("OUT_DIR"), "/information.rs"));
}