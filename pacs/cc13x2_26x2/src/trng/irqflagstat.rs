#[doc = "Register `IRQFLAGSTAT` reader"]
pub type R = crate::R<IrqflagstatSpec>;
#[doc = "Register `IRQFLAGSTAT` writer"]
pub type W = crate::W<IrqflagstatSpec>;
#[doc = "Field `RDY` reader - 0:0\\]
1: Data are available in OUT0 and OUT1. Acknowledging this state by writing '1' to IRQFLAGCLR.RDY clears this bit to '0'. If a new number is already available in the internal register of the TRNG, the number is directly clocked into the result register. In this case the status bit is asserted again, after one clock cycle."]
pub type RdyR = crate::BitReader;
#[doc = "Field `RDY` writer - 0:0\\]
1: Data are available in OUT0 and OUT1. Acknowledging this state by writing '1' to IRQFLAGCLR.RDY clears this bit to '0'. If a new number is already available in the internal register of the TRNG, the number is directly clocked into the result register. In this case the status bit is asserted again, after one clock cycle."]
pub type RdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHUTDOWN_OVF` reader - 1:1\\]
1: The number of FROs shut down (i.e. the number of '1' bits in the ALARMSTOP register) has exceeded the threshold set by ALARMCNT.SHUTDOWN_THR Writing '1' to IRQFLAGCLR.SHUTDOWN_OVF clears this bit to '0' again."]
pub type ShutdownOvfR = crate::BitReader;
#[doc = "Field `SHUTDOWN_OVF` writer - 1:1\\]
1: The number of FROs shut down (i.e. the number of '1' bits in the ALARMSTOP register) has exceeded the threshold set by ALARMCNT.SHUTDOWN_THR Writing '1' to IRQFLAGCLR.SHUTDOWN_OVF clears this bit to '0' again."]
pub type ShutdownOvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 30:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED2` writer - 30:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
#[doc = "Field `NEED_CLOCK` reader - 31:31\\]
1: Indicates that the TRNG is busy generating entropy or is in one of its test modes - clocks may not be turned off and the power supply voltage must be kept stable. 0: TRNG is idle and can be shut down"]
pub type NeedClockR = crate::BitReader;
#[doc = "Field `NEED_CLOCK` writer - 31:31\\]
1: Indicates that the TRNG is busy generating entropy or is in one of its test modes - clocks may not be turned off and the power supply voltage must be kept stable. 0: TRNG is idle and can be shut down"]
pub type NeedClockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
1: Data are available in OUT0 and OUT1. Acknowledging this state by writing '1' to IRQFLAGCLR.RDY clears this bit to '0'. If a new number is already available in the internal register of the TRNG, the number is directly clocked into the result register. In this case the status bit is asserted again, after one clock cycle."]
    #[inline(always)]
    pub fn rdy(&self) -> RdyR {
        RdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
1: The number of FROs shut down (i.e. the number of '1' bits in the ALARMSTOP register) has exceeded the threshold set by ALARMCNT.SHUTDOWN_THR Writing '1' to IRQFLAGCLR.SHUTDOWN_OVF clears this bit to '0' again."]
    #[inline(always)]
    pub fn shutdown_ovf(&self) -> ShutdownOvfR {
        ShutdownOvfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:30 - 30:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x1fff_ffff)
    }
    #[doc = "Bit 31 - 31:31\\]
1: Indicates that the TRNG is busy generating entropy or is in one of its test modes - clocks may not be turned off and the power supply voltage must be kept stable. 0: TRNG is idle and can be shut down"]
    #[inline(always)]
    pub fn need_clock(&self) -> NeedClockR {
        NeedClockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
1: Data are available in OUT0 and OUT1. Acknowledging this state by writing '1' to IRQFLAGCLR.RDY clears this bit to '0'. If a new number is already available in the internal register of the TRNG, the number is directly clocked into the result register. In this case the status bit is asserted again, after one clock cycle."]
    #[inline(always)]
    #[must_use]
    pub fn rdy(&mut self) -> RdyW<IrqflagstatSpec> {
        RdyW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
1: The number of FROs shut down (i.e. the number of '1' bits in the ALARMSTOP register) has exceeded the threshold set by ALARMCNT.SHUTDOWN_THR Writing '1' to IRQFLAGCLR.SHUTDOWN_OVF clears this bit to '0' again."]
    #[inline(always)]
    #[must_use]
    pub fn shutdown_ovf(&mut self) -> ShutdownOvfW<IrqflagstatSpec> {
        ShutdownOvfW::new(self, 1)
    }
    #[doc = "Bits 2:30 - 30:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<IrqflagstatSpec> {
        Reserved2W::new(self, 2)
    }
    #[doc = "Bit 31 - 31:31\\]
1: Indicates that the TRNG is busy generating entropy or is in one of its test modes - clocks may not be turned off and the power supply voltage must be kept stable. 0: TRNG is idle and can be shut down"]
    #[inline(always)]
    #[must_use]
    pub fn need_clock(&mut self) -> NeedClockW<IrqflagstatSpec> {
        NeedClockW::new(self, 31)
    }
}
#[doc = "Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqflagstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqflagstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqflagstatSpec;
impl crate::RegisterSpec for IrqflagstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irqflagstat::R`](R) reader structure"]
impl crate::Readable for IrqflagstatSpec {}
#[doc = "`write(|w| ..)` method takes [`irqflagstat::W`](W) writer structure"]
impl crate::Writable for IrqflagstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQFLAGSTAT to value 0"]
impl crate::Resettable for IrqflagstatSpec {
    const RESET_VALUE: u32 = 0;
}
