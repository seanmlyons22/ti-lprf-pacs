#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Input Output Mode This register controls pull-up, pull-down, and output mode for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    pub iomode: IOMODE,
    #[doc = "0x04 - General Purpose Input Output Digital Input Enable This register controls input buffers for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    pub gpiodie: GPIODIE,
    #[doc = "0x08 - Input Output Peripheral Output Enable This register selects the output source for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    pub iopoe: IOPOE,
    #[doc = "0x0c - General Purpose Input Output Data Out The output data register is used to set data on AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    pub gpiodout: GPIODOUT,
    #[doc = "0x10 - General Purpose Input Output Data In This register provides synchronized input data for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth"]
    pub gpiodin: GPIODIN,
    #[doc = "0x14 - General Purpose Input Output Data Out Set Set bits in GPIODOUT in instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    pub gpiodoutset: GPIODOUTSET,
    #[doc = "0x18 - General Purpose Input Output Data Out Clear Clear bits in GPIODOUT instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    pub gpiodoutclr: GPIODOUTCLR,
    #[doc = "0x1c - General Purpose Input Output Data Out Toggle Toggle bits in GPIODOUT in instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    pub gpiodouttgl: GPIODOUTTGL,
    #[doc = "0x20 - Input Output 0 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+0\\]
when IOPOE bit 0 is 1. To avoid glitches on AUXIO\\[8i+0\\]
you must configure this register while IOPOE bit 0 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    pub io0psel: IO0PSEL,
    #[doc = "0x24 - Input Output 1 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+1\\]
when IOPOE bit 1 is 1. To avoid glitches on AUXIO\\[8i+1\\]
you must configure this register while IOPOE bit 1 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    pub io1psel: IO1PSEL,
    #[doc = "0x28 - Input Output 2 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+2\\]
when IOPOE bit 2 is 1. To avoid glitches on AUXIO\\[8i+2\\]
you must configure this register while IOPOE bit 2 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    pub io2psel: IO2PSEL,
    #[doc = "0x2c - Input Output 3 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+3\\]
when IOPOE bit 3 is 1. To avoid glitches on AUXIO\\[8i+3\\]
you must configure this register while IOPOE bit 3 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    pub io3psel: IO3PSEL,
    #[doc = "0x30 - Input Output 4 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+4\\]
when IOPOE bit 4 is 1. To avoid glitches on AUXIO\\[8i+4\\]
you must configure this register while IOPOE bit 4 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    pub io4psel: IO4PSEL,
    #[doc = "0x34 - Input Output 5 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+5\\]
when IOPOE bit 5 is 1. To avoid glitches on AUXIO\\[8i+5\\]
you must configure this register while IOPOE bit 5 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    pub io5psel: IO5PSEL,
    #[doc = "0x38 - Input Output 6 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+6\\]
when IOPOE bit 6 is 1. To avoid glitches on AUXIO\\[8i+6\\]
you must configure this register while IOPOE bit 6 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    pub io6psel: IO6PSEL,
    #[doc = "0x3c - Input Output 7 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+7\\]
when IOPOE bit 7 is 1. To avoid glitches on AUXIO\\[8i+7\\]
you must configure this register while IOPOE bit 7 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    pub io7psel: IO7PSEL,
}
#[doc = "IOMODE (rw) register accessor: an alias for `Reg<IOMODE_SPEC>`"]
pub type IOMODE = crate::Reg<iomode::IOMODE_SPEC>;
#[doc = "Input Output Mode This register controls pull-up, pull-down, and output mode for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod iomode;
#[doc = "GPIODIE (rw) register accessor: an alias for `Reg<GPIODIE_SPEC>`"]
pub type GPIODIE = crate::Reg<gpiodie::GPIODIE_SPEC>;
#[doc = "General Purpose Input Output Digital Input Enable This register controls input buffers for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod gpiodie;
#[doc = "IOPOE (rw) register accessor: an alias for `Reg<IOPOE_SPEC>`"]
pub type IOPOE = crate::Reg<iopoe::IOPOE_SPEC>;
#[doc = "Input Output Peripheral Output Enable This register selects the output source for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod iopoe;
#[doc = "GPIODOUT (rw) register accessor: an alias for `Reg<GPIODOUT_SPEC>`"]
pub type GPIODOUT = crate::Reg<gpiodout::GPIODOUT_SPEC>;
#[doc = "General Purpose Input Output Data Out The output data register is used to set data on AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod gpiodout;
#[doc = "GPIODIN (rw) register accessor: an alias for `Reg<GPIODIN_SPEC>`"]
pub type GPIODIN = crate::Reg<gpiodin::GPIODIN_SPEC>;
#[doc = "General Purpose Input Output Data In This register provides synchronized input data for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth"]
pub mod gpiodin;
#[doc = "GPIODOUTSET (rw) register accessor: an alias for `Reg<GPIODOUTSET_SPEC>`"]
pub type GPIODOUTSET = crate::Reg<gpiodoutset::GPIODOUTSET_SPEC>;
#[doc = "General Purpose Input Output Data Out Set Set bits in GPIODOUT in instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod gpiodoutset;
#[doc = "GPIODOUTCLR (rw) register accessor: an alias for `Reg<GPIODOUTCLR_SPEC>`"]
pub type GPIODOUTCLR = crate::Reg<gpiodoutclr::GPIODOUTCLR_SPEC>;
#[doc = "General Purpose Input Output Data Out Clear Clear bits in GPIODOUT instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod gpiodoutclr;
#[doc = "GPIODOUTTGL (rw) register accessor: an alias for `Reg<GPIODOUTTGL_SPEC>`"]
pub type GPIODOUTTGL = crate::Reg<gpiodouttgl::GPIODOUTTGL_SPEC>;
#[doc = "General Purpose Input Output Data Out Toggle Toggle bits in GPIODOUT in instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod gpiodouttgl;
#[doc = "IO0PSEL (rw) register accessor: an alias for `Reg<IO0PSEL_SPEC>`"]
pub type IO0PSEL = crate::Reg<io0psel::IO0PSEL_SPEC>;
#[doc = "Input Output 0 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+0\\]
when IOPOE bit 0 is 1. To avoid glitches on AUXIO\\[8i+0\\]
you must configure this register while IOPOE bit 0 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod io0psel;
#[doc = "IO1PSEL (rw) register accessor: an alias for `Reg<IO1PSEL_SPEC>`"]
pub type IO1PSEL = crate::Reg<io1psel::IO1PSEL_SPEC>;
#[doc = "Input Output 1 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+1\\]
when IOPOE bit 1 is 1. To avoid glitches on AUXIO\\[8i+1\\]
you must configure this register while IOPOE bit 1 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod io1psel;
#[doc = "IO2PSEL (rw) register accessor: an alias for `Reg<IO2PSEL_SPEC>`"]
pub type IO2PSEL = crate::Reg<io2psel::IO2PSEL_SPEC>;
#[doc = "Input Output 2 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+2\\]
when IOPOE bit 2 is 1. To avoid glitches on AUXIO\\[8i+2\\]
you must configure this register while IOPOE bit 2 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod io2psel;
#[doc = "IO3PSEL (rw) register accessor: an alias for `Reg<IO3PSEL_SPEC>`"]
pub type IO3PSEL = crate::Reg<io3psel::IO3PSEL_SPEC>;
#[doc = "Input Output 3 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+3\\]
when IOPOE bit 3 is 1. To avoid glitches on AUXIO\\[8i+3\\]
you must configure this register while IOPOE bit 3 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod io3psel;
#[doc = "IO4PSEL (rw) register accessor: an alias for `Reg<IO4PSEL_SPEC>`"]
pub type IO4PSEL = crate::Reg<io4psel::IO4PSEL_SPEC>;
#[doc = "Input Output 4 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+4\\]
when IOPOE bit 4 is 1. To avoid glitches on AUXIO\\[8i+4\\]
you must configure this register while IOPOE bit 4 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod io4psel;
#[doc = "IO5PSEL (rw) register accessor: an alias for `Reg<IO5PSEL_SPEC>`"]
pub type IO5PSEL = crate::Reg<io5psel::IO5PSEL_SPEC>;
#[doc = "Input Output 5 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+5\\]
when IOPOE bit 5 is 1. To avoid glitches on AUXIO\\[8i+5\\]
you must configure this register while IOPOE bit 5 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod io5psel;
#[doc = "IO6PSEL (rw) register accessor: an alias for `Reg<IO6PSEL_SPEC>`"]
pub type IO6PSEL = crate::Reg<io6psel::IO6PSEL_SPEC>;
#[doc = "Input Output 6 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+6\\]
when IOPOE bit 6 is 1. To avoid glitches on AUXIO\\[8i+6\\]
you must configure this register while IOPOE bit 6 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod io6psel;
#[doc = "IO7PSEL (rw) register accessor: an alias for `Reg<IO7PSEL_SPEC>`"]
pub type IO7PSEL = crate::Reg<io7psel::IO7PSEL_SPEC>;
#[doc = "Input Output 7 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+7\\]
when IOPOE bit 7 is 1. To avoid glitches on AUXIO\\[8i+7\\]
you must configure this register while IOPOE bit 7 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod io7psel;
