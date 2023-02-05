#[doc = "Register `HWREV` reader"]
pub struct R(crate::R<HWREV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWREV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWREV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWREV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HWREV` writer"]
pub struct W(crate::W<HWREV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HWREV_SPEC>;
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
impl From<crate::W<HWREV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HWREV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BASIC_EIP_NUMBER` reader - 7:0\\]
8-bit binary encoding of the EIP number, EIP-28 gives 0x1C"]
pub type BASIC_EIP_NUMBER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BASIC_EIP_NUMBER` writer - 7:0\\]
8-bit binary encoding of the EIP number, EIP-28 gives 0x1C"]
pub type BASIC_EIP_NUMBER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HWREV_SPEC, u8, u8, 8, O>;
#[doc = "Field `COMPLEMENT_OF_BASIC_EIP_NUMBER` reader - 15:8\\]
Bit-by-bit logic complement of bits \\[7:0\\], EIP-28 gives 0xE3"]
pub type COMPLEMENT_OF_BASIC_EIP_NUMBER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPLEMENT_OF_BASIC_EIP_NUMBER` writer - 15:8\\]
Bit-by-bit logic complement of bits \\[7:0\\], EIP-28 gives 0xE3"]
pub type COMPLEMENT_OF_BASIC_EIP_NUMBER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HWREV_SPEC, u8, u8, 8, O>;
#[doc = "Field `HW_PATCH_LEVEL` reader - 19:16\\]
4-bit binary encoding of the hardware patch level, initial release will carry value zero Patches are used to remove bugs without changing the functionality or interface of a module."]
pub type HW_PATCH_LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HW_PATCH_LEVEL` writer - 19:16\\]
4-bit binary encoding of the hardware patch level, initial release will carry value zero Patches are used to remove bugs without changing the functionality or interface of a module."]
pub type HW_PATCH_LEVEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWREV_SPEC, u8, u8, 4, O>;
#[doc = "Field `MINOR_HW_REVISION` reader - 23:20\\]
4-bit binary encoding of the minor hardware revision number"]
pub type MINOR_HW_REVISION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MINOR_HW_REVISION` writer - 23:20\\]
4-bit binary encoding of the minor hardware revision number"]
pub type MINOR_HW_REVISION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HWREV_SPEC, u8, u8, 4, O>;
#[doc = "Field `MAJOR_HW_REVISION` reader - 27:24\\]
4-bit binary encoding of the major hardware revision number"]
pub type MAJOR_HW_REVISION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAJOR_HW_REVISION` writer - 27:24\\]
4-bit binary encoding of the major hardware revision number"]
pub type MAJOR_HW_REVISION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HWREV_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED28` reader - 31:28\\]
Ignore on read"]
pub type RESERVED28_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED28` writer - 31:28\\]
Ignore on read"]
pub type RESERVED28_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWREV_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
8-bit binary encoding of the EIP number, EIP-28 gives 0x1C"]
    #[inline(always)]
    pub fn basic_eip_number(&self) -> BASIC_EIP_NUMBER_R {
        BASIC_EIP_NUMBER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Bit-by-bit logic complement of bits \\[7:0\\], EIP-28 gives 0xE3"]
    #[inline(always)]
    pub fn complement_of_basic_eip_number(&self) -> COMPLEMENT_OF_BASIC_EIP_NUMBER_R {
        COMPLEMENT_OF_BASIC_EIP_NUMBER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
4-bit binary encoding of the hardware patch level, initial release will carry value zero Patches are used to remove bugs without changing the functionality or interface of a module."]
    #[inline(always)]
    pub fn hw_patch_level(&self) -> HW_PATCH_LEVEL_R {
        HW_PATCH_LEVEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
4-bit binary encoding of the minor hardware revision number"]
    #[inline(always)]
    pub fn minor_hw_revision(&self) -> MINOR_HW_REVISION_R {
        MINOR_HW_REVISION_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
4-bit binary encoding of the major hardware revision number"]
    #[inline(always)]
    pub fn major_hw_revision(&self) -> MAJOR_HW_REVISION_R {
        MAJOR_HW_REVISION_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Ignore on read"]
    #[inline(always)]
    pub fn reserved28(&self) -> RESERVED28_R {
        RESERVED28_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
8-bit binary encoding of the EIP number, EIP-28 gives 0x1C"]
    #[inline(always)]
    #[must_use]
    pub fn basic_eip_number(&mut self) -> BASIC_EIP_NUMBER_W<0> {
        BASIC_EIP_NUMBER_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Bit-by-bit logic complement of bits \\[7:0\\], EIP-28 gives 0xE3"]
    #[inline(always)]
    #[must_use]
    pub fn complement_of_basic_eip_number(&mut self) -> COMPLEMENT_OF_BASIC_EIP_NUMBER_W<8> {
        COMPLEMENT_OF_BASIC_EIP_NUMBER_W::new(self)
    }
    #[doc = "Bits 16:19 - 19:16\\]
4-bit binary encoding of the hardware patch level, initial release will carry value zero Patches are used to remove bugs without changing the functionality or interface of a module."]
    #[inline(always)]
    #[must_use]
    pub fn hw_patch_level(&mut self) -> HW_PATCH_LEVEL_W<16> {
        HW_PATCH_LEVEL_W::new(self)
    }
    #[doc = "Bits 20:23 - 23:20\\]
4-bit binary encoding of the minor hardware revision number"]
    #[inline(always)]
    #[must_use]
    pub fn minor_hw_revision(&mut self) -> MINOR_HW_REVISION_W<20> {
        MINOR_HW_REVISION_W::new(self)
    }
    #[doc = "Bits 24:27 - 27:24\\]
4-bit binary encoding of the major hardware revision number"]
    #[inline(always)]
    #[must_use]
    pub fn major_hw_revision(&mut self) -> MAJOR_HW_REVISION_W<24> {
        MAJOR_HW_REVISION_W::new(self)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Ignore on read"]
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
#[doc = "PKA hardware revision register This register allows the host access to the hardware revision number of the PKA engine for software driver matching and diagnostic purposes. It is always located at the highest address in the access space of the module and contains an encoding of the EIP number (with its complement as signature) for recognition of the hardware module.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwrev](index.html) module"]
pub struct HWREV_SPEC;
impl crate::RegisterSpec for HWREV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwrev::R](R) reader structure"]
impl crate::Readable for HWREV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hwrev::W](W) writer structure"]
impl crate::Writable for HWREV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HWREV to value 0x0151_e31c"]
impl crate::Resettable for HWREV_SPEC {
    const RESET_VALUE: Self::Ux = 0x0151_e31c;
}
