#[doc = "Register `AESDATAOUT0` reader"]
pub struct R(crate::R<AESDATAOUT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AESDATAOUT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AESDATAOUT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AESDATAOUT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AESDATAOUT0` writer"]
pub struct W(crate::W<AESDATAOUT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AESDATAOUT0_SPEC>;
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
impl From<crate::W<AESDATAOUT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AESDATAOUT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - 31:0\\]
Data register 0 for output block data from the Crypto peripheral. These bits = AES Output Data\\[31:0\\]
of {127:0\\]
For normal operations, this register is not used, since data input and output is transferred from and to the AES engine via DMA. For a Host read operation, these registers contain the 128-bit output block from the latest AES operation. Reading from a word-aligned offset within this address range will read one word (4 bytes) of data out the 4-word deep (16 bytes = 128-bits AES block) data output buffer. The words (4 words, one full block) should be read before the core will move the next block to the data output buffer. To empty the data output buffer, AESCTL.OUTPUT_RDY must be written. For the modes with authentication (CBC-MAC, GCM and CCM), the invalid (message) bytes/words can be written with any data. Note: The AAD / authentication only data is not copied to the output buffer but only used for authentication."]
pub type DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATA` writer - 31:0\\]
Data register 0 for output block data from the Crypto peripheral. These bits = AES Output Data\\[31:0\\]
of {127:0\\]
For normal operations, this register is not used, since data input and output is transferred from and to the AES engine via DMA. For a Host read operation, these registers contain the 128-bit output block from the latest AES operation. Reading from a word-aligned offset within this address range will read one word (4 bytes) of data out the 4-word deep (16 bytes = 128-bits AES block) data output buffer. The words (4 words, one full block) should be read before the core will move the next block to the data output buffer. To empty the data output buffer, AESCTL.OUTPUT_RDY must be written. For the modes with authentication (CBC-MAC, GCM and CCM), the invalid (message) bytes/words can be written with any data. Note: The AAD / authentication only data is not copied to the output buffer but only used for authentication."]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AESDATAOUT0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Data register 0 for output block data from the Crypto peripheral. These bits = AES Output Data\\[31:0\\]
of {127:0\\]
For normal operations, this register is not used, since data input and output is transferred from and to the AES engine via DMA. For a Host read operation, these registers contain the 128-bit output block from the latest AES operation. Reading from a word-aligned offset within this address range will read one word (4 bytes) of data out the 4-word deep (16 bytes = 128-bits AES block) data output buffer. The words (4 words, one full block) should be read before the core will move the next block to the data output buffer. To empty the data output buffer, AESCTL.OUTPUT_RDY must be written. For the modes with authentication (CBC-MAC, GCM and CCM), the invalid (message) bytes/words can be written with any data. Note: The AAD / authentication only data is not copied to the output buffer but only used for authentication."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Data register 0 for output block data from the Crypto peripheral. These bits = AES Output Data\\[31:0\\]
of {127:0\\]
For normal operations, this register is not used, since data input and output is transferred from and to the AES engine via DMA. For a Host read operation, these registers contain the 128-bit output block from the latest AES operation. Reading from a word-aligned offset within this address range will read one word (4 bytes) of data out the 4-word deep (16 bytes = 128-bits AES block) data output buffer. The words (4 words, one full block) should be read before the core will move the next block to the data output buffer. To empty the data output buffer, AESCTL.OUTPUT_RDY must be written. For the modes with authentication (CBC-MAC, GCM and CCM), the invalid (message) bytes/words can be written with any data. Note: The AAD / authentication only data is not copied to the output buffer but only used for authentication."]
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
#[doc = "Data Input/Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesdataout0](index.html) module"]
pub struct AESDATAOUT0_SPEC;
impl crate::RegisterSpec for AESDATAOUT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aesdataout0::R](R) reader structure"]
impl crate::Readable for AESDATAOUT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aesdataout0::W](W) writer structure"]
impl crate::Writable for AESDATAOUT0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AESDATAOUT0 to value 0"]
impl crate::Resettable for AESDATAOUT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
