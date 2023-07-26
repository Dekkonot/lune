use std::{fmt, ops::Range, sync::Arc};

use mlua::prelude::*;
use regex::Match;

pub struct LuaMatch {
    text: Arc<String>,
    start: usize,
    end: usize,
}

impl LuaMatch {
    pub fn new(text: Arc<String>, mtch: Match) -> Self {
        Self {
            text,
            start: mtch.start(),
            end: mtch.end(),
        }
    }

    pub fn range(&self) -> Range<usize> {
        self.start..self.end
    }
}

impl LuaUserData for LuaMatch {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("start", |lua, this| {
            // Strings are 0 based in Rust but 1 based in Luau
            // TODO account for overflows
            (this.start + 1).into_lua(lua)
        });
        fields.add_field_method_get("finish", |lua, this| this.end.into_lua(lua));
        fields.add_field_method_get("len", |lua, this| this.range().len().into_lua(lua));
        fields.add_field_method_get("text", |lua, this| this.text[this.range()].into_lua(lua));

        fields.add_meta_field("__type", "RegexMatch")
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("isEmpty", |lua, this, ()| {
            this.range().is_empty().into_lua(lua)
        });

        methods.add_meta_method(LuaMetaMethod::Len, |lua, this, ()| {
            this.range().len().into_lua(lua)
        });
        methods.add_meta_method(LuaMetaMethod::ToString, |_, this, ()| Ok(this.to_string()));
    }
}

impl fmt::Display for LuaMatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Match({})", &self.text[self.range()])
    }
}
