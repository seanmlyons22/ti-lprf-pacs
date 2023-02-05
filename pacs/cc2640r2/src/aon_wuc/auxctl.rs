#[doc = "Register `AUXCTL` reader"]
pub struct R(crate::R<AUXCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUXCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUXCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUXCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUXCTL` writer"]
pub struct W(crate::W<AUXCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUXCTL_SPEC>;
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
impl From<crate::W<AUXCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUXCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUX_FORCE_ON` reader - 0:0\\]
Forces the AUX domain into active mode, overriding the requests from AUX_WUC:PWROFFREQ, AUX_WUC:PWRDWNREQ and AUX_WUC:MCUBUSCTL. Note that an ongoing AUX_WUC:PWROFFREQ will complete before this bit will set the AUX domain into active mode. MCU must set this bit in order to access the AUX peripherals. The AUX domain status can be read from PWRSTAT.AUX_PD_ON 0: AUX is allowed to Power Off, Power Down or Disconnect. 1: AUX Power OFF, Power Down or Disconnect requests will be overruled"]
pub type AUX_FORCE_ON_R = crate::BitReader<bool>;
#[doc = "Field `AUX_FORCE_ON` writer - 0:0\\]
Forces the AUX domain into active mode, overriding the requests from AUX_WUC:PWROFFREQ, AUX_WUC:PWRDWNREQ and AUX_WUC:MCUBUSCTL. Note that an ongoing AUX_WUC:PWROFFREQ will complete before this bit will set the AUX domain into active mode. MCU must set this bit in order to access the AUX peripherals. The AUX domain status can be read from PWRSTAT.AUX_PD_ON 0: AUX is allowed to Power Off, Power Down or Disconnect. 1: AUX Power OFF, Power Down or Disconnect requests will be overruled"]
pub type AUX_FORCE_ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUXCTL_SPEC, bool, O>;
#[doc = "Field `SWEV` reader - 1:1\\]
Writing 1 sets the software event to the AUX domain, which can be read through AUX_WUC:WUEVFLAGS.AON_SW. This event is normally cleared by AUX_SCE through the AUX_WUC:WUEVCLR.AON_SW. It can also be cleared by writing 0 to this register. Reading 0 means that there is no outstanding software event for AUX. Note that it can take up to 1,5 SCLK_LF clock cycles to clear the event from AUX."]
pub type SWEV_R = crate::BitReader<bool>;
#[doc = "Field `SWEV` writer - 1:1\\]
Writing 1 sets the software event to the AUX domain, which can be read through AUX_WUC:WUEVFLAGS.AON_SW. This event is normally cleared by AUX_SCE through the AUX_WUC:WUEVCLR.AON_SW. It can also be cleared by writing 0 to this register. Reading 0 means that there is no outstanding software event for AUX. Note that it can take up to 1,5 SCLK_LF clock cycles to clear the event from AUX."]
pub type SWEV_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUXCTL_SPEC, bool, O>;
#[doc = "Field `SCE_RUN_EN` reader - 2:2\\]
Enables (1) or disables (0) AUX_SCE execution. AUX_SCE execution will begin when AUX Domain is powered and either this or AUX_SCE:CTL.CLK_EN is set. Setting this bit will assure that AUX_SCE execution starts as soon as AUX power domain is woken up. ( AUX_SCE:CTL.CLK_EN will be reset to 0 if AUX power domain has been off) 0: AUX_SCE execution will be disabled if AUX_SCE:CTL.CLK_EN is 0 1: AUX_SCE execution is enabled."]
pub type SCE_RUN_EN_R = crate::BitReader<bool>;
#[doc = "Field `SCE_RUN_EN` writer - 2:2\\]
Enables (1) or disables (0) AUX_SCE execution. AUX_SCE execution will begin when AUX Domain is powered and either this or AUX_SCE:CTL.CLK_EN is set. Setting this bit will assure that AUX_SCE execution starts as soon as AUX power domain is woken up. ( AUX_SCE:CTL.CLK_EN will be reset to 0 if AUX power domain has been off) 0: AUX_SCE execution will be disabled if AUX_SCE:CTL.CLK_EN is 0 1: AUX_SCE execution is enabled."]
pub type SCE_RUN_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUXCTL_SPEC, bool, O>;
#[doc = "Field `RESERVED3` reader - 30:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED3` writer - 30:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AUXCTL_SPEC, u32, u32, 28, O>;
#[doc = "Field `RESET_REQ` reader - 31:31\\]
Reset request for AUX. Writing 1 to this register will assert reset to AUX. The reset will be held until the bit is cleared again. 0: AUX reset pin will be deasserted 1: AUX reset pin will be asserted"]
pub type RESET_REQ_R = crate::BitReader<bool>;
#[doc = "Field `RESET_REQ` writer - 31:31\\]
Reset request for AUX. Writing 1 to this register will assert reset to AUX. The reset will be held until the bit is cleared again. 0: AUX reset pin will be deasserted 1: AUX reset pin will be asserted"]
pub type RESET_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUXCTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Forces the AUX domain into active mode, overriding the requests from AUX_WUC:PWROFFREQ, AUX_WUC:PWRDWNREQ and AUX_WUC:MCUBUSCTL. Note that an ongoing AUX_WUC:PWROFFREQ will complete before this bit will set the AUX domain into active mode. MCU must set this bit in order to access the AUX peripherals. The AUX domain status can be read from PWRSTAT.AUX_PD_ON 0: AUX is allowed to Power Off, Power Down or Disconnect. 1: AUX Power OFF, Power Down or Disconnect requests will be overruled"]
    #[inline(always)]
    pub fn aux_force_on(&self) -> AUX_FORCE_ON_R {
        AUX_FORCE_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 1 sets the software event to the AUX domain, which can be read through AUX_WUC:WUEVFLAGS.AON_SW. This event is normally cleared by AUX_SCE through the AUX_WUC:WUEVCLR.AON_SW. It can also be cleared by writing 0 to this register. Reading 0 means that there is no outstanding software event for AUX. Note that it can take up to 1,5 SCLK_LF clock cycles to clear the event from AUX."]
    #[inline(always)]
    pub fn swev(&self) -> SWEV_R {
        SWEV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enables (1) or disables (0) AUX_SCE execution. AUX_SCE execution will begin when AUX Domain is powered and either this or AUX_SCE:CTL.CLK_EN is set. Setting this bit will assure that AUX_SCE execution starts as soon as AUX power domain is woken up. ( AUX_SCE:CTL.CLK_EN will be reset to 0 if AUX power domain has been off) 0: AUX_SCE execution will be disabled if AUX_SCE:CTL.CLK_EN is 0 1: AUX_SCE execution is enabled."]
    #[inline(always)]
    pub fn sce_run_en(&self) -> SCE_RUN_EN_R {
        SCE_RUN_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:30 - 30:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new((self.bits >> 3) & 0x0fff_ffff)
    }
    #[doc = "Bit 31 - 31:31\\]
Reset request for AUX. Writing 1 to this register will assert reset to AUX. The reset will be held until the bit is cleared again. 0: AUX reset pin will be deasserted 1: AUX reset pin will be asserted"]
    #[inline(always)]
    pub fn reset_req(&self) -> RESET_REQ_R {
        RESET_REQ_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Forces the AUX domain into active mode, overriding the requests from AUX_WUC:PWROFFREQ, AUX_WUC:PWRDWNREQ and AUX_WUC:MCUBUSCTL. Note that an ongoing AUX_WUC:PWROFFREQ will complete before this bit will set the AUX domain into active mode. MCU must set this bit in order to access the AUX peripherals. The AUX domain status can be read from PWRSTAT.AUX_PD_ON 0: AUX is allowed to Power Off, Power Down or Disconnect. 1: AUX Power OFF, Power Down or Disconnect requests will be overruled"]
    #[inline(always)]
    #[must_use]
    pub fn aux_force_on(&mut self) -> AUX_FORCE_ON_W<0> {
        AUX_FORCE_ON_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 1 sets the software event to the AUX domain, which can be read through AUX_WUC:WUEVFLAGS.AON_SW. This event is normally cleared by AUX_SCE through the AUX_WUC:WUEVCLR.AON_SW. It can also be cleared by writing 0 to this register. Reading 0 means that there is no outstanding software event for AUX. Note that it can take up to 1,5 SCLK_LF clock cycles to clear the event from AUX."]
    #[inline(always)]
    #[must_use]
    pub fn swev(&mut self) -> SWEV_W<1> {
        SWEV_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Enables (1) or disables (0) AUX_SCE execution. AUX_SCE execution will begin when AUX Domain is powered and either this or AUX_SCE:CTL.CLK_EN is set. Setting this bit will assure that AUX_SCE execution starts as soon as AUX power domain is woken up. ( AUX_SCE:CTL.CLK_EN will be reset to 0 if AUX power domain has been off) 0: AUX_SCE execution will be disabled if AUX_SCE:CTL.CLK_EN is 0 1: AUX_SCE execution is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn sce_run_en(&mut self) -> SCE_RUN_EN_W<2> {
        SCE_RUN_EN_W::new(self)
    }
    #[doc = "Bits 3:30 - 30:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<3> {
        RESERVED3_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
Reset request for AUX. Writing 1 to this register will assert reset to AUX. The reset will be held until the bit is cleared again. 0: AUX reset pin will be deasserted 1: AUX reset pin will be asserted"]
    #[inline(always)]
    #[must_use]
    pub fn reset_req(&mut self) -> RESET_REQ_W<31> {
        RESET_REQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AUX Control This register contains events and control signals for the AUX domain.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [auxctl](index.html) module"]
pub struct AUXCTL_SPEC;
impl crate::RegisterSpec for AUXCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [auxctl::R](R) reader structure"]
impl crate::Readable for AUXCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [auxctl::W](W) writer structure"]
impl crate::Writable for AUXCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AUXCTL to value 0"]
impl crate::Resettable for AUXCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
