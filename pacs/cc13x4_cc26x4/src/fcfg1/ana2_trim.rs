#[doc = "Register `ANA2_TRIM` reader"]
pub struct R(crate::R<ANA2_TRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANA2_TRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANA2_TRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANA2_TRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANA2_TRIM` writer"]
pub struct W(crate::W<ANA2_TRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANA2_TRIM_SPEC>;
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
impl From<crate::W<ANA2_TRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANA2_TRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCDC_HIGH_EN_SEL` reader - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type DCDC_HIGH_EN_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCDC_HIGH_EN_SEL` writer - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type DCDC_HIGH_EN_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ANA2_TRIM_SPEC, u8, u8, 3, O>;
#[doc = "Field `DCDC_LOW_EN_SEL` reader - 5:3\\]
Internal. Only to be used through TI provided API."]
pub type DCDC_LOW_EN_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCDC_LOW_EN_SEL` writer - 5:3\\]
Internal. Only to be used through TI provided API."]
pub type DCDC_LOW_EN_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ANA2_TRIM_SPEC, u8, u8, 3, O>;
#[doc = "Field `DEAD_TIME_TRIM` reader - 7:6\\]
Internal. Only to be used through TI provided API."]
pub type DEAD_TIME_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEAD_TIME_TRIM` writer - 7:6\\]
Internal. Only to be used through TI provided API."]
pub type DEAD_TIME_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ANA2_TRIM_SPEC, u8, u8, 2, O>;
#[doc = "Field `DCDC_IPEAK` reader - 10:8\\]
Internal. Only to be used through TI provided API."]
pub type DCDC_IPEAK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCDC_IPEAK` writer - 10:8\\]
Internal. Only to be used through TI provided API."]
pub type DCDC_IPEAK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ANA2_TRIM_SPEC, u8, u8, 3, O>;
#[doc = "Field `DITHER_EN` reader - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type DITHER_EN_R = crate::BitReader<bool>;
#[doc = "Field `DITHER_EN` writer - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type DITHER_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANA2_TRIM_SPEC, bool, O>;
#[doc = "Field `DCDC_DRV_DS` reader - 14:12\\]
Internal. Only to be used through TI provided API."]
pub type DCDC_DRV_DS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCDC_DRV_DS` writer - 14:12\\]
Internal. Only to be used through TI provided API."]
pub type DCDC_DRV_DS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ANA2_TRIM_SPEC, u8, u8, 3, O>;
#[doc = "Field `NANOAMP_RES_TRIM` reader - 21:15\\]
Internal. Only to be used through TI provided API."]
pub type NANOAMP_RES_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NANOAMP_RES_TRIM` writer - 21:15\\]
Internal. Only to be used through TI provided API."]
pub type NANOAMP_RES_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ANA2_TRIM_SPEC, u8, u8, 7, O>;
#[doc = "Field `ATESTLF_UDIGLDO_IBIAS_TRIM` reader - 22:22\\]
Internal. Only to be used through TI provided API."]
pub type ATESTLF_UDIGLDO_IBIAS_TRIM_R = crate::BitReader<bool>;
#[doc = "Field `ATESTLF_UDIGLDO_IBIAS_TRIM` writer - 22:22\\]
Internal. Only to be used through TI provided API."]
pub type ATESTLF_UDIGLDO_IBIAS_TRIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ANA2_TRIM_SPEC, bool, O>;
#[doc = "Field `SET_RCOSC_HF_FINE_RESISTOR` reader - 24:23\\]
Internal. Only to be used through TI provided API."]
pub type SET_RCOSC_HF_FINE_RESISTOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SET_RCOSC_HF_FINE_RESISTOR` writer - 24:23\\]
Internal. Only to be used through TI provided API."]
pub type SET_RCOSC_HF_FINE_RESISTOR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ANA2_TRIM_SPEC, u8, u8, 2, O>;
#[doc = "Field `RESERVED0` reader - 25:25\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED0_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED0` writer - 25:25\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANA2_TRIM_SPEC, bool, O>;
#[doc = "Field `RCOSCHFCTRIMFRACT` reader - 30:26\\]
Internal. Only to be used through TI provided API."]
pub type RCOSCHFCTRIMFRACT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RCOSCHFCTRIMFRACT` writer - 30:26\\]
Internal. Only to be used through TI provided API."]
pub type RCOSCHFCTRIMFRACT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ANA2_TRIM_SPEC, u8, u8, 5, O>;
#[doc = "Field `RCOSCHFCTRIMFRACT_EN` reader - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type RCOSCHFCTRIMFRACT_EN_R = crate::BitReader<bool>;
#[doc = "Field `RCOSCHFCTRIMFRACT_EN` writer - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type RCOSCHFCTRIMFRACT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ANA2_TRIM_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dcdc_high_en_sel(&self) -> DCDC_HIGH_EN_SEL_R {
        DCDC_HIGH_EN_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dcdc_low_en_sel(&self) -> DCDC_LOW_EN_SEL_R {
        DCDC_LOW_EN_SEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dead_time_trim(&self) -> DEAD_TIME_TRIM_R {
        DEAD_TIME_TRIM_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dcdc_ipeak(&self) -> DCDC_IPEAK_R {
        DCDC_IPEAK_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dither_en(&self) -> DITHER_EN_R {
        DITHER_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dcdc_drv_ds(&self) -> DCDC_DRV_DS_R {
        DCDC_DRV_DS_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:21 - 21:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn nanoamp_res_trim(&self) -> NANOAMP_RES_TRIM_R {
        NANOAMP_RES_TRIM_R::new(((self.bits >> 15) & 0x7f) as u8)
    }
    #[doc = "Bit 22 - 22:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn atestlf_udigldo_ibias_trim(&self) -> ATESTLF_UDIGLDO_IBIAS_TRIM_R {
        ATESTLF_UDIGLDO_IBIAS_TRIM_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:24 - 24:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn set_rcosc_hf_fine_resistor(&self) -> SET_RCOSC_HF_FINE_RESISTOR_R {
        SET_RCOSC_HF_FINE_RESISTOR_R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bit 25 - 25:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:30 - 30:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcoschfctrimfract(&self) -> RCOSCHFCTRIMFRACT_R {
        RCOSCHFCTRIMFRACT_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcoschfctrimfract_en(&self) -> RCOSCHFCTRIMFRACT_EN_R {
        RCOSCHFCTRIMFRACT_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_high_en_sel(&mut self) -> DCDC_HIGH_EN_SEL_W<0> {
        DCDC_HIGH_EN_SEL_W::new(self)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_low_en_sel(&mut self) -> DCDC_LOW_EN_SEL_W<3> {
        DCDC_LOW_EN_SEL_W::new(self)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dead_time_trim(&mut self) -> DEAD_TIME_TRIM_W<6> {
        DEAD_TIME_TRIM_W::new(self)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_ipeak(&mut self) -> DCDC_IPEAK_W<8> {
        DCDC_IPEAK_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dither_en(&mut self) -> DITHER_EN_W<11> {
        DITHER_EN_W::new(self)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_drv_ds(&mut self) -> DCDC_DRV_DS_W<12> {
        DCDC_DRV_DS_W::new(self)
    }
    #[doc = "Bits 15:21 - 21:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn nanoamp_res_trim(&mut self) -> NANOAMP_RES_TRIM_W<15> {
        NANOAMP_RES_TRIM_W::new(self)
    }
    #[doc = "Bit 22 - 22:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn atestlf_udigldo_ibias_trim(&mut self) -> ATESTLF_UDIGLDO_IBIAS_TRIM_W<22> {
        ATESTLF_UDIGLDO_IBIAS_TRIM_W::new(self)
    }
    #[doc = "Bits 23:24 - 24:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn set_rcosc_hf_fine_resistor(&mut self) -> SET_RCOSC_HF_FINE_RESISTOR_W<23> {
        SET_RCOSC_HF_FINE_RESISTOR_W::new(self)
    }
    #[doc = "Bit 25 - 25:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<25> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bits 26:30 - 30:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rcoschfctrimfract(&mut self) -> RCOSCHFCTRIMFRACT_W<26> {
        RCOSCHFCTRIMFRACT_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rcoschfctrimfract_en(&mut self) -> RCOSCHFCTRIMFRACT_EN_W<31> {
        RCOSCHFCTRIMFRACT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana2_trim](index.html) module"]
pub struct ANA2_TRIM_SPEC;
impl crate::RegisterSpec for ANA2_TRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ana2_trim::R](R) reader structure"]
impl crate::Readable for ANA2_TRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ana2_trim::W](W) writer structure"]
impl crate::Writable for ANA2_TRIM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ANA2_TRIM to value 0x8240_087f"]
impl crate::Resettable for ANA2_TRIM_SPEC {
    const RESET_VALUE: Self::Ux = 0x8240_087f;
}
