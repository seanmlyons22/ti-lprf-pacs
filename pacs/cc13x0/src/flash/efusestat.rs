#[doc = "Register `EFUSESTAT` reader"]
pub struct R(crate::R<EFUSESTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EFUSESTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EFUSESTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EFUSESTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EFUSESTAT` writer"]
pub struct W(crate::W<EFUSESTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EFUSESTAT_SPEC>;
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
impl From<crate::W<EFUSESTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EFUSESTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESETDONE` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type RESETDONE_R = crate::BitReader<bool>;
#[doc = "Field `RESETDONE` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type RESETDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EFUSESTAT_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EFUSESTAT_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn resetdone(&self) -> RESETDONE_R {
        RESETDONE_R::new((self.bits & 1) != 0)
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
    pub fn resetdone(&mut self) -> RESETDONE_W<0> {
        RESETDONE_W::new(self)
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
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efusestat](index.html) module"]
pub struct EFUSESTAT_SPEC;
impl crate::RegisterSpec for EFUSESTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [efusestat::R](R) reader structure"]
impl crate::Readable for EFUSESTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [efusestat::W](W) writer structure"]
impl crate::Writable for EFUSESTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EFUSESTAT to value 0x01"]
impl crate::Resettable for EFUSESTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
