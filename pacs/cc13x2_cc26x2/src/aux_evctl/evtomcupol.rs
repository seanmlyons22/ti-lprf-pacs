#[doc = "Register `EVTOMCUPOL` reader"]
pub struct R(crate::R<EVTOMCUPOL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVTOMCUPOL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVTOMCUPOL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVTOMCUPOL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVTOMCUPOL` writer"]
pub struct W(crate::W<EVTOMCUPOL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVTOMCUPOL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<EVTOMCUPOL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVTOMCUPOL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUX_WU_EV` reader - 0:0\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_WU_EV."]
pub type AUX_WU_EV_R = crate::BitReader<AUX_WU_EV_A>;
#[doc = "0:0\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_WU_EV.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUX_WU_EV_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<AUX_WU_EV_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_WU_EV_A) -> Self {
        variant as u8 != 0
    }
}
impl AUX_WU_EV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_WU_EV_A {
        match self.bits {
            true => AUX_WU_EV_A::LOW,
            false => AUX_WU_EV_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AUX_WU_EV_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AUX_WU_EV_A::HIGH
    }
}
#[doc = "Field `AUX_WU_EV` writer - 0:0\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_WU_EV."]
pub type AUX_WU_EV_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVTOMCUPOL_SPEC, AUX_WU_EV_A, O>;
impl<'a, const O: u8> AUX_WU_EV_W<'a, O> {
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_WU_EV_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_WU_EV_A::HIGH)
    }
}
#[doc = "Field `AUX_COMPA` reader - 1:1\\]
Select the event source edge that sets EVTOMCUFLAGS.AUX_COMPA."]
pub type AUX_COMPA_R = crate::BitReader<AUX_COMPA_A>;
#[doc = "1:1\\]
Select the event source edge that sets EVTOMCUFLAGS.AUX_COMPA.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUX_COMPA_A {
    #[doc = "1: Falling edge"]
    FALL = 1,
    #[doc = "0: Rising edge"]
    RISE = 0,
}
impl From<AUX_COMPA_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_COMPA_A) -> Self {
        variant as u8 != 0
    }
}
impl AUX_COMPA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_COMPA_A {
        match self.bits {
            true => AUX_COMPA_A::FALL,
            false => AUX_COMPA_A::RISE,
        }
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == AUX_COMPA_A::FALL
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == AUX_COMPA_A::RISE
    }
}
#[doc = "Field `AUX_COMPA` writer - 1:1\\]
Select the event source edge that sets EVTOMCUFLAGS.AUX_COMPA."]
pub type AUX_COMPA_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVTOMCUPOL_SPEC, AUX_COMPA_A, O>;
impl<'a, const O: u8> AUX_COMPA_W<'a, O> {
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(AUX_COMPA_A::FALL)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(AUX_COMPA_A::RISE)
    }
}
#[doc = "Field `AUX_COMPB` reader - 2:2\\]
Select the event source edge that sets EVTOMCUFLAGS.AUX_COMPB."]
pub type AUX_COMPB_R = crate::BitReader<AUX_COMPB_A>;
#[doc = "2:2\\]
Select the event source edge that sets EVTOMCUFLAGS.AUX_COMPB.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUX_COMPB_A {
    #[doc = "1: Falling edge"]
    FALL = 1,
    #[doc = "0: Rising edge"]
    RISE = 0,
}
impl From<AUX_COMPB_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_COMPB_A) -> Self {
        variant as u8 != 0
    }
}
impl AUX_COMPB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_COMPB_A {
        match self.bits {
            true => AUX_COMPB_A::FALL,
            false => AUX_COMPB_A::RISE,
        }
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == AUX_COMPB_A::FALL
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == AUX_COMPB_A::RISE
    }
}
#[doc = "Field `AUX_COMPB` writer - 2:2\\]
Select the event source edge that sets EVTOMCUFLAGS.AUX_COMPB."]
pub type AUX_COMPB_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVTOMCUPOL_SPEC, AUX_COMPB_A, O>;
impl<'a, const O: u8> AUX_COMPB_W<'a, O> {
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(AUX_COMPB_A::FALL)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(AUX_COMPB_A::RISE)
    }
}
#[doc = "Field `AUX_TDC_DONE` reader - 3:3\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TDC_DONE."]
pub type AUX_TDC_DONE_R = crate::BitReader<AUX_TDC_DONE_A>;
#[doc = "3:3\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TDC_DONE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUX_TDC_DONE_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<AUX_TDC_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_TDC_DONE_A) -> Self {
        variant as u8 != 0
    }
}
impl AUX_TDC_DONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_TDC_DONE_A {
        match self.bits {
            true => AUX_TDC_DONE_A::LOW,
            false => AUX_TDC_DONE_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AUX_TDC_DONE_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AUX_TDC_DONE_A::HIGH
    }
}
#[doc = "Field `AUX_TDC_DONE` writer - 3:3\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TDC_DONE."]
pub type AUX_TDC_DONE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVTOMCUPOL_SPEC, AUX_TDC_DONE_A, O>;
impl<'a, const O: u8> AUX_TDC_DONE_W<'a, O> {
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_TDC_DONE_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_TDC_DONE_A::HIGH)
    }
}
#[doc = "Field `AUX_TIMER0_EV` reader - 4:4\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER0_EV."]
pub type AUX_TIMER0_EV_R = crate::BitReader<AUX_TIMER0_EV_A>;
#[doc = "4:4\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER0_EV.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUX_TIMER0_EV_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<AUX_TIMER0_EV_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_TIMER0_EV_A) -> Self {
        variant as u8 != 0
    }
}
impl AUX_TIMER0_EV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_TIMER0_EV_A {
        match self.bits {
            true => AUX_TIMER0_EV_A::LOW,
            false => AUX_TIMER0_EV_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AUX_TIMER0_EV_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AUX_TIMER0_EV_A::HIGH
    }
}
#[doc = "Field `AUX_TIMER0_EV` writer - 4:4\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER0_EV."]
pub type AUX_TIMER0_EV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVTOMCUPOL_SPEC, AUX_TIMER0_EV_A, O>;
impl<'a, const O: u8> AUX_TIMER0_EV_W<'a, O> {
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_TIMER0_EV_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_TIMER0_EV_A::HIGH)
    }
}
#[doc = "Field `AUX_TIMER1_EV` reader - 5:5\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER1_EV."]
pub type AUX_TIMER1_EV_R = crate::BitReader<AUX_TIMER1_EV_A>;
#[doc = "5:5\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER1_EV.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUX_TIMER1_EV_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<AUX_TIMER1_EV_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_TIMER1_EV_A) -> Self {
        variant as u8 != 0
    }
}
impl AUX_TIMER1_EV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_TIMER1_EV_A {
        match self.bits {
            true => AUX_TIMER1_EV_A::LOW,
            false => AUX_TIMER1_EV_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AUX_TIMER1_EV_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AUX_TIMER1_EV_A::HIGH
    }
}
#[doc = "Field `AUX_TIMER1_EV` writer - 5:5\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER1_EV."]
pub type AUX_TIMER1_EV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVTOMCUPOL_SPEC, AUX_TIMER1_EV_A, O>;
impl<'a, const O: u8> AUX_TIMER1_EV_W<'a, O> {
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_TIMER1_EV_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_TIMER1_EV_A::HIGH)
    }
}
#[doc = "Field `AUX_SMPH_AUTOTAKE_DONE` reader - 6:6\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_SMPH_AUTOTAKE_DONE."]
pub type AUX_SMPH_AUTOTAKE_DONE_R = crate::BitReader<AUX_SMPH_AUTOTAKE_DONE_A>;
#[doc = "6:6\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_SMPH_AUTOTAKE_DONE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUX_SMPH_AUTOTAKE_DONE_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<AUX_SMPH_AUTOTAKE_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_SMPH_AUTOTAKE_DONE_A) -> Self {
        variant as u8 != 0
    }
}
impl AUX_SMPH_AUTOTAKE_DONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_SMPH_AUTOTAKE_DONE_A {
        match self.bits {
            true => AUX_SMPH_AUTOTAKE_DONE_A::LOW,
            false => AUX_SMPH_AUTOTAKE_DONE_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AUX_SMPH_AUTOTAKE_DONE_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AUX_SMPH_AUTOTAKE_DONE_A::HIGH
    }
}
#[doc = "Field `AUX_SMPH_AUTOTAKE_DONE` writer - 6:6\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_SMPH_AUTOTAKE_DONE."]
pub type AUX_SMPH_AUTOTAKE_DONE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVTOMCUPOL_SPEC, AUX_SMPH_AUTOTAKE_DONE_A, O>;
impl<'a, const O: u8> AUX_SMPH_AUTOTAKE_DONE_W<'a, O> {
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_SMPH_AUTOTAKE_DONE_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_SMPH_AUTOTAKE_DONE_A::HIGH)
    }
}
#[doc = "Field `AUX_ADC_DONE` reader - 7:7\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_DONE."]
pub type AUX_ADC_DONE_R = crate::BitReader<AUX_ADC_DONE_A>;
#[doc = "7:7\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_DONE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUX_ADC_DONE_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<AUX_ADC_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_ADC_DONE_A) -> Self {
        variant as u8 != 0
    }
}
impl AUX_ADC_DONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_ADC_DONE_A {
        match self.bits {
            true => AUX_ADC_DONE_A::LOW,
            false => AUX_ADC_DONE_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AUX_ADC_DONE_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AUX_ADC_DONE_A::HIGH
    }
}
#[doc = "Field `AUX_ADC_DONE` writer - 7:7\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_DONE."]
pub type AUX_ADC_DONE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVTOMCUPOL_SPEC, AUX_ADC_DONE_A, O>;
impl<'a, const O: u8> AUX_ADC_DONE_W<'a, O> {
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_ADC_DONE_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_ADC_DONE_A::HIGH)
    }
}
#[doc = "Field `AUX_ADC_FIFO_ALMOST_FULL` reader - 8:8\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_FIFO_ALMOST_FULL."]
pub type AUX_ADC_FIFO_ALMOST_FULL_R = crate::BitReader<AUX_ADC_FIFO_ALMOST_FULL_A>;
#[doc = "8:8\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_FIFO_ALMOST_FULL.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUX_ADC_FIFO_ALMOST_FULL_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<AUX_ADC_FIFO_ALMOST_FULL_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_ADC_FIFO_ALMOST_FULL_A) -> Self {
        variant as u8 != 0
    }
}
impl AUX_ADC_FIFO_ALMOST_FULL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_ADC_FIFO_ALMOST_FULL_A {
        match self.bits {
            true => AUX_ADC_FIFO_ALMOST_FULL_A::LOW,
            false => AUX_ADC_FIFO_ALMOST_FULL_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AUX_ADC_FIFO_ALMOST_FULL_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AUX_ADC_FIFO_ALMOST_FULL_A::HIGH
    }
}
#[doc = "Field `AUX_ADC_FIFO_ALMOST_FULL` writer - 8:8\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_FIFO_ALMOST_FULL."]
pub type AUX_ADC_FIFO_ALMOST_FULL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVTOMCUPOL_SPEC, AUX_ADC_FIFO_ALMOST_FULL_A, O>;
impl<'a, const O: u8> AUX_ADC_FIFO_ALMOST_FULL_W<'a, O> {
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_ADC_FIFO_ALMOST_FULL_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_ADC_FIFO_ALMOST_FULL_A::HIGH)
    }
}
#[doc = "Field `MCU_OBSMUX0` reader - 9:9\\]
Select the event source level that sets EVTOMCUFLAGS.MCU_OBSMUX0."]
pub type MCU_OBSMUX0_R = crate::BitReader<MCU_OBSMUX0_A>;
#[doc = "9:9\\]
Select the event source level that sets EVTOMCUFLAGS.MCU_OBSMUX0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCU_OBSMUX0_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<MCU_OBSMUX0_A> for bool {
    #[inline(always)]
    fn from(variant: MCU_OBSMUX0_A) -> Self {
        variant as u8 != 0
    }
}
impl MCU_OBSMUX0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCU_OBSMUX0_A {
        match self.bits {
            true => MCU_OBSMUX0_A::LOW,
            false => MCU_OBSMUX0_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == MCU_OBSMUX0_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == MCU_OBSMUX0_A::HIGH
    }
}
#[doc = "Field `MCU_OBSMUX0` writer - 9:9\\]
Select the event source level that sets EVTOMCUFLAGS.MCU_OBSMUX0."]
pub type MCU_OBSMUX0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVTOMCUPOL_SPEC, MCU_OBSMUX0_A, O>;
impl<'a, const O: u8> MCU_OBSMUX0_W<'a, O> {
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(MCU_OBSMUX0_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(MCU_OBSMUX0_A::HIGH)
    }
}
#[doc = "Field `AUX_ADC_IRQ` reader - 10:10\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_IRQ."]
pub type AUX_ADC_IRQ_R = crate::BitReader<AUX_ADC_IRQ_A>;
#[doc = "10:10\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_IRQ.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUX_ADC_IRQ_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<AUX_ADC_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_ADC_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl AUX_ADC_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_ADC_IRQ_A {
        match self.bits {
            true => AUX_ADC_IRQ_A::LOW,
            false => AUX_ADC_IRQ_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AUX_ADC_IRQ_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AUX_ADC_IRQ_A::HIGH
    }
}
#[doc = "Field `AUX_ADC_IRQ` writer - 10:10\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_IRQ."]
pub type AUX_ADC_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVTOMCUPOL_SPEC, AUX_ADC_IRQ_A, O>;
impl<'a, const O: u8> AUX_ADC_IRQ_W<'a, O> {
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_ADC_IRQ_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_ADC_IRQ_A::HIGH)
    }
}
#[doc = "Field `AUX_TIMER2_EV0` reader - 11:11\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV0."]
pub type AUX_TIMER2_EV0_R = crate::BitReader<AUX_TIMER2_EV0_A>;
#[doc = "11:11\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUX_TIMER2_EV0_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<AUX_TIMER2_EV0_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_TIMER2_EV0_A) -> Self {
        variant as u8 != 0
    }
}
impl AUX_TIMER2_EV0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_TIMER2_EV0_A {
        match self.bits {
            true => AUX_TIMER2_EV0_A::LOW,
            false => AUX_TIMER2_EV0_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AUX_TIMER2_EV0_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AUX_TIMER2_EV0_A::HIGH
    }
}
#[doc = "Field `AUX_TIMER2_EV0` writer - 11:11\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV0."]
pub type AUX_TIMER2_EV0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVTOMCUPOL_SPEC, AUX_TIMER2_EV0_A, O>;
impl<'a, const O: u8> AUX_TIMER2_EV0_W<'a, O> {
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_TIMER2_EV0_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_TIMER2_EV0_A::HIGH)
    }
}
#[doc = "Field `AUX_TIMER2_EV1` reader - 12:12\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV1."]
pub type AUX_TIMER2_EV1_R = crate::BitReader<AUX_TIMER2_EV1_A>;
#[doc = "12:12\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUX_TIMER2_EV1_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<AUX_TIMER2_EV1_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_TIMER2_EV1_A) -> Self {
        variant as u8 != 0
    }
}
impl AUX_TIMER2_EV1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_TIMER2_EV1_A {
        match self.bits {
            true => AUX_TIMER2_EV1_A::LOW,
            false => AUX_TIMER2_EV1_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AUX_TIMER2_EV1_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AUX_TIMER2_EV1_A::HIGH
    }
}
#[doc = "Field `AUX_TIMER2_EV1` writer - 12:12\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV1."]
pub type AUX_TIMER2_EV1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVTOMCUPOL_SPEC, AUX_TIMER2_EV1_A, O>;
impl<'a, const O: u8> AUX_TIMER2_EV1_W<'a, O> {
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_TIMER2_EV1_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_TIMER2_EV1_A::HIGH)
    }
}
#[doc = "Field `AUX_TIMER2_EV2` reader - 13:13\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV2."]
pub type AUX_TIMER2_EV2_R = crate::BitReader<AUX_TIMER2_EV2_A>;
#[doc = "13:13\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUX_TIMER2_EV2_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<AUX_TIMER2_EV2_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_TIMER2_EV2_A) -> Self {
        variant as u8 != 0
    }
}
impl AUX_TIMER2_EV2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_TIMER2_EV2_A {
        match self.bits {
            true => AUX_TIMER2_EV2_A::LOW,
            false => AUX_TIMER2_EV2_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AUX_TIMER2_EV2_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AUX_TIMER2_EV2_A::HIGH
    }
}
#[doc = "Field `AUX_TIMER2_EV2` writer - 13:13\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV2."]
pub type AUX_TIMER2_EV2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVTOMCUPOL_SPEC, AUX_TIMER2_EV2_A, O>;
impl<'a, const O: u8> AUX_TIMER2_EV2_W<'a, O> {
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_TIMER2_EV2_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_TIMER2_EV2_A::HIGH)
    }
}
#[doc = "Field `AUX_TIMER2_EV3` reader - 14:14\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV3."]
pub type AUX_TIMER2_EV3_R = crate::BitReader<AUX_TIMER2_EV3_A>;
#[doc = "14:14\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUX_TIMER2_EV3_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<AUX_TIMER2_EV3_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_TIMER2_EV3_A) -> Self {
        variant as u8 != 0
    }
}
impl AUX_TIMER2_EV3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_TIMER2_EV3_A {
        match self.bits {
            true => AUX_TIMER2_EV3_A::LOW,
            false => AUX_TIMER2_EV3_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AUX_TIMER2_EV3_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AUX_TIMER2_EV3_A::HIGH
    }
}
#[doc = "Field `AUX_TIMER2_EV3` writer - 14:14\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV3."]
pub type AUX_TIMER2_EV3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVTOMCUPOL_SPEC, AUX_TIMER2_EV3_A, O>;
impl<'a, const O: u8> AUX_TIMER2_EV3_W<'a, O> {
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_TIMER2_EV3_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_TIMER2_EV3_A::HIGH)
    }
}
#[doc = "Field `AUX_TIMER2_PULSE` reader - 15:15\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_PULSE."]
pub type AUX_TIMER2_PULSE_R = crate::BitReader<AUX_TIMER2_PULSE_A>;
#[doc = "15:15\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_PULSE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUX_TIMER2_PULSE_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<AUX_TIMER2_PULSE_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_TIMER2_PULSE_A) -> Self {
        variant as u8 != 0
    }
}
impl AUX_TIMER2_PULSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_TIMER2_PULSE_A {
        match self.bits {
            true => AUX_TIMER2_PULSE_A::LOW,
            false => AUX_TIMER2_PULSE_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AUX_TIMER2_PULSE_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AUX_TIMER2_PULSE_A::HIGH
    }
}
#[doc = "Field `AUX_TIMER2_PULSE` writer - 15:15\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_PULSE."]
pub type AUX_TIMER2_PULSE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVTOMCUPOL_SPEC, AUX_TIMER2_PULSE_A, O>;
impl<'a, const O: u8> AUX_TIMER2_PULSE_W<'a, O> {
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_TIMER2_PULSE_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_TIMER2_PULSE_A::HIGH)
    }
}
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EVTOMCUPOL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_WU_EV."]
    #[inline(always)]
    pub fn aux_wu_ev(&self) -> AUX_WU_EV_R {
        AUX_WU_EV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Select the event source edge that sets EVTOMCUFLAGS.AUX_COMPA."]
    #[inline(always)]
    pub fn aux_compa(&self) -> AUX_COMPA_R {
        AUX_COMPA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Select the event source edge that sets EVTOMCUFLAGS.AUX_COMPB."]
    #[inline(always)]
    pub fn aux_compb(&self) -> AUX_COMPB_R {
        AUX_COMPB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TDC_DONE."]
    #[inline(always)]
    pub fn aux_tdc_done(&self) -> AUX_TDC_DONE_R {
        AUX_TDC_DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER0_EV."]
    #[inline(always)]
    pub fn aux_timer0_ev(&self) -> AUX_TIMER0_EV_R {
        AUX_TIMER0_EV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER1_EV."]
    #[inline(always)]
    pub fn aux_timer1_ev(&self) -> AUX_TIMER1_EV_R {
        AUX_TIMER1_EV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_SMPH_AUTOTAKE_DONE."]
    #[inline(always)]
    pub fn aux_smph_autotake_done(&self) -> AUX_SMPH_AUTOTAKE_DONE_R {
        AUX_SMPH_AUTOTAKE_DONE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_DONE."]
    #[inline(always)]
    pub fn aux_adc_done(&self) -> AUX_ADC_DONE_R {
        AUX_ADC_DONE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_FIFO_ALMOST_FULL."]
    #[inline(always)]
    pub fn aux_adc_fifo_almost_full(&self) -> AUX_ADC_FIFO_ALMOST_FULL_R {
        AUX_ADC_FIFO_ALMOST_FULL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Select the event source level that sets EVTOMCUFLAGS.MCU_OBSMUX0."]
    #[inline(always)]
    pub fn mcu_obsmux0(&self) -> MCU_OBSMUX0_R {
        MCU_OBSMUX0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_IRQ."]
    #[inline(always)]
    pub fn aux_adc_irq(&self) -> AUX_ADC_IRQ_R {
        AUX_ADC_IRQ_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV0."]
    #[inline(always)]
    pub fn aux_timer2_ev0(&self) -> AUX_TIMER2_EV0_R {
        AUX_TIMER2_EV0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV1."]
    #[inline(always)]
    pub fn aux_timer2_ev1(&self) -> AUX_TIMER2_EV1_R {
        AUX_TIMER2_EV1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV2."]
    #[inline(always)]
    pub fn aux_timer2_ev2(&self) -> AUX_TIMER2_EV2_R {
        AUX_TIMER2_EV2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV3."]
    #[inline(always)]
    pub fn aux_timer2_ev3(&self) -> AUX_TIMER2_EV3_R {
        AUX_TIMER2_EV3_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_PULSE."]
    #[inline(always)]
    pub fn aux_timer2_pulse(&self) -> AUX_TIMER2_PULSE_R {
        AUX_TIMER2_PULSE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_WU_EV."]
    #[inline(always)]
    #[must_use]
    pub fn aux_wu_ev(&mut self) -> AUX_WU_EV_W<0> {
        AUX_WU_EV_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Select the event source edge that sets EVTOMCUFLAGS.AUX_COMPA."]
    #[inline(always)]
    #[must_use]
    pub fn aux_compa(&mut self) -> AUX_COMPA_W<1> {
        AUX_COMPA_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Select the event source edge that sets EVTOMCUFLAGS.AUX_COMPB."]
    #[inline(always)]
    #[must_use]
    pub fn aux_compb(&mut self) -> AUX_COMPB_W<2> {
        AUX_COMPB_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TDC_DONE."]
    #[inline(always)]
    #[must_use]
    pub fn aux_tdc_done(&mut self) -> AUX_TDC_DONE_W<3> {
        AUX_TDC_DONE_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER0_EV."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer0_ev(&mut self) -> AUX_TIMER0_EV_W<4> {
        AUX_TIMER0_EV_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER1_EV."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer1_ev(&mut self) -> AUX_TIMER1_EV_W<5> {
        AUX_TIMER1_EV_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_SMPH_AUTOTAKE_DONE."]
    #[inline(always)]
    #[must_use]
    pub fn aux_smph_autotake_done(&mut self) -> AUX_SMPH_AUTOTAKE_DONE_W<6> {
        AUX_SMPH_AUTOTAKE_DONE_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_DONE."]
    #[inline(always)]
    #[must_use]
    pub fn aux_adc_done(&mut self) -> AUX_ADC_DONE_W<7> {
        AUX_ADC_DONE_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_FIFO_ALMOST_FULL."]
    #[inline(always)]
    #[must_use]
    pub fn aux_adc_fifo_almost_full(&mut self) -> AUX_ADC_FIFO_ALMOST_FULL_W<8> {
        AUX_ADC_FIFO_ALMOST_FULL_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Select the event source level that sets EVTOMCUFLAGS.MCU_OBSMUX0."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_obsmux0(&mut self) -> MCU_OBSMUX0_W<9> {
        MCU_OBSMUX0_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_IRQ."]
    #[inline(always)]
    #[must_use]
    pub fn aux_adc_irq(&mut self) -> AUX_ADC_IRQ_W<10> {
        AUX_ADC_IRQ_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV0."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer2_ev0(&mut self) -> AUX_TIMER2_EV0_W<11> {
        AUX_TIMER2_EV0_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV1."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer2_ev1(&mut self) -> AUX_TIMER2_EV1_W<12> {
        AUX_TIMER2_EV1_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV2."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer2_ev2(&mut self) -> AUX_TIMER2_EV2_W<13> {
        AUX_TIMER2_EV2_W::new(self)
    }
    #[doc = "Bit 14 - 14:14\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV3."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer2_ev3(&mut self) -> AUX_TIMER2_EV3_W<14> {
        AUX_TIMER2_EV3_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_PULSE."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer2_pulse(&mut self) -> AUX_TIMER2_PULSE_W<15> {
        AUX_TIMER2_PULSE_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> RESERVED16_W<16> {
        RESERVED16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event To MCU Polarity Event source polarity configuration for EVTOMCUFLAGS.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evtomcupol](index.html) module"]
pub struct EVTOMCUPOL_SPEC;
impl crate::RegisterSpec for EVTOMCUPOL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evtomcupol::R](R) reader structure"]
impl crate::Readable for EVTOMCUPOL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evtomcupol::W](W) writer structure"]
impl crate::Writable for EVTOMCUPOL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVTOMCUPOL to value 0"]
impl crate::Resettable for EVTOMCUPOL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
