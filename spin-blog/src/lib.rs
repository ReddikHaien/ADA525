use wasm_bindgen::prelude::*;
use web_sys;

use percy_dom::prelude::*;

#[wasm_bindgen]
pub struct App {
  pdom: PercyDom
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new () -> App {
        let start_view = html! { <div> Hello </div> };

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let mut pdom = PercyDom::new_append_to_mount(start_view, &body);

        let greetings = "Hello, World!";

        let end_view = html! {
           // Use regular Rust comments within your html
           <div class=["big", "blue"]>
              /* Interpolate values using braces */
              <strong>{ greetings }</strong>

              <button
                class="giant-button"
                onclick=|_event| {
                   web_sys::console::log_1(&"Button Clicked!".into());
                }
              >
                // No need to wrap text in quotation marks (:
                Click me and check your console
              </button>
           </div>
        };

        pdom.update(end_view);

        App { pdom }
    }
}