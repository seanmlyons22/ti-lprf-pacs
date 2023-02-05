#[doc = "Register `TEST4` reader"]
pub type R = crate::R<Test4Spec>;
#[doc = "Register `TEST4` writer"]
pub type W = crate::W<Test4Spec>;
#[doc = "Field `RESERVED0` reader - 15:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader<u16>;
#[doc = "Field `CAL_STEP_SEL` reader - 21:16\\]
ADC CAL STEP SELECTION"]
pub type CalStepSelR = crate::FieldReader;
#[doc = "Field `CAL_STEP_SEL` writer - 21:16\\]
ADC CAL STEP SELECTION"]
pub type CalStepSelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RESERVED22` reader - 23:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved22R = crate::FieldReader;
#[doc = "Field `CAL_MODE_EN` reader - 24:24\\]
ADC CDAC Calibration mode enable"]
pub type CalModeEnR = crate::BitReader;
#[doc = "Field `CAL_MODE_EN` writer - 24:24\\]
ADC CDAC Calibration mode enable"]
pub type CalModeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED25` reader - 30:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved25R = crate::FieldReader;
#[doc = "Field `HW_STEP_SEL_DIS` reader - 31:31\\]
By Enabling this bit, DLC written value overwritten of ull_usc_ulpadchp_dft_i$lt;26:0$gt; from TEST7 regsiter. This is for debug."]
pub type HwStepSelDisR = crate::BitReader;
#[doc = "Field `HW_STEP_SEL_DIS` writer - 31:31\\]
By Enabling this bit, DLC written value overwritten of ull_usc_ulpadchp_dft_i$lt;26:0$gt; from TEST7 regsiter. This is for debug."]
pub type HwStepSelDisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:21 - 21:16\\]
ADC CAL STEP SELECTION"]
    #[inline(always)]
    pub fn cal_step_sel(&self) -> CalStepSelR {
        CalStepSelR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved22(&self) -> Reserved22R {
        Reserved22R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
ADC CDAC Calibration mode enable"]
    #[inline(always)]
    pub fn cal_mode_en(&self) -> CalModeEnR {
        CalModeEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:30 - 30:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved25(&self) -> Reserved25R {
        Reserved25R::new(((self.bits >> 25) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
By Enabling this bit, DLC written value overwritten of ull_usc_ulpadchp_dft_i$lt;26:0$gt; from TEST7 regsiter. This is for debug."]
    #[inline(always)]
    pub fn hw_step_sel_dis(&self) -> HwStepSelDisR {
        HwStepSelDisR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:21 - 21:16\\]
ADC CAL STEP SELECTION"]
    #[inline(always)]
    #[must_use]
    pub fn cal_step_sel(&mut self) -> CalStepSelW<Test4Spec> {
        CalStepSelW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
ADC CDAC Calibration mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn cal_mode_en(&mut self) -> CalModeEnW<Test4Spec> {
        CalModeEnW::new(self, 24)
    }
    #[doc = "Bit 31 - 31:31\\]
By Enabling this bit, DLC written value overwritten of ull_usc_ulpadchp_dft_i$lt;26:0$gt; from TEST7 regsiter. This is for debug."]
    #[inline(always)]
    #[must_use]
    pub fn hw_step_sel_dis(&mut self) -> HwStepSelDisW<Test4Spec> {
        HwStepSelDisW::new(self, 31)
    }
}
#[doc = "CAL Control register: Average Sample count, Step number, Recal En and Debug option to override ull_usc_ulpadchp_dft_i$lt;26:0$gt;.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Test4Spec;
impl crate::RegisterSpec for Test4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test4::R`](R) reader structure"]
impl crate::Readable for Test4Spec {}
#[doc = "`write(|w| ..)` method takes [`test4::W`](W) writer structure"]
impl crate::Writable for Test4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TEST4 to value 0"]
impl crate::Resettable for Test4Spec {
    const RESET_VALUE: u32 = 0;
}
