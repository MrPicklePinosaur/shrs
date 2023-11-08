+++
title = "Introduction"
description = ""
date = 2021-05-01T18:10:00+00:00
updated = 2021-05-01T18:10:00+00:00
draft = false
weight = 1
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = ""
toc = true
top = false
+++

Firstly, thanks for taking the time to contribute to **shrs**! This page will
go over starting steps to get you ready to create your first PR!.

## Prerequisites

Since **shrs** is a rust project, you should have a working rust development
environment. You can get started with installing rust by using
[rustup](https://rustup.rs/) - the rust toolchain installer. You should also
install the nightly rust toolchain as some parts of the developer workflow
depends on nightly versions of rust tools.

**shrs** uses [just](https://github.com/casey/just) as it's command runner.
Install it in whatever method suitable for your system.

## Build the example

First we will get an example build of **shrs** up and running to take a tour of
all the features it offers.

Clone the repository and enter the project directory:
```sh
git clone https://github.com/shellrs/shrs.git
cd shrs
```

There are a couple of special git hooks that are run on commit that or
especially for developers. These git hooks do things like lint your code and
format it. You can install them with:
```sh
just devsetup
```

Now to run the example shell
```sh
just
```

You should now be throw into the example **shrs** shell, which shows off a good
number of the features **shrs** offers. Take some time to explore what the
shell is capable of. You can also examine `shrs_example/src/main.rs` to see the
actual configuration.

## Build your own shell

Next is to use **shrs** as it was intended, as a library. To get started with
creating your own shell, take a look at the [Quick Start](../getting-started/quick-start) section.

## Tackle an issue

Once you are decently familiar with the **shrs** API, you can try to tackle an
actual issue and open your first PR! See the open issues, especially those
marked [good first issue](https://github.com/shellrs/shrs/labels/good%20first%20issue).
If you have any questions, don't be afraid to reach out for help!


