use std::rc::Rc;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::MouseEvent;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};



#[derive(Props)]
pub struct HBoxLayoutProps {
    children: Children,
    
    #[prop(default = 0)]
    pub spacing: i32,
    
    #[prop(default = 0)]
    pub margin: i32,
    
    #[prop(default)]
    pub align: String, // "start", "center", "end", "stretch"
}

#[component]
pub fn HBoxLayout(props: HBoxLayoutProps) -> View {
    let align_items = match props.align.as_str() {
        "center" => "center",
        "end" => "flex-end",
        "stretch" => "stretch",
        _ => "flex-start",
    };

    let style = format!(
        r#"
        display: flex;
        flex-direction: row;
        gap: {}px;
        padding: {}px;
        align-items: {};
        box-sizing: border-box;
        "#,
        props.spacing,
        props.margin,
        align_items
    );

    view! {
        div(style = style) {
            (props.children)
        }
    }
}