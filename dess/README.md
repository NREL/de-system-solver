# dess
The Differential Equation System Solver (DESS) is a Rust crate implementing fixed-step and adaptive-step solvers and designed especially for modeling physical systems. Seven explicit ordinary differential equation (ODE) solver methods have been added so far: Euler’s, Heun’s, Midpoint, Ralston’s, Classic Runge-Kutta, Bogacki-Shampine, and Cash-Karp.  These comprise five fixed-step methods and two adaptive-step methods. Few solver packages are implemented in the Rust ecosystem and none are intended specifically for physical system modeling, so the goal of DESS is to create a Rust ODE solver crate designed specifically to easily specify and model physical systems with modular, configurable solver options. In addition to allowing users to directly input equations to solve, DESS allows users to optionally specify and define relationships between nodes in their system, which the package then translates into a system of equations via the Rust macro system, leading to simpler and more intuitive code. See the dess-examples-pyo3 folder for examples how to use DESS in this manner.

<!-- In `See the dess-examples-pyo3 folder for examples how to use DESS in this manner.` let's be sure to turn `dess-examples-pyo3` into a link or something.  
We could also add a function that copies the example files like in FASTSim or something.  I'm not totally sure this makes sense to do. 
 -->

# Installation and Running
1. [Install rust](https://www.ecosia.org/search?q=rustup%20instal&addon=firefox&addonversion=4.1.0&method=topbar)
1. Create and activate a Python environment, e.g. with
    ```
    python3.10 -m venv dess-venv
    source dess-venv/bin/activate
    ```
1. run `sh build_and_test.sh` or manually run the commands in that bash script file 