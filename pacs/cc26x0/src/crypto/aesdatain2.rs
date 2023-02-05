#[doc = "Register `AESDATAIN2` reader"]
pub struct R(crate::R<AESDATAIN2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AESDATAIN2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AESDATAIN2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AESDATAIN2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AESDATAIN2` writer"]
pub struct W(crate::W<AESDATAIN2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AESDATAIN2_SPEC>;
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
impl From<crate::W<AESDATAIN2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AESDATAIN2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - 31:0\\]
Data registers for input block data to the Crypto peripheral. These bits = AES Input Data\\[95:64\\]
of \\[127:0\\]
For normal operations, this register is not used, since data input and output is transferred from and to the AES engine via DMA. For a Host write operation, these registers must be written with the 128-bit input block for the next AES operation. Writing at a word-aligned offset within this address range will store the word (4 bytes) of data into the corresponding position of 4-word deep (16 bytes = 128-bit AES block) data input buffer. This buffer is used for the next AES operation. If the last data block is not completely filled with valid data (see notes below), it is allowed to write only the words with valid data. Next AES operation is triggered by writing to AESCTL.INPUT_RDY. Note: AES typically operates on 128 bits block multiple input data. The CTR, GCM and CCM modes form an exception. The last block of a CTR-mode message may contain less than 128 bits (refer to \\[NIST 800-38A\\]): 0 < n <= 128 bits. For GCM/CCM, the last block of both AAD and message data may contain less than 128 bits (refer to \\[NIST 800-38D\\]). The Crypto peripheral automatically pads or masks misaligned ending data blocks with zeroes for GCM, CCM and CBC-MAC. For CTR mode, the remaining data in an unaligned data block is ignored."]
pub type DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATA` writer - 31:0\\]
Data registers for input block data to the Crypto peripheral. These bits = AES Input Data\\[95:64\\]
of \\[127:0\\]
For normal operations, this register is not used, since data input and output is transferred from and to the AES engine via DMA. For a Host write operation, these registers must be written with the 128-bit input block for the next AES operation. Writing at a word-aligned offset within this address range will store the word (4 bytes) of data into the corresponding position of 4-word deep (16 bytes = 128-bit AES block) data input buffer. This buffer is used for the next AES operation. If the last data block is not completely filled with valid data (see notes below), it is allowed to write only the words with valid data. Next AES operation is triggered by writing to AESCTL.INPUT_RDY. Note: AES typically operates on 128 bits block multiple input data. The CTR, GCM and CCM modes form an exception. The last block of a CTR-mode message may contain less than 128 bits (refer to \\[NIST 800-38A\\]): 0 < n <= 128 bits. For GCM/CCM, the last block of both AAD and message data may contain less than 128 bits (refer to \\[NIST 800-38D\\]). The Crypto peripheral automatically pads or masks misaligned ending data blocks with zeroes for GCM, CCM and CBC-MAC. For CTR mode, the remaining data in an unaligned data block is ignored."]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AESDATAIN2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Data registers for input block data to the Crypto peripheral. These bits = AES Input Data\\[95:64\\]
of \\[127:0\\]
For normal operations, this register is not used, since data input and output is transferred from and to the AES engine via DMA. For a Host write operation, these registers must be written with the 128-bit input block for the next AES operation. Writing at a word-aligned offset within this address range will store the word (4 bytes) of data into the corresponding position of 4-word deep (16 bytes = 128-bit AES block) data input buffer. This buffer is used for the next AES operation. If the last data block is not completely filled with valid data (see notes below), it is allowed to write only the words with valid data. Next AES operation is triggered by writing to AESCTL.INPUT_RDY. Note: AES typically operates on 128 bits block multiple input data. The CTR, GCM and CCM modes form an exception. The last block of a CTR-mode message may contain less than 128 bits (refer to \\[NIST 800-38A\\]): 0 < n <= 128 bits. For GCM/CCM, the last block of both AAD and message data may contain less than 128 bits (refer to \\[NIST 800-38D\\]). The Crypto peripheral automatically pads or masks misaligned ending data blocks with zeroes for GCM, CCM and CBC-MAC. For CTR mode, the remaining data in an unaligned data block is ignored."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Data registers for input block data to the Crypto peripheral. These bits = AES Input Data\\[95:64\\]
of \\[127:0\\]
For normal operations, this register is not used, since data input and output is transferred from and to the AES engine via DMA. For a Host write operation, these registers must be written with the 128-bit input block for the next AES operation. Writing at a word-aligned offset within this address range will store the word (4 bytes) of data into the corresponding position of 4-word deep (16 bytes = 128-bit AES block) data input buffer. This buffer is used for the next AES operation. If the last data block is not completely filled with valid data (see notes below), it is allowed to write only the words with valid data. Next AES operation is triggered by writing to AESCTL.INPUT_RDY. Note: AES typically operates on 128 bits block multiple input data. The CTR, GCM and CCM modes form an exception. The last block of a CTR-mode message may contain less than 128 bits (refer to \\[NIST 800-38A\\]): 0 < n <= 128 bits. For GCM/CCM, the last block of both AAD and message data may contain less than 128 bits (refer to \\[NIST 800-38D\\]). The Crypto peripheral automatically pads or masks misaligned ending data blocks with zeroes for GCM, CCM and CBC-MAC. For CTR mode, the remaining data in an unaligned data block is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES Data Input/Output 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesdatain2](index.html) module"]
pub struct AESDATAIN2_SPEC;
impl crate::RegisterSpec for AESDATAIN2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aesdatain2::R](R) reader structure"]
impl crate::Readable for AESDATAIN2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aesdatain2::W](W) writer structure"]
impl crate::Writable for AESDATAIN2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AESDATAIN2 to value 0"]
impl crate::Resettable for AESDATAIN2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
