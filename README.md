Benchmarks for constant-time array comparison in Rust.

Implemented styles:

* imperative
    * generic `<T: PartialEq>`
        * with bound checking
        * no bound checking (still safe)
    * specialised `u8`
        * with bound checking
        * no bound checking (still safe)
* functional
    * generic `<T: PartialEq>`
    * specialised `u8`
