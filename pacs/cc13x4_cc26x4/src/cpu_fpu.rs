#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    fpccr: Fpccr,
    fpcar: Fpcar,
    fpdscr: Fpdscr,
    mvfr0: Mvfr0,
    mvfr1: Mvfr1,
    mvfr2: Mvfr2,
}
impl RegisterBlock {
    #[doc = "0x04 - Holds control data for the Floating-point extension"]
    #[inline(always)]
    pub const fn fpccr(&self) -> &Fpccr {
        &self.fpccr
    }
    #[doc = "0x08 - Holds the location of the unpopulated floating-point register space allocated on an exception stack frame"]
    #[inline(always)]
    pub const fn fpcar(&self) -> &Fpcar {
        &self.fpcar
    }
    #[doc = "0x0c - Holds the default values for the floating-point status control data that the PE assigns to the FPSCR when it creates a new floating-point context"]
    #[inline(always)]
    pub const fn fpdscr(&self) -> &Fpdscr {
        &self.fpdscr
    }
    #[doc = "0x10 - Describes the features provided by the Floating-point Extension"]
    #[inline(always)]
    pub const fn mvfr0(&self) -> &Mvfr0 {
        &self.mvfr0
    }
    #[doc = "0x14 - Describes the features provided by the Floating-point Extension"]
    #[inline(always)]
    pub const fn mvfr1(&self) -> &Mvfr1 {
        &self.mvfr1
    }
    #[doc = "0x18 - Describes the features provided by the Floating-point Extension"]
    #[inline(always)]
    pub const fn mvfr2(&self) -> &Mvfr2 {
        &self.mvfr2
    }
}
#[doc = "FPCCR (rw) register accessor: Holds control data for the Floating-point extension\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpccr`]
module"]
#[doc(alias = "FPCCR")]
pub type Fpccr = crate::Reg<fpccr::FpccrSpec>;
#[doc = "Holds control data for the Floating-point extension"]
pub mod fpccr;
#[doc = "FPCAR (rw) register accessor: Holds the location of the unpopulated floating-point register space allocated on an exception stack frame\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpcar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpcar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpcar`]
module"]
#[doc(alias = "FPCAR")]
pub type Fpcar = crate::Reg<fpcar::FpcarSpec>;
#[doc = "Holds the location of the unpopulated floating-point register space allocated on an exception stack frame"]
pub mod fpcar;
#[doc = "FPDSCR (rw) register accessor: Holds the default values for the floating-point status control data that the PE assigns to the FPSCR when it creates a new floating-point context\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpdscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpdscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpdscr`]
module"]
#[doc(alias = "FPDSCR")]
pub type Fpdscr = crate::Reg<fpdscr::FpdscrSpec>;
#[doc = "Holds the default values for the floating-point status control data that the PE assigns to the FPSCR when it creates a new floating-point context"]
pub mod fpdscr;
#[doc = "MVFR0 (rw) register accessor: Describes the features provided by the Floating-point Extension\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mvfr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mvfr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mvfr0`]
module"]
#[doc(alias = "MVFR0")]
pub type Mvfr0 = crate::Reg<mvfr0::Mvfr0Spec>;
#[doc = "Describes the features provided by the Floating-point Extension"]
pub mod mvfr0;
#[doc = "MVFR1 (rw) register accessor: Describes the features provided by the Floating-point Extension\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mvfr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mvfr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mvfr1`]
module"]
#[doc(alias = "MVFR1")]
pub type Mvfr1 = crate::Reg<mvfr1::Mvfr1Spec>;
#[doc = "Describes the features provided by the Floating-point Extension"]
pub mod mvfr1;
#[doc = "MVFR2 (rw) register accessor: Describes the features provided by the Floating-point Extension\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mvfr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mvfr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mvfr2`]
module"]
#[doc(alias = "MVFR2")]
pub type Mvfr2 = crate::Reg<mvfr2::Mvfr2Spec>;
#[doc = "Describes the features provided by the Floating-point Extension"]
pub mod mvfr2;
