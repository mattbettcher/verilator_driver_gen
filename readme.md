This is a simple project to generate generic 'driver' files for use with [Verilator](https://www.veripool.org/projects/verilator/wiki/Intro).

It is based on the work [ferris](https://github.com/yupferris) did for [xenowing](https://github.com/yupferris/xenowing).

Current limitations -

* Port sizes > 64 bits aren't supported
* Any expressions in the port definition (ie. output [*Width - 1*:0] data)
* Only generates generic drivers (ie. not like this [alu](https://github.com/yupferris/xenowing/tree/master/sim/alu-test) from xenowing)

Todo -

- [x] Command line interface
- [x] Add different output directories for each file type
- [x] Automatically extract 'module port' definition from SystemVerilog file
- [x] Parse 'module port' definitions
- [x] Output a generic driver in CPP
- [x] Output a generic driver in Rust

Stretch Goals -

- [ ] Add an option to create a new cargo lib including the the Rust driver file
- [ ] Allow custom enum based ports somehow 
- [ ] Tuples everywhere are very cryptic. Fix this!

Good job, nearly done!

Bugs - 

---

- [ ] Should warn before overwrite? Maybe make a .bak file?
- [x] Rust struct names should be capitalized
- [x] wrong class name!
- [x] bit size in cpp gen.
- [x] bit size in rust gen.

This is public domain.

For more Verilog & Rust see [rust-verilog](https://github.com/tcr/rust-verilog) which this project is partially based on (I also use [lalrpop](https://github.com/lalrpop/lalrpop), but I only parse a small portion of SystemVerilog)