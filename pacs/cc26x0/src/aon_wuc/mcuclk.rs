#[doc = "Register `MCUCLK` reader"]
pub type R = crate::R<McuclkSpec>;
#[doc = "Register `MCUCLK` writer"]
pub type W = crate::W<McuclkSpec>;
#[doc = "1:0\\]
Controls the clock source for the entire MCU domain while MCU is requesting powerdown. When MCU requests powerdown with SCLK_HF as source, then WUC will switch over to this clock source during powerdown, and automatically switch back to SCLK_HF when MCU is no longer requesting powerdown and system is back in active mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PwrDwnSrc {
    #[doc = "1: Use SCLK_LF in Powerdown"]
    SclkLf = 1,
    #[doc = "0: No clock in Powerdown"]
    None = 0,
}
impl From<PwrDwnSrc> for u8 {
    #[inline(always)]
    fn from(variant: PwrDwnSrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PwrDwnSrc {
    type Ux = u8;
}
impl crate::IsEnum for PwrDwnSrc {}
#[doc = "Field `PWR_DWN_SRC` reader - 1:0\\]
Controls the clock source for the entire MCU domain while MCU is requesting powerdown. When MCU requests powerdown with SCLK_HF as source, then WUC will switch over to this clock source during powerdown, and automatically switch back to SCLK_HF when MCU is no longer requesting powerdown and system is back in active mode."]
pub type PwrDwnSrcR = crate::FieldReader<PwrDwnSrc>;
impl PwrDwnSrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PwrDwnSrc> {
        match self.bits {
            1 => Some(PwrDwnSrc::SclkLf),
            0 => Some(PwrDwnSrc::None),
            _ => None,
        }
    }
    #[doc = "Use SCLK_LF in Powerdown"]
    #[inline(always)]
    pub fn is_sclk_lf(&self) -> bool {
        *self == PwrDwnSrc::SclkLf
    }
    #[doc = "No clock in Powerdown"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PwrDwnSrc::None
    }
}
#[doc = "Field `PWR_DWN_SRC` writer - 1:0\\]
Controls the clock source for the entire MCU domain while MCU is requesting powerdown. When MCU requests powerdown with SCLK_HF as source, then WUC will switch over to this clock source during powerdown, and automatically switch back to SCLK_HF when MCU is no longer requesting powerdown and system is back in active mode."]
pub type PwrDwnSrcW<'a, REG> = crate::FieldWriter<'a, REG, 2, PwrDwnSrc>;
impl<'a, REG> PwrDwnSrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Use SCLK_LF in Powerdown"]
    #[inline(always)]
    pub fn sclk_lf(self) -> &'a mut crate::W<REG> {
        self.variant(PwrDwnSrc::SclkLf)
    }
    #[doc = "No clock in Powerdown"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(PwrDwnSrc::None)
    }
}
#[doc = "Field `RCOSC_HF_CAL_DONE` reader - 2:2\\]
MCU bootcode will set this bit when RCOSC_HF is calibrated. The FLASH can not be used until this bit is set. 1: RCOSC_HF is calibrated to 48 MHz, allowing FLASH to power up. 0: RCOSC_HF is not yet calibrated, ie FLASH must not assume that the SCLK_HF is safe"]
pub type RcoscHfCalDoneR = crate::BitReader;
#[doc = "Field `RCOSC_HF_CAL_DONE` writer - 2:2\\]
MCU bootcode will set this bit when RCOSC_HF is calibrated. The FLASH can not be used until this bit is set. 1: RCOSC_HF is calibrated to 48 MHz, allowing FLASH to power up. 0: RCOSC_HF is not yet calibrated, ie FLASH must not assume that the SCLK_HF is safe"]
pub type RcoscHfCalDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Controls the clock source for the entire MCU domain while MCU is requesting powerdown. When MCU requests powerdown with SCLK_HF as source, then WUC will switch over to this clock source during powerdown, and automatically switch back to SCLK_HF when MCU is no longer requesting powerdown and system is back in active mode."]
    #[inline(always)]
    pub fn pwr_dwn_src(&self) -> PwrDwnSrcR {
        PwrDwnSrcR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
MCU bootcode will set this bit when RCOSC_HF is calibrated. The FLASH can not be used until this bit is set. 1: RCOSC_HF is calibrated to 48 MHz, allowing FLASH to power up. 0: RCOSC_HF is not yet calibrated, ie FLASH must not assume that the SCLK_HF is safe"]
    #[inline(always)]
    pub fn rcosc_hf_cal_done(&self) -> RcoscHfCalDoneR {
        RcoscHfCalDoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Controls the clock source for the entire MCU domain while MCU is requesting powerdown. When MCU requests powerdown with SCLK_HF as source, then WUC will switch over to this clock source during powerdown, and automatically switch back to SCLK_HF when MCU is no longer requesting powerdown and system is back in active mode."]
    #[inline(always)]
    #[must_use]
    pub fn pwr_dwn_src(&mut self) -> PwrDwnSrcW<McuclkSpec> {
        PwrDwnSrcW::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
MCU bootcode will set this bit when RCOSC_HF is calibrated. The FLASH can not be used until this bit is set. 1: RCOSC_HF is calibrated to 48 MHz, allowing FLASH to power up. 0: RCOSC_HF is not yet calibrated, ie FLASH must not assume that the SCLK_HF is safe"]
    #[inline(always)]
    #[must_use]
    pub fn rcosc_hf_cal_done(&mut self) -> RcoscHfCalDoneW<McuclkSpec> {
        RcoscHfCalDoneW::new(self, 2)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<McuclkSpec> {
        Reserved3W::new(self, 3)
    }
}
#[doc = "MCU Clock Management This register contains bitfields related to the MCU clock.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcuclk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcuclk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McuclkSpec;
impl crate::RegisterSpec for McuclkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcuclk::R`](R) reader structure"]
impl crate::Readable for McuclkSpec {}
#[doc = "`write(|w| ..)` method takes [`mcuclk::W`](W) writer structure"]
impl crate::Writable for McuclkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCUCLK to value 0"]
impl crate::Resettable for McuclkSpec {
    const RESET_VALUE: u32 = 0;
}
