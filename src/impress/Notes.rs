use dioxus::prelude::*;

#[derive(Props)]
pub struct NotesProps<'a> {
    children: Element<'a>,
}

pub fn Notes<'a>(cx: Scope<'a, NotesProps<'a>>) -> Element {
    cx.render(rsx!(
        div { class: "notes", &cx.props.children }
    ))
}
