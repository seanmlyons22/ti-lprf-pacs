#[doc = "Register `AESKEY2` reader"]
pub type R = crate::R<Aeskey2Spec>;
#[doc = "Register `AESKEY2` writer"]
pub type W = crate::W<Aeskey2Spec>;
#[doc = "Field `KEY2` writer - 31:0\\]
AESKEY2.* bits 31+x:0+x or AES_GHASH_H.* bits 31+x:0+x, where x = 0, 32, 64, 96 ordered from the LSW entry of this 4-deep register array. The interpretation of this field depends on the crypto operation mode."]
pub type Key2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
AESKEY2.* bits 31+x:0+x or AES_GHASH_H.* bits 31+x:0+x, where x = 0, 32, 64, 96 ordered from the LSW entry of this 4-deep register array. The interpretation of this field depends on the crypto operation mode."]
    #[inline(always)]
    #[must_use]
    pub fn key2(&mut self) -> Key2W<Aeskey2Spec> {
        Key2W::new(self, 0)
    }
}
#[doc = "Clear AES_KEY2/GHASH Key\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aeskey2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aeskey2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aeskey2Spec;
impl crate::RegisterSpec for Aeskey2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aeskey2::R`](R) reader structure"]
impl crate::Readable for Aeskey2Spec {}
#[doc = "`write(|w| ..)` method takes [`aeskey2::W`](W) writer structure"]
impl crate::Writable for Aeskey2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESKEY2 to value 0"]
impl crate::Resettable for Aeskey2Spec {
    const RESET_VALUE: u32 = 0;
}
