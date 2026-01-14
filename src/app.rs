mod ui;
mod http;

use ui::*;
use serde::{Deserialize, Serialize};
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
// use sycamore::web::events::SubmitEvent;
use wasm_bindgen::prelude::*;
use web_sys::{MouseEvent, console};

use crate::app::http::{HttpClient, HttpMethod, HttpRequest, HttpResult};


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct User {
    name: String
}

struct State {
    nome: Signal<String>,
    email: Signal<String>,
    dark_mode: Signal<bool>,
    notifications: Signal<bool>,
    volume: Signal<f64>,
    brightness: Signal<f64>,
    loading: Signal<bool>,
    theme_option: Signal<usize>,
    accept_terms: Signal<bool>,
    bio: Signal<String>,
}


#[component]
pub fn App() -> View {
    // --- ESTADO ---

    let state = State {
        nome: create_signal(String::from("Maria Oliveira")),
        email: create_signal(String::from("maria@example.com")),
        dark_mode: create_signal(false),
        notifications: create_signal(true),
        volume: create_signal(75.0),
        brightness: create_signal(0.6),
        loading: create_signal(false),
        theme_option: create_signal(0),
        accept_terms: create_signal(false),
        bio: create_signal(
            String::from("Desenvolvedora apaixonada por Rust e UI design.")
        )
    };

    let save_action = move |_| {
        state.loading.set(true);
        console::log_1(&format!("Salvando configurações para: {}", state.nome.get_clone()).into());
        // Simular um delay
        state.loading.set(false);
    };


    let export_action = move |_e: MouseEvent| {

        async fn make_request() -> HttpResult<()>  {
            console::log_1(&"Exportando configurações...".into());

            // 1. Requisição simples
            let resp: http::HttpResponse = http::get("https://httpbin.org/get").await?;
            // let users: Vec<User> = resp.json()?;

            // 2. POST com JSON
            let new_user = User { name: "João".to_string() };
            let resp: http::HttpResponse = HttpRequest::new(HttpMethod::POST, "https://httpbin.org/post")
                .json(&new_user)
                .send()
                .await?;

            // 3. Com autenticação
            let resp: http::HttpResponse = HttpRequest::new(HttpMethod::GET, "https://httpbin.org/get")
                .bearer("seu-token-aqui")
                .send()
                .await?;

            // 4. Cliente reutilizável
            let client: HttpClient = HttpClient::new("https://httpbin.org/")
                .with_bearer("token-123")
                .with_header("X-Custom", "value");

            let users: http::HttpResponse = client.get("/get").send().await?;
            let created: http::HttpResponse = client.post("/post").json(&new_user).send().await?;

            Ok(())

        }

        spawn_local_scoped(async move {
            match make_request().await {
                Ok(_) => {
                    console::log_1(&"✅ Exportação concluída com sucesso!".into());
                }
                Err(e) => {
                    console::error_1(&format!("❌ Erro na exportação: {}", e).into());
                }
            }
        });
    };

    // --- DEFINIÇÃO DAS ABAS ---
    let tabs_config = vec![

        TabItem::new("Perfil", move || view! {
            VBox(spacing = 20) {
                HBox(spacing = 12, align = Align::Between) {
                    Label(
                        text = "Informações do Perfil".to_string(),
                        kind = LabelKind::Header
                    )
                    Badge(
                        text = "Verificado".to_owned(),
                        kind = BadgeKind::Success
                    )
                }

                Label(
                    text = "Gerencie suas informações pessoais e preferências.".to_string(),
                    kind = LabelKind::Caption
                )

                Card(elevated = true) {
                    VBox(spacing = 20) {
                        Grid(columns = 2, gap = 16) {
                            VBox(spacing = 8) {
                                Label(
                                    text = "Nome Completo".to_string(),
                                    kind = LabelKind::Body
                                )
                                TextField(
                                    value = state.nome,
                                    placeholder = "Digite seu nome",
                                    grow = true
                                )
                            }
                            VBox(spacing = 8) {
                                Label(
                                    text = "E-mail".to_string(),
                                    kind = LabelKind::Body
                                )
                                TextField(
                                    value = state.email,
                                    placeholder = "seu@email.com",
                                    grow = true
                                )
                            }
                        }

                        Separator()

                        VBox(spacing = 8) {
                            Label(
                                text = "Biografia".to_string(),
                                kind = LabelKind::Body
                            )
                            TextArea(
                                value = state.bio,
                                placeholder = "Conte um pouco sobre você...",
                                rows = 4
                            )
                        }

                        Separator()

                        HBox(spacing = 12, align = Align::Between) {
                            VBox(spacing = 4, align = Align::Start) {
                                Label(
                                    text = "Notificações Desktop".to_string(),
                                    kind = LabelKind::Body
                                )
                                Label(
                                    text = "Receba alertas importantes no desktop".to_string(),
                                    kind = LabelKind::Caption
                                )
                            }
                            Toggle(checked = state.notifications)
                        }

                        Checkbox(
                            checked = state.accept_terms,
                            label = "Aceito os termos de uso e política de privacidade"
                        )
                    }
                }

                Panel(title = "Informações da Conta", collapsible = true) {
                    VBox(spacing = 12) {
                        HBox(spacing = 8, align = Align::Between) {
                            Label(
                                text = "ID da Conta:".to_string(),
                                kind = LabelKind::Body
                            )
                            Label(
                                text = "USR-2024-42A7".to_string(),
                                kind = LabelKind::Monospace
                            )
                        }
                        HBox(spacing = 8, align = Align::Between) {
                            Label(
                                text = "Membro desde:".to_string(),
                                kind = LabelKind::Body
                            )
                            Label(
                                text = "Janeiro 2024".to_string(),
                                kind = LabelKind::Caption
                            )
                        }
                        HBox(spacing = 8, align = Align::Between) {
                            Label(
                                text = "Status:".to_string(),
                                kind = LabelKind::Body
                            )
                            Badge(
                                text = "Premium".to_owned(),
                                kind = BadgeKind::Primary
                            )
                        }
                    }
                }
            }
        }),

        TabItem::new("Aparência", move || view! {

            VBox(spacing = 20) {
                Label(text = "Aparência e Tema".to_string(), kind = LabelKind::Header)
                
                Card() {
                    VBox(spacing = 20) {
                        VBox(spacing = 12) {
                            Label(
                                text = "Tema do Sistema".to_string(),
                                kind = LabelKind::Body
                            )
                            RadioGroup(
                                selected = state.theme_option,
                                options = vec!["Claro", "Escuro", "Automático"]
                            )
                        }

                        Divider(text = "Personalização")

                        HBox(align = Align::Between) {
                            VBox(spacing = 4, align = Align::Start) {
                                Label(
                                    text = "Modo Escuro".to_string(),
                                    kind = LabelKind::Body
                                )
                                Label(
                                    text =
                                        "Reduz o cansaço visual em ambientes escuros"
                                        .to_string(),
                                    kind = LabelKind::Caption
                                )
                            }
                            Toggle(checked = state.dark_mode)
                        }

                        Separator()

                        VBox(spacing = 12) {
                            HBox(align = Align::Between) {
                                Label(
                                    text = "Brilho da Tela".to_string(),
                                    kind = LabelKind::Body
                                )
                                Label(
                                    text = format!("{:.0}%", state.brightness.get() * 100.0),
                                    kind = LabelKind::Caption
                                )
                            }
                            Slider(
                                value = state.brightness,
                                min = 0.0,
                                max = 1.0,
                                grow = true,
                                step = 0.01
                            )
                            ProgressBar(
                                value = state.brightness,
                                show_label = false,
                                indeterminate = false
                            )
                        }
                    }
                }

                Card(interactive = true) {
                    HBox(spacing = 12, align = Align::Between) {
                        VBox(spacing = 4, align = Align::Start) {
                            Label(
                                text = "Gerenciador de Cores".to_string(),
                                kind = LabelKind::Body
                            )
                            Label(
                                text = "Personalize a paleta de cores do sistema".to_string(),
                                kind = LabelKind::Caption
                            )
                        }
                        Label(text = "→".to_string())
                    }
                }
            }
        }),

        TabItem::new("Sistema", move || view! {

            VBox(spacing = 20) {
                Label(text = "Configurações do Sistema".to_string(), kind = LabelKind::Header)
                
                Card() {
                    VBox(spacing = 20) {
                        VBox(spacing = 12) {
                            HBox(spacing = 8, align = Align::Between) {
                                Label(
                                    text = "Volume de Saída".to_string(),
                                    kind = LabelKind::Body
                                )
                                Badge(
                                    text = format!("{}%", state.volume.get() as u8),
                                    kind = BadgeKind::Info
                                )
                            }
                            HBox(spacing = 12, align = Align::Center) {
                                // Label(text = "🔈".to_string())
                                Slider(
                                    value = state.volume,
                                    min = 0.0, max = 100.0,
                                    grow = true, step = 1.0
                                )
                                // Label(text = "🔊".to_string())
                            }
                        }

                        Divider(text = "Dispositivos")

                        VBox(spacing = 12) {
                            Label(
                                text = "Dispositivos Conectados".to_string(),
                                kind = LabelKind::Body
                            )
                            
                            Card(class = "elevation-1") {
                                HBox(spacing = 12, align = Align::Between) {
                                    HBox(spacing = 12) {
                                        // Label(text = "🎧".to_string())
                                        VBox(spacing = 2, align = Align::Start) {
                                            Label(
                                                text = "Fones Bluetooth".to_string(),
                                                kind = LabelKind::Body
                                            )
                                            Label(
                                                text = "Bateria: 85%".to_string(),
                                                kind = LabelKind::Caption
                                            )
                                        }
                                    }
                                    Badge(
                                        text = "Conectado".to_owned(),
                                        kind = BadgeKind::Success
                                    )
                                }
                            }

                            Card(class = "elevation-1") {
                                HBox(spacing = 12, align = Align::Between) {
                                    HBox(spacing = 12) {
                                        // Label(text = "🖨️".to_string())
                                        VBox(spacing = 2, align = Align::Start) {
                                            Label(
                                                text = "Impressora HP".to_string(),
                                                kind = LabelKind::Body
                                            )
                                            Label(
                                                text = "Pronta para imprimir".to_string(),
                                                kind = LabelKind::Caption
                                            )
                                        }
                                    }
                                    Badge(
                                        text = "Online".to_owned(),
                                        kind = BadgeKind::Success
                                    )
                                }
                            }
                        }
                    }
                }

                Panel(title = "Informações do Sistema", collapsible = true) {
                    Grid(columns = 2, gap = 12) {
                        VBox(spacing = 4) {
                            Label(
                                text = "Sistema Operacional".to_string(),
                                kind = LabelKind::Caption
                            )
                            Label(
                                text = "Linux 6.5.0".to_string(),
                                kind = LabelKind::Body
                            )
                        }
                        VBox(spacing = 4) {
                            Label(
                                text = "Processador".to_string(),
                                kind = LabelKind::Caption
                            )
                            Label(
                                text = "Intel Core i7-12700K".to_string(),
                                kind = LabelKind::Body
                            )
                        }
                        VBox(spacing = 4) {
                            Label(
                                text = "Memória RAM".to_string(),
                                kind = LabelKind::Caption
                            )
                            Label(
                                text = "32 GB DDR4".to_string(),
                                kind = LabelKind::Body
                            )
                        }
                        VBox(spacing = 4) {
                            Label(
                                text = "Armazenamento".to_string(),
                                kind = LabelKind::Caption
                            )
                            Label(
                                text = "1 TB NVMe SSD".to_string(),
                                kind = LabelKind::Body
                            )
                        }
                    }
                }
            }
        }),

        TabItem::new("Avançado", move || view! {
            VBox(spacing = 20) {
                HBox(spacing = 12, align = Align::Between) {
                    Label(
                        text = "Configurações Avançadas".to_string(),
                        kind = LabelKind::Header
                    )
                    Badge(
                        text = "Cuidado".to_owned(),
                        kind = BadgeKind::Warning
                    )
                }

                Card() {
                    VBox(spacing = 16) {
                        Label(
                            text = "Ferramentas de Desenvolvedor".to_string(),
                            kind = LabelKind::Body
                        )
                        
                        HBox(spacing = 8) {
                            Button(
                                text = "Exportar Logs",
                                kind = ButtonKind::Secondary,
                                on_click = Box::new(export_action)
                            )
                            Button(
                                text = "Limpar Cache",
                                kind = ButtonKind::Ghost,
                                on_click = Box::new(
                                    move |_| console::log_1(&"Limpando cache...".into())
                                )
                            )
                        }

                        Separator()

                        VBox(spacing = 12) {
                            Label(
                                text = "Zona de Perigo".to_string(),
                                kind = LabelKind::Body
                            )
                            Label(
                                text = 
                                    "Ações irreversíveis que afetam permanentemente sua conta"
                                    .to_string(),
                                kind = LabelKind::Caption
                            )
                            Button(
                                text = "Resetar Configurações",
                                kind = ButtonKind::Destructive,
                                on_click = Box::new(
                                    move |_| console::log_1(&"Resetando...".into())
                                )
                            )
                        }
                    }
                }

                (if state.loading.get() {
                    view! {
                        Card() {
                            HBox(spacing = 12, align = Align::Center) {
                                Spinner(size = 20)
                                Label(
                                    text = "Processando...".to_string(),
                                    kind = LabelKind::Body
                                )
                            }
                        }
                    }
                } else { view! {} })
            }
        }),
    ];

    // --- VIEW ---
    view! {
        Window(
            title = "Preferências do Sistema", size = (900, 650),
            resizable = false
        ) {
            VBox(spacing = 0, align = Align::Stretch, class = "h-full") {
                Toolbar() {
                    HBox(spacing = 8) {
                        IconButton(
                            icon_src="/public/icons/edit.svg",
                            alt_text="Editar",
                            on_click = Box::new(|_| ()),
                            tooltip = "Voltar"
                        )
                        IconButton(
                            icon_src="/public/icons/edit.svg",
                            alt_text="Editar",
                            on_click = Box::new(|_| ()),
                            tooltip = "Configurações"
                        )
                        Spacer()
                        IconButton(
                            icon_src="/public/icons/edit.svg",
                            alt_text="Editar",
                            on_click = Box::new(|_| ()),
                            tooltip = "Configurações",
                        )

                    }
                }

                TabView(tabs = tabs_config)

                StatusBar() {
                    HBox(
                        spacing = 16,
                        align = Align::Between,
                        style = "width: 100%;"
                    ) {
                        HBox(spacing = 8) {
                            Label(
                                text = "v1.2.4".to_string(),
                                kind = LabelKind::Caption
                            )
                            Label(
                                text = "•".to_string(),
                                kind = LabelKind::Caption
                            )
                            Label(
                                text = "Última sincronização: 2 min atrás".to_string(),
                                kind = LabelKind::Caption
                            )
                        }
                        HBox(spacing = 12) {
                            (if state.loading.get() {
                                view! {
                                    HBox(spacing = 8) {
                                        Spinner(size = 12)
                                        Label(
                                            text = "Salvando...".to_string(),
                                            kind = LabelKind::Caption
                                        )
                                    }
                                }
                            } else {
                                view! {
                                    Label(
                                        text = "✓ Pronto".to_string(),
                                        kind = LabelKind::Caption
                                    )
                                }
                            })
                        }
                    }
                }

                Separator()

                HBox(
                    spacing = 12,
                    align = Align::End,
                    style = "padding: 16px; background: var(--bg-surface);"
                ) {
                    Button(
                        text = "Cancelar",
                        kind = ButtonKind::Ghost,
                        on_click = Box::new(
                            move |_| console::log_1(&"Cancelado".into())
                        )
                    )
                    Button(
                        text = "Aplicar",
                        kind = ButtonKind::Secondary,
                        on_click = Box::new(
                            move |_| console::log_1(&"Aplicando...".into())
                        )
                    )
                    Button(
                        text = "Salvar Alterações",
                        kind = ButtonKind::Primary,
                        on_click = Box::new(save_action),
                    )
                }
            }
        }
    }
}
