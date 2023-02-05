#[doc = "Register `CPUIRQSEL9` reader"]
pub struct R(crate::R<CPUIRQSEL9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPUIRQSEL9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPUIRQSEL9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPUIRQSEL9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPUIRQSEL9` writer"]
pub struct W(crate::W<CPUIRQSEL9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPUIRQSEL9_SPEC>;
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
impl From<crate::W<CPUIRQSEL9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPUIRQSEL9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EV` reader - 7:0\\]
Read only selection value"]
pub type EV_R = crate::FieldReader<u8, EV_A>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 27"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EV_A {
    #[doc = "27: Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE0 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_0 event"]
    RFC_CPE_0 = 27,
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
            27 => Some(EV_A::RFC_CPE_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RFC_CPE_0`"]
    #[inline(always)]
    pub fn is_rfc_cpe_0(&self) -> bool {
        *self == EV_A::RFC_CPE_0
    }
}
#[doc = "Field `EV` writer - 7:0\\]
Read only selection value"]
pub type EV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPUIRQSEL9_SPEC, u8, EV_A, 8, O>;
impl<'a, const O: u8> EV_W<'a, O> {
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE0 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_0 event"]
    #[inline(always)]
    pub fn rfc_cpe_0(self) -> &'a mut W {
        self.variant(EV_A::RFC_CPE_0)
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Read only selection value"]
    #[inline(always)]
    pub fn ev(&self) -> EV_R {
        EV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
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
#[doc = "Output Selection for CPU Interrupt 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel9](index.html) module"]
pub struct CPUIRQSEL9_SPEC;
impl crate::RegisterSpec for CPUIRQSEL9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpuirqsel9::R](R) reader structure"]
impl crate::Readable for CPUIRQSEL9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpuirqsel9::W](W) writer structure"]
impl crate::Writable for CPUIRQSEL9_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPUIRQSEL9 to value 0x1b"]
impl crate::Resettable for CPUIRQSEL9_SPEC {
    const RESET_VALUE: Self::Ux = 0x1b;
}