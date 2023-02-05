#[doc = "Register `IABR0` reader"]
pub struct R(crate::R<IABR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IABR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IABR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IABR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IABR0` writer"]
pub struct W(crate::W<IABR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IABR0_SPEC>;
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
impl From<crate::W<IABR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IABR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACTIVE` reader - 31:0\\]
For ACTIVE\\[m\\]
in NVIC_IABR*n, indicates the active state for interrupt 32*n+m"]
pub type ACTIVE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ACTIVE` writer - 31:0\\]
For ACTIVE\\[m\\]
in NVIC_IABR*n, indicates the active state for interrupt 32*n+m"]
pub type ACTIVE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IABR0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
For ACTIVE\\[m\\]
in NVIC_IABR*n, indicates the active state for interrupt 32*n+m"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
For ACTIVE\\[m\\]
in NVIC_IABR*n, indicates the active state for interrupt 32*n+m"]
    #[inline(always)]
    #[must_use]
    pub fn active(&mut self) -> ACTIVE_W<0> {
        ACTIVE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "For each group of 32 interrupts, shows the active state of each interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iabr0](index.html) module"]
pub struct IABR0_SPEC;
impl crate::RegisterSpec for IABR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iabr0::R](R) reader structure"]
impl crate::Readable for IABR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iabr0::W](W) writer structure"]
impl crate::Writable for IABR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IABR0 to value 0"]
impl crate::Resettable for IABR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
