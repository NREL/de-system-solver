use crate::imports::*;
use crate::utilities::parse_ts_as_fn_defs;

/// Derives several methods for struct
pub(crate) fn pyo3_api(attr: TokenStream, item: TokenStream) -> TokenStream {
    let forbidden_fn_names = vec!["default".into(), "default_py".into()];
    let attr_ts2: TokenStream2 =
        parse_ts_as_fn_defs(attr, vec![], false, forbidden_fn_names);

    let item_struct = syn::parse_macro_input!(item as syn::ItemStruct);
    let ident = &item_struct.ident;
    let has_walk = item_struct
        .attrs
        .iter()
        .any(|attr| attr.path.is_ident("walk"));

    let mut item_block: TokenStream2 = quote! {
        #[derive(Pyo3ApiCleanup)]
        #[cfg_attr(feature = "pyo3", pyclass)]
    };
    item_block.extend::<TokenStream2>(item_struct.to_token_stream());

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

    let mut pyo3_fns: Vec<TokenStream2> = Vec::new();

    let mut fields = item_struct.fields;

    for field in fields.iter_mut() {
        let fname = field.ident.as_ref().unwrap();

        // Conditionally add setter function
        if !field.attrs.iter().any(|a| a.path.is_ident("skip_get")) {
            let fn_get_fname: TokenStream2 =
                format!("fn get_{}(&mut self)", &fname).parse().unwrap();
            let field_type = &field.ty;
            let fn_body: TokenStream2 = format!("self.{}.clone()", &fname).parse().unwrap();
            let new_fn = quote! {
                #[getter]
                #fn_get_fname -> #field_type {
                    #fn_body
                }
            };
            pyo3_fns.push(new_fn);
        }
    }

    let py_impl_block = quote! {
        #[cfg(feature = "pyo3")]
        #[pymethods]
        impl #ident {
            #attr_ts2
            #(#pyo3_fns)*
            #walk_block
            #[classmethod]
            #[pyo3(name = "default")]
            /// Exposes `default` to python.
            fn default_py(_cls: &PyType) -> PyResult<Self> {
                Ok(Self::default())
            }

            /// Save current data structure to file. Method adaptively calls serialization methods
            /// dependent on the suffix of the file given as str.
            ///
            /// # Argument:
            ///
            /// * `filename`: a `str` storing the targeted file name. Currently `.json` and `.yaml` suffixes are
            /// supported
            ///
            /// # Returns:
            ///
            /// A Rust Result
            #[pyo3(name = "to_file")]
            fn to_file_py(&self, filename: &str) -> PyResult<()> {
                Ok(self.to_file(filename)?)
            }

            /// Read from file and return instantiated struct. Method adaptively calls deserialization
            /// methods dependent on the suffix of the file name given as str.
            /// Function returns a dynamic Error Result if it fails.
            ///
            /// # Argument:
            ///
            /// * `filename`: a `str` storing the targeted file name. Currently `.json` and `.yaml` suffixes are
            /// supported
            ///
            /// # Returns:
            ///
            /// A Rust Result wrapping data structure if method is called successfully; otherwise a dynamic
            /// Error.
            #[classmethod]
            #[pyo3(name = "from_file")]
            fn from_file_py(_cls: &PyType, filename: &str) -> PyResult<Self> {
                Ok(Self::from_file(filename)?)
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
