//! Filesystem plugin for Lake
//!
//! Provides file and directory operations.

use crate::plugins::Plugin;
use mlua::{Lua, Result as LuaResult};
use std::path::Path;

pub struct FsPlugin;

impl FsPlugin {
    pub fn new() -> Self {
        FsPlugin
    }
}

impl Plugin for FsPlugin {
    fn register(&self, lua: &Lua) -> LuaResult<()> {
        let globals = lua.globals();
        let fs = lua.create_table()?;

        // mkdir function
        fs.set(
            "mkdir",
            lua.create_function(|_, path: String| match std::fs::create_dir_all(&path) {
                Ok(_) => Ok(true),
                Err(e) => {
                    log::error!("Error creating directory {}: {}", path, e);
                    Ok(false)
                }
            })?,
        )?;

        // rmdir function
        fs.set(
            "rmdir",
            lua.create_function(|_, path: String| {
                let path = Path::new(&path);
                if path.exists() {
                    match std::fs::remove_dir_all(path) {
                        Ok(_) => Ok(true),
                        Err(e) => {
                            log::error!("Error removing directory {:?}: {}", path, e);
                            Ok(false)
                        }
                    }
                } else {
                    Ok(true) // Directory doesn't exist, consider it success
                }
            })?,
        )?;

        // rm function
        fs.set(
            "rm",
            lua.create_function(|_, path: String| match std::fs::remove_file(&path) {
                Ok(_) => Ok(true),
                Err(e) => {
                    log::error!("Error removing file {}: {}", path, e);
                    Ok(false)
                }
            })?,
        )?;

        // copy function
        fs.set(
            "copy",
            lua.create_function(|_, (src, dst): (String, String)| {
                match std::fs::copy(&src, &dst) {
                    Ok(_) => Ok(true),
                    Err(e) => {
                        log::error!("Error copying {} to {}: {}", src, dst, e);
                        Ok(false)
                    }
                }
            })?,
        )?;

        // exists function
        fs.set(
            "exists",
            lua.create_function(|_, path: String| Ok(Path::new(&path).exists()))?,
        )?;

        // is_file function
        fs.set(
            "is_file",
            lua.create_function(|_, path: String| Ok(Path::new(&path).is_file()))?,
        )?;

        // is_dir function
        fs.set(
            "is_dir",
            lua.create_function(|_, path: String| Ok(Path::new(&path).is_dir()))?,
        )?;

        // glob function
        fs.set(
            "glob",
            lua.create_function(|lua, pattern: String| match glob::glob(&pattern) {
                Ok(entries) => {
                    let result_table = lua.create_table()?;
                    for (i, entry) in entries.enumerate() {
                        match entry {
                            Ok(path) => {
                                result_table.set(i + 1, path.to_string_lossy().to_string())?;
                            }
                            Err(e) => {
                                log::error!("Error in glob pattern: {:?}", e);
                            }
                        }
                    }
                    Ok(result_table)
                }
                Err(e) => {
                    log::error!("Invalid glob pattern: {:?}", e);
                    Ok(lua.create_table()?)
                }
            })?,
        )?;

        globals.set("lake.fs", fs)?;
        Ok(())
    }

    fn name(&self) -> &str {
        "filesystem"
    }
}
