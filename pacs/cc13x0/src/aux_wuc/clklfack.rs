#[doc = "Register `CLKLFACK` reader"]
pub struct R(crate::R<CLKLFACK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKLFACK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKLFACK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKLFACK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKLFACK` writer"]
pub struct W(crate::W<CLKLFACK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKLFACK_SPEC>;
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
impl From<crate::W<CLKLFACK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKLFACK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACK` reader - 0:0\\]
Acknowledgment of CLKLFREQ.REQ 0: Acknowledgement that clock frequency is controlled by AON_WUC:AUXCLK and the system state 1: Acknowledgement that the low frequency clock SCLK_LF is the clock source for AUX"]
pub type ACK_R = crate::BitReader<bool>;
#[doc = "Field `ACK` writer - 0:0\\]
Acknowledgment of CLKLFREQ.REQ 0: Acknowledgement that clock frequency is controlled by AON_WUC:AUXCLK and the system state 1: Acknowledgement that the low frequency clock SCLK_LF is the clock source for AUX"]
pub type ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKLFACK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Acknowledgment of CLKLFREQ.REQ 0: Acknowledgement that clock frequency is controlled by AON_WUC:AUXCLK and the system state 1: Acknowledgement that the low frequency clock SCLK_LF is the clock source for AUX"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Acknowledgment of CLKLFREQ.REQ 0: Acknowledgement that clock frequency is controlled by AON_WUC:AUXCLK and the system state 1: Acknowledgement that the low frequency clock SCLK_LF is the clock source for AUX"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> ACK_W<0> {
        ACK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Frequency Clock Acknowledgment\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clklfack](index.html) module"]
pub struct CLKLFACK_SPEC;
impl crate::RegisterSpec for CLKLFACK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clklfack::R](R) reader structure"]
impl crate::Readable for CLKLFACK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clklfack::W](W) writer structure"]
impl crate::Writable for CLKLFACK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKLFACK to value 0"]
impl crate::Resettable for CLKLFACK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
