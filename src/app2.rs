mod qt;  // Declarar o módulo
use qt::*;  // Importar tudo

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
    // Signals para os widgets
    let name = create_signal(String::new());
    let email = create_signal(String::new());
    let age = create_signal(18);
    let height = create_signal(1.75);
    let weight = create_signal(70.0);
    
    let radio1 = create_signal(true);
    let radio2 = create_signal(false);
    let radio3 = create_signal(false);
    
    let combo_index = create_signal(0);
    let status_text = create_signal(String::from("Pronto"));

    let combo_items = vec![
        "Bronze".to_string(),
        "Prata".to_string(),
        "Ouro".to_string(),
        "Platina".to_string(),
    ];

    let texto = create_signal(String::new());

    view! {

        VBoxLayout {

            HBoxLayout(spacing = 5, margin = 5) {
                TextEdit(
                    value = texto.clone(),
                    placeholder = "Digite seu texto aqui...".to_string(),
                    height = 150,
                    line_wrap = true,
                    readonly = false,
                    on_change = Rc::new(|novo_texto| {
                        // Faça algo quando o texto mudar
                        web_sys::console::log_1(&format!("Texto alterado: {}", novo_texto).into());
                    })
                )
            }
        }


        Slider(
            value = create_signal(50),
            min = 0,
            max = 100,
            step = 1,
            show_value = true,
            on_change = Rc::new(|v| {
                // Fazer algo com o valor
            })
        )
    
        // // Vertical
        // Slider(
        //     value = create_signal(75),
        //     min = 0,
        //     max = 100,
        //     step = 5,
        //     orientation = Orientation::Vertical,
        //     on_change = Rc::new(|v| {
        //         // Callback
        //     })
        // )
            
        div(style = "padding: 20px; font-family: 'Segoe UI', Arial, sans-serif;") {
            h1(style = "color: #333; margin-bottom: 20px;") { "Demo Qt Widgets em Sycamore" }
            
            // Seção 1: FormLayout
            VBoxLayout(spacing = 20, margin = 10) {
                
                // Formulário com FormLayout
                div(style = "border: 1px solid #ccc; padding: 15px; border-radius: 5px; background: #f9f9f9;") {
                    h2(style = "margin-top: 0; color: #555;") { "Formulário de Cadastro" }
                    
                    FormLayout(spacing = 10, margin = 10, label_spacing = 15) {
                        FormRow(label = "Nome:".to_string()) {
                            LineEdit(
                                value = name.clone(),
                                placeholder = "Digite seu nome completo".to_string(),
                                on_input = Rc::new({
                                    let name = name.clone();
                                    move |v| name.set(v)
                                })
                            )
                        }
                        
                        FormRow(label = "E-mail:".to_string()) {
                            LineEdit(
                                value = email.clone(),
                                placeholder = "seu@email.com".to_string(),
                                on_input = Rc::new({
                                    let email = email.clone();
                                    move |v| email.set(v)
                                })
                            )
                        }
                        
                        FormRow(label = "Idade:".to_string()) {
                            SpinBox(
                                value = age.clone(),
                                min = 0,
                                max = 120,
                                step = 1,
                                on_change = Rc::new({
                                    let age = age.clone();
                                    move |v| age.set(v)
                                })
                            )
                        }
                        
                        FormRow(label = "Altura (m):".to_string()) {
                            DoubleSpinBox(
                                value = height.clone(),
                                min = 0.5,
                                max = 2.5,
                                step = 0.01,
                                decimals = 2,
                                on_change = Rc::new({
                                    let height = height.clone();
                                    move |v| height.set(v)
                                })
                            )
                        }
                        
                        FormRow(label = "Peso (kg):".to_string()) {
                            DoubleSpinBox(
                                value = weight.clone(),
                                min = 30.0,
                                max = 200.0,
                                step = 0.5,
                                decimals = 1,
                                on_change = Rc::new({
                                    let weight = weight.clone();
                                    move |v| weight.set(v)
                                })
                            )
                        }
                        
                        FormRow(label = "Categoria:".to_string()) {
                            // ComboBox(
                            //     items = combo_items,
                            //     current_index = combo_index.clone(),
                            //     on_change = Rc::new({
                            //         let combo_index = combo_index.clone();
                            //         move |idx, _value| combo_index.set(idx)
                            //     })
                            // )
                        }
                    }
                }
                
                // Seção 2: RadioButtons em VBox
                div(style = "border: 1px solid #ccc; padding: 15px; border-radius: 5px; background: #f9f9f9;") {
                    h2(style = "margin-top: 0; color: #555;") { "Escolha uma opção" }
                    
                    VBoxLayout(spacing = 8, margin = 5) {
                        RadioButton(
                            text = "Opção 1 - Iniciante".to_string(),
                            name = "nivel".to_string(),
                            checked = radio1.clone(),
                            on_toggle = Rc::new({
                                let radio1 = radio1.clone();
                                let radio2 = radio2.clone();
                                let radio3 = radio3.clone();
                                move |v| {
                                    if v {
                                        radio1.set(true);
                                        radio2.set(false);
                                        radio3.set(false);
                                    }
                                }
                            })
                        )
                        
                        RadioButton(
                            text = "Opção 2 - Intermediário".to_string(),
                            name = "nivel".to_string(),
                            checked = radio2.clone(),
                            on_toggle = Rc::new({
                                let radio1 = radio1.clone();
                                let radio2 = radio2.clone();
                                let radio3 = radio3.clone();
                                move |v| {
                                    if v {
                                        radio1.set(false);
                                        radio2.set(true);
                                        radio3.set(false);
                                    }
                                }
                            })
                        )
                        
                        RadioButton(
                            text = "Opção 3 - Avançado".to_string(),
                            name = "nivel".to_string(),
                            checked = radio3.clone(),
                            on_toggle = Rc::new({
                                let radio1 = radio1.clone();
                                let radio2 = radio2.clone();
                                let radio3 = radio3.clone();
                                move |v| {
                                    if v {
                                        radio1.set(false);
                                        radio2.set(false);
                                        radio3.set(true);
                                    }
                                }
                            })
                        )
                    }
                }
                
                // Seção 3: GridLayout com botões
                div(style = "border: 1px solid #ccc; padding: 15px; border-radius: 5px; background: #f9f9f9;") {
                    h2(style = "margin-top: 0; color: #555;") { "Grid de Ações" }
                    
                    GridLayout(columns = 3, spacing = 8, margin = 5) {
                        PushButton(
                            text = "Ação 1".to_string(),
                            on_click = Rc::new({
                                let status_text = status_text.clone();
                                move |_| status_text.set("Ação 1 executada!".to_string())
                            })
                        )
                        
                        PushButton(
                            text = "Ação 2".to_string(),
                            on_click = Rc::new({
                                let status_text = status_text.clone();
                                move |_| status_text.set("Ação 2 executada!".to_string())
                            })
                        )
                        
                        PushButton(
                            text = "Ação 3".to_string(),
                            on_click = Rc::new({
                                let status_text = status_text.clone();
                                move |_| status_text.set("Ação 3 executada!".to_string())
                            })
                        )
                        
                        PushButton(
                            text = "Ação 4".to_string(),
                            on_click = Rc::new({
                                let status_text = status_text.clone();
                                move |_| status_text.set("Ação 4 executada!".to_string())
                            })
                        )
                        
                        PushButton(
                            text = "Ação 5".to_string(),
                            on_click = Rc::new({
                                let status_text = status_text.clone();
                                move |_| status_text.set("Ação 5 executada!".to_string())
                            })
                        )
                        
                        PushButton(
                            text = "Ação 6".to_string(),
                            on_click = Rc::new({
                                let status_text = status_text.clone();
                                move |_| status_text.set("Ação 6 executada!".to_string())
                            })
                        )
                    }
                }
                
                // Seção 4: HBoxLayout com botões principais
                div(style = "border: 1px solid #ccc; padding: 15px; border-radius: 5px; background: #f9f9f9;") {
                    h2(style = "margin-top: 0; color: #555;") { "Ações Principais" }
                    
                    HBoxLayout(spacing = 10, margin = 5, align = "center".to_string()) {
                        PushButton(
                            text = "Salvar".to_string(),
                            on_click = Rc::new({
                                let status_text = status_text.clone();
                                let name = name.clone();
                                move |_: MouseEvent| {
                                    status_text.set(format!("Dados de {} salvos!", name.get_clone()))
                                }
                            })
                        )
                        
                        PushButton(
                            text = "Cancelar".to_string(),
                            on_click = Rc::new({
                                let status_text = status_text.clone();
                                move |_| status_text.set("Operação cancelada".to_string())
                            })
                        )
                        
                        PushButton(
                            text = "Limpar".to_string(),
                            on_click = Rc::new({
                                let name = name.clone();
                                let email = email.clone();
                                let age = age.clone();
                                let status_text = status_text.clone();
                                move |_| {
                                    name.set(String::new());
                                    email.set(String::new());
                                    age.set(18);
                                    status_text.set("Formulário limpo".to_string());
                                }
                            })
                        )
                    }
                }
                
                // Barra de status
                div(style = "padding: 10px; background: #e8e8e8; border-radius: 5px; border: 1px solid #ccc;") {
                    RLabel(
                        text = create_signal(format!("Status: {}", status_text.get_clone())),
                        for_id = String::new()
                    )
                }
                
                // Resumo dos dados
                div(style = "border: 1px solid #ccc; padding: 15px; border-radius: 5px; background: #f0f8ff;") {
                    h2(style = "margin-top: 0; color: #555;") { "Resumo dos Dados" }
                    
                    VBoxLayout(spacing = 5, margin = 5) {
                        RLabel(
                            text = create_signal(format!("Nome: {}", name.get_clone())),
                            for_id = String::new()
                        )
                        
                        RLabel(
                            text = create_signal(format!("E-mail: {}", email.get_clone())),
                            for_id = String::new()
                        )
                        
                        RLabel(
                            text = create_signal(format!("Idade: {} anos", age.get())),
                            for_id = String::new()
                        )
                        
                        RLabel(
                            text = create_signal(format!("Altura: {:.2} m", height.get())),
                            for_id = String::new()
                        )
                        
                        RLabel(
                            text = create_signal(format!("Peso: {:.1} kg", weight.get())),
                            for_id = String::new()
                        )
                        
                        RLabel(
                            text = create_signal(format!("IMC: {:.2}", weight.get() / (height.get() * height.get()))),
                            for_id = String::new()
                        )
                        
                        RLabel(
                            text = create_signal(format!(
                                "Nível: {}",
                                if radio1.get() { "Iniciante" }
                                else if radio2.get() { "Intermediário" }
                                else if radio3.get() { "Avançado" }
                                else { "Não selecionado" }
                            )),
                            for_id = String::new()
                        )
                    }
                }
            }
        }
    }
}





// 1. Widgets Básicos (Controles Simples)


#[derive(Props)] pub struct QToolButtonProps {}
#[component] pub fn QToolButton(_: QToolButtonProps) -> View { view! {} }

#[derive(Props)] pub struct QCheckBoxProps {}
#[component] pub fn QCheckBox(_: QCheckBoxProps) -> View { view! {} }

#[derive(Props)] pub struct QPlainTextEditProps {}
#[component] pub fn QPlainTextEdit(_: QPlainTextEditProps) -> View { view! {} }

#[derive(Props)] pub struct QDateEditProps {}
#[component] pub fn QDateEdit(_: QDateEditProps) -> View { view! {} }

#[derive(Props)] pub struct QTimeEditProps {}
#[component] pub fn QTimeEdit(_: QTimeEditProps) -> View { view! {} }

#[derive(Props)] pub struct QDateTimeEditProps {}
#[component] pub fn QDateTimeEdit(_: QDateTimeEditProps) -> View { view! {} }

#[derive(Props)] pub struct QDialProps {}
#[component] pub fn QDial(_: QDialProps) -> View { view! {} }

// #[derive(Props)] pub struct QSliderProps {}
// #[component] pub fn QSlider(_: QSliderProps) -> View { view! {} }

#[derive(Props)] pub struct QScrollBarProps {}
#[component] pub fn QScrollBar(_: QScrollBarProps) -> View { view! {} }

#[derive(Props)] pub struct QLCDNumberProps {}
#[component] pub fn QLCDNumber(_: QLCDNumberProps) -> View { view! {} }

#[derive(Props)] pub struct QProgressBarProps {}
#[component] pub fn QProgressBar(_: QProgressBarProps) -> View { view! {} }


// 2. Widgets de Seleção


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
