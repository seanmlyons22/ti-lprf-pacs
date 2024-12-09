#[doc = "Register `IMASK` reader"]
pub type R = crate::R<ImaskSpec>;
#[doc = "Register `IMASK` writer"]
pub type W = crate::W<ImaskSpec>;
#[doc = "0:0\\]
RXFIFO overflow event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxfifoOvf {
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
}
impl From<RxfifoOvf> for bool {
    #[inline(always)]
    fn from(variant: RxfifoOvf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFIFO_OVF` reader - 0:0\\]
RXFIFO overflow event mask."]
pub type RxfifoOvfR = crate::BitReader<RxfifoOvf>;
impl RxfifoOvfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxfifoOvf {
        match self.bits {
            true => RxfifoOvf::Set,
            false => RxfifoOvf::Clr,
        }
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == RxfifoOvf::Set
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == RxfifoOvf::Clr
    }
}
#[doc = "Field `RXFIFO_OVF` writer - 0:0\\]
RXFIFO overflow event mask."]
pub type RxfifoOvfW<'a, REG> = crate::BitWriter<'a, REG, RxfifoOvf>;
impl<'a, REG> RxfifoOvfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(RxfifoOvf::Set)
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(RxfifoOvf::Clr)
    }
}
#[doc = "1:1\\]
Parity error event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Per {
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
}
impl From<Per> for bool {
    #[inline(always)]
    fn from(variant: Per) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PER` reader - 1:1\\]
Parity error event mask."]
pub type PerR = crate::BitReader<Per>;
impl PerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Per {
        match self.bits {
            true => Per::Set,
            false => Per::Clr,
        }
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Per::Set
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Per::Clr
    }
}
#[doc = "Field `PER` writer - 1:1\\]
Parity error event mask."]
pub type PerW<'a, REG> = crate::BitWriter<'a, REG, Per>;
impl<'a, REG> PerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Per::Set)
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Per::Clr)
    }
}
#[doc = "2:2\\]
SPI Receive Time-Out event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtout {
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
}
impl From<Rtout> for bool {
    #[inline(always)]
    fn from(variant: Rtout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTOUT` reader - 2:2\\]
SPI Receive Time-Out event mask."]
pub type RtoutR = crate::BitReader<Rtout>;
impl RtoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtout {
        match self.bits {
            true => Rtout::Set,
            false => Rtout::Clr,
        }
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rtout::Set
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Rtout::Clr
    }
}
#[doc = "Field `RTOUT` writer - 2:2\\]
SPI Receive Time-Out event mask."]
pub type RtoutW<'a, REG> = crate::BitWriter<'a, REG, Rtout>;
impl<'a, REG> RtoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rtout::Set)
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rtout::Clr)
    }
}
#[doc = "3:3\\]
Receive FIFO event. This interrupt is set if the selected Receive FIFO level has been reached\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rx {
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
}
impl From<Rx> for bool {
    #[inline(always)]
    fn from(variant: Rx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX` reader - 3:3\\]
Receive FIFO event. This interrupt is set if the selected Receive FIFO level has been reached"]
pub type RxR = crate::BitReader<Rx>;
impl RxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rx {
        match self.bits {
            true => Rx::Set,
            false => Rx::Clr,
        }
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rx::Set
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Rx::Clr
    }
}
#[doc = "Field `RX` writer - 3:3\\]
Receive FIFO event. This interrupt is set if the selected Receive FIFO level has been reached"]
pub type RxW<'a, REG> = crate::BitWriter<'a, REG, Rx>;
impl<'a, REG> RxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rx::Set)
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rx::Clr)
    }
}
#[doc = "4:4\\]
Transmit FIFO event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tx {
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
}
impl From<Tx> for bool {
    #[inline(always)]
    fn from(variant: Tx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX` reader - 4:4\\]
Transmit FIFO event mask."]
pub type TxR = crate::BitReader<Tx>;
impl TxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tx {
        match self.bits {
            true => Tx::Set,
            false => Tx::Clr,
        }
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tx::Set
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tx::Clr
    }
}
#[doc = "Field `TX` writer - 4:4\\]
Transmit FIFO event mask."]
pub type TxW<'a, REG> = crate::BitWriter<'a, REG, Tx>;
impl<'a, REG> TxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Tx::Set)
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tx::Clr)
    }
}
#[doc = "5:5\\]
Transmit FIFO Empty event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txempty {
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
}
impl From<Txempty> for bool {
    #[inline(always)]
    fn from(variant: Txempty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEMPTY` reader - 5:5\\]
Transmit FIFO Empty event mask."]
pub type TxemptyR = crate::BitReader<Txempty>;
impl TxemptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txempty {
        match self.bits {
            true => Txempty::Set,
            false => Txempty::Clr,
        }
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Txempty::Set
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Txempty::Clr
    }
}
#[doc = "Field `TXEMPTY` writer - 5:5\\]
Transmit FIFO Empty event mask."]
pub type TxemptyW<'a, REG> = crate::BitWriter<'a, REG, Txempty>;
impl<'a, REG> TxemptyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Txempty::Set)
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Txempty::Clr)
    }
}
#[doc = "6:6\\]
SPI Idle event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idle {
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
}
impl From<Idle> for bool {
    #[inline(always)]
    fn from(variant: Idle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE` reader - 6:6\\]
SPI Idle event mask."]
pub type IdleR = crate::BitReader<Idle>;
impl IdleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Idle {
        match self.bits {
            true => Idle::Set,
            false => Idle::Clr,
        }
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Idle::Set
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Idle::Clr
    }
}
#[doc = "Field `IDLE` writer - 6:6\\]
SPI Idle event mask."]
pub type IdleW<'a, REG> = crate::BitWriter<'a, REG, Idle>;
impl<'a, REG> IdleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Idle::Set)
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Idle::Clr)
    }
}
#[doc = "7:7\\]
DMA Done event for RX event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaDoneRx {
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
}
impl From<DmaDoneRx> for bool {
    #[inline(always)]
    fn from(variant: DmaDoneRx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_DONE_RX` reader - 7:7\\]
DMA Done event for RX event mask."]
pub type DmaDoneRxR = crate::BitReader<DmaDoneRx>;
impl DmaDoneRxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaDoneRx {
        match self.bits {
            true => DmaDoneRx::Set,
            false => DmaDoneRx::Clr,
        }
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == DmaDoneRx::Set
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == DmaDoneRx::Clr
    }
}
#[doc = "Field `DMA_DONE_RX` writer - 7:7\\]
DMA Done event for RX event mask."]
pub type DmaDoneRxW<'a, REG> = crate::BitWriter<'a, REG, DmaDoneRx>;
impl<'a, REG> DmaDoneRxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(DmaDoneRx::Set)
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(DmaDoneRx::Clr)
    }
}
#[doc = "8:8\\]
DMA Done event for TX event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaDoneTx {
    #[doc = "1: Set Interrupt Mask"]
    Set = 1,
    #[doc = "0: Clear Interrupt Mask"]
    Clr = 0,
}
impl From<DmaDoneTx> for bool {
    #[inline(always)]
    fn from(variant: DmaDoneTx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_DONE_TX` reader - 8:8\\]
DMA Done event for TX event mask."]
pub type DmaDoneTxR = crate::BitReader<DmaDoneTx>;
impl DmaDoneTxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaDoneTx {
        match self.bits {
            true => DmaDoneTx::Set,
            false => DmaDoneTx::Clr,
        }
    }
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == DmaDoneTx::Set
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == DmaDoneTx::Clr
    }
}
#[doc = "Field `DMA_DONE_TX` writer - 8:8\\]
DMA Done event for TX event mask."]
pub type DmaDoneTxW<'a, REG> = crate::BitWriter<'a, REG, DmaDoneTx>;
impl<'a, REG> DmaDoneTxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(DmaDoneTx::Set)
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(DmaDoneTx::Clr)
    }
}
#[doc = "Field `RESERVED9` reader - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
RXFIFO overflow event mask."]
    #[inline(always)]
    pub fn rxfifo_ovf(&self) -> RxfifoOvfR {
        RxfifoOvfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Parity error event mask."]
    #[inline(always)]
    pub fn per(&self) -> PerR {
        PerR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
SPI Receive Time-Out event mask."]
    #[inline(always)]
    pub fn rtout(&self) -> RtoutR {
        RtoutR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Receive FIFO event. This interrupt is set if the selected Receive FIFO level has been reached"]
    #[inline(always)]
    pub fn rx(&self) -> RxR {
        RxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Transmit FIFO event mask."]
    #[inline(always)]
    pub fn tx(&self) -> TxR {
        TxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Transmit FIFO Empty event mask."]
    #[inline(always)]
    pub fn txempty(&self) -> TxemptyR {
        TxemptyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
SPI Idle event mask."]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
DMA Done event for RX event mask."]
    #[inline(always)]
    pub fn dma_done_rx(&self) -> DmaDoneRxR {
        DmaDoneRxR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
DMA Done event for TX event mask."]
    #[inline(always)]
    pub fn dma_done_tx(&self) -> DmaDoneTxR {
        DmaDoneTxR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
RXFIFO overflow event mask."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_ovf(&mut self) -> RxfifoOvfW<ImaskSpec> {
        RxfifoOvfW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Parity error event mask."]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PerW<ImaskSpec> {
        PerW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
SPI Receive Time-Out event mask."]
    #[inline(always)]
    #[must_use]
    pub fn rtout(&mut self) -> RtoutW<ImaskSpec> {
        RtoutW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Receive FIFO event. This interrupt is set if the selected Receive FIFO level has been reached"]
    #[inline(always)]
    #[must_use]
    pub fn rx(&mut self) -> RxW<ImaskSpec> {
        RxW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Transmit FIFO event mask."]
    #[inline(always)]
    #[must_use]
    pub fn tx(&mut self) -> TxW<ImaskSpec> {
        TxW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Transmit FIFO Empty event mask."]
    #[inline(always)]
    #[must_use]
    pub fn txempty(&mut self) -> TxemptyW<ImaskSpec> {
        TxemptyW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
SPI Idle event mask."]
    #[inline(always)]
    #[must_use]
    pub fn idle(&mut self) -> IdleW<ImaskSpec> {
        IdleW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
DMA Done event for RX event mask."]
    #[inline(always)]
    #[must_use]
    pub fn dma_done_rx(&mut self) -> DmaDoneRxW<ImaskSpec> {
        DmaDoneRxW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
DMA Done event for TX event mask."]
    #[inline(always)]
    #[must_use]
    pub fn dma_done_tx(&mut self) -> DmaDoneTxW<ImaskSpec> {
        DmaDoneTxW::new(self, 8)
    }
}
#[doc = "Interrupt Mask. If a bit is set, then corresponding interrupt is un-masked. Un-masking the interrupt causes the raw interrupt to be visible in IIDX, as well as MIS.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImaskSpec;
impl crate::RegisterSpec for ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imask::R`](R) reader structure"]
impl crate::Readable for ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`imask::W`](W) writer structure"]
impl crate::Writable for ImaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMASK to value 0"]
impl crate::Resettable for ImaskSpec {
    const RESET_VALUE: u32 = 0;
}
