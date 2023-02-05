#[doc = "Register `TDCREFCLKCTL` reader"]
pub struct R(crate::R<TDCREFCLKCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TDCREFCLKCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TDCREFCLKCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TDCREFCLKCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TDCREFCLKCTL` writer"]
pub struct W(crate::W<TDCREFCLKCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TDCREFCLKCTL_SPEC>;
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
impl From<crate::W<TDCREFCLKCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TDCREFCLKCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REQ` reader - 0:0\\]
TDC reference clock request. 0: Disable TDC reference clock. 1: Enable TDC reference clock. Only modify REQ when equal to ACK."]
pub type REQ_R = crate::BitReader<bool>;
#[doc = "Field `REQ` writer - 0:0\\]
TDC reference clock request. 0: Disable TDC reference clock. 1: Enable TDC reference clock. Only modify REQ when equal to ACK."]
pub type REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, TDCREFCLKCTL_SPEC, bool, O>;
#[doc = "Field `ACK` reader - 1:1\\]
TDC reference clock acknowledgement. 0: TDC reference clock is disabled. 1: TDC reference clock is enabled."]
pub type ACK_R = crate::BitReader<bool>;
#[doc = "Field `ACK` writer - 1:1\\]
TDC reference clock acknowledgement. 0: TDC reference clock is disabled. 1: TDC reference clock is enabled."]
pub type ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, TDCREFCLKCTL_SPEC, bool, O>;
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TDCREFCLKCTL_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
TDC reference clock request. 0: Disable TDC reference clock. 1: Enable TDC reference clock. Only modify REQ when equal to ACK."]
    #[inline(always)]
    pub fn req(&self) -> REQ_R {
        REQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
TDC reference clock acknowledgement. 0: TDC reference clock is disabled. 1: TDC reference clock is enabled."]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
TDC reference clock request. 0: Disable TDC reference clock. 1: Enable TDC reference clock. Only modify REQ when equal to ACK."]
    #[inline(always)]
    #[must_use]
    pub fn req(&mut self) -> REQ_W<0> {
        REQ_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
TDC reference clock acknowledgement. 0: TDC reference clock is disabled. 1: TDC reference clock is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> ACK_W<1> {
        ACK_W::new(self)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TDC Reference Clock Control Controls if the AUX_TDC reference clock source is enabled. This clock is compared against the AUX_TDC counter clock. TDC reference clock source is configured in DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdcrefclkctl](index.html) module"]
pub struct TDCREFCLKCTL_SPEC;
impl crate::RegisterSpec for TDCREFCLKCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tdcrefclkctl::R](R) reader structure"]
impl crate::Readable for TDCREFCLKCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tdcrefclkctl::W](W) writer structure"]
impl crate::Writable for TDCREFCLKCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TDCREFCLKCTL to value 0"]
impl crate::Resettable for TDCREFCLKCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
