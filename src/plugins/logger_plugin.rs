//! Logger plugin for Lake
//!
//! Provides logging functionality.

use crate::plugins::Plugin;
use mlua::{Lua, Result as LuaResult};

pub struct LoggerPlugin;

impl LoggerPlugin {
    pub fn new() -> Self {
        LoggerPlugin
    }
}

impl Plugin for LoggerPlugin {
    fn register(&self, lua: &Lua) -> LuaResult<()> {
        let globals = lua.globals();
        let logger = lua.create_table()?;

        // info level
        logger.set(
            "info",
            lua.create_function(|_, message: String| {
                log::info!("{}", message);
                Ok(())
            })?,
        )?;

        // debug level
        logger.set(
            "debug",
            lua.create_function(|_, message: String| {
                log::debug!("{}", message);
                Ok(())
            })?,
        )?;

        // trace level
        logger.set(
            "trace",
            lua.create_function(|_, message: String| {
                log::trace!("{}", message);
                Ok(())
            })?,
        )?;

        // error level
        logger.set(
            "error",
            lua.create_function(|_, message: String| {
                log::error!("{}", message);
                Ok(())
            })?,
        )?;

        // warn level
        logger.set(
            "warn",
            lua.create_function(|_, message: String| {
                log::warn!("{}", message);
                Ok(())
            })?,
        )?;

        globals.set("lake.logger", logger)?;
        Ok(())
    }

    fn name(&self) -> &str {
        "logger"
    }
}
