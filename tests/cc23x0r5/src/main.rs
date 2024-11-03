#![no_std]
#![no_main]

use cc23x0r5::ccfg::Ccfg;
use cc23x0r5_pac as cc23x0r5;
use cortex_m_rt::entry;
// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger
use rtt_target::{rprintln, rtt_init_print};

#[used]
#[no_mangle]
#[link_section = ".ccfg"]
pub static CCFG: Ccfg = Ccfg::new();

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Init");
    let p = cc23x0r5::Peripherals::take().unwrap();
    p.gpio.doe15_12().write(|w| w.dio15().set_bit());

    loop {
        p.gpio.douttgl15_12().write(|w| w.dio15().set_bit());
        cortex_m::asm::delay(24_000_000);
    }
}
