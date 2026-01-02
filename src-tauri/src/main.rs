// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    setup_logging();
    iterate_rs_lib::run()
}

fn setup_logging() {
    #[cfg(debug_assertions)] // Only runs in 'cargo tauri dev'
    {
        // Log everything to the terminal in development
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    }

    #[cfg(not(debug_assertions))] // Runs in 'cargo tauri build' (Release)
    {}
}
