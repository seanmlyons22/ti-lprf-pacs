#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - General Purpose Input Output Data Out The output data register is used to set data on AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1"]
    pub gpiodout: GPIODOUT,
    #[doc = "0x04 - Input Output Mode This register controls pull-up, pull-down, and output mode for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1"]
    pub iomode: IOMODE,
    #[doc = "0x08 - General Purpose Input Output Data In This register provides synchronized input data for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and I = 1 for AUX_AIODIO1."]
    pub gpiodin: GPIODIN,
    #[doc = "0x0c - General Purpose Input Output Data Out Set Set bits in GPIODOUT in instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1."]
    pub gpiodoutset: GPIODOUTSET,
    #[doc = "0x10 - General Purpose Input Output Data Out Clear Clear bits in GPIODOUT instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1."]
    pub gpiodoutclr: GPIODOUTCLR,
    #[doc = "0x14 - General Purpose Input Output Data Out Toggle Toggle bits in GPIODOUT in instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1."]
    pub gpiodouttgl: GPIODOUTTGL,
    #[doc = "0x18 - General Purpose Input Output Digital Input Enable This register controls input buffers for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and I = 1 for AUX_AIODIO1."]
    pub gpiodie: GPIODIE,
}
#[doc = "GPIODOUT (rw) register accessor: an alias for `Reg<GPIODOUT_SPEC>`"]
pub type GPIODOUT = crate::Reg<gpiodout::GPIODOUT_SPEC>;
#[doc = "General Purpose Input Output Data Out The output data register is used to set data on AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1"]
pub mod gpiodout;
#[doc = "IOMODE (rw) register accessor: an alias for `Reg<IOMODE_SPEC>`"]
pub type IOMODE = crate::Reg<iomode::IOMODE_SPEC>;
#[doc = "Input Output Mode This register controls pull-up, pull-down, and output mode for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1"]
pub mod iomode;
#[doc = "GPIODIN (rw) register accessor: an alias for `Reg<GPIODIN_SPEC>`"]
pub type GPIODIN = crate::Reg<gpiodin::GPIODIN_SPEC>;
#[doc = "General Purpose Input Output Data In This register provides synchronized input data for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and I = 1 for AUX_AIODIO1."]
pub mod gpiodin;
#[doc = "GPIODOUTSET (rw) register accessor: an alias for `Reg<GPIODOUTSET_SPEC>`"]
pub type GPIODOUTSET = crate::Reg<gpiodoutset::GPIODOUTSET_SPEC>;
#[doc = "General Purpose Input Output Data Out Set Set bits in GPIODOUT in instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1."]
pub mod gpiodoutset;
#[doc = "GPIODOUTCLR (rw) register accessor: an alias for `Reg<GPIODOUTCLR_SPEC>`"]
pub type GPIODOUTCLR = crate::Reg<gpiodoutclr::GPIODOUTCLR_SPEC>;
#[doc = "General Purpose Input Output Data Out Clear Clear bits in GPIODOUT instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1."]
pub mod gpiodoutclr;
#[doc = "GPIODOUTTGL (rw) register accessor: an alias for `Reg<GPIODOUTTGL_SPEC>`"]
pub type GPIODOUTTGL = crate::Reg<gpiodouttgl::GPIODOUTTGL_SPEC>;
#[doc = "General Purpose Input Output Data Out Toggle Toggle bits in GPIODOUT in instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1."]
pub mod gpiodouttgl;
#[doc = "GPIODIE (rw) register accessor: an alias for `Reg<GPIODIE_SPEC>`"]
pub type GPIODIE = crate::Reg<gpiodie::GPIODIE_SPEC>;
#[doc = "General Purpose Input Output Digital Input Enable This register controls input buffers for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and I = 1 for AUX_AIODIO1."]
pub mod gpiodie;
