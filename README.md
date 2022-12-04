<!--
SPDX-FileCopyrightText: 2022 Redox OS Developers <info@redox-os.org>
SPDX-License-Identifier: MIT
-->

# slint_orbclient

[Slint](https://slint-ui.com/) platform support based on [OrbClient](https://gitlab.redox-os.org/redox-os/orbclient). Can be used to run a `Slint` application on [Redox](https://redox-os.org/). *Originally located on https://codeberg.org/flovansl/co_sl.*

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](../../LICENSES/MIT.txt)
<!-- [![docs](https://img.shields.io/badge/docs-latest-orange.svg)](https://flovansl.codeberg.page/coop_sl/snapshots/docs/slint_orbclient/) -->

## Example

```rust
slint::slint!{
    HelloWorld := Window {
        Text {
            text: "hello world";
            color: green;
        }
    }
}

fn main() {
    slint_orbclient::init_config(slint_orbclient::Config::default().width(600).height(400).title("Hello"));
    HelloWorld::new().run();
}
```

## How to run your app on Redox

* An app template can be found on https://gitlab.redox-os.org/redox-os/slint_redox_template
* A guide to include a program to the Redox build can be found on https://doc.redox-os.org/book/ch05-03-including-programs.html.

## new to slint?

Best to start with these sources:

* getting start: https://slint-ui.com/#tryout
* Slint docs (*Slint lang docs included*): https://docs.rs/slint/latest/slint/
* Slint examples: https://github.com/slint-ui/slint/tree/master/examples
* Slint chat: https://chat.slint-ui.com/

## license

* `slint_orbclient` is available under [MIT license](../../LICENSES/MIT.txt).
 * `Slint` is available under either a [commercial license](https://github.com/slint-ui/slint/blob/master/LICENSES/LicenseRef-Slint-commercial.md)
or [GNU GPLv3](https://github.com/slint-ui/slint/blob/master/LICENSES/GPL-3.0-only.txt).
