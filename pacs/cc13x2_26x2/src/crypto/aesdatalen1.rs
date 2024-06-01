#[doc = "Register `AESDATALEN1` reader"]
pub type R = crate::R<Aesdatalen1Spec>;
#[doc = "Register `AESDATALEN1` writer"]
pub type W = crate::W<Aesdatalen1Spec>;
#[doc = "Field `C_LENGTH` reader - 28:0\\]
C_LENGTH\\[60:32\\]
Bits \\[60:0\\]
of the crypto length registers (LSW and MSW) store the cryptographic data length in bytes for all modes. Once processing with this context is started, this length decrements to 0. Data lengths up to (261: 1) bytes are allowed. For GCM, any value up to 236 - 32 bytes can be used. This is because a 32-bit counter mode is used; the maximum number of 128-bit blocks is 232 - 2, resulting in a maximum number of bytes of 236 - 32. A write to this register triggers the engine to start using this context. This is valid for all modes except GCM and CCM. Note: For the combined modes (GCM and CCM), this length does not include the authentication only data; the authentication length is specified in the AESAUTHLEN register All modes must have a length greater than 0. For the combined modes, it is allowed to have one of the lengths equal to 0. For the basic encryption modes (ECB, CBC, and CTR) it is allowed to program zero to the length field; in that case the length is assumed infinite. All data must be byte (8-bit) aligned for stream cipher modes; bit aligned data streams are not supported by the EIP-120t. For block cipher modes, the data length must be programmed in multiples of the block cipher size, 16 bytes. For a host read operation, these registers return all-0s."]
pub type CLengthR = crate::FieldReader<u32>;
#[doc = "Field `C_LENGTH` writer - 28:0\\]
C_LENGTH\\[60:32\\]
Bits \\[60:0\\]
of the crypto length registers (LSW and MSW) store the cryptographic data length in bytes for all modes. Once processing with this context is started, this length decrements to 0. Data lengths up to (261: 1) bytes are allowed. For GCM, any value up to 236 - 32 bytes can be used. This is because a 32-bit counter mode is used; the maximum number of 128-bit blocks is 232 - 2, resulting in a maximum number of bytes of 236 - 32. A write to this register triggers the engine to start using this context. This is valid for all modes except GCM and CCM. Note: For the combined modes (GCM and CCM), this length does not include the authentication only data; the authentication length is specified in the AESAUTHLEN register All modes must have a length greater than 0. For the combined modes, it is allowed to have one of the lengths equal to 0. For the basic encryption modes (ECB, CBC, and CTR) it is allowed to program zero to the length field; in that case the length is assumed infinite. All data must be byte (8-bit) aligned for stream cipher modes; bit aligned data streams are not supported by the EIP-120t. For block cipher modes, the data length must be programmed in multiples of the block cipher size, 16 bytes. For a host read operation, these registers return all-0s."]
pub type CLengthW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
#[doc = "Field `RESERVED29` reader - 31:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved29R = crate::FieldReader;
#[doc = "Field `RESERVED29` writer - 31:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved29W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:28 - 28:0\\]
C_LENGTH\\[60:32\\]
Bits \\[60:0\\]
of the crypto length registers (LSW and MSW) store the cryptographic data length in bytes for all modes. Once processing with this context is started, this length decrements to 0. Data lengths up to (261: 1) bytes are allowed. For GCM, any value up to 236 - 32 bytes can be used. This is because a 32-bit counter mode is used; the maximum number of 128-bit blocks is 232 - 2, resulting in a maximum number of bytes of 236 - 32. A write to this register triggers the engine to start using this context. This is valid for all modes except GCM and CCM. Note: For the combined modes (GCM and CCM), this length does not include the authentication only data; the authentication length is specified in the AESAUTHLEN register All modes must have a length greater than 0. For the combined modes, it is allowed to have one of the lengths equal to 0. For the basic encryption modes (ECB, CBC, and CTR) it is allowed to program zero to the length field; in that case the length is assumed infinite. All data must be byte (8-bit) aligned for stream cipher modes; bit aligned data streams are not supported by the EIP-120t. For block cipher modes, the data length must be programmed in multiples of the block cipher size, 16 bytes. For a host read operation, these registers return all-0s."]
    #[inline(always)]
    pub fn c_length(&self) -> CLengthR {
        CLengthR::new(self.bits & 0x1fff_ffff)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved29(&self) -> Reserved29R {
        Reserved29R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:28 - 28:0\\]
C_LENGTH\\[60:32\\]
Bits \\[60:0\\]
of the crypto length registers (LSW and MSW) store the cryptographic data length in bytes for all modes. Once processing with this context is started, this length decrements to 0. Data lengths up to (261: 1) bytes are allowed. For GCM, any value up to 236 - 32 bytes can be used. This is because a 32-bit counter mode is used; the maximum number of 128-bit blocks is 232 - 2, resulting in a maximum number of bytes of 236 - 32. A write to this register triggers the engine to start using this context. This is valid for all modes except GCM and CCM. Note: For the combined modes (GCM and CCM), this length does not include the authentication only data; the authentication length is specified in the AESAUTHLEN register All modes must have a length greater than 0. For the combined modes, it is allowed to have one of the lengths equal to 0. For the basic encryption modes (ECB, CBC, and CTR) it is allowed to program zero to the length field; in that case the length is assumed infinite. All data must be byte (8-bit) aligned for stream cipher modes; bit aligned data streams are not supported by the EIP-120t. For block cipher modes, the data length must be programmed in multiples of the block cipher size, 16 bytes. For a host read operation, these registers return all-0s."]
    #[inline(always)]
    #[must_use]
    pub fn c_length(&mut self) -> CLengthW<Aesdatalen1Spec> {
        CLengthW::new(self, 0)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved29(&mut self) -> Reserved29W<Aesdatalen1Spec> {
        Reserved29W::new(self, 29)
    }
}
#[doc = "AES Crypto Length 1 (MSW) These registers are used to write the Length values to the EIP-120t. While processing, the length values decrement to 0. If both lengths are 0, the data stream is finished and a new context is requested. For basic AES modes (ECB, CBC, and CTR), a crypto length of 0 can be written if multiple streams need to be processed with the same key. Writing 0 length results in continued data requests until a new context is written. For the other modes (CBC-MAC, GCM and CCM) no (new) data requests are done if the length decrements to or equals 0. It is advised to write a new length per packet. If the length registers decrement to 0, no new data is processed until a new context or length value is written. When writing a new mode without writing the length registers, the length register values from the previous context is reused.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesdatalen1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesdatalen1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
