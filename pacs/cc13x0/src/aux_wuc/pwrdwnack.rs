#[doc = "Register `PWRDWNACK` reader"]
pub struct R(crate::R<PWRDWNACK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWRDWNACK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWRDWNACK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWRDWNACK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWRDWNACK` writer"]
pub struct W(crate::W<PWRDWNACK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRDWNACK_SPEC>;
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
impl From<crate::W<PWRDWNACK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWRDWNACK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACK` reader - 0:0\\]
Power down acknowledgment. Indicates whether the power down request given by PWRDWNREQ.REQ is captured by the AON domain or not 0: AUX can assume that the system is in active mode 1: The request for power down is acknowledged and the AUX must act like the system is in power down mode and power supply is limited The system CPU cannot use this bit since the bus bridge between MCU domain and AUX domain is always disconnected when this bit is set. For AUX_SCE use only"]
pub type ACK_R = crate::BitReader<bool>;
#[doc = "Field `ACK` writer - 0:0\\]
Power down acknowledgment. Indicates whether the power down request given by PWRDWNREQ.REQ is captured by the AON domain or not 0: AUX can assume that the system is in active mode 1: The request for power down is acknowledged and the AUX must act like the system is in power down mode and power supply is limited The system CPU cannot use this bit since the bus bridge between MCU domain and AUX domain is always disconnected when this bit is set. For AUX_SCE use only"]
pub type ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRDWNACK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Power down acknowledgment. Indicates whether the power down request given by PWRDWNREQ.REQ is captured by the AON domain or not 0: AUX can assume that the system is in active mode 1: The request for power down is acknowledged and the AUX must act like the system is in power down mode and power supply is limited The system CPU cannot use this bit since the bus bridge between MCU domain and AUX domain is always disconnected when this bit is set. For AUX_SCE use only"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Power down acknowledgment. Indicates whether the power down request given by PWRDWNREQ.REQ is captured by the AON domain or not 0: AUX can assume that the system is in active mode 1: The request for power down is acknowledged and the AUX must act like the system is in power down mode and power supply is limited The system CPU cannot use this bit since the bus bridge between MCU domain and AUX domain is always disconnected when this bit is set. For AUX_SCE use only"]
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
#[doc = "Power Down Acknowledgment\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrdwnack](index.html) module"]
pub struct PWRDWNACK_SPEC;
impl crate::RegisterSpec for PWRDWNACK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwrdwnack::R](R) reader structure"]
impl crate::Readable for PWRDWNACK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwrdwnack::W](W) writer structure"]
impl crate::Writable for PWRDWNACK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWRDWNACK to value 0"]
impl crate::Resettable for PWRDWNACK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
