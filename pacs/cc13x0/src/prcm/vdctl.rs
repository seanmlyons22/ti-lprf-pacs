#[doc = "Register `VDCTL` reader"]
pub type R = crate::R<VdctlSpec>;
#[doc = "Register `VDCTL` writer"]
pub type W = crate::W<VdctlSpec>;
#[doc = "Field `ULDO` reader - 0:0\\]
Request WUC to switch to uLDO. 0: No request 1: Assert request when possible The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = 0 3. SECDMACLKGDS.DMA_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 5. RFC do no request access to BUS 6. System CPU in deepsleep"]
pub type UldoR = crate::BitReader;
#[doc = "Field `ULDO` writer - 0:0\\]
Request WUC to switch to uLDO. 0: No request 1: Assert request when possible The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = 0 3. SECDMACLKGDS.DMA_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 5. RFC do no request access to BUS 6. System CPU in deepsleep"]
pub type UldoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `RESERVED1` writer - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_VD` reader - 2:2\\]
Request WUC to power down the MCU voltage domain 0: No request 1: Assert request when possible. An asserted power down request will result in a boot of the MCU system when powered up again. The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = 0 3. SECDMACLKGDS.DMA_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 5. RFC do no request access to BUS 6. System CPU in deepsleep"]
pub type McuVdR = crate::BitReader;
#[doc = "Field `MCU_VD` writer - 2:2\\]
Request WUC to power down the MCU voltage domain 0: No request 1: Assert request when possible. An asserted power down request will result in a boot of the MCU system when powered up again. The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = 0 3. SECDMACLKGDS.DMA_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 5. RFC do no request access to BUS 6. System CPU in deepsleep"]
pub type McuVdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Request WUC to switch to uLDO. 0: No request 1: Assert request when possible The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = 0 3. SECDMACLKGDS.DMA_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 5. RFC do no request access to BUS 6. System CPU in deepsleep"]
    #[inline(always)]
    pub fn uldo(&self) -> UldoR {
        UldoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Request WUC to power down the MCU voltage domain 0: No request 1: Assert request when possible. An asserted power down request will result in a boot of the MCU system when powered up again. The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = 0 3. SECDMACLKGDS.DMA_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 5. RFC do no request access to BUS 6. System CPU in deepsleep"]
    #[inline(always)]
    pub fn mcu_vd(&self) -> McuVdR {
        McuVdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Request WUC to switch to uLDO. 0: No request 1: Assert request when possible The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = 0 3. SECDMACLKGDS.DMA_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 5. RFC do no request access to BUS 6. System CPU in deepsleep"]
    #[inline(always)]
    #[must_use]
    pub fn uldo(&mut self) -> UldoW<VdctlSpec> {
        UldoW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<VdctlSpec> {
        Reserved1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Request WUC to power down the MCU voltage domain 0: No request 1: Assert request when possible. An asserted power down request will result in a boot of the MCU system when powered up again. The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = 0 3. SECDMACLKGDS.DMA_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 5. RFC do no request access to BUS 6. System CPU in deepsleep"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_vd(&mut self) -> McuVdW<VdctlSpec> {
        McuVdW::new(self, 2)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<VdctlSpec> {
        Reserved3W::new(self, 3)
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
