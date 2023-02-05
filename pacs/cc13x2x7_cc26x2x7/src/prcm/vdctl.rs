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
Request PMCTL to switch to uLDO. 0: No request 1: Assert request when possible The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = x0 3. SECDMACLKGDS.DMA_CLK_EN = 0 and S.CRYPTO_CLK_EN\\]
= 0 and SECDMACLKGR.DMA_AM_CLK_EN = 0 (Note: Settings must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 and SECDMACLKGR.CRYPTO_AM_CLK_EN = 0 (Note: Settings must be loaded with CLKLOADCTL.LOAD) 5. I2SCLKGDS.CLK_EN = 0 and I2SCLKGR.AM_CLK_EN = 0 (Note: Settings must be loaded with CLKLOADCTL.LOAD) 6. RFC do no request access to BUS 7. System CPU in deepsleep"]
pub type ULDO_R = crate::BitReader<bool>;
#[doc = "Field `ULDO` writer - 0:0\\]
Request PMCTL to switch to uLDO. 0: No request 1: Assert request when possible The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = x0 3. SECDMACLKGDS.DMA_CLK_EN = 0 and S.CRYPTO_CLK_EN\\]
= 0 and SECDMACLKGR.DMA_AM_CLK_EN = 0 (Note: Settings must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 and SECDMACLKGR.CRYPTO_AM_CLK_EN = 0 (Note: Settings must be loaded with CLKLOADCTL.LOAD) 5. I2SCLKGDS.CLK_EN = 0 and I2SCLKGR.AM_CLK_EN = 0 (Note: Settings must be loaded with CLKLOADCTL.LOAD) 6. RFC do no request access to BUS 7. System CPU in deepsleep"]
pub type ULDO_W<'a, const O: u8> = crate::BitWriter<'a, u32, VDCTL_SPEC, bool, O>;
#[doc = "Field `SPARE1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type SPARE1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SPARE1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type SPARE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VDCTL_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Request PMCTL to switch to uLDO. 0: No request 1: Assert request when possible The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = x0 3. SECDMACLKGDS.DMA_CLK_EN = 0 and S.CRYPTO_CLK_EN\\]
= 0 and SECDMACLKGR.DMA_AM_CLK_EN = 0 (Note: Settings must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 and SECDMACLKGR.CRYPTO_AM_CLK_EN = 0 (Note: Settings must be loaded with CLKLOADCTL.LOAD) 5. I2SCLKGDS.CLK_EN = 0 and I2SCLKGR.AM_CLK_EN = 0 (Note: Settings must be loaded with CLKLOADCTL.LOAD) 6. RFC do no request access to BUS 7. System CPU in deepsleep"]
    #[inline(always)]
    pub fn uldo(&self) -> ULDO_R {
        ULDO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare1(&self) -> SPARE1_R {
        SPARE1_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Request PMCTL to switch to uLDO. 0: No request 1: Assert request when possible The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = x0 3. SECDMACLKGDS.DMA_CLK_EN = 0 and S.CRYPTO_CLK_EN\\]
= 0 and SECDMACLKGR.DMA_AM_CLK_EN = 0 (Note: Settings must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 and SECDMACLKGR.CRYPTO_AM_CLK_EN = 0 (Note: Settings must be loaded with CLKLOADCTL.LOAD) 5. I2SCLKGDS.CLK_EN = 0 and I2SCLKGR.AM_CLK_EN = 0 (Note: Settings must be loaded with CLKLOADCTL.LOAD) 6. RFC do no request access to BUS 7. System CPU in deepsleep"]
    #[inline(always)]
    #[must_use]
    pub fn uldo(&mut self) -> ULDO_W<0> {
        ULDO_W::new(self)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn spare1(&mut self) -> SPARE1_W<1> {
        SPARE1_W::new(self)
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
