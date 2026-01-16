use sycamore::prelude::*;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlTextAreaElement, FocusEvent, MouseEvent};

#[derive(Props)]
pub struct TextEditProps {
    pub value: Signal<String>,
    #[prop(default = String::new())]
    pub placeholder: String,
    #[prop(default = false)]
    pub readonly: bool,
    #[prop(default = 150)]
    pub height: i32,
    #[prop(default = false)]
    pub line_wrap: bool,
    pub on_change: Rc<dyn Fn(String)>,
}

#[component]
pub fn TextEdit(props: TextEditProps) -> View {
    let value = props.value;
    let placeholder = props.placeholder;
    let readonly = props.readonly;
    let height = props.height;
    let line_wrap = props.line_wrap;
    let on_change = props.on_change;
    
    let is_focused = create_signal(false);

    let handle_input = move |e: Event| {
        let target = e.target().unwrap();
        let textarea = target.dyn_into::<HtmlTextAreaElement>().unwrap();
        let new_value = textarea.value();
        value.set(new_value.clone());
        on_change(new_value);
    };

    let handle_focus = move |_e: FocusEvent| {
        is_focused.set(true);
    };

    let handle_blur = move |_e: FocusEvent| {
        is_focused.set(false);
    };

    let base_style = create_memo(move || {
        format!(
            "width: 100%; \
             height: {}px; \
             padding: 6px 8px; \
             font-family: 'Segoe UI', 'Consolas', monospace; \
             font-size: 13px; \
             color: #000; \
             background: {}; \
             border: 1px solid {}; \
             border-radius: 2px; \
             resize: vertical; \
             outline: none; \
             box-shadow: {}; \
             transition: border-color 0.15s ease, box-shadow 0.15s ease; \
             {}",
            height,
            if readonly { "#f0f0f0" } else { "#ffffff" },
            if is_focused.get() { "#5c9fdb" } else { "#a0a0a0" },
            if is_focused.get() { 
                "inset 0 1px 2px rgba(0,0,0,0.05), 0 0 0 2px rgba(92,159,219,0.2)" 
            } else { 
                "inset 0 1px 2px rgba(0,0,0,0.05)" 
            },
            if line_wrap { "white-space: pre-wrap; word-wrap: break-word;" } else { "white-space: pre; overflow-x: auto;" }
        )
    });

    view! {
        div(style = "position: relative; width: 100%;") {
            textarea(
                placeholder = placeholder,
                prop:value = value.get_clone(),
                readonly = readonly,
                on:input = handle_input,
                on:focus = handle_focus,
                on:blur = handle_blur,
                style = base_style.get_clone()
            )
        }
    }
}

// Variante com recursos extras
#[derive(Props)]
pub struct QTextEditRichProps {
    pub value: Signal<String>,
    #[prop(default = String::new())]
    pub placeholder: String,
    #[prop(default = false)]
    pub readonly: bool,
    #[prop(default = 200)]
    pub height: i32,
    #[prop(default = true)]
    pub line_wrap: bool,
    #[prop(default = true)]
    pub show_toolbar: bool,
    pub on_change: Rc<dyn Fn(String)>,
}

#[component]
pub fn QTextEditRich(props: QTextEditRichProps) -> View {
    let value = props.value;
    let placeholder = props.placeholder;
    let readonly = props.readonly;
    let height = props.height;
    let line_wrap = props.line_wrap;
    let show_toolbar = props.show_toolbar;
    let on_change = props.on_change.clone();
    
    let is_focused = create_signal(false);

    let handle_input = move |e: Event| {
        let target = e.target().unwrap();
        let textarea = target.dyn_into::<HtmlTextAreaElement>().unwrap();
        let new_value = textarea.value();
        value.set(new_value.clone());
        on_change(new_value);
    };

    let handle_focus = move |_e: FocusEvent| {
        is_focused.set(true);
    };

    let handle_blur = move |_e: FocusEvent| {
        is_focused.set(false);
    };

    // Ações da toolbar
    let insert_bold = {
        let value = value.clone();
        move |_e: MouseEvent| {
            let current = value.get_clone();
            value.set(format!("{}**texto em negrito**", current));
        }
    };

    let insert_italic = {
        let value = value.clone();
        move |_e: MouseEvent| {
            let current = value.get_clone();
            value.set(format!("{}*texto em itálico*", current));
        }
    };

    let clear_text = {
        let value = value.clone();
        move |_e: MouseEvent| {
            value.set(String::new());
        }
    };

    let base_style = create_memo(move || {
        format!(
            "width: 100%; \
             height: {}px; \
             padding: 8px; \
             font-family: 'Segoe UI', Arial, sans-serif; \
             font-size: 13px; \
             color: #000; \
             background: {}; \
             border: 1px solid {}; \
             border-top: {}; \
             border-radius: 0 0 2px 2px; \
             resize: vertical; \
             outline: none; \
             box-shadow: {}; \
             transition: border-color 0.15s ease; \
             {}",
            height,
            if readonly { "#f0f0f0" } else { "#ffffff" },
            if is_focused.get() { "#5c9fdb" } else { "#a0a0a0" },
            if show_toolbar { "none" } else { "1px solid #a0a0a0" },
            if is_focused.get() { 
                "inset 0 1px 2px rgba(0,0,0,0.05)" 
            } else { 
                "inset 0 1px 2px rgba(0,0,0,0.05)" 
            },
            if line_wrap { "white-space: pre-wrap; word-wrap: break-word;" } else { "white-space: pre; overflow-x: auto;" }
        )
    });

    view! {
        div(style = format!(
            "position: relative; width: 100%; border: 1px solid {}; border-radius: 2px; background: #fff;",
            if is_focused.get() { "#5c9fdb" } else { "#a0a0a0" }
        )) {
            (if show_toolbar {
                view! {
                    div(style = "display: flex; gap: 2px; padding: 4px; background: linear-gradient(to bottom, #f5f5f5, #e8e8e8); border-bottom: 1px solid #c0c0c0;") {
                        button(
                            on:click = insert_bold,
                            style = "padding: 4px 8px; background: linear-gradient(to bottom, #ffffff, #e0e0e0); border: 1px solid #a0a0a0; border-radius: 2px; cursor: pointer; font-weight: bold; font-size: 12px; min-width: 28px; height: 24px; display: flex; align-items: center; justify-content: center;",
                            title = "Negrito"
                        ) { "B" }
                        
                        button(
                            on:click = insert_italic,
                            style = "padding: 4px 8px; background: linear-gradient(to bottom, #ffffff, #e0e0e0); border: 1px solid #a0a0a0; border-radius: 2px; cursor: pointer; font-style: italic; font-size: 12px; min-width: 28px; height: 24px; display: flex; align-items: center; justify-content: center;",
                            title = "Itálico"
                        ) { "I" }
                        
                        div(style = "width: 1px; background: #c0c0c0; margin: 2px 4px;")
                        
                        button(
                            on:click = clear_text,
                            style = "padding: 4px 8px; background: linear-gradient(to bottom, #ffffff, #e0e0e0); border: 1px solid #a0a0a0; border-radius: 2px; cursor: pointer; font-size: 11px; color: #d00;",
                            title = "Limpar"
                        ) { "Limpar" }
                    }
                }
            } else {
                view! {}
            })
            
            textarea(
                placeholder = placeholder,
                prop:value = value.get_clone(),
                readonly = readonly,
                on:input = handle_input,
                on:focus = handle_focus,
                on:blur = handle_blur,
                style = base_style.get_clone()
            )
        }
    }
}