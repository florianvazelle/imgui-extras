<div align="center">

# `imextras`

**Rust crates to extends ImGui functionality**

![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/florianvazelle/imextras/nix.yml)
![GitHub License](https://img.shields.io/github/license/florianvazelle/imextras)

</div>

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
