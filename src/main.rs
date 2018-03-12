#![allow(unused_must_use)]

extern crate verilog;
extern crate clap;
use clap::{Arg, App};

use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

fn main() {

    let matches = App::new("verilator driver gen")
            .version("0.1.0")
            .author("Matthew Bettcher <mattbettcher@gmail.com>")
            .about("Generates both a CPP & Rust generic driver for use with Verilator.")
            .arg(Arg::with_name("input")
                .required(true)
                .index(1)
                .help("Input file"))
            .arg(Arg::with_name("cpp")
                .short("c")
                .long("cpp")
                .takes_value(true)
                .help("Output directory, if different from input directory, for the CPP file"))
            .arg(Arg::with_name("rs")
                .short("r")
                .long("rs")
                .takes_value(true)
                .help("Output directory, if different from input directory, for the Rust file"))
            .get_matches();

    let mut buffer = String::new();
    if let Some(path_str) = matches.value_of("input") {
        let path_with_filename = Path::new(path_str);
        let mut cpp_path = path_with_filename.parent().unwrap_or(Path::new(""));
        let mut rs_path = path_with_filename.parent().unwrap_or(Path::new(""));
        if matches.is_present("cpp") {
            cpp_path = Path::new(matches.value_of("cpp").unwrap());
        }
        if matches.is_present("rs") {
            rs_path = Path::new(matches.value_of("rs").unwrap());
        }
        if let Ok(mut file) = File::open(path_with_filename) {
            file.read_to_string(&mut buffer);
            let code: verilog::ast::Module = verilog::parse(&buffer.as_str());
    

  // write driver.cpp file
  let mut file = File::create(format!("{}/{}_driver.cpp", cpp_path.display(), (code.0).0)).unwrap();
  file.write_fmt(format_args!("#include \"../obj_dir/V{}.h\"\n", (code.0).0));
  file.write_all(b"#include \"verilated_vcd_c.h\"

#include <cinttypes>
#include <iostream>
using namespace std;

#ifdef _WIN32
#include <Windows.h>
#else
#include <dlfcn.h>
#endif

typedef struct
{\n");

  for port in &code.2 {
    let mut bit_size = 32;
    if let &Some(ref bit_range) = &port.2 {
      if (bit_range.0).0 > 31 {
        bit_size = 64;
      }
    }
    file.write_fmt(format_args!("\tuint{}_t (*get_{})();\n", bit_size, (port.0).0));
    file.write_fmt(format_args!("\tvoid (*set_{})(uint{}_t value);\n\n", (port.0).0, bit_size));
  }

  file.write_all(b"\tvoid (*eval)();
    void (*final)();
    void (*trace_dump)(uint64_t time);
} Env;

static Vtest *top = nullptr;
static VerilatedVcdC *trace = nullptr;\n\n");

  for port in &code.2 {
    let mut bit_size = 32;
    if let &Some(ref bit_range) = &port.2 {
      if (bit_range.0).0 > 31 {
        bit_size = 64;
      }
    }
    file.write_fmt(format_args!("uint{}_t get_{}()
{{
    return top->{};
}}\n", bit_size, (port.0).0, (port.0).0));

  file.write_fmt(format_args!("void set_{}(uint{}_t value)
{{
    top->{} = value;
}}\n\n", (port.0).0, bit_size, (port.0).0));
  }

  file.write_all(b"void eval()
{
    top->eval();
}

void final()
{
    top->final();
}

void trace_dump(uint64_t time)
{
    if (trace)
        trace->dump((vluint64_t)time);
}

int main(int argc, char **argv)
{
    if (argc != 2 && argc != 3)
    {
        cout << \"Invalid number of arguments\" << endl;
        exit(1);
    }

    auto libName = argv[1];
    auto lib =
#ifdef _WIN32
        LoadLibrary(libName);
#else
        dlopen(libName, RTLD_LAZY);
#endif
    if (!lib)
    {
        cout << \"Failed to load library: \" << libName << endl;
        exit(1);
    }
    auto run = (int32_t (*)(Env *))
#ifdef _WIN32
        GetProcAddress
#else
        dlsym
#endif
        (lib, \"run\");
    if (!run)
    {
        cout << \"Failed to get run proc address\" << endl;
        exit(1);
    }\n\n");
    file.write_fmt(format_args!("\ttop = new V{}();\n", (code.0).0));
    file.write_all(b"
    if (argc == 3)
    {
        Verilated::traceEverOn(true);
        trace = new VerilatedVcdC();
        top->trace(trace, 99); // TODO: What is this param?
        trace->open(argv[2]);
    }
    Env env =
    {\n\n");

    for port in &code.2 {
      file.write_fmt(format_args!("\t\t.get_{} = get_{},\n", (port.0).0, (port.0).0));
      file.write_fmt(format_args!("\t\t.set_{} = set_{},\n\n", (port.0).0, (port.0).0));
    }

    file.write_all(b"\t\t.eval = eval,
        .final = final,
        .trace_dump = trace_dump,
    };

    auto ret = run(&env);

    if (trace)
    {
        trace->close();
        delete trace;
    }

    delete top;

    return ret;
}
");
    println!("Generated: {}/{}_driver.cpp", cpp_path.display(), (code.0).0);

  // write rust driver file
  let mut file = File::create(format!("{}/{}_driver.rs", rs_path.display(), (code.0).0)).unwrap();
  file.write_all(b"#![allow(dead_code)]

#[repr(C)]
pub struct Env {
\n");

  for port in &code.2 {
    let mut bit_size = 32;
    if let &Some(ref bit_range) = &port.2 {
      if (bit_range.0).0 > 31 {
        bit_size = 64;
      }
    }
    file.write_fmt(format_args!("\tget_{}: extern \"C\" fn() -> u{},\n", (port.0).0, bit_size));
    file.write_fmt(format_args!("\tset_{}: extern \"C\" fn(value: u{}),\n\n", (port.0).0, bit_size));
  }

  file.write_all(b"\teval: extern \"C\" fn(),
    final_: extern \"C\" fn(),
    trace_dump: extern \"C\" fn(time: u64),
}\n\n");

  file.write_fmt(format_args!("pub struct {} {{
    env: *const Env,
}}\n
impl {} {{
  pub fn new(env: *const Env) -> {} {{
        {} {{
            env: env,
        }}
    }}\n\n", (code.0).0, (code.0).0, (code.0).0, (code.0).0));

  for port in &code.2 {
    let mut bit_size = 32;
    if let &Some(ref bit_range) = &port.2 {
      if (bit_range.0).0 > 31 {
        bit_size = 64;
      }
      file.write_fmt(format_args!("\tpub fn {}(&self) -> u{} {{
        unsafe {{ ((*self.env).get_{})() }}
    }}

    pub fn set_{}(&mut self, value: u{}) {{
        unsafe {{
            ((*self.env).set_{})(value);
        }}
    }}\n\n", (port.0).0, bit_size, (port.0).0, (port.0).0, bit_size, (port.0).0));
    } else {
      // one bit port
      file.write_fmt(format_args!("\tpub fn {}(&self) -> bool {{
        unsafe {{ ((*self.env).get_{})() != 0 }}
    }}

    pub fn set_{}(&mut self, value: bool) {{
        unsafe {{
            ((*self.env).set_{})(if value {{ 1 }} else {{ 0 }});
        }}
    }}\n\n", (port.0).0, (port.0).0, (port.0).0, (port.0).0));
    }
  }

  file.write_all(b"\tpub fn eval(&mut self) {
        unsafe {
            ((*self.env).eval)();
        }
    }

    pub fn final_(&mut self) {
        unsafe {
            ((*self.env).final_)();
        }
    }

    pub fn trace_dump(&mut self, time: u64) {
        unsafe {
            ((*self.env).trace_dump)(time);
        }
    }
}");

    println!("Generated: {}/{}_driver.rs", rs_path.display(), (code.0).0);
        } else {
            println!("File not found: {}", path_with_filename.display());
        }
    }
}
