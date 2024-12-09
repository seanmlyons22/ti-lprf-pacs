#[doc = "Register `HASHDATAIN19` reader"]
pub type R = crate::R<Hashdatain19Spec>;
#[doc = "Register `HASHDATAIN19` writer"]
pub type W = crate::W<Hashdatain19Spec>;
#[doc = "Field `HASH_DATA_IN` writer - 31:0\\]
HASH_DATA_IN\\[639:608\\]
These registers must be written with the 512-bit input data. The data lines are connected directly to the data input of the hash module and hence into the engine's internal data buffer. Writing to each of the registers triggers a corresponding 32-bit write enable to the internal buffer. Note: The host may only write the input data buffer when HASHIOBUFCTRL.RFD_IN is 1. If the HASHIOBUFCTRL.RFD_IN is 0, the engine is busy with processing. During processing, it is not allowed to write new input data. For message lengths larger than 64 bytes, multiple blocks of data are written to this input buffer using a handshake through flags of the HASHIOBUFCTRL register. All blocks except the last are required to be 512 bits in size. If the last block is not 512 bits long, only the least significant bits of data must be written, but they must be padded with 0s to the next 32-bit boundary. Host read operations from these register addresses return 0s."]
pub type HashDataInW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
HASH_DATA_IN\\[639:608\\]
These registers must be written with the 512-bit input data. The data lines are connected directly to the data input of the hash module and hence into the engine's internal data buffer. Writing to each of the registers triggers a corresponding 32-bit write enable to the internal buffer. Note: The host may only write the input data buffer when HASHIOBUFCTRL.RFD_IN is 1. If the HASHIOBUFCTRL.RFD_IN is 0, the engine is busy with processing. During processing, it is not allowed to write new input data. For message lengths larger than 64 bytes, multiple blocks of data are written to this input buffer using a handshake through flags of the HASHIOBUFCTRL register. All blocks except the last are required to be 512 bits in size. If the last block is not 512 bits long, only the least significant bits of data must be written, but they must be padded with 0s to the next 32-bit boundary. Host read operations from these register addresses return 0s."]
    #[inline(always)]
    #[must_use]
    pub fn hash_data_in(&mut self) -> HashDataInW<Hashdatain19Spec> {
        HashDataInW::new(self, 0)
    }
}
#[doc = "HASH Data Input 19 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdatain19::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdatain19::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hashdatain19Spec;
impl crate::RegisterSpec for Hashdatain19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hashdatain19::R`](R) reader structure"]
impl crate::Readable for Hashdatain19Spec {}
#[doc = "`write(|w| ..)` method takes [`hashdatain19::W`](W) writer structure"]
impl crate::Writable for Hashdatain19Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASHDATAIN19 to value 0"]
impl crate::Resettable for Hashdatain19Spec {
    const RESET_VALUE: u32 = 0;
}
