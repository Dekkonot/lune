use mlua::prelude::*;

use crate::lua::{regex::LuaRegex, table::TableBuilder};

pub fn create(lua: &'static Lua) -> LuaResult<LuaTable<'static>> {
    TableBuilder::new(lua)?
        .with_async_function("new", regex_new)?
        .build_readonly()
}

async fn regex_new(_lua: &'static Lua, pattern: String) -> LuaResult<LuaRegex> {
    LuaRegex::new(&pattern)
}
