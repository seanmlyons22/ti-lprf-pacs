#[doc = "Register `SECDMACLKGDS` reader"]
pub type R = crate::R<SecdmaclkgdsSpec>;
#[doc = "Register `SECDMACLKGDS` writer"]
pub type W = crate::W<SecdmaclkgdsSpec>;
#[doc = "Field `CRYPTO_CLK_EN` reader - 0:0\\]
0: Disable clock 1: Enable clock SYSBUS clock will always run when enabled Can be forced on by SECDMACLKGR.CRYPTO_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type CryptoClkEnR = crate::BitReader;
#[doc = "Field `CRYPTO_CLK_EN` writer - 0:0\\]
0: Disable clock 1: Enable clock SYSBUS clock will always run when enabled Can be forced on by SECDMACLKGR.CRYPTO_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type CryptoClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRNG_CLK_EN` reader - 1:1\\]
0: Disable clock 1: Enable clock SYSBUS clock will always run when enabled Can be forced on by SECDMACLKGR.TRNG_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type TrngClkEnR = crate::BitReader;
#[doc = "Field `TRNG_CLK_EN` writer - 1:1\\]
0: Disable clock 1: Enable clock SYSBUS clock will always run when enabled Can be forced on by SECDMACLKGR.TRNG_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type TrngClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `RESERVED2` writer - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DMA_CLK_EN` reader - 8:8\\]
0: Disable clock 1: Enable clock Can be forced on by SECDMACLKGR.DMA_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type DmaClkEnR = crate::BitReader;
#[doc = "Field `DMA_CLK_EN` writer - 8:8\\]
0: Disable clock 1: Enable clock Can be forced on by SECDMACLKGR.DMA_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type DmaClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED9` reader - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0: Disable clock 1: Enable clock SYSBUS clock will always run when enabled Can be forced on by SECDMACLKGR.CRYPTO_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn crypto_clk_en(&self) -> CryptoClkEnR {
        CryptoClkEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Disable clock 1: Enable clock SYSBUS clock will always run when enabled Can be forced on by SECDMACLKGR.TRNG_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn trng_clk_en(&self) -> TrngClkEnR {
        TrngClkEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 0x3f) as u8)
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
0: Disable clock 1: Enable clock SYSBUS clock will always run when enabled Can be forced on by SECDMACLKGR.CRYPTO_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn crypto_clk_en(&mut self) -> CryptoClkEnW<SecdmaclkgdsSpec> {
        CryptoClkEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Disable clock 1: Enable clock SYSBUS clock will always run when enabled Can be forced on by SECDMACLKGR.TRNG_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn trng_clk_en(&mut self) -> TrngClkEnW<SecdmaclkgdsSpec> {
        TrngClkEnW::new(self, 1)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<SecdmaclkgdsSpec> {
        Reserved2W::new(self, 2)
    }
    #[doc = "Bit 8 - 8:8\\]
0: Disable clock 1: Enable clock Can be forced on by SECDMACLKGR.DMA_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn dma_clk_en(&mut self) -> DmaClkEnW<SecdmaclkgdsSpec> {
        DmaClkEnW::new(self, 8)
    }
}
#[doc = "SEC (TRNG and CRYPTO) And UDMA Clock Gate For Deep Sleep Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secdmaclkgds::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secdmaclkgds::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecdmaclkgdsSpec;
impl crate::RegisterSpec for SecdmaclkgdsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secdmaclkgds::R`](R) reader structure"]
impl crate::Readable for SecdmaclkgdsSpec {}
#[doc = "`write(|w| ..)` method takes [`secdmaclkgds::W`](W) writer structure"]
impl crate::Writable for SecdmaclkgdsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECDMACLKGDS to value 0"]
impl crate::Resettable for SecdmaclkgdsSpec {
    const RESET_VALUE: u32 = 0;
}
