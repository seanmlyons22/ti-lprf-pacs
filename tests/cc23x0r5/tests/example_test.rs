#![no_std]
#![no_main]
use cc23x0r5_pac::ccfg::Ccfg;

// The exception crate provides the vector table
#[allow(unused_imports)]
use cortex_m_rt::exception;


#[used]
#[no_mangle]
#[link_section = ".ccfg"]
pub static CCFG: Ccfg = Ccfg::new().update_crcs();

/// Sets up the logging before entering the test-body, so that embedded-test internal logs (e.g. Running Test <...>)  can also be printed.
/// Note: you can also inline this method in the attribute. e.g. `#[embedded_test::tests(setup=rtt_target::rtt_init_log!())]`
fn setup_log() {
    rtt_target::rtt_init_defmt!();

}


#[cfg(test)]
#[embedded_test::tests(setup=crate::setup_log())]
mod tests {
    use cc23x0r5_pac::Peripherals;

    // An init function which is called before every test
    #[init]
    fn init() -> Peripherals {
        Peripherals::take().unwrap()
    }

    // Initiailize the SYSTIM peripheral ensure it's ticking
    #[test]
    fn check_systim(state: Peripherals) {

        // Initialize the SYSTIM peripheral
        let systim = state.systim;
        let running = systim.status().read().val().bit();
        let sync_in_progress = systim.status().read().syncup().bit();

        // SYSTIM should be running
        assert_eq!(running, true);
        // SYSTIM should already be in sync since this happens on the first LF clock edge
        assert_eq!(sync_in_progress, false);
    }

    #[test]
    fn check_gpio(state: Peripherals) {
        let gpio = state.gpio;
        gpio.doe15_12().write(|w| w.dio15().set_bit());
        let state_before = gpio.dout15_12().read().dio15().bit();
        gpio.douttgl15_12().write(|w| w.dio15().set_bit());
        let state_after = gpio.dout15_12().read().dio15().bit();
        assert_ne!(state_before, state_after);
    }
}
