phantom-enum 0.9.1
==================

[![Build Status](https://travis-ci.org/chris-morgan/phantom-enum.svg?branch=master)](https://travis-ci.org/chris-morgan/phantom-enum)

**A simple macro library for creating phantom enums.** Just simple sugar.

<!-- The rest of this section comes straight from the macro source docs with the first sentence removed. -->

A phantom type enum is an enum-like arrangement where the enum is a module
and trait and the variants are (uninstantiable) types.

This is very good for the static representation of state machines in which
*nothing* can go wrong.

    #[macro_use] #[no_link]
    extern crate phantom_enum;

    phantom_enum! {
        #[doc = "Put things here, of course"]
        pub enum TableItem {
            #[doc = "A bottle with a paper label reading “DRINK ME”."]
            Potion,
            #[doc = "A cake with the words “EAT ME” marked in currants."]
            Cake
        }
    }

    struct Person<T> {
        name: &'static str,
    }

    // Note how this restricts the methods to only meaningful types.
    // (I look forward to generic bounds in struct definitions!)
    impl<T: TableItem::Impl> Person<T> {
        fn new(name: &'static str) -> Person<T> {
            Person {
                name: name,
            }
        }
    }

    impl Person<TableItem::Potion> {
        fn drink_it(self) -> Person<TableItem::Cake> {
            println!("Shrinking! Oh look, there’s a box down here!");
            Person {
                name: self.name,
            }
        }
    }

    impl Person<TableItem::Cake> {
        fn eat_it(self) -> () {
            println!("Growing! OK, that’s enough of the story.");
            // Who can remember what comes next, anyway?
        }
    }

    fn main() {
        let person = Person::new("Alice");
        let person = person.drink_it();
        person.eat_it();
    }

As you will observe with this example, if you have a `Person<Potion>`, you
simply cannot call `.eat_it()`; for that, you must have a `Person<Cake>`.
Similarly, once you have drunk that potion, you can’t drink it again.

Usage
-----

Cargo all the way.

Author
------

[Chris Morgan](http://chrismorgan.info/) ([chris-morgan](https://github.com/chris-morgan)) is the primary author and maintainer of phantom-enum.

License
-------

This library is distributed under similar terms to Rust: dual licensed under the MIT license and the Apache license (version 2.0).

See LICENSE-APACHE, LICENSE-MIT, and COPYRIGHT for details.
