#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gpiodout: Gpiodout,
    iomode: Iomode,
    gpiodin: Gpiodin,
    gpiodoutset: Gpiodoutset,
    gpiodoutclr: Gpiodoutclr,
    gpiodouttgl: Gpiodouttgl,
    gpiodie: Gpiodie,
}
impl RegisterBlock {
    #[doc = "0x00 - General Purpose Input Output Data Out The output data register is used to set data on AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1"]
    #[inline(always)]
    pub const fn gpiodout(&self) -> &Gpiodout {
        &self.gpiodout
    }
    #[doc = "0x04 - Input Output Mode This register controls pull-up, pull-down, and output mode for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1"]
    #[inline(always)]
    pub const fn iomode(&self) -> &Iomode {
        &self.iomode
    }
    #[doc = "0x08 - General Purpose Input Output Data In This register provides synchronized input data for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and I = 1 for AUX_AIODIO1."]
    #[inline(always)]
    pub const fn gpiodin(&self) -> &Gpiodin {
        &self.gpiodin
    }
    #[doc = "0x0c - General Purpose Input Output Data Out Set Set bits in GPIODOUT in instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1."]
    #[inline(always)]
    pub const fn gpiodoutset(&self) -> &Gpiodoutset {
        &self.gpiodoutset
    }
    #[doc = "0x10 - General Purpose Input Output Data Out Clear Clear bits in GPIODOUT instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1."]
    #[inline(always)]
    pub const fn gpiodoutclr(&self) -> &Gpiodoutclr {
        &self.gpiodoutclr
    }
    #[doc = "0x14 - General Purpose Input Output Data Out Toggle Toggle bits in GPIODOUT in instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1."]
    #[inline(always)]
    pub const fn gpiodouttgl(&self) -> &Gpiodouttgl {
        &self.gpiodouttgl
    }
    #[doc = "0x18 - General Purpose Input Output Digital Input Enable This register controls input buffers for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and I = 1 for AUX_AIODIO1."]
    #[inline(always)]
    pub const fn gpiodie(&self) -> &Gpiodie {
        &self.gpiodie
    }
}
#[doc = "GPIODOUT (rw) register accessor: General Purpose Input Output Data Out The output data register is used to set data on AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiodout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiodout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiodout`]
module"]
#[doc(alias = "GPIODOUT")]
pub type Gpiodout = crate::Reg<gpiodout::GpiodoutSpec>;
#[doc = "General Purpose Input Output Data Out The output data register is used to set data on AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1"]
pub mod gpiodout;
#[doc = "IOMODE (rw) register accessor: Input Output Mode This register controls pull-up, pull-down, and output mode for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iomode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iomode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iomode`]
module"]
#[doc(alias = "IOMODE")]
pub type Iomode = crate::Reg<iomode::IomodeSpec>;
#[doc = "Input Output Mode This register controls pull-up, pull-down, and output mode for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1"]
pub mod iomode;
#[doc = "GPIODIN (rw) register accessor: General Purpose Input Output Data In This register provides synchronized input data for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and I = 1 for AUX_AIODIO1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiodin::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiodin::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiodin`]
module"]
#[doc(alias = "GPIODIN")]
pub type Gpiodin = crate::Reg<gpiodin::GpiodinSpec>;
#[doc = "General Purpose Input Output Data In This register provides synchronized input data for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and I = 1 for AUX_AIODIO1."]
pub mod gpiodin;
#[doc = "GPIODOUTSET (rw) register accessor: General Purpose Input Output Data Out Set Set bits in GPIODOUT in instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiodoutset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiodoutset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiodoutset`]
module"]
#[doc(alias = "GPIODOUTSET")]
pub type Gpiodoutset = crate::Reg<gpiodoutset::GpiodoutsetSpec>;
#[doc = "General Purpose Input Output Data Out Set Set bits in GPIODOUT in instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1."]
pub mod gpiodoutset;
#[doc = "GPIODOUTCLR (rw) register accessor: General Purpose Input Output Data Out Clear Clear bits in GPIODOUT instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiodoutclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiodoutclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiodoutclr`]
module"]
#[doc(alias = "GPIODOUTCLR")]
pub type Gpiodoutclr = crate::Reg<gpiodoutclr::GpiodoutclrSpec>;
#[doc = "General Purpose Input Output Data Out Clear Clear bits in GPIODOUT instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1."]
pub mod gpiodoutclr;
#[doc = "GPIODOUTTGL (rw) register accessor: General Purpose Input Output Data Out Toggle Toggle bits in GPIODOUT in instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiodouttgl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiodouttgl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiodouttgl`]
module"]
#[doc(alias = "GPIODOUTTGL")]
pub type Gpiodouttgl = crate::Reg<gpiodouttgl::GpiodouttglSpec>;
#[doc = "General Purpose Input Output Data Out Toggle Toggle bits in GPIODOUT in instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1."]
pub mod gpiodouttgl;
#[doc = "GPIODIE (rw) register accessor: General Purpose Input Output Digital Input Enable This register controls input buffers for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and I = 1 for AUX_AIODIO1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiodie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiodie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiodie`]
module"]
#[doc(alias = "GPIODIE")]
pub type Gpiodie = crate::Reg<gpiodie::GpiodieSpec>;
#[doc = "General Purpose Input Output Digital Input Enable This register controls input buffers for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and I = 1 for AUX_AIODIO1."]
pub mod gpiodie;
