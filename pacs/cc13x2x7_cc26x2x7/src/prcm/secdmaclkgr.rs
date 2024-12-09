#[doc = "Register `SECDMACLKGR` reader"]
pub type R = crate::R<SecdmaclkgrSpec>;
#[doc = "Register `SECDMACLKGR` writer"]
pub type W = crate::W<SecdmaclkgrSpec>;
#[doc = "Field `CRYPTO_CLK_EN` reader - 0:0\\]
0: Disable clock 1: Enable clock Can be forced on by CRYPTO_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type CryptoClkEnR = crate::BitReader;
#[doc = "Field `CRYPTO_CLK_EN` writer - 0:0\\]
0: Disable clock 1: Enable clock Can be forced on by CRYPTO_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type CryptoClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRNG_CLK_EN` reader - 1:1\\]
0: Disable clock 1: Enable clock Can be forced on by TRNG_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type TrngClkEnR = crate::BitReader;
#[doc = "Field `TRNG_CLK_EN` writer - 1:1\\]
0: Disable clock 1: Enable clock Can be forced on by TRNG_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type TrngClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKA_CLK_EN` reader - 2:2\\]
0: Disable clock 1: Enable clock Can be forced on by PKA_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type PkaClkEnR = crate::BitReader;
#[doc = "Field `PKA_CLK_EN` writer - 2:2\\]
0: Disable clock 1: Enable clock Can be forced on by PKA_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type PkaClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `DMA_CLK_EN` reader - 8:8\\]
0: Disable clock 1: Enable clock Can be forced on by DMA_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type DmaClkEnR = crate::BitReader;
#[doc = "Field `DMA_CLK_EN` writer - 8:8\\]
0: Disable clock 1: Enable clock Can be forced on by DMA_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type DmaClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED9` reader - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader;
#[doc = "Field `CRYPTO_AM_CLK_EN` reader - 16:16\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CRYPTO_CLK_EN, SECDMACLKGS.CRYPTO_CLK_EN and SECDMACLKGDS.CRYPTO_CLK_EN when enabled. SYSBUS clock will always run when enabled For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type CryptoAmClkEnR = crate::BitReader;
#[doc = "Field `CRYPTO_AM_CLK_EN` writer - 16:16\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CRYPTO_CLK_EN, SECDMACLKGS.CRYPTO_CLK_EN and SECDMACLKGDS.CRYPTO_CLK_EN when enabled. SYSBUS clock will always run when enabled For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type CryptoAmClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRNG_AM_CLK_EN` reader - 17:17\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides TRNG_CLK_EN, SECDMACLKGS.TRNG_CLK_EN and SECDMACLKGDS.TRNG_CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type TrngAmClkEnR = crate::BitReader;
#[doc = "Field `TRNG_AM_CLK_EN` writer - 17:17\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides TRNG_CLK_EN, SECDMACLKGS.TRNG_CLK_EN and SECDMACLKGDS.TRNG_CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type TrngAmClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKA_AM_CLK_EN` reader - 18:18\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides PKA_CLK_EN, SECDMACLKGS.PKA_CLK_EN and SECDMACLKGDS.PKA_CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type PkaAmClkEnR = crate::BitReader;
#[doc = "Field `PKA_AM_CLK_EN` writer - 18:18\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides PKA_CLK_EN, SECDMACLKGS.PKA_CLK_EN and SECDMACLKGDS.PKA_CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type PkaAmClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKA_ZERIOZE_RESET_N` reader - 19:19\\]
Zeroization logic hardware reset. 0: pka_zeroize logic inactive. 1: pka_zeroize of memory is enabled. This register must remain active until the memory are completely zeroized which requires 256 periods on systembus clock."]
pub type PkaZeriozeResetNR = crate::BitReader;
#[doc = "Field `PKA_ZERIOZE_RESET_N` writer - 19:19\\]
Zeroization logic hardware reset. 0: pka_zeroize logic inactive. 1: pka_zeroize of memory is enabled. This register must remain active until the memory are completely zeroized which requires 256 periods on systembus clock."]
pub type PkaZeriozeResetNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED20` reader - 23:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved20R = crate::FieldReader;
#[doc = "Field `DMA_AM_CLK_EN` reader - 24:24\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides DMA_CLK_EN, SECDMACLKGS.DMA_CLK_EN and SECDMACLKGDS.DMA_CLK_EN when enabled. SYSBUS clock will always run when enabled For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type DmaAmClkEnR = crate::BitReader;
#[doc = "Field `DMA_AM_CLK_EN` writer - 24:24\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides DMA_CLK_EN, SECDMACLKGS.DMA_CLK_EN and SECDMACLKGDS.DMA_CLK_EN when enabled. SYSBUS clock will always run when enabled For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type DmaAmClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED25` reader - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved25R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0: Disable clock 1: Enable clock Can be forced on by CRYPTO_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn crypto_clk_en(&self) -> CryptoClkEnR {
        CryptoClkEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Disable clock 1: Enable clock Can be forced on by TRNG_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn trng_clk_en(&self) -> TrngClkEnR {
        TrngClkEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
0: Disable clock 1: Enable clock Can be forced on by PKA_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
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
0: Disable clock 1: Enable clock Can be forced on by DMA_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn dma_clk_en(&self) -> DmaClkEnR {
        DmaClkEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CRYPTO_CLK_EN, SECDMACLKGS.CRYPTO_CLK_EN and SECDMACLKGDS.CRYPTO_CLK_EN when enabled. SYSBUS clock will always run when enabled For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn crypto_am_clk_en(&self) -> CryptoAmClkEnR {
        CryptoAmClkEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides TRNG_CLK_EN, SECDMACLKGS.TRNG_CLK_EN and SECDMACLKGDS.TRNG_CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn trng_am_clk_en(&self) -> TrngAmClkEnR {
        TrngAmClkEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides PKA_CLK_EN, SECDMACLKGS.PKA_CLK_EN and SECDMACLKGDS.PKA_CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn pka_am_clk_en(&self) -> PkaAmClkEnR {
        PkaAmClkEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Zeroization logic hardware reset. 0: pka_zeroize logic inactive. 1: pka_zeroize of memory is enabled. This register must remain active until the memory are completely zeroized which requires 256 periods on systembus clock."]
    #[inline(always)]
    pub fn pka_zerioze_reset_n(&self) -> PkaZeriozeResetNR {
        PkaZeriozeResetNR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved20(&self) -> Reserved20R {
        Reserved20R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides DMA_CLK_EN, SECDMACLKGS.DMA_CLK_EN and SECDMACLKGDS.DMA_CLK_EN when enabled. SYSBUS clock will always run when enabled For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn dma_am_clk_en(&self) -> DmaAmClkEnR {
        DmaAmClkEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved25(&self) -> Reserved25R {
        Reserved25R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
0: Disable clock 1: Enable clock Can be forced on by CRYPTO_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn crypto_clk_en(&mut self) -> CryptoClkEnW<SecdmaclkgrSpec> {
        CryptoClkEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Disable clock 1: Enable clock Can be forced on by TRNG_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn trng_clk_en(&mut self) -> TrngClkEnW<SecdmaclkgrSpec> {
        TrngClkEnW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
0: Disable clock 1: Enable clock Can be forced on by PKA_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn pka_clk_en(&mut self) -> PkaClkEnW<SecdmaclkgrSpec> {
        PkaClkEnW::new(self, 2)
    }
    #[doc = "Bit 8 - 8:8\\]
0: Disable clock 1: Enable clock Can be forced on by DMA_AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn dma_clk_en(&mut self) -> DmaClkEnW<SecdmaclkgrSpec> {
        DmaClkEnW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CRYPTO_CLK_EN, SECDMACLKGS.CRYPTO_CLK_EN and SECDMACLKGDS.CRYPTO_CLK_EN when enabled. SYSBUS clock will always run when enabled For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn crypto_am_clk_en(&mut self) -> CryptoAmClkEnW<SecdmaclkgrSpec> {
        CryptoAmClkEnW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides TRNG_CLK_EN, SECDMACLKGS.TRNG_CLK_EN and SECDMACLKGDS.TRNG_CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn trng_am_clk_en(&mut self) -> TrngAmClkEnW<SecdmaclkgrSpec> {
        TrngAmClkEnW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides PKA_CLK_EN, SECDMACLKGS.PKA_CLK_EN and SECDMACLKGDS.PKA_CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn pka_am_clk_en(&mut self) -> PkaAmClkEnW<SecdmaclkgrSpec> {
        PkaAmClkEnW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Zeroization logic hardware reset. 0: pka_zeroize logic inactive. 1: pka_zeroize of memory is enabled. This register must remain active until the memory are completely zeroized which requires 256 periods on systembus clock."]
    #[inline(always)]
    #[must_use]
    pub fn pka_zerioze_reset_n(&mut self) -> PkaZeriozeResetNW<SecdmaclkgrSpec> {
        PkaZeriozeResetNW::new(self, 19)
    }
    #[doc = "Bit 24 - 24:24\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides DMA_CLK_EN, SECDMACLKGS.DMA_CLK_EN and SECDMACLKGDS.DMA_CLK_EN when enabled. SYSBUS clock will always run when enabled For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn dma_am_clk_en(&mut self) -> DmaAmClkEnW<SecdmaclkgrSpec> {
        DmaAmClkEnW::new(self, 24)
    }
}
#[doc = "SEC (PKA And TRNG And CRYPTO) And UDMA Clock Gate For Run And All Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secdmaclkgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secdmaclkgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecdmaclkgrSpec;
impl crate::RegisterSpec for SecdmaclkgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secdmaclkgr::R`](R) reader structure"]
impl crate::Readable for SecdmaclkgrSpec {}
#[doc = "`write(|w| ..)` method takes [`secdmaclkgr::W`](W) writer structure"]
impl crate::Writable for SecdmaclkgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECDMACLKGR to value 0"]
impl crate::Resettable for SecdmaclkgrSpec {
    const RESET_VALUE: u32 = 0;
}
