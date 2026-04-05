/// This example demonstrates how to check if a specific Minecraft version is compatible with a given Java major version using the Oxidite library. It retrieves the required Java version for the target Minecraft version and compares it with the user's Java version to determine compatibility.
/// This is hardcoded to fail for demonstration purposes, as Java 21 is not compatible with Minecraft 26.1.1, which requires java 25 at least.

use std::path::PathBuf;

use oxidite_core::{Oxidite, models::OxiditeConfig};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let launcher = Oxidite::new(OxiditeConfig {
        base_path: PathBuf::from("C:\\.minecraft_oxidite"),
        java_path: "java".to_string(),
        max_parallel_downloads: 20,
        memory_gb: 4,
        extra_jvm_args: vec![],
    });

    let target_v = "26.1.1";

    // Checking compatibility for the target version with Java 21
    println!("Checking compatibility for {}...", target_v);
    
    // Assuming the user has Java 21 installed, we check if it's compatible with the target Minecraft version
    let is_ok = launcher.check_compatibility(target_v, 21).await?;

    if is_ok {
        println!("✅ Java compatible. Proceeding...");
        
    } else {
        let req = launcher.get_required_java_version(target_v).await?;
        println!("❌ Incompatible. At least Java {}", req);
    }

    Ok(())
}