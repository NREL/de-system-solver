#![allow(unused_imports)]
pub(crate) use proc_macro::TokenStream;
pub(crate) use proc_macro2::TokenStream as TokenStream2;
pub(crate) use proc_macro_error::{abort, abort_call_site, proc_macro_error, emit_error};
pub(crate) use quote::{quote, ToTokens, TokenStreamExt}; // ToTokens is implicitly used as a trait
pub(crate) use regex::Regex;
pub(crate) use syn::{spanned::Spanned, Field, Ident, ItemFn, ItemStruct, Meta, Type};
