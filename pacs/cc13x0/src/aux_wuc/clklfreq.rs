#[doc = "Register `CLKLFREQ` reader"]
pub struct R(crate::R<CLKLFREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKLFREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKLFREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKLFREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKLFREQ` writer"]
pub struct W(crate::W<CLKLFREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKLFREQ_SPEC>;
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
impl From<crate::W<CLKLFREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKLFREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REQ` reader - 0:0\\]
Low frequency request 0: Request clock frequency to be controlled by AON_WUC:AUXCLK and the system state 1: Request low frequency clock SCLK_LF as the clock source for AUX This bit must not be modified unless CLKLFACK.ACK matches the current value"]
pub type REQ_R = crate::BitReader<bool>;
#[doc = "Field `REQ` writer - 0:0\\]
Low frequency request 0: Request clock frequency to be controlled by AON_WUC:AUXCLK and the system state 1: Request low frequency clock SCLK_LF as the clock source for AUX This bit must not be modified unless CLKLFACK.ACK matches the current value"]
pub type REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKLFREQ_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Low frequency request 0: Request clock frequency to be controlled by AON_WUC:AUXCLK and the system state 1: Request low frequency clock SCLK_LF as the clock source for AUX This bit must not be modified unless CLKLFACK.ACK matches the current value"]
    #[inline(always)]
    pub fn req(&self) -> REQ_R {
        REQ_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Low frequency request 0: Request clock frequency to be controlled by AON_WUC:AUXCLK and the system state 1: Request low frequency clock SCLK_LF as the clock source for AUX This bit must not be modified unless CLKLFACK.ACK matches the current value"]
    #[inline(always)]
    #[must_use]
    pub fn req(&mut self) -> REQ_W<0> {
        REQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Frequency Clock Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clklfreq](index.html) module"]
pub struct CLKLFREQ_SPEC;
impl crate::RegisterSpec for CLKLFREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clklfreq::R](R) reader structure"]
impl crate::Readable for CLKLFREQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clklfreq::W](W) writer structure"]
impl crate::Writable for CLKLFREQ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKLFREQ to value 0"]
impl crate::Resettable for CLKLFREQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
