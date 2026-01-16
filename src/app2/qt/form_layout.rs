use std::rc::Rc;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::MouseEvent;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};


#[derive(Props)]
pub struct FormLayoutProps {
    children: Children,
    
    #[prop(default = 5)]
    pub spacing: i32,
    
    #[prop(default = 0)]
    pub margin: i32,
    
    #[prop(default = 10)]
    pub label_spacing: i32, // Espaçamento entre label e campo
}

#[component]
pub fn FormLayout(props: FormLayoutProps) -> View {
    let style = format!(
        r#"
        display: grid;
        grid-template-columns: auto 1fr;
        gap: {}px {}px;
        padding: {}px;
        box-sizing: border-box;
        align-items: center;
        "#,
        props.spacing,
        props.label_spacing,
        props.margin
    );

    view! {
        div(style = style) {
            (props.children)
        }
    }
}