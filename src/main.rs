#![windows_subsystem = "windows"]

use std::process::Command;
use win_hotkeys::HotkeyManager;
use win_hotkeys::VKey;

fn main() {
    let mut hkm: HotkeyManager<()> = HotkeyManager::new();
   
    hkm.register_hotkey(VKey::Space, &[VKey::LWin], || {
        log::info!("Hotkey Win + Spacebar was pressed");
        if let Err(e) = Command::new("nu")
            .current_dir("C:\\Users\\brodi")
            .spawn()

        {
            log::error!("Failed to start Nushell: {}", e)
        }
    })
    .unwrap();
   
    hkm.event_loop();
}