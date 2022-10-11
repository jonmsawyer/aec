use ruint::Uint;

pub mod modules;
pub use modules::*;

/// BI stands for Big Int
type BI = Uint<1024, 16>;
