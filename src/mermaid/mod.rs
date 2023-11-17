use dioxus::prelude::*;
use indoc::indoc;

#[inline_props]
pub fn Mermaid<'a>(cx: Scope<'a>, children: Element<'a>) -> Element {
    cx.render(rsx!(
        pre { class: "mermaid", children }
    ))
}

pub fn MermaidInit(cx: Scope) -> Element {
    cx.render(rsx!(
        script { "type": "module",
            indoc! {"
                import mermaid from 'https://cdn.jsdelivr.net/npm/mermaid@10/dist/mermaid.esm.min.mjs';
                mermaid.initialize({ startOnLoad: false });
                mermaid.run();
            "}
        }
    ))
}
