use std::rc::Rc;
use sycamore::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};
use rand;

#[derive(Props)]
pub struct ComboBoxProps {
    pub items: Vec<String>,
    #[prop(default)]
    pub current_index: Signal<i32>,
    #[prop(default)]
    pub disabled: bool,
    #[prop(default)]
    pub editable: bool,
    pub on_change: Rc<dyn Fn(i32, String)>,
}

#[component]
pub fn ComboBox(props: ComboBoxProps) -> View {
    let is_hover = create_signal(false);
    let is_focus = create_signal(false);
    let custom_value = create_signal(String::new());

    let container_style = r#"
        position: relative;
        display: inline-block;
        width: 200px;
    "#;

    let select_style = r#"
        box-sizing: border-box;
        width: 100%;
        height: 26px;
        padding: 4px 24px 4px 6px;
        background: linear-gradient(to bottom, #ffffff, #f6f6f6);
        border: 1px solid #8f8f8f;
        border-radius: 3px;
        font-family: 'Segoe UI', 'DejaVu Sans', Arial, sans-serif;
        font-size: 13px;
        color: #000000;
        outline: none;
        cursor: pointer;
        appearance: none;
    "#;

    let select_hover_style = format!("{}\n        border-color: #5a8dee;\n        background: linear-gradient(to bottom, #ffffff, #fafafa);", select_style);

    let select_focus_style = format!("{}\n        border-color: #377af5;\n        box-shadow: inset 0 0 0 1px rgba(55,122,245,0.6);", select_style);

    let select_disabled_style = format!("{}\n        background: #efefef;\n        color: #7f7f7f;\n        border-color: #bfbfbf;\n        cursor: default;", select_style);

    let arrow_style = r#"
        position: absolute;
        right: 6px;
        top: 50%;
        transform: translateY(-50%);
        width: 0;
        height: 0;
        border-left: 4px solid transparent;
        border-right: 4px solid transparent;
        border-top: 5px solid #000000;
        pointer-events: none;
    "#;

    let arrow_disabled_style = format!("{}\n        border-top-color: #7f7f7f;", arrow_style);

    let input_style = r#"
        box-sizing: border-box;
        width: 100%;
        height: 26px;
        padding: 4px 24px 4px 6px;
        background: #ffffff;
        border: 1px solid #8f8f8f;
        border-radius: 3px;
        font-family: 'Segoe UI', 'DejaVu Sans', Arial, sans-serif;
        font-size: 13px;
        color: #000000;
        outline: none;
    "#;

    let input_hover_style = format!("{}\n        border-color: #5a8dee;", input_style);

    let input_focus_style = format!("{}\n        border-color: #377af5;\n        box-shadow: inset 0 0 0 1px rgba(55,122,245,0.6);", input_style);

    let on_change = props.on_change.clone();

    if props.editable {
        let on_change_input = on_change.clone();
        let datalist_id = format!("combo-datalist-{}", rand::random::<u32>());
        
        view! {
            div(style = container_style) {
                input(
                    r#type = "text",
                    value = custom_value.get_clone(),
                    disabled = props.disabled,
                    list = datalist_id.clone(),
                    style = move || {
                        if props.disabled {
                            select_disabled_style.clone()
                        } else if is_focus.get() {
                            input_focus_style.clone()
                        } else if is_hover.get() {
                            input_hover_style.clone()
                        } else {
                            input_style.to_string()
                        }
                    },
                    on:mouseenter = move |_| is_hover.set(true),
                    on:mouseleave = move |_| is_hover.set(false),
                    on:focus = move |_| is_focus.set(true),
                    on:blur = move |_| is_focus.set(false),
                    on:input = move |e: Event| {
                        if !props.disabled {
                            let input = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            let value = input.value();
                            custom_value.set(value.clone());
                            (on_change_input)(-1, value);
                        }
                    }
                )
                
                datalist(id = datalist_id) {
                    {
                        View::new_fragment(
                            props.items.iter().map(|item| {
                                let item = item.clone();
                                view! {
                                    option(value = item)
                                }
                            }).collect::<Vec<_>>()
                        )
                    }
                }
                
                div(
                    style = if props.disabled {
                        arrow_disabled_style.as_str()
                    } else {
                        arrow_style
                    }
                )
            }
        }
    } else {
        view! {
            div(style = container_style) {
                select(
                    disabled = props.disabled,
                    style = move || {
                        if props.disabled {
                            select_disabled_style.clone()
                        } else if is_focus.get() {
                            select_focus_style.clone()
                        } else if is_hover.get() {
                            select_hover_style.clone()
                        } else {
                            select_style.to_string()
                        }
                    },
                    on:mouseenter = move |_| is_hover.set(true),
                    on:mouseleave = move |_| is_hover.set(false),
                    on:focus = move |_| is_focus.set(true),
                    on:blur = move |_| is_focus.set(false),
                    on:change = move |e: Event| {
                        if !props.disabled {
                            let select = e.target().unwrap().dyn_into::<web_sys::HtmlSelectElement>().unwrap();
                            let index = select.selected_index();
                            let value = select.value();
                            (on_change)(index, value);
                        }
                    }
                ) {
                    {
                        View::new_fragment(
                            props.items.iter().enumerate().map(|(i, item)| {
                                let item = item.clone();
                                let i = i as i32;
                                let selected = props.current_index.get() == i;
                                view! {
                                    option(value = item.clone(), selected = selected) {
                                        (item)
                                    }
                                }
                            }).collect::<Vec<_>>()
                        )
                    }
                }
                
                div(
                    style = if props.disabled {
                        arrow_disabled_style.as_str()
                    } else {
                        arrow_style
                    }
                )
            }
        }
    }
}