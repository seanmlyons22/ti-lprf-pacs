#[doc = "Register `FEDACCTL2` reader"]
pub type R = crate::R<Fedacctl2Spec>;
#[doc = "Register `FEDACCTL2` writer"]
pub type W = crate::W<Fedacctl2Spec>;
#[doc = "Field `SEC_THRESHOLD` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type SecThresholdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sec_threshold(&self) -> SecThresholdR {
        SecThresholdR::new(self.bits)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fedacctl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fedacctl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fedacctl2Spec;
impl crate::RegisterSpec for Fedacctl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fedacctl2::R`](R) reader structure"]
impl crate::Readable for Fedacctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`fedacctl2::W`](W) writer structure"]
impl crate::Writable for Fedacctl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FEDACCTL2 to value 0"]
impl crate::Resettable for Fedacctl2Spec {
    const RESET_VALUE: u32 = 0;
}
