#[doc = "Register `PUMP_TRIM_CFG_1` reader"]
pub struct R(crate::R<PUMP_TRIM_CFG_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUMP_TRIM_CFG_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUMP_TRIM_CFG_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUMP_TRIM_CFG_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PUMP_TRIM_CFG_1` writer"]
pub struct W(crate::W<PUMP_TRIM_CFG_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUMP_TRIM_CFG_1_SPEC>;
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
impl From<crate::W<PUMP_TRIM_CFG_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUMP_TRIM_CFG_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FOSCCT` reader - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type FOSCCT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FOSCCT` writer - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type FOSCCT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PUMP_TRIM_CFG_1_SPEC, u8, u8, 6, O>;
#[doc = "Field `IREFCT` reader - 9:6\\]
Internal. Only to be used through TI provided API."]
pub type IREFCT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IREFCT` writer - 9:6\\]
Internal. Only to be used through TI provided API."]
pub type IREFCT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PUMP_TRIM_CFG_1_SPEC, u8, u8, 4, O>;
#[doc = "Field `IREFTCCT` reader - 14:10\\]
Internal. Only to be used through TI provided API."]
pub type IREFTCCT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IREFTCCT` writer - 14:10\\]
Internal. Only to be used through TI provided API."]
pub type IREFTCCT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PUMP_TRIM_CFG_1_SPEC, u8, u8, 5, O>;
#[doc = "Field `IREFVRDCT` reader - 19:15\\]
Internal. Only to be used through TI provided API."]
pub type IREFVRDCT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IREFVRDCT` writer - 19:15\\]
Internal. Only to be used through TI provided API."]
pub type IREFVRDCT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PUMP_TRIM_CFG_1_SPEC, u8, u8, 5, O>;
#[doc = "Field `VCGCT` reader - 24:20\\]
Internal. Only to be used through TI provided API."]
pub type VCGCT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VCGCT` writer - 24:20\\]
Internal. Only to be used through TI provided API."]
pub type VCGCT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUMP_TRIM_CFG_1_SPEC, u8, u8, 5, O>;
#[doc = "Field `VINHCT` reader - 30:25\\]
Internal. Only to be used through TI provided API."]
pub type VINHCT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VINHCT` writer - 30:25\\]
Internal. Only to be used through TI provided API."]
pub type VINHCT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PUMP_TRIM_CFG_1_SPEC, u8, u8, 6, O>;
#[doc = "Field `VINHICCORCTLSB` reader - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type VINHICCORCTLSB_R = crate::BitReader<bool>;
#[doc = "Field `VINHICCORCTLSB` writer - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type VINHICCORCTLSB_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PUMP_TRIM_CFG_1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn foscct(&self) -> FOSCCT_R {
        FOSCCT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:9 - 9:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn irefct(&self) -> IREFCT_R {
        IREFCT_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:14 - 14:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ireftcct(&self) -> IREFTCCT_R {
        IREFTCCT_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 19:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn irefvrdct(&self) -> IREFVRDCT_R {
        IREFVRDCT_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - 24:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vcgct(&self) -> VCGCT_R {
        VCGCT_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:30 - 30:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vinhct(&self) -> VINHCT_R {
        VINHCT_R::new(((self.bits >> 25) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vinhiccorctlsb(&self) -> VINHICCORCTLSB_R {
        VINHICCORCTLSB_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn foscct(&mut self) -> FOSCCT_W<0> {
        FOSCCT_W::new(self)
    }
    #[doc = "Bits 6:9 - 9:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn irefct(&mut self) -> IREFCT_W<6> {
        IREFCT_W::new(self)
    }
    #[doc = "Bits 10:14 - 14:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ireftcct(&mut self) -> IREFTCCT_W<10> {
        IREFTCCT_W::new(self)
    }
    #[doc = "Bits 15:19 - 19:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn irefvrdct(&mut self) -> IREFVRDCT_W<15> {
        IREFVRDCT_W::new(self)
    }
    #[doc = "Bits 20:24 - 24:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vcgct(&mut self) -> VCGCT_W<20> {
        VCGCT_W::new(self)
    }
    #[doc = "Bits 25:30 - 30:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vinhct(&mut self) -> VINHCT_W<25> {
        VINHCT_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vinhiccorctlsb(&mut self) -> VINHICCORCTLSB_W<31> {
        VINHICCORCTLSB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pump_trim_cfg_1](index.html) module"]
pub struct PUMP_TRIM_CFG_1_SPEC;
impl crate::RegisterSpec for PUMP_TRIM_CFG_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pump_trim_cfg_1::R](R) reader structure"]
impl crate::Readable for PUMP_TRIM_CFG_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pump_trim_cfg_1::W](W) writer structure"]
impl crate::Writable for PUMP_TRIM_CFG_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PUMP_TRIM_CFG_1 to value 0x1920_0000"]
impl crate::Resettable for PUMP_TRIM_CFG_1_SPEC {
    const RESET_VALUE: Self::Ux = 0x1920_0000;
}
