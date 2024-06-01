#[doc = "Register `RADCEXTCFG` reader"]
pub type R = crate::R<RadcextcfgSpec>;
#[doc = "Register `RADCEXTCFG` writer"]
pub type W = crate::W<RadcextcfgSpec>;
#[doc = "Field `RESERVED0` reader - 4:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `RESERVED0` writer - 4:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RADC_MODE_IS_SAR` reader - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type RadcModeIsSarR = crate::BitReader;
#[doc = "Field `RADC_MODE_IS_SAR` writer - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type RadcModeIsSarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RADC_DAC_TH` reader - 11:6\\]
Internal. Only to be used through TI provided API."]
pub type RadcDacThR = crate::FieldReader;
#[doc = "Field `RADC_DAC_TH` writer - 11:6\\]
Internal. Only to be used through TI provided API."]
pub type RadcDacThW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `IDAC_STEP` reader - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type IdacStepR = crate::FieldReader;
#[doc = "Field `IDAC_STEP` writer - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type IdacStepW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPM_IBIAS_WAIT_CNT` reader - 21:16\\]
Internal. Only to be used through TI provided API."]
pub type LpmIbiasWaitCntR = crate::FieldReader;
#[doc = "Field `LPM_IBIAS_WAIT_CNT` writer - 21:16\\]
Internal. Only to be used through TI provided API."]
pub type LpmIbiasWaitCntW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `HPM_IBIAS_WAIT_CNT` reader - 31:22\\]
Internal. Only to be used through TI provided API."]
pub type HpmIbiasWaitCntR = crate::FieldReader<u16>;
#[doc = "Field `HPM_IBIAS_WAIT_CNT` writer - 31:22\\]
Internal. Only to be used through TI provided API."]
pub type HpmIbiasWaitCntW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn radc_mode_is_sar(&self) -> RadcModeIsSarR {
        RadcModeIsSarR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:11 - 11:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn radc_dac_th(&self) -> RadcDacThR {
        RadcDacThR::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn idac_step(&self) -> IdacStepR {
        IdacStepR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpm_ibias_wait_cnt(&self) -> LpmIbiasWaitCntR {
        LpmIbiasWaitCntR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:31 - 31:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hpm_ibias_wait_cnt(&self) -> HpmIbiasWaitCntR {
        HpmIbiasWaitCntR::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<RadcextcfgSpec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn radc_mode_is_sar(&mut self) -> RadcModeIsSarW<RadcextcfgSpec> {
        RadcModeIsSarW::new(self, 5)
    }
    #[doc = "Bits 6:11 - 11:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn radc_dac_th(&mut self) -> RadcDacThW<RadcextcfgSpec> {
        RadcDacThW::new(self, 6)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn idac_step(&mut self) -> IdacStepW<RadcextcfgSpec> {
        IdacStepW::new(self, 12)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn lpm_ibias_wait_cnt(&mut self) -> LpmIbiasWaitCntW<RadcextcfgSpec> {
        LpmIbiasWaitCntW::new(self, 16)
    }
    #[doc = "Bits 22:31 - 31:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hpm_ibias_wait_cnt(&mut self) -> HpmIbiasWaitCntW<RadcextcfgSpec> {
        HpmIbiasWaitCntW::new(self, 22)
    }
}
#[doc = "RADC External Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`radcextcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`radcextcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RadcextcfgSpec;
impl crate::RegisterSpec for RadcextcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`radcextcfg::R`](R) reader structure"]
impl crate::Readable for RadcextcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`radcextcfg::W`](W) writer structure"]
impl crate::Writable for RadcextcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RADCEXTCFG to value 0"]
impl crate::Resettable for RadcextcfgSpec {
    const RESET_VALUE: u32 = 0;
}
