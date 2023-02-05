#[doc = "Register `ALGSEL` reader"]
pub struct R(crate::R<ALGSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALGSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALGSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALGSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALGSEL` writer"]
pub struct W(crate::W<ALGSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALGSEL_SPEC>;
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
impl From<crate::W<ALGSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALGSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY_STORE` reader - 0:0\\]
If set to one, selects the Key Store as destination for the DMA The maximum transfer size to DMA engine is set to 32 bytes (however transfers of 16, 24 and 32 bytes are allowed)"]
pub type KEY_STORE_R = crate::BitReader<bool>;
#[doc = "Field `KEY_STORE` writer - 0:0\\]
If set to one, selects the Key Store as destination for the DMA The maximum transfer size to DMA engine is set to 32 bytes (however transfers of 16, 24 and 32 bytes are allowed)"]
pub type KEY_STORE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALGSEL_SPEC, bool, O>;
#[doc = "Field `AES` reader - 1:1\\]
If set to one, selects the AES engine as source/destination for the DMA The read and write maximum transfer size to the DMA engine is set to 16 bytes."]
pub type AES_R = crate::BitReader<bool>;
#[doc = "Field `AES` writer - 1:1\\]
If set to one, selects the AES engine as source/destination for the DMA The read and write maximum transfer size to the DMA engine is set to 16 bytes."]
pub type AES_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALGSEL_SPEC, bool, O>;
#[doc = "Field `HASH_SHA_256` reader - 2:2\\]
If set to one, selects the hash engine in 256B mode as destination for the DMA The maximum transfer size to DMA engine is set to 64 bytes for reading and 32 bytes for writing (the latter is only applicable if the hash result is written out through the DMA)."]
pub type HASH_SHA_256_R = crate::BitReader<bool>;
#[doc = "Field `HASH_SHA_256` writer - 2:2\\]
If set to one, selects the hash engine in 256B mode as destination for the DMA The maximum transfer size to DMA engine is set to 64 bytes for reading and 32 bytes for writing (the latter is only applicable if the hash result is written out through the DMA)."]
pub type HASH_SHA_256_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALGSEL_SPEC, bool, O>;
#[doc = "Field `RESERVED4` reader - 30:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED4` writer - 30:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALGSEL_SPEC, u32, u32, 27, O>;
#[doc = "Field `TAG` reader - 31:31\\]
If this bit is cleared to 0, the DMA operation involves only data. If this bit is set, the DMA operation includes a TAG (Authentication Result / Digest). For SHA-256 operation, a DMA must be set up for both input data and TAG. For any other selected module, setting this bit only allows a DMA that reads the TAG. No data allowed to be transferred to or from the selected module via the DMA."]
pub type TAG_R = crate::BitReader<bool>;
#[doc = "Field `TAG` writer - 31:31\\]
If this bit is cleared to 0, the DMA operation involves only data. If this bit is set, the DMA operation includes a TAG (Authentication Result / Digest). For SHA-256 operation, a DMA must be set up for both input data and TAG. For any other selected module, setting this bit only allows a DMA that reads the TAG. No data allowed to be transferred to or from the selected module via the DMA."]
pub type TAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALGSEL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
If set to one, selects the Key Store as destination for the DMA The maximum transfer size to DMA engine is set to 32 bytes (however transfers of 16, 24 and 32 bytes are allowed)"]
    #[inline(always)]
    pub fn key_store(&self) -> KEY_STORE_R {
        KEY_STORE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
If set to one, selects the AES engine as source/destination for the DMA The read and write maximum transfer size to the DMA engine is set to 16 bytes."]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
If set to one, selects the hash engine in 256B mode as destination for the DMA The maximum transfer size to DMA engine is set to 64 bytes for reading and 32 bytes for writing (the latter is only applicable if the hash result is written out through the DMA)."]
    #[inline(always)]
    pub fn hash_sha_256(&self) -> HASH_SHA_256_R {
        HASH_SHA_256_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:30 - 30:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new((self.bits >> 4) & 0x07ff_ffff)
    }
    #[doc = "Bit 31 - 31:31\\]
If this bit is cleared to 0, the DMA operation involves only data. If this bit is set, the DMA operation includes a TAG (Authentication Result / Digest). For SHA-256 operation, a DMA must be set up for both input data and TAG. For any other selected module, setting this bit only allows a DMA that reads the TAG. No data allowed to be transferred to or from the selected module via the DMA."]
    #[inline(always)]
    pub fn tag(&self) -> TAG_R {
        TAG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
If set to one, selects the Key Store as destination for the DMA The maximum transfer size to DMA engine is set to 32 bytes (however transfers of 16, 24 and 32 bytes are allowed)"]
    #[inline(always)]
    #[must_use]
    pub fn key_store(&mut self) -> KEY_STORE_W<0> {
        KEY_STORE_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
If set to one, selects the AES engine as source/destination for the DMA The read and write maximum transfer size to the DMA engine is set to 16 bytes."]
    #[inline(always)]
    #[must_use]
    pub fn aes(&mut self) -> AES_W<1> {
        AES_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
If set to one, selects the hash engine in 256B mode as destination for the DMA The maximum transfer size to DMA engine is set to 64 bytes for reading and 32 bytes for writing (the latter is only applicable if the hash result is written out through the DMA)."]
    #[inline(always)]
    #[must_use]
    pub fn hash_sha_256(&mut self) -> HASH_SHA_256_W<2> {
        HASH_SHA_256_W::new(self)
    }
    #[doc = "Bits 4:30 - 30:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> RESERVED4_W<4> {
        RESERVED4_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
If this bit is cleared to 0, the DMA operation involves only data. If this bit is set, the DMA operation includes a TAG (Authentication Result / Digest). For SHA-256 operation, a DMA must be set up for both input data and TAG. For any other selected module, setting this bit only allows a DMA that reads the TAG. No data allowed to be transferred to or from the selected module via the DMA."]
    #[inline(always)]
    #[must_use]
    pub fn tag(&mut self) -> TAG_W<31> {
        TAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Algorithm Select This algorithm selection register configures the internal destination of the DMA controller.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [algsel](index.html) module"]
pub struct ALGSEL_SPEC;
impl crate::RegisterSpec for ALGSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [algsel::R](R) reader structure"]
impl crate::Readable for ALGSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [algsel::W](W) writer structure"]
impl crate::Writable for ALGSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALGSEL to value 0"]
impl crate::Resettable for ALGSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
