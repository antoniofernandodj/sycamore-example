use std::rc::Rc;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::MouseEvent;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};



#[derive(Props)]
pub struct GridLayoutProps {
    children: Children,
    
    #[prop(default = 2)]
    pub columns: usize,
    
    #[prop(default = 5)]
    pub spacing: i32,
    
    #[prop(default = 0)]
    pub margin: i32,
    
    #[prop(default)]
    pub column_widths: String, // ex: "1fr 2fr 1fr" ou "auto auto"
}

#[component]
pub fn GridLayout(props: GridLayoutProps) -> View {
    let template_columns = if props.column_widths.is_empty() {
        format!("repeat({}, 1fr)", props.columns)
    } else {
        props.column_widths.clone()
    };

    let style = format!(
        r#"
        display: grid;
        grid-template-columns: {};
        gap: {}px;
        padding: {}px;
        box-sizing: border-box;
        "#,
        template_columns,
        props.spacing,
        props.margin
    );

    view! {
        div(style = style) {
            (props.children)
        }
    }
}