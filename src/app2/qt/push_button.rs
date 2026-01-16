use std::rc::Rc;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::MouseEvent;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};


#[derive(Props)]
pub struct PushButtonProps {
    pub text: String,
    #[prop(default)]
    pub disabled: bool,
    pub on_click: Rc<dyn Fn(MouseEvent)>,
}

#[component]
pub fn PushButton(props: PushButtonProps) -> View {
    let is_hover = create_signal(false);
    let is_pressed = create_signal(false);

    let base_style = r#"
        background: linear-gradient(to bottom, #f6f6f6, #dcdcdc);
        border: 1px solid #8f8f8f;
        border-radius: 4px;
        padding: 6px 14px;
        min-height: 28px;

        font-family: 'Segoe UI', 'DejaVu Sans', Arial, sans-serif;
        font-size: 13px;
        color: #000000;

        box-shadow: inset 0 1px 0 rgba(255,255,255,0.8);
        cursor: pointer;
        outline: none;
        user-select: none;
    "#.to_string();

    let hover_style = base_style.clone() + r#"
        background: linear-gradient(to bottom, #ffffff, #e6e6e6);
        border: 1px solid #5a8dee;
    "#;

    let pressed_style = base_style.clone() + r#"
        background: linear-gradient(to bottom, #d6d6d6, #f0f0f0);
        border: 1px solid #5a8dee;
        box-shadow: inset 0 2px 4px rgba(0,0,0,0.15);
    "#;

    let disabled_style = base_style.clone() + r#"
        background: #efefef;
        border: 1px solid #bfbfbf;
        color: #7f7f7f;
        cursor: default;
        box-shadow: none;
    "#;

    let on_click = props.on_click.clone();


    view! {
        button(
            disabled = props.disabled,
            style = move || {
                if props.disabled {
                    disabled_style.clone()
                } else if is_pressed.get() {
                    pressed_style.clone()
                } else if is_hover.get() {
                    hover_style.clone()
                } else {
                    base_style.clone()
                }
            },
            on:mouseenter = move |_| is_hover.set(true),
            on:mouseleave = move |_| {
                is_hover.set(false);
                is_pressed.set(false);
            },
            on:mousedown = move |_| is_pressed.set(true),
            on:mouseup = move |_| is_pressed.set(false),
            on:click = move |e| {
                if !props.disabled {
                    (on_click)(e);
                }
            }
        ) {
            (props.text)
        }
    }
}



