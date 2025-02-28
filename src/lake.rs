use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use mlua::{Function, Lua, LuaOptions, StdLib, Table, Value};

use crate::plugins;
use crate::sandbox;

/// Find the build.lake in the current directory or parent directories
pub fn find_build_file() -> Result<PathBuf> {
    let mut current_dir = env::current_dir()?;

    loop {
        let build_file_path = current_dir.join("build.lake");
        if build_file_path.exists() {
            return Ok(build_file_path);
        }

        if !current_dir.pop() {
            bail!("Could not find build.lake in any parent directory");
        }
    }
}

/// Run the Lake build system with the specified build.lake and task
pub fn run_lake(build_file_path: &Path, task_name: &str, task_args: &[&str]) -> Result<()> {
    if !build_file_path.exists() {
        bail!("build.lake not found at {:?}", build_file_path);
    }

    // Initialize Lua with safe defaults
    let lua = Lua::new_with(StdLib::ALL_SAFE, LuaOptions::new().catch_rust_panics(true))
        .map_err(|e| anyhow::anyhow!("Failed to initialize Lua: {}", e))?;

    // Create a sandbox
    sandbox::create_sandbox(&lua)
        .map_err(|e| anyhow::anyhow!("Failed to create sandbox: {}", e))?;

    // Register core plugins
    plugins::register_all(&lua)
        .map_err(|e| anyhow::anyhow!("Failed to register core plugins: {}", e))?;

    // Load the build.lake
    let build_file_content = fs::read_to_string(build_file_path).context(format!(
        "Failed to read build.lake at {:?}",
        build_file_path
    ))?;

    // Get current directory for relative paths
    let current_dir = build_file_path.parent().unwrap_or(Path::new("."));
    env::set_current_dir(current_dir).context(format!(
        "Failed to set working directory to {:?}",
        current_dir
    ))?;

    // Execute the build.lake
    lua.load(&build_file_content)
        .set_name("build.lake")
        .exec()
        .map_err(|e| anyhow::anyhow!("Failed to execute build.lake: {}", e))?;

    // Execute the requested task
    execute_task(&lua, task_name, task_args)
        .context(format!("Failed to execute task '{}'", task_name))?;

    Ok(())
}

/// Execute a task from the build.lake
fn execute_task(lua: &Lua, task_name: &str, args: &[&str]) -> Result<()> {
    let globals = lua.globals();

    // Get the task registry
    let task_registry: Table = globals.get("__lake_tasks").map_err(|e| {
        anyhow::anyhow!(
            "Task registry not found. Did you define any tasks? Error: {}",
            e
        )
    })?;

    // Check if the task exists
    let task_exists = task_registry
        .contains_key(task_name)
        .map_err(|e| anyhow::anyhow!("Failed to check if task '{}' exists: {}", task_name, e))?;

    if !task_exists {
        bail!("Task '{}' not found in build.lake", task_name);
    }

    // Get the task function
    let task: Function = task_registry
        .get(task_name)
        .map_err(|e| anyhow::anyhow!("Failed to get task '{}': {}", task_name, e))?;

    // Convert arguments to Lua values
    let lua_args: Vec<Value> = args
        .iter()
        .map(|&arg| Value::String(lua.create_string(arg).unwrap()))
        .collect();

    // Execute the task
    task.call::<()>(lua_args)
        .map_err(|e| anyhow::anyhow!("Failed to execute task '{}': {}", task_name, e))?;

    Ok(())
}
