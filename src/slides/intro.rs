use crate::impress::Notes;
use dioxus::prelude::*;

pub fn Intro(cx: Scope) -> Element {
    cx.render(rsx!(
        h1 { "Why I don't mock" }
        h2 { class: "daniel", "Daniel " span { "// Mason" } }
        Notes {
            p { "Hi, in case you don't know I'm Daniel and I love testing" }
            p { "But testing interconnected things gets complex" }
            p { "So I want to show you how I manage that complexity" }
        }
    ))
}
