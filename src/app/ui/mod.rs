use std::rc::Rc;
use sycamore::prelude::*;
use sycamore::web::events::{Event, MouseEvent};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

// --- UTILS & ENUMS ---

#[derive(Clone, Copy)]
pub enum Align { Start, Center, End, Stretch, Between }

impl Align {
    fn to_css(&self) -> &'static str {
        match self {
            Align::Start => "flex-start",
            Align::Center => "center",
            Align::End => "flex-end",
            Align::Stretch => "stretch",
            Align::Between => "space-between",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonKind { Primary, Secondary, Ghost, Destructive, Success }

#[derive(Clone, Copy, PartialEq)]
pub enum LabelKind { Header, Title, Body, Caption, Monospace }

#[derive(Clone, Copy, PartialEq)]
pub enum IconPosition { Left, Right }

#[derive(Clone, Copy, PartialEq)]
pub enum BadgeKind { Primary, Success, Warning, Danger, Info }

#[derive(Clone, Copy, PartialEq)]
pub enum TooltipPosition { Top, Bottom, Left, Right }

// --- CONTAINERS ---

#[derive(Props)]
pub struct WindowProps {
    pub title: &'static str,
    pub size: (u32, u32),
    pub children: Children,
    #[prop(default)]
    pub resizable: bool,
}

#[component]
pub fn Window(props: WindowProps) -> View {
    let children = props.children.call();
    let resize_class = if props.resizable { "resizable" } else { "" };
    
    view! {
        div(
            class=format!("window {}", resize_class),
            style=format!("width:{}px; height:{}px;",
            props.size.0, props.size.1)
        ) {
            div(class="window-titlebar") {
                div(class="window-title") { (props.title) }
                div(class="window-controls hbox") {
                    button(class="win-btn min", title="Minimizar") {}
                    button(class="win-btn max", title="Maximizar") {}
                    button(class="win-btn close", title="Fechar") {}
                }
            }
            div(class="window-content") { (children) }
        }
    }
}

#[derive(Props)]
pub struct BoxProps {
    #[prop(default = 0)]
    pub spacing: u8,
    #[prop(default = Align::Stretch)]
    pub align: Align,
    #[prop(default = "")]
    pub class: &'static str,
    #[prop(default = "")]
    pub style: &'static str,
    pub children: Children,
}

#[component]
pub fn VBox(props: BoxProps) -> View {
    let children = props.children.call();
    view! {
        div(
            class=format!("vbox {}", props.class),
            style=format!("gap: {}px; align-items: {}; {}", props.spacing, props.align.to_css(), props.style)
        ) { (children) }
    }
}

#[component]
pub fn HBox(props: BoxProps) -> View {
    let children = props.children.call();
    let justify = if matches!(props.align, Align::Between) { "justify-content: space-between;" } else { "" };
    let align_items = if matches!(props.align, Align::Between) { "center" } else { props.align.to_css() };

    view! {
        div(
            class=format!("hbox {}", props.class),
            style=format!("gap: {}px; align-items: {}; {} {}", props.spacing, align_items, justify, props.style)
        ) { (children) }
    }
}

// --- GRID LAYOUT (NOVO) ---
#[derive(Props)]
pub struct GridProps {
    pub columns: u8,
    #[prop(default = 16)]
    pub gap: u8,
    #[prop(default = "")]
    pub class: &'static str,
    pub children: Children,
}

#[component]
pub fn Grid(props: GridProps) -> View {
    let children = props.children.call();
    view! {
        div(
            class=format!("ui-grid {}", props.class),
            style=format!("grid-template-columns: repeat({}, 1fr); gap: {}px;", props.columns, props.gap)
        ) { (children) }
    }
}

#[component]
pub fn Separator() -> View { view! { hr(class="separator") } }

#[component]
pub fn Spacer() -> View { view! { div(style="flex-grow: 1;") } }

// --- CARD ---
#[derive(Props)]
pub struct CardProps {
    pub children: Children,
    #[prop(default)]
    pub class: &'static str,
    #[prop(default)]
    pub elevated: bool,
    #[prop(default)]
    pub interactive: bool,
}

#[component]
pub fn Card(props: CardProps) -> View {
    let children = props.children.call();
    let mut classes = format!("ui-card {}", props.class);
    if props.elevated { classes.push_str(" elevated"); }
    if props.interactive { classes.push_str(" interactive"); }
    
    view! {
        div(class=classes) { (children) }
    }
}

// --- PANEL COM HEADER (NOVO) ---
#[derive(Props)]
pub struct PanelProps {
    pub title: &'static str,
    pub children: Children,
    #[prop(default)]
    pub collapsible: bool,
}

#[component]
pub fn Panel(props: PanelProps) -> View {
    let children = props.children.call();
    let collapsed = create_signal(false);
    
    let toggle = move |_| {
        if props.collapsible {
            collapsed.set(!collapsed.get());
        }
    };
    
    view! {
        div(class="ui-panel") {
            div(class="panel-header", on:click=toggle) {
                HBox(spacing=8, align=Align::Between) {
                    Label(text=props.title.to_string(), kind=LabelKind::Title)
                    (if props.collapsible {
                        view! {
                            span(class="panel-chevron") {
                                (if collapsed.get() { "▶" } else { "▼" })
                            }
                        }
                    } else { view! {} })
                }
            }
            div(
                class="panel-content",
                style=if collapsed.get() { "display: none;" } else { "" }
            ) { 
                (children)
            }
        }
    }
}

// --- TABS ---
#[derive(Clone)]
pub struct TabItem {
    pub title: &'static str,
    pub view: Rc<dyn Fn() -> View>,
}

impl TabItem {
    pub fn new<F>(title: &'static str, renderer: F) -> Self
    where F: Fn() -> View + 'static {
        Self { title, view: Rc::new(renderer) }
    }
}

#[derive(Props)]
pub struct TabViewProps {
    pub tabs: Vec<TabItem>,
}

#[component]
pub fn TabView(props: TabViewProps) -> View {
    let active_idx = create_signal(0);
    let headers: Vec<(usize, &'static str)> = props.tabs.iter()
        .enumerate()
        .map(|(i, t)| (i, t.title))
        .collect();
    let headers_signal = create_signal(headers);

    view! {
        div(class="vbox", style="width: 100%; height: 100%;") {
            div(class="tab-header") {
                Indexed(
                    list=headers_signal,
                    view=move |(idx, label)| {
                        let is_active = move || if active_idx.get() == idx { "active" } else { "" };
                        view! {
                            button(
                                class=format!("tab-btn {}", is_active()),
                                on:click=move |_| active_idx.set(idx)
                            ) { (label) }
                        }
                    }
                )
            }
            div(class="tab-content") {
                (move || {
                    let idx = active_idx.get();
                    if let Some(tab) = props.tabs.get(idx) {
                        (tab.view)()
                    } else {
                        view! { "Erro: Aba não encontrada" }
                    }
                })
            }
        }
    }
}

// --- SIDEBAR LAYOUT (NOVO) ---
#[derive(Props)]
pub struct SidebarLayoutProps {
    pub sidebar: Children,
    pub content: Children,
    #[prop(default = 250)]
    pub sidebar_width: u32,
}

#[component]
pub fn SidebarLayout(props: SidebarLayoutProps) -> View {
    let sidebar = props.sidebar.call();
    let content = props.content.call();
    
    view! {
        div(class="sidebar-layout") {
            div(class="sidebar", style=format!("width: {}px;", props.sidebar_width)) {
                (sidebar)
            }
            div(class="sidebar-content") {
                (content)
            }
        }
    }
}

// --- INPUTS ---

#[derive(Props)]
pub struct TextFieldProps {
    pub value: Signal<String>,
    #[prop(default)]
    pub placeholder: &'static str,
    #[prop(default)]
    pub grow: bool,
    #[prop(default)]
    pub disabled: bool,
    #[prop(default)]
    pub password: bool,
}

#[component]
pub fn TextField(props: TextFieldProps) -> View {
    let on_input = move |e: Event| {
        let target: HtmlInputElement = e.target().unwrap().unchecked_into();
        props.value.set(target.value());
    };
    let style = if props.grow { "flex-grow: 1;" } else { "" };
    let input_type = if props.password { "password" } else { "text" };
    
    view! {
        input(
            class="textfield",
            r#type=input_type,
            placeholder=props.placeholder,
            prop:value=props.value.get_clone(),
            on:input=on_input,
            style=style,
            disabled=props.disabled
        )
    }
}

// --- TEXTAREA (NOVO) ---
#[derive(Props)]
pub struct TextAreaProps {
    pub value: Signal<String>,
    #[prop(default)]
    pub placeholder: &'static str,
    #[prop(default = 4)]
    pub rows: u8,
}

#[component]
pub fn TextArea(props: TextAreaProps) -> View {
    let on_input = move |e: Event| {
        let target: web_sys::HtmlTextAreaElement = e.target().unwrap().unchecked_into();
        props.value.set(target.value());
    };
    
    view! {
        textarea(
            class="textarea",
            placeholder=props.placeholder,
            rows=props.rows.to_string(),
            on:input=on_input
        ) { (props.value.get_clone()) }
    }
}

// --- BUTTON ---
#[derive(Props)]
pub struct ButtonProps {
    pub text: &'static str,
    #[prop(default = ButtonKind::Primary)]
    pub kind: ButtonKind,
    pub on_click: Box<dyn Fn(MouseEvent)>,
    #[prop(default)]
    pub disabled: bool,
    #[prop(default)]
    pub icon: &'static str,
}

#[component]
pub fn Button(props: ButtonProps) -> View {
    let kind_class = match props.kind {
        ButtonKind::Primary => "btn-primary",
        ButtonKind::Secondary => "btn-secondary",
        ButtonKind::Ghost => "btn-ghost",
        ButtonKind::Destructive => "btn-destructive",
        ButtonKind::Success => "btn-success",
    };
    
    view! {
        button(
            class=format!("btn {}", kind_class),
            on:click=props.on_click,
            disabled=props.disabled
        ) {
            (if !props.icon.is_empty() {
                view! { span(class="btn-icon") { (props.icon) } }
            } else { view! {} })
            (props.text)
        }
    }
}

// --- ICON BUTTON (NOVO) ---
#[derive(Props)]
pub struct IconButtonProps {
    pub icon_src: &'static str,
    pub on_click: Box<dyn Fn(MouseEvent)>,
    #[prop(default)]
    pub tooltip: &'static str,
    #[prop(default = "24px")]
    pub icon_size: &'static str,
    #[prop(default)]
    pub alt_text: &'static str,
}

#[component]
pub fn IconButton(props: IconButtonProps) -> View {
    view! {
        button(class="icon-btn", on:click=props.on_click, title=props.tooltip) {
            img(
                src=props.icon_src,
                alt=props.alt_text,
                width=props.icon_size,
                height=props.icon_size,
                class="icon-img"
            )
        }
    }
}

// --- TOGGLE ---
#[derive(Props)]
pub struct ToggleProps {
    pub checked: Signal<bool>,
    #[prop(default)]
    pub disabled: bool,
}

#[component]
pub fn Toggle(props: ToggleProps) -> View {
    let on_change = move |e: Event| {
        let target: HtmlInputElement = e.target().unwrap().unchecked_into();
        props.checked.set(target.checked());
    };
    
    view! {
        label(class="toggle-switch") {
            input(
                r#type="checkbox",
                style="display:none",
                prop:checked=props.checked.get(),
                on:change=on_change,
                disabled=props.disabled
            )
            span(class="slider-round")
        }
    }
}

// --- CHECKBOX (NOVO) ---
#[derive(Props)]
pub struct CheckboxProps {
    pub checked: Signal<bool>,
    pub label: &'static str,
}

#[component]
pub fn Checkbox(props: CheckboxProps) -> View {
    let on_change = move |e: Event| {
        let target: HtmlInputElement = e.target().unwrap().unchecked_into();
        props.checked.set(target.checked());
    };
    
    view! {
        label(class="checkbox-wrapper") {
            input(
                r#type="checkbox",
                class="checkbox",
                prop:checked=props.checked.get(),
                on:change=on_change
            )
            span(class="checkbox-label") { (props.label) }
        }
    }
}

// --- RADIO GROUP (NOVO) ---
#[derive(Props)]
pub struct RadioGroupProps {
    pub selected: Signal<usize>,
    pub options: Vec<&'static str>,
}


#[component]
pub fn RadioGroup(props: RadioGroupProps) -> View {
    view! {
        div(class="radio-group") {
            (props.options.iter().enumerate().map(|(index, label)| {
                let label = label.to_owned();
                let is_checked = move || props.selected.get() == index;
                view! {
                    label(class="radio-wrapper") {
                        input(
                            r#type="radio",
                            class="radio",
                            prop:checked=is_checked(),
                            on:change=move |_| props.selected.set(index)
                        )
                        span(class="radio-label") { (label) }
                    }
                }
            }).collect::<Vec<_>>())
        }
    }
}

// --- SLIDER ---
#[derive(Props)]
pub struct SliderProps {
    pub value: Signal<f64>,
    pub min: f64,
    pub max: f64,
    #[prop(default)]
    pub grow: bool,
    #[prop(default)]
    pub step: f64,
}

#[component]
pub fn Slider(props: SliderProps) -> View {
    let on_input = move |e: Event| {
        let target: HtmlInputElement = e.target().unwrap().unchecked_into();
        if let Ok(val) = target.value().parse::<f64>() {
            props.value.set(val);
        }
    };
    let style = if props.grow { "flex-grow: 1;" } else { "" };
    let step = if props.step > 0.0 { props.step.to_string() } else { "any".to_string() };

    view! {
        input(
            class="slider-range",
            r#type="range",
            min=props.min.to_string(),
            max=props.max.to_string(),
            step=step,
            prop:value=props.value.get(),
            on:input=on_input,
            style=style
        )
    }
}

// --- PROGRESS BAR ---
#[derive(Props)]
pub struct ProgressBarProps {
    pub value: Signal<f64>,
    #[prop(default)]
    pub show_label: bool,
    #[prop(default)]
    pub indeterminate: bool,
}

#[component]
pub fn ProgressBar(props: ProgressBarProps) -> View {
    let width = move || format!("width: {}%", props.value.get() * 100.0);
    let track_class = if props.indeterminate { "progress-track indeterminate" } else { "progress-track" };
    
    view! {
        div(class="vbox", style="gap: 4px; width: 100%") {
            (if props.show_label {
                view! {
                    div(class="hbox", style="justify-content: space-between") {
                        span(class="ui-label caption") { "Progresso" }
                        span(class="ui-label caption") { (format!("{:.0}%", props.value.get() * 100.0)) }
                    }
                }
            } else { view!{} })
            div(class=track_class) {
                (if !props.indeterminate {
                    view! { div(class="progress-fill", style=width()) }
                } else {
                    view! { div(class="progress-fill-indeterminate") }
                })
            }
        }
    }
}

// --- SPINNER (NOVO) ---
#[derive(Props)]
pub struct SpinnerProps {
    #[prop(default = 24)]
    pub size: u16,
}

#[component]
pub fn Spinner(props: SpinnerProps) -> View {
    view! {
        div(
            class="spinner",
            style=format!("width: {}px; height: {}px;", props.size, props.size)
        )
    }
}

// --- BADGE (NOVO) ---
#[derive(Props)]
pub struct BadgeProps {
    pub text: String,
    #[prop(default = BadgeKind::Primary)]
    pub kind: BadgeKind,
}

#[component]
pub fn Badge(props: BadgeProps) -> View {
    let kind_class = match props.kind {
        BadgeKind::Primary => "badge-primary",
        BadgeKind::Success => "badge-success",
        BadgeKind::Warning => "badge-warning",
        BadgeKind::Danger => "badge-danger",
        BadgeKind::Info => "badge-info",
    };
    
    view! {
        span(class=format!("badge {}", kind_class)) { (props.text) }
    }
}

// --- MENU / DROPDOWN (NOVO) ---
#[derive(Props)]
pub struct MenuProps {
    pub items: Vec<&'static str>,
    pub on_select: Rc<dyn Fn(usize)>,
}
#[component]
pub fn Menu(props: MenuProps) -> View {
    view! {
        div(class="menu") {
            (props.items.iter().enumerate().map(|(idx, item)| {
                let item = item.to_owned();
                let on_select = props.on_select.clone();
                view! {
                    button(
                        class="menu-item",
                        on:click=move |_| on_select(idx)
                    ) { (item) }
                }
            }).collect::<Vec<_>>())
        }
    }
}

// --- LABEL ---
#[derive(Props)]
pub struct LabelProps {
    pub text: String,
    #[prop(default = LabelKind::Body)]
    pub kind: LabelKind,
}

#[component]
pub fn Label(props: LabelProps) -> View {
    let class = match props.kind {
        LabelKind::Header => "ui-label header",
        LabelKind::Title => "ui-label title",
        LabelKind::Body => "ui-label body",
        LabelKind::Caption => "ui-label caption",
        LabelKind::Monospace => "ui-label monospace",
    };
    view! { span(class=class) { (props.text) } }
}

// --- TOOLTIP (NOVO - Simplificado) ---
#[derive(Props)]
pub struct TooltipProps {
    pub text: &'static str,
    pub children: Children,
}

#[component]
pub fn Tooltip(props: TooltipProps) -> View {
    let children = props.children.call();
    view! {
        div(class="tooltip-wrapper") {
            (children)
            span(class="tooltip-text") { (props.text) }
        }
    }
}

// --- DIVIDER COM TEXTO (NOVO) ---
#[derive(Props)]
pub struct DividerProps {
    #[prop(default)]
    pub text: &'static str,
}

#[component]
pub fn Divider(props: DividerProps) -> View {
    if props.text.is_empty() {
        view! { hr(class="separator") }
    } else {
        view! {
            div(class="divider-with-text") {
                span(class="divider-text") { (props.text) }
            }
        }
    }
}

// --- TOOLBAR (NOVO) ---
#[derive(Props)]
pub struct ToolbarProps {
    pub children: Children,
}

#[component]
pub fn Toolbar(props: ToolbarProps) -> View {
    let children = props.children.call();
    view! {
        div(class="toolbar") {
            (children)
        }
    }
}

// --- STATUSBAR (NOVO) ---
#[derive(Props)]
pub struct StatusBarProps {
    pub children: Children,
}

#[component]
pub fn StatusBar(props: StatusBarProps) -> View {
    let children = props.children.call();
    view! {
        div(class="statusbar") {
            (children)
        }
    }
}