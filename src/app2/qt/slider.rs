use sycamore::prelude::*;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};

#[derive(Props)]
pub struct QSliderProps {
    pub value: Signal<i32>,
    pub min: i32,
    pub max: i32,
    pub step: i32,
    #[prop(default = Orientation::Horizontal)]
    pub orientation: Orientation,
    #[prop(default = false)]
    pub show_value: bool,
    pub on_change: Rc<dyn Fn(i32)>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl Default for Orientation {
    fn default() -> Self {
        Orientation::Horizontal
    }
}

#[component]
pub fn Slider(props: QSliderProps) -> View {
    let value = props.value;
    let min = props.min;
    let max = props.max;
    let step = props.step;
    let orientation = props.orientation;
    let show_value = props.show_value;
    let on_change = props.on_change;

    let handle_input = move |e: Event| {
        let target = e.target().unwrap();
        let input = target.dyn_into::<HtmlInputElement>().unwrap();
        let new_value = input.value().parse::<i32>().unwrap_or(min);
        value.set(new_value);
        on_change(new_value);
    };

    let (container_style, wrapper_style) = match orientation {
        Orientation::Horizontal => (
            "position: relative; width: 100%; height: 32px; display: flex; align-items: center;",
            "display: flex; align-items: center; gap: 10px; width: 100%;",
        ),
        Orientation::Vertical => (
            "position: relative; width: 32px; height: 200px; display: flex; justify-content: center;",
            "display: flex; flex-direction: column; align-items: center; gap: 10px;",
        ),
    };

    view! {
        div(style = wrapper_style) {
            div(style = container_style) {
                // Track (fundo)
                div(style = if orientation == Orientation::Horizontal {
                    "position: absolute; width: 100%; height: 6px; background: linear-gradient(to bottom, #b0b0b0, #c8c8c8); border: 1px solid #8c8c8c; border-radius: 3px; box-shadow: inset 0 1px 2px rgba(0,0,0,0.1);"
                } else {
                    "position: absolute; width: 6px; height: 100%; background: linear-gradient(to right, #b0b0b0, #c8c8c8); border: 1px solid #8c8c8c; border-radius: 3px; box-shadow: inset 0 1px 2px rgba(0,0,0,0.1);"
                })
                
                // Filled track (parte preenchida)
                div(style = if orientation == Orientation::Horizontal {
                    format!(
                        "position: absolute; height: 6px; background: linear-gradient(to bottom, #5c9fdb, #4a8fd4); border-radius: 3px; width: {}%; transition: width 0.05s ease;",
                        ((value.get() - min) as f64 / (max - min) as f64 * 100.0).max(0.0).min(100.0)
                    )
                } else {
                    format!(
                        "position: absolute; width: 6px; background: linear-gradient(to right, #5c9fdb, #4a8fd4); border-radius: 3px; bottom: 0; height: {}%; transition: height 0.05s ease;",
                        ((value.get() - min) as f64 / (max - min) as f64 * 100.0).max(0.0).min(100.0)
                    )
                })
                
                // Thumb (círculo deslizante)
                div(style = if orientation == Orientation::Horizontal {
                    format!(
                        "position: absolute; width: 16px; height: 16px; background: linear-gradient(to bottom, #f0f0f0, #d8d8d8); border: 1px solid #8c8c8c; border-radius: 8px; cursor: pointer; left: calc({}% - 8px); box-shadow: 0 1px 3px rgba(0,0,0,0.2); transition: left 0.05s ease; pointer-events: none;",
                        ((value.get() - min) as f64 / (max - min) as f64 * 100.0).max(0.0).min(100.0)
                    )
                } else {
                    format!(
                        "position: absolute; width: 16px; height: 16px; background: linear-gradient(to right, #f0f0f0, #d8d8d8); border: 1px solid #8c8c8c; border-radius: 8px; cursor: pointer; bottom: calc({}% - 8px); box-shadow: 0 1px 3px rgba(0,0,0,0.2); transition: bottom 0.05s ease; pointer-events: none;",
                        ((value.get() - min) as f64 / (max - min) as f64 * 100.0).max(0.0).min(100.0)
                    )
                })
                
                // Input range invisível para controle
                input(
                    r#type = "range",
                    min = min.to_string(),
                    max = max.to_string(),
                    step = step.to_string(),
                    prop:value = value.get(),
                    on:input = handle_input,
                    style = "position: absolute; width: 100%; height: 100%; opacity: 0; cursor: pointer; margin: 0; z-index: 10;"
                )
            }
            
            (if show_value {
                view! {
                    span(style = "font-family: 'Segoe UI', Arial, sans-serif; font-size: 13px; color: #333; min-width: 40px; text-align: right;") {
                        (value.get().to_string())
                    }
                }
            } else {
                view! {}
            })
        }
    }
}