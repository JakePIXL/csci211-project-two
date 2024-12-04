pub mod config;
pub mod db;
pub mod game;
pub mod models;

pub fn clear_screen() {
    // Clear terminal screen
    println!("\x1B[2J\x1B[1;1H");
}
