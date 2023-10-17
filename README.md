# Overview
The Rust-based Differential Equation System Solver (DESS) is a Rust crate implementing fixed-step and adaptive-step solvers and designed for modeling physical systems. Six explicit ODE solver methods have been added so far: Euler’s, Heun’s, Midpoint, Ralston’s, Classic Runge-Kutta, and Cash-Karp.  These include five fixed-step methods and one adaptive-step method. A seventh adaptive-step solver, the Bogacki-Shampine method, will be added soon. The goal of DESS is to create a Rust differential equation solver crate designed specifically to easily model physical systems. In addition to directly inputting equations to solve, users can also define relationships between nodes in their system, which the package then translates into a system of equations, leading to simpler and more intuitive code. See the dess-examples-pyo3 folder for examples how to use DESS in this manner.

# Installation and Running
1. [Install rust](https://www.ecosia.org/search?q=rustup%20instal&addon=firefox&addonversion=4.1.0&method=topbar)
1. Create and activate a Python environment, e.g. with
    ```
    python3.10 -m venv dess-venv
    source dess-venv/bin/activate
    ```
1. run `sh build_and_test.sh` or manually run the commands in that bash script file 
