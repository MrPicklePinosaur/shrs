
<div align="center">

# sh.rs

rust library to build your own shell

</div>

## PROJECT GOALS

- **hackable**: almost every aspect of the shell should be easily customizable and extendable
- **productive**: shell should encourage and facilitate an efficient workflow
- **rusty**: implemented in rust down to the syscall level (rustix) and configurable in rust

## TOOD

- [x] pipes
- [x] file redirection
- [ ] configuration scheme (config file? builder pattern?)
- [ ] better logging + error reporting (different ways of displaying exit status)
- [x] background process + job control (&)
- [ ] environment variables
- [*] subshells
- [ ] signals (^C, ^\, ^Z etc)
- [ ] completion
- [ ] history
- [ ] alias
- [ ] test suite to ensure posix compliant
- [ ] control flow
  - [x] for
  - [ ] case
  - [ ] if 
  - [ ] while
  - [ ] until

## RESOURCES

- [build your own shell](https://github.com/tokenrove/build-your-own-shell)
- [grammar for posix shell](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html#tag_18_10)
- [oursh: rust shell using lalrpop](https://github.com/nixpulvis/oursh)
- [gnu: implementing a job control shell](https://www.gnu.org/software/libc/manual/html_node/Implementing-a-Shell.html)
- [A Brief Introduction to termios](https://blog.nelhage.com/2009/12/a-brief-introduction-to-termios/)
