use std::{fmt, ops::Range, sync::Arc};

use mlua::prelude::*;
use regex::Regex;

#[derive(Debug)]
/// A sequence of matches over a string.
pub struct LuaMatches {
    /// The String that the matches are over
    text: Arc<String>,
    /// A list of the ranges where matches are in `text`
    captures: Vec<Range<usize>>,
}

impl LuaMatches {
    pub fn new(pattern: &Regex, text: String) -> Self {
        let captures = pattern.find_iter(&text).map(|matc| matc.range()).collect();
        Self {
            text: Arc::new(text),
            captures,
        }
    }

    pub fn get(&self, n: usize) -> Option<LuaMatch> {
        self.captures
            .get(n)
            .map(|range| LuaMatch::new(Arc::clone(&self.text), range))
    }

    pub fn len(&self) -> usize {
        self.captures.len()
    }
}

impl LuaUserData for LuaMatches {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("get", |lua, this: &LuaMatches, index: usize| {
            // We must subtract 1 from the index because Lua is 1-based
            this.get(index - 1).into_lua(lua)
        });

        methods.add_meta_method("__len", |lua, this, ()| this.captures.len().into_lua(lua));
        methods.add_meta_method("__tostring", |lua, this, ()| {
            format!("Captures({})", this.len()).into_lua(lua)
        });
    }
}

#[derive(Debug)]
/// Represents a single match over a string.
pub struct LuaMatch {
    /// The entirety of the string that the match is over. This is the same
    /// allocation as the `LuaMatches` struct that this match belongs to.
    /// It cannot be a string slice because it may outlive that struct.
    text: Arc<String>,
    /// The start of the match, inclusive.
    start: usize,
    /// The end of the match, exclusive.
    end: usize,
}

impl LuaMatch {
    pub fn new(text: Arc<String>, range: &Range<usize>) -> Self {
        Self {
            text,
            start: range.start,
            end: range.end,
        }
    }

    #[inline]
    pub const fn range(&self) -> Range<usize> {
        self.start..self.end
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.end - self.start
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

impl fmt::Display for LuaMatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Capture({})", &self.text[self.range()])
    }
}

impl LuaUserData for LuaMatch {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("start", |lua, this| {
            // Strings are 0 based in Rust but 1 based in Lua
            (this.start + 1).into_lua(lua)
        });
        fields.add_field_method_get("finish", |lua, this| this.end.into_lua(lua));
        fields.add_field_method_get("len", |lua, this| this.len().into_lua(lua));

        fields.add_field_method_get("text", |lua, this| (&this.text[this.range()]).into_lua(lua));
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("isempty", |lua, this, ()| this.is_empty().into_lua(lua));

        methods.add_meta_method("__len", |lua, this, ()| this.len().into_lua(lua));
        methods.add_meta_method("__tostring", |lua, this, ()| this.to_string().into_lua(lua));
    }
}
