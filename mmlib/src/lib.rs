//! # Welcome to the mmlib documentation.
//! There is a formats module, go to the documentation for that module, and look for desired file types.

#[macro_export]
/// This macro is used to quickly re-export modules through a list.
macro_rules! quickmodule {
  () => {};
  ($modname:ident) => {
    pub mod $modname;
  };
  ($modname:ident, $($tail:ident),*) => {
    pub mod $modname;
    modlist!($($tail),*);
}}

quickmodule!(formats);