use dioxus::prelude::*;

pub fn ImpressInit(cx: Scope) -> Element {
    cx.render(rsx! { script { "impress().init()" } })
}
