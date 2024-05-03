#[doc = "Register `EVTOMCUPOL` reader"]
pub type R = crate::R<EvtomcupolSpec>;
#[doc = "Register `EVTOMCUPOL` writer"]
pub type W = crate::W<EvtomcupolSpec>;
#[doc = "0:0\\]
Select the event source level that sets EVTOMCUFLAGS.AON_WU_EV.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AonWuEv {
    #[doc = "1: Low level"]
    Low = 1,
    #[doc = "0: High level"]
    High = 0,
}
impl From<AonWuEv> for bool {
    #[inline(always)]
    fn from(variant: AonWuEv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AON_WU_EV` reader - 0:0\\]
Select the event source level that sets EVTOMCUFLAGS.AON_WU_EV."]
pub type AonWuEvR = crate::BitReader<AonWuEv>;
impl AonWuEvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AonWuEv {
        match self.bits {
            true => AonWuEv::Low,
            false => AonWuEv::High,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AonWuEv::Low
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AonWuEv::High
    }
}
#[doc = "Field `AON_WU_EV` writer - 0:0\\]
Select the event source level that sets EVTOMCUFLAGS.AON_WU_EV."]
pub type AonWuEvW<'a, REG> = crate::BitWriter<'a, REG, AonWuEv>;
impl<'a, REG> AonWuEvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(AonWuEv::Low)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(AonWuEv::High)
    }
}
#[doc = "1:1\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_COMPA.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuxCompa {
    #[doc = "1: Low level"]
    Low = 1,
    #[doc = "0: High level"]
    High = 0,
}
impl From<AuxCompa> for bool {
    #[inline(always)]
    fn from(variant: AuxCompa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUX_COMPA` reader - 1:1\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_COMPA."]
pub type AuxCompaR = crate::BitReader<AuxCompa>;
impl AuxCompaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AuxCompa {
        match self.bits {
            true => AuxCompa::Low,
            false => AuxCompa::High,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AuxCompa::Low
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AuxCompa::High
    }
}
#[doc = "Field `AUX_COMPA` writer - 1:1\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_COMPA."]
pub type AuxCompaW<'a, REG> = crate::BitWriter<'a, REG, AuxCompa>;
impl<'a, REG> AuxCompaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(AuxCompa::Low)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(AuxCompa::High)
    }
}
#[doc = "2:2\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_COMPB.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuxCompb {
    #[doc = "1: Low level"]
    Low = 1,
    #[doc = "0: High level"]
    High = 0,
}
impl From<AuxCompb> for bool {
    #[inline(always)]
    fn from(variant: AuxCompb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUX_COMPB` reader - 2:2\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_COMPB."]
pub type AuxCompbR = crate::BitReader<AuxCompb>;
impl AuxCompbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AuxCompb {
        match self.bits {
            true => AuxCompb::Low,
            false => AuxCompb::High,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AuxCompb::Low
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AuxCompb::High
    }
}
#[doc = "Field `AUX_COMPB` writer - 2:2\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_COMPB."]
pub type AuxCompbW<'a, REG> = crate::BitWriter<'a, REG, AuxCompb>;
impl<'a, REG> AuxCompbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(AuxCompb::Low)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(AuxCompb::High)
    }
}
#[doc = "3:3\\]
Select the event source level that sets EVTOMCUFLAGS.TDC_DONE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TdcDone {
    #[doc = "1: Low level"]
    Low = 1,
    #[doc = "0: High level"]
    High = 0,
}
impl From<TdcDone> for bool {
    #[inline(always)]
    fn from(variant: TdcDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDC_DONE` reader - 3:3\\]
Select the event source level that sets EVTOMCUFLAGS.TDC_DONE."]
pub type TdcDoneR = crate::BitReader<TdcDone>;
impl TdcDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TdcDone {
        match self.bits {
            true => TdcDone::Low,
            false => TdcDone::High,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == TdcDone::Low
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == TdcDone::High
    }
}
#[doc = "Field `TDC_DONE` writer - 3:3\\]
Select the event source level that sets EVTOMCUFLAGS.TDC_DONE."]
pub type TdcDoneW<'a, REG> = crate::BitWriter<'a, REG, TdcDone>;
impl<'a, REG> TdcDoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(TdcDone::Low)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(TdcDone::High)
    }
}
#[doc = "4:4\\]
Select the event source level that sets EVTOMCUFLAGS.TIMER0_EV.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer0Ev {
    #[doc = "1: Low level"]
    Low = 1,
    #[doc = "0: High level"]
    High = 0,
}
impl From<Timer0Ev> for bool {
    #[inline(always)]
    fn from(variant: Timer0Ev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER0_EV` reader - 4:4\\]
Select the event source level that sets EVTOMCUFLAGS.TIMER0_EV."]
pub type Timer0EvR = crate::BitReader<Timer0Ev>;
impl Timer0EvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer0Ev {
        match self.bits {
            true => Timer0Ev::Low,
            false => Timer0Ev::High,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Timer0Ev::Low
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Timer0Ev::High
    }
}
#[doc = "Field `TIMER0_EV` writer - 4:4\\]
Select the event source level that sets EVTOMCUFLAGS.TIMER0_EV."]
pub type Timer0EvW<'a, REG> = crate::BitWriter<'a, REG, Timer0Ev>;
impl<'a, REG> Timer0EvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0Ev::Low)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0Ev::High)
    }
}
#[doc = "5:5\\]
Select the event source level that sets EVTOMCUFLAGS.TIMER1_EV.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer1Ev {
    #[doc = "1: Low level"]
    Low = 1,
    #[doc = "0: High level"]
    High = 0,
}
impl From<Timer1Ev> for bool {
    #[inline(always)]
    fn from(variant: Timer1Ev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER1_EV` reader - 5:5\\]
Select the event source level that sets EVTOMCUFLAGS.TIMER1_EV."]
pub type Timer1EvR = crate::BitReader<Timer1Ev>;
impl Timer1EvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer1Ev {
        match self.bits {
            true => Timer1Ev::Low,
            false => Timer1Ev::High,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Timer1Ev::Low
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Timer1Ev::High
    }
}
#[doc = "Field `TIMER1_EV` writer - 5:5\\]
Select the event source level that sets EVTOMCUFLAGS.TIMER1_EV."]
pub type Timer1EvW<'a, REG> = crate::BitWriter<'a, REG, Timer1Ev>;
impl<'a, REG> Timer1EvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Timer1Ev::Low)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Timer1Ev::High)
    }
}
#[doc = "6:6\\]
Select the event source level that sets EVTOMCUFLAGS.SMPH_AUTOTAKE_DONE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SmphAutotakeDone {
    #[doc = "1: Low level"]
    Low = 1,
    #[doc = "0: High level"]
    High = 0,
}
impl From<SmphAutotakeDone> for bool {
    #[inline(always)]
    fn from(variant: SmphAutotakeDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMPH_AUTOTAKE_DONE` reader - 6:6\\]
Select the event source level that sets EVTOMCUFLAGS.SMPH_AUTOTAKE_DONE."]
pub type SmphAutotakeDoneR = crate::BitReader<SmphAutotakeDone>;
impl SmphAutotakeDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SmphAutotakeDone {
        match self.bits {
            true => SmphAutotakeDone::Low,
            false => SmphAutotakeDone::High,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SmphAutotakeDone::Low
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SmphAutotakeDone::High
    }
}
#[doc = "Field `SMPH_AUTOTAKE_DONE` writer - 6:6\\]
Select the event source level that sets EVTOMCUFLAGS.SMPH_AUTOTAKE_DONE."]
pub type SmphAutotakeDoneW<'a, REG> = crate::BitWriter<'a, REG, SmphAutotakeDone>;
impl<'a, REG> SmphAutotakeDoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(SmphAutotakeDone::Low)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(SmphAutotakeDone::High)
    }
}
#[doc = "7:7\\]
Select the event source level that sets EVTOMCUFLAGS.ADC_DONE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdcDone {
    #[doc = "1: Low level"]
    Low = 1,
    #[doc = "0: High level"]
    High = 0,
}
impl From<AdcDone> for bool {
    #[inline(always)]
    fn from(variant: AdcDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC_DONE` reader - 7:7\\]
Select the event source level that sets EVTOMCUFLAGS.ADC_DONE."]
pub type AdcDoneR = crate::BitReader<AdcDone>;
impl AdcDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AdcDone {
        match self.bits {
            true => AdcDone::Low,
            false => AdcDone::High,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AdcDone::Low
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AdcDone::High
    }
}
#[doc = "Field `ADC_DONE` writer - 7:7\\]
Select the event source level that sets EVTOMCUFLAGS.ADC_DONE."]
pub type AdcDoneW<'a, REG> = crate::BitWriter<'a, REG, AdcDone>;
impl<'a, REG> AdcDoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(AdcDone::Low)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(AdcDone::High)
    }
}
#[doc = "8:8\\]
Select the event source level that sets EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdcFifoAlmostFull {
    #[doc = "1: Low level"]
    Low = 1,
    #[doc = "0: High level"]
    High = 0,
}
impl From<AdcFifoAlmostFull> for bool {
    #[inline(always)]
    fn from(variant: AdcFifoAlmostFull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC_FIFO_ALMOST_FULL` reader - 8:8\\]
Select the event source level that sets EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL."]
pub type AdcFifoAlmostFullR = crate::BitReader<AdcFifoAlmostFull>;
impl AdcFifoAlmostFullR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AdcFifoAlmostFull {
        match self.bits {
            true => AdcFifoAlmostFull::Low,
            false => AdcFifoAlmostFull::High,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AdcFifoAlmostFull::Low
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AdcFifoAlmostFull::High
    }
}
#[doc = "Field `ADC_FIFO_ALMOST_FULL` writer - 8:8\\]
Select the event source level that sets EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL."]
pub type AdcFifoAlmostFullW<'a, REG> = crate::BitWriter<'a, REG, AdcFifoAlmostFull>;
impl<'a, REG> AdcFifoAlmostFullW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(AdcFifoAlmostFull::Low)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(AdcFifoAlmostFull::High)
    }
}
#[doc = "9:9\\]
Select the event source level that sets EVTOMCUFLAGS.OBSMUX0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Obsmux0 {
    #[doc = "1: Low level"]
    Low = 1,
    #[doc = "0: High level"]
    High = 0,
}
impl From<Obsmux0> for bool {
    #[inline(always)]
    fn from(variant: Obsmux0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OBSMUX0` reader - 9:9\\]
Select the event source level that sets EVTOMCUFLAGS.OBSMUX0."]
pub type Obsmux0R = crate::BitReader<Obsmux0>;
impl Obsmux0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Obsmux0 {
        match self.bits {
            true => Obsmux0::Low,
            false => Obsmux0::High,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Obsmux0::Low
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Obsmux0::High
    }
}
#[doc = "Field `OBSMUX0` writer - 9:9\\]
Select the event source level that sets EVTOMCUFLAGS.OBSMUX0."]
pub type Obsmux0W<'a, REG> = crate::BitWriter<'a, REG, Obsmux0>;
impl<'a, REG> Obsmux0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Obsmux0::Low)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Obsmux0::High)
    }
}
#[doc = "10:10\\]
Select the event source level that sets EVTOMCUFLAGS.ADC_IRQ.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdcIrq {
    #[doc = "1: Low level"]
    Low = 1,
    #[doc = "0: High level"]
    High = 0,
}
impl From<AdcIrq> for bool {
    #[inline(always)]
    fn from(variant: AdcIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC_IRQ` reader - 10:10\\]
Select the event source level that sets EVTOMCUFLAGS.ADC_IRQ."]
pub type AdcIrqR = crate::BitReader<AdcIrq>;
impl AdcIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AdcIrq {
        match self.bits {
            true => AdcIrq::Low,
            false => AdcIrq::High,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AdcIrq::Low
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AdcIrq::High
    }
}
#[doc = "Field `ADC_IRQ` writer - 10:10\\]
Select the event source level that sets EVTOMCUFLAGS.ADC_IRQ."]
pub type AdcIrqW<'a, REG> = crate::BitWriter<'a, REG, AdcIrq>;
impl<'a, REG> AdcIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(AdcIrq::Low)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(AdcIrq::High)
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
Select the event source level that sets EVTOMCUFLAGS.AON_WU_EV."]
    #[inline(always)]
    pub fn aon_wu_ev(&self) -> AonWuEvR {
        AonWuEvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_COMPA."]
    #[inline(always)]
    pub fn aux_compa(&self) -> AuxCompaR {
        AuxCompaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_COMPB."]
    #[inline(always)]
    pub fn aux_compb(&self) -> AuxCompbR {
        AuxCompbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Select the event source level that sets EVTOMCUFLAGS.TDC_DONE."]
    #[inline(always)]
    pub fn tdc_done(&self) -> TdcDoneR {
        TdcDoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Select the event source level that sets EVTOMCUFLAGS.TIMER0_EV."]
    #[inline(always)]
    pub fn timer0_ev(&self) -> Timer0EvR {
        Timer0EvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Select the event source level that sets EVTOMCUFLAGS.TIMER1_EV."]
    #[inline(always)]
    pub fn timer1_ev(&self) -> Timer1EvR {
        Timer1EvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Select the event source level that sets EVTOMCUFLAGS.SMPH_AUTOTAKE_DONE."]
    #[inline(always)]
    pub fn smph_autotake_done(&self) -> SmphAutotakeDoneR {
        SmphAutotakeDoneR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Select the event source level that sets EVTOMCUFLAGS.ADC_DONE."]
    #[inline(always)]
    pub fn adc_done(&self) -> AdcDoneR {
        AdcDoneR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Select the event source level that sets EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL."]
    #[inline(always)]
    pub fn adc_fifo_almost_full(&self) -> AdcFifoAlmostFullR {
        AdcFifoAlmostFullR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Select the event source level that sets EVTOMCUFLAGS.OBSMUX0."]
    #[inline(always)]
    pub fn obsmux0(&self) -> Obsmux0R {
        Obsmux0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Select the event source level that sets EVTOMCUFLAGS.ADC_IRQ."]
    #[inline(always)]
    pub fn adc_irq(&self) -> AdcIrqR {
        AdcIrqR::new(((self.bits >> 10) & 1) != 0)
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
Select the event source level that sets EVTOMCUFLAGS.AON_WU_EV."]
    #[inline(always)]
    #[must_use]
    pub fn aon_wu_ev(&mut self) -> AonWuEvW<EvtomcupolSpec> {
        AonWuEvW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_COMPA."]
    #[inline(always)]
    #[must_use]
    pub fn aux_compa(&mut self) -> AuxCompaW<EvtomcupolSpec> {
        AuxCompaW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_COMPB."]
    #[inline(always)]
    #[must_use]
    pub fn aux_compb(&mut self) -> AuxCompbW<EvtomcupolSpec> {
        AuxCompbW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Select the event source level that sets EVTOMCUFLAGS.TDC_DONE."]
    #[inline(always)]
    #[must_use]
    pub fn tdc_done(&mut self) -> TdcDoneW<EvtomcupolSpec> {
        TdcDoneW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Select the event source level that sets EVTOMCUFLAGS.TIMER0_EV."]
    #[inline(always)]
    #[must_use]
    pub fn timer0_ev(&mut self) -> Timer0EvW<EvtomcupolSpec> {
        Timer0EvW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Select the event source level that sets EVTOMCUFLAGS.TIMER1_EV."]
    #[inline(always)]
    #[must_use]
    pub fn timer1_ev(&mut self) -> Timer1EvW<EvtomcupolSpec> {
        Timer1EvW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Select the event source level that sets EVTOMCUFLAGS.SMPH_AUTOTAKE_DONE."]
    #[inline(always)]
    #[must_use]
    pub fn smph_autotake_done(&mut self) -> SmphAutotakeDoneW<EvtomcupolSpec> {
        SmphAutotakeDoneW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Select the event source level that sets EVTOMCUFLAGS.ADC_DONE."]
    #[inline(always)]
    #[must_use]
    pub fn adc_done(&mut self) -> AdcDoneW<EvtomcupolSpec> {
        AdcDoneW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Select the event source level that sets EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL."]
    #[inline(always)]
    #[must_use]
    pub fn adc_fifo_almost_full(&mut self) -> AdcFifoAlmostFullW<EvtomcupolSpec> {
        AdcFifoAlmostFullW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Select the event source level that sets EVTOMCUFLAGS.OBSMUX0."]
    #[inline(always)]
    #[must_use]
    pub fn obsmux0(&mut self) -> Obsmux0W<EvtomcupolSpec> {
        Obsmux0W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Select the event source level that sets EVTOMCUFLAGS.ADC_IRQ."]
    #[inline(always)]
    #[must_use]
    pub fn adc_irq(&mut self) -> AdcIrqW<EvtomcupolSpec> {
        AdcIrqW::new(self, 10)
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
