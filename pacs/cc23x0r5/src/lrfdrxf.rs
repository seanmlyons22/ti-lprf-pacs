#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rxd: Rxd,
}
impl RegisterBlock {
    #[doc = "0x00 - RX FIFO data. When written the register data is pushed to the RX FIFO. When read, data is popped from the RX FIFO"]
    #[inline(always)]
    pub const fn rxd(&self) -> &Rxd {
        &self.rxd
    }
}
#[doc = "RXD (rw) register accessor: RX FIFO data. When written the register data is pushed to the RX FIFO. When read, data is popped from the RX FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxd`]
module"]
#[doc(alias = "RXD")]
pub type Rxd = crate::Reg<rxd::RxdSpec>;
#[doc = "RX FIFO data. When written the register data is pushed to the RX FIFO. When read, data is popped from the RX FIFO"]
pub mod rxd;
