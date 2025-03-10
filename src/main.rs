#![windows_subsystem = "windows"]

use std::process::Command;
use notify_rust::Notification;
use rand::Rng;
use win_hotkeys::HotkeyManager;
use win_hotkeys::VKey;

fn main() {
    let mut hkm: HotkeyManager<()> = HotkeyManager::new();
   
    hkm.register_hotkey(VKey::Space, &[VKey::LWin], || {
        log::info!("Hotkey Win + Spacebar was pressed");

        let mut rng = rand::rng();

        if rng.random_bool(1.0/100.0) {
            let _ = Notification::new()
                .summary("AutoHotKeys sucks")
                .body("Fuck you AutoHotKeys")
                .show();
        };

        let task = Command::new("nu").current_dir("C:\\Users\\brodi").spawn();
        if let Err(e) = task {
            log::error!("Failed to start Nushell: {}", e)
        }
    })
    .unwrap();
   
    hkm.event_loop();
}