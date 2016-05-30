// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Topes

// #![warn(missing_docs)]
#![deny(trivial_numeric_casts,
        unsafe_code, unstable_features,
        unused_import_braces, unused_qualifications)]

extern crate uuid;

use std::ops::Range;

use uuid::Uuid;

pub struct Description<'tope> {
    pub name: &'tope str,
    pub uuid: Uuid,
    pub description: &'tope str,
    pub example: &'tope str,
}

pub struct Part<'tope> {
    pub name: &'tope str,
    pub uuid: Uuid,
    pub count: Range<usize>,
}

pub struct Variant {
    pub parts: Vec<Uuid>,
}

pub struct Tope<'tope> {
    pub description: Description<'tope>,
    pub parts: Vec<Part<'tope>>,
    pub variants: Vec<Variant>,
}

#[cfg(test)]
mod tests {
    use super::*;

    use uuid::Uuid;

    #[test]
    fn it_works() {
        let mut tope = Tope {
            description: Description {
                name: "Test tope",
                uuid: Uuid::new_v4(),
                description: "A test tope.",
                example: "Not really sure yet.",
            },
            parts: vec![Part {
                            name: "A",
                            uuid: Uuid::new_v4(),
                            count: 1..2,
                        },
                        Part {
                            name: "B",
                            uuid: Uuid::new_v4(),
                            count: 1..2,
                        }],
            variants: vec![],
        };
        tope.variants.push(Variant { parts: vec![tope.parts[0].uuid] });
    }
}
