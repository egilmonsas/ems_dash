use ems_terrain::generate;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![gen_ifc, greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn gen_ifc(
    xc: f64,
    yc: f64,
    width: f64,
    height: f64,
    resolution: f64,
    coord_sys: usize,
) -> Result<Vec<u8>, String> {
    let file = generate(xc, yc, width, height, resolution, coord_sys).await;

    match file {
        Ok(f) => Ok(f),
        Err(_) => Err("Encountered".to_string()),
    }
}
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
