#[doc = "Register `FWREV` reader"]
pub struct R(crate::R<FWREV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FWREV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FWREV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FWREV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FWREV` writer"]
pub struct W(crate::W<FWREV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FWREV_SPEC>;
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
impl From<crate::W<FWREV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FWREV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 15:0\\]
Ignore on read"]
pub type RESERVED0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED0` writer - 15:0\\]
Ignore on read"]
pub type RESERVED0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FWREV_SPEC, u16, u16, 16, O>;
#[doc = "Field `FW_PATCH_LEVEL` reader - 19:16\\]
4-bit binary encoding of the firmware patch level, initial release will carry value zero Patches are used to remove bugs without changing the functionality or interface of a module."]
pub type FW_PATCH_LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FW_PATCH_LEVEL` writer - 19:16\\]
4-bit binary encoding of the firmware patch level, initial release will carry value zero Patches are used to remove bugs without changing the functionality or interface of a module."]
pub type FW_PATCH_LEVEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FWREV_SPEC, u8, u8, 4, O>;
#[doc = "Field `MINOR_FW_REVISION` reader - 23:20\\]
4-bit binary encoding of the minor firmware revision number"]
pub type MINOR_FW_REVISION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MINOR_FW_REVISION` writer - 23:20\\]
4-bit binary encoding of the minor firmware revision number"]
pub type MINOR_FW_REVISION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FWREV_SPEC, u8, u8, 4, O>;
#[doc = "Field `MAJOR_FW_REVISION` reader - 27:24\\]
4-bit binary encoding of the major firmware revision number"]
pub type MAJOR_FW_REVISION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAJOR_FW_REVISION` writer - 27:24\\]
4-bit binary encoding of the major firmware revision number"]
pub type MAJOR_FW_REVISION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FWREV_SPEC, u8, u8, 4, O>;
#[doc = "Field `FW_CAPABILITIES` reader - 31:28\\]
Firmware Capabilities 4-bit binary encoding for the functionality implemented in the firmware. 0x0: indicates basic ModExp with/without CRT. 0x1: adds Modular Inversion, 0x2: value 2 adds Modular Inversion and ECC operations. 0x3-0xF : Reserved."]
pub type FW_CAPABILITIES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FW_CAPABILITIES` writer - 31:28\\]
Firmware Capabilities 4-bit binary encoding for the functionality implemented in the firmware. 0x0: indicates basic ModExp with/without CRT. 0x1: adds Modular Inversion, 0x2: value 2 adds Modular Inversion and ECC operations. 0x3-0xF : Reserved."]
pub type FW_CAPABILITIES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FWREV_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Ignore on read"]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
4-bit binary encoding of the firmware patch level, initial release will carry value zero Patches are used to remove bugs without changing the functionality or interface of a module."]
    #[inline(always)]
    pub fn fw_patch_level(&self) -> FW_PATCH_LEVEL_R {
        FW_PATCH_LEVEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
4-bit binary encoding of the minor firmware revision number"]
    #[inline(always)]
    pub fn minor_fw_revision(&self) -> MINOR_FW_REVISION_R {
        MINOR_FW_REVISION_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
4-bit binary encoding of the major firmware revision number"]
    #[inline(always)]
    pub fn major_fw_revision(&self) -> MAJOR_FW_REVISION_R {
        MAJOR_FW_REVISION_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Firmware Capabilities 4-bit binary encoding for the functionality implemented in the firmware. 0x0: indicates basic ModExp with/without CRT. 0x1: adds Modular Inversion, 0x2: value 2 adds Modular Inversion and ECC operations. 0x3-0xF : Reserved."]
    #[inline(always)]
    pub fn fw_capabilities(&self) -> FW_CAPABILITIES_R {
        FW_CAPABILITIES_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Ignore on read"]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<0> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bits 16:19 - 19:16\\]
4-bit binary encoding of the firmware patch level, initial release will carry value zero Patches are used to remove bugs without changing the functionality or interface of a module."]
    #[inline(always)]
    #[must_use]
    pub fn fw_patch_level(&mut self) -> FW_PATCH_LEVEL_W<16> {
        FW_PATCH_LEVEL_W::new(self)
    }
    #[doc = "Bits 20:23 - 23:20\\]
4-bit binary encoding of the minor firmware revision number"]
    #[inline(always)]
    #[must_use]
    pub fn minor_fw_revision(&mut self) -> MINOR_FW_REVISION_W<20> {
        MINOR_FW_REVISION_W::new(self)
    }
    #[doc = "Bits 24:27 - 27:24\\]
4-bit binary encoding of the major firmware revision number"]
    #[inline(always)]
    #[must_use]
    pub fn major_fw_revision(&mut self) -> MAJOR_FW_REVISION_W<24> {
        MAJOR_FW_REVISION_W::new(self)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Firmware Capabilities 4-bit binary encoding for the functionality implemented in the firmware. 0x0: indicates basic ModExp with/without CRT. 0x1: adds Modular Inversion, 0x2: value 2 adds Modular Inversion and ECC operations. 0x3-0xF : Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn fw_capabilities(&mut self) -> FW_CAPABILITIES_W<28> {
        FW_CAPABILITIES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PKA firmware revision and capabilities register This register allows the host access to the internal firmware revision number of the PKA Engine for software driver matching and diagnostic purposes. This register also contains a field that encodes the capabilities of the embedded firmware. This register is written by the firmware within a few clock cycles after starting up that firmware. The hardware reset value is zero, indicating that the information has not been written yet.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fwrev](index.html) module"]
pub struct FWREV_SPEC;
impl crate::RegisterSpec for FWREV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fwrev::R](R) reader structure"]
impl crate::Readable for FWREV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fwrev::W](W) writer structure"]
impl crate::Writable for FWREV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FWREV to value 0x2150_0000"]
impl crate::Resettable for FWREV_SPEC {
    const RESET_VALUE: Self::Ux = 0x2150_0000;
}
