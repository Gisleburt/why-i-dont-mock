use dioxus::prelude::*;

#[derive(Props)]
pub struct ImpressGroupProps<'a> {
    children: Element<'a>,
    #[props(default = 1000)]
    transition_duration: i32,
    #[props(default = 1920)]
    width: i32,
    #[props(default = 1080)]
    height: i32,
    #[props(default = 3)]
    max_scale: i32,
    #[props(default = 0)]
    min_scale: i32,
    #[props(default = 1000)]
    perspective: i32,
}

pub fn ImpressGroup<'a>(cx: Scope<'a, ImpressGroupProps<'a>>) -> Element {
    cx.render(rsx! (
        div {
            id: "impress",
            "data-transition-duration": "{cx.props.transition_duration}",
            "data-width": "{cx.props.width}",
            "data-height": "{cx.props.height}",
            "data-max-scale": "{cx.props.max_scale}",
            "data-min-scale": "{cx.props.min_scale}",
            "data-perspective": "{cx.props.perspective}",
            &cx.props.children
        }
    ))
}
