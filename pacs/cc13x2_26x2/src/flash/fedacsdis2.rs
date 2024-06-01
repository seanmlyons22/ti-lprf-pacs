#[doc = "Register `FEDACSDIS2` reader"]
pub type R = crate::R<Fedacsdis2Spec>;
#[doc = "Register `FEDACSDIS2` writer"]
pub type W = crate::W<Fedacsdis2Spec>;
#[doc = "Field `SECTORID2` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type Sectorid2R = crate::FieldReader<u32>;
#[doc = "Field `SECTORID2` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type Sectorid2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sectorid2(&self) -> Sectorid2R {
        Sectorid2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sectorid2(&mut self) -> Sectorid2W<Fedacsdis2Spec> {
        Sectorid2W::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fedacsdis2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fedacsdis2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fedacsdis2Spec;
impl crate::RegisterSpec for Fedacsdis2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fedacsdis2::R`](R) reader structure"]
impl crate::Readable for Fedacsdis2Spec {}
#[doc = "`write(|w| ..)` method takes [`fedacsdis2::W`](W) writer structure"]
impl crate::Writable for Fedacsdis2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FEDACSDIS2 to value 0"]
impl crate::Resettable for Fedacsdis2Spec {
    const RESET_VALUE: u32 = 0;
}
