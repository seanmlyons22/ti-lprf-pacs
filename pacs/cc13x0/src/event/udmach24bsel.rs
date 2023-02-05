#[doc = "Register `UDMACH24BSEL` reader"]
pub struct R(crate::R<UDMACH24BSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UDMACH24BSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UDMACH24BSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UDMACH24BSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UDMACH24BSEL` writer"]
pub struct W(crate::W<UDMACH24BSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UDMACH24BSEL_SPEC>;
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
impl From<crate::W<UDMACH24BSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UDMACH24BSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EV` reader - 6:0\\]
Read only selection value"]
pub type EV_R = crate::FieldReader<u8, EV_A>;
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 103"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EV_A {
    #[doc = "103: Software event 3, triggered by SWEV.SWEV3"]
    SWEV3 = 103,
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
            103 => Some(EV_A::SWEV3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SWEV3`"]
    #[inline(always)]
    pub fn is_swev3(&self) -> bool {
        *self == EV_A::SWEV3
    }
}
#[doc = "Field `EV` writer - 6:0\\]
Read only selection value"]
pub type EV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, UDMACH24BSEL_SPEC, u8, EV_A, 7, O>;
impl<'a, const O: u8> EV_W<'a, O> {
    #[doc = "Software event 3, triggered by SWEV.SWEV3"]
    #[inline(always)]
    pub fn swev3(self) -> &'a mut W {
        self.variant(EV_A::SWEV3)
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
#[doc = "Output Selection for DMA Channel 24 REQ\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach24bsel](index.html) module"]
pub struct UDMACH24BSEL_SPEC;
impl crate::RegisterSpec for UDMACH24BSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [udmach24bsel::R](R) reader structure"]
impl crate::Readable for UDMACH24BSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [udmach24bsel::W](W) writer structure"]
impl crate::Writable for UDMACH24BSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UDMACH24BSEL to value 0x67"]
impl crate::Resettable for UDMACH24BSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x67;
}
