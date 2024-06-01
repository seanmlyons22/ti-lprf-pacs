#[doc = "Register `SECDMACLKGS` reader"]
pub type R = crate::R<SecdmaclkgsSpec>;
#[doc = "Register `SECDMACLKGS` writer"]
pub type W = crate::W<SecdmaclkgsSpec>;
#[doc = "Field `CRYPTO_CLK_EN` reader - 0:0\\]
0: Disable clock 1: Enable clock Can be forced on by SECDMACLKGR.CRYPTO_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type CryptoClkEnR = crate::BitReader;
#[doc = "Field `CRYPTO_CLK_EN` writer - 0:0\\]
0: Disable clock 1: Enable clock Can be forced on by SECDMACLKGR.CRYPTO_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type CryptoClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRNG_CLK_EN` reader - 1:1\\]
0: Disable clock 1: Enable clock Can be forced on by SECDMACLKGR.TRNG_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type TrngClkEnR = crate::BitReader;
#[doc = "Field `TRNG_CLK_EN` writer - 1:1\\]
0: Disable clock 1: Enable clock Can be forced on by SECDMACLKGR.TRNG_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type TrngClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKA_CLK_EN` reader - 2:2\\]
0: Disable clock 1: Enable clock Can be forced on by SECDMACLKGR.PKA_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type PkaClkEnR = crate::BitReader;
#[doc = "Field `PKA_CLK_EN` writer - 2:2\\]
0: Disable clock 1: Enable clock Can be forced on by SECDMACLKGR.PKA_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type PkaClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `RESERVED3` writer - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DMA_CLK_EN` reader - 8:8\\]
0: Disable clock 1: Enable clock Can be forced on by SECDMACLKGR.DMA_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type DmaClkEnR = crate::BitReader;
#[doc = "Field `DMA_CLK_EN` writer - 8:8\\]
0: Disable clock 1: Enable clock Can be forced on by SECDMACLKGR.DMA_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type DmaClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED9` reader - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED9` writer - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0: Disable clock 1: Enable clock Can be forced on by SECDMACLKGR.CRYPTO_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn crypto_clk_en(&self) -> CryptoClkEnR {
        CryptoClkEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Disable clock 1: Enable clock Can be forced on by SECDMACLKGR.TRNG_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn trng_clk_en(&self) -> TrngClkEnR {
        TrngClkEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
0: Disable clock 1: Enable clock Can be forced on by SECDMACLKGR.PKA_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn pka_clk_en(&self) -> PkaClkEnR {
        PkaClkEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
0: Disable clock 1: Enable clock Can be forced on by SECDMACLKGR.DMA_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn dma_clk_en(&self) -> DmaClkEnR {
        DmaClkEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
0: Disable clock 1: Enable clock Can be forced on by SECDMACLKGR.CRYPTO_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn crypto_clk_en(&mut self) -> CryptoClkEnW<SecdmaclkgsSpec> {
        CryptoClkEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Disable clock 1: Enable clock Can be forced on by SECDMACLKGR.TRNG_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn trng_clk_en(&mut self) -> TrngClkEnW<SecdmaclkgsSpec> {
        TrngClkEnW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
0: Disable clock 1: Enable clock Can be forced on by SECDMACLKGR.PKA_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn pka_clk_en(&mut self) -> PkaClkEnW<SecdmaclkgsSpec> {
        PkaClkEnW::new(self, 2)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<SecdmaclkgsSpec> {
        Reserved3W::new(self, 3)
    }
    #[doc = "Bit 8 - 8:8\\]
0: Disable clock 1: Enable clock Can be forced on by SECDMACLKGR.DMA_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn dma_clk_en(&mut self) -> DmaClkEnW<SecdmaclkgsSpec> {
        DmaClkEnW::new(self, 8)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> Reserved9W<SecdmaclkgsSpec> {
        Reserved9W::new(self, 9)
    }
}
#[doc = "SEC (PKA And TRNG And CRYPTO) And UDMA Clock Gate For Sleep Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secdmaclkgs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secdmaclkgs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecdmaclkgsSpec;
impl crate::RegisterSpec for SecdmaclkgsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secdmaclkgs::R`](R) reader structure"]
impl crate::Readable for SecdmaclkgsSpec {}
#[doc = "`write(|w| ..)` method takes [`secdmaclkgs::W`](W) writer structure"]
impl crate::Writable for SecdmaclkgsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECDMACLKGS to value 0"]
impl crate::Resettable for SecdmaclkgsSpec {
    const RESET_VALUE: u32 = 0;
}
