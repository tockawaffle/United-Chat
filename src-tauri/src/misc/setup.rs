use crate::chat::twitch::auth::ImplicitGrantFlow;
use keyring::Entry;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager, State};
use tokio::task;

pub(crate) struct SetupState {
    pub(crate) frontend_task: bool,
    pub(crate) backend_task: bool,
}

async fn backend_setup(app: AppHandle) {
    let app_clone = app.clone();

    // Try to get the Twitch authentication from the keyring first
    match Entry::new("united-chat", "twitch-auth") {
        Ok(entry) => {
            let auth = entry.get_password().unwrap();

            let parsed: ImplicitGrantFlow = serde_json::from_str(&auth.clone()).unwrap();

            let val = ImplicitGrantFlow {
                access_token: parsed.access_token,
                scope: parsed.scope,
                state: parsed.state,
                token_type: parsed.token_type,
                error: None,
                error_description: None,
                skipped: None,
            };

            app.manage(val);

            task::spawn_blocking(move || {
                let runtime = tokio::runtime::Runtime::new().unwrap();
                runtime.block_on(setup_complete(
                    app.clone(),
                    app.state::<Mutex<SetupState>>(),
                    "backend".to_string(),
                    None,
                ))
            })
        }
        Err(_) => {
            // If the keyring entry doesn't exist, send an event to the frontend to start the Twitch authentication process
            app_clone
                .emit_to("splashscreen", "splashscreen::twitch-reauth", true)
                .unwrap();
            panic!("Twitch auth not found");
        }
    };
}

#[tauri::command]
pub(crate) async fn setup_complete(
    app: AppHandle,
    state: State<'_, Mutex<SetupState>>,
    task: String,
    skip: Option<bool>,
) -> Result<(), ()> {
    let mut state_lock = state.lock().unwrap();

    match task.as_str() {
        "frontend" => {
            state_lock.frontend_task = true;

            if Some(true) == skip {
                state_lock.backend_task = true;

                // manage TwitchAuth struct
                let twitch_auth = ImplicitGrantFlow {
                    access_token: "".to_string(),
                    scope: "".to_string(),
                    state: "".to_string(),
                    token_type: "".to_string(),
                    error: None,
                    error_description: None,
                    skipped: Some(true),
                };

                app.manage(twitch_auth);
            } else {
                task::spawn(backend_setup(app.clone()));
            }
        }
        "backend" => {
            state_lock.backend_task = true;
        }
        _ => {
            println!("Unknown task");
        }
    }

    if state_lock.frontend_task && state_lock.backend_task {
        let splash_window = app.get_webview_window("splashscreen").unwrap();
        let main_window = app.get_webview_window("main").unwrap();
        splash_window.close().unwrap();
        main_window.show().unwrap();
    }

    drop(state_lock);
    Ok(())
}
