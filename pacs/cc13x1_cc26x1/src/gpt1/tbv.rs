#[doc = "Register `TBV` reader"]
pub struct R(crate::R<TBV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TBV` writer"]
pub struct W(crate::W<TBV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBV_SPEC>;
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
impl From<crate::W<TBV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TBV` reader - 31:0\\]
GPT Timer B Register A read returns the current, free-running value of Timer B in all modes. When written, the value written into this register is loaded into the TBR register on the next clock cycle. Note: In 16-bit mode, only the lower 16-bits of this register can be written with a new value. Writes to the prescaler bits have no effect"]
pub type TBV_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TBV` writer - 31:0\\]
GPT Timer B Register A read returns the current, free-running value of Timer B in all modes. When written, the value written into this register is loaded into the TBR register on the next clock cycle. Note: In 16-bit mode, only the lower 16-bits of this register can be written with a new value. Writes to the prescaler bits have no effect"]
pub type TBV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TBV_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
GPT Timer B Register A read returns the current, free-running value of Timer B in all modes. When written, the value written into this register is loaded into the TBR register on the next clock cycle. Note: In 16-bit mode, only the lower 16-bits of this register can be written with a new value. Writes to the prescaler bits have no effect"]
    #[inline(always)]
    pub fn tbv(&self) -> TBV_R {
        TBV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
GPT Timer B Register A read returns the current, free-running value of Timer B in all modes. When written, the value written into this register is loaded into the TBR register on the next clock cycle. Note: In 16-bit mode, only the lower 16-bits of this register can be written with a new value. Writes to the prescaler bits have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn tbv(&mut self) -> TBV_W<0> {
        TBV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer B Value When read, this register shows the current, free-running value of Timer B in all modes. Software can use this value to determine the time elapsed between an interrupt and the ISR entry. When written, the value written into this register is loaded into the TBR register on the next clock cycle. When a 16/32-bit GPTM is configured to one of the 32-bit modes, the contents of bits 15:0 in this register are loaded into the upper 16 bits of the TAV register. Reads from this register return the current free-running value of Timer B. In a 16-bit mode, bits 15:0 contain the value of the counter and bits 23:16 contain the current, free-running value of the prescaler, which is the upper 8 bits of the count in Input Edge Count, Input Edge Time, PWM and one-shot or periodic up count modes. In one-shot or periodic down count modes, the prescaler stored in 23:16 is a true prescaler, meaning bits 23:16 count down before decrementing the value in bits 15:0. The prescaler in bits 31:24 always reads as 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbv](index.html) module"]
pub struct TBV_SPEC;
impl crate::RegisterSpec for TBV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbv::R](R) reader structure"]
impl crate::Readable for TBV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbv::W](W) writer structure"]
impl crate::Writable for TBV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TBV to value 0xffff"]
impl crate::Resettable for TBV_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
