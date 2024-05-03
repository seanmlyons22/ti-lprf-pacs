#[doc = "Register `VDCTL` reader"]
pub type R = crate::R<VdctlSpec>;
#[doc = "Register `VDCTL` writer"]
pub type W = crate::W<VdctlSpec>;
#[doc = "Field `ULDO` reader - 0:0\\]
Request PMCTL to switch to uLDO. 0: No request 1: Assert request when possible The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = x0 3. SECDMACLKGDS.DMA_CLK_EN = 0 and SECDMACLKGR.DMA_AM_CLK_EN = 0 (Note: Settings must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 and SECDMACLKGR.CRYPTO_AM_CLK_EN = 0 (Note: Settings must be loaded with CLKLOADCTL.LOAD) 5. I2SCLKGDS.CLK_EN = 0 and I2SCLKGR.AM_CLK_EN = 0 (Note: Settings must be loaded with CLKLOADCTL.LOAD) 6. RFC do no request access to BUS 7. System CPU in deepsleep"]
pub type UldoR = crate::BitReader;
#[doc = "Field `ULDO` writer - 0:0\\]
Request PMCTL to switch to uLDO. 0: No request 1: Assert request when possible The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = x0 3. SECDMACLKGDS.DMA_CLK_EN = 0 and SECDMACLKGR.DMA_AM_CLK_EN = 0 (Note: Settings must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 and SECDMACLKGR.CRYPTO_AM_CLK_EN = 0 (Note: Settings must be loaded with CLKLOADCTL.LOAD) 5. I2SCLKGDS.CLK_EN = 0 and I2SCLKGR.AM_CLK_EN = 0 (Note: Settings must be loaded with CLKLOADCTL.LOAD) 6. RFC do no request access to BUS 7. System CPU in deepsleep"]
pub type UldoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPARE1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Spare1R = crate::FieldReader<u32>;
#[doc = "Field `SPARE1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Spare1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Request PMCTL to switch to uLDO. 0: No request 1: Assert request when possible The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = x0 3. SECDMACLKGDS.DMA_CLK_EN = 0 and SECDMACLKGR.DMA_AM_CLK_EN = 0 (Note: Settings must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 and SECDMACLKGR.CRYPTO_AM_CLK_EN = 0 (Note: Settings must be loaded with CLKLOADCTL.LOAD) 5. I2SCLKGDS.CLK_EN = 0 and I2SCLKGR.AM_CLK_EN = 0 (Note: Settings must be loaded with CLKLOADCTL.LOAD) 6. RFC do no request access to BUS 7. System CPU in deepsleep"]
    #[inline(always)]
    pub fn uldo(&self) -> UldoR {
        UldoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare1(&self) -> Spare1R {
        Spare1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Request PMCTL to switch to uLDO. 0: No request 1: Assert request when possible The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = x0 3. SECDMACLKGDS.DMA_CLK_EN = 0 and SECDMACLKGR.DMA_AM_CLK_EN = 0 (Note: Settings must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 and SECDMACLKGR.CRYPTO_AM_CLK_EN = 0 (Note: Settings must be loaded with CLKLOADCTL.LOAD) 5. I2SCLKGDS.CLK_EN = 0 and I2SCLKGR.AM_CLK_EN = 0 (Note: Settings must be loaded with CLKLOADCTL.LOAD) 6. RFC do no request access to BUS 7. System CPU in deepsleep"]
    #[inline(always)]
    #[must_use]
    pub fn uldo(&mut self) -> UldoW<VdctlSpec> {
        UldoW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn spare1(&mut self) -> Spare1W<VdctlSpec> {
        Spare1W::new(self, 1)
    }
}
#[doc = "MCU Voltage Domain Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vdctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vdctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VdctlSpec;
impl crate::RegisterSpec for VdctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdctl::R`](R) reader structure"]
impl crate::Readable for VdctlSpec {}
#[doc = "`write(|w| ..)` method takes [`vdctl::W`](W) writer structure"]
impl crate::Writable for VdctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VDCTL to value 0"]
impl crate::Resettable for VdctlSpec {
    const RESET_VALUE: u32 = 0;
}
