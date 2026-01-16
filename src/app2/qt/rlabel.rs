use std::rc::Rc;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::MouseEvent;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};


#[derive(Props)]
pub struct RLabelProps {
    pub text: Signal<String>,

    #[prop(default)]
    pub disabled: bool,

    #[prop(default)]
    pub for_id: String,
}


#[component]
pub fn RLabel(props: RLabelProps) -> View {
    let base_style = r#"
        display: inline-block;
        margin-right: 6px;

        font-family: 'Segoe UI', 'DejaVu Sans', Arial, sans-serif;
        font-size: 13px;
        color: #000000;

        user-select: none;
    "#.to_string();

    let disabled_style = base_style.clone() + r#"
        color: #7f7f7f;
    "#;

    let for_attr: Option<String> =
        if props.for_id.is_empty() {
            None
        } else {
            Some(props.for_id.clone())
        };

    view! {
        label(
            r#for = for_attr,
            style = if props.disabled {
                disabled_style
            } else {
                base_style
            }
        ) {
            (props.text)
        }
    }
}

