#[doc = "Register `STRVR` reader"]
pub struct R(crate::R<STRVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STRVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STRVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STRVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STRVR` writer"]
pub struct W(crate::W<STRVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STRVR_SPEC>;
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
impl From<crate::W<STRVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STRVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RELOAD` reader - 23:0\\]
Value to load into the SysTick Current Value Register STCVR.CURRENT when the counter reaches 0."]
pub type RELOAD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RELOAD` writer - 23:0\\]
Value to load into the SysTick Current Value Register STCVR.CURRENT when the counter reaches 0."]
pub type RELOAD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STRVR_SPEC, u32, u32, 24, O>;
#[doc = "Field `RESERVED24` reader - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED24_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED24` writer - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED24_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STRVR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Value to load into the SysTick Current Value Register STCVR.CURRENT when the counter reaches 0."]
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Value to load into the SysTick Current Value Register STCVR.CURRENT when the counter reaches 0."]
    #[inline(always)]
    #[must_use]
    pub fn reload(&mut self) -> RELOAD_W<0> {
        RELOAD_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved24(&mut self) -> RESERVED24_W<24> {
        RESERVED24_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SysTick Reload Value This register is used to specify the start value to load into the current value register STCVR.CURRENT when the counter reaches 0. It can be any value between 1 and 0x00FFFFFF. A start value of 0 is possible, but has no effect because the SysTick interrupt and STCSR.COUNTFLAG are activated when counting from 1 to 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [strvr](index.html) module"]
pub struct STRVR_SPEC;
impl crate::RegisterSpec for STRVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [strvr::R](R) reader structure"]
impl crate::Readable for STRVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [strvr::W](W) writer structure"]
impl crate::Writable for STRVR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STRVR to value 0"]
impl crate::Resettable for STRVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
