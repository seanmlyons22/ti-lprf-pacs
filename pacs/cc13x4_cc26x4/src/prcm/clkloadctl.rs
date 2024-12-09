#[doc = "Register `CLKLOADCTL` reader"]
pub type R = crate::R<ClkloadctlSpec>;
#[doc = "Register `CLKLOADCTL` writer"]
pub type W = crate::W<ClkloadctlSpec>;
#[doc = "Field `LOAD` writer - 0:0\\]
0: No action 1: Load settings to CLKCTRL. Bit is HW cleared. Multiple changes to settings may be done before LOAD is written once so all changes takes place at the same time. LOAD can also be done after single setting updates. Registers that needs to be followed by LOAD before settings being applied are: - SYSBUSCLKDIV - CPUCLKDIV - PERBUSCPUCLKDIV - PERDMACLKDIV - PERBUSCPUCLKG - RFCCLKG - VIMSCLKG - SECDMACLKGR - SECDMACLKGS - SECDMACLKGDS - GPIOCLKGR - GPIOCLKGS - GPIOCLKGDS - GPTCLKGR - GPTCLKGS - GPTCLKGDS - GPTCLKDIV - I2CCLKGR - I2CCLKGS - I2CCLKGDS - SPICLKGR - SPICLKGS - SPICLKGDS - UARTCLKGR - UARTCLKGS - UARTCLKGDS - I2SCLKGR - I2SCLKGS - I2SCLKGDS - I2SBCLKSEL - I2SCLKCTL - I2SMCLKDIV - I2SBCLKDIV - I2SWCLKDIV - MCUSRAMCFG"]
pub type LoadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOAD_DONE` reader - 1:1\\]
Status of LOAD. Will be cleared to 0 when any of the registers requiring a LOAD is written to, and be set to 1 when a LOAD is done. Note that writing no change to a register will result in the LOAD_DONE being cleared. 0 : One or more registers have been write accessed after last LOAD 1 : No registers are write accessed after last LOAD"]
pub type LoadDoneR = crate::BitReader;
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 1 - 1:1\\]
Status of LOAD. Will be cleared to 0 when any of the registers requiring a LOAD is written to, and be set to 1 when a LOAD is done. Note that writing no change to a register will result in the LOAD_DONE being cleared. 0 : One or more registers have been write accessed after last LOAD 1 : No registers are write accessed after last LOAD"]
    #[inline(always)]
    pub fn load_done(&self) -> LoadDoneR {
        LoadDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
0: No action 1: Load settings to CLKCTRL. Bit is HW cleared. Multiple changes to settings may be done before LOAD is written once so all changes takes place at the same time. LOAD can also be done after single setting updates. Registers that needs to be followed by LOAD before settings being applied are: - SYSBUSCLKDIV - CPUCLKDIV - PERBUSCPUCLKDIV - PERDMACLKDIV - PERBUSCPUCLKG - RFCCLKG - VIMSCLKG - SECDMACLKGR - SECDMACLKGS - SECDMACLKGDS - GPIOCLKGR - GPIOCLKGS - GPIOCLKGDS - GPTCLKGR - GPTCLKGS - GPTCLKGDS - GPTCLKDIV - I2CCLKGR - I2CCLKGS - I2CCLKGDS - SPICLKGR - SPICLKGS - SPICLKGDS - UARTCLKGR - UARTCLKGS - UARTCLKGDS - I2SCLKGR - I2SCLKGS - I2SCLKGDS - I2SBCLKSEL - I2SCLKCTL - I2SMCLKDIV - I2SBCLKDIV - I2SWCLKDIV - MCUSRAMCFG"]
    #[inline(always)]
    #[must_use]
    pub fn load(&mut self) -> LoadW<ClkloadctlSpec> {
        LoadW::new(self, 0)
    }
}
#[doc = "Load PRCM Settings To CLKCTRL Power Domain\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkloadctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkloadctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkloadctlSpec;
impl crate::RegisterSpec for ClkloadctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkloadctl::R`](R) reader structure"]
impl crate::Readable for ClkloadctlSpec {}
#[doc = "`write(|w| ..)` method takes [`clkloadctl::W`](W) writer structure"]
impl crate::Writable for ClkloadctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKLOADCTL to value 0x02"]
impl crate::Resettable for ClkloadctlSpec {
    const RESET_VALUE: u32 = 0x02;
}
