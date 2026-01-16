use std::rc::Rc;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::MouseEvent;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};



#[derive(Props)]
pub struct LineEditProps {
    #[prop(default)]
    pub value: Signal<String>,

    #[prop(default)]
    pub placeholder: String,

    #[prop(default)]
    pub disabled: bool,

    pub on_input: Rc<dyn Fn(String)>,
}

#[component]
pub fn LineEdit(props: LineEditProps) -> View {
    let is_hover = create_signal(false);
    let is_focus = create_signal(false);

    let base_style = r#"
        box-sizing: border-box;
        height: 26px;
        padding: 4px 6px;

        background: #ffffff;
        border: 1px solid #8f8f8f;
        border-radius: 3px;

        font-family: 'Segoe UI', 'DejaVu Sans', Arial, sans-serif;
        font-size: 13px;
        color: #000000;

        outline: none;
    "#.to_string();

    let hover_style = base_style.clone() + r#"
        border-color: #5a8dee;
    "#;

    let focus_style = base_style.clone() + r#"
        border-color: #377af5;
        box-shadow: inset 0 0 0 1px rgba(55,122,245,0.6);
    "#;

    let disabled_style = base_style.clone() + r#"
        background: #efefef;
        color: #7f7f7f;
        border-color: #bfbfbf;
    "#;

    let on_input = props.on_input.clone();



    view! {
        input(
            r#type = "text",
            value = props.value,
            placeholder = props.placeholder,
            disabled = props.disabled,

            style = move || {
                if props.disabled {
                    disabled_style.clone()
                } else if is_focus.get() {
                    focus_style.clone()
                } else if is_hover.get() {
                    hover_style.clone()
                } else {
                    base_style.clone()
                }
            },

            on:mouseenter = move |_| is_hover.set(true),
            on:mouseleave = move |_| is_hover.set(false),
            on:focus = move |_| is_focus.set(true),
            on:blur = move |_| is_focus.set(false),

            on:input = move |e: Event| {
                let input = e
                    .target()
                    .unwrap()
                    .dyn_into::<HtmlInputElement>()
                    .unwrap();

                let value = input.value();
                (on_input)(value);
            }
        )
    }
}
