// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "console")]

fn main() {
    exchange_dashboard_lib::run()
}
