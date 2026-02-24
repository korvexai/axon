use std::process::Command;
use std::path::Path;
use std::fs;
use serde::{Deserialize, Serialize};

use crate::workers::ai_bridge;
 // We assume the bus is required for status logging

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UniversalTask {
    pub target_path: String,
    pub actions: Vec<String>,
    pub ai_instruction: Option<String>,
}

pub struct UniversalCommander;

impl UniversalCommander {
    pub async fn dispatch(task: UniversalTask) -> Result<(), Box<dyn std::error::Error>> {
        let root = Path::new(&task.target_path);
        
        if !root.exists() {
            return Err(format!("PATH_NOT_FOUND: {:?}", root).into());
        }

        for action in &task.actions {
            match action.to_uppercase().as_str() {
                // 1. EXECUTION: Build and error verification
                "BUILD" => Self::execute_build(root).await?,

                // 2. WRITE: AI generates and writes new code
                "WRITE_CODE" => {
                    if let Some(instruction) = &task.ai_instruction {
                        Self::ai_write_module(root, instruction).await?;
                    }
                },

                // 3. REPAIR: Search for bugs and apply automatic patches
                "AUTO_FIX" => Self::run_self_healing(root).await?,

                // 4. FORMAT: Code cleanup
                "FMT" => {
                    let _ = Command::new("cargo").args(["fmt"]).current_dir(root).status();
                },

                _ => println!(">>> [WARN] Unknown action: {}", action),
            }
        }
        Ok(())
    }

    /// Executes 'cargo build' and captures errors to send them to AI
    async fn execute_build(root: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!(">>> [COMMANDER] Running build...");
        let output = Command::new("cargo")
            .args(["build", "--message-format=short"])
            .current_dir(root)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!(">>> [BUILD_FAILED] Errors detected. Triggering AI Repair...");
            // If the build fails, send errors to the auto-fix process
            Self::handle_build_errors(root, &stderr).await?;
        } else {
            println!(">>> [SUCCESS] Build completed successfully.");
        }
        Ok(())
    }

    /// AI writes a new module based on instructions
    async fn ai_write_module(root: &Path, instruction: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!(">>> [AI_WRITE] Task: {}", instruction);
        
        // Ask AI for the raw generated code
        let generated_code: String = ai_bridge::suggest_fix("/* New Module */", instruction).await?;
        
        // Validate the generated code before writing it
        if ai_bridge::validate_fix("/* New Module */", &generated_code).await? {
            let file_path = root.join("src/ai_generated.rs"); // Example destination
            fs::write(&file_path, generated_code)?;
            println!(">>> [SUCCESS] Code written to {:?}", file_path);
        }
        Ok(())
    }

    /// Self-Healing: Takes compilation errors and automatically modifies files
    async fn handle_build_errors(root: &Path, errors: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Error-to-file mapping logic
        // At this stage, the AI Bridge receives the error and decides where to apply the patch
        println!(">>> [HEALER] Analyzing build log...");
        
        // Example: apply patch to main file if errors occur
        let main_path = root.join("src/main.rs").to_str().unwrap().to_string();
        ai_bridge::apply_ai_patch(std::path::Path::new(&main_path), errors.to_string()).await?;
        
        Ok(())
    }

    /// Scans and repairs faulty logic (without waiting for build failure)
    async fn run_self_healing(root: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let _src_path = root.join("src");
        // Find .rs files (using the previous helper method)
        // ... (file scanning logic) ...
        Ok(())
    }
}
