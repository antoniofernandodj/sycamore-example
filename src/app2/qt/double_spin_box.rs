use std::rc::Rc;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::MouseEvent;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};



#[derive(Props)]
pub struct DoubleSpinBoxProps {
    #[prop(default)]
    pub value: Signal<f64>,
    
    #[prop(default = 0.0)]
    pub min: f64,
    
    #[prop(default = 99.99)]
    pub max: f64,
    
    #[prop(default = 0.1)]
    pub step: f64,
    
    #[prop(default = 2)]
    pub decimals: usize,
    
    #[prop(default)]
    pub disabled: bool,
    
    pub on_change: Rc<dyn Fn(f64)>,
}

#[component]
pub fn DoubleSpinBox(props: DoubleSpinBoxProps) -> View {
    let is_hover = create_signal(false);
    let is_focus = create_signal(false);

    let container_style = r#"
        display: inline-flex;
        align-items: center;
        position: relative;
        height: 26px;
    "#.to_string();

    let input_style = r#"
        box-sizing: border-box;
        width: 100px;
        height: 26px;
        padding: 4px 8px 4px 6px;
        background: #ffffff;
        border: 1px solid #8f8f8f;
        border-radius: 3px;
        font-family: 'Segoe UI', 'DejaVu Sans', Arial, sans-serif;
        font-size: 13px;
        color: #000000;
        outline: none;
    "#.to_string();

    let input_hover_style = input_style.clone() + r#"
        border-color: #5a8dee;
    "#;

    let input_focus_style = input_style.clone() + r#"
        border-color: #377af5;
        box-shadow: inset 0 0 0 1px rgba(55,122,245,0.6);
    "#;

    let input_disabled_style = input_style.clone() + r#"
        background: #efefef;
        color: #7f7f7f;
        border-color: #bfbfbf;
    "#;

    let buttons_container_style = r#"
        position: absolute;
        right: 2px;
        display: flex;
        flex-direction: column;
        height: 22px;
        top: 2px;
    "#.to_string();

    let button_style = r#"
        width: 18px;
        height: 11px;
        background: linear-gradient(to bottom, #f6f6f6, #dcdcdc);
        border: 1px solid #8f8f8f;
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 8px;
        user-select: none;
        padding: 0;
        margin: 0;
    "#.to_string();

    let button_hover_style = button_style.clone() + r#"
        background: linear-gradient(to bottom, #ffffff, #e6e6e6);
    "#;

    let button_disabled_style = button_style.clone() + r#"
        background: #efefef;
        border-color: #bfbfbf;
        color: #7f7f7f;
        cursor: default;
    "#;

    let on_change = props.on_change.clone();
    let on_change_up = on_change.clone();
    let on_change_down = on_change.clone();
    let on_change_input = on_change.clone();

    let up_hover = create_signal(false);
    let down_hover = create_signal(false);

    let format_value = move || {
        format!("{:.prec$}", props.value.get(), prec = props.decimals)
    };

    let button_disabled_style_1 = button_disabled_style.clone();
    let button_hover_style_1 = button_hover_style.clone();
    let button_style_1 = button_style.clone();

    view! {
        div(style = container_style) {
            input(
                r#type = "number",
                value = format_value(),
                step = props.step.to_string(),
                disabled = props.disabled,
                style = move || {
                    if props.disabled {
                        input_disabled_style.clone()
                    } else if is_focus.get() {
                        input_focus_style.clone()
                    } else if is_hover.get() {
                        input_hover_style.clone()
                    } else {
                        input_style.clone()
                    }
                },
                on:mouseenter = move |_| is_hover.set(true),
                on:mouseleave = move |_| is_hover.set(false),
                on:focus = move |_| is_focus.set(true),
                on:blur = move |_| is_focus.set(false),
                on:input = move |e: Event| {
                    if !props.disabled {
                        let input = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                        if let Ok(val) = input.value().parse::<f64>() {
                            let clamped = val.clamp(props.min, props.max);
                            (on_change_input)(clamped);
                        }
                    }
                }
            )
        }
    }
}
