#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cc13x2_cc26x2_pac as cc13x2_cc26x2;
// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

#[entry]
fn main() -> ! {
    let p = cc13x2_cc26x2::Peripherals::take().unwrap();

    // Setup PRCM, power the perpipheral and serial domains
    p.PRCM.pdctl0periph.write(|w| w.on().set_bit());
    p.PRCM.pdctl0serial.write(|w| w.on().set_bit());
    p.PRCM.gpioclkgr.write(|w| w.clk_en().set_bit());
    p.PRCM.clkloadctl.write(|w| w.load().set_bit());

    // Set DIO6 and 7 as outputs
    p.IOC.iocfg6.write(|w| w.ie().clear_bit());
    p.IOC.iocfg7.write(|w| w.ie().clear_bit());
    p.IOC.iocfg18.write(|w| w.ie().clear_bit());

    p.GPIO
        .doe31_0
        .write(|w| w.dio7().set_bit().dio6().set_bit());
    // Set DIO18 as ITM
    p.IOC.iocfg18.write(|w| w.port_id().cpu_swv());

    loop {
        p.GPIO
            .douttgl31_0
            .write(|w| w.dio6().set_bit());

        cortex_m::asm::delay(24_000_000);
    }
}
