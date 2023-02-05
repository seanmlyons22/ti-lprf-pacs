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
#[doc = "Field `OUTPUT_READY` reader - 0:0\\]
If 1, this status bit indicates that an AES output block is available to be retrieved by the host. Writing 0 clears the bit to 0 and indicates that output data is read by the host. The AES core can provide a next output data block. Writing 1 to this bit is ignored. Note: For DMA operations, this bit is automatically controlled by the EIP-120t."]
pub type OUTPUT_READY_R = crate::BitReader<bool>;
#[doc = "Field `OUTPUT_READY` writer - 0:0\\]
If 1, this status bit indicates that an AES output block is available to be retrieved by the host. Writing 0 clears the bit to 0 and indicates that output data is read by the host. The AES core can provide a next output data block. Writing 1 to this bit is ignored. Note: For DMA operations, this bit is automatically controlled by the EIP-120t."]
pub type OUTPUT_READY_W<'a, const O: u8> = crate::BitWriter<'a, u32, AESCTL_SPEC, bool, O>;
#[doc = "Field `INPUT_READY` reader - 1:1\\]
If 1, this status bit indicates that the 16-byte AES input buffer is empty. The host is permitted to write the next block of data. Writing 0 clears the bit to 0 and indicates that the AES core can use the provided input data block. Writing 1 to this bit is ignored. Note: For DMA operations, this bit is automatically controlled by the EIP-120t. After reset, this bit is 0. After writing a context, this bit becomes 1."]
pub type INPUT_READY_R = crate::BitReader<bool>;
#[doc = "Field `INPUT_READY` writer - 1:1\\]
If 1, this status bit indicates that the 16-byte AES input buffer is empty. The host is permitted to write the next block of data. Writing 0 clears the bit to 0 and indicates that the AES core can use the provided input data block. Writing 1 to this bit is ignored. Note: For DMA operations, this bit is automatically controlled by the EIP-120t. After reset, this bit is 0. After writing a context, this bit becomes 1."]
pub type INPUT_READY_W<'a, const O: u8> = crate::BitWriter<'a, u32, AESCTL_SPEC, bool, O>;
#[doc = "Field `DIR` reader - 2:2\\]
If set to 1 an encrypt operation is performed. If set to 0 a decrypt operation is performed. This bit must be written with a 1 when CBC-MAC is selected."]
pub type DIR_R = crate::BitReader<bool>;
#[doc = "Field `DIR` writer - 2:2\\]
If set to 1 an encrypt operation is performed. If set to 0 a decrypt operation is performed. This bit must be written with a 1 when CBC-MAC is selected."]
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, AESCTL_SPEC, bool, O>;
#[doc = "Field `KEY_SIZE` reader - 4:3\\]
This read-only field specifies the key size. The key size is automatically configured when a new key is loaded through the key store module. 00 = N/A - Reserved 01 = 128-bit 10 = 192-bit 11 = 256-bit"]
pub type KEY_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KEY_SIZE` writer - 4:3\\]
This read-only field specifies the key size. The key size is automatically configured when a new key is loaded through the key store module. 00 = N/A - Reserved 01 = 128-bit 10 = 192-bit 11 = 256-bit"]
pub type KEY_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AESCTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `CBC` reader - 5:5\\]
If set to 1, cipher-block-chaining (CBC) mode is selected."]
pub type CBC_R = crate::BitReader<bool>;
#[doc = "Field `CBC` writer - 5:5\\]
If set to 1, cipher-block-chaining (CBC) mode is selected."]
pub type CBC_W<'a, const O: u8> = crate::BitWriter<'a, u32, AESCTL_SPEC, bool, O>;
#[doc = "Field `CTR` reader - 6:6\\]
If set to 1, AES counter mode (CTR) is selected. Note: This bit must also be set for GCM and CCM, when encryption/decryption is required."]
pub type CTR_R = crate::BitReader<bool>;
#[doc = "Field `CTR` writer - 6:6\\]
If set to 1, AES counter mode (CTR) is selected. Note: This bit must also be set for GCM and CCM, when encryption/decryption is required."]
pub type CTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, AESCTL_SPEC, bool, O>;
#[doc = "Field `CTR_WIDTH` reader - 8:7\\]
Specifies the counter width for AES-CTR mode 00 = 32-bit counter 01 = 64-bit counter 10 = 96-bit counter 11 = 128-bit counter"]
pub type CTR_WIDTH_R = crate::FieldReader<u8, CTR_WIDTH_A>;
#[doc = "8:7\\]
Specifies the counter width for AES-CTR mode 00 = 32-bit counter 01 = 64-bit counter 10 = 96-bit counter 11 = 128-bit counter\n\nValue on reset: 0"]
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
Specifies the counter width for AES-CTR mode 00 = 32-bit counter 01 = 64-bit counter 10 = 96-bit counter 11 = 128-bit counter"]
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
Set to 1 to select AES-CBC MAC mode. The direction bit must be set to 1 for this mode. Selecting this mode requires writing the length register after all other registers."]
pub type CBC_MAC_R = crate::BitReader<bool>;
#[doc = "Field `CBC_MAC` writer - 15:15\\]
Set to 1 to select AES-CBC MAC mode. The direction bit must be set to 1 for this mode. Selecting this mode requires writing the length register after all other registers."]
pub type CBC_MAC_W<'a, const O: u8> = crate::BitWriter<'a, u32, AESCTL_SPEC, bool, O>;
#[doc = "Field `GCM` reader - 17:16\\]
Set these bits to 11 to select AES-GCM mode. AES-GCM is a combined mode, using the Galois field multiplier GF(2 to the power of 128) for authentication and AES-CTR mode for encryption. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR Bit combination description: 00 = No GCM mode 01 = Reserved, do not select 10 = Reserved, do not select 11 = Autonomous GHASH (both H- and Y0-encrypted calculated internally) Note: The EIP-120t-1 configuration only supports mode 11 (autonomous GHASH), other GCM modes are not allowed."]
pub type GCM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GCM` writer - 17:16\\]
Set these bits to 11 to select AES-GCM mode. AES-GCM is a combined mode, using the Galois field multiplier GF(2 to the power of 128) for authentication and AES-CTR mode for encryption. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR Bit combination description: 00 = No GCM mode 01 = Reserved, do not select 10 = Reserved, do not select 11 = Autonomous GHASH (both H- and Y0-encrypted calculated internally) Note: The EIP-120t-1 configuration only supports mode 11 (autonomous GHASH), other GCM modes are not allowed."]
pub type GCM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AESCTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `CCM` reader - 18:18\\]
If set to 1, AES-CCM is selected AES-CCM is a combined mode, using AES for authentication and encryption. Note: Selecting AES-CCM mode requires writing of the AAD length register after all other registers. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR; selecting other AES modes than CTR mode is invalid."]
pub type CCM_R = crate::BitReader<bool>;
#[doc = "Field `CCM` writer - 18:18\\]
If set to 1, AES-CCM is selected AES-CCM is a combined mode, using AES for authentication and encryption. Note: Selecting AES-CCM mode requires writing of the AAD length register after all other registers. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR; selecting other AES modes than CTR mode is invalid."]
pub type CCM_W<'a, const O: u8> = crate::BitWriter<'a, u32, AESCTL_SPEC, bool, O>;
#[doc = "Field `CCM_L` reader - 21:19\\]
Defines L, which indicates the width of the length field for CCM operations; the length field in bytes equals the value of CMM-L plus one. All values are supported."]
pub type CCM_L_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CCM_L` writer - 21:19\\]
Defines L, which indicates the width of the length field for CCM operations; the length field in bytes equals the value of CMM-L plus one. All values are supported."]
pub type CCM_L_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AESCTL_SPEC, u8, u8, 3, O>;
#[doc = "Field `CCM_M` reader - 24:22\\]
Defines M, which indicates the length of the authentication field for CCM operations; the authentication field length equals two times (the value of CCM-M plus one). Note: The EIP-120t always returns a 128-bit authentication field, of which the M least significant bytes are valid. All values are supported."]
pub type CCM_M_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CCM_M` writer - 24:22\\]
Defines M, which indicates the length of the authentication field for CCM operations; the authentication field length equals two times (the value of CCM-M plus one). Note: The EIP-120t always returns a 128-bit authentication field, of which the M least significant bytes are valid. All values are supported."]
pub type CCM_M_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AESCTL_SPEC, u8, u8, 3, O>;
#[doc = "Field `XCBC_MAC` reader - 25:25\\]
Set to ‘1’ to select AES-XCBC MAC mode. The direction bit must be set to ‘1’ for this mode. Selecting this mode requires writing the length register"]
pub type XCBC_MAC_R = crate::BitReader<bool>;
#[doc = "Field `XCBC_MAC` writer - 25:25\\]
Set to ‘1’ to select AES-XCBC MAC mode. The direction bit must be set to ‘1’ for this mode. Selecting this mode requires writing the length register"]
pub type XCBC_MAC_W<'a, const O: u8> = crate::BitWriter<'a, u32, AESCTL_SPEC, bool, O>;
#[doc = "Field `GCM_CCM_CONTINUE_AAD` reader - 26:26\\]
Continue processing of an interrupted AES-GCM or AES-CCM operation in the AAD phase. Set this write-only signal to ‘1b’ together with the regular mode bit settings for a GCM or CCM operation, to continue processing from the next full AAD block (128 bits) boundary. Before setting this bit all applicable context to resume processing must have been loaded into the engine: Keys, IV, intermediate digest/TAG, block counter and the CCM align data word (the latter is for CCM mode only). The mode can be written together with this bit, as it is part of the same register."]
pub type GCM_CCM_CONTINUE_AAD_R = crate::BitReader<bool>;
#[doc = "Field `GCM_CCM_CONTINUE_AAD` writer - 26:26\\]
Continue processing of an interrupted AES-GCM or AES-CCM operation in the AAD phase. Set this write-only signal to ‘1b’ together with the regular mode bit settings for a GCM or CCM operation, to continue processing from the next full AAD block (128 bits) boundary. Before setting this bit all applicable context to resume processing must have been loaded into the engine: Keys, IV, intermediate digest/TAG, block counter and the CCM align data word (the latter is for CCM mode only). The mode can be written together with this bit, as it is part of the same register."]
pub type GCM_CCM_CONTINUE_AAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, AESCTL_SPEC, bool, O>;
#[doc = "Field `GET_DIGEST` reader - 27:27\\]
Interrupt processing and generate an intermediate digest during an AES-GCM or AES-CCM operation. Set this write-only signal to ‘1b’ to interrupt GCM or CCM processing at the next full block (128 bits) boundary. An intermediate digest may be requested during the encryption/decryption data phase or in the AAD phase. Interruption can only be done on full block (128 bits) boundaries."]
pub type GET_DIGEST_R = crate::BitReader<bool>;
#[doc = "Field `GET_DIGEST` writer - 27:27\\]
Interrupt processing and generate an intermediate digest during an AES-GCM or AES-CCM operation. Set this write-only signal to ‘1b’ to interrupt GCM or CCM processing at the next full block (128 bits) boundary. An intermediate digest may be requested during the encryption/decryption data phase or in the AAD phase. Interruption can only be done on full block (128 bits) boundaries."]
pub type GET_DIGEST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AESCTL_SPEC, bool, O>;
#[doc = "Field `GCM_CCM_CONTINUE` reader - 28:28\\]
Continue processing of an interrupted AES-GCM or AES-CCM operation in the crypto/payload phase. Set this write-only signal to ‘1b’ together with the regular mode bit settings for a GCM or CCM operation, to continue processing from the next full block (128 bits) boundary. Before setting this bit all applicable context to resume processing must have been loaded into the engine: Keys, IV, intermediate digest/TAG and block counter. The mode can be written together with this bit, as it is part of the same register."]
pub type GCM_CCM_CONTINUE_R = crate::BitReader<bool>;
#[doc = "Field `GCM_CCM_CONTINUE` writer - 28:28\\]
Continue processing of an interrupted AES-GCM or AES-CCM operation in the crypto/payload phase. Set this write-only signal to ‘1b’ together with the regular mode bit settings for a GCM or CCM operation, to continue processing from the next full block (128 bits) boundary. Before setting this bit all applicable context to resume processing must have been loaded into the engine: Keys, IV, intermediate digest/TAG and block counter. The mode can be written together with this bit, as it is part of the same register."]
pub type GCM_CCM_CONTINUE_W<'a, const O: u8> = crate::BitWriter<'a, u32, AESCTL_SPEC, bool, O>;
#[doc = "Field `SAVE_CONTEXT` reader - 29:29\\]
This bit indicates that an authentication TAG or result IV needs to be stored as a result context. Typically this bit must be set for authentication modes returning a TAG (CBC-MAC, GCM and CCM), or for basic encryption modes that require future continuation with the current result IV. If this bit is set, the engine retains its full context until the TAG and/or IV registers are read. The TAG or IV must be read before the AES engine can start a new operation."]
pub type SAVE_CONTEXT_R = crate::BitReader<bool>;
#[doc = "Field `SAVE_CONTEXT` writer - 29:29\\]
This bit indicates that an authentication TAG or result IV needs to be stored as a result context. Typically this bit must be set for authentication modes returning a TAG (CBC-MAC, GCM and CCM), or for basic encryption modes that require future continuation with the current result IV. If this bit is set, the engine retains its full context until the TAG and/or IV registers are read. The TAG or IV must be read before the AES engine can start a new operation."]
pub type SAVE_CONTEXT_W<'a, const O: u8> = crate::BitWriter<'a, u32, AESCTL_SPEC, bool, O>;
#[doc = "Field `SAVED_CONTEXT_RDY` reader - 30:30\\]
If 1, this status bit indicates that an AES authentication TAG and/or IV block(s) is/are available for the host to retrieve. This bit is only asserted if the save_context bit is set to 1. The bit is mutual exclusive with the context_ready bit. Writing one clears the bit to 0, indicating the AES core can start its next operation. This bit is also cleared when the 4th word of the output TAG and/or IV is read. Note: All other mode bit writes are ignored when this mode bit is written with 1. Note: This bit is controlled automatically by the EIP-120t for TAG read DMA operations."]
pub type SAVED_CONTEXT_RDY_R = crate::BitReader<bool>;
#[doc = "Field `SAVED_CONTEXT_RDY` writer - 30:30\\]
If 1, this status bit indicates that an AES authentication TAG and/or IV block(s) is/are available for the host to retrieve. This bit is only asserted if the save_context bit is set to 1. The bit is mutual exclusive with the context_ready bit. Writing one clears the bit to 0, indicating the AES core can start its next operation. This bit is also cleared when the 4th word of the output TAG and/or IV is read. Note: All other mode bit writes are ignored when this mode bit is written with 1. Note: This bit is controlled automatically by the EIP-120t for TAG read DMA operations."]
pub type SAVED_CONTEXT_RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, AESCTL_SPEC, bool, O>;
#[doc = "Field `CONTEXT_READY` reader - 31:31\\]
If 1, this read-only status bit indicates that the context data registers can be overwritten and the host is permitted to write the next context."]
pub type CONTEXT_READY_R = crate::BitReader<bool>;
#[doc = "Field `CONTEXT_READY` writer - 31:31\\]
If 1, this read-only status bit indicates that the context data registers can be overwritten and the host is permitted to write the next context."]
pub type CONTEXT_READY_W<'a, const O: u8> = crate::BitWriter<'a, u32, AESCTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
If 1, this status bit indicates that an AES output block is available to be retrieved by the host. Writing 0 clears the bit to 0 and indicates that output data is read by the host. The AES core can provide a next output data block. Writing 1 to this bit is ignored. Note: For DMA operations, this bit is automatically controlled by the EIP-120t."]
    #[inline(always)]
    pub fn output_ready(&self) -> OUTPUT_READY_R {
        OUTPUT_READY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
If 1, this status bit indicates that the 16-byte AES input buffer is empty. The host is permitted to write the next block of data. Writing 0 clears the bit to 0 and indicates that the AES core can use the provided input data block. Writing 1 to this bit is ignored. Note: For DMA operations, this bit is automatically controlled by the EIP-120t. After reset, this bit is 0. After writing a context, this bit becomes 1."]
    #[inline(always)]
    pub fn input_ready(&self) -> INPUT_READY_R {
        INPUT_READY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
If set to 1 an encrypt operation is performed. If set to 0 a decrypt operation is performed. This bit must be written with a 1 when CBC-MAC is selected."]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - 4:3\\]
This read-only field specifies the key size. The key size is automatically configured when a new key is loaded through the key store module. 00 = N/A - Reserved 01 = 128-bit 10 = 192-bit 11 = 256-bit"]
    #[inline(always)]
    pub fn key_size(&self) -> KEY_SIZE_R {
        KEY_SIZE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
If set to 1, cipher-block-chaining (CBC) mode is selected."]
    #[inline(always)]
    pub fn cbc(&self) -> CBC_R {
        CBC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
If set to 1, AES counter mode (CTR) is selected. Note: This bit must also be set for GCM and CCM, when encryption/decryption is required."]
    #[inline(always)]
    pub fn ctr(&self) -> CTR_R {
        CTR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - 8:7\\]
Specifies the counter width for AES-CTR mode 00 = 32-bit counter 01 = 64-bit counter 10 = 96-bit counter 11 = 128-bit counter"]
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
Set to 1 to select AES-CBC MAC mode. The direction bit must be set to 1 for this mode. Selecting this mode requires writing the length register after all other registers."]
    #[inline(always)]
    pub fn cbc_mac(&self) -> CBC_MAC_R {
        CBC_MAC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Set these bits to 11 to select AES-GCM mode. AES-GCM is a combined mode, using the Galois field multiplier GF(2 to the power of 128) for authentication and AES-CTR mode for encryption. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR Bit combination description: 00 = No GCM mode 01 = Reserved, do not select 10 = Reserved, do not select 11 = Autonomous GHASH (both H- and Y0-encrypted calculated internally) Note: The EIP-120t-1 configuration only supports mode 11 (autonomous GHASH), other GCM modes are not allowed."]
    #[inline(always)]
    pub fn gcm(&self) -> GCM_R {
        GCM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - 18:18\\]
If set to 1, AES-CCM is selected AES-CCM is a combined mode, using AES for authentication and encryption. Note: Selecting AES-CCM mode requires writing of the AAD length register after all other registers. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR; selecting other AES modes than CTR mode is invalid."]
    #[inline(always)]
    pub fn ccm(&self) -> CCM_R {
        CCM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:21 - 21:19\\]
Defines L, which indicates the width of the length field for CCM operations; the length field in bytes equals the value of CMM-L plus one. All values are supported."]
    #[inline(always)]
    pub fn ccm_l(&self) -> CCM_L_R {
        CCM_L_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:24 - 24:22\\]
Defines M, which indicates the length of the authentication field for CCM operations; the authentication field length equals two times (the value of CCM-M plus one). Note: The EIP-120t always returns a 128-bit authentication field, of which the M least significant bytes are valid. All values are supported."]
    #[inline(always)]
    pub fn ccm_m(&self) -> CCM_M_R {
        CCM_M_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bit 25 - 25:25\\]
Set to ‘1’ to select AES-XCBC MAC mode. The direction bit must be set to ‘1’ for this mode. Selecting this mode requires writing the length register"]
    #[inline(always)]
    pub fn xcbc_mac(&self) -> XCBC_MAC_R {
        XCBC_MAC_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Continue processing of an interrupted AES-GCM or AES-CCM operation in the AAD phase. Set this write-only signal to ‘1b’ together with the regular mode bit settings for a GCM or CCM operation, to continue processing from the next full AAD block (128 bits) boundary. Before setting this bit all applicable context to resume processing must have been loaded into the engine: Keys, IV, intermediate digest/TAG, block counter and the CCM align data word (the latter is for CCM mode only). The mode can be written together with this bit, as it is part of the same register."]
    #[inline(always)]
    pub fn gcm_ccm_continue_aad(&self) -> GCM_CCM_CONTINUE_AAD_R {
        GCM_CCM_CONTINUE_AAD_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Interrupt processing and generate an intermediate digest during an AES-GCM or AES-CCM operation. Set this write-only signal to ‘1b’ to interrupt GCM or CCM processing at the next full block (128 bits) boundary. An intermediate digest may be requested during the encryption/decryption data phase or in the AAD phase. Interruption can only be done on full block (128 bits) boundaries."]
    #[inline(always)]
    pub fn get_digest(&self) -> GET_DIGEST_R {
        GET_DIGEST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Continue processing of an interrupted AES-GCM or AES-CCM operation in the crypto/payload phase. Set this write-only signal to ‘1b’ together with the regular mode bit settings for a GCM or CCM operation, to continue processing from the next full block (128 bits) boundary. Before setting this bit all applicable context to resume processing must have been loaded into the engine: Keys, IV, intermediate digest/TAG and block counter. The mode can be written together with this bit, as it is part of the same register."]
    #[inline(always)]
    pub fn gcm_ccm_continue(&self) -> GCM_CCM_CONTINUE_R {
        GCM_CCM_CONTINUE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
This bit indicates that an authentication TAG or result IV needs to be stored as a result context. Typically this bit must be set for authentication modes returning a TAG (CBC-MAC, GCM and CCM), or for basic encryption modes that require future continuation with the current result IV. If this bit is set, the engine retains its full context until the TAG and/or IV registers are read. The TAG or IV must be read before the AES engine can start a new operation."]
    #[inline(always)]
    pub fn save_context(&self) -> SAVE_CONTEXT_R {
        SAVE_CONTEXT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
If 1, this status bit indicates that an AES authentication TAG and/or IV block(s) is/are available for the host to retrieve. This bit is only asserted if the save_context bit is set to 1. The bit is mutual exclusive with the context_ready bit. Writing one clears the bit to 0, indicating the AES core can start its next operation. This bit is also cleared when the 4th word of the output TAG and/or IV is read. Note: All other mode bit writes are ignored when this mode bit is written with 1. Note: This bit is controlled automatically by the EIP-120t for TAG read DMA operations."]
    #[inline(always)]
    pub fn saved_context_rdy(&self) -> SAVED_CONTEXT_RDY_R {
        SAVED_CONTEXT_RDY_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
If 1, this read-only status bit indicates that the context data registers can be overwritten and the host is permitted to write the next context."]
    #[inline(always)]
    pub fn context_ready(&self) -> CONTEXT_READY_R {
        CONTEXT_READY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
If 1, this status bit indicates that an AES output block is available to be retrieved by the host. Writing 0 clears the bit to 0 and indicates that output data is read by the host. The AES core can provide a next output data block. Writing 1 to this bit is ignored. Note: For DMA operations, this bit is automatically controlled by the EIP-120t."]
    #[inline(always)]
    #[must_use]
    pub fn output_ready(&mut self) -> OUTPUT_READY_W<0> {
        OUTPUT_READY_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
If 1, this status bit indicates that the 16-byte AES input buffer is empty. The host is permitted to write the next block of data. Writing 0 clears the bit to 0 and indicates that the AES core can use the provided input data block. Writing 1 to this bit is ignored. Note: For DMA operations, this bit is automatically controlled by the EIP-120t. After reset, this bit is 0. After writing a context, this bit becomes 1."]
    #[inline(always)]
    #[must_use]
    pub fn input_ready(&mut self) -> INPUT_READY_W<1> {
        INPUT_READY_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
If set to 1 an encrypt operation is performed. If set to 0 a decrypt operation is performed. This bit must be written with a 1 when CBC-MAC is selected."]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<2> {
        DIR_W::new(self)
    }
    #[doc = "Bits 3:4 - 4:3\\]
This read-only field specifies the key size. The key size is automatically configured when a new key is loaded through the key store module. 00 = N/A - Reserved 01 = 128-bit 10 = 192-bit 11 = 256-bit"]
    #[inline(always)]
    #[must_use]
    pub fn key_size(&mut self) -> KEY_SIZE_W<3> {
        KEY_SIZE_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
If set to 1, cipher-block-chaining (CBC) mode is selected."]
    #[inline(always)]
    #[must_use]
    pub fn cbc(&mut self) -> CBC_W<5> {
        CBC_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
If set to 1, AES counter mode (CTR) is selected. Note: This bit must also be set for GCM and CCM, when encryption/decryption is required."]
    #[inline(always)]
    #[must_use]
    pub fn ctr(&mut self) -> CTR_W<6> {
        CTR_W::new(self)
    }
    #[doc = "Bits 7:8 - 8:7\\]
Specifies the counter width for AES-CTR mode 00 = 32-bit counter 01 = 64-bit counter 10 = 96-bit counter 11 = 128-bit counter"]
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
Set to 1 to select AES-CBC MAC mode. The direction bit must be set to 1 for this mode. Selecting this mode requires writing the length register after all other registers."]
    #[inline(always)]
    #[must_use]
    pub fn cbc_mac(&mut self) -> CBC_MAC_W<15> {
        CBC_MAC_W::new(self)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Set these bits to 11 to select AES-GCM mode. AES-GCM is a combined mode, using the Galois field multiplier GF(2 to the power of 128) for authentication and AES-CTR mode for encryption. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR Bit combination description: 00 = No GCM mode 01 = Reserved, do not select 10 = Reserved, do not select 11 = Autonomous GHASH (both H- and Y0-encrypted calculated internally) Note: The EIP-120t-1 configuration only supports mode 11 (autonomous GHASH), other GCM modes are not allowed."]
    #[inline(always)]
    #[must_use]
    pub fn gcm(&mut self) -> GCM_W<16> {
        GCM_W::new(self)
    }
    #[doc = "Bit 18 - 18:18\\]
If set to 1, AES-CCM is selected AES-CCM is a combined mode, using AES for authentication and encryption. Note: Selecting AES-CCM mode requires writing of the AAD length register after all other registers. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR; selecting other AES modes than CTR mode is invalid."]
    #[inline(always)]
    #[must_use]
    pub fn ccm(&mut self) -> CCM_W<18> {
        CCM_W::new(self)
    }
    #[doc = "Bits 19:21 - 21:19\\]
Defines L, which indicates the width of the length field for CCM operations; the length field in bytes equals the value of CMM-L plus one. All values are supported."]
    #[inline(always)]
    #[must_use]
    pub fn ccm_l(&mut self) -> CCM_L_W<19> {
        CCM_L_W::new(self)
    }
    #[doc = "Bits 22:24 - 24:22\\]
Defines M, which indicates the length of the authentication field for CCM operations; the authentication field length equals two times (the value of CCM-M plus one). Note: The EIP-120t always returns a 128-bit authentication field, of which the M least significant bytes are valid. All values are supported."]
    #[inline(always)]
    #[must_use]
    pub fn ccm_m(&mut self) -> CCM_M_W<22> {
        CCM_M_W::new(self)
    }
    #[doc = "Bit 25 - 25:25\\]
Set to ‘1’ to select AES-XCBC MAC mode. The direction bit must be set to ‘1’ for this mode. Selecting this mode requires writing the length register"]
    #[inline(always)]
    #[must_use]
    pub fn xcbc_mac(&mut self) -> XCBC_MAC_W<25> {
        XCBC_MAC_W::new(self)
    }
    #[doc = "Bit 26 - 26:26\\]
Continue processing of an interrupted AES-GCM or AES-CCM operation in the AAD phase. Set this write-only signal to ‘1b’ together with the regular mode bit settings for a GCM or CCM operation, to continue processing from the next full AAD block (128 bits) boundary. Before setting this bit all applicable context to resume processing must have been loaded into the engine: Keys, IV, intermediate digest/TAG, block counter and the CCM align data word (the latter is for CCM mode only). The mode can be written together with this bit, as it is part of the same register."]
    #[inline(always)]
    #[must_use]
    pub fn gcm_ccm_continue_aad(&mut self) -> GCM_CCM_CONTINUE_AAD_W<26> {
        GCM_CCM_CONTINUE_AAD_W::new(self)
    }
    #[doc = "Bit 27 - 27:27\\]
Interrupt processing and generate an intermediate digest during an AES-GCM or AES-CCM operation. Set this write-only signal to ‘1b’ to interrupt GCM or CCM processing at the next full block (128 bits) boundary. An intermediate digest may be requested during the encryption/decryption data phase or in the AAD phase. Interruption can only be done on full block (128 bits) boundaries."]
    #[inline(always)]
    #[must_use]
    pub fn get_digest(&mut self) -> GET_DIGEST_W<27> {
        GET_DIGEST_W::new(self)
    }
    #[doc = "Bit 28 - 28:28\\]
Continue processing of an interrupted AES-GCM or AES-CCM operation in the crypto/payload phase. Set this write-only signal to ‘1b’ together with the regular mode bit settings for a GCM or CCM operation, to continue processing from the next full block (128 bits) boundary. Before setting this bit all applicable context to resume processing must have been loaded into the engine: Keys, IV, intermediate digest/TAG and block counter. The mode can be written together with this bit, as it is part of the same register."]
    #[inline(always)]
    #[must_use]
    pub fn gcm_ccm_continue(&mut self) -> GCM_CCM_CONTINUE_W<28> {
        GCM_CCM_CONTINUE_W::new(self)
    }
    #[doc = "Bit 29 - 29:29\\]
This bit indicates that an authentication TAG or result IV needs to be stored as a result context. Typically this bit must be set for authentication modes returning a TAG (CBC-MAC, GCM and CCM), or for basic encryption modes that require future continuation with the current result IV. If this bit is set, the engine retains its full context until the TAG and/or IV registers are read. The TAG or IV must be read before the AES engine can start a new operation."]
    #[inline(always)]
    #[must_use]
    pub fn save_context(&mut self) -> SAVE_CONTEXT_W<29> {
        SAVE_CONTEXT_W::new(self)
    }
    #[doc = "Bit 30 - 30:30\\]
If 1, this status bit indicates that an AES authentication TAG and/or IV block(s) is/are available for the host to retrieve. This bit is only asserted if the save_context bit is set to 1. The bit is mutual exclusive with the context_ready bit. Writing one clears the bit to 0, indicating the AES core can start its next operation. This bit is also cleared when the 4th word of the output TAG and/or IV is read. Note: All other mode bit writes are ignored when this mode bit is written with 1. Note: This bit is controlled automatically by the EIP-120t for TAG read DMA operations."]
    #[inline(always)]
    #[must_use]
    pub fn saved_context_rdy(&mut self) -> SAVED_CONTEXT_RDY_W<30> {
        SAVED_CONTEXT_RDY_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
If 1, this read-only status bit indicates that the context data registers can be overwritten and the host is permitted to write the next context."]
    #[inline(always)]
    #[must_use]
    pub fn context_ready(&mut self) -> CONTEXT_READY_W<31> {
        CONTEXT_READY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES Control AES input/output buffer control and mode register This register specifies the AES mode of operation for the EIP-120t. Electronic codebook (ECB) mode is automatically selected if bits \\[28:5\\]
of this register are all 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesctl](index.html) module"]
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
