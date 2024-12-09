#[doc = "Register `AESDATAIN1` reader"]
pub type R = crate::R<Aesdatain1Spec>;
#[doc = "Register `AESDATAIN1` writer"]
pub type W = crate::W<Aesdatain1Spec>;
#[doc = "Field `DATA` writer - 31:0\\]
Data registers for input block data to the Crypto peripheral. These bits = AES Input Data\\[63:32\\]
of \\[127:0\\]
For normal operations, this register is not used, since data input and output is transferred from and to the AES engine via DMA. For a Host write operation, these registers must be written with the 128-bit input block for the next AES operation. Writing at a word-aligned offset within this address range will store the word (4 bytes) of data into the corresponding position of 4-word deep (16 bytes = 128-bit AES block) data input buffer. This buffer is used for the next AES operation. If the last data block is not completely filled with valid data (see notes below), it is allowed to write only the words with valid data. Next AES operation is triggered by writing to AESCTL.INPUT_RDY. Note: AES typically operates on 128 bits block multiple input data. The CTR, GCM and CCM modes form an exception. The last block of a CTR-mode message may contain less than 128 bits (refer to \\[NIST 800-38A\\]): 0 &lt; n &lt;= 128 bits. For GCM/CCM, the last block of both AAD and message data may contain less than 128 bits (refer to \\[NIST 800-38D\\]). The Crypto peripheral automatically pads or masks misaligned ending data blocks with zeroes for GCM, CCM and CBC-MAC. For CTR mode, the remaining data in an unaligned data block is ignored."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Data registers for input block data to the Crypto peripheral. These bits = AES Input Data\\[63:32\\]
of \\[127:0\\]
For normal operations, this register is not used, since data input and output is transferred from and to the AES engine via DMA. For a Host write operation, these registers must be written with the 128-bit input block for the next AES operation. Writing at a word-aligned offset within this address range will store the word (4 bytes) of data into the corresponding position of 4-word deep (16 bytes = 128-bit AES block) data input buffer. This buffer is used for the next AES operation. If the last data block is not completely filled with valid data (see notes below), it is allowed to write only the words with valid data. Next AES operation is triggered by writing to AESCTL.INPUT_RDY. Note: AES typically operates on 128 bits block multiple input data. The CTR, GCM and CCM modes form an exception. The last block of a CTR-mode message may contain less than 128 bits (refer to \\[NIST 800-38A\\]): 0 &lt; n &lt;= 128 bits. For GCM/CCM, the last block of both AAD and message data may contain less than 128 bits (refer to \\[NIST 800-38D\\]). The Crypto peripheral automatically pads or masks misaligned ending data blocks with zeroes for GCM, CCM and CBC-MAC. For CTR mode, the remaining data in an unaligned data block is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<Aesdatain1Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "AES Data Input/Output 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesdatain1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesdatain1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aesdatain1Spec;
impl crate::RegisterSpec for Aesdatain1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesdatain1::R`](R) reader structure"]
impl crate::Readable for Aesdatain1Spec {}
#[doc = "`write(|w| ..)` method takes [`aesdatain1::W`](W) writer structure"]
impl crate::Writable for Aesdatain1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESDATAIN1 to value 0"]
impl crate::Resettable for Aesdatain1Spec {
    const RESET_VALUE: u32 = 0;
}
