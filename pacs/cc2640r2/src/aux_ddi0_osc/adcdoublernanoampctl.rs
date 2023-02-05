#[doc = "Register `ADCDOUBLERNANOAMPCTL` reader"]
pub struct R(crate::R<ADCDOUBLERNANOAMPCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCDOUBLERNANOAMPCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCDOUBLERNANOAMPCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCDOUBLERNANOAMPCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCDOUBLERNANOAMPCTL` writer"]
pub struct W(crate::W<ADCDOUBLERNANOAMPCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCDOUBLERNANOAMPCTL_SPEC>;
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
impl From<crate::W<ADCDOUBLERNANOAMPCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCDOUBLERNANOAMPCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_IREF_CTRL` reader - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type ADC_IREF_CTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_IREF_CTRL` writer - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type ADC_IREF_CTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADCDOUBLERNANOAMPCTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `RESERVED2` reader - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED2` writer - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADCDOUBLERNANOAMPCTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `ADC_SH_VBUF_EN` reader - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type ADC_SH_VBUF_EN_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SH_VBUF_EN` writer - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type ADC_SH_VBUF_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADCDOUBLERNANOAMPCTL_SPEC, bool, O>;
#[doc = "Field `ADC_SH_MODE_EN` reader - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type ADC_SH_MODE_EN_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SH_MODE_EN` writer - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type ADC_SH_MODE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADCDOUBLERNANOAMPCTL_SPEC, bool, O>;
#[doc = "Field `RESERVED6` reader - 22:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED6` writer - 22:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADCDOUBLERNANOAMPCTL_SPEC, u32, u32, 17, O>;
#[doc = "Field `SPARE23` reader - 23:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
pub type SPARE23_R = crate::BitReader<bool>;
#[doc = "Field `SPARE23` writer - 23:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
pub type SPARE23_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCDOUBLERNANOAMPCTL_SPEC, bool, O>;
#[doc = "Field `NANOAMP_BIAS_ENABLE` reader - 24:24\\]
Internal. Only to be used through TI provided API."]
pub type NANOAMP_BIAS_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `NANOAMP_BIAS_ENABLE` writer - 24:24\\]
Internal. Only to be used through TI provided API."]
pub type NANOAMP_BIAS_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADCDOUBLERNANOAMPCTL_SPEC, bool, O>;
#[doc = "Field `RESERVED25` reader - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED25_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED25` writer - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED25_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADCDOUBLERNANOAMPCTL_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adc_iref_ctrl(&self) -> ADC_IREF_CTRL_R {
        ADC_IREF_CTRL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adc_sh_vbuf_en(&self) -> ADC_SH_VBUF_EN_R {
        ADC_SH_VBUF_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adc_sh_mode_en(&self) -> ADC_SH_MODE_EN_R {
        ADC_SH_MODE_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:22 - 22:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new((self.bits >> 6) & 0x0001_ffff)
    }
    #[doc = "Bit 23 - 23:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
    #[inline(always)]
    pub fn spare23(&self) -> SPARE23_R {
        SPARE23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn nanoamp_bias_enable(&self) -> NANOAMP_BIAS_ENABLE_R {
        NANOAMP_BIAS_ENABLE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved25(&self) -> RESERVED25_R {
        RESERVED25_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn adc_iref_ctrl(&mut self) -> ADC_IREF_CTRL_W<0> {
        ADC_IREF_CTRL_W::new(self)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn adc_sh_vbuf_en(&mut self) -> ADC_SH_VBUF_EN_W<4> {
        ADC_SH_VBUF_EN_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn adc_sh_mode_en(&mut self) -> ADC_SH_MODE_EN_W<5> {
        ADC_SH_MODE_EN_W::new(self)
    }
    #[doc = "Bits 6:22 - 22:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<6> {
        RESERVED6_W::new(self)
    }
    #[doc = "Bit 23 - 23:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
    #[inline(always)]
    #[must_use]
    pub fn spare23(&mut self) -> SPARE23_W<23> {
        SPARE23_W::new(self)
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn nanoamp_bias_enable(&mut self) -> NANOAMP_BIAS_ENABLE_W<24> {
        NANOAMP_BIAS_ENABLE_W::new(self)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved25(&mut self) -> RESERVED25_W<25> {
        RESERVED25_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Doubler Nanoamp Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcdoublernanoampctl](index.html) module"]
pub struct ADCDOUBLERNANOAMPCTL_SPEC;
impl crate::RegisterSpec for ADCDOUBLERNANOAMPCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcdoublernanoampctl::R](R) reader structure"]
impl crate::Readable for ADCDOUBLERNANOAMPCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcdoublernanoampctl::W](W) writer structure"]
impl crate::Writable for ADCDOUBLERNANOAMPCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCDOUBLERNANOAMPCTL to value 0"]
impl crate::Resettable for ADCDOUBLERNANOAMPCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
