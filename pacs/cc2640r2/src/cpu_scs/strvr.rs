#[doc = "Register `STRVR` reader"]
pub type R = crate::R<StrvrSpec>;
#[doc = "Register `STRVR` writer"]
pub type W = crate::W<StrvrSpec>;
#[doc = "Field `RELOAD` reader - 23:0\\]
Value to load into the SysTick Current Value Register STCVR.CURRENT when the counter reaches 0."]
pub type ReloadR = crate::FieldReader<u32>;
#[doc = "Field `RELOAD` writer - 23:0\\]
Value to load into the SysTick Current Value Register STCVR.CURRENT when the counter reaches 0."]
pub type ReloadW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `RESERVED24` reader - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24R = crate::FieldReader;
#[doc = "Field `RESERVED24` writer - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Value to load into the SysTick Current Value Register STCVR.CURRENT when the counter reaches 0."]
    #[inline(always)]
    pub fn reload(&self) -> ReloadR {
        ReloadR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> Reserved24R {
        Reserved24R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Value to load into the SysTick Current Value Register STCVR.CURRENT when the counter reaches 0."]
    #[inline(always)]
    #[must_use]
    pub fn reload(&mut self) -> ReloadW<StrvrSpec> {
        ReloadW::new(self, 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved24(&mut self) -> Reserved24W<StrvrSpec> {
        Reserved24W::new(self, 24)
    }
}
#[doc = "SysTick Reload Value This register is used to specify the start value to load into the current value register STCVR.CURRENT when the counter reaches 0. It can be any value between 1 and 0x00FFFFFF. A start value of 0 is possible, but has no effect because the SysTick interrupt and STCSR.COUNTFLAG are activated when counting from 1 to 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`strvr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`strvr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StrvrSpec;
impl crate::RegisterSpec for StrvrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`strvr::R`](R) reader structure"]
impl crate::Readable for StrvrSpec {}
#[doc = "`write(|w| ..)` method takes [`strvr::W`](W) writer structure"]
impl crate::Writable for StrvrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STRVR to value 0"]
impl crate::Resettable for StrvrSpec {
    const RESET_VALUE: u32 = 0;
}
