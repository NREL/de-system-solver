(cd dess-examples/ && cargo test && \
cd ../dess-pyo3/ && maturin develop --release) \
&& python -m unittest