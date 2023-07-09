mod matches;

use mlua::prelude::*;
use regex::Regex;

use self::matches::LuaMatches;

#[derive(Debug)]
pub struct LuaRegex {
    inner: Regex,
}

impl LuaRegex {
    pub fn new(pattern: &str) -> LuaResult<Self> {
        Ok(Self {
            inner: Regex::new(pattern).map_err(LuaError::external)?,
        })
    }
}

impl LuaUserData for LuaRegex {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("ismatch", |lua, this, text: String| {
            this.inner.is_match(&text).into_lua(lua)
        });

        methods.add_method("find", |lua, this, text: String| {
            let matches = LuaMatches::new(&this.inner, text);
            matches.get(0).into_lua(lua)
        });

        methods.add_method("findall", |lua, this, text: String| {
            LuaMatches::new(&this.inner, text).into_lua(lua)
        });
    }
}
