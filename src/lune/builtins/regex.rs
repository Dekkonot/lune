use mlua::prelude::*;

use crate::lune::lua::{regex::LuaRegex, table::TableBuilder};

pub fn create(lua: &'static Lua) -> LuaResult<LuaTable> {
    TableBuilder::new(lua)?
        .with_function("new", new_regex)?
        .build_readonly()
}

fn new_regex(_: &Lua, pattern: String) -> LuaResult<LuaRegex> {
    LuaRegex::new(&pattern)
}
