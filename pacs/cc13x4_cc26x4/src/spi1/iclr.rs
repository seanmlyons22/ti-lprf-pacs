#[doc = "Register `ICLR` reader"]
pub type R = crate::R<IclrSpec>;
#[doc = "Register `ICLR` writer"]
pub type W = crate::W<IclrSpec>;
#[doc = "0:0\\]
Clear RXFIFO overflow event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxfifoOvf {
    #[doc = "1: Clear Interrupt"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
}
impl From<RxfifoOvf> for bool {
    #[inline(always)]
    fn from(variant: RxfifoOvf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFIFO_OVF` reader - 0:0\\]
Clear RXFIFO overflow event."]
pub type RxfifoOvfR = crate::BitReader<RxfifoOvf>;
impl RxfifoOvfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxfifoOvf {
        match self.bits {
            true => RxfifoOvf::Clr,
            false => RxfifoOvf::NoEffect,
        }
    }
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == RxfifoOvf::Clr
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == RxfifoOvf::NoEffect
    }
}
#[doc = "Field `RXFIFO_OVF` writer - 0:0\\]
Clear RXFIFO overflow event."]
pub type RxfifoOvfW<'a, REG> = crate::BitWriter<'a, REG, RxfifoOvf>;
impl<'a, REG> RxfifoOvfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(RxfifoOvf::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(RxfifoOvf::NoEffect)
    }
}
#[doc = "1:1\\]
Clear Parity error event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Per {
    #[doc = "1: Clear Interrupt"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
}
impl From<Per> for bool {
    #[inline(always)]
    fn from(variant: Per) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PER` reader - 1:1\\]
Clear Parity error event."]
pub type PerR = crate::BitReader<Per>;
impl PerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Per {
        match self.bits {
            true => Per::Clr,
            false => Per::NoEffect,
        }
    }
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Per::Clr
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Per::NoEffect
    }
}
#[doc = "Field `PER` writer - 1:1\\]
Clear Parity error event."]
pub type PerW<'a, REG> = crate::BitWriter<'a, REG, Per>;
impl<'a, REG> PerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Per::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Per::NoEffect)
    }
}
#[doc = "2:2\\]
Clear SPI Receive Time-Out event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtout {
    #[doc = "1: Clear Interrupt"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
}
impl From<Rtout> for bool {
    #[inline(always)]
    fn from(variant: Rtout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTOUT` reader - 2:2\\]
Clear SPI Receive Time-Out event."]
pub type RtoutR = crate::BitReader<Rtout>;
impl RtoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtout {
        match self.bits {
            true => Rtout::Clr,
            false => Rtout::NoEffect,
        }
    }
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Rtout::Clr
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Rtout::NoEffect
    }
}
#[doc = "Field `RTOUT` writer - 2:2\\]
Clear SPI Receive Time-Out event."]
pub type RtoutW<'a, REG> = crate::BitWriter<'a, REG, Rtout>;
impl<'a, REG> RtoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rtout::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Rtout::NoEffect)
    }
}
#[doc = "3:3\\]
Clear Receive FIFO event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rx {
    #[doc = "1: Clear Interrupt"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
}
impl From<Rx> for bool {
    #[inline(always)]
    fn from(variant: Rx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX` reader - 3:3\\]
Clear Receive FIFO event."]
pub type RxR = crate::BitReader<Rx>;
impl RxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rx {
        match self.bits {
            true => Rx::Clr,
            false => Rx::NoEffect,
        }
    }
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Rx::Clr
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Rx::NoEffect
    }
}
#[doc = "Field `RX` writer - 3:3\\]
Clear Receive FIFO event."]
pub type RxW<'a, REG> = crate::BitWriter<'a, REG, Rx>;
impl<'a, REG> RxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rx::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Rx::NoEffect)
    }
}
#[doc = "4:4\\]
Clear Transmit FIFO event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tx {
    #[doc = "1: Clear Interrupt"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
}
impl From<Tx> for bool {
    #[inline(always)]
    fn from(variant: Tx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX` reader - 4:4\\]
Clear Transmit FIFO event."]
pub type TxR = crate::BitReader<Tx>;
impl TxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tx {
        match self.bits {
            true => Tx::Clr,
            false => Tx::NoEffect,
        }
    }
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tx::Clr
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Tx::NoEffect
    }
}
#[doc = "Field `TX` writer - 4:4\\]
Clear Transmit FIFO event."]
pub type TxW<'a, REG> = crate::BitWriter<'a, REG, Tx>;
impl<'a, REG> TxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tx::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Tx::NoEffect)
    }
}
#[doc = "5:5\\]
Clear Transmit FIFO Empty event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txempty {
    #[doc = "1: Clear Interrupt"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
}
impl From<Txempty> for bool {
    #[inline(always)]
    fn from(variant: Txempty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEMPTY` reader - 5:5\\]
Clear Transmit FIFO Empty event."]
pub type TxemptyR = crate::BitReader<Txempty>;
impl TxemptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txempty {
        match self.bits {
            true => Txempty::Clr,
            false => Txempty::NoEffect,
        }
    }
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Txempty::Clr
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Txempty::NoEffect
    }
}
#[doc = "Field `TXEMPTY` writer - 5:5\\]
Clear Transmit FIFO Empty event."]
pub type TxemptyW<'a, REG> = crate::BitWriter<'a, REG, Txempty>;
impl<'a, REG> TxemptyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Txempty::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Txempty::NoEffect)
    }
}
#[doc = "6:6\\]
Clear SPI IDLE mode event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idle {
    #[doc = "1: Clear Interrupt"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
}
impl From<Idle> for bool {
    #[inline(always)]
    fn from(variant: Idle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE` reader - 6:6\\]
Clear SPI IDLE mode event."]
pub type IdleR = crate::BitReader<Idle>;
impl IdleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Idle {
        match self.bits {
            true => Idle::Clr,
            false => Idle::NoEffect,
        }
    }
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Idle::Clr
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Idle::NoEffect
    }
}
#[doc = "Field `IDLE` writer - 6:6\\]
Clear SPI IDLE mode event."]
pub type IdleW<'a, REG> = crate::BitWriter<'a, REG, Idle>;
impl<'a, REG> IdleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Idle::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Idle::NoEffect)
    }
}
#[doc = "7:7\\]
Clear DMA Done event for RX.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaDoneRx {
    #[doc = "1: Clear Interrupt"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
}
impl From<DmaDoneRx> for bool {
    #[inline(always)]
    fn from(variant: DmaDoneRx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_DONE_RX` reader - 7:7\\]
Clear DMA Done event for RX."]
pub type DmaDoneRxR = crate::BitReader<DmaDoneRx>;
impl DmaDoneRxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaDoneRx {
        match self.bits {
            true => DmaDoneRx::Clr,
            false => DmaDoneRx::NoEffect,
        }
    }
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == DmaDoneRx::Clr
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == DmaDoneRx::NoEffect
    }
}
#[doc = "Field `DMA_DONE_RX` writer - 7:7\\]
Clear DMA Done event for RX."]
pub type DmaDoneRxW<'a, REG> = crate::BitWriter<'a, REG, DmaDoneRx>;
impl<'a, REG> DmaDoneRxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(DmaDoneRx::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(DmaDoneRx::NoEffect)
    }
}
#[doc = "8:8\\]
Clear DMA Done event for TX.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaDoneTx {
    #[doc = "1: Clear Interrupt"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    NoEffect = 0,
}
impl From<DmaDoneTx> for bool {
    #[inline(always)]
    fn from(variant: DmaDoneTx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_DONE_TX` reader - 8:8\\]
Clear DMA Done event for TX."]
pub type DmaDoneTxR = crate::BitReader<DmaDoneTx>;
impl DmaDoneTxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaDoneTx {
        match self.bits {
            true => DmaDoneTx::Clr,
            false => DmaDoneTx::NoEffect,
        }
    }
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == DmaDoneTx::Clr
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == DmaDoneTx::NoEffect
    }
}
#[doc = "Field `DMA_DONE_TX` writer - 8:8\\]
Clear DMA Done event for TX."]
pub type DmaDoneTxW<'a, REG> = crate::BitWriter<'a, REG, DmaDoneTx>;
impl<'a, REG> DmaDoneTxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(DmaDoneTx::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(DmaDoneTx::NoEffect)
    }
}
#[doc = "Field `RESERVED9` reader - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Clear RXFIFO overflow event."]
    #[inline(always)]
    pub fn rxfifo_ovf(&self) -> RxfifoOvfR {
        RxfifoOvfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clear Parity error event."]
    #[inline(always)]
    pub fn per(&self) -> PerR {
        PerR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Clear SPI Receive Time-Out event."]
    #[inline(always)]
    pub fn rtout(&self) -> RtoutR {
        RtoutR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Clear Receive FIFO event."]
    #[inline(always)]
    pub fn rx(&self) -> RxR {
        RxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Clear Transmit FIFO event."]
    #[inline(always)]
    pub fn tx(&self) -> TxR {
        TxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Clear Transmit FIFO Empty event."]
    #[inline(always)]
    pub fn txempty(&self) -> TxemptyR {
        TxemptyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Clear SPI IDLE mode event."]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Clear DMA Done event for RX."]
    #[inline(always)]
    pub fn dma_done_rx(&self) -> DmaDoneRxR {
        DmaDoneRxR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Clear DMA Done event for TX."]
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
Clear RXFIFO overflow event."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_ovf(&mut self) -> RxfifoOvfW<IclrSpec> {
        RxfifoOvfW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clear Parity error event."]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PerW<IclrSpec> {
        PerW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Clear SPI Receive Time-Out event."]
    #[inline(always)]
    #[must_use]
    pub fn rtout(&mut self) -> RtoutW<IclrSpec> {
        RtoutW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Clear Receive FIFO event."]
    #[inline(always)]
    #[must_use]
    pub fn rx(&mut self) -> RxW<IclrSpec> {
        RxW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Clear Transmit FIFO event."]
    #[inline(always)]
    #[must_use]
    pub fn tx(&mut self) -> TxW<IclrSpec> {
        TxW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Clear Transmit FIFO Empty event."]
    #[inline(always)]
    #[must_use]
    pub fn txempty(&mut self) -> TxemptyW<IclrSpec> {
        TxemptyW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Clear SPI IDLE mode event."]
    #[inline(always)]
    #[must_use]
    pub fn idle(&mut self) -> IdleW<IclrSpec> {
        IdleW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Clear DMA Done event for RX."]
    #[inline(always)]
    #[must_use]
    pub fn dma_done_rx(&mut self) -> DmaDoneRxW<IclrSpec> {
        DmaDoneRxW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Clear DMA Done event for TX."]
    #[inline(always)]
    #[must_use]
    pub fn dma_done_tx(&mut self) -> DmaDoneTxW<IclrSpec> {
        DmaDoneTxW::new(self, 8)
    }
}
#[doc = "Interrupt clear. Write a 1 to clear the corresponding Interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IclrSpec;
impl crate::RegisterSpec for IclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iclr::R`](R) reader structure"]
impl crate::Readable for IclrSpec {}
#[doc = "`write(|w| ..)` method takes [`iclr::W`](W) writer structure"]
impl crate::Writable for IclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICLR to value 0"]
impl crate::Resettable for IclrSpec {
    const RESET_VALUE: u32 = 0;
}
