#[doc = "Register `FCOR_ERR_POS` reader"]
pub struct R(crate::R<FCOR_ERR_POS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCOR_ERR_POS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCOR_ERR_POS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCOR_ERR_POS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCOR_ERR_POS` writer"]
pub struct W(crate::W<FCOR_ERR_POS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCOR_ERR_POS_SPEC>;
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
impl From<crate::W<FCOR_ERR_POS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCOR_ERR_POS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SERR_POS` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type SERR_POS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SERR_POS` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type SERR_POS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FCOR_ERR_POS_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn serr_pos(&self) -> SERR_POS_R {
        SERR_POS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn serr_pos(&mut self) -> SERR_POS_W<0> {
        SERR_POS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcor_err_pos](index.html) module"]
pub struct FCOR_ERR_POS_SPEC;
impl crate::RegisterSpec for FCOR_ERR_POS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcor_err_pos::R](R) reader structure"]
impl crate::Readable for FCOR_ERR_POS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcor_err_pos::W](W) writer structure"]
impl crate::Writable for FCOR_ERR_POS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCOR_ERR_POS to value 0"]
impl crate::Resettable for FCOR_ERR_POS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
