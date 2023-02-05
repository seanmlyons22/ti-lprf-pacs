#[doc = "Register `RCOSCHFCTL` reader"]
pub struct R(crate::R<RCOSCHFCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCOSCHFCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCOSCHFCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCOSCHFCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCOSCHFCTL` writer"]
pub struct W(crate::W<RCOSCHFCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCOSCHFCTL_SPEC>;
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
impl From<crate::W<RCOSCHFCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCOSCHFCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED0` writer - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCOSCHFCTL_SPEC, u8, u8, 8, O>;
#[doc = "Field `RCOSCHF_CTRIM` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type RCOSCHF_CTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RCOSCHF_CTRIM` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type RCOSCHF_CTRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RCOSCHFCTL_SPEC, u8, u8, 8, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RCOSCHFCTL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcoschf_ctrim(&self) -> RCOSCHF_CTRIM_R {
        RCOSCHF_CTRIM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<0> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rcoschf_ctrim(&mut self) -> RCOSCHF_CTRIM_W<8> {
        RCOSCHF_CTRIM_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> RESERVED16_W<16> {
        RESERVED16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCOSCHF Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcoschfctl](index.html) module"]
pub struct RCOSCHFCTL_SPEC;
impl crate::RegisterSpec for RCOSCHFCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcoschfctl::R](R) reader structure"]
impl crate::Readable for RCOSCHFCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcoschfctl::W](W) writer structure"]
impl crate::Writable for RCOSCHFCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCOSCHFCTL to value 0"]
impl crate::Resettable for RCOSCHFCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
