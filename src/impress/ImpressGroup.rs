use dioxus::prelude::*;

#[inline_props]
pub fn ImpressGroup<'a>(cx: Scope, children: Element<'a>) -> Element {
    cx.render(rsx! {
        div { id: "impress", children }
    })
}
