#[doc = "Register `STCR` reader"]
pub type R = crate::R<StcrSpec>;
#[doc = "Register `STCR` writer"]
pub type W = crate::W<StcrSpec>;
#[doc = "Field `TENMS` reader - 23:0\\]
An optional Reload value to be used for 10ms (100Hz) timing, subject to system clock skew errors. The value read is valid only when core clock is at 48MHz."]
pub type TenmsR = crate::FieldReader<u32>;
#[doc = "Field `RESERVED24` reader - 29:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24R = crate::FieldReader;
#[doc = "Field `SKEW` reader - 30:30\\]
Reads as one. The calibration value is not exactly 10ms because of clock frequency. This could affect its suitability as a software real time clock."]
pub type SkewR = crate::BitReader;
#[doc = "Field `NOREF` reader - 31:31\\]
Reads as one. Indicates that no separate reference clock is provided."]
pub type NorefR = crate::BitReader;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
An optional Reload value to be used for 10ms (100Hz) timing, subject to system clock skew errors. The value read is valid only when core clock is at 48MHz."]
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
Reads as one. The calibration value is not exactly 10ms because of clock frequency. This could affect its suitability as a software real time clock."]
    #[inline(always)]
    pub fn skew(&self) -> SkewR {
        SkewR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Reads as one. Indicates that no separate reference clock is provided."]
    #[inline(always)]
    pub fn noref(&self) -> NorefR {
        NorefR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "SysTick Calibration Value Used to enable software to scale to any required speed using divide and multiply.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StcrSpec;
impl crate::RegisterSpec for StcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stcr::R`](R) reader structure"]
impl crate::Readable for StcrSpec {}
#[doc = "`write(|w| ..)` method takes [`stcr::W`](W) writer structure"]
impl crate::Writable for StcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STCR to value 0xc007_5300"]
impl crate::Resettable for StcrSpec {
    const RESET_VALUE: u32 = 0xc007_5300;
}
