#[doc = "Register `MCUBUSSTAT` reader"]
pub struct R(crate::R<MCUBUSSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCUBUSSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCUBUSSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCUBUSSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCUBUSSTAT` writer"]
pub struct W(crate::W<MCUBUSSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCUBUSSTAT_SPEC>;
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
impl From<crate::W<MCUBUSSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCUBUSSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DISCONNECT_ACK` reader - 0:0\\]
Acknowledges reception of the bus disconnection request, by matching the value of MCUBUSCTL.DISCONNECT_REQ. Note that if AON_WUC:AUXCTL.AUX_FORCE_ON = 1 a reconnect to the MCU domain bus will be made regardless of the state of MCUBUSCTL.DISCONNECT_REQ"]
pub type DISCONNECT_ACK_R = crate::BitReader<bool>;
#[doc = "Field `DISCONNECT_ACK` writer - 0:0\\]
Acknowledges reception of the bus disconnection request, by matching the value of MCUBUSCTL.DISCONNECT_REQ. Note that if AON_WUC:AUXCTL.AUX_FORCE_ON = 1 a reconnect to the MCU domain bus will be made regardless of the state of MCUBUSCTL.DISCONNECT_REQ"]
pub type DISCONNECT_ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCUBUSSTAT_SPEC, bool, O>;
#[doc = "Field `DISCONNECTED` reader - 1:1\\]
Indicates whether the AUX domain and MCU domain buses are currently disconnected (1) or connected (0)."]
pub type DISCONNECTED_R = crate::BitReader<bool>;
#[doc = "Field `DISCONNECTED` writer - 1:1\\]
Indicates whether the AUX domain and MCU domain buses are currently disconnected (1) or connected (0)."]
pub type DISCONNECTED_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCUBUSSTAT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Acknowledges reception of the bus disconnection request, by matching the value of MCUBUSCTL.DISCONNECT_REQ. Note that if AON_WUC:AUXCTL.AUX_FORCE_ON = 1 a reconnect to the MCU domain bus will be made regardless of the state of MCUBUSCTL.DISCONNECT_REQ"]
    #[inline(always)]
    pub fn disconnect_ack(&self) -> DISCONNECT_ACK_R {
        DISCONNECT_ACK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates whether the AUX domain and MCU domain buses are currently disconnected (1) or connected (0)."]
    #[inline(always)]
    pub fn disconnected(&self) -> DISCONNECTED_R {
        DISCONNECTED_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Acknowledges reception of the bus disconnection request, by matching the value of MCUBUSCTL.DISCONNECT_REQ. Note that if AON_WUC:AUXCTL.AUX_FORCE_ON = 1 a reconnect to the MCU domain bus will be made regardless of the state of MCUBUSCTL.DISCONNECT_REQ"]
    #[inline(always)]
    #[must_use]
    pub fn disconnect_ack(&mut self) -> DISCONNECT_ACK_W<0> {
        DISCONNECT_ACK_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates whether the AUX domain and MCU domain buses are currently disconnected (1) or connected (0)."]
    #[inline(always)]
    #[must_use]
    pub fn disconnected(&mut self) -> DISCONNECTED_W<1> {
        DISCONNECTED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCU Bus Status Indicates the connection state of the AUX domain and MCU domain buses. Note that this register cannot be read from the MCU domain while disconnected, and is therefore only useful for the AUX_SCE.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcubusstat](index.html) module"]
pub struct MCUBUSSTAT_SPEC;
impl crate::RegisterSpec for MCUBUSSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcubusstat::R](R) reader structure"]
impl crate::Readable for MCUBUSSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcubusstat::W](W) writer structure"]
impl crate::Writable for MCUBUSSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCUBUSSTAT to value 0"]
impl crate::Resettable for MCUBUSSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
