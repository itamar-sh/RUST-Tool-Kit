# Using rust with python
We have here some rust code which can be compiled and imported as python code.

` maturin develop ` will compile the rust package.

` import python_in_rust `
` python_in_rust.sum_as_string(5, 20)`

Is legit python code.

# Compile with makefile

A makefile like that will be used with ` make build ` or init or clean:
```
init:
    maturin init

build:
    cargo build --release
    cp targets/release/libdigits_pi.so .

clean:
    rm -f libdigits.so
```


# dataypes_conversion
This is where we use rust objects in python objects. like primitive rust into python dict.
We use `pyo3::types::PyDict;` to import python dict.

# rust_ownership_model
We create a python class which is equal to struct with implmeneted methods in rust.
To expose it we create the rust-struct twice one for the rust code and one for the python-interface.
Then we can use it the python class like this:

```
import libownership_pyrust

list_instance = libownership_pyrust.NumberList()
list_instance.add(5)
list_instance.length()
```


# Python Fire - another way to import rust code in python and use it as CLI tool
This is how the logic of using rust code in python looks like with Python Fire:

<img
  src="../python_in_rust/resources/Screenshot from 2025-01-17 03-52-07.png"
  title="PyO3 project diagram"
  style="display: inline-block; margin: 0 auto;" width="700" height="390"><br/>


# Rust handle python exceptions
On exception_handleing we can call use inside rust code exception handleing of python such that when import in python and use the rust code the reuslt will be python exception and not rust error.

# Python Polars rust
Profiling same opertation with different falvors of rust, python calling rust, python.
criterion library is something we use to profile the time the code runs.

