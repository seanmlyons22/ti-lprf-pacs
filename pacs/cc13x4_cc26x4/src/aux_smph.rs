#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    smph0: Smph0,
    smph1: Smph1,
    smph2: Smph2,
    smph3: Smph3,
    smph4: Smph4,
    smph5: Smph5,
    smph6: Smph6,
    smph7: Smph7,
    autotake: Autotake,
}
impl RegisterBlock {
    #[doc = "0x00 - Semaphore 0"]
    #[inline(always)]
    pub const fn smph0(&self) -> &Smph0 {
        &self.smph0
    }
    #[doc = "0x04 - Semaphore 1"]
    #[inline(always)]
    pub const fn smph1(&self) -> &Smph1 {
        &self.smph1
    }
    #[doc = "0x08 - Semaphore 2"]
    #[inline(always)]
    pub const fn smph2(&self) -> &Smph2 {
        &self.smph2
    }
    #[doc = "0x0c - Semaphore 3"]
    #[inline(always)]
    pub const fn smph3(&self) -> &Smph3 {
        &self.smph3
    }
    #[doc = "0x10 - Semaphore 4"]
    #[inline(always)]
    pub const fn smph4(&self) -> &Smph4 {
        &self.smph4
    }
    #[doc = "0x14 - Semaphore 5"]
    #[inline(always)]
    pub const fn smph5(&self) -> &Smph5 {
        &self.smph5
    }
    #[doc = "0x18 - Semaphore 6"]
    #[inline(always)]
    pub const fn smph6(&self) -> &Smph6 {
        &self.smph6
    }
    #[doc = "0x1c - Semaphore 7"]
    #[inline(always)]
    pub const fn smph7(&self) -> &Smph7 {
        &self.smph7
    }
    #[doc = "0x20 - Auto Take Sticky Request for Single Semaphore."]
    #[inline(always)]
    pub const fn autotake(&self) -> &Autotake {
        &self.autotake
    }
}
#[doc = "SMPH0 (rw) register accessor: Semaphore 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph0`]
module"]
#[doc(alias = "SMPH0")]
pub type Smph0 = crate::Reg<smph0::Smph0Spec>;
#[doc = "Semaphore 0"]
pub mod smph0;
#[doc = "SMPH1 (rw) register accessor: Semaphore 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph1`]
module"]
#[doc(alias = "SMPH1")]
pub type Smph1 = crate::Reg<smph1::Smph1Spec>;
#[doc = "Semaphore 1"]
pub mod smph1;
#[doc = "SMPH2 (rw) register accessor: Semaphore 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph2`]
module"]
#[doc(alias = "SMPH2")]
pub type Smph2 = crate::Reg<smph2::Smph2Spec>;
#[doc = "Semaphore 2"]
pub mod smph2;
#[doc = "SMPH3 (rw) register accessor: Semaphore 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph3`]
module"]
#[doc(alias = "SMPH3")]
pub type Smph3 = crate::Reg<smph3::Smph3Spec>;
#[doc = "Semaphore 3"]
pub mod smph3;
#[doc = "SMPH4 (rw) register accessor: Semaphore 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph4`]
module"]
#[doc(alias = "SMPH4")]
pub type Smph4 = crate::Reg<smph4::Smph4Spec>;
#[doc = "Semaphore 4"]
pub mod smph4;
#[doc = "SMPH5 (rw) register accessor: Semaphore 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph5`]
module"]
#[doc(alias = "SMPH5")]
pub type Smph5 = crate::Reg<smph5::Smph5Spec>;
#[doc = "Semaphore 5"]
pub mod smph5;
#[doc = "SMPH6 (rw) register accessor: Semaphore 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph6`]
module"]
#[doc(alias = "SMPH6")]
pub type Smph6 = crate::Reg<smph6::Smph6Spec>;
#[doc = "Semaphore 6"]
pub mod smph6;
#[doc = "SMPH7 (rw) register accessor: Semaphore 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph7`]
module"]
#[doc(alias = "SMPH7")]
pub type Smph7 = crate::Reg<smph7::Smph7Spec>;
#[doc = "Semaphore 7"]
pub mod smph7;
#[doc = "AUTOTAKE (rw) register accessor: Auto Take Sticky Request for Single Semaphore.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`autotake::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`autotake::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@autotake`]
module"]
#[doc(alias = "AUTOTAKE")]
pub type Autotake = crate::Reg<autotake::AutotakeSpec>;
#[doc = "Auto Take Sticky Request for Single Semaphore."]
pub mod autotake;
