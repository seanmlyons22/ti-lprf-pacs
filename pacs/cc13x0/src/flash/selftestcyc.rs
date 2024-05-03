#[doc = "Register `SELFTESTCYC` reader"]
pub type R = crate::R<SelftestcycSpec>;
#[doc = "Register `SELFTESTCYC` writer"]
pub type W = crate::W<SelftestcycSpec>;
#[doc = "Field `CYCLES` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type CyclesR = crate::FieldReader<u32>;
#[doc = "Field `CYCLES` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type CyclesW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cycles(&self) -> CyclesR {
        CyclesR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn cycles(&mut self) -> CyclesW<SelftestcycSpec> {
        CyclesW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`selftestcyc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`selftestcyc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SelftestcycSpec;
impl crate::RegisterSpec for SelftestcycSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`selftestcyc::R`](R) reader structure"]
impl crate::Readable for SelftestcycSpec {}
#[doc = "`write(|w| ..)` method takes [`selftestcyc::W`](W) writer structure"]
impl crate::Writable for SelftestcycSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SELFTESTCYC to value 0"]
impl crate::Resettable for SelftestcycSpec {
    const RESET_VALUE: u32 = 0;
}
