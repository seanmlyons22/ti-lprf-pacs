#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dmach0ctl: Dmach0ctl,
    dmach0extaddr: Dmach0extaddr,
    _reserved2: [u8; 0x04],
    dmach0len: Dmach0len,
    _reserved3: [u8; 0x08],
    dmastat: Dmastat,
    dmaswreset: Dmaswreset,
    dmach1ctl: Dmach1ctl,
    dmach1extaddr: Dmach1extaddr,
    _reserved7: [u8; 0x04],
    dmach1len: Dmach1len,
    _reserved8: [u8; 0x48],
    dmabuscfg: Dmabuscfg,
    dmaporterr: Dmaporterr,
    _reserved10: [u8; 0x7c],
    dmahwver: Dmahwver,
    _reserved11: [u8; 0x0300],
    keywritearea: Keywritearea,
    keywrittenarea: Keywrittenarea,
    keysize: Keysize,
    keyreadarea: Keyreadarea,
    _reserved15: [u8; 0xf0],
    aeskey2: Aeskey2,
    _reserved16: [u8; 0x0c],
    aeskey3: Aeskey3,
    _reserved17: [u8; 0x2c],
    aesiv: Aesiv,
    _reserved18: [u8; 0x0c],
    aesctl: Aesctl,
    aesdatalen0: Aesdatalen0,
    aesdatalen1: Aesdatalen1,
    aesauthlen: Aesauthlen,
    _reserved_22_aesdatain0: [u8; 0x04],
    _reserved_23_aesdatain1: [u8; 0x04],
    _reserved_24_aesdatain2: [u8; 0x04],
    _reserved_25_aesdatain3: [u8; 0x04],
    aestagout: Aestagout,
    _reserved27: [u8; 0x018c],
    algsel: Algsel,
    dmaprotctl: Dmaprotctl,
    _reserved29: [u8; 0x38],
    swreset: Swreset,
    _reserved30: [u8; 0x3c],
    irqtype: Irqtype,
    irqen: Irqen,
    irqclr: Irqclr,
    irqset: Irqset,
    irqstat: Irqstat,
    _reserved35: [u8; 0x68],
    hwver: Hwver,
}
impl RegisterBlock {
    #[doc = "0x00 - DMA Channel 0 Control"]
    #[inline(always)]
    pub const fn dmach0ctl(&self) -> &Dmach0ctl {
        &self.dmach0ctl
    }
    #[doc = "0x04 - DMA Channel 0 External Address"]
    #[inline(always)]
    pub const fn dmach0extaddr(&self) -> &Dmach0extaddr {
        &self.dmach0extaddr
    }
    #[doc = "0x0c - DMA Channel 0 Length"]
    #[inline(always)]
    pub const fn dmach0len(&self) -> &Dmach0len {
        &self.dmach0len
    }
    #[doc = "0x18 - DMA Controller Status"]
    #[inline(always)]
    pub const fn dmastat(&self) -> &Dmastat {
        &self.dmastat
    }
    #[doc = "0x1c - DMA Controller Software Reset"]
    #[inline(always)]
    pub const fn dmaswreset(&self) -> &Dmaswreset {
        &self.dmaswreset
    }
    #[doc = "0x20 - DMA Channel 1 Control"]
    #[inline(always)]
    pub const fn dmach1ctl(&self) -> &Dmach1ctl {
        &self.dmach1ctl
    }
    #[doc = "0x24 - DMA Channel 1 External Address"]
    #[inline(always)]
    pub const fn dmach1extaddr(&self) -> &Dmach1extaddr {
        &self.dmach1extaddr
    }
    #[doc = "0x2c - DMA Channel 1 Length"]
    #[inline(always)]
    pub const fn dmach1len(&self) -> &Dmach1len {
        &self.dmach1len
    }
    #[doc = "0x78 - DMA Controller Master Configuration"]
    #[inline(always)]
    pub const fn dmabuscfg(&self) -> &Dmabuscfg {
        &self.dmabuscfg
    }
    #[doc = "0x7c - DMA Controller Port Error"]
    #[inline(always)]
    pub const fn dmaporterr(&self) -> &Dmaporterr {
        &self.dmaporterr
    }
    #[doc = "0xfc - DMA Controller Version"]
    #[inline(always)]
    pub const fn dmahwver(&self) -> &Dmahwver {
        &self.dmahwver
    }
    #[doc = "0x400 - Key Write Area"]
    #[inline(always)]
    pub const fn keywritearea(&self) -> &Keywritearea {
        &self.keywritearea
    }
    #[doc = "0x404 - Key Written Area Status This register shows which areas of the key store RAM contain valid written keys. When a new key needs to be written to the key store, on a location that is already occupied by a valid key, this key area must be cleared first. This can be done by writing this register before the new key is written to the key store memory. Attempting to write to a key area that already contains a valid key is not allowed and will result in an error."]
    #[inline(always)]
    pub const fn keywrittenarea(&self) -> &Keywrittenarea {
        &self.keywrittenarea
    }
    #[doc = "0x408 - Key Size This register defines the size of the keys that are written with DMA."]
    #[inline(always)]
    pub const fn keysize(&self) -> &Keysize {
        &self.keysize
    }
    #[doc = "0x40c - Key Read Area"]
    #[inline(always)]
    pub const fn keyreadarea(&self) -> &Keyreadarea {
        &self.keyreadarea
    }
    #[doc = "0x500 - Clear AES_KEY2/GHASH Key"]
    #[inline(always)]
    pub const fn aeskey2(&self) -> &Aeskey2 {
        &self.aeskey2
    }
    #[doc = "0x510 - Clear AES_KEY3"]
    #[inline(always)]
    pub const fn aeskey3(&self) -> &Aeskey3 {
        &self.aeskey3
    }
    #[doc = "0x540 - AES Initialization Vector"]
    #[inline(always)]
    pub const fn aesiv(&self) -> &Aesiv {
        &self.aesiv
    }
    #[doc = "0x550 - AES Input/Output Buffer Control"]
    #[inline(always)]
    pub const fn aesctl(&self) -> &Aesctl {
        &self.aesctl
    }
    #[doc = "0x554 - Crypto Data Length LSW"]
    #[inline(always)]
    pub const fn aesdatalen0(&self) -> &Aesdatalen0 {
        &self.aesdatalen0
    }
    #[doc = "0x558 - Crypto Data Length MSW"]
    #[inline(always)]
    pub const fn aesdatalen1(&self) -> &Aesdatalen1 {
        &self.aesdatalen1
    }
    #[doc = "0x55c - AES Authentication Length"]
    #[inline(always)]
    pub const fn aesauthlen(&self) -> &Aesauthlen {
        &self.aesauthlen
    }
    #[doc = "0x560 - AES Data Input/Output 0"]
    #[inline(always)]
    pub const fn aesdatain0(&self) -> &Aesdatain0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1376).cast() }
    }
    #[doc = "0x560 - Data Input/Output"]
    #[inline(always)]
    pub const fn aesdataout0(&self) -> &Aesdataout0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1376).cast() }
    }
    #[doc = "0x564 - AES Data Input/Output 1"]
    #[inline(always)]
    pub const fn aesdatain1(&self) -> &Aesdatain1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1380).cast() }
    }
    #[doc = "0x564 - AES Data Input/Output 3"]
    #[inline(always)]
    pub const fn aesdataout1(&self) -> &Aesdataout1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1380).cast() }
    }
    #[doc = "0x568 - AES Data Input/Output 2"]
    #[inline(always)]
    pub const fn aesdatain2(&self) -> &Aesdatain2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1384).cast() }
    }
    #[doc = "0x568 - AES Data Input/Output 2"]
    #[inline(always)]
    pub const fn aesdataout2(&self) -> &Aesdataout2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1384).cast() }
    }
    #[doc = "0x56c - Data Input/Output"]
    #[inline(always)]
    pub const fn aesdatain3(&self) -> &Aesdatain3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1388).cast() }
    }
    #[doc = "0x56c - AES Data Input/Output 3"]
    #[inline(always)]
    pub const fn aesdataout3(&self) -> &Aesdataout3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1388).cast() }
    }
    #[doc = "0x570 - AES Tag Output"]
    #[inline(always)]
    pub const fn aestagout(&self) -> &Aestagout {
        &self.aestagout
    }
    #[doc = "0x700 - Master Algorithm Select This register configures the internal destination of the DMA controller."]
    #[inline(always)]
    pub const fn algsel(&self) -> &Algsel {
        &self.algsel
    }
    #[doc = "0x704 - Master Protection Control"]
    #[inline(always)]
    pub const fn dmaprotctl(&self) -> &Dmaprotctl {
        &self.dmaprotctl
    }
    #[doc = "0x740 - Software Reset"]
    #[inline(always)]
    pub const fn swreset(&self) -> &Swreset {
        &self.swreset
    }
    #[doc = "0x780 - Control Interrupt Configuration"]
    #[inline(always)]
    pub const fn irqtype(&self) -> &Irqtype {
        &self.irqtype
    }
    #[doc = "0x784 - Interrupt Enable"]
    #[inline(always)]
    pub const fn irqen(&self) -> &Irqen {
        &self.irqen
    }
    #[doc = "0x788 - Interrupt Clear"]
    #[inline(always)]
    pub const fn irqclr(&self) -> &Irqclr {
        &self.irqclr
    }
    #[doc = "0x78c - Interrupt Set"]
    #[inline(always)]
    pub const fn irqset(&self) -> &Irqset {
        &self.irqset
    }
    #[doc = "0x790 - Interrupt Status"]
    #[inline(always)]
    pub const fn irqstat(&self) -> &Irqstat {
        &self.irqstat
    }
    #[doc = "0x7fc - CTRL Module Version"]
    #[inline(always)]
    pub const fn hwver(&self) -> &Hwver {
        &self.hwver
    }
}
#[doc = "DMACH0CTL (rw) register accessor: DMA Channel 0 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmach0ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmach0ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmach0ctl`]
module"]
#[doc(alias = "DMACH0CTL")]
pub type Dmach0ctl = crate::Reg<dmach0ctl::Dmach0ctlSpec>;
#[doc = "DMA Channel 0 Control"]
pub mod dmach0ctl;
#[doc = "DMACH0EXTADDR (rw) register accessor: DMA Channel 0 External Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmach0extaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmach0extaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmach0extaddr`]
module"]
#[doc(alias = "DMACH0EXTADDR")]
pub type Dmach0extaddr = crate::Reg<dmach0extaddr::Dmach0extaddrSpec>;
#[doc = "DMA Channel 0 External Address"]
pub mod dmach0extaddr;
#[doc = "DMACH0LEN (rw) register accessor: DMA Channel 0 Length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmach0len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmach0len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmach0len`]
module"]
#[doc(alias = "DMACH0LEN")]
pub type Dmach0len = crate::Reg<dmach0len::Dmach0lenSpec>;
#[doc = "DMA Channel 0 Length"]
pub mod dmach0len;
#[doc = "DMASTAT (rw) register accessor: DMA Controller Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmastat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmastat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmastat`]
module"]
#[doc(alias = "DMASTAT")]
pub type Dmastat = crate::Reg<dmastat::DmastatSpec>;
#[doc = "DMA Controller Status"]
pub mod dmastat;
#[doc = "DMASWRESET (rw) register accessor: DMA Controller Software Reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaswreset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaswreset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaswreset`]
module"]
#[doc(alias = "DMASWRESET")]
pub type Dmaswreset = crate::Reg<dmaswreset::DmaswresetSpec>;
#[doc = "DMA Controller Software Reset"]
pub mod dmaswreset;
#[doc = "DMACH1CTL (rw) register accessor: DMA Channel 1 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmach1ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmach1ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmach1ctl`]
module"]
#[doc(alias = "DMACH1CTL")]
pub type Dmach1ctl = crate::Reg<dmach1ctl::Dmach1ctlSpec>;
#[doc = "DMA Channel 1 Control"]
pub mod dmach1ctl;
#[doc = "DMACH1EXTADDR (rw) register accessor: DMA Channel 1 External Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmach1extaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmach1extaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmach1extaddr`]
module"]
#[doc(alias = "DMACH1EXTADDR")]
pub type Dmach1extaddr = crate::Reg<dmach1extaddr::Dmach1extaddrSpec>;
#[doc = "DMA Channel 1 External Address"]
pub mod dmach1extaddr;
#[doc = "DMACH1LEN (rw) register accessor: DMA Channel 1 Length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmach1len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmach1len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmach1len`]
module"]
#[doc(alias = "DMACH1LEN")]
pub type Dmach1len = crate::Reg<dmach1len::Dmach1lenSpec>;
#[doc = "DMA Channel 1 Length"]
pub mod dmach1len;
#[doc = "DMABUSCFG (rw) register accessor: DMA Controller Master Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmabuscfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmabuscfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmabuscfg`]
module"]
#[doc(alias = "DMABUSCFG")]
pub type Dmabuscfg = crate::Reg<dmabuscfg::DmabuscfgSpec>;
#[doc = "DMA Controller Master Configuration"]
pub mod dmabuscfg;
#[doc = "DMAPORTERR (rw) register accessor: DMA Controller Port Error\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaporterr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaporterr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaporterr`]
module"]
#[doc(alias = "DMAPORTERR")]
pub type Dmaporterr = crate::Reg<dmaporterr::DmaporterrSpec>;
#[doc = "DMA Controller Port Error"]
pub mod dmaporterr;
#[doc = "DMAHWVER (rw) register accessor: DMA Controller Version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmahwver::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmahwver::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmahwver`]
module"]
#[doc(alias = "DMAHWVER")]
pub type Dmahwver = crate::Reg<dmahwver::DmahwverSpec>;
#[doc = "DMA Controller Version"]
pub mod dmahwver;
#[doc = "KEYWRITEAREA (rw) register accessor: Key Write Area\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keywritearea::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keywritearea::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keywritearea`]
module"]
#[doc(alias = "KEYWRITEAREA")]
pub type Keywritearea = crate::Reg<keywritearea::KeywriteareaSpec>;
#[doc = "Key Write Area"]
pub mod keywritearea;
#[doc = "KEYWRITTENAREA (rw) register accessor: Key Written Area Status This register shows which areas of the key store RAM contain valid written keys. When a new key needs to be written to the key store, on a location that is already occupied by a valid key, this key area must be cleared first. This can be done by writing this register before the new key is written to the key store memory. Attempting to write to a key area that already contains a valid key is not allowed and will result in an error.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keywrittenarea::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keywrittenarea::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keywrittenarea`]
module"]
#[doc(alias = "KEYWRITTENAREA")]
pub type Keywrittenarea = crate::Reg<keywrittenarea::KeywrittenareaSpec>;
#[doc = "Key Written Area Status This register shows which areas of the key store RAM contain valid written keys. When a new key needs to be written to the key store, on a location that is already occupied by a valid key, this key area must be cleared first. This can be done by writing this register before the new key is written to the key store memory. Attempting to write to a key area that already contains a valid key is not allowed and will result in an error."]
pub mod keywrittenarea;
#[doc = "KEYSIZE (rw) register accessor: Key Size This register defines the size of the keys that are written with DMA.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keysize::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keysize::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keysize`]
module"]
#[doc(alias = "KEYSIZE")]
pub type Keysize = crate::Reg<keysize::KeysizeSpec>;
#[doc = "Key Size This register defines the size of the keys that are written with DMA."]
pub mod keysize;
#[doc = "KEYREADAREA (rw) register accessor: Key Read Area\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyreadarea::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyreadarea::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyreadarea`]
module"]
#[doc(alias = "KEYREADAREA")]
pub type Keyreadarea = crate::Reg<keyreadarea::KeyreadareaSpec>;
#[doc = "Key Read Area"]
pub mod keyreadarea;
#[doc = "AESKEY2 (rw) register accessor: Clear AES_KEY2/GHASH Key\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aeskey2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aeskey2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aeskey2`]
module"]
#[doc(alias = "AESKEY2")]
pub type Aeskey2 = crate::Reg<aeskey2::Aeskey2Spec>;
#[doc = "Clear AES_KEY2/GHASH Key"]
pub mod aeskey2;
#[doc = "AESKEY3 (rw) register accessor: Clear AES_KEY3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aeskey3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aeskey3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aeskey3`]
module"]
#[doc(alias = "AESKEY3")]
pub type Aeskey3 = crate::Reg<aeskey3::Aeskey3Spec>;
#[doc = "Clear AES_KEY3"]
pub mod aeskey3;
#[doc = "AESIV (rw) register accessor: AES Initialization Vector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesiv`]
module"]
#[doc(alias = "AESIV")]
pub type Aesiv = crate::Reg<aesiv::AesivSpec>;
#[doc = "AES Initialization Vector"]
pub mod aesiv;
#[doc = "AESCTL (rw) register accessor: AES Input/Output Buffer Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesctl`]
module"]
#[doc(alias = "AESCTL")]
pub type Aesctl = crate::Reg<aesctl::AesctlSpec>;
#[doc = "AES Input/Output Buffer Control"]
pub mod aesctl;
#[doc = "AESDATALEN0 (rw) register accessor: Crypto Data Length LSW\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesdatalen0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesdatalen0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesdatalen0`]
module"]
#[doc(alias = "AESDATALEN0")]
pub type Aesdatalen0 = crate::Reg<aesdatalen0::Aesdatalen0Spec>;
#[doc = "Crypto Data Length LSW"]
pub mod aesdatalen0;
#[doc = "AESDATALEN1 (rw) register accessor: Crypto Data Length MSW\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesdatalen1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesdatalen1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesdatalen1`]
module"]
#[doc(alias = "AESDATALEN1")]
pub type Aesdatalen1 = crate::Reg<aesdatalen1::Aesdatalen1Spec>;
#[doc = "Crypto Data Length MSW"]
pub mod aesdatalen1;
#[doc = "AESAUTHLEN (rw) register accessor: AES Authentication Length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesauthlen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesauthlen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesauthlen`]
module"]
#[doc(alias = "AESAUTHLEN")]
pub type Aesauthlen = crate::Reg<aesauthlen::AesauthlenSpec>;
#[doc = "AES Authentication Length"]
pub mod aesauthlen;
#[doc = "AESDATAOUT0 (rw) register accessor: Data Input/Output\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesdataout0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesdataout0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesdataout0`]
module"]
#[doc(alias = "AESDATAOUT0")]
pub type Aesdataout0 = crate::Reg<aesdataout0::Aesdataout0Spec>;
#[doc = "Data Input/Output"]
pub mod aesdataout0;
#[doc = "AESDATAIN0 (rw) register accessor: AES Data Input/Output 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesdatain0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesdatain0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesdatain0`]
module"]
#[doc(alias = "AESDATAIN0")]
pub type Aesdatain0 = crate::Reg<aesdatain0::Aesdatain0Spec>;
#[doc = "AES Data Input/Output 0"]
pub mod aesdatain0;
#[doc = "AESDATAOUT1 (rw) register accessor: AES Data Input/Output 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesdataout1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesdataout1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesdataout1`]
module"]
#[doc(alias = "AESDATAOUT1")]
pub type Aesdataout1 = crate::Reg<aesdataout1::Aesdataout1Spec>;
#[doc = "AES Data Input/Output 3"]
pub mod aesdataout1;
#[doc = "AESDATAIN1 (rw) register accessor: AES Data Input/Output 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesdatain1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesdatain1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesdatain1`]
module"]
#[doc(alias = "AESDATAIN1")]
pub type Aesdatain1 = crate::Reg<aesdatain1::Aesdatain1Spec>;
#[doc = "AES Data Input/Output 1"]
pub mod aesdatain1;
#[doc = "AESDATAOUT2 (rw) register accessor: AES Data Input/Output 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesdataout2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesdataout2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesdataout2`]
module"]
#[doc(alias = "AESDATAOUT2")]
pub type Aesdataout2 = crate::Reg<aesdataout2::Aesdataout2Spec>;
#[doc = "AES Data Input/Output 2"]
pub mod aesdataout2;
#[doc = "AESDATAIN2 (rw) register accessor: AES Data Input/Output 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesdatain2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesdatain2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesdatain2`]
module"]
#[doc(alias = "AESDATAIN2")]
pub type Aesdatain2 = crate::Reg<aesdatain2::Aesdatain2Spec>;
#[doc = "AES Data Input/Output 2"]
pub mod aesdatain2;
#[doc = "AESDATAOUT3 (rw) register accessor: AES Data Input/Output 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesdataout3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesdataout3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesdataout3`]
module"]
#[doc(alias = "AESDATAOUT3")]
pub type Aesdataout3 = crate::Reg<aesdataout3::Aesdataout3Spec>;
#[doc = "AES Data Input/Output 3"]
pub mod aesdataout3;
#[doc = "AESDATAIN3 (rw) register accessor: Data Input/Output\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesdatain3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesdatain3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesdatain3`]
module"]
#[doc(alias = "AESDATAIN3")]
pub type Aesdatain3 = crate::Reg<aesdatain3::Aesdatain3Spec>;
#[doc = "Data Input/Output"]
pub mod aesdatain3;
#[doc = "AESTAGOUT (rw) register accessor: AES Tag Output\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aestagout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aestagout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aestagout`]
module"]
#[doc(alias = "AESTAGOUT")]
pub type Aestagout = crate::Reg<aestagout::AestagoutSpec>;
#[doc = "AES Tag Output"]
pub mod aestagout;
#[doc = "ALGSEL (rw) register accessor: Master Algorithm Select This register configures the internal destination of the DMA controller.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`algsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`algsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@algsel`]
module"]
#[doc(alias = "ALGSEL")]
pub type Algsel = crate::Reg<algsel::AlgselSpec>;
#[doc = "Master Algorithm Select This register configures the internal destination of the DMA controller."]
pub mod algsel;
#[doc = "DMAPROTCTL (rw) register accessor: Master Protection Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaprotctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaprotctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaprotctl`]
module"]
#[doc(alias = "DMAPROTCTL")]
pub type Dmaprotctl = crate::Reg<dmaprotctl::DmaprotctlSpec>;
#[doc = "Master Protection Control"]
pub mod dmaprotctl;
#[doc = "SWRESET (rw) register accessor: Software Reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreset`]
module"]
#[doc(alias = "SWRESET")]
pub type Swreset = crate::Reg<swreset::SwresetSpec>;
#[doc = "Software Reset"]
pub mod swreset;
#[doc = "IRQTYPE (rw) register accessor: Control Interrupt Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqtype::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqtype::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqtype`]
module"]
#[doc(alias = "IRQTYPE")]
pub type Irqtype = crate::Reg<irqtype::IrqtypeSpec>;
#[doc = "Control Interrupt Configuration"]
pub mod irqtype;
#[doc = "IRQEN (rw) register accessor: Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqen`]
module"]
#[doc(alias = "IRQEN")]
pub type Irqen = crate::Reg<irqen::IrqenSpec>;
#[doc = "Interrupt Enable"]
pub mod irqen;
#[doc = "IRQCLR (rw) register accessor: Interrupt Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqclr`]
module"]
#[doc(alias = "IRQCLR")]
pub type Irqclr = crate::Reg<irqclr::IrqclrSpec>;
#[doc = "Interrupt Clear"]
pub mod irqclr;
#[doc = "IRQSET (rw) register accessor: Interrupt Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqset`]
module"]
#[doc(alias = "IRQSET")]
pub type Irqset = crate::Reg<irqset::IrqsetSpec>;
#[doc = "Interrupt Set"]
pub mod irqset;
#[doc = "IRQSTAT (rw) register accessor: Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqstat`]
module"]
#[doc(alias = "IRQSTAT")]
pub type Irqstat = crate::Reg<irqstat::IrqstatSpec>;
#[doc = "Interrupt Status"]
pub mod irqstat;
#[doc = "HWVER (rw) register accessor: CTRL Module Version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwver::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwver::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwver`]
module"]
#[doc(alias = "HWVER")]
pub type Hwver = crate::Reg<hwver::HwverSpec>;
#[doc = "CTRL Module Version"]
pub mod hwver;
