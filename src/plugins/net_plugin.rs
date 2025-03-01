//! Network plugin for Lake
//!
//! Provides network functionality such as downloading files.

use crate::plugins::Plugin;
use mlua::{Error as LuaError, Lua, Result as LuaResult, Table, Value};
use reqwest::blocking::{Client, RequestBuilder, Response};

pub struct NetPlugin;

impl NetPlugin {
    pub fn new() -> Self {
        NetPlugin
    }
}

// Function to convert Rust errors to Lua errors
fn to_lua_error<E: std::error::Error + Send + Sync + 'static>(e: E, context: &str) -> LuaError {
    log::error!("{}: {}", context, e);
    LuaError::RuntimeError(format!("{}: {}", context, e))
}

// Helper function to extract URL from args
fn extract_url(args: &mlua::MultiValue, arg_pos: usize) -> LuaResult<String> {
    match args.get(arg_pos) {
        Some(Value::String(s)) => Ok(s.to_str()?.to_string()),
        _ => Err(LuaError::RuntimeError("URL must be a string".to_string())),
    }
}

// Helper function to extract optional headers table
fn extract_headers_table<'lua>(
    args: &mlua::MultiValue,
    arg_pos: usize,
) -> LuaResult<Option<&Table>> {
    match args.get(arg_pos) {
        Some(Value::Table(t)) => Ok(Some(t)),
        None => Ok(None),
        _ => Err(LuaError::RuntimeError(
            "Headers must be a table".to_string(),
        )),
    }
}

// Helper function to add headers from a Lua table to a reqwest RequestBuilder
fn add_headers_from_table(
    mut req_builder: RequestBuilder,
    headers_table: &Table,
) -> LuaResult<RequestBuilder> {
    for pair in headers_table.pairs::<String, String>() {
        let (key, value) = pair?;
        req_builder = req_builder.header(key, value);
    }

    Ok(req_builder)
}

// Helper function to create a response table from an HTTP response
fn create_response_table<'lua>(lua: &'lua Lua, response: Response) -> LuaResult<Table> {
    let status = response.status().as_u16();
    let headers = response.headers().clone();

    let body = response
        .text()
        .map_err(|e| to_lua_error(e, "Error reading response body"))?;

    let response_table = lua.create_table()?;
    response_table.set("status", status)?;
    response_table.set("body", body)?;

    // Add headers to response table
    let headers_table = lua.create_table()?;
    for (name, value) in headers {
        if let Ok(value_str) = value.to_str() {
            headers_table.set(name.unwrap().as_str(), value_str)?;
        }
    }
    response_table.set("headers", headers_table)?;

    Ok(response_table)
}

impl Plugin for NetPlugin {
    fn register(&self, lua: &Lua) -> LuaResult<()> {
        let globals = lua.globals();
        let net = lua.create_table()?;

        // download function
        net.set(
            "download",
            lua.create_function(|_, (url, path): (String, String)| {
                let response = reqwest::blocking::get(&url)
                    .map_err(|e| to_lua_error(e, &format!("Error downloading from {}", url)))?;

                if !response.status().is_success() {
                    return Err(LuaError::RuntimeError(format!(
                        "Error downloading {}: Status {}",
                        url,
                        response.status()
                    )));
                }

                let bytes = response
                    .bytes()
                    .map_err(|e| to_lua_error(e, "Error downloading content"))?;

                std::fs::write(&path, &bytes)
                    .map(|_| true)
                    .map_err(|e| to_lua_error(e, &format!("Error writing file {}", path)))
            })?,
        )?;

        // http_get function
        net.set(
            "http_get",
            lua.create_function(|lua, args: mlua::MultiValue| {
                let url = extract_url(&args, 0)?;
                let headers_table = extract_headers_table(&args, 1)?;

                // Build the request
                let client = Client::new();
                let mut req_builder = client.get(&url);

                // Add headers if provided
                if let Some(headers) = headers_table {
                    req_builder = add_headers_from_table(req_builder, headers)?;
                }

                // Execute the request
                let response = req_builder
                    .send()
                    .map_err(|e| to_lua_error(e, &format!("Error in GET request to {}", url)))?;

                create_response_table(lua, response)
            })?,
        )?;

        // http_post function
        net.set(
            "http_post",
            lua.create_function(|lua, args: mlua::MultiValue| {
                let url = extract_url(&args, 0)?;

                // Extract body
                let body = match args.get(1) {
                    Some(Value::String(s)) => s.to_str()?.to_string(),
                    _ => return Err(LuaError::RuntimeError("Body must be a string".to_string())),
                };

                let headers_table = extract_headers_table(&args, 2)?;

                // Build the request
                let client = Client::new();
                let mut req_builder = client.post(&url).body(body);

                // Add headers if provided
                if let Some(headers) = headers_table {
                    req_builder = add_headers_from_table(req_builder, headers)?;
                }

                // Execute the request
                let response = req_builder
                    .send()
                    .map_err(|e| to_lua_error(e, &format!("Error in POST request to {}", url)))?;

                create_response_table(lua, response)
            })?,
        )?;

        globals.set("lake.net", net)?;
        Ok(())
    }

    fn name(&self) -> &str {
        "network"
    }
}
