#[doc = "Register `TAPR` reader"]
pub struct R(crate::R<TAPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAPR` writer"]
pub struct W(crate::W<TAPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAPR_SPEC>;
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
impl From<crate::W<TAPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAPSR` reader - 7:0\\]
Timer A Pre-scale. Prescaler ratio in one-shot and periodic count mode is TAPSR + 1, that is: 0: Prescaler ratio = 1 1: Prescaler ratio = 2 2: Prescaler ratio = 3 ... 255: Prescaler ratio = 256"]
pub type TAPSR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TAPSR` writer - 7:0\\]
Timer A Pre-scale. Prescaler ratio in one-shot and periodic count mode is TAPSR + 1, that is: 0: Prescaler ratio = 1 1: Prescaler ratio = 2 2: Prescaler ratio = 3 ... 255: Prescaler ratio = 256"]
pub type TAPSR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAPR_SPEC, u8, u8, 8, O>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAPR_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Timer A Pre-scale. Prescaler ratio in one-shot and periodic count mode is TAPSR + 1, that is: 0: Prescaler ratio = 1 1: Prescaler ratio = 2 2: Prescaler ratio = 3 ... 255: Prescaler ratio = 256"]
    #[inline(always)]
    pub fn tapsr(&self) -> TAPSR_R {
        TAPSR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Timer A Pre-scale. Prescaler ratio in one-shot and periodic count mode is TAPSR + 1, that is: 0: Prescaler ratio = 1 1: Prescaler ratio = 2 2: Prescaler ratio = 3 ... 255: Prescaler ratio = 256"]
    #[inline(always)]
    #[must_use]
    pub fn tapsr(&mut self) -> TAPSR_W<0> {
        TAPSR_W::new(self)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> RESERVED8_W<8> {
        RESERVED8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer A Pre-scale This register allows software to extend the range of the timers when they are used individually. When in one-shot or periodic down count modes, this register acts as a true prescaler for the timer counter. When acting as a true prescaler, the prescaler counts down to 0 before the value in TAR and TAV registers are incremented. In all other individual/split modes, this register is a linear extension of the upper range of the timer counter, holding bits 23:16 in the 16-bit modes of the 16/32-bit GPT.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tapr](index.html) module"]
pub struct TAPR_SPEC;
impl crate::RegisterSpec for TAPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tapr::R](R) reader structure"]
impl crate::Readable for TAPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tapr::W](W) writer structure"]
impl crate::Writable for TAPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAPR to value 0"]
impl crate::Resettable for TAPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
