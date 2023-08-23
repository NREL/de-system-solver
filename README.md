# Overview
The Rust-based Differential Equation System Solver (DESS, de-system-solver) is a proof-of-concept Rust crate that will attempt to demonstrate that the macro system allows for component-by-component building of a system which can be solved by a variety of solver options.  The crate implements both a framework for specifying ODEs and numerous solvers for application to that framework. Once it's working well, it'll likely become a dependency for numerous other projects.  

# Installation and Running
1. [Install rust](https://www.ecosia.org/search?q=rustup%20instal&addon=firefox&addonversion=4.1.0&method=topbar)
1. Create and activate a Python environment, e.g. with
    ```
    python3.10 -m venv dess-venv
    source dess-venv/bin/activate
    ```
1. run `sh build_and_test.sh` or manually run the commands in that bash script file 
