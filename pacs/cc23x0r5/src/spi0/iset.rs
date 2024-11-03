#[doc = "Register `ISET` reader"]
pub type R = crate::R<IsetSpec>;
#[doc = "Register `ISET` writer"]
pub type W = crate::W<IsetSpec>;
#[doc = "0:0\\]
Set RXFIFO overflow event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxovf {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Rxovf> for bool {
    #[inline(always)]
    fn from(variant: Rxovf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXOVF` reader - 0:0\\]
Set RXFIFO overflow event."]
pub type RxovfR = crate::BitReader<Rxovf>;
impl RxovfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxovf {
        match self.bits {
            true => Rxovf::Set,
            false => Rxovf::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rxovf::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Rxovf::Noeff
    }
}
#[doc = "Field `RXOVF` writer - 0:0\\]
Set RXFIFO overflow event."]
pub type RxovfW<'a, REG> = crate::BitWriter<'a, REG, Rxovf>;
impl<'a, REG> RxovfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rxovf::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Rxovf::Noeff)
    }
}
#[doc = "1:1\\]
Set Parity error event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Per {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Per> for bool {
    #[inline(always)]
    fn from(variant: Per) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PER` reader - 1:1\\]
Set Parity error event."]
pub type PerR = crate::BitReader<Per>;
impl PerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Per {
        match self.bits {
            true => Per::Set,
            false => Per::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Per::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Per::Noeff
    }
}
#[doc = "Field `PER` writer - 1:1\\]
Set Parity error event."]
pub type PerW<'a, REG> = crate::BitWriter<'a, REG, Per>;
impl<'a, REG> PerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Per::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Per::Noeff)
    }
}
#[doc = "2:2\\]
Set SPI Receive Time-Out Event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtout {
    #[doc = "1: Set Interrrupt Mask"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Rtout> for bool {
    #[inline(always)]
    fn from(variant: Rtout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTOUT` reader - 2:2\\]
Set SPI Receive Time-Out Event."]
pub type RtoutR = crate::BitReader<Rtout>;
impl RtoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtout {
        match self.bits {
            true => Rtout::Set,
            false => Rtout::Noeff,
        }
    }
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rtout::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Rtout::Noeff
    }
}
#[doc = "Field `RTOUT` writer - 2:2\\]
Set SPI Receive Time-Out Event."]
pub type RtoutW<'a, REG> = crate::BitWriter<'a, REG, Rtout>;
impl<'a, REG> RtoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrrupt Mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rtout::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Rtout::Noeff)
    }
}
#[doc = "3:3\\]
Set Receive FIFO event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rx {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Rx> for bool {
    #[inline(always)]
    fn from(variant: Rx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX` reader - 3:3\\]
Set Receive FIFO event."]
pub type RxR = crate::BitReader<Rx>;
impl RxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rx {
        match self.bits {
            true => Rx::Set,
            false => Rx::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rx::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Rx::Noeff
    }
}
#[doc = "Field `RX` writer - 3:3\\]
Set Receive FIFO event."]
pub type RxW<'a, REG> = crate::BitWriter<'a, REG, Rx>;
impl<'a, REG> RxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rx::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Rx::Noeff)
    }
}
#[doc = "4:4\\]
Set Transmit FIFO event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tx {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Tx> for bool {
    #[inline(always)]
    fn from(variant: Tx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX` reader - 4:4\\]
Set Transmit FIFO event."]
pub type TxR = crate::BitReader<Tx>;
impl TxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tx {
        match self.bits {
            true => Tx::Set,
            false => Tx::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tx::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Tx::Noeff
    }
}
#[doc = "Field `TX` writer - 4:4\\]
Set Transmit FIFO event."]
pub type TxW<'a, REG> = crate::BitWriter<'a, REG, Tx>;
impl<'a, REG> TxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Tx::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Tx::Noeff)
    }
}
#[doc = "5:5\\]
Set Transmit FIFO Empty event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txempty {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Txempty> for bool {
    #[inline(always)]
    fn from(variant: Txempty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEMPTY` reader - 5:5\\]
Set Transmit FIFO Empty event."]
pub type TxemptyR = crate::BitReader<Txempty>;
impl TxemptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txempty {
        match self.bits {
            true => Txempty::Set,
            false => Txempty::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Txempty::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Txempty::Noeff
    }
}
#[doc = "Field `TXEMPTY` writer - 5:5\\]
Set Transmit FIFO Empty event."]
pub type TxemptyW<'a, REG> = crate::BitWriter<'a, REG, Txempty>;
impl<'a, REG> TxemptyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Txempty::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Txempty::Noeff)
    }
}
#[doc = "6:6\\]
Set SPI IDLE event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idle {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Idle> for bool {
    #[inline(always)]
    fn from(variant: Idle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE` reader - 6:6\\]
Set SPI IDLE event."]
pub type IdleR = crate::BitReader<Idle>;
impl IdleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Idle {
        match self.bits {
            true => Idle::Set,
            false => Idle::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Idle::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Idle::Noeff
    }
}
#[doc = "Field `IDLE` writer - 6:6\\]
Set SPI IDLE event."]
pub type IdleW<'a, REG> = crate::BitWriter<'a, REG, Idle>;
impl<'a, REG> IdleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Idle::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Idle::Noeff)
    }
}
#[doc = "7:7\\]
Set DMA Done event for RX.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmarx {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Dmarx> for bool {
    #[inline(always)]
    fn from(variant: Dmarx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMARX` reader - 7:7\\]
Set DMA Done event for RX."]
pub type DmarxR = crate::BitReader<Dmarx>;
impl DmarxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmarx {
        match self.bits {
            true => Dmarx::Set,
            false => Dmarx::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dmarx::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Dmarx::Noeff
    }
}
#[doc = "Field `DMARX` writer - 7:7\\]
Set DMA Done event for RX."]
pub type DmarxW<'a, REG> = crate::BitWriter<'a, REG, Dmarx>;
impl<'a, REG> DmarxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dmarx::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Dmarx::Noeff)
    }
}
#[doc = "8:8\\]
Set DMA Done event for TX.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmatx {
    #[doc = "1: Set Interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Dmatx> for bool {
    #[inline(always)]
    fn from(variant: Dmatx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMATX` reader - 8:8\\]
Set DMA Done event for TX."]
pub type DmatxR = crate::BitReader<Dmatx>;
impl DmatxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmatx {
        match self.bits {
            true => Dmatx::Set,
            false => Dmatx::Noeff,
        }
    }
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dmatx::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Dmatx::Noeff
    }
}
#[doc = "Field `DMATX` writer - 8:8\\]
Set DMA Done event for TX."]
pub type DmatxW<'a, REG> = crate::BitWriter<'a, REG, Dmatx>;
impl<'a, REG> DmatxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dmatx::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Dmatx::Noeff)
    }
}
#[doc = "Field `RESERVED9` reader - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED9` writer - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Set RXFIFO overflow event."]
    #[inline(always)]
    pub fn rxovf(&self) -> RxovfR {
        RxovfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Set Parity error event."]
    #[inline(always)]
    pub fn per(&self) -> PerR {
        PerR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Set SPI Receive Time-Out Event."]
    #[inline(always)]
    pub fn rtout(&self) -> RtoutR {
        RtoutR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Set Receive FIFO event."]
    #[inline(always)]
    pub fn rx(&self) -> RxR {
        RxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Set Transmit FIFO event."]
    #[inline(always)]
    pub fn tx(&self) -> TxR {
        TxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Set Transmit FIFO Empty event."]
    #[inline(always)]
    pub fn txempty(&self) -> TxemptyR {
        TxemptyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Set SPI IDLE event."]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Set DMA Done event for RX."]
    #[inline(always)]
    pub fn dmarx(&self) -> DmarxR {
        DmarxR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Set DMA Done event for TX."]
    #[inline(always)]
    pub fn dmatx(&self) -> DmatxR {
        DmatxR::new(((self.bits >> 8) & 1) != 0)
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
Set RXFIFO overflow event."]
    #[inline(always)]
    #[must_use]
    pub fn rxovf(&mut self) -> RxovfW<IsetSpec> {
        RxovfW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Set Parity error event."]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PerW<IsetSpec> {
        PerW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Set SPI Receive Time-Out Event."]
    #[inline(always)]
    #[must_use]
    pub fn rtout(&mut self) -> RtoutW<IsetSpec> {
        RtoutW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Set Receive FIFO event."]
    #[inline(always)]
    #[must_use]
    pub fn rx(&mut self) -> RxW<IsetSpec> {
        RxW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Set Transmit FIFO event."]
    #[inline(always)]
    #[must_use]
    pub fn tx(&mut self) -> TxW<IsetSpec> {
        TxW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Set Transmit FIFO Empty event."]
    #[inline(always)]
    #[must_use]
    pub fn txempty(&mut self) -> TxemptyW<IsetSpec> {
        TxemptyW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Set SPI IDLE event."]
    #[inline(always)]
    #[must_use]
    pub fn idle(&mut self) -> IdleW<IsetSpec> {
        IdleW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Set DMA Done event for RX."]
    #[inline(always)]
    #[must_use]
    pub fn dmarx(&mut self) -> DmarxW<IsetSpec> {
        DmarxW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Set DMA Done event for TX."]
    #[inline(always)]
    #[must_use]
    pub fn dmatx(&mut self) -> DmatxW<IsetSpec> {
        DmatxW::new(self, 8)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> Reserved9W<IsetSpec> {
        Reserved9W::new(self, 9)
    }
}
#[doc = "Interrupt set register. This register can used by software for diagnostics and safety checking purposes. Writing a 1 to a bit in this register will set the event and the corresponding RIS bit also gets set. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsetSpec;
impl crate::RegisterSpec for IsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iset::R`](R) reader structure"]
impl crate::Readable for IsetSpec {}
#[doc = "`write(|w| ..)` method takes [`iset::W`](W) writer structure"]
impl crate::Writable for IsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISET to value 0"]
impl crate::Resettable for IsetSpec {
    const RESET_VALUE: u32 = 0;
}
