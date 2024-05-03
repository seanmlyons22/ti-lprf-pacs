#[doc = "Register `EVTOMCUPOL` reader"]
pub type R = crate::R<EvtomcupolSpec>;
#[doc = "Register `EVTOMCUPOL` writer"]
pub type W = crate::W<EvtomcupolSpec>;
#[doc = "Field `RESERVED0` reader - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::BitReader;
#[doc = "Field `RESERVED0` writer - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "1:1\\]
Select the event source edge that sets EVTOMCUFLAGS.AUX_COMPA.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuxCompa {
    #[doc = "1: Falling edge"]
    Fall = 1,
    #[doc = "0: Rising edge"]
    Rise = 0,
}
impl From<AuxCompa> for bool {
    #[inline(always)]
    fn from(variant: AuxCompa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUX_COMPA` reader - 1:1\\]
Select the event source edge that sets EVTOMCUFLAGS.AUX_COMPA."]
pub type AuxCompaR = crate::BitReader<AuxCompa>;
impl AuxCompaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AuxCompa {
        match self.bits {
            true => AuxCompa::Fall,
            false => AuxCompa::Rise,
        }
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == AuxCompa::Fall
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == AuxCompa::Rise
    }
}
#[doc = "Field `AUX_COMPA` writer - 1:1\\]
Select the event source edge that sets EVTOMCUFLAGS.AUX_COMPA."]
pub type AuxCompaW<'a, REG> = crate::BitWriter<'a, REG, AuxCompa>;
impl<'a, REG> AuxCompaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(AuxCompa::Fall)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(AuxCompa::Rise)
    }
}
#[doc = "2:2\\]
Select the event source edge that sets EVTOMCUFLAGS.AUX_COMPB.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuxCompb {
    #[doc = "1: Falling edge"]
    Fall = 1,
    #[doc = "0: Rising edge"]
    Rise = 0,
}
impl From<AuxCompb> for bool {
    #[inline(always)]
    fn from(variant: AuxCompb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUX_COMPB` reader - 2:2\\]
Select the event source edge that sets EVTOMCUFLAGS.AUX_COMPB."]
pub type AuxCompbR = crate::BitReader<AuxCompb>;
impl AuxCompbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AuxCompb {
        match self.bits {
            true => AuxCompb::Fall,
            false => AuxCompb::Rise,
        }
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == AuxCompb::Fall
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == AuxCompb::Rise
    }
}
#[doc = "Field `AUX_COMPB` writer - 2:2\\]
Select the event source edge that sets EVTOMCUFLAGS.AUX_COMPB."]
pub type AuxCompbW<'a, REG> = crate::BitWriter<'a, REG, AuxCompb>;
impl<'a, REG> AuxCompbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(AuxCompb::Fall)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(AuxCompb::Rise)
    }
}
#[doc = "3:3\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TDC_DONE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuxTdcDone {
    #[doc = "1: Low level"]
    Low = 1,
    #[doc = "0: High level"]
    High = 0,
}
impl From<AuxTdcDone> for bool {
    #[inline(always)]
    fn from(variant: AuxTdcDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUX_TDC_DONE` reader - 3:3\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TDC_DONE."]
pub type AuxTdcDoneR = crate::BitReader<AuxTdcDone>;
impl AuxTdcDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AuxTdcDone {
        match self.bits {
            true => AuxTdcDone::Low,
            false => AuxTdcDone::High,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AuxTdcDone::Low
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AuxTdcDone::High
    }
}
#[doc = "Field `AUX_TDC_DONE` writer - 3:3\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TDC_DONE."]
pub type AuxTdcDoneW<'a, REG> = crate::BitWriter<'a, REG, AuxTdcDone>;
impl<'a, REG> AuxTdcDoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(AuxTdcDone::Low)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(AuxTdcDone::High)
    }
}
#[doc = "4:4\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER0_EV.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuxTimer0Ev {
    #[doc = "1: Low level"]
    Low = 1,
    #[doc = "0: High level"]
    High = 0,
}
impl From<AuxTimer0Ev> for bool {
    #[inline(always)]
    fn from(variant: AuxTimer0Ev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUX_TIMER0_EV` reader - 4:4\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER0_EV."]
pub type AuxTimer0EvR = crate::BitReader<AuxTimer0Ev>;
impl AuxTimer0EvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AuxTimer0Ev {
        match self.bits {
            true => AuxTimer0Ev::Low,
            false => AuxTimer0Ev::High,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AuxTimer0Ev::Low
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AuxTimer0Ev::High
    }
}
#[doc = "Field `AUX_TIMER0_EV` writer - 4:4\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER0_EV."]
pub type AuxTimer0EvW<'a, REG> = crate::BitWriter<'a, REG, AuxTimer0Ev>;
impl<'a, REG> AuxTimer0EvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(AuxTimer0Ev::Low)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(AuxTimer0Ev::High)
    }
}
#[doc = "5:5\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER1_EV.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuxTimer1Ev {
    #[doc = "1: Low level"]
    Low = 1,
    #[doc = "0: High level"]
    High = 0,
}
impl From<AuxTimer1Ev> for bool {
    #[inline(always)]
    fn from(variant: AuxTimer1Ev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUX_TIMER1_EV` reader - 5:5\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER1_EV."]
pub type AuxTimer1EvR = crate::BitReader<AuxTimer1Ev>;
impl AuxTimer1EvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AuxTimer1Ev {
        match self.bits {
            true => AuxTimer1Ev::Low,
            false => AuxTimer1Ev::High,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AuxTimer1Ev::Low
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AuxTimer1Ev::High
    }
}
#[doc = "Field `AUX_TIMER1_EV` writer - 5:5\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER1_EV."]
pub type AuxTimer1EvW<'a, REG> = crate::BitWriter<'a, REG, AuxTimer1Ev>;
impl<'a, REG> AuxTimer1EvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(AuxTimer1Ev::Low)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(AuxTimer1Ev::High)
    }
}
#[doc = "6:6\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_SMPH_AUTOTAKE_DONE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuxSmphAutotakeDone {
    #[doc = "1: Low level"]
    Low = 1,
    #[doc = "0: High level"]
    High = 0,
}
impl From<AuxSmphAutotakeDone> for bool {
    #[inline(always)]
    fn from(variant: AuxSmphAutotakeDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUX_SMPH_AUTOTAKE_DONE` reader - 6:6\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_SMPH_AUTOTAKE_DONE."]
pub type AuxSmphAutotakeDoneR = crate::BitReader<AuxSmphAutotakeDone>;
impl AuxSmphAutotakeDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AuxSmphAutotakeDone {
        match self.bits {
            true => AuxSmphAutotakeDone::Low,
            false => AuxSmphAutotakeDone::High,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AuxSmphAutotakeDone::Low
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AuxSmphAutotakeDone::High
    }
}
#[doc = "Field `AUX_SMPH_AUTOTAKE_DONE` writer - 6:6\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_SMPH_AUTOTAKE_DONE."]
pub type AuxSmphAutotakeDoneW<'a, REG> = crate::BitWriter<'a, REG, AuxSmphAutotakeDone>;
impl<'a, REG> AuxSmphAutotakeDoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(AuxSmphAutotakeDone::Low)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(AuxSmphAutotakeDone::High)
    }
}
#[doc = "7:7\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_DONE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuxAdcDone {
    #[doc = "1: Low level"]
    Low = 1,
    #[doc = "0: High level"]
    High = 0,
}
impl From<AuxAdcDone> for bool {
    #[inline(always)]
    fn from(variant: AuxAdcDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUX_ADC_DONE` reader - 7:7\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_DONE."]
pub type AuxAdcDoneR = crate::BitReader<AuxAdcDone>;
impl AuxAdcDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AuxAdcDone {
        match self.bits {
            true => AuxAdcDone::Low,
            false => AuxAdcDone::High,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AuxAdcDone::Low
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AuxAdcDone::High
    }
}
#[doc = "Field `AUX_ADC_DONE` writer - 7:7\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_DONE."]
pub type AuxAdcDoneW<'a, REG> = crate::BitWriter<'a, REG, AuxAdcDone>;
impl<'a, REG> AuxAdcDoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(AuxAdcDone::Low)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(AuxAdcDone::High)
    }
}
#[doc = "8:8\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_FIFO_ALMOST_FULL.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuxAdcFifoAlmostFull {
    #[doc = "1: Low level"]
    Low = 1,
    #[doc = "0: High level"]
    High = 0,
}
impl From<AuxAdcFifoAlmostFull> for bool {
    #[inline(always)]
    fn from(variant: AuxAdcFifoAlmostFull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUX_ADC_FIFO_ALMOST_FULL` reader - 8:8\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_FIFO_ALMOST_FULL."]
pub type AuxAdcFifoAlmostFullR = crate::BitReader<AuxAdcFifoAlmostFull>;
impl AuxAdcFifoAlmostFullR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AuxAdcFifoAlmostFull {
        match self.bits {
            true => AuxAdcFifoAlmostFull::Low,
            false => AuxAdcFifoAlmostFull::High,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AuxAdcFifoAlmostFull::Low
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AuxAdcFifoAlmostFull::High
    }
}
#[doc = "Field `AUX_ADC_FIFO_ALMOST_FULL` writer - 8:8\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_FIFO_ALMOST_FULL."]
pub type AuxAdcFifoAlmostFullW<'a, REG> = crate::BitWriter<'a, REG, AuxAdcFifoAlmostFull>;
impl<'a, REG> AuxAdcFifoAlmostFullW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(AuxAdcFifoAlmostFull::Low)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(AuxAdcFifoAlmostFull::High)
    }
}
#[doc = "9:9\\]
Select the event source level that sets EVTOMCUFLAGS.MCU_OBSMUX0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McuObsmux0 {
    #[doc = "1: Low level"]
    Low = 1,
    #[doc = "0: High level"]
    High = 0,
}
impl From<McuObsmux0> for bool {
    #[inline(always)]
    fn from(variant: McuObsmux0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCU_OBSMUX0` reader - 9:9\\]
Select the event source level that sets EVTOMCUFLAGS.MCU_OBSMUX0."]
pub type McuObsmux0R = crate::BitReader<McuObsmux0>;
impl McuObsmux0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McuObsmux0 {
        match self.bits {
            true => McuObsmux0::Low,
            false => McuObsmux0::High,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == McuObsmux0::Low
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == McuObsmux0::High
    }
}
#[doc = "Field `MCU_OBSMUX0` writer - 9:9\\]
Select the event source level that sets EVTOMCUFLAGS.MCU_OBSMUX0."]
pub type McuObsmux0W<'a, REG> = crate::BitWriter<'a, REG, McuObsmux0>;
impl<'a, REG> McuObsmux0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(McuObsmux0::Low)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(McuObsmux0::High)
    }
}
#[doc = "10:10\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_IRQ.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuxAdcIrq {
    #[doc = "1: Low level"]
    Low = 1,
    #[doc = "0: High level"]
    High = 0,
}
impl From<AuxAdcIrq> for bool {
    #[inline(always)]
    fn from(variant: AuxAdcIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUX_ADC_IRQ` reader - 10:10\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_IRQ."]
pub type AuxAdcIrqR = crate::BitReader<AuxAdcIrq>;
impl AuxAdcIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AuxAdcIrq {
        match self.bits {
            true => AuxAdcIrq::Low,
            false => AuxAdcIrq::High,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AuxAdcIrq::Low
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AuxAdcIrq::High
    }
}
#[doc = "Field `AUX_ADC_IRQ` writer - 10:10\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_IRQ."]
pub type AuxAdcIrqW<'a, REG> = crate::BitWriter<'a, REG, AuxAdcIrq>;
impl<'a, REG> AuxAdcIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(AuxAdcIrq::Low)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(AuxAdcIrq::High)
    }
}
#[doc = "Field `RESERVED11` reader - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED11` writer - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Select the event source edge that sets EVTOMCUFLAGS.AUX_COMPA."]
    #[inline(always)]
    pub fn aux_compa(&self) -> AuxCompaR {
        AuxCompaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Select the event source edge that sets EVTOMCUFLAGS.AUX_COMPB."]
    #[inline(always)]
    pub fn aux_compb(&self) -> AuxCompbR {
        AuxCompbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TDC_DONE."]
    #[inline(always)]
    pub fn aux_tdc_done(&self) -> AuxTdcDoneR {
        AuxTdcDoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER0_EV."]
    #[inline(always)]
    pub fn aux_timer0_ev(&self) -> AuxTimer0EvR {
        AuxTimer0EvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER1_EV."]
    #[inline(always)]
    pub fn aux_timer1_ev(&self) -> AuxTimer1EvR {
        AuxTimer1EvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_SMPH_AUTOTAKE_DONE."]
    #[inline(always)]
    pub fn aux_smph_autotake_done(&self) -> AuxSmphAutotakeDoneR {
        AuxSmphAutotakeDoneR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_DONE."]
    #[inline(always)]
    pub fn aux_adc_done(&self) -> AuxAdcDoneR {
        AuxAdcDoneR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_FIFO_ALMOST_FULL."]
    #[inline(always)]
    pub fn aux_adc_fifo_almost_full(&self) -> AuxAdcFifoAlmostFullR {
        AuxAdcFifoAlmostFullR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Select the event source level that sets EVTOMCUFLAGS.MCU_OBSMUX0."]
    #[inline(always)]
    pub fn mcu_obsmux0(&self) -> McuObsmux0R {
        McuObsmux0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_IRQ."]
    #[inline(always)]
    pub fn aux_adc_irq(&self) -> AuxAdcIrqR {
        AuxAdcIrqR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<EvtomcupolSpec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Select the event source edge that sets EVTOMCUFLAGS.AUX_COMPA."]
    #[inline(always)]
    #[must_use]
    pub fn aux_compa(&mut self) -> AuxCompaW<EvtomcupolSpec> {
        AuxCompaW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Select the event source edge that sets EVTOMCUFLAGS.AUX_COMPB."]
    #[inline(always)]
    #[must_use]
    pub fn aux_compb(&mut self) -> AuxCompbW<EvtomcupolSpec> {
        AuxCompbW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TDC_DONE."]
    #[inline(always)]
    #[must_use]
    pub fn aux_tdc_done(&mut self) -> AuxTdcDoneW<EvtomcupolSpec> {
        AuxTdcDoneW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER0_EV."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer0_ev(&mut self) -> AuxTimer0EvW<EvtomcupolSpec> {
        AuxTimer0EvW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER1_EV."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer1_ev(&mut self) -> AuxTimer1EvW<EvtomcupolSpec> {
        AuxTimer1EvW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_SMPH_AUTOTAKE_DONE."]
    #[inline(always)]
    #[must_use]
    pub fn aux_smph_autotake_done(&mut self) -> AuxSmphAutotakeDoneW<EvtomcupolSpec> {
        AuxSmphAutotakeDoneW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_DONE."]
    #[inline(always)]
    #[must_use]
    pub fn aux_adc_done(&mut self) -> AuxAdcDoneW<EvtomcupolSpec> {
        AuxAdcDoneW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_FIFO_ALMOST_FULL."]
    #[inline(always)]
    #[must_use]
    pub fn aux_adc_fifo_almost_full(&mut self) -> AuxAdcFifoAlmostFullW<EvtomcupolSpec> {
        AuxAdcFifoAlmostFullW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Select the event source level that sets EVTOMCUFLAGS.MCU_OBSMUX0."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_obsmux0(&mut self) -> McuObsmux0W<EvtomcupolSpec> {
        McuObsmux0W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_IRQ."]
    #[inline(always)]
    #[must_use]
    pub fn aux_adc_irq(&mut self) -> AuxAdcIrqW<EvtomcupolSpec> {
        AuxAdcIrqW::new(self, 10)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved11(&mut self) -> Reserved11W<EvtomcupolSpec> {
        Reserved11W::new(self, 11)
    }
}
#[doc = "Event To MCU Polarity Event source polarity configuration for EVTOMCUFLAGS.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evtomcupol::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evtomcupol::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvtomcupolSpec;
impl crate::RegisterSpec for EvtomcupolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evtomcupol::R`](R) reader structure"]
impl crate::Readable for EvtomcupolSpec {}
#[doc = "`write(|w| ..)` method takes [`evtomcupol::W`](W) writer structure"]
impl crate::Writable for EvtomcupolSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVTOMCUPOL to value 0"]
impl crate::Resettable for EvtomcupolSpec {
    const RESET_VALUE: u32 = 0;
}
