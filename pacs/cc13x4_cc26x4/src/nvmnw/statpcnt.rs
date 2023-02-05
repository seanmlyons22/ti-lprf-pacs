#[doc = "Register `STATPCNT` reader"]
pub struct R(crate::R<STATPCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATPCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATPCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATPCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATPCNT` writer"]
pub struct W(crate::W<STATPCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATPCNT_SPEC>;
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
impl From<crate::W<STATPCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATPCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PULSECNT` reader - 11:0\\]
Current Pulse Counter Value"]
pub type PULSECNT_R = crate::FieldReader<u16, PULSECNT_A>;
#[doc = "11:0\\]
Current Pulse Counter Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PULSECNT_A {
    #[doc = "4095: Maximum value"]
    MAXIMUM = 4095,
    #[doc = "0: Minimum value"]
    MINIMUM = 0,
}
impl From<PULSECNT_A> for u16 {
    #[inline(always)]
    fn from(variant: PULSECNT_A) -> Self {
        variant as _
    }
}
impl PULSECNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PULSECNT_A> {
        match self.bits {
            4095 => Some(PULSECNT_A::MAXIMUM),
            0 => Some(PULSECNT_A::MINIMUM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAXIMUM`"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == PULSECNT_A::MAXIMUM
    }
    #[doc = "Checks if the value of the field is `MINIMUM`"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == PULSECNT_A::MINIMUM
    }
}
#[doc = "Field `PULSECNT` writer - 11:0\\]
Current Pulse Counter Value"]
pub type PULSECNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STATPCNT_SPEC, u16, PULSECNT_A, 12, O>;
impl<'a, const O: u8> PULSECNT_W<'a, O> {
    #[doc = "Maximum value"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut W {
        self.variant(PULSECNT_A::MAXIMUM)
    }
    #[doc = "Minimum value"]
    #[inline(always)]
    pub fn minimum(self) -> &'a mut W {
        self.variant(PULSECNT_A::MINIMUM)
    }
}
#[doc = "Field `RESERVED12` reader - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED12_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED12` writer - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED12_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STATPCNT_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Current Pulse Counter Value"]
    #[inline(always)]
    pub fn pulsecnt(&self) -> PULSECNT_R {
        PULSECNT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Current Pulse Counter Value"]
    #[inline(always)]
    #[must_use]
    pub fn pulsecnt(&mut self) -> PULSECNT_W<0> {
        PULSECNT_W::new(self)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> RESERVED12_W<12> {
        RESERVED12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Current Pulse Count Register: Read only register giving read access to the state machine current pulse count value for program/erase operations.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statpcnt](index.html) module"]
pub struct STATPCNT_SPEC;
impl crate::RegisterSpec for STATPCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [statpcnt::R](R) reader structure"]
impl crate::Readable for STATPCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [statpcnt::W](W) writer structure"]
impl crate::Writable for STATPCNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATPCNT to value 0"]
impl crate::Resettable for STATPCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
