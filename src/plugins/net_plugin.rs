//! Network plugin for Lake
//!
//! Provides network functionality such as downloading files.

use crate::plugins::Plugin;
use mlua::{Lua, Result as LuaResult};

pub struct NetPlugin;

impl NetPlugin {
    pub fn new() -> Self {
        NetPlugin
    }
}

impl Plugin for NetPlugin {
    fn register(&self, lua: &Lua) -> LuaResult<()> {
        let globals = lua.globals();
        let net = lua.create_table()?;

        // download function
        net.set(
            "download",
            lua.create_function(|_, (url, path): (String, String)| {
                // This is a basic implementation using blocking reqwest
                // In a real implementation, you would use async and better error handling
                match reqwest::blocking::get(&url) {
                    Ok(response) => {
                        if response.status().is_success() {
                            match response.bytes() {
                                Ok(bytes) => match std::fs::write(&path, &bytes) {
                                    Ok(_) => Ok(true),
                                    Err(e) => {
                                        log::error!("Error writing file {}: {}", path, e);
                                        Ok(false)
                                    }
                                },
                                Err(e) => {
                                    log::error!("Error downloading content: {}", e);
                                    Ok(false)
                                }
                            }
                        } else {
                            log::error!("Error downloading {}: Status {}", url, response.status());
                            Ok(false)
                        }
                    }
                    Err(e) => {
                        log::error!("Error downloading {}: {}", url, e);
                        Ok(false)
                    }
                }
            })?,
        )?;

        globals.set("lake.net", net)?;
        Ok(())
    }

    fn name(&self) -> &str {
        "network"
    }
}
