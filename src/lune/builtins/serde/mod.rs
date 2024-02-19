use mlua::prelude::*;

pub(super) mod compress_decompress;
pub(super) mod encode_decode;
pub(super) mod hash;

use compress_decompress::{compress, decompress, CompressDecompressFormat};
use encode_decode::{EncodeDecodeConfig, EncodeDecodeFormat};

use crate::lune::util::TableBuilder;

use self::hash::HashAlgorithm;

pub fn create(lua: &'static Lua) -> LuaResult<LuaTable> {
    TableBuilder::new(lua)?
        .with_function("encode", serde_encode)?
        .with_function("decode", serde_decode)?
        .with_async_function("compress", serde_compress)?
        .with_async_function("decompress", serde_decompress)?
        .with_function("hash", hash_message)?
        .with_function("hmac", hmac_message)?
        .build_readonly()
}

fn serde_encode<'lua>(
    lua: &'lua Lua,
    (format, val, pretty): (EncodeDecodeFormat, LuaValue<'lua>, Option<bool>),
) -> LuaResult<LuaString<'lua>> {
    let config = EncodeDecodeConfig::from((format, pretty.unwrap_or_default()));
    config.serialize_to_string(lua, val)
}

fn serde_decode<'lua>(
    lua: &'lua Lua,
    (format, str): (EncodeDecodeFormat, LuaString<'lua>),
) -> LuaResult<LuaValue<'lua>> {
    let config = EncodeDecodeConfig::from(format);
    config.deserialize_from_string(lua, str)
}

async fn serde_compress<'lua>(
    lua: &'lua Lua,
    (format, str): (CompressDecompressFormat, LuaString<'lua>),
) -> LuaResult<LuaString<'lua>> {
    let bytes = compress(format, str).await?;
    lua.create_string(bytes)
}

async fn serde_decompress<'lua>(
    lua: &'lua Lua,
    (format, str): (CompressDecompressFormat, LuaString<'lua>),
) -> LuaResult<LuaString<'lua>> {
    let bytes = decompress(format, str).await?;
    lua.create_string(bytes)
}

fn hash_message<'lua>(
    lua: &'lua Lua,
    (algorithm, message): (HashAlgorithm, LuaValue<'lua>),
) -> LuaResult<LuaString<'lua>> {
    let bytes = algorithm.hash(message)?;
    lua.create_string(bytes)
}

fn hmac_message<'lua>(
    lua: &'lua Lua,
    (algorithm, message, key): (HashAlgorithm, LuaValue<'lua>, LuaValue<'lua>),
) -> LuaResult<LuaString<'lua>> {
    let bytes = algorithm.hmac(message, key)?;
    lua.create_string(bytes)
}
