#[doc = "Register `STMPXCNTCAPT0` reader"]
pub type R = crate::R<Stmpxcntcapt0Spec>;
#[doc = "Register `STMPXCNTCAPT0` writer"]
pub type W = crate::W<Stmpxcntcapt0Spec>;
#[doc = "Field `CAPT_VALUE` reader - 15:0\\]
The value of the samplestamp XOSC counter (STMPXCNT.CURR_VALUE) last time an event was pulsed (event source selected in \\[EVENT.I2SSTMPSEL0.EV\\]
for channel 0). This number corresponds to the number of 24 MHz clock cycles since the last positive edge of the selected WCLK. The value is cleared when STMPCTL.STMP_EN = 0. Note: Due to buffering and synchronization, WCLK is delayed by a small number of BCLK periods and clk periods. Note: When calculating the fractional part of the sample stamp, STMPXPER may be less than this bit field."]
pub type CaptValueR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
The value of the samplestamp XOSC counter (STMPXCNT.CURR_VALUE) last time an event was pulsed (event source selected in \\[EVENT.I2SSTMPSEL0.EV\\]
for channel 0). This number corresponds to the number of 24 MHz clock cycles since the last positive edge of the selected WCLK. The value is cleared when STMPCTL.STMP_EN = 0. Note: Due to buffering and synchronization, WCLK is delayed by a small number of BCLK periods and clk periods. Note: When calculating the fractional part of the sample stamp, STMPXPER may be less than this bit field."]
    #[inline(always)]
    pub fn capt_value(&self) -> CaptValueR {
        CaptValueR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Captured XOSC Counter Value, Capture Channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stmpxcntcapt0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stmpxcntcapt0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stmpxcntcapt0Spec;
impl crate::RegisterSpec for Stmpxcntcapt0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stmpxcntcapt0::R`](R) reader structure"]
impl crate::Readable for Stmpxcntcapt0Spec {}
#[doc = "`write(|w| ..)` method takes [`stmpxcntcapt0::W`](W) writer structure"]
impl crate::Writable for Stmpxcntcapt0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STMPXCNTCAPT0 to value 0"]
impl crate::Resettable for Stmpxcntcapt0Spec {
    const RESET_VALUE: u32 = 0;
}
