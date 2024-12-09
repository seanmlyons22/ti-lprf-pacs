#[doc = "Register `AESAUTHLEN` reader"]
pub type R = crate::R<AesauthlenSpec>;
#[doc = "Register `AESAUTHLEN` writer"]
pub type W = crate::W<AesauthlenSpec>;
#[doc = "Field `LEN` writer - 31:0\\]
Authentication data length in bytes for combined mode, CCM only. Supported AAD-lengths for CCM are from 0 to (216 - 28) bytes. Once processing with this context is started, this length decrements to zero. Writing this register triggers the engine to start using this context for CCM."]
pub type LenW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Authentication data length in bytes for combined mode, CCM only. Supported AAD-lengths for CCM are from 0 to (216 - 28) bytes. Once processing with this context is started, this length decrements to zero. Writing this register triggers the engine to start using this context for CCM."]
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LenW<AesauthlenSpec> {
        LenW::new(self, 0)
    }
}
#[doc = "AES Authentication Length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesauthlen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesauthlen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesauthlenSpec;
impl crate::RegisterSpec for AesauthlenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesauthlen::R`](R) reader structure"]
impl crate::Readable for AesauthlenSpec {}
#[doc = "`write(|w| ..)` method takes [`aesauthlen::W`](W) writer structure"]
impl crate::Writable for AesauthlenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESAUTHLEN to value 0"]
impl crate::Resettable for AesauthlenSpec {
    const RESET_VALUE: u32 = 0;
}
