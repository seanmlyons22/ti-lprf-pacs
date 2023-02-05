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
If set to 1, selects the Key Store to be loaded via DMA. The maximum transfer size to DMA engine is set to 32 bytes (however transfers of 16, 24 and 32 bytes are allowed)"]
pub type KEY_STORE_R = crate::BitReader<bool>;
#[doc = "Field `KEY_STORE` writer - 0:0\\]
If set to 1, selects the Key Store to be loaded via DMA. The maximum transfer size to DMA engine is set to 32 bytes (however transfers of 16, 24 and 32 bytes are allowed)"]
pub type KEY_STORE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALGSEL_SPEC, bool, O>;
#[doc = "Field `AES` reader - 1:1\\]
If set to 1, the AES data is loaded via DMA Both Read and Write maximum transfer size to DMA engine is set to 16 bytes"]
pub type AES_R = crate::BitReader<bool>;
#[doc = "Field `AES` writer - 1:1\\]
If set to 1, the AES data is loaded via DMA Both Read and Write maximum transfer size to DMA engine is set to 16 bytes"]
pub type AES_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALGSEL_SPEC, bool, O>;
#[doc = "Field `RESERVED2` reader - 30:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED2` writer - 30:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALGSEL_SPEC, u32, u32, 29, O>;
#[doc = "Field `TAG` reader - 31:31\\]
If this bit is cleared to 0, the DMA operation involves only data. If this bit is set, the DMA operation includes a TAG (Authentication Result / Digest)."]
pub type TAG_R = crate::BitReader<bool>;
#[doc = "Field `TAG` writer - 31:31\\]
If this bit is cleared to 0, the DMA operation involves only data. If this bit is set, the DMA operation includes a TAG (Authentication Result / Digest)."]
pub type TAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALGSEL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
If set to 1, selects the Key Store to be loaded via DMA. The maximum transfer size to DMA engine is set to 32 bytes (however transfers of 16, 24 and 32 bytes are allowed)"]
    #[inline(always)]
    pub fn key_store(&self) -> KEY_STORE_R {
        KEY_STORE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
If set to 1, the AES data is loaded via DMA Both Read and Write maximum transfer size to DMA engine is set to 16 bytes"]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:30 - 30:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new((self.bits >> 2) & 0x1fff_ffff)
    }
    #[doc = "Bit 31 - 31:31\\]
If this bit is cleared to 0, the DMA operation involves only data. If this bit is set, the DMA operation includes a TAG (Authentication Result / Digest)."]
    #[inline(always)]
    pub fn tag(&self) -> TAG_R {
        TAG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
If set to 1, selects the Key Store to be loaded via DMA. The maximum transfer size to DMA engine is set to 32 bytes (however transfers of 16, 24 and 32 bytes are allowed)"]
    #[inline(always)]
    #[must_use]
    pub fn key_store(&mut self) -> KEY_STORE_W<0> {
        KEY_STORE_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
If set to 1, the AES data is loaded via DMA Both Read and Write maximum transfer size to DMA engine is set to 16 bytes"]
    #[inline(always)]
    #[must_use]
    pub fn aes(&mut self) -> AES_W<1> {
        AES_W::new(self)
    }
    #[doc = "Bits 2:30 - 30:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
If this bit is cleared to 0, the DMA operation involves only data. If this bit is set, the DMA operation includes a TAG (Authentication Result / Digest)."]
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
#[doc = "Master Algorithm Select This register configures the internal destination of the DMA controller.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [algsel](index.html) module"]
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
