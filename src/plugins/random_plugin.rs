//! Random plugin for Lake
//!
//! Provides random value generation functions.

use crate::plugins::Plugin;
use mlua::{Error as LuaError, Lua, Result as LuaResult};
use rand::{
    distr::{Alphanumeric, Uniform},
    Rng,
};

pub struct RandomPlugin;

impl RandomPlugin {
    pub fn new() -> Self {
        RandomPlugin
    }
}

fn to_lua_error<E: std::error::Error + Send + Sync + 'static>(e: E, context: &str) -> LuaError {
    log::error!("{}: {}", context, e);
    LuaError::RuntimeError(format!("{}: {}", context, e))
}

impl Plugin for RandomPlugin {
    fn register(&self, lua: &Lua) -> LuaResult<()> {
        let globals = lua.globals();
        let random = lua.create_table()?;

        // rnd_int function
        random.set(
            "rnd_int",
            lua.create_function(|_, (min, max): (i64, i64)| {
                if min >= max {
                    return Err(LuaError::RuntimeError(
                        "min must be less than max for rnd_int".to_string(),
                    ));
                }
                Ok(rand::rng().random_range(min..max))
            })?,
        )?;

        // rnd_string function
        random.set(
            "rnd_string",
            lua.create_function(|_, length: i64| {
                if length <= 0 {
                    return Err(LuaError::RuntimeError(
                        "length must be positive for rnd_string".to_string(),
                    ));
                }
                let rng = rand::rng();
                let random_string: String = rng
                    .sample_iter(&Alphanumeric)
                    .take(length as usize)
                    .map(char::from)
                    .collect();
                Ok(random_string)
            })?,
        )?;

        // rnd_bool function
        random.set(
            "rnd_bool",
            lua.create_function(|_, ()| Ok(rand::rng().random_bool(0.5)))?,
        )?;

        // rnd_float function
        random.set(
            "rnd_float",
            lua.create_function(
                |_, args: mlua::MultiValue| match (args.get(0), args.get(1)) {
                    (None, None) => Ok(rand::random::<f64>()),
                    (Some(mlua::Value::Number(min)), Some(mlua::Value::Number(max))) => {
                        let min = *min;
                        let max = *max;
                        if min >= max {
                            return Err(LuaError::RuntimeError(
                                "min must be less than max for rnd_float".to_string(),
                            ));
                        }
                        let dist = Uniform::new(min, max)
                            .map_err(|e| to_lua_error(e, "Error creating distribution"))?;
                        Ok(rand::rng().sample(dist))
                    }
                    _ => Err(LuaError::RuntimeError(
                        "rnd_float expects either no arguments or (min, max) as numbers"
                            .to_string(),
                    )),
                },
            )?,
        )?;

        globals.set("lake.random", random)?;
        Ok(())
    }

    fn name(&self) -> &str {
        "random"
    }
}
