use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, Headers};

// --- TIPOS ---

#[derive(Debug, Clone)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

impl HttpMethod {
    fn as_str(&self) -> &str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::PATCH => "PATCH",
            HttpMethod::DELETE => "DELETE",
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpError {
    pub status: u16,
    pub message: String,
}

impl From<serde_json::Error> for HttpError {
    fn from(err: serde_json::Error) -> Self {
        HttpError {
            status: 0,
            message: format!("JSON parse error: {}", err),
        }
    }
}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HTTP {} - {}", self.status, self.message)
    }
}

pub type HttpResult<T> = Result<T, HttpError>;

// --- REQUEST BUILDER ---

pub struct HttpRequest {
    url: String,
    method: HttpMethod,
    headers: HashMap<String, String>,
    body: Option<String>,
    timeout: Option<u32>,
}

impl HttpRequest {
    pub fn new(method: HttpMethod, url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            method,
            headers: HashMap::new(),
            body: None,
            timeout: None,
        }
    }

    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    pub fn bearer(self, token: impl Into<String>) -> Self {
        self.header("Authorization", format!("Bearer {}", token.into()))
    }

    pub fn json<T: Serialize>(mut self, data: &T) -> Self {
        if let Ok(json_str) = serde_json::to_string(data) {
            self.body = Some(json_str);
            self.header("Content-Type", "application/json")
        } else {
            self
        }
    }

    pub fn body(mut self, data: impl Into<String>) -> Self {
        self.body = Some(data.into());
        self
    }

    pub fn timeout(mut self, ms: u32) -> Self {
        self.timeout = Some(ms);
        self
    }

    pub async fn send(self) -> HttpResult<HttpResponse> {
        let window = web_sys::window().ok_or_else(|| HttpError {
            status: 0,
            message: "Sem acesso ao window".to_string(),
        })?;

        let opts = RequestInit::new();
        opts.set_method(self.method.as_str());
        opts.set_mode(RequestMode::Cors);

        if let Some(body) = self.body {
            opts.set_body(&wasm_bindgen::JsValue::from_str(&body));
        }

        let request = Request::new_with_str_and_init(&self.url, &opts).map_err(|_| HttpError {
            status: 0,
            message: "Falha ao criar request".to_string(),
        })?;

        let headers = request.headers();

        // Adiciona headers
        for (key, value) in self.headers {
            headers
                .set(&key, &value)
                .map_err(|_| HttpError {
                    status: 0,
                    message: format!("Falha ao definir header: {}", key),
                })?;
        }

        // Faz o fetch
        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|_| HttpError {
                status: 0,
                message: "Falha na requisição".to_string(),
            })?;

        let response: Response = resp_value.dyn_into().map_err(|_| HttpError {
            status: 0,
            message: "Resposta inválida".to_string(),
        })?;

        let status = response.status() as u16;
        let ok = response.ok();

        // Lê o body
        let text = if let Ok(text_promise) = response.text() {
            JsFuture::from(text_promise)
                .await
                .ok()
                .and_then(|v| v.as_string())
                .unwrap_or_default()
        } else {
            String::new()
        };

        if !ok {
            return Err(HttpError {
                status,
                message: text,
            });
        }

        Ok(HttpResponse { status, body: text })
    }
}

// --- RESPONSE ---

pub struct HttpResponse {
    pub status: u16,
    pub body: String,
}

impl HttpResponse {
    pub fn json<T: for<'de> Deserialize<'de>>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_str(&self.body)
    }

    pub fn text(&self) -> &str {
        &self.body
    }

    pub fn is_success(&self) -> bool {
        self.status >= 200 && self.status < 300
    }
}

// --- HTTP CLIENT ---

pub struct HttpClient {
    base_url: String,
    default_headers: HashMap<String, String>,
}

impl HttpClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            default_headers: HashMap::new(),
        }
    }

    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.default_headers.insert(key.into(), value.into());
        self
    }

    pub fn with_bearer(self, token: impl Into<String>) -> Self {
        self.with_header("Authorization", format!("Bearer {}", token.into()))
    }

    fn build_url(&self, path: &str) -> String {
        if path.starts_with("http://") || path.starts_with("https://") {
            path.to_string()
        } else {
            format!("{}{}", self.base_url.trim_end_matches('/'), path)
        }
    }

    fn create_request(&self, method: HttpMethod, path: &str) -> HttpRequest {
        let mut req = HttpRequest::new(method, self.build_url(path));
        for (k, v) in &self.default_headers {
            req = req.header(k, v);
        }
        req
    }

    pub fn get(&self, path: &str) -> HttpRequest {
        self.create_request(HttpMethod::GET, path)
    }

    pub fn post(&self, path: &str) -> HttpRequest {
        self.create_request(HttpMethod::POST, path)
    }

    pub fn put(&self, path: &str) -> HttpRequest {
        self.create_request(HttpMethod::PUT, path)
    }

    pub fn patch(&self, path: &str) -> HttpRequest {
        self.create_request(HttpMethod::PATCH, path)
    }

    pub fn delete(&self, path: &str) -> HttpRequest {
        self.create_request(HttpMethod::DELETE, path)
    }
}

// --- HELPERS RÁPIDOS ---

pub async fn get(url: impl Into<String>) -> HttpResult<HttpResponse> {
    HttpRequest::new(HttpMethod::GET, url).send().await
}

pub async fn post(url: impl Into<String>) -> HttpRequest {
    HttpRequest::new(HttpMethod::POST, url)
}

pub async fn get_json<T: for<'de> Deserialize<'de>>(url: impl Into<String>) -> HttpResult<T> {
    let resp = get(url).await?;
    resp.json().map_err(|e| HttpError {
        status: 0,
        message: format!("JSON parse error: {}", e),
    })
}

pub async fn post_json<B: Serialize, T: for<'de> Deserialize<'de>>(
    url: impl Into<String>,
    body: &B,
) -> HttpResult<T> {
    let resp = HttpRequest::new(HttpMethod::POST, url)
        .json(body)
        .send()
        .await?;
    
    resp.json().map_err(|e| HttpError {
        status: 0,
        message: format!("JSON parse error: {}", e),
    })
}