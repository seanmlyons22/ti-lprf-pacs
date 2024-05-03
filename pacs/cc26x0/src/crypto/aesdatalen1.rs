#[doc = "Register `AESDATALEN1` reader"]
pub type R = crate::R<Aesdatalen1Spec>;
#[doc = "Register `AESDATALEN1` writer"]
pub type W = crate::W<Aesdatalen1Spec>;
#[doc = "Field `LEN_MSW` reader - 28:0\\]
Bits \\[60:32\\]
of the combined data length. Bits \\[60:0\\]
of the crypto length registers AESDATALEN1 and AESDATALEN0 store the cryptographic data length in bytes for all modes. Once processing with this context is started, this length decrements to zero. Data lengths up to (2^61 - 1) bytes are allowed. For GCM, any value up to 2^36 - 32 bytes can be used. This is because a 32-bit counter mode is used; the maximum number of 128-bit blocks is 2^32 - 2, resulting in a maximum number of bytes of 2^36 - 32. Writing to this register triggers the engine to start using this context. This is valid for all modes except GCM and CCM. Note: For the combined modes (GCM and CCM), this length does not include the authentication only data; the authentication length is specified in the AESAUTHLEN.LEN. All modes must have a length > 0. For the combined modes, it is allowed to have one of the lengths equal to zero. For the basic encryption modes (ECB/CBC/CTR) it is allowed to program zero to the length field; in that case the length is assumed infinite. All data must be byte (8-bit) aligned for stream cipher modes; bit aligned data streams are not supported by the Crypto peripheral. For block cipher modes, the data length must be programmed in multiples of the block cipher size, 16 bytes."]
pub type LenMswR = crate::FieldReader<u32>;
#[doc = "Field `LEN_MSW` writer - 28:0\\]
Bits \\[60:32\\]
of the combined data length. Bits \\[60:0\\]
of the crypto length registers AESDATALEN1 and AESDATALEN0 store the cryptographic data length in bytes for all modes. Once processing with this context is started, this length decrements to zero. Data lengths up to (2^61 - 1) bytes are allowed. For GCM, any value up to 2^36 - 32 bytes can be used. This is because a 32-bit counter mode is used; the maximum number of 128-bit blocks is 2^32 - 2, resulting in a maximum number of bytes of 2^36 - 32. Writing to this register triggers the engine to start using this context. This is valid for all modes except GCM and CCM. Note: For the combined modes (GCM and CCM), this length does not include the authentication only data; the authentication length is specified in the AESAUTHLEN.LEN. All modes must have a length > 0. For the combined modes, it is allowed to have one of the lengths equal to zero. For the basic encryption modes (ECB/CBC/CTR) it is allowed to program zero to the length field; in that case the length is assumed infinite. All data must be byte (8-bit) aligned for stream cipher modes; bit aligned data streams are not supported by the Crypto peripheral. For block cipher modes, the data length must be programmed in multiples of the block cipher size, 16 bytes."]
pub type LenMswW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28 - 28:0\\]
Bits \\[60:32\\]
of the combined data length. Bits \\[60:0\\]
of the crypto length registers AESDATALEN1 and AESDATALEN0 store the cryptographic data length in bytes for all modes. Once processing with this context is started, this length decrements to zero. Data lengths up to (2^61 - 1) bytes are allowed. For GCM, any value up to 2^36 - 32 bytes can be used. This is because a 32-bit counter mode is used; the maximum number of 128-bit blocks is 2^32 - 2, resulting in a maximum number of bytes of 2^36 - 32. Writing to this register triggers the engine to start using this context. This is valid for all modes except GCM and CCM. Note: For the combined modes (GCM and CCM), this length does not include the authentication only data; the authentication length is specified in the AESAUTHLEN.LEN. All modes must have a length > 0. For the combined modes, it is allowed to have one of the lengths equal to zero. For the basic encryption modes (ECB/CBC/CTR) it is allowed to program zero to the length field; in that case the length is assumed infinite. All data must be byte (8-bit) aligned for stream cipher modes; bit aligned data streams are not supported by the Crypto peripheral. For block cipher modes, the data length must be programmed in multiples of the block cipher size, 16 bytes."]
    #[inline(always)]
    pub fn len_msw(&self) -> LenMswR {
        LenMswR::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - 28:0\\]
Bits \\[60:32\\]
of the combined data length. Bits \\[60:0\\]
of the crypto length registers AESDATALEN1 and AESDATALEN0 store the cryptographic data length in bytes for all modes. Once processing with this context is started, this length decrements to zero. Data lengths up to (2^61 - 1) bytes are allowed. For GCM, any value up to 2^36 - 32 bytes can be used. This is because a 32-bit counter mode is used; the maximum number of 128-bit blocks is 2^32 - 2, resulting in a maximum number of bytes of 2^36 - 32. Writing to this register triggers the engine to start using this context. This is valid for all modes except GCM and CCM. Note: For the combined modes (GCM and CCM), this length does not include the authentication only data; the authentication length is specified in the AESAUTHLEN.LEN. All modes must have a length > 0. For the combined modes, it is allowed to have one of the lengths equal to zero. For the basic encryption modes (ECB/CBC/CTR) it is allowed to program zero to the length field; in that case the length is assumed infinite. All data must be byte (8-bit) aligned for stream cipher modes; bit aligned data streams are not supported by the Crypto peripheral. For block cipher modes, the data length must be programmed in multiples of the block cipher size, 16 bytes."]
    #[inline(always)]
    #[must_use]
    pub fn len_msw(&mut self) -> LenMswW<Aesdatalen1Spec> {
        LenMswW::new(self, 0)
    }
}
#[doc = "Crypto Data Length MSW\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesdatalen1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesdatalen1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aesdatalen1Spec;
impl crate::RegisterSpec for Aesdatalen1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesdatalen1::R`](R) reader structure"]
impl crate::Readable for Aesdatalen1Spec {}
#[doc = "`write(|w| ..)` method takes [`aesdatalen1::W`](W) writer structure"]
impl crate::Writable for Aesdatalen1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESDATALEN1 to value 0"]
impl crate::Resettable for Aesdatalen1Spec {
    const RESET_VALUE: u32 = 0;
}
