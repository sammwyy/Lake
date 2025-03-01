//! Filesystem plugin for Lake
//!
//! Provides file and directory operations.

use crate::plugins::Plugin;
use mlua::{Error as LuaError, Lua, Result as LuaResult};
use std::path::Path;

pub struct FsPlugin;

impl FsPlugin {
    pub fn new() -> Self {
        FsPlugin
    }
}

fn to_lua_error<E: std::error::Error + Send + Sync + 'static>(e: E, context: &str) -> LuaError {
    log::error!("{}: {}", context, e);
    LuaError::RuntimeError(format!("{}: {}", context, e))
}

impl Plugin for FsPlugin {
    fn register(&self, lua: &Lua) -> LuaResult<()> {
        let globals = lua.globals();
        let fs = lua.create_table()?;

        // mkdir function
        fs.set(
            "mkdir",
            lua.create_function(|_, path: String| {
                std::fs::create_dir_all(&path)
                    .map(|_| true)
                    .map_err(|e| to_lua_error(e, &format!("Error creating directory {}", path)))
            })?,
        )?;

        // rmdir function
        fs.set(
            "rmdir",
            lua.create_function(|_, path: String| {
                let path = Path::new(&path);
                if path.exists() {
                    std::fs::remove_dir_all(path).map(|_| true).map_err(|e| {
                        to_lua_error(e, &format!("Error removing directory {:?}", path))
                    })
                } else {
                    Ok(true)
                }
            })?,
        )?;

        // rm function
        fs.set(
            "rm",
            lua.create_function(|_, path: String| {
                std::fs::remove_file(&path)
                    .map(|_| true)
                    .map_err(|e| to_lua_error(e, &format!("Error removing file {}", path)))
            })?,
        )?;

        // copy function
        fs.set(
            "copy",
            lua.create_function(|_, (src, dst): (String, String)| {
                std::fs::copy(&src, &dst)
                    .map(|_| true)
                    .map_err(|e| to_lua_error(e, &format!("Error copying {} to {}", src, dst)))
            })?,
        )?;

        // exists function (no errors to propagate)
        fs.set(
            "exists",
            lua.create_function(|_, path: String| Ok(Path::new(&path).exists()))?,
        )?;

        // is_file function (no errors to propagate)
        fs.set(
            "is_file",
            lua.create_function(|_, path: String| Ok(Path::new(&path).is_file()))?,
        )?;

        // is_dir function (no errors to propagate)
        fs.set(
            "is_dir",
            lua.create_function(|_, path: String| Ok(Path::new(&path).is_dir()))?,
        )?;

        // glob function
        fs.set(
            "glob",
            lua.create_function(|lua, pattern: String| {
                let entries = glob::glob(&pattern).map_err(|e| {
                    to_lua_error(e, &format!("Invalid glob pattern: {:?}", pattern))
                })?;

                let result_table = lua.create_table()?;
                for (i, entry) in entries.enumerate() {
                    match entry {
                        Ok(path) => {
                            result_table.set(i + 1, path.to_string_lossy().to_string())?;
                        }
                        Err(e) => {
                            return Err(to_lua_error(e, "Error in glob pattern"));
                        }
                    }
                }
                Ok(result_table)
            })?,
        )?;

        // read_file function
        fs.set(
            "read_file",
            lua.create_function(|_, path: String| {
                std::fs::read_to_string(&path)
                    .map_err(|e| to_lua_error(e, &format!("Error reading file {}", path)))
            })?,
        )?;

        // write_file function
        fs.set(
            "write_file",
            lua.create_function(|_, (path, content): (String, String)| {
                std::fs::write(&path, content)
                    .map(|_| true)
                    .map_err(|e| to_lua_error(e, &format!("Error writing file {}", path)))
            })?,
        )?;

        // list_dir function
        fs.set(
            "list_dir",
            lua.create_function(|lua, path: String| {
                let entries = std::fs::read_dir(&path)
                    .map_err(|e| to_lua_error(e, &format!("Error listing directory {}", path)))?;

                let result_table = lua.create_table()?;
                for (i, entry) in entries.enumerate() {
                    match entry {
                        Ok(entry) => {
                            result_table.set(i + 1, entry.path().to_string_lossy().to_string())?;
                        }
                        Err(e) => {
                            return Err(to_lua_error(e, "Error reading directory entry"));
                        }
                    }
                }
                Ok(result_table)
            })?,
        )?;

        globals.set("lake.fs", fs)?;
        Ok(())
    }

    fn name(&self) -> &str {
        "filesystem"
    }
}
