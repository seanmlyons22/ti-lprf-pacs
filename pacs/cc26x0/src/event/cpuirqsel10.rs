#[doc = "Register `CPUIRQSEL10` reader"]
pub struct R(crate::R<CPUIRQSEL10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPUIRQSEL10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPUIRQSEL10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPUIRQSEL10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPUIRQSEL10` writer"]
pub struct W(crate::W<CPUIRQSEL10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPUIRQSEL10_SPEC>;
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
impl From<crate::W<CPUIRQSEL10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPUIRQSEL10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EV` reader - 6:0\\]
Read only selection value"]
pub type EV_R = crate::FieldReader<u8, EV_A>;
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 26"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EV_A {
    #[doc = "26: Combined RFC hardware interrupt, corresponding flag is here RFC_DBELL:RFHWIFG"]
    RFC_HW_COMB = 26,
}
impl From<EV_A> for u8 {
    #[inline(always)]
    fn from(variant: EV_A) -> Self {
        variant as _
    }
}
impl EV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EV_A> {
        match self.bits {
            26 => Some(EV_A::RFC_HW_COMB),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RFC_HW_COMB`"]
    #[inline(always)]
    pub fn is_rfc_hw_comb(&self) -> bool {
        *self == EV_A::RFC_HW_COMB
    }
}
#[doc = "Field `EV` writer - 6:0\\]
Read only selection value"]
pub type EV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPUIRQSEL10_SPEC, u8, EV_A, 7, O>;
impl<'a, const O: u8> EV_W<'a, O> {
    #[doc = "Combined RFC hardware interrupt, corresponding flag is here RFC_DBELL:RFHWIFG"]
    #[inline(always)]
    pub fn rfc_hw_comb(self) -> &'a mut W {
        self.variant(EV_A::RFC_HW_COMB)
    }
}
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Read only selection value"]
    #[inline(always)]
    pub fn ev(&self) -> EV_R {
        EV_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Read only selection value"]
    #[inline(always)]
    #[must_use]
    pub fn ev(&mut self) -> EV_W<0> {
        EV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output Selection for CPU Interrupt 10\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel10](index.html) module"]
pub struct CPUIRQSEL10_SPEC;
impl crate::RegisterSpec for CPUIRQSEL10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpuirqsel10::R](R) reader structure"]
impl crate::Readable for CPUIRQSEL10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpuirqsel10::W](W) writer structure"]
impl crate::Writable for CPUIRQSEL10_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPUIRQSEL10 to value 0x1a"]
impl crate::Resettable for CPUIRQSEL10_SPEC {
    const RESET_VALUE: Self::Ux = 0x1a;
}
