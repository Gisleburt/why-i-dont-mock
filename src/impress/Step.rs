use dioxus::prelude::*;

#[derive(Props)]
pub struct StepProps<'a> {
    children: Element<'a>,
    name: &'a str,
    #[props(default = 0)]
    x: i32,
    #[props(default = 0)]
    y: i32,
    #[props(default = 0)]
    z: i32,
    #[props(default = 0)]
    rotate: i32,
    #[props(default = 0)]
    rotate_x: i32,
    #[props(default = 0)]
    rotate_y: i32,
    #[props(default = 0)]
    rotate_z: i32,
    #[props(default = 1)]
    scale: i32,
}

pub fn Step<'a>(cx: Scope<'a, StepProps<'a>>) -> Element {
    cx.render(rsx!(
        div {
            class: "step",
            id: "{cx.props.name}",
            "data-x": "{cx.props.x}",
            "data-y": "{cx.props.y}",
            "data-z": "{cx.props.z}",
            "data-rotate": "{cx.props.rotate}",
            "data-rotate-x": "{cx.props.rotate_x}",
            "data-rotate-y": "{cx.props.rotate_y}",
            "data-rotate-z": "{cx.props.rotate_z}",
            "data-scale": "{cx.props.scale}",
            &cx.props.children
        }
    ))
}
