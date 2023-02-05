#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    desc: Desc,
    _reserved1: [u8; 0x0c],
    trg: Trg,
    abort: Abort,
    clr: Clr,
    sta: Sta,
    dma: Dma,
    dmacha: Dmacha,
    dmachb: Dmachb,
    autocfg: Autocfg,
    _reserved9: [u8; 0x20],
    key0: Key0,
    key1: Key1,
    key2: Key2,
    key3: Key3,
    _reserved13: [u8; 0x10],
    txt0: Txt0,
    txt1: Txt1,
    txt2: Txt2,
    txt3: Txt3,
    txtx0: Txtx0,
    txtx1: Txtx1,
    txtx2: Txtx2,
    txtx3: Txtx3,
    buf0: Buf0,
    buf1: Buf1,
    buf2: Buf2,
    buf3: Buf3,
    txtxbuf0: Txtxbuf0,
    txtxbuf1: Txtxbuf1,
    txtxbuf2: Txtxbuf2,
    txtxbuf3: Txtxbuf3,
    _reserved29: [u8; 0x54],
    imask: Imask,
    ris: Ris,
    mis: Mis,
    iset: Iset,
    iclr: Iclr,
    imset: Imset,
    imclr: Imclr,
}
impl RegisterBlock {
    #[doc = "0x00 - Description Register. This register provides IP module ID, revision information, instance index and standard MMR registers offset."]
    #[inline(always)]
    pub const fn desc(&self) -> &Desc {
        &self.desc
    }
    #[doc = "0x10 - Trigger Use this register to manually trigger operations."]
    #[inline(always)]
    pub const fn trg(&self) -> &Trg {
        &self.trg
    }
    #[doc = "0x14 - Abort Use this register to abort current ECB encryption."]
    #[inline(always)]
    pub const fn abort(&self) -> &Abort {
        &self.abort
    }
    #[doc = "0x18 - Clear Use this register to clear contents of TXT and BUF when STA.STATE = IDLE. If condition is not met, the contents remain unchanged."]
    #[inline(always)]
    pub const fn clr(&self) -> &Clr {
        &self.clr
    }
    #[doc = "0x1c - Status This register provides information on ECB accellerator state and BUF status."]
    #[inline(always)]
    pub const fn sta(&self) -> &Sta {
        &self.sta
    }
    #[doc = "0x20 - Direct Memory Access This register controls the conditions that will generate burst requests on each DMA channel."]
    #[inline(always)]
    pub const fn dma(&self) -> &Dma {
        &self.dma
    }
    #[doc = "0x24 - DMA Channel A data transfer DMA accesses this register to read or write contents from sequential addresses specifed by DMA.ADRCHA."]
    #[inline(always)]
    pub const fn dmacha(&self) -> &Dmacha {
        &self.dmacha
    }
    #[doc = "0x28 - DMA Channel B data transfer DMA accesses this register to read or write contents from sequential addresses specifed by DMA.ADRCHB."]
    #[inline(always)]
    pub const fn dmachb(&self) -> &Dmachb {
        &self.dmachb
    }
    #[doc = "0x2c - Automatic Configuration This register configures automatic hardware updates to TXT and BUF. Configure this register to reduce software overhead during cipher modes."]
    #[inline(always)]
    pub const fn autocfg(&self) -> &Autocfg {
        &self.autocfg
    }
    #[doc = "0x50 - Key Word 0 Write KEY0 through KEY3 to populate the 128-bit key. The key is not consumed by the hardware. It is hence not required to reload the key for subsequent block encryptions unless required by the application."]
    #[inline(always)]
    pub const fn key0(&self) -> &Key0 {
        &self.key0
    }
    #[doc = "0x54 - Key Word 1 Write KEY0 through KEY3 to populate the 128-bit key. The key is not consumed by the hardware. It is hence not required to reload the key for subsequent block encryptions unless required by the application."]
    #[inline(always)]
    pub const fn key1(&self) -> &Key1 {
        &self.key1
    }
    #[doc = "0x58 - Key Word 2 Write KEY0 through KEY3 to populate the 128-bit key. The key is not consumed by the hardware. It is hence not required to reload the key for subsequent block encryptions unless required by the application."]
    #[inline(always)]
    pub const fn key2(&self) -> &Key2 {
        &self.key2
    }
    #[doc = "0x5c - Key Word 3 Write KEY0 through KEY3 to populate the 128-bit key. The key is not consumed by the hardware. It is hence not required to reload the key for subsequent block encryptions unless required by the application."]
    #[inline(always)]
    pub const fn key3(&self) -> &Key3 {
        &self.key3
    }
    #[doc = "0x70 - Text Word 0 TXT is the 128-bit buffer, the AES-128 encryption algorithm performs its operations on. ECB input can be written to TXT, and ciphertext can be read from TXT."]
    #[inline(always)]
    pub const fn txt0(&self) -> &Txt0 {
        &self.txt0
    }
    #[doc = "0x74 - Text Word 1 TXT is the 128-bit buffer, the AES-128 encryption algorithm performs its operations on. ECB input can be written to TXT, and ciphertext can be read from TXT."]
    #[inline(always)]
    pub const fn txt1(&self) -> &Txt1 {
        &self.txt1
    }
    #[doc = "0x78 - Text Word 2 TXT is the 128-bit buffer, the AES-128 encryption algorithm performs its operations on. ECB input can be written to TXT, and ciphertext can be read from TXT."]
    #[inline(always)]
    pub const fn txt2(&self) -> &Txt2 {
        &self.txt2
    }
    #[doc = "0x7c - Text Word 3 TXT is the 128-bit buffer, the AES-128 encryption algorithm performs its operations on. ECB input can be written to TXT, and ciphertext can be read from TXT."]
    #[inline(always)]
    pub const fn txt3(&self) -> &Txt3 {
        &self.txt3
    }
    #[doc = "0x80 - Text Word 0 XOR Write data to this register to XOR data with contents in TXT0.VAL."]
    #[inline(always)]
    pub const fn txtx0(&self) -> &Txtx0 {
        &self.txtx0
    }
    #[doc = "0x84 - Text Word 1 XOR Write data to this register to XOR data with contents in TXT1.VAL."]
    #[inline(always)]
    pub const fn txtx1(&self) -> &Txtx1 {
        &self.txtx1
    }
    #[doc = "0x88 - Text Word 2 XOR Write data to this register to XOR data with contents in TXT2.VAL."]
    #[inline(always)]
    pub const fn txtx2(&self) -> &Txtx2 {
        &self.txtx2
    }
    #[doc = "0x8c - Text Word 3 XOR Write data to this register to XOR data with contents in TXT3.VAL. AUTOCFG.TRGECB decides if a write to or a read of this field triggers an encryption."]
    #[inline(always)]
    pub const fn txtx3(&self) -> &Txtx3 {
        &self.txtx3
    }
    #[doc = "0x90 - Buffer Word 0 BUF is a 128-bit auxiliary register that functions as a buffer, counter, or storage of operations in cipher modes."]
    #[inline(always)]
    pub const fn buf0(&self) -> &Buf0 {
        &self.buf0
    }
    #[doc = "0x94 - Buffer Word 1 BUF is a 128-bit auxiliary register that functions as a buffer, counter, or storage of operations in cipher modes."]
    #[inline(always)]
    pub const fn buf1(&self) -> &Buf1 {
        &self.buf1
    }
    #[doc = "0x98 - Buffer Word 2 BUF is a 128-bit auxiliary register that functions as a buffer, counter, or storage of operations in cipher modes."]
    #[inline(always)]
    pub const fn buf2(&self) -> &Buf2 {
        &self.buf2
    }
    #[doc = "0x9c - Buffer Word 3 BUF is a 128-bit auxiliary register that functions as a buffer, counter, or storage of operations in cipher modes. AUTOCFG.TRGECB decides if a write to this field triggers an encryption."]
    #[inline(always)]
    pub const fn buf3(&self) -> &Buf3 {
        &self.buf3
    }
    #[doc = "0xa0 - Text Word 0 XOR Buffer Word 0 Read this register to obtain plaintext during CFB decryption."]
    #[inline(always)]
    pub const fn txtxbuf0(&self) -> &Txtxbuf0 {
        &self.txtxbuf0
    }
    #[doc = "0xa4 - Text Word 1 XOR Buffer Word 1 Read this register to obtain plaintext during CFB decryption."]
    #[inline(always)]
    pub const fn txtxbuf1(&self) -> &Txtxbuf1 {
        &self.txtxbuf1
    }
    #[doc = "0xa8 - Text Word 2 XOR Buffer Word 2 Read this register to obtain plaintext during CFB decryption."]
    #[inline(always)]
    pub const fn txtxbuf2(&self) -> &Txtxbuf2 {
        &self.txtxbuf2
    }
    #[doc = "0xac - Text Word 3 XOR Buffer Word3 Read this register to obtain plaintext during CFB decryption."]
    #[inline(always)]
    pub const fn txtxbuf3(&self) -> &Txtxbuf3 {
        &self.txtxbuf3
    }
    #[doc = "0x104 - Interrupt mask. This register selects interrupt sources which are allowed to pass from RIS to MIS when the corresponding bit-fields are set to 1."]
    #[inline(always)]
    pub const fn imask(&self) -> &Imask {
        &self.imask
    }
    #[doc = "0x108 - Raw interrupt status. This register reflects the state of all pending interrupts, regardless of masking. This register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit."]
    #[inline(always)]
    pub const fn ris(&self) -> &Ris {
        &self.ris
    }
    #[doc = "0x10c - Masked interrupt status. This register is simply a bitwise AND of the contents of IMASK and RIS.*\\]
registers. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit."]
    #[inline(always)]
    pub const fn mis(&self) -> &Mis {
        &self.mis
    }
    #[doc = "0x110 - Interrupt set register. This register can used by software for diagnostics and safety checking purposes. Writing a 1 to a bit in this register will set the event and the corresponding RIS bit also gets set. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets set."]
    #[inline(always)]
    pub const fn iset(&self) -> &Iset {
        &self.iset
    }
    #[doc = "0x114 - Interrupt clear register. This register allows software to clear interrupts. Writing a 1 to a bit in this register will clear the event and the corresponding RIS bit also gets cleared. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets cleared."]
    #[inline(always)]
    pub const fn iclr(&self) -> &Iclr {
        &self.iclr
    }
    #[doc = "0x118 - Interrupt mask set register. Writing a 1 to a bit in this register will set the corresponding IMASK bit."]
    #[inline(always)]
    pub const fn imset(&self) -> &Imset {
        &self.imset
    }
    #[doc = "0x11c - Interrupt mask clear register. Writing a 1 to a bit in this register will clear the corresponding IMASK bit."]
    #[inline(always)]
    pub const fn imclr(&self) -> &Imclr {
        &self.imclr
    }
}
#[doc = "DESC (rw) register accessor: Description Register. This register provides IP module ID, revision information, instance index and standard MMR registers offset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`desc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`desc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@desc`]
module"]
#[doc(alias = "DESC")]
pub type Desc = crate::Reg<desc::DescSpec>;
#[doc = "Description Register. This register provides IP module ID, revision information, instance index and standard MMR registers offset."]
pub mod desc;
#[doc = "TRG (rw) register accessor: Trigger Use this register to manually trigger operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trg`]
module"]
#[doc(alias = "TRG")]
pub type Trg = crate::Reg<trg::TrgSpec>;
#[doc = "Trigger Use this register to manually trigger operations."]
pub mod trg;
#[doc = "ABORT (rw) register accessor: Abort Use this register to abort current ECB encryption.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`abort::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`abort::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@abort`]
module"]
#[doc(alias = "ABORT")]
pub type Abort = crate::Reg<abort::AbortSpec>;
#[doc = "Abort Use this register to abort current ECB encryption."]
pub mod abort;
#[doc = "CLR (rw) register accessor: Clear Use this register to clear contents of TXT and BUF when STA.STATE = IDLE. If condition is not met, the contents remain unchanged.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr`]
module"]
#[doc(alias = "CLR")]
pub type Clr = crate::Reg<clr::ClrSpec>;
#[doc = "Clear Use this register to clear contents of TXT and BUF when STA.STATE = IDLE. If condition is not met, the contents remain unchanged."]
pub mod clr;
#[doc = "STA (rw) register accessor: Status This register provides information on ECB accellerator state and BUF status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sta`]
module"]
#[doc(alias = "STA")]
pub type Sta = crate::Reg<sta::StaSpec>;
#[doc = "Status This register provides information on ECB accellerator state and BUF status."]
pub mod sta;
#[doc = "DMA (rw) register accessor: Direct Memory Access This register controls the conditions that will generate burst requests on each DMA channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma`]
module"]
#[doc(alias = "DMA")]
pub type Dma = crate::Reg<dma::DmaSpec>;
#[doc = "Direct Memory Access This register controls the conditions that will generate burst requests on each DMA channel."]
pub mod dma;
#[doc = "DMACHA (rw) register accessor: DMA Channel A data transfer DMA accesses this register to read or write contents from sequential addresses specifed by DMA.ADRCHA.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacha::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacha::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacha`]
module"]
#[doc(alias = "DMACHA")]
pub type Dmacha = crate::Reg<dmacha::DmachaSpec>;
#[doc = "DMA Channel A data transfer DMA accesses this register to read or write contents from sequential addresses specifed by DMA.ADRCHA."]
pub mod dmacha;
#[doc = "DMACHB (rw) register accessor: DMA Channel B data transfer DMA accesses this register to read or write contents from sequential addresses specifed by DMA.ADRCHB.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmachb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmachb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmachb`]
module"]
#[doc(alias = "DMACHB")]
pub type Dmachb = crate::Reg<dmachb::DmachbSpec>;
#[doc = "DMA Channel B data transfer DMA accesses this register to read or write contents from sequential addresses specifed by DMA.ADRCHB."]
pub mod dmachb;
#[doc = "AUTOCFG (rw) register accessor: Automatic Configuration This register configures automatic hardware updates to TXT and BUF. Configure this register to reduce software overhead during cipher modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`autocfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`autocfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@autocfg`]
module"]
#[doc(alias = "AUTOCFG")]
pub type Autocfg = crate::Reg<autocfg::AutocfgSpec>;
#[doc = "Automatic Configuration This register configures automatic hardware updates to TXT and BUF. Configure this register to reduce software overhead during cipher modes."]
pub mod autocfg;
#[doc = "KEY0 (rw) register accessor: Key Word 0 Write KEY0 through KEY3 to populate the 128-bit key. The key is not consumed by the hardware. It is hence not required to reload the key for subsequent block encryptions unless required by the application.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key0`]
module"]
#[doc(alias = "KEY0")]
pub type Key0 = crate::Reg<key0::Key0Spec>;
#[doc = "Key Word 0 Write KEY0 through KEY3 to populate the 128-bit key. The key is not consumed by the hardware. It is hence not required to reload the key for subsequent block encryptions unless required by the application."]
pub mod key0;
#[doc = "KEY1 (rw) register accessor: Key Word 1 Write KEY0 through KEY3 to populate the 128-bit key. The key is not consumed by the hardware. It is hence not required to reload the key for subsequent block encryptions unless required by the application.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key1`]
module"]
#[doc(alias = "KEY1")]
pub type Key1 = crate::Reg<key1::Key1Spec>;
#[doc = "Key Word 1 Write KEY0 through KEY3 to populate the 128-bit key. The key is not consumed by the hardware. It is hence not required to reload the key for subsequent block encryptions unless required by the application."]
pub mod key1;
#[doc = "KEY2 (rw) register accessor: Key Word 2 Write KEY0 through KEY3 to populate the 128-bit key. The key is not consumed by the hardware. It is hence not required to reload the key for subsequent block encryptions unless required by the application.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key2`]
module"]
#[doc(alias = "KEY2")]
pub type Key2 = crate::Reg<key2::Key2Spec>;
#[doc = "Key Word 2 Write KEY0 through KEY3 to populate the 128-bit key. The key is not consumed by the hardware. It is hence not required to reload the key for subsequent block encryptions unless required by the application."]
pub mod key2;
#[doc = "KEY3 (rw) register accessor: Key Word 3 Write KEY0 through KEY3 to populate the 128-bit key. The key is not consumed by the hardware. It is hence not required to reload the key for subsequent block encryptions unless required by the application.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key3`]
module"]
#[doc(alias = "KEY3")]
pub type Key3 = crate::Reg<key3::Key3Spec>;
#[doc = "Key Word 3 Write KEY0 through KEY3 to populate the 128-bit key. The key is not consumed by the hardware. It is hence not required to reload the key for subsequent block encryptions unless required by the application."]
pub mod key3;
#[doc = "TXT0 (rw) register accessor: Text Word 0 TXT is the 128-bit buffer, the AES-128 encryption algorithm performs its operations on. ECB input can be written to TXT, and ciphertext can be read from TXT.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txt0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txt0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txt0`]
module"]
#[doc(alias = "TXT0")]
pub type Txt0 = crate::Reg<txt0::Txt0Spec>;
#[doc = "Text Word 0 TXT is the 128-bit buffer, the AES-128 encryption algorithm performs its operations on. ECB input can be written to TXT, and ciphertext can be read from TXT."]
pub mod txt0;
#[doc = "TXT1 (rw) register accessor: Text Word 1 TXT is the 128-bit buffer, the AES-128 encryption algorithm performs its operations on. ECB input can be written to TXT, and ciphertext can be read from TXT.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txt1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txt1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txt1`]
module"]
#[doc(alias = "TXT1")]
pub type Txt1 = crate::Reg<txt1::Txt1Spec>;
#[doc = "Text Word 1 TXT is the 128-bit buffer, the AES-128 encryption algorithm performs its operations on. ECB input can be written to TXT, and ciphertext can be read from TXT."]
pub mod txt1;
#[doc = "TXT2 (rw) register accessor: Text Word 2 TXT is the 128-bit buffer, the AES-128 encryption algorithm performs its operations on. ECB input can be written to TXT, and ciphertext can be read from TXT.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txt2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txt2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txt2`]
module"]
#[doc(alias = "TXT2")]
pub type Txt2 = crate::Reg<txt2::Txt2Spec>;
#[doc = "Text Word 2 TXT is the 128-bit buffer, the AES-128 encryption algorithm performs its operations on. ECB input can be written to TXT, and ciphertext can be read from TXT."]
pub mod txt2;
#[doc = "TXT3 (rw) register accessor: Text Word 3 TXT is the 128-bit buffer, the AES-128 encryption algorithm performs its operations on. ECB input can be written to TXT, and ciphertext can be read from TXT.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txt3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txt3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txt3`]
module"]
#[doc(alias = "TXT3")]
pub type Txt3 = crate::Reg<txt3::Txt3Spec>;
#[doc = "Text Word 3 TXT is the 128-bit buffer, the AES-128 encryption algorithm performs its operations on. ECB input can be written to TXT, and ciphertext can be read from TXT."]
pub mod txt3;
#[doc = "TXTX0 (rw) register accessor: Text Word 0 XOR Write data to this register to XOR data with contents in TXT0.VAL.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txtx0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txtx0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txtx0`]
module"]
#[doc(alias = "TXTX0")]
pub type Txtx0 = crate::Reg<txtx0::Txtx0Spec>;
#[doc = "Text Word 0 XOR Write data to this register to XOR data with contents in TXT0.VAL."]
pub mod txtx0;
#[doc = "TXTX1 (rw) register accessor: Text Word 1 XOR Write data to this register to XOR data with contents in TXT1.VAL.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txtx1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txtx1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txtx1`]
module"]
#[doc(alias = "TXTX1")]
pub type Txtx1 = crate::Reg<txtx1::Txtx1Spec>;
#[doc = "Text Word 1 XOR Write data to this register to XOR data with contents in TXT1.VAL."]
pub mod txtx1;
#[doc = "TXTX2 (rw) register accessor: Text Word 2 XOR Write data to this register to XOR data with contents in TXT2.VAL.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txtx2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txtx2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txtx2`]
module"]
#[doc(alias = "TXTX2")]
pub type Txtx2 = crate::Reg<txtx2::Txtx2Spec>;
#[doc = "Text Word 2 XOR Write data to this register to XOR data with contents in TXT2.VAL."]
pub mod txtx2;
#[doc = "TXTX3 (rw) register accessor: Text Word 3 XOR Write data to this register to XOR data with contents in TXT3.VAL. AUTOCFG.TRGECB decides if a write to or a read of this field triggers an encryption.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txtx3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txtx3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txtx3`]
module"]
#[doc(alias = "TXTX3")]
pub type Txtx3 = crate::Reg<txtx3::Txtx3Spec>;
#[doc = "Text Word 3 XOR Write data to this register to XOR data with contents in TXT3.VAL. AUTOCFG.TRGECB decides if a write to or a read of this field triggers an encryption."]
pub mod txtx3;
#[doc = "BUF0 (rw) register accessor: Buffer Word 0 BUF is a 128-bit auxiliary register that functions as a buffer, counter, or storage of operations in cipher modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf0`]
module"]
#[doc(alias = "BUF0")]
pub type Buf0 = crate::Reg<buf0::Buf0Spec>;
#[doc = "Buffer Word 0 BUF is a 128-bit auxiliary register that functions as a buffer, counter, or storage of operations in cipher modes."]
pub mod buf0;
#[doc = "BUF1 (rw) register accessor: Buffer Word 1 BUF is a 128-bit auxiliary register that functions as a buffer, counter, or storage of operations in cipher modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf1`]
module"]
#[doc(alias = "BUF1")]
pub type Buf1 = crate::Reg<buf1::Buf1Spec>;
#[doc = "Buffer Word 1 BUF is a 128-bit auxiliary register that functions as a buffer, counter, or storage of operations in cipher modes."]
pub mod buf1;
#[doc = "BUF2 (rw) register accessor: Buffer Word 2 BUF is a 128-bit auxiliary register that functions as a buffer, counter, or storage of operations in cipher modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf2`]
module"]
#[doc(alias = "BUF2")]
pub type Buf2 = crate::Reg<buf2::Buf2Spec>;
#[doc = "Buffer Word 2 BUF is a 128-bit auxiliary register that functions as a buffer, counter, or storage of operations in cipher modes."]
pub mod buf2;
#[doc = "BUF3 (rw) register accessor: Buffer Word 3 BUF is a 128-bit auxiliary register that functions as a buffer, counter, or storage of operations in cipher modes. AUTOCFG.TRGECB decides if a write to this field triggers an encryption.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf3`]
module"]
#[doc(alias = "BUF3")]
pub type Buf3 = crate::Reg<buf3::Buf3Spec>;
#[doc = "Buffer Word 3 BUF is a 128-bit auxiliary register that functions as a buffer, counter, or storage of operations in cipher modes. AUTOCFG.TRGECB decides if a write to this field triggers an encryption."]
pub mod buf3;
#[doc = "TXTXBUF0 (rw) register accessor: Text Word 0 XOR Buffer Word 0 Read this register to obtain plaintext during CFB decryption.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txtxbuf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txtxbuf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txtxbuf0`]
module"]
#[doc(alias = "TXTXBUF0")]
pub type Txtxbuf0 = crate::Reg<txtxbuf0::Txtxbuf0Spec>;
#[doc = "Text Word 0 XOR Buffer Word 0 Read this register to obtain plaintext during CFB decryption."]
pub mod txtxbuf0;
#[doc = "TXTXBUF1 (rw) register accessor: Text Word 1 XOR Buffer Word 1 Read this register to obtain plaintext during CFB decryption.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txtxbuf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txtxbuf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txtxbuf1`]
module"]
#[doc(alias = "TXTXBUF1")]
pub type Txtxbuf1 = crate::Reg<txtxbuf1::Txtxbuf1Spec>;
#[doc = "Text Word 1 XOR Buffer Word 1 Read this register to obtain plaintext during CFB decryption."]
pub mod txtxbuf1;
#[doc = "TXTXBUF2 (rw) register accessor: Text Word 2 XOR Buffer Word 2 Read this register to obtain plaintext during CFB decryption.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txtxbuf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txtxbuf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txtxbuf2`]
module"]
#[doc(alias = "TXTXBUF2")]
pub type Txtxbuf2 = crate::Reg<txtxbuf2::Txtxbuf2Spec>;
#[doc = "Text Word 2 XOR Buffer Word 2 Read this register to obtain plaintext during CFB decryption."]
pub mod txtxbuf2;
#[doc = "TXTXBUF3 (rw) register accessor: Text Word 3 XOR Buffer Word3 Read this register to obtain plaintext during CFB decryption.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txtxbuf3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txtxbuf3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txtxbuf3`]
module"]
#[doc(alias = "TXTXBUF3")]
pub type Txtxbuf3 = crate::Reg<txtxbuf3::Txtxbuf3Spec>;
#[doc = "Text Word 3 XOR Buffer Word3 Read this register to obtain plaintext during CFB decryption."]
pub mod txtxbuf3;
#[doc = "IMASK (rw) register accessor: Interrupt mask. This register selects interrupt sources which are allowed to pass from RIS to MIS when the corresponding bit-fields are set to 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imask`]
module"]
#[doc(alias = "IMASK")]
pub type Imask = crate::Reg<imask::ImaskSpec>;
#[doc = "Interrupt mask. This register selects interrupt sources which are allowed to pass from RIS to MIS when the corresponding bit-fields are set to 1."]
pub mod imask;
#[doc = "RIS (rw) register accessor: Raw interrupt status. This register reflects the state of all pending interrupts, regardless of masking. This register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`]
module"]
#[doc(alias = "RIS")]
pub type Ris = crate::Reg<ris::RisSpec>;
#[doc = "Raw interrupt status. This register reflects the state of all pending interrupts, regardless of masking. This register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit."]
pub mod ris;
#[doc = "MIS (rw) register accessor: Masked interrupt status. This register is simply a bitwise AND of the contents of IMASK and RIS.*\\]
registers. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`]
module"]
#[doc(alias = "MIS")]
pub type Mis = crate::Reg<mis::MisSpec>;
#[doc = "Masked interrupt status. This register is simply a bitwise AND of the contents of IMASK and RIS.*\\]
registers. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit."]
pub mod mis;
#[doc = "ISET (rw) register accessor: Interrupt set register. This register can used by software for diagnostics and safety checking purposes. Writing a 1 to a bit in this register will set the event and the corresponding RIS bit also gets set. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iset`]
module"]
#[doc(alias = "ISET")]
pub type Iset = crate::Reg<iset::IsetSpec>;
#[doc = "Interrupt set register. This register can used by software for diagnostics and safety checking purposes. Writing a 1 to a bit in this register will set the event and the corresponding RIS bit also gets set. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets set."]
pub mod iset;
#[doc = "ICLR (rw) register accessor: Interrupt clear register. This register allows software to clear interrupts. Writing a 1 to a bit in this register will clear the event and the corresponding RIS bit also gets cleared. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets cleared.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iclr`]
module"]
#[doc(alias = "ICLR")]
pub type Iclr = crate::Reg<iclr::IclrSpec>;
#[doc = "Interrupt clear register. This register allows software to clear interrupts. Writing a 1 to a bit in this register will clear the event and the corresponding RIS bit also gets cleared. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets cleared."]
pub mod iclr;
#[doc = "IMSET (rw) register accessor: Interrupt mask set register. Writing a 1 to a bit in this register will set the corresponding IMASK bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imset`]
module"]
#[doc(alias = "IMSET")]
pub type Imset = crate::Reg<imset::ImsetSpec>;
#[doc = "Interrupt mask set register. Writing a 1 to a bit in this register will set the corresponding IMASK bit."]
pub mod imset;
#[doc = "IMCLR (rw) register accessor: Interrupt mask clear register. Writing a 1 to a bit in this register will clear the corresponding IMASK bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imclr`]
module"]
#[doc(alias = "IMCLR")]
pub type Imclr = crate::Reg<imclr::ImclrSpec>;
#[doc = "Interrupt mask clear register. Writing a 1 to a bit in this register will clear the corresponding IMASK bit."]
pub mod imclr;
