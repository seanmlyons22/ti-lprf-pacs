#[doc = "Register `HASHIOBUFCTRL` reader"]
pub type R = crate::R<HashiobufctrlSpec>;
#[doc = "Register `HASHIOBUFCTRL` writer"]
pub type W = crate::W<HashiobufctrlSpec>;
#[doc = "Field `OUTPUT_FULL` reader - 0:0\\]
Indicates that the output buffer registers (HASHDIGESTn) are available for reading by the host. When this bit reads 0, the output buffer registers are released; the hash engine is allowed to write new data to it. In this case, the registers should not be read by the host. When this bit reads 1, the hash engine has stored the result of the latest hash operation in the output buffer registers. As long as this bit reads 1, the host may read output buffer registers and the hash engine is prevented from writing new data to the output buffer. After retrieving the hash result data from the output buffer, the host must write a 1 to this bit to clear it. This makes the digest output buffer available for the hash engine to store new hash results. Writing 0 to this bit has no effect. Note: If this bit is asserted (1) no new operation should be started before the digest is retrieved from the hash engine and this bit is cleared (0)."]
pub type OutputFullR = crate::BitReader;
#[doc = "Field `OUTPUT_FULL` writer - 0:0\\]
Indicates that the output buffer registers (HASHDIGESTn) are available for reading by the host. When this bit reads 0, the output buffer registers are released; the hash engine is allowed to write new data to it. In this case, the registers should not be read by the host. When this bit reads 1, the hash engine has stored the result of the latest hash operation in the output buffer registers. As long as this bit reads 1, the host may read output buffer registers and the hash engine is prevented from writing new data to the output buffer. After retrieving the hash result data from the output buffer, the host must write a 1 to this bit to clear it. This makes the digest output buffer available for the hash engine to store new hash results. Writing 0 to this bit has no effect. Note: If this bit is asserted (1) no new operation should be started before the digest is retrieved from the hash engine and this bit is cleared (0)."]
pub type OutputFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_IN_AV` reader - 1:1\\]
Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates that the HASHDATAINn registers contain new input data for processing. The host must write a 1 to this bit to start processing the data in HASHDATAINn; the hash engine will process the new data as soon as it is ready for it (rfd_in bit is 1). Writing 0 to this bit has no effect. This bit is automatically cleared (i.e. reads as 0) when the hash engine starts processing the HASHDATAINn contents. This bit reads 1 between the time it was set by the host and the hash engine actually starts processing the input data block."]
pub type DataInAvR = crate::BitReader;
#[doc = "Field `DATA_IN_AV` writer - 1:1\\]
Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates that the HASHDATAINn registers contain new input data for processing. The host must write a 1 to this bit to start processing the data in HASHDATAINn; the hash engine will process the new data as soon as it is ready for it (rfd_in bit is 1). Writing 0 to this bit has no effect. This bit is automatically cleared (i.e. reads as 0) when the hash engine starts processing the HASHDATAINn contents. This bit reads 1 between the time it was set by the host and the hash engine actually starts processing the input data block."]
pub type DataInAvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFD_IN` reader - 2:2\\]
Note: The bit description below is only applicable when data is sent through the slave interface. This bit can be ignored when data is received through the DMA. Read-only status of the input buffer of the hash engine. When 1, the input buffer of the hash engine can accept new data; the HASHDATAINn registers can safely be populated with new data. When 0, the input buffer of the hash engine is processing the data that is currently in HASHDATAINn; writing new data to these registers is not allowed."]
pub type RfdInR = crate::BitReader;
#[doc = "Field `RFD_IN` writer - 2:2\\]
Note: The bit description below is only applicable when data is sent through the slave interface. This bit can be ignored when data is received through the DMA. Read-only status of the input buffer of the hash engine. When 1, the input buffer of the hash engine can accept new data; the HASHDATAINn registers can safely be populated with new data. When 0, the input buffer of the hash engine is processing the data that is currently in HASHDATAINn; writing new data to these registers is not allowed."]
pub type RfdInW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 4:3\\]
Write 0s and ignore on reading"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `RESERVED3` writer - 4:3\\]
Write 0s and ignore on reading"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PAD_MESSAGE` reader - 5:5\\]
Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates that the HASHDATAINn registers hold the last data of the message and hash padding must be applied. The host must write this bit to 1 in order to indicate to the hash engine that the HASHDATAINn register currently holds the last data of the message. When pad_message is set to 1, the hash engine will add padding bits to the data currently in the HASHDATAINn register. When the last message block is smaller than 1024 bits for SHA-512/384 or 512 bits for SHA-256/224, the pad_message bit must be set to ‘1’ together with the data_in_av bit. When the last message block is equal to the block size, pad_message may be set together with data_in_av. In this case, the pad_message bit may also be set after the last data block has been written to the hash engine (so when the rfd_in bit has become ‘1’ again after writing the last data block). Writing 0 to this bit has no effect. This bit is automatically cleared (i.e. reads 0) by the hash engine. This bit reads 1 between the time it was set by the host and the hash engine interpreted its value."]
pub type PadMessageR = crate::BitReader;
#[doc = "Field `PAD_MESSAGE` writer - 5:5\\]
Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates that the HASHDATAINn registers hold the last data of the message and hash padding must be applied. The host must write this bit to 1 in order to indicate to the hash engine that the HASHDATAINn register currently holds the last data of the message. When pad_message is set to 1, the hash engine will add padding bits to the data currently in the HASHDATAINn register. When the last message block is smaller than 1024 bits for SHA-512/384 or 512 bits for SHA-256/224, the pad_message bit must be set to ‘1’ together with the data_in_av bit. When the last message block is equal to the block size, pad_message may be set together with data_in_av. In this case, the pad_message bit may also be set after the last data block has been written to the hash engine (so when the rfd_in bit has become ‘1’ again after writing the last data block). Writing 0 to this bit has no effect. This bit is automatically cleared (i.e. reads 0) by the hash engine. This bit reads 1 between the time it was set by the host and the hash engine interpreted its value."]
pub type PadMessageW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GET_DIGEST` reader - 6:6\\]
Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates whether the hash engine should provide the hash digest. When provided simultaneously with data_in_av, the hash digest is provided after processing the data that is currently in the HASHDATAINn register. When provided without data_in_av, the current internal digest buffer value is copied to the HASHDIGESTn registers. The host must write a 1 to this bit to make the intermediate hash digest available. Writing 0 to this bit has no effect. This bit is automatically cleared (that is, reads 0) when the hash engine has processed the contents of the HASHDATAINn register. In the period between this bit is set by the host and the actual HASHDATAINn processing, this bit reads 1."]
pub type GetDigestR = crate::BitReader;
#[doc = "Field `GET_DIGEST` writer - 6:6\\]
Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates whether the hash engine should provide the hash digest. When provided simultaneously with data_in_av, the hash digest is provided after processing the data that is currently in the HASHDATAINn register. When provided without data_in_av, the current internal digest buffer value is copied to the HASHDIGESTn registers. The host must write a 1 to this bit to make the intermediate hash digest available. Writing 0 to this bit has no effect. This bit is automatically cleared (that is, reads 0) when the hash engine has processed the contents of the HASHDATAINn register. In the period between this bit is set by the host and the actual HASHDATAINn processing, this bit reads 1."]
pub type GetDigestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAD_DMA_MESSAGE` reader - 7:7\\]
Note: This bit must only be used when data is supplied through the DMA. It should not be used when data is supplied through the slave interface. This bit indicates whether the hash engine has to pad the message, received through the DMA and finalize the hash. When set to 1, the hash engine pads the last block using the programmed length. After padding, the final hash result is calculated. When set to 0, the hash engine treats the last written block as block-size aligned and calculates the intermediate digest. This bit is automatically cleared when the last DMA data block is arrived in the hash engine."]
pub type PadDmaMessageR = crate::BitReader;
#[doc = "Field `PAD_DMA_MESSAGE` writer - 7:7\\]
Note: This bit must only be used when data is supplied through the DMA. It should not be used when data is supplied through the slave interface. This bit indicates whether the hash engine has to pad the message, received through the DMA and finalize the hash. When set to 1, the hash engine pads the last block using the programmed length. After padding, the final hash result is calculated. When set to 0, the hash engine treats the last written block as block-size aligned and calculates the intermediate digest. This bit is automatically cleared when the last DMA data block is arrived in the hash engine."]
pub type PadDmaMessageW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates that the output buffer registers (HASHDIGESTn) are available for reading by the host. When this bit reads 0, the output buffer registers are released; the hash engine is allowed to write new data to it. In this case, the registers should not be read by the host. When this bit reads 1, the hash engine has stored the result of the latest hash operation in the output buffer registers. As long as this bit reads 1, the host may read output buffer registers and the hash engine is prevented from writing new data to the output buffer. After retrieving the hash result data from the output buffer, the host must write a 1 to this bit to clear it. This makes the digest output buffer available for the hash engine to store new hash results. Writing 0 to this bit has no effect. Note: If this bit is asserted (1) no new operation should be started before the digest is retrieved from the hash engine and this bit is cleared (0)."]
    #[inline(always)]
    pub fn output_full(&self) -> OutputFullR {
        OutputFullR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates that the HASHDATAINn registers contain new input data for processing. The host must write a 1 to this bit to start processing the data in HASHDATAINn; the hash engine will process the new data as soon as it is ready for it (rfd_in bit is 1). Writing 0 to this bit has no effect. This bit is automatically cleared (i.e. reads as 0) when the hash engine starts processing the HASHDATAINn contents. This bit reads 1 between the time it was set by the host and the hash engine actually starts processing the input data block."]
    #[inline(always)]
    pub fn data_in_av(&self) -> DataInAvR {
        DataInAvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Note: The bit description below is only applicable when data is sent through the slave interface. This bit can be ignored when data is received through the DMA. Read-only status of the input buffer of the hash engine. When 1, the input buffer of the hash engine can accept new data; the HASHDATAINn registers can safely be populated with new data. When 0, the input buffer of the hash engine is processing the data that is currently in HASHDATAINn; writing new data to these registers is not allowed."]
    #[inline(always)]
    pub fn rfd_in(&self) -> RfdInR {
        RfdInR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Write 0s and ignore on reading"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates that the HASHDATAINn registers hold the last data of the message and hash padding must be applied. The host must write this bit to 1 in order to indicate to the hash engine that the HASHDATAINn register currently holds the last data of the message. When pad_message is set to 1, the hash engine will add padding bits to the data currently in the HASHDATAINn register. When the last message block is smaller than 1024 bits for SHA-512/384 or 512 bits for SHA-256/224, the pad_message bit must be set to ‘1’ together with the data_in_av bit. When the last message block is equal to the block size, pad_message may be set together with data_in_av. In this case, the pad_message bit may also be set after the last data block has been written to the hash engine (so when the rfd_in bit has become ‘1’ again after writing the last data block). Writing 0 to this bit has no effect. This bit is automatically cleared (i.e. reads 0) by the hash engine. This bit reads 1 between the time it was set by the host and the hash engine interpreted its value."]
    #[inline(always)]
    pub fn pad_message(&self) -> PadMessageR {
        PadMessageR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates whether the hash engine should provide the hash digest. When provided simultaneously with data_in_av, the hash digest is provided after processing the data that is currently in the HASHDATAINn register. When provided without data_in_av, the current internal digest buffer value is copied to the HASHDIGESTn registers. The host must write a 1 to this bit to make the intermediate hash digest available. Writing 0 to this bit has no effect. This bit is automatically cleared (that is, reads 0) when the hash engine has processed the contents of the HASHDATAINn register. In the period between this bit is set by the host and the actual HASHDATAINn processing, this bit reads 1."]
    #[inline(always)]
    pub fn get_digest(&self) -> GetDigestR {
        GetDigestR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Note: This bit must only be used when data is supplied through the DMA. It should not be used when data is supplied through the slave interface. This bit indicates whether the hash engine has to pad the message, received through the DMA and finalize the hash. When set to 1, the hash engine pads the last block using the programmed length. After padding, the final hash result is calculated. When set to 0, the hash engine treats the last written block as block-size aligned and calculates the intermediate digest. This bit is automatically cleared when the last DMA data block is arrived in the hash engine."]
    #[inline(always)]
    pub fn pad_dma_message(&self) -> PadDmaMessageR {
        PadDmaMessageR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates that the output buffer registers (HASHDIGESTn) are available for reading by the host. When this bit reads 0, the output buffer registers are released; the hash engine is allowed to write new data to it. In this case, the registers should not be read by the host. When this bit reads 1, the hash engine has stored the result of the latest hash operation in the output buffer registers. As long as this bit reads 1, the host may read output buffer registers and the hash engine is prevented from writing new data to the output buffer. After retrieving the hash result data from the output buffer, the host must write a 1 to this bit to clear it. This makes the digest output buffer available for the hash engine to store new hash results. Writing 0 to this bit has no effect. Note: If this bit is asserted (1) no new operation should be started before the digest is retrieved from the hash engine and this bit is cleared (0)."]
    #[inline(always)]
    #[must_use]
    pub fn output_full(&mut self) -> OutputFullW<HashiobufctrlSpec> {
        OutputFullW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates that the HASHDATAINn registers contain new input data for processing. The host must write a 1 to this bit to start processing the data in HASHDATAINn; the hash engine will process the new data as soon as it is ready for it (rfd_in bit is 1). Writing 0 to this bit has no effect. This bit is automatically cleared (i.e. reads as 0) when the hash engine starts processing the HASHDATAINn contents. This bit reads 1 between the time it was set by the host and the hash engine actually starts processing the input data block."]
    #[inline(always)]
    #[must_use]
    pub fn data_in_av(&mut self) -> DataInAvW<HashiobufctrlSpec> {
        DataInAvW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Note: The bit description below is only applicable when data is sent through the slave interface. This bit can be ignored when data is received through the DMA. Read-only status of the input buffer of the hash engine. When 1, the input buffer of the hash engine can accept new data; the HASHDATAINn registers can safely be populated with new data. When 0, the input buffer of the hash engine is processing the data that is currently in HASHDATAINn; writing new data to these registers is not allowed."]
    #[inline(always)]
    #[must_use]
    pub fn rfd_in(&mut self) -> RfdInW<HashiobufctrlSpec> {
        RfdInW::new(self, 2)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Write 0s and ignore on reading"]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<HashiobufctrlSpec> {
        Reserved3W::new(self, 3)
    }
    #[doc = "Bit 5 - 5:5\\]
Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates that the HASHDATAINn registers hold the last data of the message and hash padding must be applied. The host must write this bit to 1 in order to indicate to the hash engine that the HASHDATAINn register currently holds the last data of the message. When pad_message is set to 1, the hash engine will add padding bits to the data currently in the HASHDATAINn register. When the last message block is smaller than 1024 bits for SHA-512/384 or 512 bits for SHA-256/224, the pad_message bit must be set to ‘1’ together with the data_in_av bit. When the last message block is equal to the block size, pad_message may be set together with data_in_av. In this case, the pad_message bit may also be set after the last data block has been written to the hash engine (so when the rfd_in bit has become ‘1’ again after writing the last data block). Writing 0 to this bit has no effect. This bit is automatically cleared (i.e. reads 0) by the hash engine. This bit reads 1 between the time it was set by the host and the hash engine interpreted its value."]
    #[inline(always)]
    #[must_use]
    pub fn pad_message(&mut self) -> PadMessageW<HashiobufctrlSpec> {
        PadMessageW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates whether the hash engine should provide the hash digest. When provided simultaneously with data_in_av, the hash digest is provided after processing the data that is currently in the HASHDATAINn register. When provided without data_in_av, the current internal digest buffer value is copied to the HASHDIGESTn registers. The host must write a 1 to this bit to make the intermediate hash digest available. Writing 0 to this bit has no effect. This bit is automatically cleared (that is, reads 0) when the hash engine has processed the contents of the HASHDATAINn register. In the period between this bit is set by the host and the actual HASHDATAINn processing, this bit reads 1."]
    #[inline(always)]
    #[must_use]
    pub fn get_digest(&mut self) -> GetDigestW<HashiobufctrlSpec> {
        GetDigestW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Note: This bit must only be used when data is supplied through the DMA. It should not be used when data is supplied through the slave interface. This bit indicates whether the hash engine has to pad the message, received through the DMA and finalize the hash. When set to 1, the hash engine pads the last block using the programmed length. After padding, the final hash result is calculated. When set to 0, the hash engine treats the last written block as block-size aligned and calculates the intermediate digest. This bit is automatically cleared when the last DMA data block is arrived in the hash engine."]
    #[inline(always)]
    #[must_use]
    pub fn pad_dma_message(&mut self) -> PadDmaMessageW<HashiobufctrlSpec> {
        PadDmaMessageW::new(self, 7)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<HashiobufctrlSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "HASH Input_Output Buffer Control This register pair shares a single address location and contains bits that control and monitor the data flow between the host and the hash engine.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashiobufctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashiobufctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashiobufctrlSpec;
impl crate::RegisterSpec for HashiobufctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hashiobufctrl::R`](R) reader structure"]
impl crate::Readable for HashiobufctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`hashiobufctrl::W`](W) writer structure"]
impl crate::Writable for HashiobufctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASHIOBUFCTRL to value 0x04"]
impl crate::Resettable for HashiobufctrlSpec {
    const RESET_VALUE: u32 = 0x04;
}
