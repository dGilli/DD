#![allow(non_snake_case)]
use dioxus::prelude::*;
use log::{info, LevelFilter};

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    let start_coords = use_state(&cx, || (0f32, 0f32));
    let end_coords = use_state(&cx, || (0f32, 0f32));

    cx.render(rsx! {
        div {
            style: "position: absolute; top: 0; left: 0",
            width: "100%",
            height: "100%",
            onmousemove: move |event| {
                end_coords.set((event.client_x as f32, event.client_y as f32));
            },
            onmousedown: move |event| {
                let mouse_x = event.client_x;
                let mouse_y = event.client_y;
                log::info!("Mouse position: x = {mouse_x}, y = {mouse_y}");
                start_coords.set((mouse_x as f32, mouse_y as f32));
            },
            svg {
                width: "100%",
                height: "100%",
                line {
                    x1: "{start_coords.0}",
                    y1: "{start_coords.1}",
                    x2: "{end_coords.0}",
                    y2: "{end_coords.1}",
                    stroke: "black",
                    stroke_width: "2"
                }
            }
        }
    })
}

