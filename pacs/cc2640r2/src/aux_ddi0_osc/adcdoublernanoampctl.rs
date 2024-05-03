#[doc = "Register `ADCDOUBLERNANOAMPCTL` reader"]
pub type R = crate::R<AdcdoublernanoampctlSpec>;
#[doc = "Register `ADCDOUBLERNANOAMPCTL` writer"]
pub type W = crate::W<AdcdoublernanoampctlSpec>;
#[doc = "Field `ADC_IREF_CTRL` reader - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type AdcIrefCtrlR = crate::FieldReader;
#[doc = "Field `ADC_IREF_CTRL` writer - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type AdcIrefCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESERVED2` reader - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `RESERVED2` writer - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADC_SH_VBUF_EN` reader - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type AdcShVbufEnR = crate::BitReader;
#[doc = "Field `ADC_SH_VBUF_EN` writer - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type AdcShVbufEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_SH_MODE_EN` reader - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type AdcShModeEnR = crate::BitReader;
#[doc = "Field `ADC_SH_MODE_EN` writer - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type AdcShModeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED6` reader - 22:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED6` writer - 22:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
#[doc = "Field `SPARE23` reader - 23:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
pub type Spare23R = crate::BitReader;
#[doc = "Field `SPARE23` writer - 23:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
pub type Spare23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NANOAMP_BIAS_ENABLE` reader - 24:24\\]
Internal. Only to be used through TI provided API."]
pub type NanoampBiasEnableR = crate::BitReader;
#[doc = "Field `NANOAMP_BIAS_ENABLE` writer - 24:24\\]
Internal. Only to be used through TI provided API."]
pub type NanoampBiasEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED25` reader - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved25R = crate::FieldReader;
#[doc = "Field `RESERVED25` writer - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved25W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adc_iref_ctrl(&self) -> AdcIrefCtrlR {
        AdcIrefCtrlR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adc_sh_vbuf_en(&self) -> AdcShVbufEnR {
        AdcShVbufEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adc_sh_mode_en(&self) -> AdcShModeEnR {
        AdcShModeEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:22 - 22:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new((self.bits >> 6) & 0x0001_ffff)
    }
    #[doc = "Bit 23 - 23:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
    #[inline(always)]
    pub fn spare23(&self) -> Spare23R {
        Spare23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn nanoamp_bias_enable(&self) -> NanoampBiasEnableR {
        NanoampBiasEnableR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved25(&self) -> Reserved25R {
        Reserved25R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn adc_iref_ctrl(&mut self) -> AdcIrefCtrlW<AdcdoublernanoampctlSpec> {
        AdcIrefCtrlW::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<AdcdoublernanoampctlSpec> {
        Reserved2W::new(self, 2)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn adc_sh_vbuf_en(&mut self) -> AdcShVbufEnW<AdcdoublernanoampctlSpec> {
        AdcShVbufEnW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn adc_sh_mode_en(&mut self) -> AdcShModeEnW<AdcdoublernanoampctlSpec> {
        AdcShModeEnW::new(self, 5)
    }
    #[doc = "Bits 6:22 - 22:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<AdcdoublernanoampctlSpec> {
        Reserved6W::new(self, 6)
    }
    #[doc = "Bit 23 - 23:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
    #[inline(always)]
    #[must_use]
    pub fn spare23(&mut self) -> Spare23W<AdcdoublernanoampctlSpec> {
        Spare23W::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn nanoamp_bias_enable(&mut self) -> NanoampBiasEnableW<AdcdoublernanoampctlSpec> {
        NanoampBiasEnableW::new(self, 24)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved25(&mut self) -> Reserved25W<AdcdoublernanoampctlSpec> {
        Reserved25W::new(self, 25)
    }
}
#[doc = "ADC Doubler Nanoamp Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcdoublernanoampctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcdoublernanoampctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcdoublernanoampctlSpec;
impl crate::RegisterSpec for AdcdoublernanoampctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcdoublernanoampctl::R`](R) reader structure"]
impl crate::Readable for AdcdoublernanoampctlSpec {}
#[doc = "`write(|w| ..)` method takes [`adcdoublernanoampctl::W`](W) writer structure"]
impl crate::Writable for AdcdoublernanoampctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCDOUBLERNANOAMPCTL to value 0"]
impl crate::Resettable for AdcdoublernanoampctlSpec {
    const RESET_VALUE: u32 = 0;
}
