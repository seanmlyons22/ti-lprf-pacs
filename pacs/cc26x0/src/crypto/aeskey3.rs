#[doc = "Register `AESKEY3` reader"]
pub type R = crate::R<Aeskey3Spec>;
#[doc = "Register `AESKEY3` writer"]
pub type W = crate::W<Aeskey3Spec>;
#[doc = "Field `KEY3` reader - 31:0\\]
AESKEY3.* bits 31+x:0+x or AESKEY2.* bits 159+x:128+x, where x = 0, 32, 64, 96 ordered from the LSW entry of this 4-deep register arrary. The interpretation of this field depends on the crypto operation mode."]
pub type Key3R = crate::FieldReader<u32>;
#[doc = "Field `KEY3` writer - 31:0\\]
AESKEY3.* bits 31+x:0+x or AESKEY2.* bits 159+x:128+x, where x = 0, 32, 64, 96 ordered from the LSW entry of this 4-deep register arrary. The interpretation of this field depends on the crypto operation mode."]
pub type Key3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
AESKEY3.* bits 31+x:0+x or AESKEY2.* bits 159+x:128+x, where x = 0, 32, 64, 96 ordered from the LSW entry of this 4-deep register arrary. The interpretation of this field depends on the crypto operation mode."]
    #[inline(always)]
    pub fn key3(&self) -> Key3R {
        Key3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
AESKEY3.* bits 31+x:0+x or AESKEY2.* bits 159+x:128+x, where x = 0, 32, 64, 96 ordered from the LSW entry of this 4-deep register arrary. The interpretation of this field depends on the crypto operation mode."]
    #[inline(always)]
    #[must_use]
    pub fn key3(&mut self) -> Key3W<Aeskey3Spec> {
        Key3W::new(self, 0)
    }
}
#[doc = "Clear AES_KEY3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aeskey3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aeskey3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aeskey3Spec;
impl crate::RegisterSpec for Aeskey3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aeskey3::R`](R) reader structure"]
impl crate::Readable for Aeskey3Spec {}
#[doc = "`write(|w| ..)` method takes [`aeskey3::W`](W) writer structure"]
impl crate::Writable for Aeskey3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESKEY3 to value 0"]
impl crate::Resettable for Aeskey3Spec {
    const RESET_VALUE: u32 = 0;
}
