#[doc = "Register `ALGSEL` reader"]
pub type R = crate::R<AlgselSpec>;
#[doc = "Register `ALGSEL` writer"]
pub type W = crate::W<AlgselSpec>;
#[doc = "Field `KEY_STORE` reader - 0:0\\]
If set to 1, selects the Key Store to be loaded via DMA. The maximum transfer size to DMA engine is set to 32 bytes (however transfers of 16, 24 and 32 bytes are allowed)"]
pub type KeyStoreR = crate::BitReader;
#[doc = "Field `KEY_STORE` writer - 0:0\\]
If set to 1, selects the Key Store to be loaded via DMA. The maximum transfer size to DMA engine is set to 32 bytes (however transfers of 16, 24 and 32 bytes are allowed)"]
pub type KeyStoreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AES` reader - 1:1\\]
If set to 1, the AES data is loaded via DMA Both Read and Write maximum transfer size to DMA engine is set to 16 bytes"]
pub type AesR = crate::BitReader;
#[doc = "Field `AES` writer - 1:1\\]
If set to 1, the AES data is loaded via DMA Both Read and Write maximum transfer size to DMA engine is set to 16 bytes"]
pub type AesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 30:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED2` writer - 30:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
#[doc = "Field `TAG` reader - 31:31\\]
If this bit is cleared to 0, the DMA operation involves only data. If this bit is set, the DMA operation includes a TAG (Authentication Result / Digest)."]
pub type TagR = crate::BitReader;
#[doc = "Field `TAG` writer - 31:31\\]
If this bit is cleared to 0, the DMA operation involves only data. If this bit is set, the DMA operation includes a TAG (Authentication Result / Digest)."]
pub type TagW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
If set to 1, selects the Key Store to be loaded via DMA. The maximum transfer size to DMA engine is set to 32 bytes (however transfers of 16, 24 and 32 bytes are allowed)"]
    #[inline(always)]
    pub fn key_store(&self) -> KeyStoreR {
        KeyStoreR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
If set to 1, the AES data is loaded via DMA Both Read and Write maximum transfer size to DMA engine is set to 16 bytes"]
    #[inline(always)]
    pub fn aes(&self) -> AesR {
        AesR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:30 - 30:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x1fff_ffff)
    }
    #[doc = "Bit 31 - 31:31\\]
If this bit is cleared to 0, the DMA operation involves only data. If this bit is set, the DMA operation includes a TAG (Authentication Result / Digest)."]
    #[inline(always)]
    pub fn tag(&self) -> TagR {
        TagR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
If set to 1, selects the Key Store to be loaded via DMA. The maximum transfer size to DMA engine is set to 32 bytes (however transfers of 16, 24 and 32 bytes are allowed)"]
    #[inline(always)]
    #[must_use]
    pub fn key_store(&mut self) -> KeyStoreW<AlgselSpec> {
        KeyStoreW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
If set to 1, the AES data is loaded via DMA Both Read and Write maximum transfer size to DMA engine is set to 16 bytes"]
    #[inline(always)]
    #[must_use]
    pub fn aes(&mut self) -> AesW<AlgselSpec> {
        AesW::new(self, 1)
    }
    #[doc = "Bits 2:30 - 30:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<AlgselSpec> {
        Reserved2W::new(self, 2)
    }
    #[doc = "Bit 31 - 31:31\\]
If this bit is cleared to 0, the DMA operation involves only data. If this bit is set, the DMA operation includes a TAG (Authentication Result / Digest)."]
    #[inline(always)]
    #[must_use]
    pub fn tag(&mut self) -> TagW<AlgselSpec> {
        TagW::new(self, 31)
    }
}
#[doc = "Master Algorithm Select This register configures the internal destination of the DMA controller.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`algsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`algsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
