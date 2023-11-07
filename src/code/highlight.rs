use dioxus::prelude::*;

pub fn HighlightInit(cx: Scope) -> Element {
    cx.render(rsx! { script { "hljs.highlightAll();" } })
}
