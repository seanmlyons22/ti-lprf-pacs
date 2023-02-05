#[doc = "Register `DMACH0EXTADDR` reader"]
pub struct R(crate::R<DMACH0EXTADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACH0EXTADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACH0EXTADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACH0EXTADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACH0EXTADDR` writer"]
pub struct W(crate::W<DMACH0EXTADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACH0EXTADDR_SPEC>;
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
impl From<crate::W<DMACH0EXTADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACH0EXTADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - 31:0\\]
Channel external address value. Holds the last updated external address after being sent to the master interface."]
pub type ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDR` writer - 31:0\\]
Channel external address value. Holds the last updated external address after being sent to the master interface."]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMACH0EXTADDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Channel external address value. Holds the last updated external address after being sent to the master interface."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Channel external address value. Holds the last updated external address after being sent to the master interface."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<0> {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channel 0 External Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmach0extaddr](index.html) module"]
pub struct DMACH0EXTADDR_SPEC;
impl crate::RegisterSpec for DMACH0EXTADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmach0extaddr::R](R) reader structure"]
impl crate::Readable for DMACH0EXTADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmach0extaddr::W](W) writer structure"]
impl crate::Writable for DMACH0EXTADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMACH0EXTADDR to value 0"]
impl crate::Resettable for DMACH0EXTADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
