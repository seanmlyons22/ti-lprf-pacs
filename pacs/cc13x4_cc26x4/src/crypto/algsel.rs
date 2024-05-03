#[doc = "Register `ALGSEL` reader"]
pub type R = crate::R<AlgselSpec>;
#[doc = "Register `ALGSEL` writer"]
pub type W = crate::W<AlgselSpec>;
#[doc = "Field `KEY_STORE` reader - 0:0\\]
If set to one, selects the Key Store as destination for the DMA The maximum transfer size to DMA engine is set to 32 bytes (however transfers of 16, 24 and 32 bytes are allowed)"]
pub type KeyStoreR = crate::BitReader;
#[doc = "Field `KEY_STORE` writer - 0:0\\]
If set to one, selects the Key Store as destination for the DMA The maximum transfer size to DMA engine is set to 32 bytes (however transfers of 16, 24 and 32 bytes are allowed)"]
pub type KeyStoreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AES` reader - 1:1\\]
If set to one, selects the AES engine as source/destination for the DMA The read and write maximum transfer size to the DMA engine is set to 16 bytes."]
pub type AesR = crate::BitReader;
#[doc = "Field `AES` writer - 1:1\\]
If set to one, selects the AES engine as source/destination for the DMA The read and write maximum transfer size to the DMA engine is set to 16 bytes."]
pub type AesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASH_SHA_256` reader - 2:2\\]
If set to one, selects the hash engine in 256B mode as destination for the DMA The maximum transfer size to DMA engine is set to 64 bytes for reading and 32 bytes for writing (the latter is only applicable if the hash result is written out through the DMA)."]
pub type HashSha256R = crate::BitReader;
#[doc = "Field `HASH_SHA_256` writer - 2:2\\]
If set to one, selects the hash engine in 256B mode as destination for the DMA The maximum transfer size to DMA engine is set to 64 bytes for reading and 32 bytes for writing (the latter is only applicable if the hash result is written out through the DMA)."]
pub type HashSha256W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASH_SHA_512` reader - 3:3\\]
If set to one, selects the Hash engine as destination for the DMA. The maximum transfer size to DMA engine is set to 128 bytes for reading and 64 bytes for writing (the latter is only applicable if the hash result is written out via DMA)."]
pub type HashSha512R = crate::BitReader;
#[doc = "Field `HASH_SHA_512` writer - 3:3\\]
If set to one, selects the Hash engine as destination for the DMA. The maximum transfer size to DMA engine is set to 128 bytes for reading and 64 bytes for writing (the latter is only applicable if the hash result is written out via DMA)."]
pub type HashSha512W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED4` reader - 30:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED4` writer - 30:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
#[doc = "Field `TAG` reader - 31:31\\]
If this bit is cleared to 0, the DMA operation involves only data. If this bit is set, the DMA operation includes a TAG (Authentication Result / Digest). For SHA-2 operation, a DMA must be set up for both input data and TAG. For any other selected module, setting this bit only allows a DMA that reads the TAG. No data allowed to be transferred to or from the selected module via the DMA."]
pub type TagR = crate::BitReader;
#[doc = "Field `TAG` writer - 31:31\\]
If this bit is cleared to 0, the DMA operation involves only data. If this bit is set, the DMA operation includes a TAG (Authentication Result / Digest). For SHA-2 operation, a DMA must be set up for both input data and TAG. For any other selected module, setting this bit only allows a DMA that reads the TAG. No data allowed to be transferred to or from the selected module via the DMA."]
pub type TagW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
If set to one, selects the Key Store as destination for the DMA The maximum transfer size to DMA engine is set to 32 bytes (however transfers of 16, 24 and 32 bytes are allowed)"]
    #[inline(always)]
    pub fn key_store(&self) -> KeyStoreR {
        KeyStoreR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
If set to one, selects the AES engine as source/destination for the DMA The read and write maximum transfer size to the DMA engine is set to 16 bytes."]
    #[inline(always)]
    pub fn aes(&self) -> AesR {
        AesR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
If set to one, selects the hash engine in 256B mode as destination for the DMA The maximum transfer size to DMA engine is set to 64 bytes for reading and 32 bytes for writing (the latter is only applicable if the hash result is written out through the DMA)."]
    #[inline(always)]
    pub fn hash_sha_256(&self) -> HashSha256R {
        HashSha256R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
If set to one, selects the Hash engine as destination for the DMA. The maximum transfer size to DMA engine is set to 128 bytes for reading and 64 bytes for writing (the latter is only applicable if the hash result is written out via DMA)."]
    #[inline(always)]
    pub fn hash_sha_512(&self) -> HashSha512R {
        HashSha512R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:30 - 30:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new((self.bits >> 4) & 0x07ff_ffff)
    }
    #[doc = "Bit 31 - 31:31\\]
If this bit is cleared to 0, the DMA operation involves only data. If this bit is set, the DMA operation includes a TAG (Authentication Result / Digest). For SHA-2 operation, a DMA must be set up for both input data and TAG. For any other selected module, setting this bit only allows a DMA that reads the TAG. No data allowed to be transferred to or from the selected module via the DMA."]
    #[inline(always)]
    pub fn tag(&self) -> TagR {
        TagR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
If set to one, selects the Key Store as destination for the DMA The maximum transfer size to DMA engine is set to 32 bytes (however transfers of 16, 24 and 32 bytes are allowed)"]
    #[inline(always)]
    #[must_use]
    pub fn key_store(&mut self) -> KeyStoreW<AlgselSpec> {
        KeyStoreW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
If set to one, selects the AES engine as source/destination for the DMA The read and write maximum transfer size to the DMA engine is set to 16 bytes."]
    #[inline(always)]
    #[must_use]
    pub fn aes(&mut self) -> AesW<AlgselSpec> {
        AesW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
If set to one, selects the hash engine in 256B mode as destination for the DMA The maximum transfer size to DMA engine is set to 64 bytes for reading and 32 bytes for writing (the latter is only applicable if the hash result is written out through the DMA)."]
    #[inline(always)]
    #[must_use]
    pub fn hash_sha_256(&mut self) -> HashSha256W<AlgselSpec> {
        HashSha256W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
If set to one, selects the Hash engine as destination for the DMA. The maximum transfer size to DMA engine is set to 128 bytes for reading and 64 bytes for writing (the latter is only applicable if the hash result is written out via DMA)."]
    #[inline(always)]
    #[must_use]
    pub fn hash_sha_512(&mut self) -> HashSha512W<AlgselSpec> {
        HashSha512W::new(self, 3)
    }
    #[doc = "Bits 4:30 - 30:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<AlgselSpec> {
        Reserved4W::new(self, 4)
    }
    #[doc = "Bit 31 - 31:31\\]
If this bit is cleared to 0, the DMA operation involves only data. If this bit is set, the DMA operation includes a TAG (Authentication Result / Digest). For SHA-2 operation, a DMA must be set up for both input data and TAG. For any other selected module, setting this bit only allows a DMA that reads the TAG. No data allowed to be transferred to or from the selected module via the DMA."]
    #[inline(always)]
    #[must_use]
    pub fn tag(&mut self) -> TagW<AlgselSpec> {
        TagW::new(self, 31)
    }
}
#[doc = "Algorithm Select This algorithm selection register configures the internal destination of the DMA controller.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`algsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`algsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlgselSpec;
impl crate::RegisterSpec for AlgselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`algsel::R`](R) reader structure"]
impl crate::Readable for AlgselSpec {}
#[doc = "`write(|w| ..)` method takes [`algsel::W`](W) writer structure"]
impl crate::Writable for AlgselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALGSEL to value 0"]
impl crate::Resettable for AlgselSpec {
    const RESET_VALUE: u32 = 0;
}
