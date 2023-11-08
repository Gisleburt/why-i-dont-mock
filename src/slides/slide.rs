use dioxus::prelude::*;
use crate::impress::Step;

#[derive(Props)]
pub struct SlideProps<'a> {
    children: Element<'a>,
    name: &'a str,
    #[props(default = 0)]
    x: i32,
    #[props(default = 0)]
    y: i32,
}

pub fn Slide<'a>(cx: Scope<'a, SlideProps<'a>>) -> Element {
    cx.render(rsx!(
        Step {
            name: cx.props.name,
            x: cx.props.x,
            y: cx.props.y,
            &cx.props.children
        }
    ))
}
