#[doc = "Register `PBISTCTL` reader"]
pub type R = crate::R<PbistctlSpec>;
#[doc = "Register `PBISTCTL` writer"]
pub type W = crate::W<PbistctlSpec>;
#[doc = "Field `PBIST_KEY` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type PbistKeyR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pbist_key(&self) -> PbistKeyR {
        PbistKeyR::new(self.bits)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pbistctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pbistctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbistctlSpec;
impl crate::RegisterSpec for PbistctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbistctl::R`](R) reader structure"]
impl crate::Readable for PbistctlSpec {}
#[doc = "`write(|w| ..)` method takes [`pbistctl::W`](W) writer structure"]
impl crate::Writable for PbistctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PBISTCTL to value 0"]
impl crate::Resettable for PbistctlSpec {
    const RESET_VALUE: u32 = 0;
}
