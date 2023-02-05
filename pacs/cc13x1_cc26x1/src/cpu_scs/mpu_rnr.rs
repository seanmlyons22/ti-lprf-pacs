#[doc = "Register `MPU_RNR` reader"]
pub struct R(crate::R<MPU_RNR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPU_RNR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPU_RNR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPU_RNR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPU_RNR` writer"]
pub struct W(crate::W<MPU_RNR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPU_RNR_SPEC>;
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
impl From<crate::W<MPU_RNR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPU_RNR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGION` reader - 7:0\\]
Region select field. This field selects the region to operate on when using the MPU_RASR and MPU_RBAR. It must be written first except when the address MPU_RBAR.VALID and MPU_RBAR.REGION fields are written, which overwrites this."]
pub type REGION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REGION` writer - 7:0\\]
Region select field. This field selects the region to operate on when using the MPU_RASR and MPU_RBAR. It must be written first except when the address MPU_RBAR.VALID and MPU_RBAR.REGION fields are written, which overwrites this."]
pub type REGION_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPU_RNR_SPEC, u8, u8, 8, O>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPU_RNR_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Region select field. This field selects the region to operate on when using the MPU_RASR and MPU_RBAR. It must be written first except when the address MPU_RBAR.VALID and MPU_RBAR.REGION fields are written, which overwrites this."]
    #[inline(always)]
    pub fn region(&self) -> REGION_R {
        REGION_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Region select field. This field selects the region to operate on when using the MPU_RASR and MPU_RBAR. It must be written first except when the address MPU_RBAR.VALID and MPU_RBAR.REGION fields are written, which overwrites this."]
    #[inline(always)]
    #[must_use]
    pub fn region(&mut self) -> REGION_W<0> {
        REGION_W::new(self)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> RESERVED8_W<8> {
        RESERVED8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPU Region Number This register is used to select which protection region is accessed. The following write to MPU_RASR or MPU_RBAR configures the characteristics of the protection region that is selected by this register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpu_rnr](index.html) module"]
pub struct MPU_RNR_SPEC;
impl crate::RegisterSpec for MPU_RNR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpu_rnr::R](R) reader structure"]
impl crate::Readable for MPU_RNR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpu_rnr::W](W) writer structure"]
impl crate::Writable for MPU_RNR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MPU_RNR to value 0"]
impl crate::Resettable for MPU_RNR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
