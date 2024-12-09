#[doc = "Register `RVR` reader"]
pub type R = crate::R<RvrSpec>;
#[doc = "Register `RVR` writer"]
pub type W = crate::W<RvrSpec>;
#[doc = "Field `RELOAD` reader - 23:0\\]
The value to load into the SYST_CVR `FTSSS when the counter reaches 0"]
pub type ReloadR = crate::FieldReader<u32>;
#[doc = "Field `RELOAD` writer - 23:0\\]
The value to load into the SYST_CVR `FTSSS when the counter reaches 0"]
pub type ReloadW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `RESERVED24` reader - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
The value to load into the SYST_CVR `FTSSS when the counter reaches 0"]
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
The value to load into the SYST_CVR `FTSSS when the counter reaches 0"]
    #[inline(always)]
    #[must_use]
    pub fn reload(&mut self) -> ReloadW<RvrSpec> {
        ReloadW::new(self, 0)
    }
}
#[doc = "Provides access SysTick timer counter reload value `FTSSS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rvr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rvr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RvrSpec;
impl crate::RegisterSpec for RvrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rvr::R`](R) reader structure"]
impl crate::Readable for RvrSpec {}
#[doc = "`write(|w| ..)` method takes [`rvr::W`](W) writer structure"]
impl crate::Writable for RvrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RVR to value 0"]
impl crate::Resettable for RvrSpec {
    const RESET_VALUE: u32 = 0;
}
