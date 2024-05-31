#![no_std]
#![no_main]

use cc13x4_cc26x4::ccfg::Ccfg;
use cc13x4_cc26x4_pac as cc13x4_cc26x4;
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
    let p = cc13x4_cc26x4::Peripherals::take().unwrap();

    // Setup PRCM, power the perpipheral and serial domains
    p.prcm.pdctl0periph().write(|w| w.on().set_bit());
    p.prcm.pdctl0serial().write(|w| w.on().set_bit());
    p.prcm.gpioclkgr().write(|w| w.clk_en().set_bit());
    p.prcm.clkloadctl().write(|w| w.load().set_bit());

    // Set DIO6 and 7 as outputs
    p.ioc.iocfg6().write(|w| w.ie().clear_bit());
    p.ioc.iocfg7().write(|w| w.ie().clear_bit());
    p.ioc.iocfg18().write(|w| w.ie().clear_bit());

    p.gpio
        .doe31_0()
        .write(|w| w.dio7().set_bit().dio6().set_bit());
    // Set DIO18 as ITM
    p.ioc.iocfg18().write(|w| w.port_id().cpu_swv());

    loop {
        p.gpio.douttgl31_0().write(|w| w.dio6().set_bit());

        cortex_m::asm::delay(24_000_000);
    }
}
