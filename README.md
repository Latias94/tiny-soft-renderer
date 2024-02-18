# Tiny Soft Renderer

A tiny software renderer implemented in Rust.

## Build

```shell
cargo r --example playground
```

The first build may take some time as it statically compiles SDL2. The decision not to use `softbuffer` or `pixels`
crate stems from their requirement for the application to handle DPI scaling.

## References

- [ssloy/tinyrenderer](https://github.com/ssloy/tinyrenderer)
