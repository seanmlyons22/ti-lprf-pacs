#[doc = "Register `TAMATCHR` reader"]
pub struct R(crate::R<TAMATCHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAMATCHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAMATCHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAMATCHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAMATCHR` writer"]
pub struct W(crate::W<TAMATCHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAMATCHR_SPEC>;
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
impl From<crate::W<TAMATCHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAMATCHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAMATCHR` reader - 31:0\\]
GPT Timer A Match Register"]
pub type TAMATCHR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TAMATCHR` writer - 31:0\\]
GPT Timer A Match Register"]
pub type TAMATCHR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAMATCHR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
GPT Timer A Match Register"]
    #[inline(always)]
    pub fn tamatchr(&self) -> TAMATCHR_R {
        TAMATCHR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
GPT Timer A Match Register"]
    #[inline(always)]
    #[must_use]
    pub fn tamatchr(&mut self) -> TAMATCHR_W<0> {
        TAMATCHR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer A Match Register Interrupts can be generated when the timer value is equal to the value in this register in one-shot or periodic mode. In Edge-Count mode, this register along with TAILR, determines how many edge events are counted. The total number of edge events counted is equal to the value in TAILR minus this value. Note that in edge-count mode, when executing an up-count, the value of TAPR and TAILR must be greater than the value of TAPMR and this register. In PWM mode, this value along with TAILR, determines the duty cycle of the output PWM signal. When a 16/32-bit GPT is configured to one of the 32-bit modes, TAMATCHR appears as a 32-bit register. (The upper 16-bits correspond to the contents TBMATCHR). In a 16-bit mode, the upper 16 bits of this register read as 0s and have no effect on the state of TBMATCHR. Note : This register is updated internally (takes effect) based on TAMR.TAMRSU\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamatchr](index.html) module"]
pub struct TAMATCHR_SPEC;
impl crate::RegisterSpec for TAMATCHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tamatchr::R](R) reader structure"]
impl crate::Readable for TAMATCHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tamatchr::W](W) writer structure"]
impl crate::Writable for TAMATCHR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAMATCHR to value 0xffff_ffff"]
impl crate::Resettable for TAMATCHR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
