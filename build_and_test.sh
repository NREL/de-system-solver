(cd dess-examples/ && cargo test && \
cd ../dess-pyo3/ && maturin develop --release) \
&& python -m unittest \
&& sh run_py_scripts.sh
