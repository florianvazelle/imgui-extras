<div align="center">

# `imextras`

**ImGui extras collection**

![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/florianvazelle/imextras/nix.yml)
![GitHub License](https://img.shields.io/github/license/florianvazelle/imextras)

</div>

## [`imgui-sdl3`](./crates/imgui-sdl3/)

[![Crates.io](https://img.shields.io/crates/v/imgui-sdl3.svg)](https://crates.io/crates/imgui-sdl3)
[![API Docs](https://docs.rs/imgui-sdl3/badge.svg)](https://docs.rs/imgui-sdl3)

This crate provides an SDL3 backend platform and renderer for imgui-rs.

- The backend platform handles window/input device events (based on [ghtalpo/imgui-sdl3-support](https://github.com/ghtalpo/imgui-sdl3-support)),
- The rendering backend use the SDL3 GPU API, and can be use as a render pass.

> For a canvas rendering backend, check out [masonjmj/imgui-rs-sdl3-renderer](https://github.com/masonjmj/imgui-rs-sdl3-renderer).


## [`imoguizmo`](./crates/imoguizmo/) 

[![Crates.io](https://img.shields.io/crates/v/imoguizmo.svg)](https://crates.io/crates/imoguizmo)
[![API Docs](https://docs.rs/imoguizmo/badge.svg)](https://docs.rs/imoguizmo)

This crate integrate an interactive orientation gizmo, based on [ImOGuizmo](https://github.com/fknfilewalker/imoguizmo).

## [`imstyle`](./crates/imstyle/) 

[![Crates.io](https://img.shields.io/crates/v/imstyle.svg)](https://crates.io/crates/imstyle)
[![API Docs](https://docs.rs/imstyle/badge.svg)](https://docs.rs/imstyle)

This crate allows to manage ImGui style with TOML, based on [ImStyle](https://github.com/Patitotective/ImStyle).

## Development

The project use [`just`](https://just.systems/man/en/) as command runner.

To check all available recipes, run:
```
just
```

To run formatters:
```
just fmt
```
