#[doc = "Register `MPU_TYPE` reader"]
pub struct R(crate::R<MPU_TYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPU_TYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPU_TYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPU_TYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPU_TYPE` writer"]
pub struct W(crate::W<MPU_TYPE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPU_TYPE_SPEC>;
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
impl From<crate::W<MPU_TYPE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPU_TYPE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEPARATE` reader - 0:0\\]
The processor core uses only a unified MPU, thus this field is always 0."]
pub type SEPARATE_R = crate::BitReader<bool>;
#[doc = "Field `SEPARATE` writer - 0:0\\]
The processor core uses only a unified MPU, thus this field is always 0."]
pub type SEPARATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPU_TYPE_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 7:1\\]
Reads 0."]
pub type RESERVED1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED1` writer - 7:1\\]
Reads 0."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPU_TYPE_SPEC, u8, u8, 7, O>;
#[doc = "Field `DREGION` reader - 15:8\\]
Number of supported MPU regions field. This field reads 0x08 indicating eight MPU regions."]
pub type DREGION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DREGION` writer - 15:8\\]
Number of supported MPU regions field. This field reads 0x08 indicating eight MPU regions."]
pub type DREGION_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPU_TYPE_SPEC, u8, u8, 8, O>;
#[doc = "Field `IREGION` reader - 23:16\\]
The processor core uses only a unified MPU, this field always reads 0x0."]
pub type IREGION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IREGION` writer - 23:16\\]
The processor core uses only a unified MPU, this field always reads 0x0."]
pub type IREGION_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPU_TYPE_SPEC, u8, u8, 8, O>;
#[doc = "Field `RESERVED24` reader - 31:24\\]
Reads 0."]
pub type RESERVED24_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED24` writer - 31:24\\]
Reads 0."]
pub type RESERVED24_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPU_TYPE_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
The processor core uses only a unified MPU, thus this field is always 0."]
    #[inline(always)]
    pub fn separate(&self) -> SEPARATE_R {
        SEPARATE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Reads 0."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Number of supported MPU regions field. This field reads 0x08 indicating eight MPU regions."]
    #[inline(always)]
    pub fn dregion(&self) -> DREGION_R {
        DREGION_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
The processor core uses only a unified MPU, this field always reads 0x0."]
    #[inline(always)]
    pub fn iregion(&self) -> IREGION_R {
        IREGION_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reads 0."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
The processor core uses only a unified MPU, thus this field is always 0."]
    #[inline(always)]
    #[must_use]
    pub fn separate(&mut self) -> SEPARATE_W<0> {
        SEPARATE_W::new(self)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Reads 0."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Number of supported MPU regions field. This field reads 0x08 indicating eight MPU regions."]
    #[inline(always)]
    #[must_use]
    pub fn dregion(&mut self) -> DREGION_W<8> {
        DREGION_W::new(self)
    }
    #[doc = "Bits 16:23 - 23:16\\]
The processor core uses only a unified MPU, this field always reads 0x0."]
    #[inline(always)]
    #[must_use]
    pub fn iregion(&mut self) -> IREGION_W<16> {
        IREGION_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reads 0."]
    #[inline(always)]
    #[must_use]
    pub fn reserved24(&mut self) -> RESERVED24_W<24> {
        RESERVED24_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPU Type This register indicates many regions the MPU supports.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpu_type](index.html) module"]
pub struct MPU_TYPE_SPEC;
impl crate::RegisterSpec for MPU_TYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpu_type::R](R) reader structure"]
impl crate::Readable for MPU_TYPE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpu_type::W](W) writer structure"]
impl crate::Writable for MPU_TYPE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MPU_TYPE to value 0x0800"]
impl crate::Resettable for MPU_TYPE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0800;
}
