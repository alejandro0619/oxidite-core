use oxidite_core::models::OxiditeConfig;
use oxidite_core::{Oxidite, LaunchSettings}; 
use std::path::PathBuf;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initial config
    let launcher = Oxidite::new(OxiditeConfig {
        base_path: PathBuf::from("C:\\.minecraft_oxidite"),
        java_path: "java".to_string(), // Or the absolute path to your Java executable if needed
        max_parallel_downloads: 20,    // How many files to download in paralle, this applies when downloading the assets and the libraries
        memory_gb: 4,                  // How many GB of RAM to allocate to Minecraft
        extra_jvm_args: vec!["-XX:+UseG1GC".to_string()], // Extra JVM arguments to optimize performance, you can add more if you want
    });
    
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