#[doc = "Register `HASHDATAIN14` reader"]
pub struct R(crate::R<HASHDATAIN14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASHDATAIN14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASHDATAIN14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASHDATAIN14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HASHDATAIN14` writer"]
pub struct W(crate::W<HASHDATAIN14_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASHDATAIN14_SPEC>;
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
impl From<crate::W<HASHDATAIN14_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASHDATAIN14_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HASH_DATA_IN` reader - 31:0\\]
HASH_DATA_IN\\[479:448\\]
These registers must be written with the 512-bit or 1024-bit (depending on block size of chosen SHA-2 algorithm) input data. The data lines are connected directly to the data input of the hash module and hence into the engine's internal data buffer. Writing to each of the registers triggers a corresponding 32-bit write enable to the internal buffer. Note: The host may only write the input data buffer when HASHIOBUFCTRL.RFD_IN is 1. If the HASHIOBUFCTRL.RFD_IN is 0, the engine is busy with processing. During processing, it is not allowed to write new input data. For message lengths larger than a block size, multiple blocks of data are written to this input buffer using a handshake through flags of the HASHIOBUFCTRL register. All blocks except the last are required to be 512 bits (or 1024 bits depending on block size) in size. If the last block is not 512 bits ( or 1024 bits depending on block size) long, only the least significant bits of data must be written, but they must be padded with 0s to the next 32-bit boundary. Host read operations from these register addresses return 0s."]
pub type HASH_DATA_IN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HASH_DATA_IN` writer - 31:0\\]
HASH_DATA_IN\\[479:448\\]
These registers must be written with the 512-bit or 1024-bit (depending on block size of chosen SHA-2 algorithm) input data. The data lines are connected directly to the data input of the hash module and hence into the engine's internal data buffer. Writing to each of the registers triggers a corresponding 32-bit write enable to the internal buffer. Note: The host may only write the input data buffer when HASHIOBUFCTRL.RFD_IN is 1. If the HASHIOBUFCTRL.RFD_IN is 0, the engine is busy with processing. During processing, it is not allowed to write new input data. For message lengths larger than a block size, multiple blocks of data are written to this input buffer using a handshake through flags of the HASHIOBUFCTRL register. All blocks except the last are required to be 512 bits (or 1024 bits depending on block size) in size. If the last block is not 512 bits ( or 1024 bits depending on block size) long, only the least significant bits of data must be written, but they must be padded with 0s to the next 32-bit boundary. Host read operations from these register addresses return 0s."]
pub type HASH_DATA_IN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HASHDATAIN14_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
HASH_DATA_IN\\[479:448\\]
These registers must be written with the 512-bit or 1024-bit (depending on block size of chosen SHA-2 algorithm) input data. The data lines are connected directly to the data input of the hash module and hence into the engine's internal data buffer. Writing to each of the registers triggers a corresponding 32-bit write enable to the internal buffer. Note: The host may only write the input data buffer when HASHIOBUFCTRL.RFD_IN is 1. If the HASHIOBUFCTRL.RFD_IN is 0, the engine is busy with processing. During processing, it is not allowed to write new input data. For message lengths larger than a block size, multiple blocks of data are written to this input buffer using a handshake through flags of the HASHIOBUFCTRL register. All blocks except the last are required to be 512 bits (or 1024 bits depending on block size) in size. If the last block is not 512 bits ( or 1024 bits depending on block size) long, only the least significant bits of data must be written, but they must be padded with 0s to the next 32-bit boundary. Host read operations from these register addresses return 0s."]
    #[inline(always)]
    pub fn hash_data_in(&self) -> HASH_DATA_IN_R {
        HASH_DATA_IN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
HASH_DATA_IN\\[479:448\\]
These registers must be written with the 512-bit or 1024-bit (depending on block size of chosen SHA-2 algorithm) input data. The data lines are connected directly to the data input of the hash module and hence into the engine's internal data buffer. Writing to each of the registers triggers a corresponding 32-bit write enable to the internal buffer. Note: The host may only write the input data buffer when HASHIOBUFCTRL.RFD_IN is 1. If the HASHIOBUFCTRL.RFD_IN is 0, the engine is busy with processing. During processing, it is not allowed to write new input data. For message lengths larger than a block size, multiple blocks of data are written to this input buffer using a handshake through flags of the HASHIOBUFCTRL register. All blocks except the last are required to be 512 bits (or 1024 bits depending on block size) in size. If the last block is not 512 bits ( or 1024 bits depending on block size) long, only the least significant bits of data must be written, but they must be padded with 0s to the next 32-bit boundary. Host read operations from these register addresses return 0s."]
    #[inline(always)]
    #[must_use]
    pub fn hash_data_in(&mut self) -> HASH_DATA_IN_W<0> {
        HASH_DATA_IN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HASH Data Input 14 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdatain14](index.html) module"]
pub struct HASHDATAIN14_SPEC;
impl crate::RegisterSpec for HASHDATAIN14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hashdatain14::R](R) reader structure"]
impl crate::Readable for HASHDATAIN14_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hashdatain14::W](W) writer structure"]
impl crate::Writable for HASHDATAIN14_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HASHDATAIN14 to value 0"]
impl crate::Resettable for HASHDATAIN14_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
