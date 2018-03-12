#![allow(unused_must_use)]

extern crate verilog;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let code: verilog::ast::Module = verilog::parse("
          `default_nettype none

module xenowing(
    input reset_n,
    input clk,

    output [13:0] program_rom_addr,
    input [31:0] program_rom_q,

    output [2:0] leds,

    output uart_tx,

    input avl_ready,
    output avl_burstbegin,
    output [23:0] avl_addr,
    input avl_rdata_valid,
    input [63:0] avl_rdata,
    output [63:0] avl_wdata,
    output [7:0] avl_be,
    output avl_read_req,
    output avl_write_req,
    output [6:0] avl_size);

    logic [13:0] program_rom_interface_addr;
    logic program_rom_read_req;
    logic [31:0] program_rom_read_data;
    logic program_rom_read_data_valid;
    program_rom_interface program_rom_interface0(
        .reset_n(reset_n),
        .clk(clk),

        .addr(program_rom_interface_addr),
        .read_req(program_rom_read_req),
        .read_data(program_rom_read_data),
        .read_data_valid(program_rom_read_data_valid),
        
        .program_rom_addr(program_rom_addr),
        .program_rom_q(program_rom_q));

    logic [31:0] led_interface_write_data;
    logic [3:0] led_interface_byte_enable;
    logic led_interface_write_req;
    logic led_interface_read_req;
    logic [31:0] led_interface_read_data;
    logic led_interface_read_data_valid;
    led_interface led_interface0(
        .reset_n(reset_n),
        .clk(clk),

        .write_data(led_interface_write_data),
        .byte_enable(led_interface_byte_enable),
        .write_req(led_interface_write_req),
        .read_req(led_interface_read_req),
        .read_data(led_interface_read_data),
        .read_data_valid(led_interface_read_data_valid),

        .leds(leds));

    logic [7:0] uart_transmitter_write_data;
    logic uart_transmitter_write_req;
    logic uart_transmitter_ready;
    uart_transmitter uart_transmitter0(
        .reset_n(reset_n),
        .clk(clk),

        .write_data(uart_transmitter_write_data),
        .write_req(uart_transmitter_write_req),
        .ready(uart_transmitter_ready),

        .tx(uart_tx));

    logic uart_transmitter_interface_addr;
    logic [31:0] uart_transmitter_interface_write_data;
    logic [3:0] uart_transmitter_interface_byte_enable;
    logic uart_transmitter_interface_write_req;
    logic uart_transmitter_interface_read_req;
    logic [31:0] uart_transmitter_interface_read_data;
    logic uart_transmitter_interface_read_data_valid;
    uart_transmitter_interface uart_transmitter_interface0(
        .reset_n(reset_n),
        .clk(clk),

        .addr(uart_transmitter_interface_addr),
        .write_data(uart_transmitter_interface_write_data),
        .byte_enable(uart_transmitter_interface_byte_enable),
        .write_req(uart_transmitter_interface_write_req),
        .read_req(uart_transmitter_interface_read_req),
        .read_data(uart_transmitter_interface_read_data),
        .read_data_valid(uart_transmitter_interface_read_data_valid),

        .uart_write_data(uart_transmitter_write_data),
        .uart_write_req(uart_transmitter_write_req),
        .uart_ready(uart_transmitter_ready));

    logic ddr3_interface_ready;
    logic [26:0] ddr3_interface_addr;
    logic [31:0] ddr3_interface_write_data;
    logic [3:0] ddr3_interface_byte_enable;
    logic ddr3_interface_write_req;
    logic ddr3_interface_read_req;
    logic [31:0] ddr3_interface_read_data;
    logic ddr3_interface_read_data_valid;
    ddr3_interface ddr3_interface0(
        .reset_n(reset_n),
        .clk(clk),

        .ready(ddr3_interface_ready),
        .addr(ddr3_interface_addr),
        .write_data(ddr3_interface_write_data),
        .byte_enable(ddr3_interface_byte_enable),
        .write_req(ddr3_interface_write_req),
        .read_req(ddr3_interface_read_req),
        .read_data(ddr3_interface_read_data),
        .read_data_valid(ddr3_interface_read_data_valid),

        .avl_ready(avl_ready),
        .avl_burstbegin(avl_burstbegin),
        .avl_addr(avl_addr),
        .avl_rdata_valid(avl_rdata_valid),
        .avl_rdata(avl_rdata),
        .avl_wdata(avl_wdata),
        .avl_be(avl_be),
        .avl_read_req(avl_read_req),
        .avl_write_req(avl_write_req),
        .avl_size(avl_size));

    logic mem_mapper_ready;
    logic [31:0] mem_mapper_addr;
    logic [31:0] mem_mapper_write_data;
    logic [3:0] mem_mapper_byte_enable;
    logic mem_mapper_write_req;
    logic mem_mapper_read_req;
    logic [31:0] mem_mapper_read_data;
    logic mem_mapper_read_data_valid;
    mem_mapper mem_mapper0(
        .reset_n(reset_n),
        .clk(clk),

        .ready(mem_mapper_ready),
        .addr(mem_mapper_addr),
        .write_data(mem_mapper_write_data),
        .byte_enable(mem_mapper_byte_enable),
        .write_req(mem_mapper_write_req),
        .read_req(mem_mapper_read_req),
        .read_data(mem_mapper_read_data),
        .read_data_valid(mem_mapper_read_data_valid),

        .program_rom_interface_addr(program_rom_interface_addr),
        .program_rom_interface_read_req(program_rom_read_req),
        .program_rom_interface_read_data(program_rom_read_data),
        .program_rom_interface_read_data_valid(program_rom_read_data_valid),

        .led_interface_write_data(led_interface_write_data),
        .led_interface_byte_enable(led_interface_byte_enable),
        .led_interface_write_req(led_interface_write_req),
        .led_interface_read_req(led_interface_read_req),
        .led_interface_read_data(led_interface_read_data),
        .led_interface_read_data_valid(led_interface_read_data_valid),

        .uart_transmitter_interface_addr(uart_transmitter_interface_addr),
        .uart_transmitter_interface_write_data(uart_transmitter_interface_write_data),
        .uart_transmitter_interface_byte_enable(uart_transmitter_interface_byte_enable),
        .uart_transmitter_interface_write_req(uart_transmitter_interface_write_req),
        .uart_transmitter_interface_read_req(uart_transmitter_interface_read_req),
        .uart_transmitter_interface_read_data(uart_transmitter_interface_read_data),
        .uart_transmitter_interface_read_data_valid(uart_transmitter_interface_read_data_valid),

        .ddr3_interface_ready(ddr3_interface_ready),
        .ddr3_interface_addr(ddr3_interface_addr),
        .ddr3_interface_write_data(ddr3_interface_write_data),
        .ddr3_interface_byte_enable(ddr3_interface_byte_enable),
        .ddr3_interface_write_req(ddr3_interface_write_req),
        .ddr3_interface_read_req(ddr3_interface_read_req),
        .ddr3_interface_read_data(ddr3_interface_read_data),
        .ddr3_interface_read_data_valid(ddr3_interface_read_data_valid));

    cpu cpu0(
        .reset_n(reset_n),
        .clk(clk),

        .mem_ready(mem_mapper_ready),
        .mem_addr(mem_mapper_addr),
        .mem_write_data(mem_mapper_write_data),
        .mem_byte_enable(mem_mapper_byte_enable),
        .mem_write_req(mem_mapper_write_req),
        .mem_read_req(mem_mapper_read_req),
        .mem_read_data(mem_mapper_read_data),
        .mem_read_data_valid(mem_mapper_read_data_valid));

endmodule
          ");

  // write driver.cpp file
  let mut file = File::create(format!("{}_driver.cpp", (code.0).0)).unwrap();
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
    }

    top = new Vsdram();

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

  // write rust driver file
  let mut file = File::create(format!("{}_driver.rs", (code.0).0)).unwrap();
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
}
