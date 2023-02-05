#[doc = "Register `IRQFLAGSTAT` reader"]
pub struct R(crate::R<IRQFLAGSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQFLAGSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQFLAGSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQFLAGSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQFLAGSTAT` writer"]
pub struct W(crate::W<IRQFLAGSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQFLAGSTAT_SPEC>;
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
impl From<crate::W<IRQFLAGSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQFLAGSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDY` reader - 0:0\\]
1: Data are available in OUT0 and OUT1. Acknowledging this state by writing '1' to IRQFLAGCLR.RDY clears this bit to '0'. If a new number is already available in the internal register of the TRNG, the number is directly clocked into the result register. In this case the status bit is asserted again, after one clock cycle."]
pub type RDY_R = crate::BitReader<bool>;
#[doc = "Field `RDY` writer - 0:0\\]
1: Data are available in OUT0 and OUT1. Acknowledging this state by writing '1' to IRQFLAGCLR.RDY clears this bit to '0'. If a new number is already available in the internal register of the TRNG, the number is directly clocked into the result register. In this case the status bit is asserted again, after one clock cycle."]
pub type RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQFLAGSTAT_SPEC, bool, O>;
#[doc = "Field `SHUTDOWN_OVF` reader - 1:1\\]
1: The number of FROs shut down (i.e. the number of '1' bits in the ALARMSTOP register) has exceeded the threshold set by ALARMCNT.SHUTDOWN_THR Writing '1' to IRQFLAGCLR.SHUTDOWN_OVF clears this bit to '0' again."]
pub type SHUTDOWN_OVF_R = crate::BitReader<bool>;
#[doc = "Field `SHUTDOWN_OVF` writer - 1:1\\]
1: The number of FROs shut down (i.e. the number of '1' bits in the ALARMSTOP register) has exceeded the threshold set by ALARMCNT.SHUTDOWN_THR Writing '1' to IRQFLAGCLR.SHUTDOWN_OVF clears this bit to '0' again."]
pub type SHUTDOWN_OVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQFLAGSTAT_SPEC, bool, O>;
#[doc = "Field `RESERVED2` reader - 30:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED2` writer - 30:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IRQFLAGSTAT_SPEC, u32, u32, 29, O>;
#[doc = "Field `NEED_CLOCK` reader - 31:31\\]
1: Indicates that the TRNG is busy generating entropy or is in one of its test modes - clocks may not be turned off and the power supply voltage must be kept stable. 0: TRNG is idle and can be shut down"]
pub type NEED_CLOCK_R = crate::BitReader<bool>;
#[doc = "Field `NEED_CLOCK` writer - 31:31\\]
1: Indicates that the TRNG is busy generating entropy or is in one of its test modes - clocks may not be turned off and the power supply voltage must be kept stable. 0: TRNG is idle and can be shut down"]
pub type NEED_CLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQFLAGSTAT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
1: Data are available in OUT0 and OUT1. Acknowledging this state by writing '1' to IRQFLAGCLR.RDY clears this bit to '0'. If a new number is already available in the internal register of the TRNG, the number is directly clocked into the result register. In this case the status bit is asserted again, after one clock cycle."]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
1: The number of FROs shut down (i.e. the number of '1' bits in the ALARMSTOP register) has exceeded the threshold set by ALARMCNT.SHUTDOWN_THR Writing '1' to IRQFLAGCLR.SHUTDOWN_OVF clears this bit to '0' again."]
    #[inline(always)]
    pub fn shutdown_ovf(&self) -> SHUTDOWN_OVF_R {
        SHUTDOWN_OVF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:30 - 30:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new((self.bits >> 2) & 0x1fff_ffff)
    }
    #[doc = "Bit 31 - 31:31\\]
1: Indicates that the TRNG is busy generating entropy or is in one of its test modes - clocks may not be turned off and the power supply voltage must be kept stable. 0: TRNG is idle and can be shut down"]
    #[inline(always)]
    pub fn need_clock(&self) -> NEED_CLOCK_R {
        NEED_CLOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
1: Data are available in OUT0 and OUT1. Acknowledging this state by writing '1' to IRQFLAGCLR.RDY clears this bit to '0'. If a new number is already available in the internal register of the TRNG, the number is directly clocked into the result register. In this case the status bit is asserted again, after one clock cycle."]
    #[inline(always)]
    #[must_use]
    pub fn rdy(&mut self) -> RDY_W<0> {
        RDY_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
1: The number of FROs shut down (i.e. the number of '1' bits in the ALARMSTOP register) has exceeded the threshold set by ALARMCNT.SHUTDOWN_THR Writing '1' to IRQFLAGCLR.SHUTDOWN_OVF clears this bit to '0' again."]
    #[inline(always)]
    #[must_use]
    pub fn shutdown_ovf(&mut self) -> SHUTDOWN_OVF_W<1> {
        SHUTDOWN_OVF_W::new(self)
    }
    #[doc = "Bits 2:30 - 30:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
1: Indicates that the TRNG is busy generating entropy or is in one of its test modes - clocks may not be turned off and the power supply voltage must be kept stable. 0: TRNG is idle and can be shut down"]
    #[inline(always)]
    #[must_use]
    pub fn need_clock(&mut self) -> NEED_CLOCK_W<31> {
        NEED_CLOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqflagstat](index.html) module"]
pub struct IRQFLAGSTAT_SPEC;
impl crate::RegisterSpec for IRQFLAGSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irqflagstat::R](R) reader structure"]
impl crate::Readable for IRQFLAGSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irqflagstat::W](W) writer structure"]
impl crate::Writable for IRQFLAGSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IRQFLAGSTAT to value 0"]
impl crate::Resettable for IRQFLAGSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
