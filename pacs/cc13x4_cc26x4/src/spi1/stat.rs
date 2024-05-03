#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "0:0\\]
Transmit FIFO empty.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tfe {
    #[doc = "1: Transmit FIFO is empty."]
    Empty = 1,
    #[doc = "0: Transmit FIFO is not empty."]
    NotEmpty = 0,
}
impl From<Tfe> for bool {
    #[inline(always)]
    fn from(variant: Tfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFE` reader - 0:0\\]
Transmit FIFO empty."]
pub type TfeR = crate::BitReader<Tfe>;
impl TfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tfe {
        match self.bits {
            true => Tfe::Empty,
            false => Tfe::NotEmpty,
        }
    }
    #[doc = "Transmit FIFO is empty."]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Tfe::Empty
    }
    #[doc = "Transmit FIFO is not empty."]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == Tfe::NotEmpty
    }
}
#[doc = "Field `TFE` writer - 0:0\\]
Transmit FIFO empty."]
pub type TfeW<'a, REG> = crate::BitWriter<'a, REG, Tfe>;
impl<'a, REG> TfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit FIFO is empty."]
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(Tfe::Empty)
    }
    #[doc = "Transmit FIFO is not empty."]
    #[inline(always)]
    pub fn not_empty(self) -> &'a mut crate::W<REG> {
        self.variant(Tfe::NotEmpty)
    }
}
#[doc = "1:1\\]
Transmit FIFO not full\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tnf {
    #[doc = "1: Transmit FIFO is not full."]
    NotFull = 1,
    #[doc = "0: Transmit FIFO is full."]
    Full = 0,
}
impl From<Tnf> for bool {
    #[inline(always)]
    fn from(variant: Tnf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TNF` reader - 1:1\\]
Transmit FIFO not full"]
pub type TnfR = crate::BitReader<Tnf>;
impl TnfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tnf {
        match self.bits {
            true => Tnf::NotFull,
            false => Tnf::Full,
        }
    }
    #[doc = "Transmit FIFO is not full."]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == Tnf::NotFull
    }
    #[doc = "Transmit FIFO is full."]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Tnf::Full
    }
}
#[doc = "Field `TNF` writer - 1:1\\]
Transmit FIFO not full"]
pub type TnfW<'a, REG> = crate::BitWriter<'a, REG, Tnf>;
impl<'a, REG> TnfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit FIFO is not full."]
    #[inline(always)]
    pub fn not_full(self) -> &'a mut crate::W<REG> {
        self.variant(Tnf::NotFull)
    }
    #[doc = "Transmit FIFO is full."]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(Tnf::Full)
    }
}
#[doc = "2:2\\]
Receive FIFO empty.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfe {
    #[doc = "1: Receive FIFO is empty."]
    Empty = 1,
    #[doc = "0: Receive FIFO is not empty."]
    NotEmpty = 0,
}
impl From<Rfe> for bool {
    #[inline(always)]
    fn from(variant: Rfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFE` reader - 2:2\\]
Receive FIFO empty."]
pub type RfeR = crate::BitReader<Rfe>;
impl RfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfe {
        match self.bits {
            true => Rfe::Empty,
            false => Rfe::NotEmpty,
        }
    }
    #[doc = "Receive FIFO is empty."]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Rfe::Empty
    }
    #[doc = "Receive FIFO is not empty."]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == Rfe::NotEmpty
    }
}
#[doc = "Field `RFE` writer - 2:2\\]
Receive FIFO empty."]
pub type RfeW<'a, REG> = crate::BitWriter<'a, REG, Rfe>;
impl<'a, REG> RfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive FIFO is empty."]
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(Rfe::Empty)
    }
    #[doc = "Receive FIFO is not empty."]
    #[inline(always)]
    pub fn not_empty(self) -> &'a mut crate::W<REG> {
        self.variant(Rfe::NotEmpty)
    }
}
#[doc = "3:3\\]
Receive FIFO not full\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rnf {
    #[doc = "1: Receive FIFO is not full."]
    NotFull = 1,
    #[doc = "0: Receive FIFO is full."]
    Full = 0,
}
impl From<Rnf> for bool {
    #[inline(always)]
    fn from(variant: Rnf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNF` reader - 3:3\\]
Receive FIFO not full"]
pub type RnfR = crate::BitReader<Rnf>;
impl RnfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rnf {
        match self.bits {
            true => Rnf::NotFull,
            false => Rnf::Full,
        }
    }
    #[doc = "Receive FIFO is not full."]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == Rnf::NotFull
    }
    #[doc = "Receive FIFO is full."]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Rnf::Full
    }
}
#[doc = "Field `RNF` writer - 3:3\\]
Receive FIFO not full"]
pub type RnfW<'a, REG> = crate::BitWriter<'a, REG, Rnf>;
impl<'a, REG> RnfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive FIFO is not full."]
    #[inline(always)]
    pub fn not_full(self) -> &'a mut crate::W<REG> {
        self.variant(Rnf::NotFull)
    }
    #[doc = "Receive FIFO is full."]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(Rnf::Full)
    }
}
#[doc = "4:4\\]
Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busy {
    #[doc = "1: SPI is currently transmitting and/or receiving data, or transmit FIFO is not empty."]
    Active = 1,
    #[doc = "0: SPI is in idle mode."]
    Idle = 0,
}
impl From<Busy> for bool {
    #[inline(always)]
    fn from(variant: Busy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - 4:4\\]
Busy"]
pub type BusyR = crate::BitReader<Busy>;
impl BusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busy {
        match self.bits {
            true => Busy::Active,
            false => Busy::Idle,
        }
    }
    #[doc = "SPI is currently transmitting and/or receiving data, or transmit FIFO is not empty."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Busy::Active
    }
    #[doc = "SPI is in idle mode."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Busy::Idle
    }
}
#[doc = "Field `BUSY` writer - 4:4\\]
Busy"]
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG, Busy>;
impl<'a, REG> BusyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI is currently transmitting and/or receiving data, or transmit FIFO is not empty."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Busy::Active)
    }
    #[doc = "SPI is in idle mode."]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(Busy::Idle)
    }
}
#[doc = "Field `RESERVED5` reader - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved5R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED5` writer - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved5W<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Transmit FIFO empty."]
    #[inline(always)]
    pub fn tfe(&self) -> TfeR {
        TfeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Transmit FIFO not full"]
    #[inline(always)]
    pub fn tnf(&self) -> TnfR {
        TnfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Receive FIFO empty."]
    #[inline(always)]
    pub fn rfe(&self) -> RfeR {
        RfeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Receive FIFO not full"]
    #[inline(always)]
    pub fn rnf(&self) -> RnfR {
        RnfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Transmit FIFO empty."]
    #[inline(always)]
    #[must_use]
    pub fn tfe(&mut self) -> TfeW<StatSpec> {
        TfeW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Transmit FIFO not full"]
    #[inline(always)]
    #[must_use]
    pub fn tnf(&mut self) -> TnfW<StatSpec> {
        TnfW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Receive FIFO empty."]
    #[inline(always)]
    #[must_use]
    pub fn rfe(&mut self) -> RfeW<StatSpec> {
        RfeW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Receive FIFO not full"]
    #[inline(always)]
    #[must_use]
    pub fn rnf(&mut self) -> RnfW<StatSpec> {
        RnfW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Busy"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BusyW<StatSpec> {
        BusyW::new(self, 4)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> Reserved5W<StatSpec> {
        Reserved5W::new(self, 5)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for StatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT to value 0x0f"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0x0f;
}
