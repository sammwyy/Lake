//! Process plugin for Lake
//!
//! Provides functionality to execute external processes.

use crate::plugins::Plugin;
use mlua::{Lua, Result as LuaResult, Table};
use std::process::{Command, Stdio};

pub struct ProcessPlugin;

impl ProcessPlugin {
    pub fn new() -> Self {
        ProcessPlugin
    }
}

impl Plugin for ProcessPlugin {
    fn register(&self, lua: &Lua) -> LuaResult<()> {
        let globals = lua.globals();
        let process = lua.create_table()?;

        // exec function
        process.set(
            "exec",
            lua.create_function(|lua, (cmd, args): (String, Option<Table>)| {
                let args_vec: Vec<String> = match args {
                    Some(args_table) => {
                        let mut result = Vec::new();
                        for i in 1..=args_table.len()? {
                            if let Ok(arg) = args_table.get(i) {
                                result.push(arg);
                            }
                        }
                        result
                    }
                    None => Vec::new(),
                };

                let output = Command::new(&cmd).args(&args_vec).output();

                match output {
                    Ok(output) => {
                        let result = lua.create_table()?;
                        result.set("status", output.status.code().unwrap_or(-1))?;
                        result.set(
                            "stdout",
                            String::from_utf8_lossy(&output.stdout).to_string(),
                        )?;
                        result.set(
                            "stderr",
                            String::from_utf8_lossy(&output.stderr).to_string(),
                        )?;
                        Ok(result)
                    }
                    Err(e) => {
                        log::error!("Error executing process: {}", e);
                        let result = lua.create_table()?;
                        result.set("status", -1)?;
                        result.set("stdout", "")?;
                        result.set("stderr", format!("Failed to execute process: {}", e))?;
                        Ok(result)
                    }
                }
            })?,
        )?;

        // exec_piped function - Execute command and return handle
        process.set(
            "exec_piped",
            lua.create_function(|lua, (cmd, args): (String, Option<Table>)| {
                let args_vec: Vec<String> = match args {
                    Some(args_table) => {
                        let mut result = Vec::new();
                        for i in 1..=args_table.len()? {
                            if let Ok(arg) = args_table.get(i) {
                                result.push(arg);
                            }
                        }
                        result
                    }
                    None => Vec::new(),
                };

                let child = Command::new(&cmd)
                    .args(&args_vec)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn();

                match child {
                    Ok(_) => {
                        // In a real implementation, we'd return a handle that allows
                        // reading from stdout/stderr and checking status
                        let result = lua.create_table()?;
                        result.set("pid", 0)?; // Placeholder
                        Ok(result)
                    }
                    Err(e) => {
                        log::error!("Error executing process: {}", e);
                        let result = lua.create_table()?;
                        result.set("pid", -1)?;
                        Ok(result)
                    }
                }
            })?,
        )?;

        globals.set("lake.process", process)?;
        Ok(())
    }

    fn name(&self) -> &str {
        "process"
    }
}
