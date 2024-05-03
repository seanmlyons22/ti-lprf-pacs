#[doc = "Register `AESAUTHLEN` reader"]
pub type R = crate::R<AesauthlenSpec>;
#[doc = "Register `AESAUTHLEN` writer"]
pub type W = crate::W<AesauthlenSpec>;
#[doc = "Field `AUTH_LENGTH` reader - 31:0\\]
Bits \\[31:0\\]
of the authentication length register store the authentication data length in bytes for combined modes only (GCM or CCM). Supported AAD-lengths for CCM are from 0 to (2^16 - 2^8) bytes. For GCM any value up to (2^32 - 1) bytes can be used. Once processing with this context is started, this length decrements to 0. A write to this register triggers the engine to start using this context for GCM and CCM. For a host read operation, these registers return all-0s."]
pub type AuthLengthR = crate::FieldReader<u32>;
#[doc = "Field `AUTH_LENGTH` writer - 31:0\\]
Bits \\[31:0\\]
of the authentication length register store the authentication data length in bytes for combined modes only (GCM or CCM). Supported AAD-lengths for CCM are from 0 to (2^16 - 2^8) bytes. For GCM any value up to (2^32 - 1) bytes can be used. Once processing with this context is started, this length decrements to 0. A write to this register triggers the engine to start using this context for GCM and CCM. For a host read operation, these registers return all-0s."]
pub type AuthLengthW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Bits \\[31:0\\]
of the authentication length register store the authentication data length in bytes for combined modes only (GCM or CCM). Supported AAD-lengths for CCM are from 0 to (2^16 - 2^8) bytes. For GCM any value up to (2^32 - 1) bytes can be used. Once processing with this context is started, this length decrements to 0. A write to this register triggers the engine to start using this context for GCM and CCM. For a host read operation, these registers return all-0s."]
    #[inline(always)]
    pub fn auth_length(&self) -> AuthLengthR {
        AuthLengthR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Bits \\[31:0\\]
of the authentication length register store the authentication data length in bytes for combined modes only (GCM or CCM). Supported AAD-lengths for CCM are from 0 to (2^16 - 2^8) bytes. For GCM any value up to (2^32 - 1) bytes can be used. Once processing with this context is started, this length decrements to 0. A write to this register triggers the engine to start using this context for GCM and CCM. For a host read operation, these registers return all-0s."]
    #[inline(always)]
    #[must_use]
    pub fn auth_length(&mut self) -> AuthLengthW<AesauthlenSpec> {
        AuthLengthW::new(self, 0)
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
