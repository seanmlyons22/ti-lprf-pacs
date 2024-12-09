#[doc = "Register `HASHDATAIN13` reader"]
pub type R = crate::R<Hashdatain13Spec>;
#[doc = "Register `HASHDATAIN13` writer"]
pub type W = crate::W<Hashdatain13Spec>;
#[doc = "Field `HASH_DATA_IN` writer - 31:0\\]
HASH_DATA_IN\\[447:416\\]
These registers must be written with the 512-bit or 1024-bit (depending on block size of chosen SHA-2 algorithm) input data. The data lines are connected directly to the data input of the hash module and hence into the engine's internal data buffer. Writing to each of the registers triggers a corresponding 32-bit write enable to the internal buffer. Note: The host may only write the input data buffer when HASHIOBUFCTRL.RFD_IN is 1. If the HASHIOBUFCTRL.RFD_IN is 0, the engine is busy with processing. During processing, it is not allowed to write new input data. For message lengths larger than a block size, multiple blocks of data are written to this input buffer using a handshake through flags of the HASHIOBUFCTRL register. All blocks except the last are required to be 512 bits (or 1024 bits depending on block size) in size. If the last block is not 512 bits ( or 1024 bits depending on block size) long, only the least significant bits of data must be written, but they must be padded with 0s to the next 32-bit boundary. Host read operations from these register addresses return 0s."]
pub type HashDataInW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
HASH_DATA_IN\\[447:416\\]
These registers must be written with the 512-bit or 1024-bit (depending on block size of chosen SHA-2 algorithm) input data. The data lines are connected directly to the data input of the hash module and hence into the engine's internal data buffer. Writing to each of the registers triggers a corresponding 32-bit write enable to the internal buffer. Note: The host may only write the input data buffer when HASHIOBUFCTRL.RFD_IN is 1. If the HASHIOBUFCTRL.RFD_IN is 0, the engine is busy with processing. During processing, it is not allowed to write new input data. For message lengths larger than a block size, multiple blocks of data are written to this input buffer using a handshake through flags of the HASHIOBUFCTRL register. All blocks except the last are required to be 512 bits (or 1024 bits depending on block size) in size. If the last block is not 512 bits ( or 1024 bits depending on block size) long, only the least significant bits of data must be written, but they must be padded with 0s to the next 32-bit boundary. Host read operations from these register addresses return 0s."]
    #[inline(always)]
    #[must_use]
    pub fn hash_data_in(&mut self) -> HashDataInW<Hashdatain13Spec> {
        HashDataInW::new(self, 0)
    }
}
#[doc = "HASH Data Input 13 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdatain13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdatain13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hashdatain13Spec;
impl crate::RegisterSpec for Hashdatain13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hashdatain13::R`](R) reader structure"]
impl crate::Readable for Hashdatain13Spec {}
#[doc = "`write(|w| ..)` method takes [`hashdatain13::W`](W) writer structure"]
impl crate::Writable for Hashdatain13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASHDATAIN13 to value 0"]
impl crate::Resettable for Hashdatain13Spec {
    const RESET_VALUE: u32 = 0;
}
