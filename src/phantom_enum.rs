//! A simple macro library for creating phantom enums. Just simple sugar.
//!
//! See the `phantom_enum!` macro for all the details.
#![feature(macro_rules)]

/// Create a phantom type enum.
///
/// A phantom type enum is an enum-like arrangement where the enum is a module
/// and trait and the variants are (uninstantiable) types.
///
/// This is very good for the static representation of state machines in which
/// *nothing* can go wrong.
///
///     #![feature(phase)]
///
///     #[phase(plugin)]
///     extern crate phantom_enum;
///
///     phantom_enum! {
///         #[doc = "Put things here, of course"]
///         pub enum TableItem {
///             #[doc = "A bottle with a paper label reading “DRINK ME”."]
///             Potion,
///             #[doc = "A cake with the words “EAT ME” marked in currants."]
///             Cake
///         }
///     }
///
///     struct Person<T> {
///         name: &'static str,
///     }
///
///     // Note how this restricts the methods to only meaningful types.
///     // (I look forward to generic bounds in struct definitions!)
///     impl<T: TableItem::Impl> Person<T> {
///         fn new(name: &'static str) -> Person<T> {
///             Person {
///                 name: name,
///             }
///         }
///     }
///
///     impl Person<TableItem::Potion> {
///         fn drink_it(self) -> Person<TableItem::Cake> {
///             println!("Shrinking! Oh look, there’s a box down here!");
///             Person {
///                 name: self.name,
///             }
///         }
///     }
///
///     impl Person<TableItem::Cake> {
///         fn eat_it(self) -> () {
///             println!("Growing! OK, that’s enough of the story.");
///             // Who can remember what comes next, anyway?
///         }
///     }
///
///     fn main() {
///         let person = Person::new("Alice");
///         let person = person.drink_it();
///         person.eat_it();
///     }
///
/// As you will observe with this example, if you have a `Person<Potion>`, you
/// simply cannot call `.eat_it()`; for that, you must have a `Person<Cake>`.
/// Similarly, once you have drunk that potion, you can’t drink it again.
#[macro_export]
macro_rules! phantom_enum {
    (
        #[$enum_attr:meta]
        pub enum $name:ident {
            $(
                #[$variant_attr:meta]
                $variant:ident
             ),*
        }
    ) => {
        #[$enum_attr]
        #[allow(non_snake_case)]
        mod $name {
            /// Implemented exclusively by members of this phantom type enum.
            /// This is for use as a generic bound.
            pub trait Impl { }

            $(
                #[$variant_attr]
                pub enum $variant { }
                impl Impl for $variant { }
            )*
        }
    }
}
