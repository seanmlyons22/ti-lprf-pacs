#[doc = "Register `STCVR` reader"]
pub struct R(crate::R<STCVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STCVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STCVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STCVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STCVR` writer"]
pub struct W(crate::W<STCVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STCVR_SPEC>;
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
impl From<crate::W<STCVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STCVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CURRENT` reader - 23:0\\]
Current value at the time the register is accessed. No read-modify-write protection is provided, so change with care. Writing to it with any value clears the register to 0. Clearing this register also clears STCSR.COUNTFLAG."]
pub type CURRENT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CURRENT` writer - 23:0\\]
Current value at the time the register is accessed. No read-modify-write protection is provided, so change with care. Writing to it with any value clears the register to 0. Clearing this register also clears STCSR.COUNTFLAG."]
pub type CURRENT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STCVR_SPEC, u32, u32, 24, O>;
#[doc = "Field `RESERVED24` reader - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED24_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED24` writer - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED24_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STCVR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Current value at the time the register is accessed. No read-modify-write protection is provided, so change with care. Writing to it with any value clears the register to 0. Clearing this register also clears STCSR.COUNTFLAG."]
    #[inline(always)]
    pub fn current(&self) -> CURRENT_R {
        CURRENT_R::new(self.bits & 0x00ff_ffff)
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
Current value at the time the register is accessed. No read-modify-write protection is provided, so change with care. Writing to it with any value clears the register to 0. Clearing this register also clears STCSR.COUNTFLAG."]
    #[inline(always)]
    #[must_use]
    pub fn current(&mut self) -> CURRENT_W<0> {
        CURRENT_W::new(self)
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
#[doc = "SysTick Current Value Read from this register returns the current value of SysTick counter. Writing to this register resets the SysTick counter (as well as STCSR.COUNTFLAG).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stcvr](index.html) module"]
pub struct STCVR_SPEC;
impl crate::RegisterSpec for STCVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stcvr::R](R) reader structure"]
impl crate::Readable for STCVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stcvr::W](W) writer structure"]
impl crate::Writable for STCVR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STCVR to value 0"]
impl crate::Resettable for STCVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
