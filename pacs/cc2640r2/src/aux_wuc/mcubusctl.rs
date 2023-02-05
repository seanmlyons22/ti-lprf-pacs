#[doc = "Register `MCUBUSCTL` reader"]
pub struct R(crate::R<MCUBUSCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCUBUSCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCUBUSCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCUBUSCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCUBUSCTL` writer"]
pub struct W(crate::W<MCUBUSCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCUBUSCTL_SPEC>;
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
impl From<crate::W<MCUBUSCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCUBUSCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DISCONNECT_REQ` reader - 0:0\\]
Requests the AUX domain bus to be disconnected from the MCU domain bus. The request has no effect when AON_WUC:AUX_CTL.AUX_FORCE_ON is set. The disconnection status can be monitored through MCUBUSSTAT. Note however that this register cannot be read by the system CPU while disconnected. It is recommended that this bit is set and remains set after initial power-up, and that the system CPU uses AON_WUC:AUX_CTL.AUX_FORCE_ON to connect/disconnect the bus."]
pub type DISCONNECT_REQ_R = crate::BitReader<bool>;
#[doc = "Field `DISCONNECT_REQ` writer - 0:0\\]
Requests the AUX domain bus to be disconnected from the MCU domain bus. The request has no effect when AON_WUC:AUX_CTL.AUX_FORCE_ON is set. The disconnection status can be monitored through MCUBUSSTAT. Note however that this register cannot be read by the system CPU while disconnected. It is recommended that this bit is set and remains set after initial power-up, and that the system CPU uses AON_WUC:AUX_CTL.AUX_FORCE_ON to connect/disconnect the bus."]
pub type DISCONNECT_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCUBUSCTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Requests the AUX domain bus to be disconnected from the MCU domain bus. The request has no effect when AON_WUC:AUX_CTL.AUX_FORCE_ON is set. The disconnection status can be monitored through MCUBUSSTAT. Note however that this register cannot be read by the system CPU while disconnected. It is recommended that this bit is set and remains set after initial power-up, and that the system CPU uses AON_WUC:AUX_CTL.AUX_FORCE_ON to connect/disconnect the bus."]
    #[inline(always)]
    pub fn disconnect_req(&self) -> DISCONNECT_REQ_R {
        DISCONNECT_REQ_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Requests the AUX domain bus to be disconnected from the MCU domain bus. The request has no effect when AON_WUC:AUX_CTL.AUX_FORCE_ON is set. The disconnection status can be monitored through MCUBUSSTAT. Note however that this register cannot be read by the system CPU while disconnected. It is recommended that this bit is set and remains set after initial power-up, and that the system CPU uses AON_WUC:AUX_CTL.AUX_FORCE_ON to connect/disconnect the bus."]
    #[inline(always)]
    #[must_use]
    pub fn disconnect_req(&mut self) -> DISCONNECT_REQ_W<0> {
        DISCONNECT_REQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCU Bus Control Controls the connection between the AUX domain bus and the MCU domain bus. The buses must be disconnected to allow power-down or power-off of the AUX domain.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcubusctl](index.html) module"]
pub struct MCUBUSCTL_SPEC;
impl crate::RegisterSpec for MCUBUSCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcubusctl::R](R) reader structure"]
impl crate::Readable for MCUBUSCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcubusctl::W](W) writer structure"]
impl crate::Writable for MCUBUSCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCUBUSCTL to value 0"]
impl crate::Resettable for MCUBUSCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
