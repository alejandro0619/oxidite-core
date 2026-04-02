use clap::Parser;
use std::path::PathBuf;

mod dirs;
mod downloader;
mod sources;
mod models;
mod libraries;

use downloader::Downloader;
use models::manifest::Manifest;
use models::piston_meta::PistonMeta;
use sources::Sources;

const DEFAULT_PATH: &str = "C:\\.minecraft_oxidite"; 

#[derive(Parser, Debug)]
#[command(author, about = "Oxidite - Minecraft Launcher in Rust")]
struct Args {

    #[arg(short, long, default_value = DEFAULT_PATH)]
    path: PathBuf,


    #[arg(short, long)]
    version: Option<String>,


    #[arg(short, long, default_value = "LATER I NEED TO GEN SOME NAME lmao")]
    username: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();


    let options = dirs::CreateDirOptions::new(args.version.clone());
    dirs::create_dir(&args.path, options).await?;


    let dl = Downloader::new();


    println!("📥 Cargando manifiesto de versiones...");
    let manifest: Manifest = dl.download(Sources::Manifest).await?;


    let target_id = args.version.unwrap_or_else(|| manifest.latest.release.clone());
    
    println!("🔍 Searching information about this version{}", target_id);

    if let Some(entry) = manifest.find_version(&target_id) {
        let hash = entry
            .get_hash()
            .ok_or("Could not extract hash from manifest URL")?;

        println!("✅ Hash found: {}", hash);


        let meta: PistonMeta = dl
            .download(Sources::PistonMeta {
                hash: hash.to_string(),
                version: entry.id.clone(),
            })
            .await?;

        println!("🚀 Processing version: {} (Main Class: {})", target_id, meta.main_class);


        println!("📚 Verifying libraries for Windows...");
        libraries::download_all(&meta, &args.path, &dl).await?;

        println!("\n✨ Sync completed successfully.");
        println!("📂 Files located in: {}", args.path.display());
        
    } else {
        eprintln!("❌ Error: The version '{}' does not exist in the Mojang manifest.", target_id);
    }

    Ok(())
}