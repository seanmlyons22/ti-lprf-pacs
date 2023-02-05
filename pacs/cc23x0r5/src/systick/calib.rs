#[doc = "Register `CALIB` reader"]
pub type R = crate::R<CalibSpec>;
#[doc = "Register `CALIB` writer"]
pub type W = crate::W<CalibSpec>;
#[doc = "Field `TENMS` reader - 23:0\\]
An optional Reload value to be used for 10ms (100Hz) timing, subject to system clock skew errors. If the value reads as 0, the calibration value is not known."]
pub type TenmsR = crate::FieldReader<u32>;
#[doc = "Field `RESERVED24` reader - 29:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24R = crate::FieldReader;
#[doc = "Field `SKEW` reader - 30:30\\]
If reads as 1, the calibration value for 10ms is inexact (due to clock frequency)."]
pub type SkewR = crate::BitReader;
#[doc = "Field `NOREF` reader - 31:31\\]
If reads as 1, the Reference clock is not provided - the CLKSOURCE bit of the SysTick Control and Status register will be forced to 1 and cannot be cleared to 0."]
pub type NorefR = crate::BitReader;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
An optional Reload value to be used for 10ms (100Hz) timing, subject to system clock skew errors. If the value reads as 0, the calibration value is not known."]
    #[inline(always)]
    pub fn tenms(&self) -> TenmsR {
        TenmsR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> Reserved24R {
        Reserved24R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - 30:30\\]
If reads as 1, the calibration value for 10ms is inexact (due to clock frequency)."]
    #[inline(always)]
    pub fn skew(&self) -> SkewR {
        SkewR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
If reads as 1, the Reference clock is not provided - the CLKSOURCE bit of the SysTick Control and Status register will be forced to 1 and cannot be cleared to 0."]
    #[inline(always)]
    pub fn noref(&self) -> NorefR {
        NorefR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "Use the SysTick Calibration Value Register to enable software to scale to any required speed using divide and multiply.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calib::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calib::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalibSpec;
impl crate::RegisterSpec for CalibSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calib::R`](R) reader structure"]
impl crate::Readable for CalibSpec {}
#[doc = "`write(|w| ..)` method takes [`calib::W`](W) writer structure"]
impl crate::Writable for CalibSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CALIB to value 0"]
impl crate::Resettable for CalibSpec {
    const RESET_VALUE: u32 = 0;
}
