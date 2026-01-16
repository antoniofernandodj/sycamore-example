use std::rc::Rc;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::MouseEvent;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};



#[derive(Props)]
pub struct FormRowProps {
    pub label: String,
    children: Children,
}

#[component]
pub fn FormRow(props: FormRowProps) -> View {
    let label_style = r#"
        font-family: 'Segoe UI', 'DejaVu Sans', Arial, sans-serif;
        font-size: 13px;
        color: #000000;
        text-align: right;
        padding-right: 5px;
        white-space: nowrap;
    "#.to_string();

    view! {
        label(style = label_style) {
            (props.label)
        }
        div {
            (props.children)
        }
    }
}