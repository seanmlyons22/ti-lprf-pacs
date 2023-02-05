#[doc = "Register `PWRDWNREQ` reader"]
pub struct R(crate::R<PWRDWNREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWRDWNREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWRDWNREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWRDWNREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWRDWNREQ` writer"]
pub struct W(crate::W<PWRDWNREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRDWNREQ_SPEC>;
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
impl From<crate::W<PWRDWNREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWRDWNREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REQ` reader - 0:0\\]
Power down request 0: Request for system to be in active mode 1: Request for system to be in power down mode When REQ is 1 one shall assume that the system is in power down, and that current supply is limited. When setting REQ = 0, one shall assume that the system is in power down until PWRDWNACK.ACK = 0"]
pub type REQ_R = crate::BitReader<bool>;
#[doc = "Field `REQ` writer - 0:0\\]
Power down request 0: Request for system to be in active mode 1: Request for system to be in power down mode When REQ is 1 one shall assume that the system is in power down, and that current supply is limited. When setting REQ = 0, one shall assume that the system is in power down until PWRDWNACK.ACK = 0"]
pub type REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRDWNREQ_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Power down request 0: Request for system to be in active mode 1: Request for system to be in power down mode When REQ is 1 one shall assume that the system is in power down, and that current supply is limited. When setting REQ = 0, one shall assume that the system is in power down until PWRDWNACK.ACK = 0"]
    #[inline(always)]
    pub fn req(&self) -> REQ_R {
        REQ_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Power down request 0: Request for system to be in active mode 1: Request for system to be in power down mode When REQ is 1 one shall assume that the system is in power down, and that current supply is limited. When setting REQ = 0, one shall assume that the system is in power down until PWRDWNACK.ACK = 0"]
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
#[doc = "Power Down Request Request from AUX for system to enter power down. When system is in power down there is limited current supply available and the clock source is set by AON_WUC:AUXCLK.PWR_DWN_SRC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrdwnreq](index.html) module"]
pub struct PWRDWNREQ_SPEC;
impl crate::RegisterSpec for PWRDWNREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwrdwnreq::R](R) reader structure"]
impl crate::Readable for PWRDWNREQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwrdwnreq::W](W) writer structure"]
impl crate::Writable for PWRDWNREQ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWRDWNREQ to value 0"]
impl crate::Resettable for PWRDWNREQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
