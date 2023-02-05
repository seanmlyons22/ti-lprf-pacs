#[doc = "Register `MPU_RASR_A1` reader"]
pub struct R(crate::R<MPU_RASR_A1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPU_RASR_A1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPU_RASR_A1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPU_RASR_A1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPU_RASR_A1` writer"]
pub struct W(crate::W<MPU_RASR_A1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPU_RASR_A1_SPEC>;
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
impl From<crate::W<MPU_RASR_A1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPU_RASR_A1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPU_RASR_A1` reader - 31:0\\]
Alias for MPU_RASR"]
pub type MPU_RASR_A1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MPU_RASR_A1` writer - 31:0\\]
Alias for MPU_RASR"]
pub type MPU_RASR_A1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MPU_RASR_A1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Alias for MPU_RASR"]
    #[inline(always)]
    pub fn mpu_rasr_a1(&self) -> MPU_RASR_A1_R {
        MPU_RASR_A1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Alias for MPU_RASR"]
    #[inline(always)]
    #[must_use]
    pub fn mpu_rasr_a1(&mut self) -> MPU_RASR_A1_W<0> {
        MPU_RASR_A1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPU Alias 1 Region Attribute and Size Alias for MPU_RASR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpu_rasr_a1](index.html) module"]
pub struct MPU_RASR_A1_SPEC;
impl crate::RegisterSpec for MPU_RASR_A1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpu_rasr_a1::R](R) reader structure"]
impl crate::Readable for MPU_RASR_A1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpu_rasr_a1::W](W) writer structure"]
impl crate::Writable for MPU_RASR_A1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MPU_RASR_A1 to value 0"]
impl crate::Resettable for MPU_RASR_A1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
