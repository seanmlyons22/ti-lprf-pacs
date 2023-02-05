#[doc = "Register `VDCTL` reader"]
pub struct R(crate::R<VDCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VDCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VDCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VDCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VDCTL` writer"]
pub struct W(crate::W<VDCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VDCTL_SPEC>;
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
impl From<crate::W<VDCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VDCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ULDO` reader - 0:0\\]
Request WUC to switch to uLDO. 0: No request 1: Assert request when possible The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = 0 3. SECDMACLKGDS.DMA_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 5. RFC do no request access to BUS 6. System CPU in deepsleep"]
pub type ULDO_R = crate::BitReader<bool>;
#[doc = "Field `ULDO` writer - 0:0\\]
Request WUC to switch to uLDO. 0: No request 1: Assert request when possible The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = 0 3. SECDMACLKGDS.DMA_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 5. RFC do no request access to BUS 6. System CPU in deepsleep"]
pub type ULDO_W<'a, const O: u8> = crate::BitWriter<'a, u32, VDCTL_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED1` writer - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::BitWriter<'a, u32, VDCTL_SPEC, bool, O>;
#[doc = "Field `MCU_VD` reader - 2:2\\]
Request WUC to power down the MCU voltage domain 0: No request 1: Assert request when possible. An asserted power down request will result in a boot of the MCU system when powered up again. The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = 0 3. SECDMACLKGDS.DMA_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 5. RFC do no request access to BUS 6. System CPU in deepsleep"]
pub type MCU_VD_R = crate::BitReader<bool>;
#[doc = "Field `MCU_VD` writer - 2:2\\]
Request WUC to power down the MCU voltage domain 0: No request 1: Assert request when possible. An asserted power down request will result in a boot of the MCU system when powered up again. The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = 0 3. SECDMACLKGDS.DMA_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 5. RFC do no request access to BUS 6. System CPU in deepsleep"]
pub type MCU_VD_W<'a, const O: u8> = crate::BitWriter<'a, u32, VDCTL_SPEC, bool, O>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VDCTL_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Request WUC to switch to uLDO. 0: No request 1: Assert request when possible The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = 0 3. SECDMACLKGDS.DMA_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 5. RFC do no request access to BUS 6. System CPU in deepsleep"]
    #[inline(always)]
    pub fn uldo(&self) -> ULDO_R {
        ULDO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Request WUC to power down the MCU voltage domain 0: No request 1: Assert request when possible. An asserted power down request will result in a boot of the MCU system when powered up again. The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = 0 3. SECDMACLKGDS.DMA_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 5. RFC do no request access to BUS 6. System CPU in deepsleep"]
    #[inline(always)]
    pub fn mcu_vd(&self) -> MCU_VD_R {
        MCU_VD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Request WUC to switch to uLDO. 0: No request 1: Assert request when possible The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = 0 3. SECDMACLKGDS.DMA_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 5. RFC do no request access to BUS 6. System CPU in deepsleep"]
    #[inline(always)]
    #[must_use]
    pub fn uldo(&mut self) -> ULDO_W<0> {
        ULDO_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Request WUC to power down the MCU voltage domain 0: No request 1: Assert request when possible. An asserted power down request will result in a boot of the MCU system when powered up again. The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = 0 3. SECDMACLKGDS.DMA_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 5. RFC do no request access to BUS 6. System CPU in deepsleep"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_vd(&mut self) -> MCU_VD_W<2> {
        MCU_VD_W::new(self)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<3> {
        RESERVED3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCU Voltage Domain Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vdctl](index.html) module"]
pub struct VDCTL_SPEC;
impl crate::RegisterSpec for VDCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vdctl::R](R) reader structure"]
impl crate::Readable for VDCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vdctl::W](W) writer structure"]
impl crate::Writable for VDCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VDCTL to value 0"]
impl crate::Resettable for VDCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
