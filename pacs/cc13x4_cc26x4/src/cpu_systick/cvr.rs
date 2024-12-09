#[doc = "Register `CVR` reader"]
pub type R = crate::R<CvrSpec>;
#[doc = "Register `CVR` writer"]
pub type W = crate::W<CvrSpec>;
#[doc = "Field `CURRENT` writer - 23:0\\]
Writing any value clears the SysTick timer counter `FTSSS to zero"]
pub type CurrentW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `RESERVED24` reader - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24R = crate::FieldReader;
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> Reserved24R {
        Reserved24R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Writing any value clears the SysTick timer counter `FTSSS to zero"]
    #[inline(always)]
    #[must_use]
    pub fn current(&mut self) -> CurrentW<CvrSpec> {
        CurrentW::new(self, 0)
    }
}
#[doc = "Reads or clears the SysTick timer current counter value `FTSSS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cvr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cvr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CvrSpec;
impl crate::RegisterSpec for CvrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cvr::R`](R) reader structure"]
impl crate::Readable for CvrSpec {}
#[doc = "`write(|w| ..)` method takes [`cvr::W`](W) writer structure"]
impl crate::Writable for CvrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CVR to value 0"]
impl crate::Resettable for CvrSpec {
    const RESET_VALUE: u32 = 0;
}
