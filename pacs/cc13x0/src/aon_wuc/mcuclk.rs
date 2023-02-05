#[doc = "Register `MCUCLK` reader"]
pub struct R(crate::R<MCUCLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCUCLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCUCLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCUCLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCUCLK` writer"]
pub struct W(crate::W<MCUCLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCUCLK_SPEC>;
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
impl From<crate::W<MCUCLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCUCLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWR_DWN_SRC` reader - 1:0\\]
Controls the clock source for the entire MCU domain while MCU is requesting powerdown. When MCU requests powerdown with SCLK_HF as source, then WUC will switch over to this clock source during powerdown, and automatically switch back to SCLK_HF when MCU is no longer requesting powerdown and system is back in active mode."]
pub type PWR_DWN_SRC_R = crate::FieldReader<u8, PWR_DWN_SRC_A>;
#[doc = "1:0\\]
Controls the clock source for the entire MCU domain while MCU is requesting powerdown. When MCU requests powerdown with SCLK_HF as source, then WUC will switch over to this clock source during powerdown, and automatically switch back to SCLK_HF when MCU is no longer requesting powerdown and system is back in active mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWR_DWN_SRC_A {
    #[doc = "1: Use SCLK_LF in Powerdown"]
    SCLK_LF = 1,
    #[doc = "0: No clock in Powerdown"]
    NONE = 0,
}
impl From<PWR_DWN_SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: PWR_DWN_SRC_A) -> Self {
        variant as _
    }
}
impl PWR_DWN_SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PWR_DWN_SRC_A> {
        match self.bits {
            1 => Some(PWR_DWN_SRC_A::SCLK_LF),
            0 => Some(PWR_DWN_SRC_A::NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SCLK_LF`"]
    #[inline(always)]
    pub fn is_sclk_lf(&self) -> bool {
        *self == PWR_DWN_SRC_A::SCLK_LF
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PWR_DWN_SRC_A::NONE
    }
}
#[doc = "Field `PWR_DWN_SRC` writer - 1:0\\]
Controls the clock source for the entire MCU domain while MCU is requesting powerdown. When MCU requests powerdown with SCLK_HF as source, then WUC will switch over to this clock source during powerdown, and automatically switch back to SCLK_HF when MCU is no longer requesting powerdown and system is back in active mode."]
pub type PWR_DWN_SRC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MCUCLK_SPEC, u8, PWR_DWN_SRC_A, 2, O>;
impl<'a, const O: u8> PWR_DWN_SRC_W<'a, O> {
    #[doc = "Use SCLK_LF in Powerdown"]
    #[inline(always)]
    pub fn sclk_lf(self) -> &'a mut W {
        self.variant(PWR_DWN_SRC_A::SCLK_LF)
    }
    #[doc = "No clock in Powerdown"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(PWR_DWN_SRC_A::NONE)
    }
}
#[doc = "Field `RCOSC_HF_CAL_DONE` reader - 2:2\\]
MCU bootcode will set this bit when RCOSC_HF is calibrated. The FLASH can not be used until this bit is set. 1: RCOSC_HF is calibrated to 48 MHz, allowing FLASH to power up. 0: RCOSC_HF is not yet calibrated, ie FLASH must not assume that the SCLK_HF is safe"]
pub type RCOSC_HF_CAL_DONE_R = crate::BitReader<bool>;
#[doc = "Field `RCOSC_HF_CAL_DONE` writer - 2:2\\]
MCU bootcode will set this bit when RCOSC_HF is calibrated. The FLASH can not be used until this bit is set. 1: RCOSC_HF is calibrated to 48 MHz, allowing FLASH to power up. 0: RCOSC_HF is not yet calibrated, ie FLASH must not assume that the SCLK_HF is safe"]
pub type RCOSC_HF_CAL_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCUCLK_SPEC, bool, O>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCUCLK_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Controls the clock source for the entire MCU domain while MCU is requesting powerdown. When MCU requests powerdown with SCLK_HF as source, then WUC will switch over to this clock source during powerdown, and automatically switch back to SCLK_HF when MCU is no longer requesting powerdown and system is back in active mode."]
    #[inline(always)]
    pub fn pwr_dwn_src(&self) -> PWR_DWN_SRC_R {
        PWR_DWN_SRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
MCU bootcode will set this bit when RCOSC_HF is calibrated. The FLASH can not be used until this bit is set. 1: RCOSC_HF is calibrated to 48 MHz, allowing FLASH to power up. 0: RCOSC_HF is not yet calibrated, ie FLASH must not assume that the SCLK_HF is safe"]
    #[inline(always)]
    pub fn rcosc_hf_cal_done(&self) -> RCOSC_HF_CAL_DONE_R {
        RCOSC_HF_CAL_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Controls the clock source for the entire MCU domain while MCU is requesting powerdown. When MCU requests powerdown with SCLK_HF as source, then WUC will switch over to this clock source during powerdown, and automatically switch back to SCLK_HF when MCU is no longer requesting powerdown and system is back in active mode."]
    #[inline(always)]
    #[must_use]
    pub fn pwr_dwn_src(&mut self) -> PWR_DWN_SRC_W<0> {
        PWR_DWN_SRC_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
MCU bootcode will set this bit when RCOSC_HF is calibrated. The FLASH can not be used until this bit is set. 1: RCOSC_HF is calibrated to 48 MHz, allowing FLASH to power up. 0: RCOSC_HF is not yet calibrated, ie FLASH must not assume that the SCLK_HF is safe"]
    #[inline(always)]
    #[must_use]
    pub fn rcosc_hf_cal_done(&mut self) -> RCOSC_HF_CAL_DONE_W<2> {
        RCOSC_HF_CAL_DONE_W::new(self)
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
#[doc = "MCU Clock Management This register contains bitfields related to the MCU clock.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcuclk](index.html) module"]
pub struct MCUCLK_SPEC;
impl crate::RegisterSpec for MCUCLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcuclk::R](R) reader structure"]
impl crate::Readable for MCUCLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcuclk::W](W) writer structure"]
impl crate::Writable for MCUCLK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCUCLK to value 0"]
impl crate::Resettable for MCUCLK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
