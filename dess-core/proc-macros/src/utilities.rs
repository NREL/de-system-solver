use crate::imports::*;

// taken from https://github.com/lumol-org/soa-derive/blob/master/soa-derive-internal/src/input.rs
pub(crate) trait TokenStreamIterator {
    fn concat_by(
        self,
        f: impl Fn(proc_macro2::TokenStream, proc_macro2::TokenStream) -> proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream;
    fn concat(self) -> proc_macro2::TokenStream;
}

impl<T: Iterator<Item = proc_macro2::TokenStream>> TokenStreamIterator for T {
    fn concat_by(
        mut self,
        f: impl Fn(proc_macro2::TokenStream, proc_macro2::TokenStream) -> proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream {
        match self.next() {
            Some(first) => self.fold(first, f),
            None => quote! {},
        }
    }

    fn concat(self) -> proc_macro2::TokenStream {
        self.concat_by(|a, b| quote! { #a #b })
    }
}

/// Checks if a field is a vec
pub fn is_vec(field: &Field) -> bool {
    if let Type::Path(type_path) = &field.ty {
        if let Some(segment) = type_path.path.segments.first() {
            if segment.ident == "Vec" {
                return true;
            }
        }
    }
    false
}
