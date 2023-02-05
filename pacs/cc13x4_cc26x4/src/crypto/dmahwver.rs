#[doc = "Register `DMAHWVER` reader"]
pub struct R(crate::R<DMAHWVER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAHWVER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAHWVER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAHWVER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAHWVER` writer"]
pub struct W(crate::W<DMAHWVER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAHWVER_SPEC>;
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
impl From<crate::W<DMAHWVER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAHWVER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EIP_NUMBER` reader - 7:0\\]
Binary encoding of the EIP-number of this DMA controller (209)"]
pub type EIP_NUMBER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EIP_NUMBER` writer - 7:0\\]
Binary encoding of the EIP-number of this DMA controller (209)"]
pub type EIP_NUMBER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMAHWVER_SPEC, u8, u8, 8, O>;
#[doc = "Field `EIP_NUMBER_COMPL` reader - 15:8\\]
Bit-by-bit complement of the EIP_NUMBER field bits."]
pub type EIP_NUMBER_COMPL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EIP_NUMBER_COMPL` writer - 15:8\\]
Bit-by-bit complement of the EIP_NUMBER field bits."]
pub type EIP_NUMBER_COMPL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMAHWVER_SPEC, u8, u8, 8, O>;
#[doc = "Field `HW_PATCH_LEVEL` reader - 19:16\\]
Patch level Starts at 0 at first delivery of this version"]
pub type HW_PATCH_LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HW_PATCH_LEVEL` writer - 19:16\\]
Patch level Starts at 0 at first delivery of this version"]
pub type HW_PATCH_LEVEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMAHWVER_SPEC, u8, u8, 4, O>;
#[doc = "Field `HW_MINOR_VERSION` reader - 23:20\\]
Minor version number"]
pub type HW_MINOR_VERSION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HW_MINOR_VERSION` writer - 23:20\\]
Minor version number"]
pub type HW_MINOR_VERSION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMAHWVER_SPEC, u8, u8, 4, O>;
#[doc = "Field `HW_MAJOR_VERSION` reader - 27:24\\]
Major version number"]
pub type HW_MAJOR_VERSION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HW_MAJOR_VERSION` writer - 27:24\\]
Major version number"]
pub type HW_MAJOR_VERSION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMAHWVER_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED28` reader - 31:28\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED28_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED28` writer - 31:28\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED28_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMAHWVER_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Binary encoding of the EIP-number of this DMA controller (209)"]
    #[inline(always)]
    pub fn eip_number(&self) -> EIP_NUMBER_R {
        EIP_NUMBER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Bit-by-bit complement of the EIP_NUMBER field bits."]
    #[inline(always)]
    pub fn eip_number_compl(&self) -> EIP_NUMBER_COMPL_R {
        EIP_NUMBER_COMPL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Patch level Starts at 0 at first delivery of this version"]
    #[inline(always)]
    pub fn hw_patch_level(&self) -> HW_PATCH_LEVEL_R {
        HW_PATCH_LEVEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Minor version number"]
    #[inline(always)]
    pub fn hw_minor_version(&self) -> HW_MINOR_VERSION_R {
        HW_MINOR_VERSION_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Major version number"]
    #[inline(always)]
    pub fn hw_major_version(&self) -> HW_MAJOR_VERSION_R {
        HW_MAJOR_VERSION_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved28(&self) -> RESERVED28_R {
        RESERVED28_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Binary encoding of the EIP-number of this DMA controller (209)"]
    #[inline(always)]
    #[must_use]
    pub fn eip_number(&mut self) -> EIP_NUMBER_W<0> {
        EIP_NUMBER_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Bit-by-bit complement of the EIP_NUMBER field bits."]
    #[inline(always)]
    #[must_use]
    pub fn eip_number_compl(&mut self) -> EIP_NUMBER_COMPL_W<8> {
        EIP_NUMBER_COMPL_W::new(self)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Patch level Starts at 0 at first delivery of this version"]
    #[inline(always)]
    #[must_use]
    pub fn hw_patch_level(&mut self) -> HW_PATCH_LEVEL_W<16> {
        HW_PATCH_LEVEL_W::new(self)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Minor version number"]
    #[inline(always)]
    #[must_use]
    pub fn hw_minor_version(&mut self) -> HW_MINOR_VERSION_W<20> {
        HW_MINOR_VERSION_W::new(self)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Major version number"]
    #[inline(always)]
    #[must_use]
    pub fn hw_major_version(&mut self) -> HW_MAJOR_VERSION_W<24> {
        HW_MAJOR_VERSION_W::new(self)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
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
#[doc = "DMAC Version This register contains an indication (or signature) of the EIP type of this DMAC, as well as the hardware version/patch numbers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmahwver](index.html) module"]
pub struct DMAHWVER_SPEC;
impl crate::RegisterSpec for DMAHWVER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmahwver::R](R) reader structure"]
impl crate::Readable for DMAHWVER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmahwver::W](W) writer structure"]
impl crate::Writable for DMAHWVER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAHWVER to value 0x0101_2ed1"]
impl crate::Resettable for DMAHWVER_SPEC {
    const RESET_VALUE: Self::Ux = 0x0101_2ed1;
}
