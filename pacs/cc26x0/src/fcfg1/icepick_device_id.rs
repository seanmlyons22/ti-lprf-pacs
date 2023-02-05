#[doc = "Register `ICEPICK_DEVICE_ID` reader"]
pub struct R(crate::R<ICEPICK_DEVICE_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICEPICK_DEVICE_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICEPICK_DEVICE_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICEPICK_DEVICE_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICEPICK_DEVICE_ID` writer"]
pub struct W(crate::W<ICEPICK_DEVICE_ID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICEPICK_DEVICE_ID_SPEC>;
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
impl From<crate::W<ICEPICK_DEVICE_ID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICEPICK_DEVICE_ID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MANUFACTURER_ID` reader - 11:0\\]
Manufacturer code. 0x02F: Texas Instruments"]
pub type MANUFACTURER_ID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MANUFACTURER_ID` writer - 11:0\\]
Manufacturer code. 0x02F: Texas Instruments"]
pub type MANUFACTURER_ID_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ICEPICK_DEVICE_ID_SPEC, u16, u16, 12, O>;
#[doc = "Field `WAFER_ID` reader - 27:12\\]
Field used to identify silicon die."]
pub type WAFER_ID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WAFER_ID` writer - 27:12\\]
Field used to identify silicon die."]
pub type WAFER_ID_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ICEPICK_DEVICE_ID_SPEC, u16, u16, 16, O>;
#[doc = "Field `PG_REV` reader - 31:28\\]
Field used to distinguish revisions of the device."]
pub type PG_REV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PG_REV` writer - 31:28\\]
Field used to distinguish revisions of the device."]
pub type PG_REV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ICEPICK_DEVICE_ID_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Manufacturer code. 0x02F: Texas Instruments"]
    #[inline(always)]
    pub fn manufacturer_id(&self) -> MANUFACTURER_ID_R {
        MANUFACTURER_ID_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:27 - 27:12\\]
Field used to identify silicon die."]
    #[inline(always)]
    pub fn wafer_id(&self) -> WAFER_ID_R {
        WAFER_ID_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Field used to distinguish revisions of the device."]
    #[inline(always)]
    pub fn pg_rev(&self) -> PG_REV_R {
        PG_REV_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Manufacturer code. 0x02F: Texas Instruments"]
    #[inline(always)]
    #[must_use]
    pub fn manufacturer_id(&mut self) -> MANUFACTURER_ID_W<0> {
        MANUFACTURER_ID_W::new(self)
    }
    #[doc = "Bits 12:27 - 27:12\\]
Field used to identify silicon die."]
    #[inline(always)]
    #[must_use]
    pub fn wafer_id(&mut self) -> WAFER_ID_W<12> {
        WAFER_ID_W::new(self)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Field used to distinguish revisions of the device."]
    #[inline(always)]
    #[must_use]
    pub fn pg_rev(&mut self) -> PG_REV_W<28> {
        PG_REV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IcePick Device Identification Reading this register and the USER_ID register is the only support way of identifying a device.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icepick_device_id](index.html) module"]
pub struct ICEPICK_DEVICE_ID_SPEC;
impl crate::RegisterSpec for ICEPICK_DEVICE_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icepick_device_id::R](R) reader structure"]
impl crate::Readable for ICEPICK_DEVICE_ID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icepick_device_id::W](W) writer structure"]
impl crate::Writable for ICEPICK_DEVICE_ID_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICEPICK_DEVICE_ID to value 0x8b99_a02f"]
impl crate::Resettable for ICEPICK_DEVICE_ID_SPEC {
    const RESET_VALUE: Self::Ux = 0x8b99_a02f;
}
