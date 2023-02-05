#[doc = "Register `CMDR` reader"]
pub struct R(crate::R<CMDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMDR` writer"]
pub struct W(crate::W<CMDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMDR_SPEC>;
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
impl From<crate::W<CMDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMD` reader - 31:0\\]
Command register. Raises an interrupt to the Command and packet engine (CPE) upon write."]
pub type CMD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CMD` writer - 31:0\\]
Command register. Raises an interrupt to the Command and packet engine (CPE) upon write."]
pub type CMD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Command register. Raises an interrupt to the Command and packet engine (CPE) upon write."]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Command register. Raises an interrupt to the Command and packet engine (CPE) upon write."]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CMD_W<0> {
        CMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Doorbell Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdr](index.html) module"]
pub struct CMDR_SPEC;
impl crate::RegisterSpec for CMDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmdr::R](R) reader structure"]
impl crate::Readable for CMDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmdr::W](W) writer structure"]
impl crate::Writable for CMDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMDR to value 0"]
impl crate::Resettable for CMDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
