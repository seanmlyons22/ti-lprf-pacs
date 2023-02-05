#[doc = "Register `MPU_RBAR_A2` reader"]
pub struct R(crate::R<MPU_RBAR_A2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPU_RBAR_A2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPU_RBAR_A2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPU_RBAR_A2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPU_RBAR_A2` writer"]
pub struct W(crate::W<MPU_RBAR_A2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPU_RBAR_A2_SPEC>;
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
impl From<crate::W<MPU_RBAR_A2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPU_RBAR_A2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPU_RBAR_A2` reader - 31:0\\]
Alias for MPU_RBAR"]
pub type MPU_RBAR_A2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MPU_RBAR_A2` writer - 31:0\\]
Alias for MPU_RBAR"]
pub type MPU_RBAR_A2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MPU_RBAR_A2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Alias for MPU_RBAR"]
    #[inline(always)]
    pub fn mpu_rbar_a2(&self) -> MPU_RBAR_A2_R {
        MPU_RBAR_A2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Alias for MPU_RBAR"]
    #[inline(always)]
    #[must_use]
    pub fn mpu_rbar_a2(&mut self) -> MPU_RBAR_A2_W<0> {
        MPU_RBAR_A2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPU Alias 2 Region Base Address Alias for MPU_RBAR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpu_rbar_a2](index.html) module"]
pub struct MPU_RBAR_A2_SPEC;
impl crate::RegisterSpec for MPU_RBAR_A2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpu_rbar_a2::R](R) reader structure"]
impl crate::Readable for MPU_RBAR_A2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpu_rbar_a2::W](W) writer structure"]
impl crate::Writable for MPU_RBAR_A2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MPU_RBAR_A2 to value 0"]
impl crate::Resettable for MPU_RBAR_A2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
