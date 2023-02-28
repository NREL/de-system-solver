(cd dess-examples/ && cargo test --workspace  && \
cd ../dess-pyo3/ && maturin develop --release) \
&& python -m unittest