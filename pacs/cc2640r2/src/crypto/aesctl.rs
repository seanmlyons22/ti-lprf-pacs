#[doc = "Register `AESCTL` reader"]
pub type R = crate::R<AesctlSpec>;
#[doc = "Register `AESCTL` writer"]
pub type W = crate::W<AesctlSpec>;
#[doc = "Field `OUTPUT_RDY` reader - 0:0\\]
If read as 1, this status bit indicates that an AES output block is available to be retrieved by the Host. Writing a 0 clears the bit to zero and indicates that output data is read by the Host. The AES engine can provide a next output data block. Writing a 1 to this bit will be ignored. Note: For DMA operations, this bit is automatically controlled by the Crypto peripheral. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
pub type OutputRdyR = crate::BitReader;
#[doc = "Field `OUTPUT_RDY` writer - 0:0\\]
If read as 1, this status bit indicates that an AES output block is available to be retrieved by the Host. Writing a 0 clears the bit to zero and indicates that output data is read by the Host. The AES engine can provide a next output data block. Writing a 1 to this bit will be ignored. Note: For DMA operations, this bit is automatically controlled by the Crypto peripheral. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
pub type OutputRdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INPUT_RDY` reader - 1:1\\]
If read as 1, this status bit indicates that the 16-byte AES input buffer is empty. The Host is permitted to write the next block of data. Writing a 0 clears the bit to zero and indicates that the AES engine can use the provided input data block. Writing a 1 to this bit will be ignored. Note: For DMA operations, this bit is automatically controlled by the Crypto peripheral. After reset, this bit is 0. After writing a context (note 1), this bit will become 1. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
pub type InputRdyR = crate::BitReader;
#[doc = "Field `INPUT_RDY` writer - 1:1\\]
If read as 1, this status bit indicates that the 16-byte AES input buffer is empty. The Host is permitted to write the next block of data. Writing a 0 clears the bit to zero and indicates that the AES engine can use the provided input data block. Writing a 1 to this bit will be ignored. Note: For DMA operations, this bit is automatically controlled by the Crypto peripheral. After reset, this bit is 0. After writing a context (note 1), this bit will become 1. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
pub type InputRdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR` reader - 2:2\\]
Direction. 0 : Decrypt operation is performed. 1 : Encrypt operation is performed. This bit must be written with a 1 when CBC-MAC is selected."]
pub type DirR = crate::BitReader;
#[doc = "Field `DIR` writer - 2:2\\]
Direction. 0 : Decrypt operation is performed. 1 : Encrypt operation is performed. This bit must be written with a 1 when CBC-MAC is selected."]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY_SIZE` reader - 4:3\\]
This field specifies the key size. The key size is automatically configured when a new key is loaded via the key store module. 00 = N/A - reserved 01 = 128 bits 10 = N/A - reserved 11 = N/A - reserved For the Crypto peripheral this field is fixed to 128 bits."]
pub type KeySizeR = crate::FieldReader;
#[doc = "Field `CBC` reader - 5:5\\]
CBC mode enable"]
pub type CbcR = crate::BitReader;
#[doc = "Field `CBC` writer - 5:5\\]
CBC mode enable"]
pub type CbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTR` reader - 6:6\\]
AES-CTR mode enable This bit must also be set for CCM, when encryption/decryption is required."]
pub type CtrR = crate::BitReader;
#[doc = "Field `CTR` writer - 6:6\\]
AES-CTR mode enable This bit must also be set for CCM, when encryption/decryption is required."]
pub type CtrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "8:7\\]
Specifies the counter width for AES-CTR mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CtrWidth {
    #[doc = "3: 128 bits"]
    _128Bit = 3,
    #[doc = "2: 96 bits"]
    _96Bit = 2,
    #[doc = "1: 64 bits"]
    _64Bit = 1,
    #[doc = "0: 32 bits"]
    _32Bit = 0,
}
impl From<CtrWidth> for u8 {
    #[inline(always)]
    fn from(variant: CtrWidth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CtrWidth {
    type Ux = u8;
}
impl crate::IsEnum for CtrWidth {}
#[doc = "Field `CTR_WIDTH` reader - 8:7\\]
Specifies the counter width for AES-CTR mode"]
pub type CtrWidthR = crate::FieldReader<CtrWidth>;
impl CtrWidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CtrWidth {
        match self.bits {
            3 => CtrWidth::_128Bit,
            2 => CtrWidth::_96Bit,
            1 => CtrWidth::_64Bit,
            0 => CtrWidth::_32Bit,
            _ => unreachable!(),
        }
    }
    #[doc = "128 bits"]
    #[inline(always)]
    pub fn is_128_bit(&self) -> bool {
        *self == CtrWidth::_128Bit
    }
    #[doc = "96 bits"]
    #[inline(always)]
    pub fn is_96_bit(&self) -> bool {
        *self == CtrWidth::_96Bit
    }
    #[doc = "64 bits"]
    #[inline(always)]
    pub fn is_64_bit(&self) -> bool {
        *self == CtrWidth::_64Bit
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn is_32_bit(&self) -> bool {
        *self == CtrWidth::_32Bit
    }
}
#[doc = "Field `CTR_WIDTH` writer - 8:7\\]
Specifies the counter width for AES-CTR mode"]
pub type CtrWidthW<'a, REG> = crate::FieldWriter<'a, REG, 2, CtrWidth, crate::Safe>;
impl<'a, REG> CtrWidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "128 bits"]
    #[inline(always)]
    pub fn _128_bit(self) -> &'a mut crate::W<REG> {
        self.variant(CtrWidth::_128Bit)
    }
    #[doc = "96 bits"]
    #[inline(always)]
    pub fn _96_bit(self) -> &'a mut crate::W<REG> {
        self.variant(CtrWidth::_96Bit)
    }
    #[doc = "64 bits"]
    #[inline(always)]
    pub fn _64_bit(self) -> &'a mut crate::W<REG> {
        self.variant(CtrWidth::_64Bit)
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn _32_bit(self) -> &'a mut crate::W<REG> {
        self.variant(CtrWidth::_32Bit)
    }
}
#[doc = "Field `RESERVED9` reader - 14:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader;
#[doc = "Field `RESERVED9` writer - 14:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CBC_MAC` reader - 15:15\\]
MAC mode enable. The DIR bit must be set to 1 for this mode. Selecting this mode requires writing the AESDATALEN1.LEN_MSW and AESDATALEN0.LEN_LSW registers after all other registers."]
pub type CbcMacR = crate::BitReader;
#[doc = "Field `CBC_MAC` writer - 15:15\\]
MAC mode enable. The DIR bit must be set to 1 for this mode. Selecting this mode requires writing the AESDATALEN1.LEN_MSW and AESDATALEN0.LEN_LSW registers after all other registers."]
pub type CbcMacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCM` reader - 18:18\\]
AES-CCM mode enable. AES-CCM is a combined mode, using AES for both authentication and encryption. Note: Selecting AES-CCM mode requires writing of AESDATALEN1.LEN_MSW and AESDATALEN0.LEN_LSW after all other registers. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR; selecting other AES modes than CTR mode is invalid."]
pub type CcmR = crate::BitReader;
#[doc = "Field `CCM` writer - 18:18\\]
AES-CCM mode enable. AES-CCM is a combined mode, using AES for both authentication and encryption. Note: Selecting AES-CCM mode requires writing of AESDATALEN1.LEN_MSW and AESDATALEN0.LEN_LSW after all other registers. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR; selecting other AES modes than CTR mode is invalid."]
pub type CcmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCM_L` reader - 21:19\\]
Defines L that indicates the width of the length field for CCM operations; the length field in bytes equals the value of CMM_L plus one. All values are supported."]
pub type CcmLR = crate::FieldReader;
#[doc = "Field `CCM_L` writer - 21:19\\]
Defines L that indicates the width of the length field for CCM operations; the length field in bytes equals the value of CMM_L plus one. All values are supported."]
pub type CcmLW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CCM_M` reader - 24:22\\]
Defines M that indicates the length of the authentication field for CCM operations; the authentication field length equals two times the value of CCM_M plus one. Note: The Crypto peripheral always returns a 128-bit authentication field, of which the M least significant bytes are valid. All values are supported."]
pub type CcmMR = crate::FieldReader;
#[doc = "Field `CCM_M` writer - 24:22\\]
Defines M that indicates the length of the authentication field for CCM operations; the authentication field length equals two times the value of CCM_M plus one. Note: The Crypto peripheral always returns a 128-bit authentication field, of which the M least significant bytes are valid. All values are supported."]
pub type CcmMW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESERVED25` reader - 28:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved25R = crate::FieldReader;
#[doc = "Field `RESERVED25` writer - 28:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved25W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SAVE_CONTEXT` reader - 29:29\\]
IV must be read before the AES engine can start a new operation."]
pub type SaveContextR = crate::BitReader;
#[doc = "Field `SAVE_CONTEXT` writer - 29:29\\]
IV must be read before the AES engine can start a new operation."]
pub type SaveContextW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAVED_CONTEXT_RDY` reader - 30:30\\]
If read as 1, this status bit indicates that an AES authentication TAG and/or IV block(s) is/are available for the Host to retrieve. This bit is only asserted if SAVE_CONTEXT is set to 1. The bit is mutually exclusive with CONTEXT_RDY. Writing 1 clears the bit to zero, indicating the Crypto peripheral can start its next operation. This bit is also cleared when the 4th word of the output TAG and/or IV is read. Note: All other mode bit writes will be ignored when this mode bit is written with 1. Note: This bit is controlled automatically by the Crypto peripheral for TAG read DMA operations. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
pub type SavedContextRdyR = crate::BitReader;
#[doc = "Field `SAVED_CONTEXT_RDY` writer - 30:30\\]
If read as 1, this status bit indicates that an AES authentication TAG and/or IV block(s) is/are available for the Host to retrieve. This bit is only asserted if SAVE_CONTEXT is set to 1. The bit is mutually exclusive with CONTEXT_RDY. Writing 1 clears the bit to zero, indicating the Crypto peripheral can start its next operation. This bit is also cleared when the 4th word of the output TAG and/or IV is read. Note: All other mode bit writes will be ignored when this mode bit is written with 1. Note: This bit is controlled automatically by the Crypto peripheral for TAG read DMA operations. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
pub type SavedContextRdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONTEXT_RDY` reader - 31:31\\]
If 1, this status bit indicates that the context data registers can be overwritten and the Host is permitted to write the next context. Writing a context means writing either a mode, the crypto length or AESDATALEN1.LEN_MSW, AESDATALEN0.LEN_LSW length registers"]
pub type ContextRdyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 0:0\\]
If read as 1, this status bit indicates that an AES output block is available to be retrieved by the Host. Writing a 0 clears the bit to zero and indicates that output data is read by the Host. The AES engine can provide a next output data block. Writing a 1 to this bit will be ignored. Note: For DMA operations, this bit is automatically controlled by the Crypto peripheral. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
    #[inline(always)]
    pub fn output_rdy(&self) -> OutputRdyR {
        OutputRdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
If read as 1, this status bit indicates that the 16-byte AES input buffer is empty. The Host is permitted to write the next block of data. Writing a 0 clears the bit to zero and indicates that the AES engine can use the provided input data block. Writing a 1 to this bit will be ignored. Note: For DMA operations, this bit is automatically controlled by the Crypto peripheral. After reset, this bit is 0. After writing a context (note 1), this bit will become 1. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
    #[inline(always)]
    pub fn input_rdy(&self) -> InputRdyR {
        InputRdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Direction. 0 : Decrypt operation is performed. 1 : Encrypt operation is performed. This bit must be written with a 1 when CBC-MAC is selected."]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - 4:3\\]
This field specifies the key size. The key size is automatically configured when a new key is loaded via the key store module. 00 = N/A - reserved 01 = 128 bits 10 = N/A - reserved 11 = N/A - reserved For the Crypto peripheral this field is fixed to 128 bits."]
    #[inline(always)]
    pub fn key_size(&self) -> KeySizeR {
        KeySizeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
CBC mode enable"]
    #[inline(always)]
    pub fn cbc(&self) -> CbcR {
        CbcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
AES-CTR mode enable This bit must also be set for CCM, when encryption/decryption is required."]
    #[inline(always)]
    pub fn ctr(&self) -> CtrR {
        CtrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - 8:7\\]
Specifies the counter width for AES-CTR mode"]
    #[inline(always)]
    pub fn ctr_width(&self) -> CtrWidthR {
        CtrWidthR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:14 - 14:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 9) & 0x3f) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
MAC mode enable. The DIR bit must be set to 1 for this mode. Selecting this mode requires writing the AESDATALEN1.LEN_MSW and AESDATALEN0.LEN_LSW registers after all other registers."]
    #[inline(always)]
    pub fn cbc_mac(&self) -> CbcMacR {
        CbcMacR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
AES-CCM mode enable. AES-CCM is a combined mode, using AES for both authentication and encryption. Note: Selecting AES-CCM mode requires writing of AESDATALEN1.LEN_MSW and AESDATALEN0.LEN_LSW after all other registers. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR; selecting other AES modes than CTR mode is invalid."]
    #[inline(always)]
    pub fn ccm(&self) -> CcmR {
        CcmR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:21 - 21:19\\]
Defines L that indicates the width of the length field for CCM operations; the length field in bytes equals the value of CMM_L plus one. All values are supported."]
    #[inline(always)]
    pub fn ccm_l(&self) -> CcmLR {
        CcmLR::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:24 - 24:22\\]
Defines M that indicates the length of the authentication field for CCM operations; the authentication field length equals two times the value of CCM_M plus one. Note: The Crypto peripheral always returns a 128-bit authentication field, of which the M least significant bytes are valid. All values are supported."]
    #[inline(always)]
    pub fn ccm_m(&self) -> CcmMR {
        CcmMR::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:28 - 28:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved25(&self) -> Reserved25R {
        Reserved25R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - 29:29\\]
IV must be read before the AES engine can start a new operation."]
    #[inline(always)]
    pub fn save_context(&self) -> SaveContextR {
        SaveContextR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
If read as 1, this status bit indicates that an AES authentication TAG and/or IV block(s) is/are available for the Host to retrieve. This bit is only asserted if SAVE_CONTEXT is set to 1. The bit is mutually exclusive with CONTEXT_RDY. Writing 1 clears the bit to zero, indicating the Crypto peripheral can start its next operation. This bit is also cleared when the 4th word of the output TAG and/or IV is read. Note: All other mode bit writes will be ignored when this mode bit is written with 1. Note: This bit is controlled automatically by the Crypto peripheral for TAG read DMA operations. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
    #[inline(always)]
    pub fn saved_context_rdy(&self) -> SavedContextRdyR {
        SavedContextRdyR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
If 1, this status bit indicates that the context data registers can be overwritten and the Host is permitted to write the next context. Writing a context means writing either a mode, the crypto length or AESDATALEN1.LEN_MSW, AESDATALEN0.LEN_LSW length registers"]
    #[inline(always)]
    pub fn context_rdy(&self) -> ContextRdyR {
        ContextRdyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
If read as 1, this status bit indicates that an AES output block is available to be retrieved by the Host. Writing a 0 clears the bit to zero and indicates that output data is read by the Host. The AES engine can provide a next output data block. Writing a 1 to this bit will be ignored. Note: For DMA operations, this bit is automatically controlled by the Crypto peripheral. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
    #[inline(always)]
    #[must_use]
    pub fn output_rdy(&mut self) -> OutputRdyW<AesctlSpec> {
        OutputRdyW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
If read as 1, this status bit indicates that the 16-byte AES input buffer is empty. The Host is permitted to write the next block of data. Writing a 0 clears the bit to zero and indicates that the AES engine can use the provided input data block. Writing a 1 to this bit will be ignored. Note: For DMA operations, this bit is automatically controlled by the Crypto peripheral. After reset, this bit is 0. After writing a context (note 1), this bit will become 1. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
    #[inline(always)]
    #[must_use]
    pub fn input_rdy(&mut self) -> InputRdyW<AesctlSpec> {
        InputRdyW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Direction. 0 : Decrypt operation is performed. 1 : Encrypt operation is performed. This bit must be written with a 1 when CBC-MAC is selected."]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DirW<AesctlSpec> {
        DirW::new(self, 2)
    }
    #[doc = "Bit 5 - 5:5\\]
CBC mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn cbc(&mut self) -> CbcW<AesctlSpec> {
        CbcW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
AES-CTR mode enable This bit must also be set for CCM, when encryption/decryption is required."]
    #[inline(always)]
    #[must_use]
    pub fn ctr(&mut self) -> CtrW<AesctlSpec> {
        CtrW::new(self, 6)
    }
    #[doc = "Bits 7:8 - 8:7\\]
Specifies the counter width for AES-CTR mode"]
    #[inline(always)]
    #[must_use]
    pub fn ctr_width(&mut self) -> CtrWidthW<AesctlSpec> {
        CtrWidthW::new(self, 7)
    }
    #[doc = "Bits 9:14 - 14:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> Reserved9W<AesctlSpec> {
        Reserved9W::new(self, 9)
    }
    #[doc = "Bit 15 - 15:15\\]
MAC mode enable. The DIR bit must be set to 1 for this mode. Selecting this mode requires writing the AESDATALEN1.LEN_MSW and AESDATALEN0.LEN_LSW registers after all other registers."]
    #[inline(always)]
    #[must_use]
    pub fn cbc_mac(&mut self) -> CbcMacW<AesctlSpec> {
        CbcMacW::new(self, 15)
    }
    #[doc = "Bit 18 - 18:18\\]
AES-CCM mode enable. AES-CCM is a combined mode, using AES for both authentication and encryption. Note: Selecting AES-CCM mode requires writing of AESDATALEN1.LEN_MSW and AESDATALEN0.LEN_LSW after all other registers. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR; selecting other AES modes than CTR mode is invalid."]
    #[inline(always)]
    #[must_use]
    pub fn ccm(&mut self) -> CcmW<AesctlSpec> {
        CcmW::new(self, 18)
    }
    #[doc = "Bits 19:21 - 21:19\\]
Defines L that indicates the width of the length field for CCM operations; the length field in bytes equals the value of CMM_L plus one. All values are supported."]
    #[inline(always)]
    #[must_use]
    pub fn ccm_l(&mut self) -> CcmLW<AesctlSpec> {
        CcmLW::new(self, 19)
    }
    #[doc = "Bits 22:24 - 24:22\\]
Defines M that indicates the length of the authentication field for CCM operations; the authentication field length equals two times the value of CCM_M plus one. Note: The Crypto peripheral always returns a 128-bit authentication field, of which the M least significant bytes are valid. All values are supported."]
    #[inline(always)]
    #[must_use]
    pub fn ccm_m(&mut self) -> CcmMW<AesctlSpec> {
        CcmMW::new(self, 22)
    }
    #[doc = "Bits 25:28 - 28:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved25(&mut self) -> Reserved25W<AesctlSpec> {
        Reserved25W::new(self, 25)
    }
    #[doc = "Bit 29 - 29:29\\]
IV must be read before the AES engine can start a new operation."]
    #[inline(always)]
    #[must_use]
    pub fn save_context(&mut self) -> SaveContextW<AesctlSpec> {
        SaveContextW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
If read as 1, this status bit indicates that an AES authentication TAG and/or IV block(s) is/are available for the Host to retrieve. This bit is only asserted if SAVE_CONTEXT is set to 1. The bit is mutually exclusive with CONTEXT_RDY. Writing 1 clears the bit to zero, indicating the Crypto peripheral can start its next operation. This bit is also cleared when the 4th word of the output TAG and/or IV is read. Note: All other mode bit writes will be ignored when this mode bit is written with 1. Note: This bit is controlled automatically by the Crypto peripheral for TAG read DMA operations. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
    #[inline(always)]
    #[must_use]
    pub fn saved_context_rdy(&mut self) -> SavedContextRdyW<AesctlSpec> {
        SavedContextRdyW::new(self, 30)
    }
}
#[doc = "AES Input/Output Buffer Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesctlSpec;
impl crate::RegisterSpec for AesctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesctl::R`](R) reader structure"]
impl crate::Readable for AesctlSpec {}
#[doc = "`write(|w| ..)` method takes [`aesctl::W`](W) writer structure"]
impl crate::Writable for AesctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESCTL to value 0x8000_0000"]
impl crate::Resettable for AesctlSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
