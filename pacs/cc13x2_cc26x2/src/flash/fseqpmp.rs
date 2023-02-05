#[doc = "Register `FSEQPMP` reader"]
pub struct R(crate::R<FSEQPMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSEQPMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSEQPMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSEQPMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSEQPMP` writer"]
pub struct W(crate::W<FSEQPMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSEQPMP_SPEC>;
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
impl From<crate::W<FSEQPMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSEQPMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEQ_PUMP` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type SEQ_PUMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEQ_PUMP` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type SEQ_PUMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSEQPMP_SPEC, u8, u8, 8, O>;
#[doc = "Field `VIN_BY_PASS` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type VIN_BY_PASS_R = crate::BitReader<bool>;
#[doc = "Field `VIN_BY_PASS` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type VIN_BY_PASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSEQPMP_SPEC, bool, O>;
#[doc = "Field `RESERVED9` reader - 11:9\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED9` writer - 11:9\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSEQPMP_SPEC, u8, u8, 3, O>;
#[doc = "Field `VIN_AT_X` reader - 14:12\\]
Internal. Only to be used through TI provided API."]
pub type VIN_AT_X_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VIN_AT_X` writer - 14:12\\]
Internal. Only to be used through TI provided API."]
pub type VIN_AT_X_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSEQPMP_SPEC, u8, u8, 3, O>;
#[doc = "Field `RESERVED15` reader - 15:15\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED15_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED15` writer - 15:15\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED15_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSEQPMP_SPEC, bool, O>;
#[doc = "Field `TRIM_0P8` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type TRIM_0P8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIM_0P8` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type TRIM_0P8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSEQPMP_SPEC, u8, u8, 4, O>;
#[doc = "Field `TRIM_1P7` reader - 21:20\\]
Internal. Only to be used through TI provided API."]
pub type TRIM_1P7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIM_1P7` writer - 21:20\\]
Internal. Only to be used through TI provided API."]
pub type TRIM_1P7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSEQPMP_SPEC, u8, u8, 2, O>;
#[doc = "Field `RESERVED22` reader - 23:22\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED22_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED22` writer - 23:22\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED22_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSEQPMP_SPEC, u8, u8, 2, O>;
#[doc = "Field `TRIM_3P4` reader - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type TRIM_3P4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIM_3P4` writer - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type TRIM_3P4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSEQPMP_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED28` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED28_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED28` writer - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED28_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSEQPMP_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn seq_pump(&self) -> SEQ_PUMP_R {
        SEQ_PUMP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vin_by_pass(&self) -> VIN_BY_PASS_R {
        VIN_BY_PASS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - 11:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vin_at_x(&self) -> VIN_AT_X_R {
        VIN_AT_X_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved15(&self) -> RESERVED15_R {
        RESERVED15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim_0p8(&self) -> TRIM_0P8_R {
        TRIM_0P8_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - 21:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim_1p7(&self) -> TRIM_1P7_R {
        TRIM_1P7_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved22(&self) -> RESERVED22_R {
        RESERVED22_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim_3p4(&self) -> TRIM_3P4_R {
        TRIM_3P4_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved28(&self) -> RESERVED28_R {
        RESERVED28_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn seq_pump(&mut self) -> SEQ_PUMP_W<0> {
        SEQ_PUMP_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vin_by_pass(&mut self) -> VIN_BY_PASS_W<8> {
        VIN_BY_PASS_W::new(self)
    }
    #[doc = "Bits 9:11 - 11:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> RESERVED9_W<9> {
        RESERVED9_W::new(self)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vin_at_x(&mut self) -> VIN_AT_X_W<12> {
        VIN_AT_X_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved15(&mut self) -> RESERVED15_W<15> {
        RESERVED15_W::new(self)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn trim_0p8(&mut self) -> TRIM_0P8_W<16> {
        TRIM_0P8_W::new(self)
    }
    #[doc = "Bits 20:21 - 21:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn trim_1p7(&mut self) -> TRIM_1P7_W<20> {
        TRIM_1P7_W::new(self)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved22(&mut self) -> RESERVED22_W<22> {
        RESERVED22_W::new(self)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn trim_3p4(&mut self) -> TRIM_3P4_W<24> {
        TRIM_3P4_W::new(self)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved28(&mut self) -> RESERVED28_W<28> {
        RESERVED28_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fseqpmp](index.html) module"]
pub struct FSEQPMP_SPEC;
impl crate::RegisterSpec for FSEQPMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fseqpmp::R](R) reader structure"]
impl crate::Readable for FSEQPMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fseqpmp::W](W) writer structure"]
impl crate::Writable for FSEQPMP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSEQPMP to value 0x8508_0000"]
impl crate::Resettable for FSEQPMP_SPEC {
    const RESET_VALUE: Self::Ux = 0x8508_0000;
}
