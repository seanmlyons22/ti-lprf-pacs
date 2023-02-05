#[doc = "Register `SECDMACLKGR` reader"]
pub struct R(crate::R<SECDMACLKGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECDMACLKGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECDMACLKGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECDMACLKGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECDMACLKGR` writer"]
pub struct W(crate::W<SECDMACLKGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECDMACLKGR_SPEC>;
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
impl From<crate::W<SECDMACLKGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECDMACLKGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRYPTO_CLK_EN` reader - 0:0\\]
0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type CRYPTO_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `CRYPTO_CLK_EN` writer - 0:0\\]
0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type CRYPTO_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECDMACLKGR_SPEC, bool, O>;
#[doc = "Field `TRNG_CLK_EN` reader - 1:1\\]
0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type TRNG_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `TRNG_CLK_EN` writer - 1:1\\]
0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type TRNG_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECDMACLKGR_SPEC, bool, O>;
#[doc = "Field `RESERVED2` reader - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED2` writer - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SECDMACLKGR_SPEC, u8, u8, 6, O>;
#[doc = "Field `DMA_CLK_EN` reader - 8:8\\]
0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type DMA_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `DMA_CLK_EN` writer - 8:8\\]
0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type DMA_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECDMACLKGR_SPEC, bool, O>;
#[doc = "Field `RESERVED9` reader - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED9` writer - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SECDMACLKGR_SPEC, u32, u32, 23, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn crypto_clk_en(&self) -> CRYPTO_CLK_EN_R {
        CRYPTO_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn trng_clk_en(&self) -> TRNG_CLK_EN_R {
        TRNG_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn dma_clk_en(&self) -> DMA_CLK_EN_R {
        DMA_CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn crypto_clk_en(&mut self) -> CRYPTO_CLK_EN_W<0> {
        CRYPTO_CLK_EN_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn trng_clk_en(&mut self) -> TRNG_CLK_EN_W<1> {
        TRNG_CLK_EN_W::new(self)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn dma_clk_en(&mut self) -> DMA_CLK_EN_W<8> {
        DMA_CLK_EN_W::new(self)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> RESERVED9_W<9> {
        RESERVED9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TRNG, CRYPTO And UDMA Clock Gate For Run Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secdmaclkgr](index.html) module"]
pub struct SECDMACLKGR_SPEC;
impl crate::RegisterSpec for SECDMACLKGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secdmaclkgr::R](R) reader structure"]
impl crate::Readable for SECDMACLKGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [secdmaclkgr::W](W) writer structure"]
impl crate::Writable for SECDMACLKGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SECDMACLKGR to value 0"]
impl crate::Resettable for SECDMACLKGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
