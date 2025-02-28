//! Environment plugin for Lake
//!
//! Provides access to environment variables and system information.

use crate::plugins::Plugin;
use mlua::{Lua, Result as LuaResult};

pub struct EnvPlugin;

impl EnvPlugin {
    pub fn new() -> Self {
        EnvPlugin
    }
}

impl Plugin for EnvPlugin {
    fn register(&self, lua: &Lua) -> LuaResult<()> {
        let globals = lua.globals();
        let env = lua.create_table()?;

        // get function
        env.set(
            "get",
            lua.create_function(|_, name: String| match std::env::var(&name) {
                Ok(value) => Ok(Some(value)),
                Err(_) => Ok(None),
            })?,
        )?;

        // set function
        env.set(
            "set",
            lua.create_function(|_, (name, value): (String, String)| {
                std::env::set_var(name, value);
                Ok(())
            })?,
        )?;

        // os function
        env.set(
            "os",
            lua.create_function(|_, ()| {
                #[cfg(target_os = "windows")]
                return Ok("windows");

                #[cfg(target_os = "macos")]
                return Ok("macos");

                #[cfg(target_os = "linux")]
                return Ok("linux");

                #[cfg(not(any(
                    target_os = "windows",
                    target_os = "macos",
                    target_os = "linux"
                )))]
                return Ok("unknown");
            })?,
        )?;

        globals.set("lake.env", env)?;
        Ok(())
    }

    fn name(&self) -> &str {
        "environment"
    }
}
