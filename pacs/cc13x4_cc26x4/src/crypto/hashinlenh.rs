#[doc = "Register `HASHINLENH` reader"]
pub type R = crate::R<HashinlenhSpec>;
#[doc = "Register `HASHINLENH` writer"]
pub type W = crate::W<HashinlenhSpec>;
#[doc = "Field `LENGTH_IN` reader - 31:0\\]
LENGTH_IN\\[63:32\\]
Message length registers. The content of these registers is used by the hash engine during the message padding phase of the hash session. The data lines of this registers are directly connected to the interface of the hash engine. For a write operation by the host, these registers should be written with the message length in bits. Final hash operations: The total input data length must be programmed for new hash operations that require finalization (padding). The input data must be provided through the slave or DMA interface. Continued hash operations (finalized): For continued hash operations that require finalization, the total message length must be programmed, including the length of previously hashed data that corresponds to the written input digest. Non-final hash operations: For hash operations that do not require finalization (input data length is multiple of 1024-bits for SHA-512, which is SHA-512 data block size. Or multiple of 512-bits for SHA-256, which is SHA-256 data block size), the length field does not need to be programmed since it is not used by the operation. If the message length in bits is below (2^32-1), then only HASHINLENL needs to be written. The hardware automatically sets HASH_LENGTH_IN_H to 0s in this case. The host may write the length register at any time during the hash session when the HASHIOBUFCTRL.RFD_IN is high. The length register must be written before the last data of the active hash session is written into the hash engine. host read operations from these register locations will return 0s. Note: When getting data from DMA, this register must be programmed before DMA is programmed to start."]
pub type LengthInR = crate::FieldReader<u32>;
#[doc = "Field `LENGTH_IN` writer - 31:0\\]
LENGTH_IN\\[63:32\\]
Message length registers. The content of these registers is used by the hash engine during the message padding phase of the hash session. The data lines of this registers are directly connected to the interface of the hash engine. For a write operation by the host, these registers should be written with the message length in bits. Final hash operations: The total input data length must be programmed for new hash operations that require finalization (padding). The input data must be provided through the slave or DMA interface. Continued hash operations (finalized): For continued hash operations that require finalization, the total message length must be programmed, including the length of previously hashed data that corresponds to the written input digest. Non-final hash operations: For hash operations that do not require finalization (input data length is multiple of 1024-bits for SHA-512, which is SHA-512 data block size. Or multiple of 512-bits for SHA-256, which is SHA-256 data block size), the length field does not need to be programmed since it is not used by the operation. If the message length in bits is below (2^32-1), then only HASHINLENL needs to be written. The hardware automatically sets HASH_LENGTH_IN_H to 0s in this case. The host may write the length register at any time during the hash session when the HASHIOBUFCTRL.RFD_IN is high. The length register must be written before the last data of the active hash session is written into the hash engine. host read operations from these register locations will return 0s. Note: When getting data from DMA, this register must be programmed before DMA is programmed to start."]
pub type LengthInW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
LENGTH_IN\\[63:32\\]
Message length registers. The content of these registers is used by the hash engine during the message padding phase of the hash session. The data lines of this registers are directly connected to the interface of the hash engine. For a write operation by the host, these registers should be written with the message length in bits. Final hash operations: The total input data length must be programmed for new hash operations that require finalization (padding). The input data must be provided through the slave or DMA interface. Continued hash operations (finalized): For continued hash operations that require finalization, the total message length must be programmed, including the length of previously hashed data that corresponds to the written input digest. Non-final hash operations: For hash operations that do not require finalization (input data length is multiple of 1024-bits for SHA-512, which is SHA-512 data block size. Or multiple of 512-bits for SHA-256, which is SHA-256 data block size), the length field does not need to be programmed since it is not used by the operation. If the message length in bits is below (2^32-1), then only HASHINLENL needs to be written. The hardware automatically sets HASH_LENGTH_IN_H to 0s in this case. The host may write the length register at any time during the hash session when the HASHIOBUFCTRL.RFD_IN is high. The length register must be written before the last data of the active hash session is written into the hash engine. host read operations from these register locations will return 0s. Note: When getting data from DMA, this register must be programmed before DMA is programmed to start."]
    #[inline(always)]
    pub fn length_in(&self) -> LengthInR {
        LengthInR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
LENGTH_IN\\[63:32\\]
Message length registers. The content of these registers is used by the hash engine during the message padding phase of the hash session. The data lines of this registers are directly connected to the interface of the hash engine. For a write operation by the host, these registers should be written with the message length in bits. Final hash operations: The total input data length must be programmed for new hash operations that require finalization (padding). The input data must be provided through the slave or DMA interface. Continued hash operations (finalized): For continued hash operations that require finalization, the total message length must be programmed, including the length of previously hashed data that corresponds to the written input digest. Non-final hash operations: For hash operations that do not require finalization (input data length is multiple of 1024-bits for SHA-512, which is SHA-512 data block size. Or multiple of 512-bits for SHA-256, which is SHA-256 data block size), the length field does not need to be programmed since it is not used by the operation. If the message length in bits is below (2^32-1), then only HASHINLENL needs to be written. The hardware automatically sets HASH_LENGTH_IN_H to 0s in this case. The host may write the length register at any time during the hash session when the HASHIOBUFCTRL.RFD_IN is high. The length register must be written before the last data of the active hash session is written into the hash engine. host read operations from these register locations will return 0s. Note: When getting data from DMA, this register must be programmed before DMA is programmed to start."]
    #[inline(always)]
    #[must_use]
    pub fn length_in(&mut self) -> LengthInW<HashinlenhSpec> {
        LengthInW::new(self, 0)
    }
}
#[doc = "HASH Input Length MSB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashinlenh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashinlenh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashinlenhSpec;
impl crate::RegisterSpec for HashinlenhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hashinlenh::R`](R) reader structure"]
impl crate::Readable for HashinlenhSpec {}
#[doc = "`write(|w| ..)` method takes [`hashinlenh::W`](W) writer structure"]
impl crate::Writable for HashinlenhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASHINLENH to value 0"]
impl crate::Resettable for HashinlenhSpec {
    const RESET_VALUE: u32 = 0;
}
