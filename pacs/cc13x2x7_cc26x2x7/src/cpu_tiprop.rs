#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    reserved000: Reserved000,
    _reserved1: [u8; 0x0ff4],
    traceclkmux: Traceclkmux,
}
impl RegisterBlock {
    #[doc = "0x00 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn reserved000(&self) -> &Reserved000 {
        &self.reserved000
    }
    #[doc = "0xff8 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn traceclkmux(&self) -> &Traceclkmux {
        &self.traceclkmux
    }
}
#[doc = "RESERVED000 (rw) register accessor: Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reserved000::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reserved000::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved000`]
module"]
#[doc(alias = "RESERVED000")]
pub type Reserved000 = crate::Reg<reserved000::Reserved000Spec>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved000;
#[doc = "TRACECLKMUX (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`traceclkmux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`traceclkmux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@traceclkmux`]
module"]
#[doc(alias = "TRACECLKMUX")]
pub type Traceclkmux = crate::Reg<traceclkmux::TraceclkmuxSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod traceclkmux;
