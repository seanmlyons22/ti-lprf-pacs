#[doc = "Register `ADCREF0` reader"]
pub struct R(crate::R<ADCREF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCREF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCREF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCREF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCREF0` writer"]
pub struct W(crate::W<ADCREF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCREF0_SPEC>;
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
impl From<crate::W<ADCREF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCREF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - 0:0\\]
ADC reference module enable: 0: ADC reference module powered down 1: ADC reference module enabled"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - 0:0\\]
ADC reference module enable: 0: ADC reference module powered down 1: ADC reference module enabled"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, ADCREF0_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 2:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED1` writer - 2:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u8, ADCREF0_SPEC, u8, u8, 2, O>;
#[doc = "Field `SRC` reader - 3:3\\]
ADC reference source: 0: Fixed reference = 4.3V 1: Relative reference = VDDS"]
pub type SRC_R = crate::BitReader<bool>;
#[doc = "Field `SRC` writer - 3:3\\]
ADC reference source: 0: Fixed reference = 4.3V 1: Relative reference = VDDS"]
pub type SRC_W<'a, const O: u8> = crate::BitWriter<'a, u8, ADCREF0_SPEC, bool, O>;
#[doc = "Field `EXT` reader - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type EXT_R = crate::BitReader<bool>;
#[doc = "Field `EXT` writer - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type EXT_W<'a, const O: u8> = crate::BitWriter<'a, u8, ADCREF0_SPEC, bool, O>;
#[doc = "Field `IOMUX` reader - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type IOMUX_R = crate::BitReader<bool>;
#[doc = "Field `IOMUX` writer - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type IOMUX_W<'a, const O: u8> = crate::BitWriter<'a, u8, ADCREF0_SPEC, bool, O>;
#[doc = "Field `REF_ON_IDLE` reader - 6:6\\]
Keep ADCREF powered up in IDLE state when ADC0.SMPL_MODE = 0. Set to 1 if ADC0.SMPL_CYCLE_EXP is less than 6 (21.3us sampling time)"]
pub type REF_ON_IDLE_R = crate::BitReader<bool>;
#[doc = "Field `REF_ON_IDLE` writer - 6:6\\]
Keep ADCREF powered up in IDLE state when ADC0.SMPL_MODE = 0. Set to 1 if ADC0.SMPL_CYCLE_EXP is less than 6 (21.3us sampling time)"]
pub type REF_ON_IDLE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ADCREF0_SPEC, bool, O>;
#[doc = "Field `RESERVED7` reader - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED7_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED7` writer - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED7_W<'a, const O: u8> = crate::BitWriter<'a, u8, ADCREF0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
ADC reference module enable: 0: ADC reference module powered down 1: ADC reference module enabled"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new((self.bits >> 1) & 3)
    }
    #[doc = "Bit 3 - 3:3\\]
ADC reference source: 0: Fixed reference = 4.3V 1: Relative reference = VDDS"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ext(&self) -> EXT_R {
        EXT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn iomux(&self) -> IOMUX_R {
        IOMUX_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Keep ADCREF powered up in IDLE state when ADC0.SMPL_MODE = 0. Set to 1 if ADC0.SMPL_CYCLE_EXP is less than 6 (21.3us sampling time)"]
    #[inline(always)]
    pub fn ref_on_idle(&self) -> REF_ON_IDLE_R {
        REF_ON_IDLE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
ADC reference module enable: 0: ADC reference module powered down 1: ADC reference module enabled"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
ADC reference source: 0: Fixed reference = 4.3V 1: Relative reference = VDDS"]
    #[inline(always)]
    #[must_use]
    pub fn src(&mut self) -> SRC_W<3> {
        SRC_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ext(&mut self) -> EXT_W<4> {
        EXT_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn iomux(&mut self) -> IOMUX_W<5> {
        IOMUX_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Keep ADCREF powered up in IDLE state when ADC0.SMPL_MODE = 0. Set to 1 if ADC0.SMPL_CYCLE_EXP is less than 6 (21.3us sampling time)"]
    #[inline(always)]
    #[must_use]
    pub fn ref_on_idle(&mut self) -> REF_ON_IDLE_W<6> {
        REF_ON_IDLE_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> RESERVED7_W<7> {
        RESERVED7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Reference 0 Control reference used by the ADC. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcref0](index.html) module"]
pub struct ADCREF0_SPEC;
impl crate::RegisterSpec for ADCREF0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [adcref0::R](R) reader structure"]
impl crate::Readable for ADCREF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcref0::W](W) writer structure"]
impl crate::Writable for ADCREF0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCREF0 to value 0"]
impl crate::Resettable for ADCREF0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
