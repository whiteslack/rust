// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// So we don't have to document the actual methods on the traits.
#[allow(missing_doc)];

/*!
 *
 * Traits for the built-in operators. Implementing these traits allows you to get
 * an effect similar to overloading operators.
 *
 * The values for the right hand side of an operator are automatically
 * borrowed, so `a + b` is sugar for `a.add(&b)`.
 *
 * All of these traits are imported by the prelude, so they are available in
 * every Rust program.
 *
 * # Example
 *
 * This example creates a `Point` struct that implements `Add` and `Sub`, and then
 * demonstrates adding and subtracting two `Point`s.
 *
 * ```rust
 * struct Point {
 *     x: int,
 *     y: int
 * }
 *
 * impl Add<Point, Point> for Point {
 *     fn add(&self, other: &Point) -> Point {
 *         Point {x: self.x + other.x, y: self.y + other.y}
 *     }
 * }
 *
 * impl Sub<Point, Point> for Point {
 *     fn sub(&self, other: &Point) -> Point {
 *         Point {x: self.x - other.x, y: self.y - other.y}
 *     }
 * }
 * fn main() {
 *     println(format!("{:?}", Point {x: 1, y: 0} + Point {x: 2, y: 3}));
 *     println(format!("{:?}", Point {x: 1, y: 0} - Point {x: 2, y: 3}));
 * }
 * ```
 *
 * See the documentation for each trait for a minimum implementation that prints
 * something to the screen.
 *
 */

/**
 *
 * The `Drop` trait is used to run some code when a value goes out of scope. This
 * is sometimes called a 'destructor'.
 *
 * # Example
 *
 * A trivial implementation of `Drop`. The `drop` method is called when `_x` goes
 * out of scope, and therefore `main` prints `Dropping!`.
 *
 * ```rust
 * struct HasDrop;
 *
 * impl Drop for HasDrop {
 *   fn drop(&mut self) {
 *       println("Dropping!");
 *   }
 * }
 *
 * fn main() {
 *   let _x = HasDrop;
 * }
 * ```
 */
#[lang="drop"]
pub trait Drop {
    fn drop(&mut self);
}

/**
 *
 * The `Add` trait is used to specify the functionality of `+`.
 *
 * # Example
 *
 * A trivial implementation of `Add`. When `Foo + Foo` happens, it ends up
 * calling `add`, and therefore, `main` prints `Adding!`.
 *
 * ```rust
 * struct Foo;
 *
 * impl Add<Foo, Foo> for Foo {
 *     fn add(&self, _rhs: &Foo) -> Foo {
 *       println("Adding!");
 *       *self
 *   }
 * }
 *
 * fn main() {
 *   Foo + Foo;
 * }
 * ```
 */
#[lang="add"]
pub trait Add<RHS,Result> {
    fn add(&self, rhs: &RHS) -> Result;
}

#[lang="add_assign"]
pub trait AddAssign<RHS>: Add<RHS, Self> {
    #[inline]
    fn add_assign(&mut self, rhs: &RHS) {
        *self = *self + *rhs;
    }
}

/**
 *
 * The `Sub` trait is used to specify the functionality of `-`.
 *
 * # Example
 *
 * A trivial implementation of `Sub`. When `Foo - Foo` happens, it ends up
 * calling `sub`, and therefore, `main` prints `Subtracting!`.
 *
 * ```rust
 * struct Foo;
 *
 * impl Sub<Foo, Foo> for Foo {
 *     fn sub(&self, _rhs: &Foo) -> Foo {
 *         println("Subtracting!");
 *         *self
 *     }
 * }
 *
 * fn main() {
 *     Foo - Foo;
 * }
 * ```
 */
#[lang="sub"]
pub trait Sub<RHS,Result> {
    fn sub(&self, rhs: &RHS) -> Result;
}

#[lang="sub_assign"]
pub trait SubAssign<RHS>: Sub<RHS, Self> {
    #[inline]
    fn sub_assign(&mut self, rhs: &RHS) {
        *self = *self - *rhs;
    }
}

/**
 *
 * The `Mul` trait is used to specify the functionality of `*`.
 *
 * # Example
 *
 * A trivial implementation of `Mul`. When `Foo * Foo` happens, it ends up
 * calling `mul`, and therefore, `main` prints `Multiplying!`.
 *
 * ```rust
 * struct Foo;
 *
 * impl Mul<Foo, Foo> for Foo {
 *     fn mul(&self, _rhs: &Foo) -> Foo {
 *         println("Multiplying!");
 *         *self
 *     }
 * }
 *
 * fn main() {
 *     Foo * Foo;
 * }
 * ```
 */
#[lang="mul"]
pub trait Mul<RHS,Result> {
    fn mul(&self, rhs: &RHS) -> Result;
}

#[lang="mul_assign"]
pub trait MulAssign<RHS>: Mul<RHS, Self> {
    #[inline]
    fn mul_assign(&mut self, rhs: &RHS) {
        *self = *self * *rhs;
    }
}

/**
 *
 * The `Div` trait is used to specify the functionality of `/`.
 *
 * # Example
 *
 * A trivial implementation of `Div`. When `Foo / Foo` happens, it ends up
 * calling `div`, and therefore, `main` prints `Dividing!`.
 *
 * ```
 * struct Foo;
 *
 * impl Div<Foo, Foo> for Foo {
 *     fn div(&self, _rhs: &Foo) -> Foo {
 *         println("Dividing!");
 *         *self
 *     }
 * }
 *
 * fn main() {
 *     Foo / Foo;
 * }
 * ```
 */
#[lang="div"]
pub trait Div<RHS,Result> {
    fn div(&self, rhs: &RHS) -> Result;
}

#[lang="div_assign"]
pub trait DivAssign<RHS>: Div<RHS, Self> {
    #[inline]
    fn div_assign(&mut self, rhs: &RHS) {
        *self = *self / *rhs;
    }
}

/**
 *
 * The `Rem` trait is used to specify the functionality of `%`.
 *
 * # Example
 *
 * A trivial implementation of `Rem`. When `Foo % Foo` happens, it ends up
 * calling `rem`, and therefore, `main` prints `Remainder-ing!`.
 *
 * ```
 * struct Foo;
 *
 * impl Rem<Foo, Foo> for Foo {
 *     fn rem(&self, _rhs: &Foo) -> Foo {
 *         println("Remainder-ing!");
 *         *self
 *     }
 * }
 *
 * fn main() {
 *     Foo % Foo;
 * }
 * ```
 */
#[lang="rem"]
pub trait Rem<RHS,Result> {
    fn rem(&self, rhs: &RHS) -> Result;
}

#[lang="rem_assign"]
pub trait RemAssign<RHS>: Rem<RHS, Self> {
    #[inline]
    fn rem_assign(&mut self, rhs: &RHS) {
        *self = *self % *rhs;
    }
}

/**
 *
 * The `Neg` trait is used to specify the functionality of unary `-`.
 *
 * # Example
 *
 * A trivial implementation of `Neg`. When `-Foo` happens, it ends up calling
 * `neg`, and therefore, `main` prints `Negating!`.
 *
 * ```
 * struct Foo;
 *
 * impl Neg<Foo> for Foo {
 *     fn neg(&self) -> Foo {
 *         println("Negating!");
 *         *self
 *     }
 * }
 *
 * fn main() {
 *     -Foo;
 * }
 * ```
 */
#[lang="neg"]
pub trait Neg<Result> {
    fn neg(&self) -> Result;
}

/**
 *
 * The `Not` trait is used to specify the functionality of unary `!`.
 *
 * # Example
 *
 * A trivial implementation of `Not`. When `!Foo` happens, it ends up calling
 * `not`, and therefore, `main` prints `Not-ing!`.
 *
 * ```
 * struct Foo;
 *
 * impl Not<Foo> for Foo {
 *     fn not(&self) -> Foo {
 *         println("Not-ing!");
 *         *self
 *     }
 * }
 *
 * fn main() {
 *     !Foo;
 * }
 * ```
 */
#[lang="not"]
pub trait Not<Result> {
    fn not(&self) -> Result;
}

/**
 *
 * The `BitAnd` trait is used to specify the functionality of `&`.
 *
 * # Example
 *
 * A trivial implementation of `BitAnd`. When `Foo & Foo` happens, it ends up
 * calling `bitand`, and therefore, `main` prints `Bitwise And-ing!`.
 *
 * ```
 * struct Foo;
 *
 * impl BitAnd<Foo, Foo> for Foo {
 *     fn bitand(&self, _rhs: &Foo) -> Foo {
 *         println("Bitwise And-ing!");
 *         *self
 *     }
 * }
 *
 * fn main() {
 *     Foo & Foo;
 * }
 * ```
 */
#[lang="bitand"]
pub trait BitAnd<RHS,Result> {
    fn bitand(&self, rhs: &RHS) -> Result;
}

#[lang="bitand_assign"]
pub trait BitAndAssign<RHS>: BitAnd<RHS, Self> {
    #[inline]
    fn bitand_assign(&mut self, rhs: &RHS) {
        *self = *self & *rhs;
    }
}

/**
 *
 * The `BitOr` trait is used to specify the functionality of `|`.
 *
 * # Example
 *
 * A trivial implementation of `BitOr`. When `Foo | Foo` happens, it ends up
 * calling `bitor`, and therefore, `main` prints `Bitwise Or-ing!`.
 *
 * ```
 * struct Foo;
 *
 * impl BitOr<Foo, Foo> for Foo {
 *     fn bitor(&self, _rhs: &Foo) -> Foo {
 *         println("Bitwise Or-ing!");
 *         *self
 *     }
 * }
 *
 * fn main() {
 *     Foo | Foo;
 * }
 * ```
 */
#[lang="bitor"]
pub trait BitOr<RHS,Result> {
    fn bitor(&self, rhs: &RHS) -> Result;
}

#[lang="bitor_assign"]
pub trait BitOrAssign<RHS>: BitOr<RHS, Self> {
    #[inline]
    fn bitor_assign(&mut self, rhs: &RHS) {
        *self = *self | *rhs;
    }
}

/**
 *
 * The `BitXor` trait is used to specify the functionality of `^`.
 *
 * # Example
 *
 * A trivial implementation of `BitXor`. When `Foo ^ Foo` happens, it ends up
 * calling `bixtor`, and therefore, `main` prints `Bitwise Xor-ing!`.
 *
 * ```
 * struct Foo;
 *
 * impl BitXor<Foo, Foo> for Foo {
 *     fn bitxor(&self, _rhs: &Foo) -> Foo {
 *         println("Bitwise Xor-ing!");
 *         *self
 *     }
 * }
 *
 * fn main() {
 *     Foo ^ Foo;
 * }
 * ```
 */
#[lang="bitxor"]
pub trait BitXor<RHS,Result> {
    fn bitxor(&self, rhs: &RHS) -> Result;
}

#[lang="bitxor_assign"]
pub trait BitXorAssign<RHS>: BitXor<RHS, Self> {
    #[inline]
    fn bitxor_assign(&mut self, rhs: &RHS) {
        *self = *self ^ *rhs;
    }
}

/**
 *
 * The `Shl` trait is used to specify the functionality of `<<`.
 *
 * # Example
 *
 * A trivial implementation of `Shl`. When `Foo << Foo` happens, it ends up
 * calling `shl`, and therefore, `main` prints `Shifting left!`.
 *
 * ```
 * struct Foo;
 *
 * impl Shl<Foo, Foo> for Foo {
 *     fn shl(&self, _rhs: &Foo) -> Foo {
 *         println("Shifting left!");
 *         *self
 *     }
 * }
 *
 * fn main() {
 *     Foo << Foo;
 * }
 * ```
 */
#[lang="shl"]
pub trait Shl<RHS,Result> {
    fn shl(&self, rhs: &RHS) -> Result;
}

#[lang="shl_assign"]
pub trait ShlAssign<RHS>: Shl<RHS, Self> {
    #[inline]
    fn shl_assign(&mut self, rhs: &RHS) {
        *self = *self << *rhs;
    }
}

/**
 *
 * The `Shr` trait is used to specify the functionality of `>>`.
 *
 * # Example
 *
 * A trivial implementation of `Shr`. When `Foo >> Foo` happens, it ends up
 * calling `shr`, and therefore, `main` prints `Shifting right!`.
 *
 * ```
 * struct Foo;
 *
 * impl Shr<Foo, Foo> for Foo {
 *     fn shr(&self, _rhs: &Foo) -> Foo {
 *         println("Shifting right!");
 *         *self
 *     }
 * }
 *
 * fn main() {
 *     Foo >> Foo;
 * }
 * ```
 */
#[lang="shr"]
pub trait Shr<RHS,Result> {
    fn shr(&self, rhs: &RHS) -> Result;
}

#[lang="shr_assign"]
pub trait ShrAssign<RHS>: Shr<RHS, Self> {
    #[inline]
    fn shr_assign(&mut self, rhs: &RHS) {
        *self = *self >> *rhs;
    }
}

/**
 *
 * The `Index` trait is used to specify the functionality of indexing operations
 * like `arr[idx]`.
 *
 * # Example
 *
 * A trivial implementation of `Index`. When `Foo[Foo]` happens, it ends up
 * calling `index`, and therefore, `main` prints `Indexing!`.
 *
 * ```
 * struct Foo;
 *
 * impl Index<Foo, Foo> for Foo {
 *     fn index(&self, _rhs: &Foo) -> Foo {
 *         println("Indexing!");
 *         *self
 *     }
 * }
 *
 * fn main() {
 *     Foo[Foo];
 * }
 * ```
 */
#[lang="index"]
pub trait Index<Index,Result> {
    fn index(&self, index: &Index) -> Result;
}

#[cfg(test)]
mod bench {

    use extra::test::BenchHarness;
    use ops::Drop;

    // Overhead of dtors

    struct HasDtor {
        x: int
    }

    impl Drop for HasDtor {
        fn drop(&mut self) {
        }
    }

    #[bench]
    fn alloc_obj_with_dtor(bh: &mut BenchHarness) {
        bh.iter(|| {
            HasDtor { x : 10 };
        })
    }
}
