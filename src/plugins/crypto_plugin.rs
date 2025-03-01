//! Crypto plugin for Lake
//!
//! Provides cryptographic functions.

use crate::plugins::Plugin;
use base64::{prelude::BASE64_STANDARD, Engine};
use md5::compute as md5_compute;
use mlua::{Error as LuaError, Lua, Result as LuaResult};
use sha2::{Digest, Sha256, Sha512};
use uuid::Uuid;

pub struct CryptoPlugin;

impl CryptoPlugin {
    pub fn new() -> Self {
        CryptoPlugin
    }
}

fn to_lua_error<E: std::error::Error + Send + Sync + 'static>(e: E, context: &str) -> LuaError {
    log::error!("{}: {}", context, e);
    LuaError::RuntimeError(format!("{}: {}", context, e))
}

impl Plugin for CryptoPlugin {
    fn register(&self, lua: &Lua) -> LuaResult<()> {
        let globals = lua.globals();
        let crypto = lua.create_table()?;

        // hash_sha256 function
        crypto.set(
            "hash_sha256",
            lua.create_function(|_, data: String| {
                return Ok(format!("{:x}", Sha256::digest(data.as_bytes())));
            })?,
        )?;

        // hash_sha512 function
        crypto.set(
            "hash_sha512",
            lua.create_function(|_, data: String| {
                return Ok(format!("{:x}", Sha512::digest(data.as_bytes())));
            })?,
        )?;

        // hash_md5 function
        crypto.set(
            "hash_md5",
            lua.create_function(|_, data: String| {
                return Ok(format!("{:x}", md5_compute(data.as_bytes())));
            })?,
        )?;

        // to_base64 function
        crypto.set(
            "to_base64",
            lua.create_function(|_, data: String| Ok(BASE64_STANDARD.encode(data.as_bytes())))?,
        )?;

        // from_base64 function
        crypto.set(
            "from_base64",
            lua.create_function(|_, data: String| {
                Ok(BASE64_STANDARD
                    .decode(data.as_bytes())
                    .map_err(|e| to_lua_error(e, "Error decoding base64"))?)
            })?,
        )?;

        // from_base64_str
        crypto.set(
            "from_base64_str",
            lua.create_function(|_, data: String| {
                let decoded_bytes = BASE64_STANDARD
                    .decode(data.as_bytes())
                    .map_err(|e| to_lua_error(e, "Error decoding base64"))?;

                let decoded_str = String::from_utf8(decoded_bytes)
                    .map_err(|e| to_lua_error(e, "Decoded base64 is not valid UTF-8"))?;

                Ok(decoded_str)
            })?,
        )?;

        // uuid_v4 function
        crypto.set(
            "uuid_v4",
            lua.create_function(|_, ()| Ok(Uuid::new_v4().to_string()))?,
        )?;

        globals.set("lake.crypto", crypto)?;
        Ok(())
    }

    fn name(&self) -> &str {
        "crypto"
    }
}
