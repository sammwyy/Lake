use mlua::{Lua, Result as LuaResult, Table};

/// Create a sand-boxed environment for Lua scripts
pub fn create_sandbox(lua: &Lua) -> LuaResult<()> {
    let globals = lua.globals();

    // Set up the task registry
    let task_registry = lua.create_table()?;
    globals.set("__lake_tasks", task_registry)?;

    // Define print functions
    globals.set(
        "print",
        lua.create_function(|_, message: String| {
            println!("{}", message);
            Ok(())
        })?,
    )?;

    // Define plugin loader function
    globals.set(
        "plugin",
        lua.create_function(|lua, name: String| load_plugin(lua, name))?,
    )?;

    // Define task registration function
    globals.set(
        "task",
        lua.create_function(|lua, (name, func): (String, mlua::Function)| {
            let globals = lua.globals();
            let task_registry: Table = globals.get("__lake_tasks")?;
            task_registry.set(name.clone(), func)?;
            log::debug!("Registered task: {}", name);
            Ok(())
        })?,
    )?;

    Ok(())
}

/// Load a plugin by name
fn load_plugin(lua: &Lua, name: String) -> LuaResult<Table> {
    let globals = lua.globals();

    // Check if it's a core plugin with dot notation
    if name.starts_with("lake.") {
        let plugin_table: Table = globals.get(&*name)?;
        return Ok(plugin_table);
    }

    // Try to load from plugins directory
    // For now, we'll just check if it's a predefined plugin
    let plugin_name = format!("lake.{}", name);
    let plugin_table: mlua::Result<Table> = globals.get(&*plugin_name);

    match plugin_table {
        Ok(table) => Ok(table),
        Err(_) => {
            // Could be a user-defined plugin
            // Try to load plugin from Lua file
            let plugin_path = format!("plugins/{}.lua", name);
            match std::fs::read_to_string(&plugin_path) {
                Ok(content) => {
                    let result = lua.load(&content).set_name(&plugin_path).eval::<Table>()?;
                    log::debug!("Loaded external plugin: {}", name);
                    Ok(result)
                }
                Err(_) => {
                    // Return an empty table as fallback
                    log::warn!(
                        "Warning: Plugin '{}' not found, returning empty table",
                        name
                    );
                    Ok(lua.create_table()?)
                }
            }
        }
    }
}
