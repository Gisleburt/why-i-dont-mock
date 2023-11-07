use dioxus::prelude::*;

#[derive(Props)]
pub struct TypescriptProps<'a> {
    children: Element<'a>,
}


pub fn Typescript<'a>(cx: Scope<'a, TypescriptProps<'a>>) -> Element {
    cx.render(rsx!(
        pre { class: "theme-ally-dark",
            code { class:"language-typescript",
                &cx.props.children
            }
        }
    ))
}
