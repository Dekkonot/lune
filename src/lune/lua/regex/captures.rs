use std::sync::Arc;

use mlua::prelude::*;
use regex::{Captures, Regex};
use self_cell::self_cell;

use super::matches::LuaMatch;

type OptionalCaptures<'a> = Option<Captures<'a>>;

self_cell! {
    struct LuaCapturesInner {
        owner: Arc<String>,
        #[covariant]
        dependent: OptionalCaptures,
    }
}
pub struct LuaCaptures {
    inner: LuaCapturesInner,
}

impl LuaCaptures {
    pub fn new(pattern: &Regex, text: String) -> Self {
        Self {
            inner: LuaCapturesInner::new(Arc::from(text), |owned| pattern.captures(owned.as_str())),
        }
    }

    pub fn captures(&self) -> &Captures {
        self.inner
            .borrow_dependent()
            .as_ref()
            .expect("None captures should not be used")
    }

    pub fn text(&self) -> Arc<String> {
        Arc::clone(self.inner.borrow_owner())
    }
}

impl LuaUserData for LuaCaptures {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("get", |lua, this, n: usize| {
            let captures = this.captures();
            let mtch = captures.get(n).expect("invalid match");
            LuaMatch::new(this.text(), mtch).into_lua(lua)
        });

        methods.add_method("group", |lua, this, group: String| {
            let captures = this.captures();
            match captures.name(&group) {
                Some(mtch) => LuaMatch::new(this.text(), mtch).into_lua(lua),
                None => Ok(LuaNil),
            }
        });

        methods.add_method("format", |lua, this, format: String| {
            let mut new = String::new();
            this.captures().expand(&format, &mut new);
            new.into_lua(lua)
        });

        methods.add_meta_method(LuaMetaMethod::Len, |lua, this, ()| {
            this.captures().len().into_lua(lua)
        });
        methods.add_meta_method(LuaMetaMethod::ToString, |lua, this, ()| {
            format!("Captures({} captures)", this.captures().len()).into_lua(lua)
        });
    }

    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_meta_field("__type", "RegexCaptures")
    }
}
