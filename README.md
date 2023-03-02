
<div align="center">

# sh.rs

rust library to build your own shell

[![book](https://img.shields.io/badge/book-website-orange)](#)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](#)

</div>
<!-- [![build](https://github.com/MrPicklePinosaur/shrs/workflows/Deploy/badge.svg)](https://github.com/MrPicklePinosaur/shrs/actions) -->

## PROJECT GOALS

- **hackable**: almost every aspect of the shell should be easily customizable and extendable
- **productive**: shell should encourage and facilitate an efficient workflow
- **rusty**: implemented in rust down to the syscall level (rustix) and configurable in rust

## FEATURES

Project is currently very much VIP, below lists the current feature statuses:

| Feature | Status |
| --- | --- |
| posix shell | wip |
| history | mvp |
| aliases | mvp |
| completion | mvp |
| readline | using reedline |
| hooks | TODO |
| keybindings | TODO |
| syntax highlighting | TODO |
| shell scripts | TODO |

## RESOURCES

- [build your own shell](https://github.com/tokenrove/build-your-own-shell)
- [grammar for posix shell](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html#tag_18_10)
- [oursh: rust shell using lalrpop](https://github.com/nixpulvis/oursh)
- [gnu: implementing a job control shell](https://www.gnu.org/software/libc/manual/html_node/Implementing-a-Shell.html)
- [A Brief Introduction to termios](https://blog.nelhage.com/2009/12/a-brief-introduction-to-termios/)
