# Gamma Setter Core

![License](https://img.shields.io/badge/License-MIT-dark_green)

This is the core module of the **[Gamma Setter](https://github.com/Zhoucheng133/Gamma-Setter)** project written in Rust. It contains the core logic and functionality of the application.  
Inspired by [HotKey-Gamma](https://github.com/wasupandceacar/HotKey-Gamma).  
This module supports **Windows** only.

## Generate Dynamic Library

Use `cargo build --release` to generate the dynamic library.

## API

`set_gamma`: Set the gamma value of the screen. The parameter is a float value between -1 and 1. The higher the value, the more contrast. Set 0 to reset.