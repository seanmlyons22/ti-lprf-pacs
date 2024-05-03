#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    iomode: Iomode,
    gpiodie: Gpiodie,
    iopoe: Iopoe,
    gpiodout: Gpiodout,
    gpiodin: Gpiodin,
    gpiodoutset: Gpiodoutset,
    gpiodoutclr: Gpiodoutclr,
    gpiodouttgl: Gpiodouttgl,
    io0psel: Io0psel,
    io1psel: Io1psel,
    io2psel: Io2psel,
    io3psel: Io3psel,
    io4psel: Io4psel,
    io5psel: Io5psel,
    io6psel: Io6psel,
    io7psel: Io7psel,
}
impl RegisterBlock {
    #[doc = "0x00 - Input Output Mode This register controls pull-up, pull-down, and output mode for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    #[inline(always)]
    pub const fn iomode(&self) -> &Iomode {
        &self.iomode
    }
    #[doc = "0x04 - General Purpose Input Output Digital Input Enable This register controls input buffers for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    #[inline(always)]
    pub const fn gpiodie(&self) -> &Gpiodie {
        &self.gpiodie
    }
    #[doc = "0x08 - Input Output Peripheral Output Enable This register selects the output source for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    #[inline(always)]
    pub const fn iopoe(&self) -> &Iopoe {
        &self.iopoe
    }
    #[doc = "0x0c - General Purpose Input Output Data Out The output data register is used to set data on AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    #[inline(always)]
    pub const fn gpiodout(&self) -> &Gpiodout {
        &self.gpiodout
    }
    #[doc = "0x10 - General Purpose Input Output Data In This register provides synchronized input data for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth"]
    #[inline(always)]
    pub const fn gpiodin(&self) -> &Gpiodin {
        &self.gpiodin
    }
    #[doc = "0x14 - General Purpose Input Output Data Out Set Set bits in GPIODOUT in instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    #[inline(always)]
    pub const fn gpiodoutset(&self) -> &Gpiodoutset {
        &self.gpiodoutset
    }
    #[doc = "0x18 - General Purpose Input Output Data Out Clear Clear bits in GPIODOUT instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    #[inline(always)]
    pub const fn gpiodoutclr(&self) -> &Gpiodoutclr {
        &self.gpiodoutclr
    }
    #[doc = "0x1c - General Purpose Input Output Data Out Toggle Toggle bits in GPIODOUT in instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    #[inline(always)]
    pub const fn gpiodouttgl(&self) -> &Gpiodouttgl {
        &self.gpiodouttgl
    }
    #[doc = "0x20 - Input Output 0 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+0\\]
when IOPOE bit 0 is 1. To avoid glitches on AUXIO\\[8i+0\\]
you must configure this register while IOPOE bit 0 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    #[inline(always)]
    pub const fn io0psel(&self) -> &Io0psel {
        &self.io0psel
    }
    #[doc = "0x24 - Input Output 1 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+1\\]
when IOPOE bit 1 is 1. To avoid glitches on AUXIO\\[8i+1\\]
you must configure this register while IOPOE bit 1 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    #[inline(always)]
    pub const fn io1psel(&self) -> &Io1psel {
        &self.io1psel
    }
    #[doc = "0x28 - Input Output 2 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+2\\]
when IOPOE bit 2 is 1. To avoid glitches on AUXIO\\[8i+2\\]
you must configure this register while IOPOE bit 2 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    #[inline(always)]
    pub const fn io2psel(&self) -> &Io2psel {
        &self.io2psel
    }
    #[doc = "0x2c - Input Output 3 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+3\\]
when IOPOE bit 3 is 1. To avoid glitches on AUXIO\\[8i+3\\]
you must configure this register while IOPOE bit 3 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    #[inline(always)]
    pub const fn io3psel(&self) -> &Io3psel {
        &self.io3psel
    }
    #[doc = "0x30 - Input Output 4 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+4\\]
when IOPOE bit 4 is 1. To avoid glitches on AUXIO\\[8i+4\\]
you must configure this register while IOPOE bit 4 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    #[inline(always)]
    pub const fn io4psel(&self) -> &Io4psel {
        &self.io4psel
    }
    #[doc = "0x34 - Input Output 5 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+5\\]
when IOPOE bit 5 is 1. To avoid glitches on AUXIO\\[8i+5\\]
you must configure this register while IOPOE bit 5 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    #[inline(always)]
    pub const fn io5psel(&self) -> &Io5psel {
        &self.io5psel
    }
    #[doc = "0x38 - Input Output 6 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+6\\]
when IOPOE bit 6 is 1. To avoid glitches on AUXIO\\[8i+6\\]
you must configure this register while IOPOE bit 6 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    #[inline(always)]
    pub const fn io6psel(&self) -> &Io6psel {
        &self.io6psel
    }
    #[doc = "0x3c - Input Output 7 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+7\\]
when IOPOE bit 7 is 1. To avoid glitches on AUXIO\\[8i+7\\]
you must configure this register while IOPOE bit 7 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
    #[inline(always)]
    pub const fn io7psel(&self) -> &Io7psel {
        &self.io7psel
    }
}
#[doc = "IOMODE (rw) register accessor: Input Output Mode This register controls pull-up, pull-down, and output mode for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iomode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iomode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iomode`]
module"]
#[doc(alias = "IOMODE")]
pub type Iomode = crate::Reg<iomode::IomodeSpec>;
#[doc = "Input Output Mode This register controls pull-up, pull-down, and output mode for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod iomode;
#[doc = "GPIODIE (rw) register accessor: General Purpose Input Output Digital Input Enable This register controls input buffers for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiodie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiodie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiodie`]
module"]
#[doc(alias = "GPIODIE")]
pub type Gpiodie = crate::Reg<gpiodie::GpiodieSpec>;
#[doc = "General Purpose Input Output Digital Input Enable This register controls input buffers for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod gpiodie;
#[doc = "IOPOE (rw) register accessor: Input Output Peripheral Output Enable This register selects the output source for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iopoe::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iopoe::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iopoe`]
module"]
#[doc(alias = "IOPOE")]
pub type Iopoe = crate::Reg<iopoe::IopoeSpec>;
#[doc = "Input Output Peripheral Output Enable This register selects the output source for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod iopoe;
#[doc = "GPIODOUT (rw) register accessor: General Purpose Input Output Data Out The output data register is used to set data on AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiodout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiodout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiodout`]
module"]
#[doc(alias = "GPIODOUT")]
pub type Gpiodout = crate::Reg<gpiodout::GpiodoutSpec>;
#[doc = "General Purpose Input Output Data Out The output data register is used to set data on AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod gpiodout;
#[doc = "GPIODIN (rw) register accessor: General Purpose Input Output Data In This register provides synchronized input data for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiodin::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiodin::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiodin`]
module"]
#[doc(alias = "GPIODIN")]
pub type Gpiodin = crate::Reg<gpiodin::GpiodinSpec>;
#[doc = "General Purpose Input Output Data In This register provides synchronized input data for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth"]
pub mod gpiodin;
#[doc = "GPIODOUTSET (rw) register accessor: General Purpose Input Output Data Out Set Set bits in GPIODOUT in instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiodoutset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiodoutset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiodoutset`]
module"]
#[doc(alias = "GPIODOUTSET")]
pub type Gpiodoutset = crate::Reg<gpiodoutset::GpiodoutsetSpec>;
#[doc = "General Purpose Input Output Data Out Set Set bits in GPIODOUT in instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod gpiodoutset;
#[doc = "GPIODOUTCLR (rw) register accessor: General Purpose Input Output Data Out Clear Clear bits in GPIODOUT instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiodoutclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiodoutclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiodoutclr`]
module"]
#[doc(alias = "GPIODOUTCLR")]
pub type Gpiodoutclr = crate::Reg<gpiodoutclr::GpiodoutclrSpec>;
#[doc = "General Purpose Input Output Data Out Clear Clear bits in GPIODOUT instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod gpiodoutclr;
#[doc = "GPIODOUTTGL (rw) register accessor: General Purpose Input Output Data Out Toggle Toggle bits in GPIODOUT in instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiodouttgl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiodouttgl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiodouttgl`]
module"]
#[doc(alias = "GPIODOUTTGL")]
pub type Gpiodouttgl = crate::Reg<gpiodouttgl::GpiodouttglSpec>;
#[doc = "General Purpose Input Output Data Out Toggle Toggle bits in GPIODOUT in instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod gpiodouttgl;
#[doc = "IO0PSEL (rw) register accessor: Input Output 0 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+0\\]
when IOPOE bit 0 is 1. To avoid glitches on AUXIO\\[8i+0\\]
you must configure this register while IOPOE bit 0 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io0psel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io0psel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io0psel`]
module"]
#[doc(alias = "IO0PSEL")]
pub type Io0psel = crate::Reg<io0psel::Io0pselSpec>;
#[doc = "Input Output 0 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+0\\]
when IOPOE bit 0 is 1. To avoid glitches on AUXIO\\[8i+0\\]
you must configure this register while IOPOE bit 0 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod io0psel;
#[doc = "IO1PSEL (rw) register accessor: Input Output 1 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+1\\]
when IOPOE bit 1 is 1. To avoid glitches on AUXIO\\[8i+1\\]
you must configure this register while IOPOE bit 1 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io1psel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io1psel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io1psel`]
module"]
#[doc(alias = "IO1PSEL")]
pub type Io1psel = crate::Reg<io1psel::Io1pselSpec>;
#[doc = "Input Output 1 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+1\\]
when IOPOE bit 1 is 1. To avoid glitches on AUXIO\\[8i+1\\]
you must configure this register while IOPOE bit 1 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod io1psel;
#[doc = "IO2PSEL (rw) register accessor: Input Output 2 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+2\\]
when IOPOE bit 2 is 1. To avoid glitches on AUXIO\\[8i+2\\]
you must configure this register while IOPOE bit 2 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io2psel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io2psel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io2psel`]
module"]
#[doc(alias = "IO2PSEL")]
pub type Io2psel = crate::Reg<io2psel::Io2pselSpec>;
#[doc = "Input Output 2 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+2\\]
when IOPOE bit 2 is 1. To avoid glitches on AUXIO\\[8i+2\\]
you must configure this register while IOPOE bit 2 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod io2psel;
#[doc = "IO3PSEL (rw) register accessor: Input Output 3 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+3\\]
when IOPOE bit 3 is 1. To avoid glitches on AUXIO\\[8i+3\\]
you must configure this register while IOPOE bit 3 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io3psel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io3psel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io3psel`]
module"]
#[doc(alias = "IO3PSEL")]
pub type Io3psel = crate::Reg<io3psel::Io3pselSpec>;
#[doc = "Input Output 3 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+3\\]
when IOPOE bit 3 is 1. To avoid glitches on AUXIO\\[8i+3\\]
you must configure this register while IOPOE bit 3 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod io3psel;
#[doc = "IO4PSEL (rw) register accessor: Input Output 4 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+4\\]
when IOPOE bit 4 is 1. To avoid glitches on AUXIO\\[8i+4\\]
you must configure this register while IOPOE bit 4 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io4psel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io4psel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io4psel`]
module"]
#[doc(alias = "IO4PSEL")]
pub type Io4psel = crate::Reg<io4psel::Io4pselSpec>;
#[doc = "Input Output 4 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+4\\]
when IOPOE bit 4 is 1. To avoid glitches on AUXIO\\[8i+4\\]
you must configure this register while IOPOE bit 4 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod io4psel;
#[doc = "IO5PSEL (rw) register accessor: Input Output 5 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+5\\]
when IOPOE bit 5 is 1. To avoid glitches on AUXIO\\[8i+5\\]
you must configure this register while IOPOE bit 5 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io5psel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io5psel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io5psel`]
module"]
#[doc(alias = "IO5PSEL")]
pub type Io5psel = crate::Reg<io5psel::Io5pselSpec>;
#[doc = "Input Output 5 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+5\\]
when IOPOE bit 5 is 1. To avoid glitches on AUXIO\\[8i+5\\]
you must configure this register while IOPOE bit 5 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod io5psel;
#[doc = "IO6PSEL (rw) register accessor: Input Output 6 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+6\\]
when IOPOE bit 6 is 1. To avoid glitches on AUXIO\\[8i+6\\]
you must configure this register while IOPOE bit 6 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io6psel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io6psel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io6psel`]
module"]
#[doc(alias = "IO6PSEL")]
pub type Io6psel = crate::Reg<io6psel::Io6pselSpec>;
#[doc = "Input Output 6 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+6\\]
when IOPOE bit 6 is 1. To avoid glitches on AUXIO\\[8i+6\\]
you must configure this register while IOPOE bit 6 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod io6psel;
#[doc = "IO7PSEL (rw) register accessor: Input Output 7 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+7\\]
when IOPOE bit 7 is 1. To avoid glitches on AUXIO\\[8i+7\\]
you must configure this register while IOPOE bit 7 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io7psel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io7psel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io7psel`]
module"]
#[doc(alias = "IO7PSEL")]
pub type Io7psel = crate::Reg<io7psel::Io7pselSpec>;
#[doc = "Input Output 7 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+7\\]
when IOPOE bit 7 is 1. To avoid glitches on AUXIO\\[8i+7\\]
you must configure this register while IOPOE bit 7 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth."]
pub mod io7psel;
