use std::rc::Rc;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::MouseEvent;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};


#[derive(Props)]
pub struct RadioButtonProps {
    pub text: String,
    
    #[prop(default)]
    pub checked: Signal<bool>,
    
    #[prop(default)]
    pub disabled: bool,
    
    #[prop(default)]
    pub name: String,
    
    pub on_toggle: Rc<dyn Fn(bool)>,
}

#[component]
pub fn RadioButton(props: RadioButtonProps) -> View {
    let is_hover = create_signal(false);

    let container_style = r#"
        display: inline-flex;
        align-items: center;
        cursor: pointer;
        user-select: none;
        margin: 4px 0;
    "#.to_string();

    let disabled_container_style = container_style.clone() + r#"
        cursor: default;
        opacity: 0.6;
    "#;

    let radio_style = r#"
        appearance: none;
        width: 16px;
        height: 16px;
        border: 1px solid #8f8f8f;
        border-radius: 50%;
        background: #ffffff;
        margin: 0;
        cursor: pointer;
        outline: none;
        position: relative;
        flex-shrink: 0;
    "#.to_string();

    let radio_hover_style = radio_style.clone() + r#"
        border-color: #5a8dee;
    "#;

    let radio_checked_style = r#"
        appearance: none;
        width: 16px;
        height: 16px;
        border: 1px solid #377af5;
        border-radius: 50%;
        background: #ffffff;
        margin: 0;
        cursor: pointer;
        outline: none;
        position: relative;
        flex-shrink: 0;
    "#.to_string();

    let radio_disabled_style = radio_style.clone() + r#"
        background: #efefef;
        border-color: #bfbfbf;
        cursor: default;
    "#;

    let label_style = r#"
        margin-left: 6px;
        font-family: 'Segoe UI', 'DejaVu Sans', Arial, sans-serif;
        font-size: 13px;
        color: #000000;
    "#.to_string();

    let label_disabled_style = label_style.clone() + r#"
        color: #7f7f7f;
    "#;

    let on_toggle = props.on_toggle.clone();

    view! {
        label(
            style = if props.disabled {
                disabled_container_style
            } else {
                container_style
            },
            on:mouseenter = move |_| {
                if !props.disabled {
                    is_hover.set(true);
                }
            },
            on:mouseleave = move |_| is_hover.set(false)
        ) {
            input(
                r#type = "radio",
                name = props.name,
                checked = props.checked,
                disabled = props.disabled,
                style = move || {
                    if props.disabled {
                        radio_disabled_style.clone()
                    } else if props.checked.get() {
                        radio_checked_style.clone() + if is_hover.get() {
                            "border-color: #2a5fd4;"
                        } else {
                            ""
                        }
                    } else if is_hover.get() {
                        radio_hover_style.clone()
                    } else {
                        radio_style.clone()
                    }
                },
                on:change = move |e: Event| {
                    if !props.disabled {
                        let input = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                        let checked = input.checked();
                        (on_toggle)(checked);
                    }
                }
            )
            
            // Indicador central quando marcado
            (if props.checked.get() {
                view! {
                    span(style = r#"
                        position: absolute;
                        width: 8px;
                        height: 8px;
                        background: #377af5;
                        border-radius: 50%;
                        margin-left: 4px;
                        pointer-events: none;
                    "#) {}
                }
            } else {
                view! {}
            })
            
            span(
                style = if props.disabled {
                    label_disabled_style
                } else {
                    label_style
                }
            ) {
                (props.text)
            }
        }
    }
}