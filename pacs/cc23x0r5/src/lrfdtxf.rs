#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    txd: Txd,
}
impl RegisterBlock {
    #[doc = "0x00 - TX FIFO data. When written the register data is pushed to the TX FIFO. When read, data is popped from the TX FIFO"]
    #[inline(always)]
    pub const fn txd(&self) -> &Txd {
        &self.txd
    }
}
#[doc = "TXD (rw) register accessor: TX FIFO data. When written the register data is pushed to the TX FIFO. When read, data is popped from the TX FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txd`]
module"]
#[doc(alias = "TXD")]
pub type Txd = crate::Reg<txd::TxdSpec>;
#[doc = "TX FIFO data. When written the register data is pushed to the TX FIFO. When read, data is popped from the TX FIFO"]
pub mod txd;
