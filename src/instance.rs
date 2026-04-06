use std::process::{Child};
use std::time::{Instant};
use std::io::{BufRead, BufReader};
use std::sync::mpsc::{self, Receiver};

pub struct GameInstance {
    pub version: String,
    pub username: String,
    pub start_time: Instant,
    pub process: Child,
    // Channel to receive status updates from the log parser thread
    pub status_receiver: Receiver<GameStatus>,
}

#[derive(Debug, Clone)]
pub enum GameStatus {
    Singleplayer,
    Multiplayer { ip: String },
    Menu,
}

impl GameInstance {
    pub fn new(version: String, username: String, mut process: Child) -> Self {
        let (tx, rx) = mpsc::channel();
        
        // Captre the stdout of the Minecraft process to parse logs
        if let Some(stdout) = process.stdout.take() {
            let reader = BufReader::new(stdout);
            
            // Create a background thread to parse the logs and send status updates
            std::thread::spawn(move || {
                for line in reader.lines() {
                    if let Ok(l) = line {
                        // Log pasing 
                        if l.contains("Stopping @") || l.contains("Saving chunks for level") {
                            let _ = tx.send(GameStatus::Singleplayer);
                        } else if l.contains("Connecting to") {
                            // Ex: [Client thread/INFO]: Connecting to localhost, 25565
                            let ip = l.split("Connecting to ").nth(1)
                                .unwrap_or("Unknown")
                                .split(',')
                                .next()
                                .unwrap_or("Unknown")
                                .trim()
                                .to_string();
                            let _ = tx.send(GameStatus::Multiplayer { ip });
                        } else if l.contains("Returning to meta-menu") {
                            let _ = tx.send(GameStatus::Menu);
                        }
                    }
                }
            });
        }

        Self {
            version,
            username,
            start_time: Instant::now(),
            process,
            status_receiver: rx,
        }
    }

    pub fn is_running(&mut self) -> bool {
        match self.process.try_wait() {
            Ok(None) => true,
            _ => false,
        }
    }
    /// Retrieves the exit code of the Minecraft process.
    /// Returns Some(i32) if the game finished, or None if it's still running.
    pub fn exit_code(&mut self) -> Option<i32> {
        match self.process.try_wait() {
            Ok(Some(status)) => status.code(), // Extracts the integer code (0, 1, etc.)
            _ => None, // Still running or couldn't retrieve status
        }
    }
}