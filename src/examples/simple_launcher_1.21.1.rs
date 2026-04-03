use oxidite_core::{Oxidite, LaunchSettings}; 
use std::path::PathBuf;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initial config
    let base_path = PathBuf::from("C:\\.minecraft_oxidite");
    let launcher = Oxidite::new(base_path);
    
    let version_id = "1.21.1"; // Just to try
    println!("🔍 Initializing Oxidite for version {}...", version_id);

    // 2. Preparation with the Stepper (Callback)
    launcher.prepare_version(Some(version_id), |report| {
        print!("\r{} [{} / {}] {:.2}%          ", 
            report.message, 
            report.current, 
            report.total,
            (report.current as f32 / report.total as f32) * 100.0
        );
        io::stdout().flush().unwrap();
    }).await?;

    println!("\n✅ Preparation completed.");

    // 3. Launch
    let settings = LaunchSettings::offline("Alejandro");
    
    // If you need to change the Java path manually due to the previous error:
    // settings.java_path = r"C:\Ruta\A\Tu\Java\bin\java.exe".to_string();

    launcher.launch(version_id, settings).await?;

    Ok(())
}