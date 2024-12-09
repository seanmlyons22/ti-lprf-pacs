#[doc = "Register `FEDACSDIS` reader"]
pub type R = crate::R<FedacsdisSpec>;
#[doc = "Register `FEDACSDIS` writer"]
pub type W = crate::W<FedacsdisSpec>;
#[doc = "Field `SECTORID0` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type Sectorid0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sectorid0(&self) -> Sectorid0R {
        Sectorid0R::new(self.bits)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fedacsdis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fedacsdis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FedacsdisSpec;
impl crate::RegisterSpec for FedacsdisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fedacsdis::R`](R) reader structure"]
impl crate::Readable for FedacsdisSpec {}
#[doc = "`write(|w| ..)` method takes [`fedacsdis::W`](W) writer structure"]
impl crate::Writable for FedacsdisSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FEDACSDIS to value 0"]
impl crate::Resettable for FedacsdisSpec {
    const RESET_VALUE: u32 = 0;
}
