#[doc = "Register `CLAIMSET` reader"]
pub struct R(crate::R<CLAIMSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLAIMSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLAIMSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLAIMSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLAIMSET` writer"]
pub struct W(crate::W<CLAIMSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLAIMSET_SPEC>;
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
impl From<crate::W<CLAIMSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLAIMSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLAIMSET` reader - 31:0\\]
This register forms one half of the Claim Tag value. Writing to this location allows individual bits to be set (each bit is considered separately): 0: No effect 1: Set this bit in the claim tag The behavior when reading from this location is described in CLAIMMASK."]
pub type CLAIMSET_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CLAIMSET` writer - 31:0\\]
This register forms one half of the Claim Tag value. Writing to this location allows individual bits to be set (each bit is considered separately): 0: No effect 1: Set this bit in the claim tag The behavior when reading from this location is described in CLAIMMASK."]
pub type CLAIMSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLAIMSET_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register forms one half of the Claim Tag value. Writing to this location allows individual bits to be set (each bit is considered separately): 0: No effect 1: Set this bit in the claim tag The behavior when reading from this location is described in CLAIMMASK."]
    #[inline(always)]
    pub fn claimset(&self) -> CLAIMSET_R {
        CLAIMSET_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register forms one half of the Claim Tag value. Writing to this location allows individual bits to be set (each bit is considered separately): 0: No effect 1: Set this bit in the claim tag The behavior when reading from this location is described in CLAIMMASK."]
    #[inline(always)]
    #[must_use]
    pub fn claimset(&mut self) -> CLAIMSET_W<0> {
        CLAIMSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Claim Tag Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [claimset](index.html) module"]
pub struct CLAIMSET_SPEC;
impl crate::RegisterSpec for CLAIMSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [claimset::R](R) reader structure"]
impl crate::Readable for CLAIMSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [claimset::W](W) writer structure"]
impl crate::Writable for CLAIMSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLAIMSET to value 0x0f"]
impl crate::Resettable for CLAIMSET_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}
