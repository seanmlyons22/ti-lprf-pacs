#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "0:0\\]
Transmit FIFO empty status.\n\nValue on reset: 1"]
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
Transmit FIFO empty status."]
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
Transmit FIFO empty status."]
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
Transmit FIFO not full status.\n\nValue on reset: 1"]
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
Transmit FIFO not full status."]
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
Transmit FIFO not full status."]
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
Receive FIFO empty status.\n\nValue on reset: 1"]
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
Receive FIFO empty status."]
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
Receive FIFO empty status."]
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
Receive FIFO not full status.\n\nValue on reset: 1"]
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
Receive FIFO not full status."]
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
Receive FIFO not full status."]
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
SPI Busy status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busy {
    #[doc = "1: SPI is currently transmitting and/or recieving data, or transmit FIFO is not empty."]
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
SPI Busy status"]
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
    #[doc = "SPI is currently transmitting and/or recieving data, or transmit FIFO is not empty."]
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
SPI Busy status"]
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG, Busy>;
impl<'a, REG> BusyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI is currently transmitting and/or recieving data, or transmit FIFO is not empty."]
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
#[doc = "5:5\\]
Detection of CS deassertion in the middle of a word transmission results in this error being set. This feature is only available in the peripheral mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Csd {
    #[doc = "1: An error is generated when CS posedge (deassertion) is detected before the entire word is transmitted."]
    Err = 1,
    #[doc = "0: No CS posedge is detected before the entire word has been transmitted."]
    Noerr = 0,
}
impl From<Csd> for bool {
    #[inline(always)]
    fn from(variant: Csd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSD` reader - 5:5\\]
Detection of CS deassertion in the middle of a word transmission results in this error being set. This feature is only available in the peripheral mode."]
pub type CsdR = crate::BitReader<Csd>;
impl CsdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Csd {
        match self.bits {
            true => Csd::Err,
            false => Csd::Noerr,
        }
    }
    #[doc = "An error is generated when CS posedge (deassertion) is detected before the entire word is transmitted."]
    #[inline(always)]
    pub fn is_err(&self) -> bool {
        *self == Csd::Err
    }
    #[doc = "No CS posedge is detected before the entire word has been transmitted."]
    #[inline(always)]
    pub fn is_noerr(&self) -> bool {
        *self == Csd::Noerr
    }
}
#[doc = "Field `CSD` writer - 5:5\\]
Detection of CS deassertion in the middle of a word transmission results in this error being set. This feature is only available in the peripheral mode."]
pub type CsdW<'a, REG> = crate::BitWriter<'a, REG, Csd>;
impl<'a, REG> CsdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "An error is generated when CS posedge (deassertion) is detected before the entire word is transmitted."]
    #[inline(always)]
    pub fn err(self) -> &'a mut crate::W<REG> {
        self.variant(Csd::Err)
    }
    #[doc = "No CS posedge is detected before the entire word has been transmitted."]
    #[inline(always)]
    pub fn noerr(self) -> &'a mut crate::W<REG> {
        self.variant(Csd::Noerr)
    }
}
#[doc = "6:6\\]
Transmit done. Indicates whether the last bit left the Shift register after a transmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransDone {
    #[doc = "1: Last bit has been shifted out, and the transmission is done"]
    TransmitDone = 1,
    #[doc = "0: Last bit has not yet left the Shift register, and the transmission is ongoing."]
    TransmitOngoing = 0,
}
impl From<TransDone> for bool {
    #[inline(always)]
    fn from(variant: TransDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRANS_DONE` reader - 6:6\\]
Transmit done. Indicates whether the last bit left the Shift register after a transmission"]
pub type TransDoneR = crate::BitReader<TransDone>;
impl TransDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TransDone {
        match self.bits {
            true => TransDone::TransmitDone,
            false => TransDone::TransmitOngoing,
        }
    }
    #[doc = "Last bit has been shifted out, and the transmission is done"]
    #[inline(always)]
    pub fn is_transmit_done(&self) -> bool {
        *self == TransDone::TransmitDone
    }
    #[doc = "Last bit has not yet left the Shift register, and the transmission is ongoing."]
    #[inline(always)]
    pub fn is_transmit_ongoing(&self) -> bool {
        *self == TransDone::TransmitOngoing
    }
}
#[doc = "Field `TRANS_DONE` writer - 6:6\\]
Transmit done. Indicates whether the last bit left the Shift register after a transmission"]
pub type TransDoneW<'a, REG> = crate::BitWriter<'a, REG, TransDone>;
impl<'a, REG> TransDoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Last bit has been shifted out, and the transmission is done"]
    #[inline(always)]
    pub fn transmit_done(self) -> &'a mut crate::W<REG> {
        self.variant(TransDone::TransmitDone)
    }
    #[doc = "Last bit has not yet left the Shift register, and the transmission is ongoing."]
    #[inline(always)]
    pub fn transmit_ongoing(self) -> &'a mut crate::W<REG> {
        self.variant(TransDone::TransmitOngoing)
    }
}
#[doc = "Field `RESERVED7` reader - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `RESERVED7` writer - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFO_FILL_LVL` reader - 13:8\\]
Indicates how many locations of TXFIFO is currently filled with data"]
pub type TxfifoFillLvlR = crate::FieldReader;
#[doc = "Field `TXFIFO_FILL_LVL` writer - 13:8\\]
Indicates how many locations of TXFIFO is currently filled with data"]
pub type TxfifoFillLvlW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RESERVED14` reader - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved14R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED14` writer - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved14W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Transmit FIFO empty status."]
    #[inline(always)]
    pub fn tfe(&self) -> TfeR {
        TfeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Transmit FIFO not full status."]
    #[inline(always)]
    pub fn tnf(&self) -> TnfR {
        TnfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Receive FIFO empty status."]
    #[inline(always)]
    pub fn rfe(&self) -> RfeR {
        RfeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Receive FIFO not full status."]
    #[inline(always)]
    pub fn rnf(&self) -> RnfR {
        RnfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
SPI Busy status"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Detection of CS deassertion in the middle of a word transmission results in this error being set. This feature is only available in the peripheral mode."]
    #[inline(always)]
    pub fn csd(&self) -> CsdR {
        CsdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Transmit done. Indicates whether the last bit left the Shift register after a transmission"]
    #[inline(always)]
    pub fn trans_done(&self) -> TransDoneR {
        TransDoneR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Indicates how many locations of TXFIFO is currently filled with data"]
    #[inline(always)]
    pub fn txfifo_fill_lvl(&self) -> TxfifoFillLvlR {
        TxfifoFillLvlR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&self) -> Reserved14R {
        Reserved14R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Transmit FIFO empty status."]
    #[inline(always)]
    #[must_use]
    pub fn tfe(&mut self) -> TfeW<StatSpec> {
        TfeW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Transmit FIFO not full status."]
    #[inline(always)]
    #[must_use]
    pub fn tnf(&mut self) -> TnfW<StatSpec> {
        TnfW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Receive FIFO empty status."]
    #[inline(always)]
    #[must_use]
    pub fn rfe(&mut self) -> RfeW<StatSpec> {
        RfeW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Receive FIFO not full status."]
    #[inline(always)]
    #[must_use]
    pub fn rnf(&mut self) -> RnfW<StatSpec> {
        RnfW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
SPI Busy status"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BusyW<StatSpec> {
        BusyW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Detection of CS deassertion in the middle of a word transmission results in this error being set. This feature is only available in the peripheral mode."]
    #[inline(always)]
    #[must_use]
    pub fn csd(&mut self) -> CsdW<StatSpec> {
        CsdW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Transmit done. Indicates whether the last bit left the Shift register after a transmission"]
    #[inline(always)]
    #[must_use]
    pub fn trans_done(&mut self) -> TransDoneW<StatSpec> {
        TransDoneW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> Reserved7W<StatSpec> {
        Reserved7W::new(self, 7)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Indicates how many locations of TXFIFO is currently filled with data"]
    #[inline(always)]
    #[must_use]
    pub fn txfifo_fill_lvl(&mut self) -> TxfifoFillLvlW<StatSpec> {
        TxfifoFillLvlW::new(self, 8)
    }
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved14(&mut self) -> Reserved14W<StatSpec> {
        Reserved14W::new(self, 14)
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
