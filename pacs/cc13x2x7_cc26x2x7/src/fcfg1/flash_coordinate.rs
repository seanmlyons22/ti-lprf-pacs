#[doc = "Register `FLASH_COORDINATE` reader"]
pub struct R(crate::R<FLASH_COORDINATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_COORDINATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_COORDINATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_COORDINATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_COORDINATE` writer"]
pub struct W(crate::W<FLASH_COORDINATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_COORDINATE_SPEC>;
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
impl From<crate::W<FLASH_COORDINATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_COORDINATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `YCOORDINATE` reader - 15:0\\]
Y coordinate of this unit on the wafer."]
pub type YCOORDINATE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `YCOORDINATE` writer - 15:0\\]
Y coordinate of this unit on the wafer."]
pub type YCOORDINATE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_COORDINATE_SPEC, u16, u16, 16, O>;
#[doc = "Field `XCOORDINATE` reader - 31:16\\]
X coordinate of this unit on the wafer."]
pub type XCOORDINATE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `XCOORDINATE` writer - 31:16\\]
X coordinate of this unit on the wafer."]
pub type XCOORDINATE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_COORDINATE_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Y coordinate of this unit on the wafer."]
    #[inline(always)]
    pub fn ycoordinate(&self) -> YCOORDINATE_R {
        YCOORDINATE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
X coordinate of this unit on the wafer."]
    #[inline(always)]
    pub fn xcoordinate(&self) -> XCOORDINATE_R {
        XCOORDINATE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Y coordinate of this unit on the wafer."]
    #[inline(always)]
    #[must_use]
    pub fn ycoordinate(&mut self) -> YCOORDINATE_W<0> {
        YCOORDINATE_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
X coordinate of this unit on the wafer."]
    #[inline(always)]
    #[must_use]
    pub fn xcoordinate(&mut self) -> XCOORDINATE_W<16> {
        XCOORDINATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash information\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_coordinate](index.html) module"]
pub struct FLASH_COORDINATE_SPEC;
impl crate::RegisterSpec for FLASH_COORDINATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_coordinate::R](R) reader structure"]
impl crate::Readable for FLASH_COORDINATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_coordinate::W](W) writer structure"]
impl crate::Writable for FLASH_COORDINATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLASH_COORDINATE to value 0"]
impl crate::Resettable for FLASH_COORDINATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
