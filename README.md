# ABOUT
This is the template for Rust-backed Python interface.

## HOW to START
```sh
cd rust_python_bound_setup
docker compose up -d
```
And in the docker container's command line,
```sh
maturin build --release
pip install .
```

## Example
In the docker container's command line,
```sh
python examples/sincurve.py
```