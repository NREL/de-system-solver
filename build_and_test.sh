(cd dess-examples/ && cargo test && \
cd ../dess-pyo3/ && maturin develop --release) \
&& python -m unittest

(
    cd python/Sytem3TM/ && 
    for file in *.py; do 
        python "$file"
    done
)