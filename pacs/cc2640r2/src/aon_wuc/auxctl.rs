#[doc = "Register `AUXCTL` reader"]
pub type R = crate::R<AuxctlSpec>;
#[doc = "Register `AUXCTL` writer"]
pub type W = crate::W<AuxctlSpec>;
#[doc = "Field `AUX_FORCE_ON` reader - 0:0\\]
Forces the AUX domain into active mode, overriding the requests from AUX_WUC:PWROFFREQ, AUX_WUC:PWRDWNREQ and AUX_WUC:MCUBUSCTL. Note that an ongoing AUX_WUC:PWROFFREQ will complete before this bit will set the AUX domain into active mode. MCU must set this bit in order to access the AUX peripherals. The AUX domain status can be read from PWRSTAT.AUX_PD_ON 0: AUX is allowed to Power Off, Power Down or Disconnect. 1: AUX Power OFF, Power Down or Disconnect requests will be overruled"]
pub type AuxForceOnR = crate::BitReader;
#[doc = "Field `AUX_FORCE_ON` writer - 0:0\\]
Forces the AUX domain into active mode, overriding the requests from AUX_WUC:PWROFFREQ, AUX_WUC:PWRDWNREQ and AUX_WUC:MCUBUSCTL. Note that an ongoing AUX_WUC:PWROFFREQ will complete before this bit will set the AUX domain into active mode. MCU must set this bit in order to access the AUX peripherals. The AUX domain status can be read from PWRSTAT.AUX_PD_ON 0: AUX is allowed to Power Off, Power Down or Disconnect. 1: AUX Power OFF, Power Down or Disconnect requests will be overruled"]
pub type AuxForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWEV` reader - 1:1\\]
Writing 1 sets the software event to the AUX domain, which can be read through AUX_WUC:WUEVFLAGS.AON_SW. This event is normally cleared by AUX_SCE through the AUX_WUC:WUEVCLR.AON_SW. It can also be cleared by writing 0 to this register. Reading 0 means that there is no outstanding software event for AUX. Note that it can take up to 1,5 SCLK_LF clock cycles to clear the event from AUX."]
pub type SwevR = crate::BitReader;
#[doc = "Field `SWEV` writer - 1:1\\]
Writing 1 sets the software event to the AUX domain, which can be read through AUX_WUC:WUEVFLAGS.AON_SW. This event is normally cleared by AUX_SCE through the AUX_WUC:WUEVCLR.AON_SW. It can also be cleared by writing 0 to this register. Reading 0 means that there is no outstanding software event for AUX. Note that it can take up to 1,5 SCLK_LF clock cycles to clear the event from AUX."]
pub type SwevW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCE_RUN_EN` reader - 2:2\\]
Enables (1) or disables (0) AUX_SCE execution. AUX_SCE execution will begin when AUX Domain is powered and either this or AUX_SCE:CTL.CLK_EN is set. Setting this bit will assure that AUX_SCE execution starts as soon as AUX power domain is woken up. ( AUX_SCE:CTL.CLK_EN will be reset to 0 if AUX power domain has been off) 0: AUX_SCE execution will be disabled if AUX_SCE:CTL.CLK_EN is 0 1: AUX_SCE execution is enabled."]
pub type SceRunEnR = crate::BitReader;
#[doc = "Field `SCE_RUN_EN` writer - 2:2\\]
Enables (1) or disables (0) AUX_SCE execution. AUX_SCE execution will begin when AUX Domain is powered and either this or AUX_SCE:CTL.CLK_EN is set. Setting this bit will assure that AUX_SCE execution starts as soon as AUX power domain is woken up. ( AUX_SCE:CTL.CLK_EN will be reset to 0 if AUX power domain has been off) 0: AUX_SCE execution will be disabled if AUX_SCE:CTL.CLK_EN is 0 1: AUX_SCE execution is enabled."]
pub type SceRunEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 30:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
#[doc = "Field `RESET_REQ` reader - 31:31\\]
Reset request for AUX. Writing 1 to this register will assert reset to AUX. The reset will be held until the bit is cleared again. 0: AUX reset pin will be deasserted 1: AUX reset pin will be asserted"]
pub type ResetReqR = crate::BitReader;
#[doc = "Field `RESET_REQ` writer - 31:31\\]
Reset request for AUX. Writing 1 to this register will assert reset to AUX. The reset will be held until the bit is cleared again. 0: AUX reset pin will be deasserted 1: AUX reset pin will be asserted"]
pub type ResetReqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Forces the AUX domain into active mode, overriding the requests from AUX_WUC:PWROFFREQ, AUX_WUC:PWRDWNREQ and AUX_WUC:MCUBUSCTL. Note that an ongoing AUX_WUC:PWROFFREQ will complete before this bit will set the AUX domain into active mode. MCU must set this bit in order to access the AUX peripherals. The AUX domain status can be read from PWRSTAT.AUX_PD_ON 0: AUX is allowed to Power Off, Power Down or Disconnect. 1: AUX Power OFF, Power Down or Disconnect requests will be overruled"]
    #[inline(always)]
    pub fn aux_force_on(&self) -> AuxForceOnR {
        AuxForceOnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 1 sets the software event to the AUX domain, which can be read through AUX_WUC:WUEVFLAGS.AON_SW. This event is normally cleared by AUX_SCE through the AUX_WUC:WUEVCLR.AON_SW. It can also be cleared by writing 0 to this register. Reading 0 means that there is no outstanding software event for AUX. Note that it can take up to 1,5 SCLK_LF clock cycles to clear the event from AUX."]
    #[inline(always)]
    pub fn swev(&self) -> SwevR {
        SwevR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enables (1) or disables (0) AUX_SCE execution. AUX_SCE execution will begin when AUX Domain is powered and either this or AUX_SCE:CTL.CLK_EN is set. Setting this bit will assure that AUX_SCE execution starts as soon as AUX power domain is woken up. ( AUX_SCE:CTL.CLK_EN will be reset to 0 if AUX power domain has been off) 0: AUX_SCE execution will be disabled if AUX_SCE:CTL.CLK_EN is 0 1: AUX_SCE execution is enabled."]
    #[inline(always)]
    pub fn sce_run_en(&self) -> SceRunEnR {
        SceRunEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:30 - 30:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x0fff_ffff)
    }
    #[doc = "Bit 31 - 31:31\\]
Reset request for AUX. Writing 1 to this register will assert reset to AUX. The reset will be held until the bit is cleared again. 0: AUX reset pin will be deasserted 1: AUX reset pin will be asserted"]
    #[inline(always)]
    pub fn reset_req(&self) -> ResetReqR {
        ResetReqR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Forces the AUX domain into active mode, overriding the requests from AUX_WUC:PWROFFREQ, AUX_WUC:PWRDWNREQ and AUX_WUC:MCUBUSCTL. Note that an ongoing AUX_WUC:PWROFFREQ will complete before this bit will set the AUX domain into active mode. MCU must set this bit in order to access the AUX peripherals. The AUX domain status can be read from PWRSTAT.AUX_PD_ON 0: AUX is allowed to Power Off, Power Down or Disconnect. 1: AUX Power OFF, Power Down or Disconnect requests will be overruled"]
    #[inline(always)]
    #[must_use]
    pub fn aux_force_on(&mut self) -> AuxForceOnW<AuxctlSpec> {
        AuxForceOnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 1 sets the software event to the AUX domain, which can be read through AUX_WUC:WUEVFLAGS.AON_SW. This event is normally cleared by AUX_SCE through the AUX_WUC:WUEVCLR.AON_SW. It can also be cleared by writing 0 to this register. Reading 0 means that there is no outstanding software event for AUX. Note that it can take up to 1,5 SCLK_LF clock cycles to clear the event from AUX."]
    #[inline(always)]
    #[must_use]
    pub fn swev(&mut self) -> SwevW<AuxctlSpec> {
        SwevW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Enables (1) or disables (0) AUX_SCE execution. AUX_SCE execution will begin when AUX Domain is powered and either this or AUX_SCE:CTL.CLK_EN is set. Setting this bit will assure that AUX_SCE execution starts as soon as AUX power domain is woken up. ( AUX_SCE:CTL.CLK_EN will be reset to 0 if AUX power domain has been off) 0: AUX_SCE execution will be disabled if AUX_SCE:CTL.CLK_EN is 0 1: AUX_SCE execution is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn sce_run_en(&mut self) -> SceRunEnW<AuxctlSpec> {
        SceRunEnW::new(self, 2)
    }
    #[doc = "Bit 31 - 31:31\\]
Reset request for AUX. Writing 1 to this register will assert reset to AUX. The reset will be held until the bit is cleared again. 0: AUX reset pin will be deasserted 1: AUX reset pin will be asserted"]
    #[inline(always)]
    #[must_use]
    pub fn reset_req(&mut self) -> ResetReqW<AuxctlSpec> {
        ResetReqW::new(self, 31)
    }
}
#[doc = "AUX Control This register contains events and control signals for the AUX domain.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auxctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`auxctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AuxctlSpec;
impl crate::RegisterSpec for AuxctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`auxctl::R`](R) reader structure"]
impl crate::Readable for AuxctlSpec {}
#[doc = "`write(|w| ..)` method takes [`auxctl::W`](W) writer structure"]
impl crate::Writable for AuxctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUXCTL to value 0"]
impl crate::Resettable for AuxctlSpec {
    const RESET_VALUE: u32 = 0;
}
