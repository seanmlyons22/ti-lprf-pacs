#[doc = "Register `RTC_OFFSET` reader"]
pub type R = crate::R<RtcOffsetSpec>;
#[doc = "Register `RTC_OFFSET` writer"]
pub type W = crate::W<RtcOffsetSpec>;
#[doc = "Field `RTC_COMP_P2` reader - 7:0\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type RtcCompP2R = crate::FieldReader;
#[doc = "Field `RTC_COMP_P2` writer - 7:0\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type RtcCompP2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RTC_COMP_P1` reader - 15:8\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type RtcCompP1R = crate::FieldReader;
#[doc = "Field `RTC_COMP_P1` writer - 15:8\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type RtcCompP1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RTC_COMP_P0` reader - 31:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type RtcCompP0R = crate::FieldReader<u16>;
#[doc = "Field `RTC_COMP_P0` writer - 31:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type RtcCompP0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn rtc_comp_p2(&self) -> RtcCompP2R {
        RtcCompP2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn rtc_comp_p1(&self) -> RtcCompP1R {
        RtcCompP1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn rtc_comp_p0(&self) -> RtcCompP0R {
        RtcCompP0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn rtc_comp_p2(&mut self) -> RtcCompP2W<RtcOffsetSpec> {
        RtcCompP2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn rtc_comp_p1(&mut self) -> RtcCompP1W<RtcOffsetSpec> {
        RtcCompP1W::new(self, 8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn rtc_comp_p0(&mut self) -> RtcCompP0W<RtcOffsetSpec> {
        RtcCompP0W::new(self, 16)
    }
}
#[doc = "Real Time Clock Offset Enabled by MODE_CONF.RTC_COMP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_offset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_offset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcOffsetSpec;
impl crate::RegisterSpec for RtcOffsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_offset::R`](R) reader structure"]
impl crate::Readable for RtcOffsetSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_offset::W`](W) writer structure"]
impl crate::Writable for RtcOffsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTC_OFFSET to value 0xffff_ffff"]
impl crate::Resettable for RtcOffsetSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
