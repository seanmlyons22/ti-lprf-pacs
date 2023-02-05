#[doc = "Register `HASHINLENL` reader"]
pub struct R(crate::R<HASHINLENL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASHINLENL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASHINLENL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASHINLENL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HASHINLENL` writer"]
pub struct W(crate::W<HASHINLENL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASHINLENL_SPEC>;
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
impl From<crate::W<HASHINLENL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASHINLENL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LENGTH_IN` reader - 31:0\\]
LENGTH_IN\\[31:0\\]
Message length registers. The content of these registers is used by the hash engine during the message padding phase of the hash session. The data lines of this registers are directly connected to the interface of the hash engine. For a write operation by the host, these registers should be written with the message length in bits. Final hash operations: The total input data length must be programmed for new hash operations that require finalization (padding). The input data must be provided through the slave or DMA interface. Continued hash operations (finalized): For continued hash operations that require finalization, the total message length must be programmed, including the length of previously hashed data that corresponds to the written input digest. Non-final hash operations: For hash operations that do not require finalization (input data length is multiple of 1024-bits for SHA-512, which is SHA-512 data block size. Or multiple of 512-bits for SHA-256, which is SHA-256 data block size), the length field does not need to be programmed since it is not used by the operation If the message length in bits is below (2^32-1), then only this register needs to be written. The hardware automatically sets HASH_LENGTH_IN_H to 0s in this case. The host may write the length register at any time during the hash session when the HASHIOBUFCTRL.RFD_IN is high. The length register must be written before the last data of the active hash session is written into the hash engine. host read operations from these register locations will return 0s. Note: When getting data from DMA, this register must be programmed before DMA is programmed to start."]
pub type LENGTH_IN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LENGTH_IN` writer - 31:0\\]
LENGTH_IN\\[31:0\\]
Message length registers. The content of these registers is used by the hash engine during the message padding phase of the hash session. The data lines of this registers are directly connected to the interface of the hash engine. For a write operation by the host, these registers should be written with the message length in bits. Final hash operations: The total input data length must be programmed for new hash operations that require finalization (padding). The input data must be provided through the slave or DMA interface. Continued hash operations (finalized): For continued hash operations that require finalization, the total message length must be programmed, including the length of previously hashed data that corresponds to the written input digest. Non-final hash operations: For hash operations that do not require finalization (input data length is multiple of 1024-bits for SHA-512, which is SHA-512 data block size. Or multiple of 512-bits for SHA-256, which is SHA-256 data block size), the length field does not need to be programmed since it is not used by the operation If the message length in bits is below (2^32-1), then only this register needs to be written. The hardware automatically sets HASH_LENGTH_IN_H to 0s in this case. The host may write the length register at any time during the hash session when the HASHIOBUFCTRL.RFD_IN is high. The length register must be written before the last data of the active hash session is written into the hash engine. host read operations from these register locations will return 0s. Note: When getting data from DMA, this register must be programmed before DMA is programmed to start."]
pub type LENGTH_IN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HASHINLENL_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
LENGTH_IN\\[31:0\\]
Message length registers. The content of these registers is used by the hash engine during the message padding phase of the hash session. The data lines of this registers are directly connected to the interface of the hash engine. For a write operation by the host, these registers should be written with the message length in bits. Final hash operations: The total input data length must be programmed for new hash operations that require finalization (padding). The input data must be provided through the slave or DMA interface. Continued hash operations (finalized): For continued hash operations that require finalization, the total message length must be programmed, including the length of previously hashed data that corresponds to the written input digest. Non-final hash operations: For hash operations that do not require finalization (input data length is multiple of 1024-bits for SHA-512, which is SHA-512 data block size. Or multiple of 512-bits for SHA-256, which is SHA-256 data block size), the length field does not need to be programmed since it is not used by the operation If the message length in bits is below (2^32-1), then only this register needs to be written. The hardware automatically sets HASH_LENGTH_IN_H to 0s in this case. The host may write the length register at any time during the hash session when the HASHIOBUFCTRL.RFD_IN is high. The length register must be written before the last data of the active hash session is written into the hash engine. host read operations from these register locations will return 0s. Note: When getting data from DMA, this register must be programmed before DMA is programmed to start."]
    #[inline(always)]
    pub fn length_in(&self) -> LENGTH_IN_R {
        LENGTH_IN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
LENGTH_IN\\[31:0\\]
Message length registers. The content of these registers is used by the hash engine during the message padding phase of the hash session. The data lines of this registers are directly connected to the interface of the hash engine. For a write operation by the host, these registers should be written with the message length in bits. Final hash operations: The total input data length must be programmed for new hash operations that require finalization (padding). The input data must be provided through the slave or DMA interface. Continued hash operations (finalized): For continued hash operations that require finalization, the total message length must be programmed, including the length of previously hashed data that corresponds to the written input digest. Non-final hash operations: For hash operations that do not require finalization (input data length is multiple of 1024-bits for SHA-512, which is SHA-512 data block size. Or multiple of 512-bits for SHA-256, which is SHA-256 data block size), the length field does not need to be programmed since it is not used by the operation If the message length in bits is below (2^32-1), then only this register needs to be written. The hardware automatically sets HASH_LENGTH_IN_H to 0s in this case. The host may write the length register at any time during the hash session when the HASHIOBUFCTRL.RFD_IN is high. The length register must be written before the last data of the active hash session is written into the hash engine. host read operations from these register locations will return 0s. Note: When getting data from DMA, this register must be programmed before DMA is programmed to start."]
    #[inline(always)]
    #[must_use]
    pub fn length_in(&mut self) -> LENGTH_IN_W<0> {
        LENGTH_IN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HASH Input Length LSB\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashinlenl](index.html) module"]
pub struct HASHINLENL_SPEC;
impl crate::RegisterSpec for HASHINLENL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hashinlenl::R](R) reader structure"]
impl crate::Readable for HASHINLENL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hashinlenl::W](W) writer structure"]
impl crate::Writable for HASHINLENL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HASHINLENL to value 0"]
impl crate::Resettable for HASHINLENL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
