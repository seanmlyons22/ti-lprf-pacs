#[doc = "Register `LOOPADDR` reader"]
pub struct R(crate::R<LOOPADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOOPADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOOPADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOOPADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOOPADDR` writer"]
pub struct W(crate::W<LOOPADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOOPADDR_SPEC>;
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
impl From<crate::W<LOOPADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOOPADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type START_R = crate::FieldReader<u16, u16>;
#[doc = "Field `START` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type START_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LOOPADDR_SPEC, u16, u16, 16, O>;
#[doc = "Field `STOP` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type STOP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STOP` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type STOP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LOOPADDR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<16> {
        STOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [loopaddr](index.html) module"]
pub struct LOOPADDR_SPEC;
impl crate::RegisterSpec for LOOPADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [loopaddr::R](R) reader structure"]
impl crate::Readable for LOOPADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [loopaddr::W](W) writer structure"]
impl crate::Writable for LOOPADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOOPADDR to value 0"]
impl crate::Resettable for LOOPADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
