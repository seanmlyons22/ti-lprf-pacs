#[doc = "Register `WUTIME` reader"]
pub type R = crate::R<WutimeSpec>;
#[doc = "Register `WUTIME` writer"]
pub type W = crate::W<WutimeSpec>;
#[doc = "Field `HFXTWU` reader - 7:0\\]
HFXT wakeup time Gives the time (in us) from HFSOC is running until HFXT auto enable is triggered. Measurement is enabled when PREPUCTL.WUTIMEN is set."]
pub type HfxtwuR = crate::FieldReader;
#[doc = "Field `RESERVED8` reader - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader;
#[doc = "Field `DIGWU` reader - 23:16\\]
Digital wakeup time. Gives the time (in us) from HFSOC is running until CPU execution starts. Measurement is enabled when PREPUCTL.WUTIMEN is set."]
pub type DigwuR = crate::FieldReader;
#[doc = "Field `RESERVED24` reader - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
HFXT wakeup time Gives the time (in us) from HFSOC is running until HFXT auto enable is triggered. Measurement is enabled when PREPUCTL.WUTIMEN is set."]
    #[inline(always)]
    pub fn hfxtwu(&self) -> HfxtwuR {
        HfxtwuR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Digital wakeup time. Gives the time (in us) from HFSOC is running until CPU execution starts. Measurement is enabled when PREPUCTL.WUTIMEN is set."]
    #[inline(always)]
    pub fn digwu(&self) -> DigwuR {
        DigwuR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> Reserved24R {
        Reserved24R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {}
#[doc = "WakeUp Time Register This register contains the measured wakeup times from STANDBY mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wutime::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wutime::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WutimeSpec;
impl crate::RegisterSpec for WutimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wutime::R`](R) reader structure"]
impl crate::Readable for WutimeSpec {}
#[doc = "`write(|w| ..)` method takes [`wutime::W`](W) writer structure"]
impl crate::Writable for WutimeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WUTIME to value 0"]
impl crate::Resettable for WutimeSpec {
    const RESET_VALUE: u32 = 0;
}
