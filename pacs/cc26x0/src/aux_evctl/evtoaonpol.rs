#[doc = "Register `EVTOAONPOL` reader"]
pub struct R(crate::R<EVTOAONPOL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVTOAONPOL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVTOAONPOL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVTOAONPOL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVTOAONPOL` writer"]
pub struct W(crate::W<EVTOAONPOL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVTOAONPOL_SPEC>;
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
impl From<crate::W<EVTOAONPOL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVTOAONPOL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED2` reader - 2:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED2` writer - 2:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EVTOAONPOL_SPEC, u8, u8, 3, O>;
#[doc = "Field `AUX_COMPA` reader - 3:3\\]
Select the edge of EVSTAT0.AUX_COMPA that sets EVTOAONFLAGS.AUX_COMPA."]
pub type AUX_COMPA_R = crate::BitReader<AUX_COMPA_A>;
#[doc = "3:3\\]
Select the edge of EVSTAT0.AUX_COMPA that sets EVTOAONFLAGS.AUX_COMPA.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUX_COMPA_A {
    #[doc = "1: Falling edge"]
    LOW = 1,
    #[doc = "0: Rising edge"]
    HIGH = 0,
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
            true => AUX_COMPA_A::LOW,
            false => AUX_COMPA_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AUX_COMPA_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AUX_COMPA_A::HIGH
    }
}
#[doc = "Field `AUX_COMPA` writer - 3:3\\]
Select the edge of EVSTAT0.AUX_COMPA that sets EVTOAONFLAGS.AUX_COMPA."]
pub type AUX_COMPA_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVTOAONPOL_SPEC, AUX_COMPA_A, O>;
impl<'a, const O: u8> AUX_COMPA_W<'a, O> {
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_COMPA_A::LOW)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_COMPA_A::HIGH)
    }
}
#[doc = "Field `AUX_COMPB` reader - 4:4\\]
Select the edge of EVSTAT0.AUX_COMPB that sets EVTOAONFLAGS.AUX_COMPB."]
pub type AUX_COMPB_R = crate::BitReader<AUX_COMPB_A>;
#[doc = "4:4\\]
Select the edge of EVSTAT0.AUX_COMPB that sets EVTOAONFLAGS.AUX_COMPB.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUX_COMPB_A {
    #[doc = "1: Falling edge"]
    LOW = 1,
    #[doc = "0: Rising edge"]
    HIGH = 0,
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
            true => AUX_COMPB_A::LOW,
            false => AUX_COMPB_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AUX_COMPB_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AUX_COMPB_A::HIGH
    }
}
#[doc = "Field `AUX_COMPB` writer - 4:4\\]
Select the edge of EVSTAT0.AUX_COMPB that sets EVTOAONFLAGS.AUX_COMPB."]
pub type AUX_COMPB_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVTOAONPOL_SPEC, AUX_COMPB_A, O>;
impl<'a, const O: u8> AUX_COMPB_W<'a, O> {
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_COMPB_A::LOW)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_COMPB_A::HIGH)
    }
}
#[doc = "Field `ADC_DONE` reader - 5:5\\]
Select the level of EVSTAT0.ADC_DONE that sets EVTOAONFLAGS.ADC_DONE."]
pub type ADC_DONE_R = crate::BitReader<ADC_DONE_A>;
#[doc = "5:5\\]
Select the level of EVSTAT0.ADC_DONE that sets EVTOAONFLAGS.ADC_DONE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_DONE_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<ADC_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_DONE_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_DONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_DONE_A {
        match self.bits {
            true => ADC_DONE_A::LOW,
            false => ADC_DONE_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == ADC_DONE_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == ADC_DONE_A::HIGH
    }
}
#[doc = "Field `ADC_DONE` writer - 5:5\\]
Select the level of EVSTAT0.ADC_DONE that sets EVTOAONFLAGS.ADC_DONE."]
pub type ADC_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVTOAONPOL_SPEC, ADC_DONE_A, O>;
impl<'a, const O: u8> ADC_DONE_W<'a, O> {
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(ADC_DONE_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(ADC_DONE_A::HIGH)
    }
}
#[doc = "Field `TDC_DONE` reader - 6:6\\]
Select level of EVSTAT0.TDC_DONE that sets EVTOAONFLAGS.TDC_DONE."]
pub type TDC_DONE_R = crate::BitReader<TDC_DONE_A>;
#[doc = "6:6\\]
Select level of EVSTAT0.TDC_DONE that sets EVTOAONFLAGS.TDC_DONE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDC_DONE_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<TDC_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: TDC_DONE_A) -> Self {
        variant as u8 != 0
    }
}
impl TDC_DONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDC_DONE_A {
        match self.bits {
            true => TDC_DONE_A::LOW,
            false => TDC_DONE_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == TDC_DONE_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == TDC_DONE_A::HIGH
    }
}
#[doc = "Field `TDC_DONE` writer - 6:6\\]
Select level of EVSTAT0.TDC_DONE that sets EVTOAONFLAGS.TDC_DONE."]
pub type TDC_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVTOAONPOL_SPEC, TDC_DONE_A, O>;
impl<'a, const O: u8> TDC_DONE_W<'a, O> {
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(TDC_DONE_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(TDC_DONE_A::HIGH)
    }
}
#[doc = "Field `TIMER0_EV` reader - 7:7\\]
Select the level of EVSTAT0.TIMER0_EV that sets EVTOAONFLAGS.TIMER0_EV."]
pub type TIMER0_EV_R = crate::BitReader<TIMER0_EV_A>;
#[doc = "7:7\\]
Select the level of EVSTAT0.TIMER0_EV that sets EVTOAONFLAGS.TIMER0_EV.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMER0_EV_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<TIMER0_EV_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER0_EV_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMER0_EV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER0_EV_A {
        match self.bits {
            true => TIMER0_EV_A::LOW,
            false => TIMER0_EV_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == TIMER0_EV_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == TIMER0_EV_A::HIGH
    }
}
#[doc = "Field `TIMER0_EV` writer - 7:7\\]
Select the level of EVSTAT0.TIMER0_EV that sets EVTOAONFLAGS.TIMER0_EV."]
pub type TIMER0_EV_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVTOAONPOL_SPEC, TIMER0_EV_A, O>;
impl<'a, const O: u8> TIMER0_EV_W<'a, O> {
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(TIMER0_EV_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(TIMER0_EV_A::HIGH)
    }
}
#[doc = "Field `TIMER1_EV` reader - 8:8\\]
Select the level of EVSTAT0.TIMER1_EV that sets EVTOAONFLAGS.TIMER1_EV."]
pub type TIMER1_EV_R = crate::BitReader<TIMER1_EV_A>;
#[doc = "8:8\\]
Select the level of EVSTAT0.TIMER1_EV that sets EVTOAONFLAGS.TIMER1_EV.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMER1_EV_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<TIMER1_EV_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER1_EV_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMER1_EV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER1_EV_A {
        match self.bits {
            true => TIMER1_EV_A::LOW,
            false => TIMER1_EV_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == TIMER1_EV_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == TIMER1_EV_A::HIGH
    }
}
#[doc = "Field `TIMER1_EV` writer - 8:8\\]
Select the level of EVSTAT0.TIMER1_EV that sets EVTOAONFLAGS.TIMER1_EV."]
pub type TIMER1_EV_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVTOAONPOL_SPEC, TIMER1_EV_A, O>;
impl<'a, const O: u8> TIMER1_EV_W<'a, O> {
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(TIMER1_EV_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(TIMER1_EV_A::HIGH)
    }
}
#[doc = "Field `RESERVED9` reader - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED9` writer - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EVTOAONPOL_SPEC, u32, u32, 23, O>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
Select the edge of EVSTAT0.AUX_COMPA that sets EVTOAONFLAGS.AUX_COMPA."]
    #[inline(always)]
    pub fn aux_compa(&self) -> AUX_COMPA_R {
        AUX_COMPA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Select the edge of EVSTAT0.AUX_COMPB that sets EVTOAONFLAGS.AUX_COMPB."]
    #[inline(always)]
    pub fn aux_compb(&self) -> AUX_COMPB_R {
        AUX_COMPB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Select the level of EVSTAT0.ADC_DONE that sets EVTOAONFLAGS.ADC_DONE."]
    #[inline(always)]
    pub fn adc_done(&self) -> ADC_DONE_R {
        ADC_DONE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Select level of EVSTAT0.TDC_DONE that sets EVTOAONFLAGS.TDC_DONE."]
    #[inline(always)]
    pub fn tdc_done(&self) -> TDC_DONE_R {
        TDC_DONE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Select the level of EVSTAT0.TIMER0_EV that sets EVTOAONFLAGS.TIMER0_EV."]
    #[inline(always)]
    pub fn timer0_ev(&self) -> TIMER0_EV_R {
        TIMER0_EV_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Select the level of EVSTAT0.TIMER1_EV that sets EVTOAONFLAGS.TIMER1_EV."]
    #[inline(always)]
    pub fn timer1_ev(&self) -> TIMER1_EV_R {
        TIMER1_EV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<0> {
        RESERVED2_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Select the edge of EVSTAT0.AUX_COMPA that sets EVTOAONFLAGS.AUX_COMPA."]
    #[inline(always)]
    #[must_use]
    pub fn aux_compa(&mut self) -> AUX_COMPA_W<3> {
        AUX_COMPA_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Select the edge of EVSTAT0.AUX_COMPB that sets EVTOAONFLAGS.AUX_COMPB."]
    #[inline(always)]
    #[must_use]
    pub fn aux_compb(&mut self) -> AUX_COMPB_W<4> {
        AUX_COMPB_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Select the level of EVSTAT0.ADC_DONE that sets EVTOAONFLAGS.ADC_DONE."]
    #[inline(always)]
    #[must_use]
    pub fn adc_done(&mut self) -> ADC_DONE_W<5> {
        ADC_DONE_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Select level of EVSTAT0.TDC_DONE that sets EVTOAONFLAGS.TDC_DONE."]
    #[inline(always)]
    #[must_use]
    pub fn tdc_done(&mut self) -> TDC_DONE_W<6> {
        TDC_DONE_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Select the level of EVSTAT0.TIMER0_EV that sets EVTOAONFLAGS.TIMER0_EV."]
    #[inline(always)]
    #[must_use]
    pub fn timer0_ev(&mut self) -> TIMER0_EV_W<7> {
        TIMER0_EV_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Select the level of EVSTAT0.TIMER1_EV that sets EVTOAONFLAGS.TIMER1_EV."]
    #[inline(always)]
    #[must_use]
    pub fn timer1_ev(&mut self) -> TIMER1_EV_W<8> {
        TIMER1_EV_W::new(self)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> RESERVED9_W<9> {
        RESERVED9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Events To AON Polarity Event source polarity configuration for EVTOAONFLAGS.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evtoaonpol](index.html) module"]
pub struct EVTOAONPOL_SPEC;
impl crate::RegisterSpec for EVTOAONPOL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evtoaonpol::R](R) reader structure"]
impl crate::Readable for EVTOAONPOL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evtoaonpol::W](W) writer structure"]
impl crate::Writable for EVTOAONPOL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVTOAONPOL to value 0"]
impl crate::Resettable for EVTOAONPOL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
