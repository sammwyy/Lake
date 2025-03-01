//! Plugins module for Lake
//!
//! Handles registration and management of core plugins.

use mlua::{Lua, Result as LuaResult};

mod crypto_plugin;
mod env_plugin;
mod fs_plugin;
mod logger_plugin;
mod net_plugin;
mod process_plugin;
mod random_plugin;

/// API for registering plugins
pub trait Plugin {
    /// Register the plugin with the Lua state
    fn register(&self, lua: &Lua) -> LuaResult<()>;

    /// Get the plugin name
    fn name(&self) -> &str;
}

/// Register all core plugins
pub fn register_all(lua: &Lua) -> LuaResult<()> {
    // Create instances of all core plugins
    let plugins: Vec<Box<dyn Plugin>> = vec![
        Box::new(crypto_plugin::CryptoPlugin::new()),
        Box::new(fs_plugin::FsPlugin::new()),
        Box::new(process_plugin::ProcessPlugin::new()),
        Box::new(env_plugin::EnvPlugin::new()),
        Box::new(net_plugin::NetPlugin::new()),
        Box::new(logger_plugin::LoggerPlugin::new()),
        Box::new(random_plugin::RandomPlugin::new()),
    ];

    // Register each plugin
    for plugin in plugins {
        plugin.register(lua)?;
        log::debug!("Registered plugin: {}", plugin.name());
    }

    Ok(())
}
