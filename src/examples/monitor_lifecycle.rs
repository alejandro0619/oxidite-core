/// This example demonstrates how to launch a Minecraft version and monitor its full lifecycle.
/// It uses the `GameInstance` to track uptime, process status, and real-time game activity 
/// (such as identifying the Server IP if the user joins a multiplayer game).

use std::path::PathBuf;
use std::time::Duration;
use tokio::time::sleep;
use oxidite_core::{Oxidite, LaunchSettings, models::OxiditeConfig};
use oxidite_core::instance::GameStatus; // Ensure GameStatus is exported in your lib

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Setup the launcher configuration
    let launcher = Oxidite::new(OxiditeConfig {
        base_path: PathBuf::from("C:\\.minecraft_oxidite"),
        java_path: "java.exe".to_string(), 
        max_parallel_downloads: 20,
        memory_gb: 4,
        extra_jvm_args: vec!["-XX:+UseG1GC".to_string()],
    });

    let version_id = "1.20.1";
    println!("🔍 Preparing environment for version {}...", version_id);

    // 2. Prepare the environment (Silent progress for this example)
    launcher.prepare_version(Some(version_id), |_| {}).await?;

    // 3. Launch the game
    println!("🚀 Launching Minecraft...");
    let settings = LaunchSettings::offline("Alejandro");
    
    // The launch method returns the GameInstance with a background log parser
    let mut instance = launcher.launch(version_id, settings).await?;

    println!("🎮 Game started successfully! (PID: {})", instance.process.id());

    let mut current_activity = "In Menu / Loading".to_string();

    // 4. Lifecycle Monitoring Loop
    while instance.is_running() {
        // Check for status updates from the game's internal logs
        // We use try_recv() to avoid blocking the monitoring loop
        while let Ok(status) = instance.status_receiver.try_recv() {
            match status {
                GameStatus::Singleplayer => {
                    current_activity = "Playing Singleplayer".to_string();
                }
                GameStatus::Multiplayer { ip } => {
                    current_activity = format!("Playing Multiplayer (IP: {})", ip);
                }
                GameStatus::Menu => {
                    current_activity = "In Main Menu".to_string();
                }
            }
        }

        let uptime = instance.start_time.elapsed().as_secs();
        
        // Print real-time telemetry
        print!(
            "\r🕒 Session: {}m {}s | Activity: {} | Status: RUNNING",
            uptime / 60,
            uptime % 60,
            current_activity
        );
        
        use std::io::{self, Write};
        io::stdout().flush().unwrap();

        sleep(Duration::from_millis(500)).await;
    }

    // 5. Post-Game Analysis
    println!("\n\n🛑 Game process has terminated.");
    
    if let Some(exit_code) = instance.exit_code() {
        match exit_code {
            0 => println!("✅ Minecraft closed gracefully."),
            _ => {
                println!("⚠️ Minecraft crashed or was killed (Exit Code: {})", exit_code);
                println!("💡 Tip: If the code is 1, check for conflicting mods or Java version issues.");
            }
        }
    }

    Ok(())
}