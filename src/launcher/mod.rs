use crate::models::piston_meta::PistonMeta;
use crate::dirs::MinecraftDirs;

pub struct LaunchSettings {
    pub username: String,
    pub uuid: String,
    pub access_token: String,
    pub java_path: String, // Either "java" or a custom path to javaw.exe
}

impl LaunchSettings {
  pub fn build_classpath(&self, meta: &PistonMeta, dirs: &MinecraftDirs) -> String {
      let mut cp_parts = Vec::new();

      // 1. Add all compatible libraries to the classpath
      for lib in &meta.libraries {
          if lib.is_compatible() {
              let lib_path = dirs.libraries_dir().join(&lib.downloads.artifact.path);
              if let Some(path_str) = lib_path.to_str() {
                  cp_parts.push(path_str.to_string());
              }
          }
      }

      // 2. Add the client jar to the classpath
      let client_jar = dirs.client_jar_path(&meta.id);
      if let Some(client_str) = client_jar.to_str() {
          cp_parts.push(client_str.to_string());
      }

      // 3. Join all parts with the appropriate separator for the OS
      #[cfg(target_os = "windows")]
      let separator = ";";
      #[cfg(not(target_os = "windows"))]
      let separator = ":";

      cp_parts.join(separator)
  }
}

impl LaunchSettings {
  pub fn create_command(
    &self, 
    meta: &PistonMeta, 
    dirs: &MinecraftDirs
) -> Result<std::process::Command, Box<dyn std::error::Error>> {
    
    let classpath = self.build_classpath(meta, dirs);
    let mut cmd = std::process::Command::new(&self.java_path);

    // 1. JVM Arguments
    cmd.arg("-Xmx2G");
    cmd.arg("-Xms1G");
    
    // Normalize the library path to avoid issues with trailing backslashes on Windows
    let lib_path = dirs.libraries_dir().to_string_lossy().trim_end_matches('\\').to_string();
    
    // NOTE: library path must be normalized to avoid issues with trailing backslashes on Windows, which can cause the JVM to misinterpret the path
    cmd.arg(format!("-Djava.library.path={}", lib_path));
    
    cmd.arg("-cp");
    cmd.arg(classpath); 
    
    // 3. Main class
    cmd.arg(&meta.main_class);

    // 4. Args for the game
    cmd.arg("--username").arg(&self.username);
    cmd.arg("--version").arg(&meta.id);
    cmd.arg("--gameDir").arg(dirs.base.to_string_lossy().as_ref());
    cmd.arg("--assetsDir").arg(dirs.assets_dir());
    cmd.arg("--assetIndex").arg(&meta.asset_index.id);
    cmd.arg("--uuid").arg(&self.uuid);
    cmd.arg("--accessToken").arg(&self.access_token);
    cmd.arg("--userType").arg("mojang");
    cmd.arg("--versionType").arg("release");

    Ok(cmd)
}
}
impl LaunchSettings {
  pub fn offline(username: &str) -> Self {
      Self {
          username: username.to_string(),
          // fake UUID and token for offline mode, since Minecraft doesn't validate them in offline mode
          uuid: "00000000-0000-0000-0000-000000000000".to_string(),
          access_token: "oxidite_token".to_string(),
          // On windows, "java" will resolve to java.exe, and on other OSes it will resolve to the appropriate java binary. This allows users to have Java in their PATH without needing to specify the full path.
          java_path: "java.exe".to_string(), 
      }
  }
}

