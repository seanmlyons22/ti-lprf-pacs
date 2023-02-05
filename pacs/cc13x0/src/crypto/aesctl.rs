#[doc = "Register `AESCTL` reader"]
pub struct R(crate::R<AESCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AESCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AESCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AESCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AESCTL` writer"]
pub struct W(crate::W<AESCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AESCTL_SPEC>;
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
impl From<crate::W<AESCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AESCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTPUT_RDY` reader - 0:0\\]
If read as 1, this status bit indicates that an AES output block is available to be retrieved by the Host. Writing a 0 clears the bit to zero and indicates that output data is read by the Host. The AES engine can provide a next output data block. Writing a 1 to this bit will be ignored. Note: For DMA operations, this bit is automatically controlled by the Crypto peripheral. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
pub type OUTPUT_RDY_R = crate::BitReader<bool>;
#[doc = "Field `OUTPUT_RDY` writer - 0:0\\]
If read as 1, this status bit indicates that an AES output block is available to be retrieved by the Host. Writing a 0 clears the bit to zero and indicates that output data is read by the Host. The AES engine can provide a next output data block. Writing a 1 to this bit will be ignored. Note: For DMA operations, this bit is automatically controlled by the Crypto peripheral. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
pub type OUTPUT_RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, AESCTL_SPEC, bool, O>;
#[doc = "Field `INPUT_RDY` reader - 1:1\\]
If read as 1, this status bit indicates that the 16-byte AES input buffer is empty. The Host is permitted to write the next block of data. Writing a 0 clears the bit to zero and indicates that the AES engine can use the provided input data block. Writing a 1 to this bit will be ignored. Note: For DMA operations, this bit is automatically controlled by the Crypto peripheral. After reset, this bit is 0. After writing a context (note 1), this bit will become 1. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
pub type INPUT_RDY_R = crate::BitReader<bool>;
#[doc = "Field `INPUT_RDY` writer - 1:1\\]
If read as 1, this status bit indicates that the 16-byte AES input buffer is empty. The Host is permitted to write the next block of data. Writing a 0 clears the bit to zero and indicates that the AES engine can use the provided input data block. Writing a 1 to this bit will be ignored. Note: For DMA operations, this bit is automatically controlled by the Crypto peripheral. After reset, this bit is 0. After writing a context (note 1), this bit will become 1. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
pub type INPUT_RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, AESCTL_SPEC, bool, O>;
#[doc = "Field `DIR` reader - 2:2\\]
Direction. 0 : Decrypt operation is performed. 1 : Encrypt operation is performed. This bit must be written with a 1 when CBC-MAC is selected."]
pub type DIR_R = crate::BitReader<bool>;
#[doc = "Field `DIR` writer - 2:2\\]
Direction. 0 : Decrypt operation is performed. 1 : Encrypt operation is performed. This bit must be written with a 1 when CBC-MAC is selected."]
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, AESCTL_SPEC, bool, O>;
#[doc = "Field `KEY_SIZE` reader - 4:3\\]
This field specifies the key size. The key size is automatically configured when a new key is loaded via the key store module. 00 = N/A - reserved 01 = 128 bits 10 = N/A - reserved 11 = N/A - reserved For the Crypto peripheral this field is fixed to 128 bits."]
pub type KEY_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KEY_SIZE` writer - 4:3\\]
This field specifies the key size. The key size is automatically configured when a new key is loaded via the key store module. 00 = N/A - reserved 01 = 128 bits 10 = N/A - reserved 11 = N/A - reserved For the Crypto peripheral this field is fixed to 128 bits."]
pub type KEY_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AESCTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `CBC` reader - 5:5\\]
CBC mode enable"]
pub type CBC_R = crate::BitReader<bool>;
#[doc = "Field `CBC` writer - 5:5\\]
CBC mode enable"]
pub type CBC_W<'a, const O: u8> = crate::BitWriter<'a, u32, AESCTL_SPEC, bool, O>;
#[doc = "Field `CTR` reader - 6:6\\]
AES-CTR mode enable This bit must also be set for CCM, when encryption/decryption is required."]
pub type CTR_R = crate::BitReader<bool>;
#[doc = "Field `CTR` writer - 6:6\\]
AES-CTR mode enable This bit must also be set for CCM, when encryption/decryption is required."]
pub type CTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, AESCTL_SPEC, bool, O>;
#[doc = "Field `CTR_WIDTH` reader - 8:7\\]
Specifies the counter width for AES-CTR mode"]
pub type CTR_WIDTH_R = crate::FieldReader<u8, CTR_WIDTH_A>;
#[doc = "8:7\\]
Specifies the counter width for AES-CTR mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTR_WIDTH_A {
    #[doc = "3: 128 bits"]
    _128_BIT = 3,
    #[doc = "2: 96 bits"]
    _96_BIT = 2,
    #[doc = "1: 64 bits"]
    _64_BIT = 1,
    #[doc = "0: 32 bits"]
    _32_BIT = 0,
}
impl From<CTR_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: CTR_WIDTH_A) -> Self {
        variant as _
    }
}
impl CTR_WIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTR_WIDTH_A {
        match self.bits {
            3 => CTR_WIDTH_A::_128_BIT,
            2 => CTR_WIDTH_A::_96_BIT,
            1 => CTR_WIDTH_A::_64_BIT,
            0 => CTR_WIDTH_A::_32_BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_128_BIT`"]
    #[inline(always)]
    pub fn is_128_bit(&self) -> bool {
        *self == CTR_WIDTH_A::_128_BIT
    }
    #[doc = "Checks if the value of the field is `_96_BIT`"]
    #[inline(always)]
    pub fn is_96_bit(&self) -> bool {
        *self == CTR_WIDTH_A::_96_BIT
    }
    #[doc = "Checks if the value of the field is `_64_BIT`"]
    #[inline(always)]
    pub fn is_64_bit(&self) -> bool {
        *self == CTR_WIDTH_A::_64_BIT
    }
    #[doc = "Checks if the value of the field is `_32_BIT`"]
    #[inline(always)]
    pub fn is_32_bit(&self) -> bool {
        *self == CTR_WIDTH_A::_32_BIT
    }
}
#[doc = "Field `CTR_WIDTH` writer - 8:7\\]
Specifies the counter width for AES-CTR mode"]
pub type CTR_WIDTH_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AESCTL_SPEC, u8, CTR_WIDTH_A, 2, O>;
impl<'a, const O: u8> CTR_WIDTH_W<'a, O> {
    #[doc = "128 bits"]
    #[inline(always)]
    pub fn _128_bit(self) -> &'a mut W {
        self.variant(CTR_WIDTH_A::_128_BIT)
    }
    #[doc = "96 bits"]
    #[inline(always)]
    pub fn _96_bit(self) -> &'a mut W {
        self.variant(CTR_WIDTH_A::_96_BIT)
    }
    #[doc = "64 bits"]
    #[inline(always)]
    pub fn _64_bit(self) -> &'a mut W {
        self.variant(CTR_WIDTH_A::_64_BIT)
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn _32_bit(self) -> &'a mut W {
        self.variant(CTR_WIDTH_A::_32_BIT)
    }
}
#[doc = "Field `RESERVED9` reader - 14:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED9` writer - 14:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AESCTL_SPEC, u8, u8, 6, O>;
#[doc = "Field `CBC_MAC` reader - 15:15\\]
MAC mode enable. The DIR bit must be set to 1 for this mode. Selecting this mode requires writing the AESDATALEN1.LEN_MSW and AESDATALEN0.LEN_LSW registers after all other registers."]
pub type CBC_MAC_R = crate::BitReader<bool>;
#[doc = "Field `CBC_MAC` writer - 15:15\\]
MAC mode enable. The DIR bit must be set to 1 for this mode. Selecting this mode requires writing the AESDATALEN1.LEN_MSW and AESDATALEN0.LEN_LSW registers after all other registers."]
pub type CBC_MAC_W<'a, const O: u8> = crate::BitWriter<'a, u32, AESCTL_SPEC, bool, O>;
#[doc = "Field `CCM` reader - 18:18\\]
AES-CCM mode enable. AES-CCM is a combined mode, using AES for both authentication and encryption. Note: Selecting AES-CCM mode requires writing of AESDATALEN1.LEN_MSW and AESDATALEN0.LEN_LSW after all other registers. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR; selecting other AES modes than CTR mode is invalid."]
pub type CCM_R = crate::BitReader<bool>;
#[doc = "Field `CCM` writer - 18:18\\]
AES-CCM mode enable. AES-CCM is a combined mode, using AES for both authentication and encryption. Note: Selecting AES-CCM mode requires writing of AESDATALEN1.LEN_MSW and AESDATALEN0.LEN_LSW after all other registers. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR; selecting other AES modes than CTR mode is invalid."]
pub type CCM_W<'a, const O: u8> = crate::BitWriter<'a, u32, AESCTL_SPEC, bool, O>;
#[doc = "Field `CCM_L` reader - 21:19\\]
Defines L that indicates the width of the length field for CCM operations; the length field in bytes equals the value of CMM_L plus one. All values are supported."]
pub type CCM_L_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CCM_L` writer - 21:19\\]
Defines L that indicates the width of the length field for CCM operations; the length field in bytes equals the value of CMM_L plus one. All values are supported."]
pub type CCM_L_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AESCTL_SPEC, u8, u8, 3, O>;
#[doc = "Field `CCM_M` reader - 24:22\\]
Defines M that indicates the length of the authentication field for CCM operations; the authentication field length equals two times the value of CCM_M plus one. Note: The Crypto peripheral always returns a 128-bit authentication field, of which the M least significant bytes are valid. All values are supported."]
pub type CCM_M_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CCM_M` writer - 24:22\\]
Defines M that indicates the length of the authentication field for CCM operations; the authentication field length equals two times the value of CCM_M plus one. Note: The Crypto peripheral always returns a 128-bit authentication field, of which the M least significant bytes are valid. All values are supported."]
pub type CCM_M_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AESCTL_SPEC, u8, u8, 3, O>;
#[doc = "Field `RESERVED25` reader - 28:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED25_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED25` writer - 28:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED25_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AESCTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `SAVE_CONTEXT` reader - 29:29\\]
IV must be read before the AES engine can start a new operation."]
pub type SAVE_CONTEXT_R = crate::BitReader<bool>;
#[doc = "Field `SAVE_CONTEXT` writer - 29:29\\]
IV must be read before the AES engine can start a new operation."]
pub type SAVE_CONTEXT_W<'a, const O: u8> = crate::BitWriter<'a, u32, AESCTL_SPEC, bool, O>;
#[doc = "Field `SAVED_CONTEXT_RDY` reader - 30:30\\]
If read as 1, this status bit indicates that an AES authentication TAG and/or IV block(s) is/are available for the Host to retrieve. This bit is only asserted if SAVE_CONTEXT is set to 1. The bit is mutually exclusive with CONTEXT_RDY. Writing 1 clears the bit to zero, indicating the Crypto peripheral can start its next operation. This bit is also cleared when the 4th word of the output TAG and/or IV is read. Note: All other mode bit writes will be ignored when this mode bit is written with 1. Note: This bit is controlled automatically by the Crypto peripheral for TAG read DMA operations. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
pub type SAVED_CONTEXT_RDY_R = crate::BitReader<bool>;
#[doc = "Field `SAVED_CONTEXT_RDY` writer - 30:30\\]
If read as 1, this status bit indicates that an AES authentication TAG and/or IV block(s) is/are available for the Host to retrieve. This bit is only asserted if SAVE_CONTEXT is set to 1. The bit is mutually exclusive with CONTEXT_RDY. Writing 1 clears the bit to zero, indicating the Crypto peripheral can start its next operation. This bit is also cleared when the 4th word of the output TAG and/or IV is read. Note: All other mode bit writes will be ignored when this mode bit is written with 1. Note: This bit is controlled automatically by the Crypto peripheral for TAG read DMA operations. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
pub type SAVED_CONTEXT_RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, AESCTL_SPEC, bool, O>;
#[doc = "Field `CONTEXT_RDY` reader - 31:31\\]
If 1, this status bit indicates that the context data registers can be overwritten and the Host is permitted to write the next context. Writing a context means writing either a mode, the crypto length or AESDATALEN1.LEN_MSW, AESDATALEN0.LEN_LSW length registers"]
pub type CONTEXT_RDY_R = crate::BitReader<bool>;
#[doc = "Field `CONTEXT_RDY` writer - 31:31\\]
If 1, this status bit indicates that the context data registers can be overwritten and the Host is permitted to write the next context. Writing a context means writing either a mode, the crypto length or AESDATALEN1.LEN_MSW, AESDATALEN0.LEN_LSW length registers"]
pub type CONTEXT_RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, AESCTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
If read as 1, this status bit indicates that an AES output block is available to be retrieved by the Host. Writing a 0 clears the bit to zero and indicates that output data is read by the Host. The AES engine can provide a next output data block. Writing a 1 to this bit will be ignored. Note: For DMA operations, this bit is automatically controlled by the Crypto peripheral. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
    #[inline(always)]
    pub fn output_rdy(&self) -> OUTPUT_RDY_R {
        OUTPUT_RDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
If read as 1, this status bit indicates that the 16-byte AES input buffer is empty. The Host is permitted to write the next block of data. Writing a 0 clears the bit to zero and indicates that the AES engine can use the provided input data block. Writing a 1 to this bit will be ignored. Note: For DMA operations, this bit is automatically controlled by the Crypto peripheral. After reset, this bit is 0. After writing a context (note 1), this bit will become 1. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
    #[inline(always)]
    pub fn input_rdy(&self) -> INPUT_RDY_R {
        INPUT_RDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Direction. 0 : Decrypt operation is performed. 1 : Encrypt operation is performed. This bit must be written with a 1 when CBC-MAC is selected."]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - 4:3\\]
This field specifies the key size. The key size is automatically configured when a new key is loaded via the key store module. 00 = N/A - reserved 01 = 128 bits 10 = N/A - reserved 11 = N/A - reserved For the Crypto peripheral this field is fixed to 128 bits."]
    #[inline(always)]
    pub fn key_size(&self) -> KEY_SIZE_R {
        KEY_SIZE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
CBC mode enable"]
    #[inline(always)]
    pub fn cbc(&self) -> CBC_R {
        CBC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
AES-CTR mode enable This bit must also be set for CCM, when encryption/decryption is required."]
    #[inline(always)]
    pub fn ctr(&self) -> CTR_R {
        CTR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - 8:7\\]
Specifies the counter width for AES-CTR mode"]
    #[inline(always)]
    pub fn ctr_width(&self) -> CTR_WIDTH_R {
        CTR_WIDTH_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:14 - 14:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 0x3f) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
MAC mode enable. The DIR bit must be set to 1 for this mode. Selecting this mode requires writing the AESDATALEN1.LEN_MSW and AESDATALEN0.LEN_LSW registers after all other registers."]
    #[inline(always)]
    pub fn cbc_mac(&self) -> CBC_MAC_R {
        CBC_MAC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
AES-CCM mode enable. AES-CCM is a combined mode, using AES for both authentication and encryption. Note: Selecting AES-CCM mode requires writing of AESDATALEN1.LEN_MSW and AESDATALEN0.LEN_LSW after all other registers. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR; selecting other AES modes than CTR mode is invalid."]
    #[inline(always)]
    pub fn ccm(&self) -> CCM_R {
        CCM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:21 - 21:19\\]
Defines L that indicates the width of the length field for CCM operations; the length field in bytes equals the value of CMM_L plus one. All values are supported."]
    #[inline(always)]
    pub fn ccm_l(&self) -> CCM_L_R {
        CCM_L_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:24 - 24:22\\]
Defines M that indicates the length of the authentication field for CCM operations; the authentication field length equals two times the value of CCM_M plus one. Note: The Crypto peripheral always returns a 128-bit authentication field, of which the M least significant bytes are valid. All values are supported."]
    #[inline(always)]
    pub fn ccm_m(&self) -> CCM_M_R {
        CCM_M_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:28 - 28:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved25(&self) -> RESERVED25_R {
        RESERVED25_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - 29:29\\]
IV must be read before the AES engine can start a new operation."]
    #[inline(always)]
    pub fn save_context(&self) -> SAVE_CONTEXT_R {
        SAVE_CONTEXT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
If read as 1, this status bit indicates that an AES authentication TAG and/or IV block(s) is/are available for the Host to retrieve. This bit is only asserted if SAVE_CONTEXT is set to 1. The bit is mutually exclusive with CONTEXT_RDY. Writing 1 clears the bit to zero, indicating the Crypto peripheral can start its next operation. This bit is also cleared when the 4th word of the output TAG and/or IV is read. Note: All other mode bit writes will be ignored when this mode bit is written with 1. Note: This bit is controlled automatically by the Crypto peripheral for TAG read DMA operations. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
    #[inline(always)]
    pub fn saved_context_rdy(&self) -> SAVED_CONTEXT_RDY_R {
        SAVED_CONTEXT_RDY_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
If 1, this status bit indicates that the context data registers can be overwritten and the Host is permitted to write the next context. Writing a context means writing either a mode, the crypto length or AESDATALEN1.LEN_MSW, AESDATALEN0.LEN_LSW length registers"]
    #[inline(always)]
    pub fn context_rdy(&self) -> CONTEXT_RDY_R {
        CONTEXT_RDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
If read as 1, this status bit indicates that an AES output block is available to be retrieved by the Host. Writing a 0 clears the bit to zero and indicates that output data is read by the Host. The AES engine can provide a next output data block. Writing a 1 to this bit will be ignored. Note: For DMA operations, this bit is automatically controlled by the Crypto peripheral. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
    #[inline(always)]
    #[must_use]
    pub fn output_rdy(&mut self) -> OUTPUT_RDY_W<0> {
        OUTPUT_RDY_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
If read as 1, this status bit indicates that the 16-byte AES input buffer is empty. The Host is permitted to write the next block of data. Writing a 0 clears the bit to zero and indicates that the AES engine can use the provided input data block. Writing a 1 to this bit will be ignored. Note: For DMA operations, this bit is automatically controlled by the Crypto peripheral. After reset, this bit is 0. After writing a context (note 1), this bit will become 1. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
    #[inline(always)]
    #[must_use]
    pub fn input_rdy(&mut self) -> INPUT_RDY_W<1> {
        INPUT_RDY_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Direction. 0 : Decrypt operation is performed. 1 : Encrypt operation is performed. This bit must be written with a 1 when CBC-MAC is selected."]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<2> {
        DIR_W::new(self)
    }
    #[doc = "Bits 3:4 - 4:3\\]
This field specifies the key size. The key size is automatically configured when a new key is loaded via the key store module. 00 = N/A - reserved 01 = 128 bits 10 = N/A - reserved 11 = N/A - reserved For the Crypto peripheral this field is fixed to 128 bits."]
    #[inline(always)]
    #[must_use]
    pub fn key_size(&mut self) -> KEY_SIZE_W<3> {
        KEY_SIZE_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
CBC mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn cbc(&mut self) -> CBC_W<5> {
        CBC_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
AES-CTR mode enable This bit must also be set for CCM, when encryption/decryption is required."]
    #[inline(always)]
    #[must_use]
    pub fn ctr(&mut self) -> CTR_W<6> {
        CTR_W::new(self)
    }
    #[doc = "Bits 7:8 - 8:7\\]
Specifies the counter width for AES-CTR mode"]
    #[inline(always)]
    #[must_use]
    pub fn ctr_width(&mut self) -> CTR_WIDTH_W<7> {
        CTR_WIDTH_W::new(self)
    }
    #[doc = "Bits 9:14 - 14:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> RESERVED9_W<9> {
        RESERVED9_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
MAC mode enable. The DIR bit must be set to 1 for this mode. Selecting this mode requires writing the AESDATALEN1.LEN_MSW and AESDATALEN0.LEN_LSW registers after all other registers."]
    #[inline(always)]
    #[must_use]
    pub fn cbc_mac(&mut self) -> CBC_MAC_W<15> {
        CBC_MAC_W::new(self)
    }
    #[doc = "Bit 18 - 18:18\\]
AES-CCM mode enable. AES-CCM is a combined mode, using AES for both authentication and encryption. Note: Selecting AES-CCM mode requires writing of AESDATALEN1.LEN_MSW and AESDATALEN0.LEN_LSW after all other registers. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR; selecting other AES modes than CTR mode is invalid."]
    #[inline(always)]
    #[must_use]
    pub fn ccm(&mut self) -> CCM_W<18> {
        CCM_W::new(self)
    }
    #[doc = "Bits 19:21 - 21:19\\]
Defines L that indicates the width of the length field for CCM operations; the length field in bytes equals the value of CMM_L plus one. All values are supported."]
    #[inline(always)]
    #[must_use]
    pub fn ccm_l(&mut self) -> CCM_L_W<19> {
        CCM_L_W::new(self)
    }
    #[doc = "Bits 22:24 - 24:22\\]
Defines M that indicates the length of the authentication field for CCM operations; the authentication field length equals two times the value of CCM_M plus one. Note: The Crypto peripheral always returns a 128-bit authentication field, of which the M least significant bytes are valid. All values are supported."]
    #[inline(always)]
    #[must_use]
    pub fn ccm_m(&mut self) -> CCM_M_W<22> {
        CCM_M_W::new(self)
    }
    #[doc = "Bits 25:28 - 28:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved25(&mut self) -> RESERVED25_W<25> {
        RESERVED25_W::new(self)
    }
    #[doc = "Bit 29 - 29:29\\]
IV must be read before the AES engine can start a new operation."]
    #[inline(always)]
    #[must_use]
    pub fn save_context(&mut self) -> SAVE_CONTEXT_W<29> {
        SAVE_CONTEXT_W::new(self)
    }
    #[doc = "Bit 30 - 30:30\\]
If read as 1, this status bit indicates that an AES authentication TAG and/or IV block(s) is/are available for the Host to retrieve. This bit is only asserted if SAVE_CONTEXT is set to 1. The bit is mutually exclusive with CONTEXT_RDY. Writing 1 clears the bit to zero, indicating the Crypto peripheral can start its next operation. This bit is also cleared when the 4th word of the output TAG and/or IV is read. Note: All other mode bit writes will be ignored when this mode bit is written with 1. Note: This bit is controlled automatically by the Crypto peripheral for TAG read DMA operations. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
    #[inline(always)]
    #[must_use]
    pub fn saved_context_rdy(&mut self) -> SAVED_CONTEXT_RDY_W<30> {
        SAVED_CONTEXT_RDY_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
If 1, this status bit indicates that the context data registers can be overwritten and the Host is permitted to write the next context. Writing a context means writing either a mode, the crypto length or AESDATALEN1.LEN_MSW, AESDATALEN0.LEN_LSW length registers"]
    #[inline(always)]
    #[must_use]
    pub fn context_rdy(&mut self) -> CONTEXT_RDY_W<31> {
        CONTEXT_RDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES Input/Output Buffer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesctl](index.html) module"]
pub struct AESCTL_SPEC;
impl crate::RegisterSpec for AESCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aesctl::R](R) reader structure"]
impl crate::Readable for AESCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aesctl::W](W) writer structure"]
impl crate::Writable for AESCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AESCTL to value 0x8000_0000"]
impl crate::Resettable for AESCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
