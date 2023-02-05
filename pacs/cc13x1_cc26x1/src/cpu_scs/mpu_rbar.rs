#[doc = "Register `MPU_RBAR` reader"]
pub struct R(crate::R<MPU_RBAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPU_RBAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPU_RBAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPU_RBAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPU_RBAR` writer"]
pub struct W(crate::W<MPU_RBAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPU_RBAR_SPEC>;
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
impl From<crate::W<MPU_RBAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPU_RBAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGION` reader - 3:0\\]
MPU region override field"]
pub type REGION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REGION` writer - 3:0\\]
MPU region override field"]
pub type REGION_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPU_RBAR_SPEC, u8, u8, 4, O>;
#[doc = "Field `VALID` reader - 4:4\\]
MPU region number valid: 0: MPU_RNR remains unchanged and is interpreted. 1: MPU_RNR is overwritten by REGION."]
pub type VALID_R = crate::BitReader<bool>;
#[doc = "Field `VALID` writer - 4:4\\]
MPU region number valid: 0: MPU_RNR remains unchanged and is interpreted. 1: MPU_RNR is overwritten by REGION."]
pub type VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPU_RBAR_SPEC, bool, O>;
#[doc = "Field `ADDR` reader - 31:5\\]
Region base address field. The position of the LSB depends on the region size, so that the base address is aligned according to an even multiple of size. The power of 2 size specified by the SZENABLE field of the MPU Region Attribute and Size Register defines how many bits of base address are used."]
pub type ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDR` writer - 31:5\\]
Region base address field. The position of the LSB depends on the region size, so that the base address is aligned according to an even multiple of size. The power of 2 size specified by the SZENABLE field of the MPU Region Attribute and Size Register defines how many bits of base address are used."]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPU_RBAR_SPEC, u32, u32, 27, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
MPU region override field"]
    #[inline(always)]
    pub fn region(&self) -> REGION_R {
        REGION_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
MPU region number valid: 0: MPU_RNR remains unchanged and is interpreted. 1: MPU_RNR is overwritten by REGION."]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Region base address field. The position of the LSB depends on the region size, so that the base address is aligned according to an even multiple of size. The power of 2 size specified by the SZENABLE field of the MPU Region Attribute and Size Register defines how many bits of base address are used."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
MPU region override field"]
    #[inline(always)]
    #[must_use]
    pub fn region(&mut self) -> REGION_W<0> {
        REGION_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
MPU region number valid: 0: MPU_RNR remains unchanged and is interpreted. 1: MPU_RNR is overwritten by REGION."]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> VALID_W<4> {
        VALID_W::new(self)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Region base address field. The position of the LSB depends on the region size, so that the base address is aligned according to an even multiple of size. The power of 2 size specified by the SZENABLE field of the MPU Region Attribute and Size Register defines how many bits of base address are used."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<5> {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPU Region Base Address This register writes the base address of a region. It also contains a REGION field that can be used to override MPU_RNR.REGION, if the VALID bit is set. This register sets the base for the region. It is aligned by the size. So, a 64-KB sized region must be aligned on a multiple of 64KB, for example, 0x00010000 or 0x00020000. The region always reads back as the current MPU region number. VALID always reads back as 0. Writing VALID = 1 and REGION = n changes the region number to n. This is a short-hand way to write the MPU_RNR. This register is unpredictable if accessed other than as a word.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpu_rbar](index.html) module"]
pub struct MPU_RBAR_SPEC;
impl crate::RegisterSpec for MPU_RBAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpu_rbar::R](R) reader structure"]
impl crate::Readable for MPU_RBAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpu_rbar::W](W) writer structure"]
impl crate::Writable for MPU_RBAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MPU_RBAR to value 0"]
impl crate::Resettable for MPU_RBAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
