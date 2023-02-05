#[doc = "Register `RBAR` reader"]
pub struct R(crate::R<RBAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RBAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RBAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RBAR` writer"]
pub struct W(crate::W<RBAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RBAR_SPEC>;
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
impl From<crate::W<RBAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RBAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XN` reader - 0:0\\]
Defines whether code can be executed from this region"]
pub type XN_R = crate::BitReader<bool>;
#[doc = "Field `XN` writer - 0:0\\]
Defines whether code can be executed from this region"]
pub type XN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RBAR_SPEC, bool, O>;
#[doc = "Field `AP` reader - 2:1\\]
Defines the access permissions for this region"]
pub type AP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AP` writer - 2:1\\]
Defines the access permissions for this region"]
pub type AP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RBAR_SPEC, u8, u8, 2, O>;
#[doc = "Field `SH` reader - 4:3\\]
Defines the Shareability domain of this region for Normal memory"]
pub type SH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SH` writer - 4:3\\]
Defines the Shareability domain of this region for Normal memory"]
pub type SH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RBAR_SPEC, u8, u8, 2, O>;
#[doc = "Field `BASE` reader - 31:5\\]
Contains bits \\[31:5\\]
of the lower inclusive limit of the selected MPU memory region. This value is zero extended to provide the base address to be checked against"]
pub type BASE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BASE` writer - 31:5\\]
Contains bits \\[31:5\\]
of the lower inclusive limit of the selected MPU memory region. This value is zero extended to provide the base address to be checked against"]
pub type BASE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RBAR_SPEC, u32, u32, 27, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Defines whether code can be executed from this region"]
    #[inline(always)]
    pub fn xn(&self) -> XN_R {
        XN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Defines the access permissions for this region"]
    #[inline(always)]
    pub fn ap(&self) -> AP_R {
        AP_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Defines the Shareability domain of this region for Normal memory"]
    #[inline(always)]
    pub fn sh(&self) -> SH_R {
        SH_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Contains bits \\[31:5\\]
of the lower inclusive limit of the selected MPU memory region. This value is zero extended to provide the base address to be checked against"]
    #[inline(always)]
    pub fn base(&self) -> BASE_R {
        BASE_R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Defines whether code can be executed from this region"]
    #[inline(always)]
    #[must_use]
    pub fn xn(&mut self) -> XN_W<0> {
        XN_W::new(self)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Defines the access permissions for this region"]
    #[inline(always)]
    #[must_use]
    pub fn ap(&mut self) -> AP_W<1> {
        AP_W::new(self)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Defines the Shareability domain of this region for Normal memory"]
    #[inline(always)]
    #[must_use]
    pub fn sh(&mut self) -> SH_W<3> {
        SH_W::new(self)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Contains bits \\[31:5\\]
of the lower inclusive limit of the selected MPU memory region. This value is zero extended to provide the base address to be checked against"]
    #[inline(always)]
    #[must_use]
    pub fn base(&mut self) -> BASE_W<5> {
        BASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Provides indirect read and write access to the base address of the currently selected MPU region\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbar](index.html) module"]
pub struct RBAR_SPEC;
impl crate::RegisterSpec for RBAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbar::R](R) reader structure"]
impl crate::Readable for RBAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rbar::W](W) writer structure"]
impl crate::Writable for RBAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RBAR to value 0"]
impl crate::Resettable for RBAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
