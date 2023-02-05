#[doc = "Register `RFCSEL0` reader"]
pub struct R(crate::R<RFCSEL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFCSEL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFCSEL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFCSEL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFCSEL0` writer"]
pub struct W(crate::W<RFCSEL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFCSEL0_SPEC>;
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
impl From<crate::W<RFCSEL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFCSEL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EV` reader - 7:0\\]
Read only selection value"]
pub type EV_R = crate::FieldReader<u8, EV_A>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 61"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EV_A {
    #[doc = "61: GPT0A compare event. Configured by GPT0:TAMR.TCACT"]
    GPT0A_CMP = 61,
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
            61 => Some(EV_A::GPT0A_CMP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GPT0A_CMP`"]
    #[inline(always)]
    pub fn is_gpt0a_cmp(&self) -> bool {
        *self == EV_A::GPT0A_CMP
    }
}
#[doc = "Field `EV` writer - 7:0\\]
Read only selection value"]
pub type EV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RFCSEL0_SPEC, u8, EV_A, 8, O>;
impl<'a, const O: u8> EV_W<'a, O> {
    #[doc = "GPT0A compare event. Configured by GPT0:TAMR.TCACT"]
    #[inline(always)]
    pub fn gpt0a_cmp(self) -> &'a mut W {
        self.variant(EV_A::GPT0A_CMP)
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
#[doc = "Output Selection for RFC Event 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcsel0](index.html) module"]
pub struct RFCSEL0_SPEC;
impl crate::RegisterSpec for RFCSEL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfcsel0::R](R) reader structure"]
impl crate::Readable for RFCSEL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfcsel0::W](W) writer structure"]
impl crate::Writable for RFCSEL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RFCSEL0 to value 0x3d"]
impl crate::Resettable for RFCSEL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x3d;
}
