use std::rc::Rc;

use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::MouseEvent;

use wasm_bindgen::JsCast;

use web_sys::{Event, HtmlInputElement};


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}


#[component]
pub fn App() -> View {
    let text = create_signal(String::new());
    let label_text = create_signal(String::from("Nome:"));

    let text_for_lineedit = text.clone();
    let text_for_button = text.clone();
    let label_for_button = label_text.clone();

    view! {

            RLabel(
                text = label_text.clone(),
                for_id = "name".to_string()
            )

            LineEdit(
                value = text_for_lineedit,
                placeholder = "Digite algo...".to_string(),
                on_input = Rc::new({
                    move |v| {
                        text_for_lineedit.set(v);
                    }
                })
            )

            br {}

            PushButton(
                text = "OK".to_string(),
                on_click = Rc::new(move |_| {
                    label_for_button.set(text_for_button.get_clone());
                })
            )

    }
}




#[derive(Props)]
pub struct ButtonProps {
    pub text: String,
    #[prop(default)]
    pub disabled: bool,
    pub on_click: Rc<dyn Fn(MouseEvent)>,
}

#[component]
pub fn PushButton(props: ButtonProps) -> View {
    let is_hover = create_signal(false);
    let is_pressed = create_signal(false);

    let base_style = r#"
        background: linear-gradient(to bottom, #f6f6f6, #dcdcdc);
        border: 1px solid #8f8f8f;
        border-radius: 4px;
        padding: 6px 14px;
        min-height: 28px;

        font-family: 'Segoe UI', 'DejaVu Sans', Arial, sans-serif;
        font-size: 13px;
        color: #000000;

        box-shadow: inset 0 1px 0 rgba(255,255,255,0.8);
        cursor: pointer;
        outline: none;
        user-select: none;
    "#.to_string();

    let hover_style = base_style.clone() + r#"
        background: linear-gradient(to bottom, #ffffff, #e6e6e6);
        border: 1px solid #5a8dee;
    "#;

    let pressed_style = base_style.clone() + r#"
        background: linear-gradient(to bottom, #d6d6d6, #f0f0f0);
        border: 1px solid #5a8dee;
        box-shadow: inset 0 2px 4px rgba(0,0,0,0.15);
    "#;

    let disabled_style = base_style.clone() + r#"
        background: #efefef;
        border: 1px solid #bfbfbf;
        color: #7f7f7f;
        cursor: default;
        box-shadow: none;
    "#;

    let on_click = props.on_click.clone();


    view! {
        button(
            disabled = props.disabled,
            style = move || {
                if props.disabled {
                    disabled_style.clone()
                } else if is_pressed.get() {
                    pressed_style.clone()
                } else if is_hover.get() {
                    hover_style.clone()
                } else {
                    base_style.clone()
                }
            },
            on:mouseenter = move |_| is_hover.set(true),
            on:mouseleave = move |_| {
                is_hover.set(false);
                is_pressed.set(false);
            },
            on:mousedown = move |_| is_pressed.set(true),
            on:mouseup = move |_| is_pressed.set(false),
            on:click = move |e| {
                if !props.disabled {
                    (on_click)(e);
                }
            }
        ) {
            (props.text)
        }
    }
}




#[derive(Props)]
pub struct LineEditProps {
    #[prop(default)]
    pub value: Signal<String>,

    #[prop(default)]
    pub placeholder: String,

    #[prop(default)]
    pub disabled: bool,

    pub on_input: Rc<dyn Fn(String)>,
}

#[component]
pub fn LineEdit(props: LineEditProps) -> View {
    let is_hover = create_signal(false);
    let is_focus = create_signal(false);

    let base_style = r#"
        box-sizing: border-box;
        height: 26px;
        padding: 4px 6px;

        background: #ffffff;
        border: 1px solid #8f8f8f;
        border-radius: 3px;

        font-family: 'Segoe UI', 'DejaVu Sans', Arial, sans-serif;
        font-size: 13px;
        color: #000000;

        outline: none;
    "#.to_string();

    let hover_style = base_style.clone() + r#"
        border-color: #5a8dee;
    "#;

    let focus_style = base_style.clone() + r#"
        border-color: #377af5;
        box-shadow: inset 0 0 0 1px rgba(55,122,245,0.6);
    "#;

    let disabled_style = base_style.clone() + r#"
        background: #efefef;
        color: #7f7f7f;
        border-color: #bfbfbf;
    "#;

    let on_input = props.on_input.clone();



    view! {
        input(
            r#type = "text",
            value = props.value,
            placeholder = props.placeholder,
            disabled = props.disabled,

            style = move || {
                if props.disabled {
                    disabled_style.clone()
                } else if is_focus.get() {
                    focus_style.clone()
                } else if is_hover.get() {
                    hover_style.clone()
                } else {
                    base_style.clone()
                }
            },

            on:mouseenter = move |_| is_hover.set(true),
            on:mouseleave = move |_| is_hover.set(false),
            on:focus = move |_| is_focus.set(true),
            on:blur = move |_| is_focus.set(false),

            on:input = move |e: Event| {
                let input = e
                    .target()
                    .unwrap()
                    .dyn_into::<HtmlInputElement>()
                    .unwrap();

                let value = input.value();
                (on_input)(value);
            }
        )
    }
}

#[derive(Props)]
pub struct RLabelProps {
    pub text: Signal<String>,

    #[prop(default)]
    pub disabled: bool,

    #[prop(default)]
    pub for_id: String,
}


#[component]
pub fn RLabel(props: RLabelProps) -> View {
    let base_style = r#"
        display: inline-block;
        margin-right: 6px;

        font-family: 'Segoe UI', 'DejaVu Sans', Arial, sans-serif;
        font-size: 13px;
        color: #000000;

        user-select: none;
    "#.to_string();

    let disabled_style = base_style.clone() + r#"
        color: #7f7f7f;
    "#;

    let for_attr: Option<String> =
        if props.for_id.is_empty() {
            None
        } else {
            Some(props.for_id.clone())
        };

    view! {
        label(
            r#for = for_attr,
            style = if props.disabled {
                disabled_style
            } else {
                base_style
            }
        ) {
            (props.text)
        }
    }
}


// 1. Widgets Básicos (Controles Simples)
#[derive(Props)] pub struct QToolButtonProps {}
#[component] pub fn QToolButton(_: QToolButtonProps) -> View { view! {} }

#[derive(Props)] pub struct QCheckBoxProps {}
#[component] pub fn QCheckBox(_: QCheckBoxProps) -> View { view! {} }

#[derive(Props)] pub struct QRadioButtonProps {}
#[component] pub fn QRadioButton(_: QRadioButtonProps) -> View { view! {} }

#[derive(Props)] pub struct QTextEditProps {}
#[component] pub fn QTextEdit(_: QTextEditProps) -> View { view! {} }

#[derive(Props)] pub struct QPlainTextEditProps {}
#[component] pub fn QPlainTextEdit(_: QPlainTextEditProps) -> View { view! {} }

#[derive(Props)] pub struct QSpinBoxProps {}
#[component] pub fn QSpinBox(_: QSpinBoxProps) -> View { view! {} }

#[derive(Props)] pub struct QDoubleSpinBoxProps {}
#[component] pub fn QDoubleSpinBox(_: QDoubleSpinBoxProps) -> View { view! {} }

#[derive(Props)] pub struct QDateEditProps {}
#[component] pub fn QDateEdit(_: QDateEditProps) -> View { view! {} }

#[derive(Props)] pub struct QTimeEditProps {}
#[component] pub fn QTimeEdit(_: QTimeEditProps) -> View { view! {} }

#[derive(Props)] pub struct QDateTimeEditProps {}
#[component] pub fn QDateTimeEdit(_: QDateTimeEditProps) -> View { view! {} }

#[derive(Props)] pub struct QDialProps {}
#[component] pub fn QDial(_: QDialProps) -> View { view! {} }

#[derive(Props)] pub struct QSliderProps {}
#[component] pub fn QSlider(_: QSliderProps) -> View { view! {} }

#[derive(Props)] pub struct QScrollBarProps {}
#[component] pub fn QScrollBar(_: QScrollBarProps) -> View { view! {} }

#[derive(Props)] pub struct QLCDNumberProps {}
#[component] pub fn QLCDNumber(_: QLCDNumberProps) -> View { view! {} }

#[derive(Props)] pub struct QProgressBarProps {}
#[component] pub fn QProgressBar(_: QProgressBarProps) -> View { view! {} }


// 2. Widgets de Seleção
#[derive(Props)] pub struct QComboBoxProps {}
#[component] pub fn QComboBox(_: QComboBoxProps) -> View { view! {} }

#[derive(Props)] pub struct QFontComboBoxProps {}
#[component] pub fn QFontComboBox(_: QFontComboBoxProps) -> View { view! {} }

#[derive(Props)] pub struct QListWidgetProps {}
#[component] pub fn QListWidget(_: QListWidgetProps) -> View { view! {} }

#[derive(Props)] pub struct QTreeWidgetProps {}
#[component] pub fn QTreeWidget(_: QTreeWidgetProps) -> View { view! {} }

#[derive(Props)] pub struct QTableWidgetProps {}
#[component] pub fn QTableWidget(_: QTableWidgetProps) -> View { view! {} }

#[derive(Props)] pub struct QListViewProps {}
#[component] pub fn QListView(_: QListViewProps) -> View { view! {} }

#[derive(Props)] pub struct QTreeViewProps {}
#[component] pub fn QTreeView(_: QTreeViewProps) -> View { view! {} }

#[derive(Props)] pub struct QTableViewProps {}
#[component] pub fn QTableView(_: QTableViewProps) -> View { view! {} }

#[derive(Props)] pub struct QColumnViewProps {}
#[component] pub fn QColumnView(_: QColumnViewProps) -> View { view! {} }

#[derive(Props)] pub struct QUndoViewProps {}
#[component] pub fn QUndoView(_: QUndoViewProps) -> View { view! {} }


// 3. Containers e Organização de Layout
// Containers Visuais

#[derive(Props)] pub struct QGroupBoxProps {}
#[component] pub fn QGroupBox(_: QGroupBoxProps) -> View { view! {} }

#[derive(Props)] pub struct QFrameProps {}
#[component] pub fn QFrame(_: QFrameProps) -> View { view! {} }

#[derive(Props)] pub struct QTabWidgetProps {}
#[component] pub fn QTabWidget(_: QTabWidgetProps) -> View { view! {} }

#[derive(Props)] pub struct QStackedWidgetProps {}
#[component] pub fn QStackedWidget(_: QStackedWidgetProps) -> View { view! {} }

#[derive(Props)] pub struct QToolBoxProps {}
#[component] pub fn QToolBox(_: QToolBoxProps) -> View { view! {} }

#[derive(Props)] pub struct QScrollAreaProps {}
#[component] pub fn QScrollArea(_: QScrollAreaProps) -> View { view! {} }

#[derive(Props)] pub struct QSplitterProps {}
#[component] pub fn QSplitter(_: QSplitterProps) -> View { view! {} }

#[derive(Props)] pub struct QMdiAreaProps {}
#[component] pub fn QMdiArea(_: QMdiAreaProps) -> View { view! {} }

#[derive(Props)] pub struct QMdiSubWindowProps {}
#[component] pub fn QMdiSubWindow(_: QMdiSubWindowProps) -> View { view! {} }

#[derive(Props)] pub struct QDockWidgetProps {}
#[component] pub fn QDockWidget(_: QDockWidgetProps) -> View { view! {} }

// Layouts (não visuais)

pub struct QLayout;
pub struct QHBoxLayout;
pub struct QVBoxLayout;
pub struct QGridLayout;
pub struct QFormLayout;


// 4. Menus, Barras e Ações
#[derive(Props)] pub struct QMenuProps {}
#[component] pub fn QMenu(_: QMenuProps) -> View { view! {} }

#[derive(Props)] pub struct QMenuBarProps {}
#[component] pub fn QMenuBar(_: QMenuBarProps) -> View { view! {} }

#[derive(Props)] pub struct QToolBarProps {}
#[component] pub fn QToolBar(_: QToolBarProps) -> View { view! {} }

#[derive(Props)] pub struct QStatusBarProps {}
#[component] pub fn QStatusBar(_: QStatusBarProps) -> View { view! {} }

pub struct QAction;
pub struct QActionGroup;


// 5. Janelas Principais e Diálogos
#[derive(Props)] pub struct QMainWindowProps {}
#[component] pub fn QMainWindow(_: QMainWindowProps) -> View { view! {} }

#[derive(Props)] pub struct QDialogProps {}
#[component] pub fn QDialog(_: QDialogProps) -> View { view! {} }

#[derive(Props)] pub struct QDialogButtonBoxProps {}
#[component] pub fn QDialogButtonBox(_: QDialogButtonBoxProps) -> View { view! {} }

#[derive(Props)] pub struct QMessageBoxProps {}
#[component] pub fn QMessageBox(_: QMessageBoxProps) -> View { view! {} }

#[derive(Props)] pub struct QFileDialogProps {}
#[component] pub fn QFileDialog(_: QFileDialogProps) -> View { view! {} }

#[derive(Props)] pub struct QColorDialogProps {}
#[component] pub fn QColorDialog(_: QColorDialogProps) -> View { view! {} }

#[derive(Props)] pub struct QFontDialogProps {}
#[component] pub fn QFontDialog(_: QFontDialogProps) -> View { view! {} }

#[derive(Props)] pub struct QInputDialogProps {}
#[component] pub fn QInputDialog(_: QInputDialogProps) -> View { view! {} }

#[derive(Props)] pub struct QWizardProps {}
#[component] pub fn QWizard(_: QWizardProps) -> View { view! {} }

#[derive(Props)] pub struct QWizardPageProps {}
#[component] pub fn QWizardPage(_: QWizardPageProps) -> View { view! {} }

#[derive(Props)] pub struct QProgressDialogProps {}
#[component] pub fn QProgressDialog(_: QProgressDialogProps) -> View { view! {} }

#[derive(Props)] pub struct QErrorMessageProps {}
#[component] pub fn QErrorMessage(_: QErrorMessageProps) -> View { view! {} }


// 6. Widgets Avançados / Específicos
#[derive(Props)] pub struct QCalendarWidgetProps {}
#[component] pub fn QCalendarWidget(_: QCalendarWidgetProps) -> View { view! {} }

#[derive(Props)] pub struct QOpenGLWidgetProps {}
#[component] pub fn QOpenGLWidget(_: QOpenGLWidgetProps) -> View { view! {} }

#[derive(Props)] pub struct QVideoWidgetProps {}
#[component] pub fn QVideoWidget(_: QVideoWidgetProps) -> View { view! {} }

#[derive(Props)] pub struct QWebEngineViewProps {}
#[component] pub fn QWebEngineView(_: QWebEngineViewProps) -> View { view! {} }

#[derive(Props)] pub struct QGraphicsViewProps {}
#[component] pub fn QGraphicsView(_: QGraphicsViewProps) -> View { view! {} }

pub struct QGraphicsScene;
pub struct QGraphicsItem;

#[derive(Props)] pub struct QKeySequenceEditProps {}
#[component] pub fn QKeySequenceEdit(_: QKeySequenceEditProps) -> View { view! {} }

pub struct QUndoStack;
pub struct QUndoCommand;


// 7. Model / View
pub struct QAbstractItemView;
pub struct QAbstractListModel;
pub struct QAbstractTableModel;
pub struct QAbstractItemModel;
pub struct QStringListModel;
pub struct QStandardItemModel;
pub struct QStandardItem;


// 7. Model / View
pub struct QSystemTrayIcon;

#[derive(Props)] pub struct QSizeGripProps {}
#[component] pub fn QSizeGrip(_: QSizeGripProps) -> View { view! {} }

#[derive(Props)] pub struct QSplashScreenProps {}
#[component] pub fn QSplashScreen(_: QSplashScreenProps) -> View { view! {} }

#[derive(Props)] pub struct QRubberBandProps {}
#[component] pub fn QRubberBand(_: QRubberBandProps) -> View { view! {} }


// 9. Classes Base Importantes (Não Visuais)
pub struct QObject;
pub struct QApplication;
pub struct QGuiApplication;
pub struct QStyle;
pub struct QPalette;
pub struct QFont;
pub struct QIcon;
pub struct QPixmap;
pub struct QCursor;
