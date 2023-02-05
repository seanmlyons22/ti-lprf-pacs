#[doc = "Register `CLKLOADCTL` reader"]
pub struct R(crate::R<CLKLOADCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKLOADCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKLOADCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKLOADCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKLOADCTL` writer"]
pub struct W(crate::W<CLKLOADCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKLOADCTL_SPEC>;
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
impl From<crate::W<CLKLOADCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKLOADCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOAD` reader - 0:0\\]
0: No action 1: Load settings to CLKCTRL. Bit is HW cleared. Multiple changes to settings may be done before LOAD is written once so all changes takes place at the same time. LOAD can also be done after single setting updates. Registers that needs to be followed by LOAD before settings being applied are: - SYSBUSCLKDIV - CPUCLKDIV - PERBUSCPUCLKDIV - PERDMACLKDIV - PERBUSCPUCLKG - RFCCLKG - VIMSCLKG - SECDMACLKGR - SECDMACLKGS - SECDMACLKGDS - GPIOCLKGR - GPIOCLKGS - GPIOCLKGDS - GPTCLKGR - GPTCLKGS - GPTCLKGDS - GPTCLKDIV - I2CCLKGR - I2CCLKGS - I2CCLKGDS - SSICLKGR - SSICLKGS - SSICLKGDS - UARTCLKGR - UARTCLKGS - UARTCLKGDS - I2SCLKGR - I2SCLKGS - I2SCLKGDS - I2SBCLKSEL - I2SCLKCTL - I2SMCLKDIV - I2SBCLKDIV - I2SWCLKDIV"]
pub type LOAD_R = crate::BitReader<bool>;
#[doc = "Field `LOAD` writer - 0:0\\]
0: No action 1: Load settings to CLKCTRL. Bit is HW cleared. Multiple changes to settings may be done before LOAD is written once so all changes takes place at the same time. LOAD can also be done after single setting updates. Registers that needs to be followed by LOAD before settings being applied are: - SYSBUSCLKDIV - CPUCLKDIV - PERBUSCPUCLKDIV - PERDMACLKDIV - PERBUSCPUCLKG - RFCCLKG - VIMSCLKG - SECDMACLKGR - SECDMACLKGS - SECDMACLKGDS - GPIOCLKGR - GPIOCLKGS - GPIOCLKGDS - GPTCLKGR - GPTCLKGS - GPTCLKGDS - GPTCLKDIV - I2CCLKGR - I2CCLKGS - I2CCLKGDS - SSICLKGR - SSICLKGS - SSICLKGDS - UARTCLKGR - UARTCLKGS - UARTCLKGDS - I2SCLKGR - I2SCLKGS - I2SCLKGDS - I2SBCLKSEL - I2SCLKCTL - I2SMCLKDIV - I2SBCLKDIV - I2SWCLKDIV"]
pub type LOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKLOADCTL_SPEC, bool, O>;
#[doc = "Field `LOAD_DONE` reader - 1:1\\]
Status of LOAD. Will be cleared to 0 when any of the registers requiring a LOAD is written to, and be set to 1 when a LOAD is done. Note that writing no change to a register will result in the LOAD_DONE being cleared. 0 : One or more registers have been write accessed after last LOAD 1 : No registers are write accessed after last LOAD"]
pub type LOAD_DONE_R = crate::BitReader<bool>;
#[doc = "Field `LOAD_DONE` writer - 1:1\\]
Status of LOAD. Will be cleared to 0 when any of the registers requiring a LOAD is written to, and be set to 1 when a LOAD is done. Note that writing no change to a register will result in the LOAD_DONE being cleared. 0 : One or more registers have been write accessed after last LOAD 1 : No registers are write accessed after last LOAD"]
pub type LOAD_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKLOADCTL_SPEC, bool, O>;
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLKLOADCTL_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0: No action 1: Load settings to CLKCTRL. Bit is HW cleared. Multiple changes to settings may be done before LOAD is written once so all changes takes place at the same time. LOAD can also be done after single setting updates. Registers that needs to be followed by LOAD before settings being applied are: - SYSBUSCLKDIV - CPUCLKDIV - PERBUSCPUCLKDIV - PERDMACLKDIV - PERBUSCPUCLKG - RFCCLKG - VIMSCLKG - SECDMACLKGR - SECDMACLKGS - SECDMACLKGDS - GPIOCLKGR - GPIOCLKGS - GPIOCLKGDS - GPTCLKGR - GPTCLKGS - GPTCLKGDS - GPTCLKDIV - I2CCLKGR - I2CCLKGS - I2CCLKGDS - SSICLKGR - SSICLKGS - SSICLKGDS - UARTCLKGR - UARTCLKGS - UARTCLKGDS - I2SCLKGR - I2SCLKGS - I2SCLKGDS - I2SBCLKSEL - I2SCLKCTL - I2SMCLKDIV - I2SBCLKDIV - I2SWCLKDIV"]
    #[inline(always)]
    pub fn load(&self) -> LOAD_R {
        LOAD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Status of LOAD. Will be cleared to 0 when any of the registers requiring a LOAD is written to, and be set to 1 when a LOAD is done. Note that writing no change to a register will result in the LOAD_DONE being cleared. 0 : One or more registers have been write accessed after last LOAD 1 : No registers are write accessed after last LOAD"]
    #[inline(always)]
    pub fn load_done(&self) -> LOAD_DONE_R {
        LOAD_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
0: No action 1: Load settings to CLKCTRL. Bit is HW cleared. Multiple changes to settings may be done before LOAD is written once so all changes takes place at the same time. LOAD can also be done after single setting updates. Registers that needs to be followed by LOAD before settings being applied are: - SYSBUSCLKDIV - CPUCLKDIV - PERBUSCPUCLKDIV - PERDMACLKDIV - PERBUSCPUCLKG - RFCCLKG - VIMSCLKG - SECDMACLKGR - SECDMACLKGS - SECDMACLKGDS - GPIOCLKGR - GPIOCLKGS - GPIOCLKGDS - GPTCLKGR - GPTCLKGS - GPTCLKGDS - GPTCLKDIV - I2CCLKGR - I2CCLKGS - I2CCLKGDS - SSICLKGR - SSICLKGS - SSICLKGDS - UARTCLKGR - UARTCLKGS - UARTCLKGDS - I2SCLKGR - I2SCLKGS - I2SCLKGDS - I2SBCLKSEL - I2SCLKCTL - I2SMCLKDIV - I2SBCLKDIV - I2SWCLKDIV"]
    #[inline(always)]
    #[must_use]
    pub fn load(&mut self) -> LOAD_W<0> {
        LOAD_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Status of LOAD. Will be cleared to 0 when any of the registers requiring a LOAD is written to, and be set to 1 when a LOAD is done. Note that writing no change to a register will result in the LOAD_DONE being cleared. 0 : One or more registers have been write accessed after last LOAD 1 : No registers are write accessed after last LOAD"]
    #[inline(always)]
    #[must_use]
    pub fn load_done(&mut self) -> LOAD_DONE_W<1> {
        LOAD_DONE_W::new(self)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Load PRCM Settings To CLKCTRL Power Domain\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkloadctl](index.html) module"]
pub struct CLKLOADCTL_SPEC;
impl crate::RegisterSpec for CLKLOADCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkloadctl::R](R) reader structure"]
impl crate::Readable for CLKLOADCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkloadctl::W](W) writer structure"]
impl crate::Writable for CLKLOADCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKLOADCTL to value 0x02"]
impl crate::Resettable for CLKLOADCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
