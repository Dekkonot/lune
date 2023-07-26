mod captures;
mod matches;

use std::sync::Arc;

use mlua::prelude::*;
use regex::Regex;

use self::captures::LuaCaptures;
use self::matches::LuaMatch;

/// Represents a wrapper around a Regex pattern for use in the runtime.
#[derive(Debug)]
pub struct LuaRegex {
    inner: Regex,
}

impl LuaRegex {
    /// Constructs a new `LuaRegex` from the provided pattern.
    pub fn new(pattern: &str) -> LuaResult<Self> {
        Ok(Self {
            inner: Regex::new(pattern).map_err(LuaError::external)?,
        })
    }
}

impl LuaUserData for LuaRegex {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("isMatch", |lua, this, text: String| {
            this.inner.is_match(&text).into_lua(lua)
        });

        methods.add_method("find", |lua, this, text: String| {
            let arc = Arc::new(text);
            match this.inner.find(&arc) {
                Some(mtch) => LuaMatch::new(Arc::clone(&arc), mtch).into_lua(lua),
                None => Ok(LuaNil),
            }
        });

        methods.add_method("captures", |lua, this, text: String| {
            LuaCaptures::new(&this.inner, text).into_lua(lua)
        });

        methods.add_method("split", |lua, this, text: String| {
            this.inner.split(&text).collect::<Vec<&str>>().into_lua(lua)
        });

        // TODO determine whether it's desirable and feasible to support
        // using a function or table for `replace` like in the string library
        methods.add_method("replace", |lua, this, arg: (String, String)| {
            let (text, replacer) = arg;
            this.inner.replace(&text, replacer).into_lua(lua)
        });

        methods.add_method("replaceAll", |lua, this, arg: (String, String)| {
            let (text, replacer) = arg;
            this.inner.replace_all(&text, replacer).into_lua(lua)
        });

        methods.add_meta_method(LuaMetaMethod::ToString, |lua, this, ()| {
            format!("Regex({})", this.inner.as_str()).into_lua(lua)
        });
    }

    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("pattern", |lua, this| this.inner.as_str().into_lua(lua));

        fields.add_meta_field("__type", "Regex");
    }
}
