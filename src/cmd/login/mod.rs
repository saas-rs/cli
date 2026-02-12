use crate::protocol::saas_rs::user::v1::FindAccountRequest;
use crate::{apiclient, config, consts};
use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{App, HttpServer, Responder, post, web};
use clap::Parser;
use serde::Deserialize;
use std::error::Error;
use std::net::TcpListener;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use webbrowser::Browser;

#[derive(Debug, Parser)]
pub struct Opts {
    /// Override the default api url
    #[arg(long = "api-url", alias = "apiUrl")]
    pub api_url: Option<String>,

    /// Override the default browser
    #[arg(long = "browser")]
    pub browser: Option<Browser>,

    /// Override the default console url
    #[arg(long = "console-url", alias = "consoleUrl")]
    pub console_url: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
struct JsonContents {
    token: String,
}

struct AppState {
    api_url: String,
    tx: Sender<String>,
}

pub async fn run(
    api_url: Option<String>,
    console_url: Option<String>,
    browser: Option<Browser>,
) -> Result<(), Box<dyn Error>> {
    // Bind to a dynmamic port
    let tcp = TcpListener::bind("127.0.0.1:0")?;
    let port = tcp.local_addr().unwrap().port();
    let bind_addr = format!("127.0.0.1:{port}");

    // Launch browser to login form
    let console_url = console_url.unwrap_or(consts::env_vars::CONSOLE_URL.to_string());
    let url = format!("{console_url}/login?callback=http://{bind_addr}/callback");
    let browser = browser.unwrap_or(Browser::Default);
    webbrowser::open_browser(browser, &url)?;

    // Start an embedded http server
    let api_url = api_url.unwrap_or(consts::env_vars::API_URL.to_string());
    HttpServer::new(move || {
        let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

        // Start a thread to respond to shutdowns
        tokio::spawn(async move {
            // Wait for shutdown signal
            match rx.recv() {
                Ok(msg) => {
                    eprintln!("{msg}");
                    std::process::exit(0);
                }
                Err(e) => {
                    eprintln!("An error occurred receiving callback notification from browser: {e:?}");
                    std::process::exit(1);
                }
            }
        });

        // Run the web server
        let cors = Cors::permissive();
        App::new()
            .app_data(Data::new(AppState {
                api_url: api_url.clone(),
                tx,
            }))
            .wrap(cors)
            .service(callback_handler)
    })
    .listen(tcp)
    .unwrap()
    .run()
    .await
    .unwrap();

    Ok(())
}

#[post("/callback")]
async fn callback_handler(app_state: Data<AppState>, data: web::Json<JsonContents>) -> impl Responder {
    log::trace!("callback_handler({data:?})");

    // Save the new token as an api key
    let mut conf = config::load().unwrap();
    conf.api_key = Some(data.token.to_string());
    conf.api_url = Some(app_state.api_url.clone());
    config::save(&conf).unwrap();

    // Test it
    let mut client = apiclient::new_user_service_client().await.unwrap();
    let req = FindAccountRequest { id: "me".to_string() };
    match client.find_account(req).await {
        Ok(res) => {
            let name = res.into_inner().account.unwrap().name.unwrap();
            let msg = format!("Logged in. Greetings {name}!");
            app_state
                .tx
                .send(msg.clone())
                .expect("Failed signaling embedded web server to shutdown");
            msg
        }
        Err(e) => {
            eprintln!("Failed looking up current user with new access token: {e:?}");
            e.to_string()
        }
    }
}
