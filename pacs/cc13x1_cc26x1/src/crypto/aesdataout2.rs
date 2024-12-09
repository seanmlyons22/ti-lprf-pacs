#[doc = "Register `AESDATAOUT2` reader"]
pub type R = crate::R<Aesdataout2Spec>;
#[doc = "Register `AESDATAOUT2` writer"]
pub type W = crate::W<Aesdataout2Spec>;
#[doc = "Field `DATA` reader - 31:0\\]
Data registers for output block data from the Crypto peripheral. These bits = AES Output Data\\[95:64\\]
of \\[127:0\\]
For normal operations, this register is not used, since data input and output is transferred from and to the AES engine via DMA. For a Host read operation, these registers contain the 128-bit output block from the latest AES operation. Reading from a word-aligned offset within this address range will read one word (4 bytes) of data out the 4-word deep (16 bytes = 128-bits AES block) data output buffer. The words (4 words, one full block) should be read before the core will move the next block to the data output buffer. To empty the data output buffer, AESCTL.OUTPUT_RDY must be written. For the modes with authentication (CBC-MAC and CCM), the invalid (message) bytes/words can be written with any data. Note: The AAD / authentication only data is not copied to the output buffer but only used for authentication."]
pub type DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Data registers for output block data from the Crypto peripheral. These bits = AES Output Data\\[95:64\\]
of \\[127:0\\]
For normal operations, this register is not used, since data input and output is transferred from and to the AES engine via DMA. For a Host read operation, these registers contain the 128-bit output block from the latest AES operation. Reading from a word-aligned offset within this address range will read one word (4 bytes) of data out the 4-word deep (16 bytes = 128-bits AES block) data output buffer. The words (4 words, one full block) should be read before the core will move the next block to the data output buffer. To empty the data output buffer, AESCTL.OUTPUT_RDY must be written. For the modes with authentication (CBC-MAC and CCM), the invalid (message) bytes/words can be written with any data. Note: The AAD / authentication only data is not copied to the output buffer but only used for authentication."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {}
#[doc = "AES Data Input/Output 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesdataout2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesdataout2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aesdataout2Spec;
impl crate::RegisterSpec for Aesdataout2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesdataout2::R`](R) reader structure"]
impl crate::Readable for Aesdataout2Spec {}
#[doc = "`write(|w| ..)` method takes [`aesdataout2::W`](W) writer structure"]
impl crate::Writable for Aesdataout2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESDATAOUT2 to value 0"]
impl crate::Resettable for Aesdataout2Spec {
    const RESET_VALUE: u32 = 0;
}
