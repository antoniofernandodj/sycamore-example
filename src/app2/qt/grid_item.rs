use std::rc::Rc;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::MouseEvent;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};



#[derive(Props)]
pub struct GridItemProps {
    children: Children,
    
    #[prop(default = 0)]
    pub row: i32,
    
    #[prop(default = 0)]
    pub col: i32,
    
    #[prop(default = 1)]
    pub row_span: i32,
    
    #[prop(default = 1)]
    pub col_span: i32,
}

#[component]
pub fn GridItem(props: GridItemProps) -> View {
    let style = if props.row > 0 || props.col > 0 {
        format!(
            r#"
            grid-row: {} / span {};
            grid-column: {} / span {};
            "#,
            if props.row > 0 { props.row.to_string() } else { "auto".to_string() },
            props.row_span,
            if props.col > 0 { props.col.to_string() } else { "auto".to_string() },
            props.col_span
        )
    } else {
        String::new()
    };

    view! {
        div(style = style) {
            (props.children)
        }
    }
}
