#[doc(inline)]
pub mod de;
#[doc(inline)]
pub mod error;
#[doc(inline)]
pub mod ser;

pub use de::{from_str, from_str_with_key, Deserializer};
pub use error::{Error, Result};
pub use ser::{to_string, to_string_with_key, to_writer, to_writer_with_key, Serializer};
