This is a proof-of-concept Rust crate that will attempt to demonstrate that the macro system allows for component-by-component building of a system which can be solved by a variety of solver options.  

# Cool things already implemented
- `HistoryMethods` derive macro for generating widely usable method
- Solver and System are modular and independent

# Ideas for how to do this:
- Solvers that we may want to use:
   - fixed-step euler
   - adaptive euler
   - adaptive Runga-Kutta (e.g. RK45)
   - ???
- Solver framework:
   - could reference https://github.nrel.gov/cbaker2/rust_kepler_example and use `ode-solvers` crate.  This may require impractical code gymnastics to support the required state-space formulation, or macros (possibly procedural?) may make this straightforward.  
   - could build out own solvers that are oriented for black-box formulation
