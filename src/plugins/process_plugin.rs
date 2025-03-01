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

        // spawn function (returns pid)
        process.set(
            "spawn",
            lua.create_function(|_, (cmd, args): (String, Option<Table>)| {
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

                let output = Command::new(&cmd)
                    .args(&args_vec)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn();

                match output {
                    Ok(output) => Ok(output.id() as i32),
                    Err(e) => {
                        log::error!("Error executing process: {}", e);
                        Ok(-1)
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
