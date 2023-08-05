mod constant;
mod custom;
pub(crate) mod functional;
mod simplex;
mod value;
pub use constant::Constant;
pub use custom::{Custom1D, Custom2D, Custom3D, Custom4D};
pub use simplex::Simplex;
pub use value::Value;
