#![allow(non_snake_case)]
mod components;

// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use crate::components::ImpressGroup::ImpressGroup;
use crate::components::ImpressInit::ImpressInit;
use crate::components::Step::Step;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        ImpressGroup {
            Step {
                name: "step-1",
                x: 0,
                "Step 1"
            }
            Step {
                name: "step-2",
                x: 1000,
                "Step 2"
            }
        }
        ImpressInit {}
    })
}
