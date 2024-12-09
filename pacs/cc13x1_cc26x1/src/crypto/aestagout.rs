#[doc = "Register `AESTAGOUT` reader"]
pub type R = crate::R<AestagoutSpec>;
#[doc = "Register `AESTAGOUT` writer"]
pub type W = crate::W<AestagoutSpec>;
#[doc = "Field `TAG` reader - 31:0\\]
This register contains the authentication TAG for the combined and authentication-only modes."]
pub type TagR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register contains the authentication TAG for the combined and authentication-only modes."]
    #[inline(always)]
    pub fn tag(&self) -> TagR {
        TagR::new(self.bits)
    }
}
impl W {}
#[doc = "AES Tag Output\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aestagout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aestagout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AestagoutSpec;
impl crate::RegisterSpec for AestagoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aestagout::R`](R) reader structure"]
impl crate::Readable for AestagoutSpec {}
#[doc = "`write(|w| ..)` method takes [`aestagout::W`](W) writer structure"]
impl crate::Writable for AestagoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESTAGOUT to value 0"]
impl crate::Resettable for AestagoutSpec {
    const RESET_VALUE: u32 = 0;
}
