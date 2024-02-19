use mlua::prelude::*;

use blake3::Hasher as Blake3;
use sha2::{Sha224, Sha256, Sha384, Sha512};
use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512};

#[derive(Debug, Clone, Copy)]
pub enum HashAlgorithm {
    // SHA-2 variants
    Sha2_224,
    Sha2_256,
    Sha2_384,
    Sha2_512,
    // SHA-3 variants
    Sha3_224,
    Sha3_256,
    Sha3_384,
    Sha3_512,
    // Blake3
    Blake3,
}

impl HashAlgorithm {
    pub fn list_all_as_string() -> String {
        [
            "sha224", "sha256", "sha384", "sha512", "sha3-224", "sha3-256", "sha3-384", "sha3-512",
            "blake3",
        ]
        .join(", ")
    }

    #[inline]
    pub fn hash(&self, message: LuaValue) -> LuaResult<Vec<u8>> {
        use digest::Digest;

        // TODO support buffers here
        let message = match message {
            LuaValue::String(str) => str,
            _ => unimplemented!(),
        };
        match self {
            Self::Sha2_224 => Ok(Sha224::digest(message).to_vec()),
            Self::Sha2_256 => Ok(Sha256::digest(message).to_vec()),
            Self::Sha2_384 => Ok(Sha384::digest(message).to_vec()),
            Self::Sha2_512 => Ok(Sha512::digest(message).to_vec()),

            Self::Sha3_224 => Ok(Sha3_224::digest(message).to_vec()),
            Self::Sha3_256 => Ok(Sha3_256::digest(message).to_vec()),
            Self::Sha3_384 => Ok(Sha3_384::digest(message).to_vec()),
            Self::Sha3_512 => Ok(Sha3_512::digest(message).to_vec()),

            Self::Blake3 => Ok(Blake3::digest(message).to_vec()),
        }
    }

    #[inline]
    pub fn hmac(&self, message: LuaValue, key: LuaValue) -> LuaResult<Vec<u8>> {
        use hmac::{Hmac, Mac, SimpleHmac};
        macro_rules! hmac {
            ($Type:ty, $message:expr, $key:expr) => {{
                let mut mac: Hmac<$Type> = Hmac::new_from_slice($key.as_bytes()).into_lua_err()?;
                mac.update($message.as_bytes());
                Ok(mac.finalize().into_bytes().to_vec())
            }};
        }
        macro_rules! hmac_no_blocks {
            ($Type:ty, $message:expr, $key:expr) => {{
                let mut mac: SimpleHmac<$Type> =
                    SimpleHmac::new_from_slice($key.as_bytes()).into_lua_err()?;
                mac.update($message.as_bytes());
                Ok(mac.finalize().into_bytes().to_vec())
            }};
        }

        // TODO support buffers here
        let message = match message {
            LuaValue::String(str) => str,
            _ => unimplemented!(),
        };
        let key = match key {
            LuaValue::String(str) => str,
            _ => unimplemented!(),
        };

        match self {
            Self::Sha2_224 => hmac!(Sha224, message, key),
            Self::Sha2_256 => hmac!(Sha256, message, key),
            Self::Sha2_384 => hmac!(Sha384, message, key),
            Self::Sha2_512 => hmac!(Sha512, message, key),

            Self::Sha3_224 => hmac!(Sha3_224, message, key),
            Self::Sha3_256 => hmac!(Sha3_256, message, key),
            Self::Sha3_384 => hmac!(Sha3_384, message, key),
            Self::Sha3_512 => hmac!(Sha3_512, message, key),

            Self::Blake3 => hmac_no_blocks!(Blake3, message, key),
        }
    }
}

impl<'lua> FromLua<'lua> for HashAlgorithm {
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        if let LuaValue::String(str) = value {
            /*
                Casing tends to vary for alogorithms, so rather than force
                people to remember it we'll just accept any casing.
            */
            let str = str.to_str()?.to_ascii_lowercase();
            match str.as_str() {
                "sha224" => Ok(Self::Sha2_224),
                "sha256" => Ok(Self::Sha2_256),
                "sha384" => Ok(Self::Sha2_384),
                "sha512" => Ok(Self::Sha2_512),

                "sha3-224" => Ok(Self::Sha3_224),
                "sha3-256" => Ok(Self::Sha3_256),
                "sha3-384" => Ok(Self::Sha3_384),
                "sha3-512" => Ok(Self::Sha3_512),

                "blake3" => Ok(Self::Blake3),

                _ => Err(LuaError::FromLuaConversionError {
                    from: "string",
                    to: "HashAlgorithm",
                    message: Some(format!(
                        "Invalid hashing algorithm '{str}', valie kinds are:\n{}",
                        HashAlgorithm::list_all_as_string()
                    )),
                }),
            }
        } else {
            Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "HashAlgorithm",
                message: None,
            })
        }
    }
}
