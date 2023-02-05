#[doc = "Register `AESDATALEN1` reader"]
pub struct R(crate::R<AESDATALEN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AESDATALEN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AESDATALEN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AESDATALEN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AESDATALEN1` writer"]
pub struct W(crate::W<AESDATALEN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AESDATALEN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<AESDATALEN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AESDATALEN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LEN_MSW` reader - 28:0\\]
Bits \\[60:32\\]
of the combined data length. Bits \\[60:0\\]
of the crypto length registers AESDATALEN1 and AESDATALEN0 store the cryptographic data length in bytes for all modes. Once processing with this context is started, this length decrements to zero. Data lengths up to (2^61 - 1) bytes are allowed. For GCM, any value up to 2^36 - 32 bytes can be used. This is because a 32-bit counter mode is used; the maximum number of 128-bit blocks is 2^32 - 2, resulting in a maximum number of bytes of 2^36 - 32. Writing to this register triggers the engine to start using this context. This is valid for all modes except GCM and CCM. Note: For the combined modes (GCM and CCM), this length does not include the authentication only data; the authentication length is specified in the AESAUTHLEN.LEN. All modes must have a length > 0. For the combined modes, it is allowed to have one of the lengths equal to zero. For the basic encryption modes (ECB/CBC/CTR) it is allowed to program zero to the length field; in that case the length is assumed infinite. All data must be byte (8-bit) aligned for stream cipher modes; bit aligned data streams are not supported by the Crypto peripheral. For block cipher modes, the data length must be programmed in multiples of the block cipher size, 16 bytes."]
pub type LEN_MSW_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LEN_MSW` writer - 28:0\\]
Bits \\[60:32\\]
of the combined data length. Bits \\[60:0\\]
of the crypto length registers AESDATALEN1 and AESDATALEN0 store the cryptographic data length in bytes for all modes. Once processing with this context is started, this length decrements to zero. Data lengths up to (2^61 - 1) bytes are allowed. For GCM, any value up to 2^36 - 32 bytes can be used. This is because a 32-bit counter mode is used; the maximum number of 128-bit blocks is 2^32 - 2, resulting in a maximum number of bytes of 2^36 - 32. Writing to this register triggers the engine to start using this context. This is valid for all modes except GCM and CCM. Note: For the combined modes (GCM and CCM), this length does not include the authentication only data; the authentication length is specified in the AESAUTHLEN.LEN. All modes must have a length > 0. For the combined modes, it is allowed to have one of the lengths equal to zero. For the basic encryption modes (ECB/CBC/CTR) it is allowed to program zero to the length field; in that case the length is assumed infinite. All data must be byte (8-bit) aligned for stream cipher modes; bit aligned data streams are not supported by the Crypto peripheral. For block cipher modes, the data length must be programmed in multiples of the block cipher size, 16 bytes."]
pub type LEN_MSW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AESDATALEN1_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bits 0:28 - 28:0\\]
Bits \\[60:32\\]
of the combined data length. Bits \\[60:0\\]
of the crypto length registers AESDATALEN1 and AESDATALEN0 store the cryptographic data length in bytes for all modes. Once processing with this context is started, this length decrements to zero. Data lengths up to (2^61 - 1) bytes are allowed. For GCM, any value up to 2^36 - 32 bytes can be used. This is because a 32-bit counter mode is used; the maximum number of 128-bit blocks is 2^32 - 2, resulting in a maximum number of bytes of 2^36 - 32. Writing to this register triggers the engine to start using this context. This is valid for all modes except GCM and CCM. Note: For the combined modes (GCM and CCM), this length does not include the authentication only data; the authentication length is specified in the AESAUTHLEN.LEN. All modes must have a length > 0. For the combined modes, it is allowed to have one of the lengths equal to zero. For the basic encryption modes (ECB/CBC/CTR) it is allowed to program zero to the length field; in that case the length is assumed infinite. All data must be byte (8-bit) aligned for stream cipher modes; bit aligned data streams are not supported by the Crypto peripheral. For block cipher modes, the data length must be programmed in multiples of the block cipher size, 16 bytes."]
    #[inline(always)]
    pub fn len_msw(&self) -> LEN_MSW_R {
        LEN_MSW_R::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - 28:0\\]
Bits \\[60:32\\]
of the combined data length. Bits \\[60:0\\]
of the crypto length registers AESDATALEN1 and AESDATALEN0 store the cryptographic data length in bytes for all modes. Once processing with this context is started, this length decrements to zero. Data lengths up to (2^61 - 1) bytes are allowed. For GCM, any value up to 2^36 - 32 bytes can be used. This is because a 32-bit counter mode is used; the maximum number of 128-bit blocks is 2^32 - 2, resulting in a maximum number of bytes of 2^36 - 32. Writing to this register triggers the engine to start using this context. This is valid for all modes except GCM and CCM. Note: For the combined modes (GCM and CCM), this length does not include the authentication only data; the authentication length is specified in the AESAUTHLEN.LEN. All modes must have a length > 0. For the combined modes, it is allowed to have one of the lengths equal to zero. For the basic encryption modes (ECB/CBC/CTR) it is allowed to program zero to the length field; in that case the length is assumed infinite. All data must be byte (8-bit) aligned for stream cipher modes; bit aligned data streams are not supported by the Crypto peripheral. For block cipher modes, the data length must be programmed in multiples of the block cipher size, 16 bytes."]
    #[inline(always)]
    #[must_use]
    pub fn len_msw(&mut self) -> LEN_MSW_W<0> {
        LEN_MSW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crypto Data Length MSW\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesdatalen1](index.html) module"]
pub struct AESDATALEN1_SPEC;
impl crate::RegisterSpec for AESDATALEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aesdatalen1::R](R) reader structure"]
impl crate::Readable for AESDATALEN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aesdatalen1::W](W) writer structure"]
impl crate::Writable for AESDATALEN1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AESDATALEN1 to value 0"]
impl crate::Resettable for AESDATALEN1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
