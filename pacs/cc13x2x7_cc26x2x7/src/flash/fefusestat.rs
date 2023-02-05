#[doc = "Register `FEFUSESTAT` reader"]
pub struct R(crate::R<FEFUSESTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FEFUSESTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FEFUSESTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FEFUSESTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FEFUSESTAT` writer"]
pub struct W(crate::W<FEFUSESTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FEFUSESTAT_SPEC>;
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
impl From<crate::W<FEFUSESTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FEFUSESTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHIFT_DONE` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type SHIFT_DONE_R = crate::BitReader<bool>;
#[doc = "Field `SHIFT_DONE` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type SHIFT_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FEFUSESTAT_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FEFUSESTAT_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn shift_done(&self) -> SHIFT_DONE_R {
        SHIFT_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn shift_done(&mut self) -> SHIFT_DONE_W<0> {
        SHIFT_DONE_W::new(self)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fefusestat](index.html) module"]
pub struct FEFUSESTAT_SPEC;
impl crate::RegisterSpec for FEFUSESTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fefusestat::R](R) reader structure"]
impl crate::Readable for FEFUSESTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fefusestat::W](W) writer structure"]
impl crate::Writable for FEFUSESTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FEFUSESTAT to value 0"]
impl crate::Resettable for FEFUSESTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
