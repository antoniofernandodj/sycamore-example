use std::rc::Rc;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::MouseEvent;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};



#[derive(Props)]
pub struct VBoxLayoutProps {
    children: Children,
    
    #[prop(default = 0)]
    pub spacing: i32,
    
    #[prop(default = 0)]
    pub margin: i32,
}

#[component]
pub fn VBoxLayout(props: VBoxLayoutProps) -> View {
    let style = format!(
        r#"
        display: flex;
        flex-direction: column;
        gap: {}px;
        padding: {}px;
        box-sizing: border-box;
        "#,
        props.spacing,
        props.margin
    );

    view! {
        div(style = style) {
            (props.children)
        }
    }
}

