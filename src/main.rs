#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};
use log::{info, LevelFilter};
use std::fs;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    dioxus_desktop::launch_cfg(
        App,
        Config::default().with_window(
            WindowBuilder::new()
                .with_maximized(true)
                .with_title("DD"),
        ),
    );
}

fn App(cx: Scope) -> Element {
    let start_coords = use_state(&cx, || (0f32, 0f32));
    let end_coords = use_state(&cx, || (0f32, 0f32));
    let first_mousedown = use_state(&cx, || false);

    cx.render(rsx! {
        div {
            onmousemove: move |event| {
                end_coords.set((event.client_x as f32, event.client_y as f32));
            },
            onmousedown: move |event| {
                let mouse_x = event.client_x;
                let mouse_y = event.client_y;
                log::info!("Mouse position: x = {mouse_x}, y = {mouse_y}");
                start_coords.set((mouse_x as f32, mouse_y as f32));
                first_mousedown.set(true);
            },
            style: "position: absolute; top: 0; left: 0",
            width: "100%",
            height: "100%",
            svg {
                width: "100%",
                height: "100%",
                line {
                    x1: "{start_coords.0}",
                    y1: "{start_coords.1}",
                    x2: "{end_coords.0}",
                    y2: "{end_coords.1}",
                    stroke: "black",
                    stroke_width: "2",
                    style: if *first_mousedown.get() { "visibility: visible;" } else { "visibility: hidden;" },
                }
            },
        }
    })
}

