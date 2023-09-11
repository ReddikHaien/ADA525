pub mod components;
pub mod pages;
use pages::Home;
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
    match route {
        Route::Home => html! { <Home /> },
        Route::Post { id } => todo!(),
    }
}

#[function_component]
fn App () -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main(){
    yew::Renderer::<App>::new().render(); 
}


mod information{
   include!(concat!(env!("OUT_DIR"), "/information.rs"));
}