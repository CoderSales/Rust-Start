The-Cargo-Book-3-8-Build-Scripts

____

[Build Scripts - The Cargo Book](https://doc.rust-lang.org/cargo/reference/build-scripts.html#outputs-of-the-build-script)

[FFI - The Rustonomicon](https://doc.rust-lang.org/nomicon/ffi.html)

[Introduction - The Rustonomicon](https://doc.rust-lang.org/nomicon/intro.html)

[Issues · rust-lang/nomicon](https://github.com/rust-lang/nomicon/issues)

____

libc

[libc - crates.io: Rust Package Registry](https://crates.io/crates/libc)

[rust-lang/libc: Raw bindings to platform APIs for Rust](https://github.com/rust-lang/libc)

docs.rs [libc - Rust](https://docs.rs/libc/0.2.158/libc/)

Type Alias libc::size_t [size_t in libc - Rust](https://docs.rs/libc/0.2.158/libc/type.size_t.html)

docs.rs/libc [mod.rs - source](https://docs.rs/libc/0.2.158/src/libc/unix/mod.rs.html#19)

GitHub [rfcs/text/1291-promote-libc.md - rust-lang/rfcs](https://github.com/rust-lang/rfcs/blob/HEAD/text/1291-promote-libc.md)

____

[Calling foreign functions | FFI | The Rustonomicon](https://doc.rust-lang.org/nomicon/ffi.html#calling-foreign-functions)

____

Trait std::ops::Drop [Drop in std::ops | Rust | doc.rust-lang.org](https://doc.rust-lang.org/std/ops/trait.Drop.html)

[Destructors - The Rust Reference | The Rust Reference | doc.rust-lang.org](https://doc.rust-lang.org/reference/destructors.html)

[Scrutinee | Glossary - The Rust Reference | doc.rust-lang.org](https://doc.rust-lang.org/reference/glossary.html#scrutinee)

[10.4 Interior mutability - The Rust Reference](https://doc.rust-lang.org/reference/interior-mutability.html)

[Shared references (&) | Pointer types | The Rust Reference | doc.rust-lang.org](https://doc.rust-lang.org/reference/types/pointer.html#shared-references-)

[10.1.13. Pointer types | The Rust Reference | doc.rust-lang.org](https://doc.rust-lang.org/reference/types/pointer.html#pointer-types)

[16. Unsafety | 16.2. Behavior considered undefined | The Rust Reference | doc.rust-lang.org](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)

____

[Behavior considered undefined - The Rust Reference](https://doc.rust-lang.org/reference/behavior-considered-undefined.html#places-based-on-misaligned-pointers)

[Places based on misaligned pointers](https://doc.rust-lang.org/reference/behavior-considered-undefined.html#places-based-on-misaligned-pointers)

- A place is said to be 

"based on a misaligned pointer" 

if the last * projection 

during place computation 

was performed on a pointer 

that was not aligned for its type. 

- (If there is no * projection

in the place expression, 

then this is accessing the field of a local 

and rustc will guarantee proper alignment. 

- If there are multiple * projection,

then each of them incurs a load of the pointer-to-be-dereferenced itself from memory, 

and each of these loads is subject to the alignment constraint. 

- Note that some * projections
  
can be omitted in surface Rust syntax 

due to automatic dereferencing; 

we are considering the fully expanded place expression here.)

<br><br>

For instance, 

if ptr has type *const S 

where S has an alignment of 8, 

then ptr must be 8-aligned 

or else 

(*ptr).f 

is 

"based on an misaligned pointer". 

- This is true even if the type of the field f is

u8 (i.e., a type with alignment 1). 

In other words, 

the alignment requirement derives from 

the type of the pointer 

that was dereferenced, 

not 

the type of the field 

that is being accessed.



____
