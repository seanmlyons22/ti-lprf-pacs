#[doc = "Register `AESDATAIN0` reader"]
pub type R = crate::R<Aesdatain0Spec>;
#[doc = "Register `AESDATAIN0` writer"]
pub type W = crate::W<Aesdatain0Spec>;
#[doc = "Field `AES_DATA_IN_OUT` writer - 31:0\\]
AES input data\\[31:0\\]
/ AES output data\\[31:0\\]
Data registers for input/output block data to/from the EIP-120t. For normal operations, this register is not used, since data input and output is transferred from and to the AES core via DMA. For a host write operation, these registers must be written with the 128-bit input block for the next AES operation. Writing at a word-aligned offset within this address range stores the word (4 bytes) of data into the corresponding position of 4-word deep (16 bytes = 128-bit AES block) data input buffer. This buffer is used for the next AES operation. If the last data block is not completely filled with valid data (see notes below), it is allowed to write only the words with valid data. Next AES operation is triggered by writing to the input_ready flag of the AES_CTRL register. For a host read operation, these registers contain the 128-bit output block from the latest AES operation. Reading from a word-aligned offset within this address range reads one word (4 bytes) of data out the 4-word deep (16 bytes = 128-bits AES block) data output buffer. The words (4 words, one full block) should be read before the core will move the next block to the data output buffer. To empty the data output buffer, the output_ready flag of the AES_CTRL register must be written. For the modes with authentication (CBC-MAC, GCM and CCM), the invalid (message) bytes/words can be written with any data. Note: AES typically operates on 128 bits block multiple input data. The CTR, GCM and CCM modes form an exception. The last block of a CTR-mode message may contain less than 128 bits (refer to \\[NIST 800-38A\\]). For GCM/CCM, the last block of both AAD and message data may contain less than 128 bits (refer to \\[NIST 800-38D\\]). The EIP-120t automatically pads or masks misaligned ending data blocks with 0s for GCM, CCM and CBC-MAC. For CTR mode, the remaining data in an unaligned data block is ignored. Note: The AAD / authentication only data is not copied to the output buffer but only used for authentication."]
pub type AesDataInOutW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
AES input data\\[31:0\\]
/ AES output data\\[31:0\\]
Data registers for input/output block data to/from the EIP-120t. For normal operations, this register is not used, since data input and output is transferred from and to the AES core via DMA. For a host write operation, these registers must be written with the 128-bit input block for the next AES operation. Writing at a word-aligned offset within this address range stores the word (4 bytes) of data into the corresponding position of 4-word deep (16 bytes = 128-bit AES block) data input buffer. This buffer is used for the next AES operation. If the last data block is not completely filled with valid data (see notes below), it is allowed to write only the words with valid data. Next AES operation is triggered by writing to the input_ready flag of the AES_CTRL register. For a host read operation, these registers contain the 128-bit output block from the latest AES operation. Reading from a word-aligned offset within this address range reads one word (4 bytes) of data out the 4-word deep (16 bytes = 128-bits AES block) data output buffer. The words (4 words, one full block) should be read before the core will move the next block to the data output buffer. To empty the data output buffer, the output_ready flag of the AES_CTRL register must be written. For the modes with authentication (CBC-MAC, GCM and CCM), the invalid (message) bytes/words can be written with any data. Note: AES typically operates on 128 bits block multiple input data. The CTR, GCM and CCM modes form an exception. The last block of a CTR-mode message may contain less than 128 bits (refer to \\[NIST 800-38A\\]). For GCM/CCM, the last block of both AAD and message data may contain less than 128 bits (refer to \\[NIST 800-38D\\]). The EIP-120t automatically pads or masks misaligned ending data blocks with 0s for GCM, CCM and CBC-MAC. For CTR mode, the remaining data in an unaligned data block is ignored. Note: The AAD / authentication only data is not copied to the output buffer but only used for authentication."]
    #[inline(always)]
    #[must_use]
    pub fn aes_data_in_out(&mut self) -> AesDataInOutW<Aesdatain0Spec> {
        AesDataInOutW::new(self, 0)
    }
}
#[doc = "AES Data Input_Output 0 The data registers are typically accessed through the DMA and not with host writes and/or reads. However, for debugging purposes the data input/output registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesdatain0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesdatain0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aesdatain0Spec;
impl crate::RegisterSpec for Aesdatain0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesdatain0::R`](R) reader structure"]
impl crate::Readable for Aesdatain0Spec {}
#[doc = "`write(|w| ..)` method takes [`aesdatain0::W`](W) writer structure"]
impl crate::Writable for Aesdatain0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESDATAIN0 to value 0"]
impl crate::Resettable for Aesdatain0Spec {
    const RESET_VALUE: u32 = 0;
}
