#[cfg(feature = "bug")]
/// This is a module
pub mod demo;

#[cfg(not(feature = "bug"))]
pub mod demo;
