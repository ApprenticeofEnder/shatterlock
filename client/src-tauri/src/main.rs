// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Commands:
// - Create Auth Network
// - Join Auth Network
// - Invite Client To Auth Network
// - Create Keys (Encryption, Signing)
// - Create Shared Keys (Encryption, Signing)
// - Retrieve Keys (Encryption, Signing, Shared)
// - Add Secret
// - Retrieve Secret
// - Update Secret
// - Sync Secrets
// - Join Data Network
// - Create Data Network
//
// Database Model:
// - Keys
// - Auth/Sync Networks
// - Data Networks
// - Secrets (id, network, key ID)
// - Sync File
// Probably need a central sync file in the auth network
// Maybe call it a sync network instead?

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
