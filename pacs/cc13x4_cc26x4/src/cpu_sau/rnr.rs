#[doc = "Register `RNR` reader"]
pub type R = crate::R<RnrSpec>;
#[doc = "Register `RNR` writer"]
pub type W = crate::W<RnrSpec>;
#[doc = "Field `REGION` reader - 7:0\\]
Indicates the SAU region accessed by SAU_RBAR and SAU_RLAR"]
pub type RegionR = crate::FieldReader;
#[doc = "Field `REGION` writer - 7:0\\]
Indicates the SAU region accessed by SAU_RBAR and SAU_RLAR"]
pub type RegionW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Indicates the SAU region accessed by SAU_RBAR and SAU_RLAR"]
    #[inline(always)]
    pub fn region(&self) -> RegionR {
        RegionR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Indicates the SAU region accessed by SAU_RBAR and SAU_RLAR"]
    #[inline(always)]
    #[must_use]
    pub fn region(&mut self) -> RegionW<RnrSpec> {
        RegionW::new(self, 0)
    }
}
#[doc = "Selects the region currently accessed by SAU_RBAR and SAU_RLAR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rnr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rnr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RnrSpec;
impl crate::RegisterSpec for RnrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rnr::R`](R) reader structure"]
impl crate::Readable for RnrSpec {}
#[doc = "`write(|w| ..)` method takes [`rnr::W`](W) writer structure"]
impl crate::Writable for RnrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RNR to value 0"]
impl crate::Resettable for RnrSpec {
    const RESET_VALUE: u32 = 0;
}
