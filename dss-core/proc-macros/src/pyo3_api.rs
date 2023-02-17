use crate::imports::*;

/// Derives several methods for struct
pub(crate) fn pyo3_api(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast_item = item.clone();
    let mut ast = syn::parse_macro_input!(ast_item as syn::ItemStruct);
    let ident = &ast.ident;
    let has_walk = ast.attrs.iter().any(|attr| attr.path.is_ident("walk"));

    let mut item_block: TokenStream2 = quote! {
        #[derive(Pyo3ApiCleanup)]
        #[cfg_attr(feature = "pyo3", pyclass)]
    };
    item_block.extend::<TokenStream2>(item.into());

    let walk_block = if has_walk {
        quote! {
            #[pyo3(name = "walk")]
            fn walk_py(&mut self) {
                self.walk();
            }
        }
    } else {
        TokenStream2::new()
    };

    let mut pyo3_fns = Vec::new();

    if let syn::Fields::Named(syn::FieldsNamed { named, .. }) = &mut ast.fields {
        // struct with named fields
        for field in named.iter_mut() {
            let fname = field.ident.as_ref().unwrap();

            // Conditionally add setter function
            if field.attrs.iter().any(|a| a.path.is_ident("skip_get")) {
                pyo3_fns.push(quote! {
                    fn get_ #fname(&mut self) -> #field.ty {
                        self.#fname
                    }
                });
            }
        }
    } else {
        abort!(ident.span(), "Only works on structs with named fields.");
    }

    let py_impl_block = quote! {
        #[cfg(feature = "pyo3")]
        #[pymethods]
        impl #ident {
            #(#pyo3_fns)*
            #walk_block
            #[classmethod]
            #[pyo3(name = "default")]
            /// Exposes `default` to python.
            fn default_py(_cls: &PyType) -> PyResult<Self> {
                Ok(Self::default())
            }

            /// json serialization method.
            #[pyo3(name = "to_json")]
            fn to_json_py(&self) -> PyResult<String> {
                Ok(self.to_json())
            }

            #[classmethod]
            /// json deserialization method.
            #[pyo3(name = "from_json")]
            fn from_json_py(_cls: &PyType, json_str: &str) -> PyResult<Self> {
                Ok(Self::from_json(json_str)?)
            }

            /// yaml serialization method.
            #[pyo3(name = "to_yaml")]
            fn to_yaml_py(&self) -> PyResult<String> {
                Ok(self.to_yaml())
            }

            #[classmethod]
            /// yaml deserialization method.
            #[pyo3(name = "from_yaml")]
            fn from_yaml_py(_cls: &PyType, yaml_str: &str) -> PyResult<Self> {
                Ok(Self::from_yaml(yaml_str)?)
            }

            /// bincode serialization method.
            #[pyo3(name = "to_bincode")]
            fn to_bincode_py<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
                Ok(PyBytes::new(py, &self.to_bincode()))
            }

            #[classmethod]
            /// bincode deserialization method.
            #[pyo3(name = "from_bincode")]
            fn from_bincode_py(_cls: &PyType, encoded: &PyBytes) -> PyResult<Self> {
                Ok(Self::from_bincode(encoded.as_bytes())?)
            }

            /// `__copy__` magic method that uses `clone`.
            fn __copy__(&self) -> Self {
                self.clone()
            }

            /// `__deepcopy__` magic method that uses `clone`.
            fn __deepcopy__(&self) -> Self {
                self.clone()
            }

            #[pyo3(name = "clone")]
            /// `__deepcopy__` magic method that uses `clone`.
            fn clone_py(&self) -> Self {
                self.clone()
            }
        }
    };

    let mut item_and_impl_block = py_impl_block;
    item_and_impl_block.extend(item_block);
    item_and_impl_block.into()
}
