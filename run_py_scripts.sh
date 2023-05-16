(
    cd python/Sytem3TM/ && 
    for file in *.py; do 
        python "$file"
    done
) && \
(
    cd python/System3TMWithBC/ && 
    for file in *.py; do 
        python "$file"
    done
)