(cd dess-examples/ && cargo test && \
cd ../dess-examples-pyo3/ && maturin develop --release) && \
# pytest -v tests && \ # make a tests/ folder
sh run_py_scripts.sh
