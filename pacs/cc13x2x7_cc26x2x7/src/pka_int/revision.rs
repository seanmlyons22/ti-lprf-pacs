#[doc = "Register `REVISION` reader"]
pub struct R(crate::R<REVISION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REVISION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REVISION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REVISION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REVISION` writer"]
pub struct W(crate::W<REVISION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REVISION_SPEC>;
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
impl From<crate::W<REVISION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REVISION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EIP_NUM` reader - 7:0\\]
These bits encode the AuthenTec EIP number for the EIP150"]
pub type EIP_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EIP_NUM` writer - 7:0\\]
These bits encode the AuthenTec EIP number for the EIP150"]
pub type EIP_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REVISION_SPEC, u8, u8, 8, O>;
#[doc = "Field `COMP_EIP_NUM` reader - 15:8\\]
These bits simply contain the complement of bits \\[7:0\\], used by a driver to ascertain that the EIP150 revision register is indeed read"]
pub type COMP_EIP_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP_EIP_NUM` writer - 15:8\\]
These bits simply contain the complement of bits \\[7:0\\], used by a driver to ascertain that the EIP150 revision register is indeed read"]
pub type COMP_EIP_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REVISION_SPEC, u8, u8, 8, O>;
#[doc = "Field `PATCH_LEVEL` reader - 19:16\\]
These bits encode the hardware patch level for this module they start at value 0 on the first release"]
pub type PATCH_LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PATCH_LEVEL` writer - 19:16\\]
These bits encode the hardware patch level for this module they start at value 0 on the first release"]
pub type PATCH_LEVEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REVISION_SPEC, u8, u8, 4, O>;
#[doc = "Field `MINOR_REVISION` reader - 23:20\\]
These bits encode the minor version number for this module"]
pub type MINOR_REVISION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MINOR_REVISION` writer - 23:20\\]
These bits encode the minor version number for this module"]
pub type MINOR_REVISION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, REVISION_SPEC, u8, u8, 4, O>;
#[doc = "Field `MAJOR_REVISION` reader - 27:24\\]
These bits encode the major version number for this module"]
pub type MAJOR_REVISION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAJOR_REVISION` writer - 27:24\\]
These bits encode the major version number for this module"]
pub type MAJOR_REVISION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, REVISION_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED28` reader - 31:28\\]
These bits should be ignored on read"]
pub type RESERVED28_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED28` writer - 31:28\\]
These bits should be ignored on read"]
pub type RESERVED28_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REVISION_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
These bits encode the AuthenTec EIP number for the EIP150"]
    #[inline(always)]
    pub fn eip_num(&self) -> EIP_NUM_R {
        EIP_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
These bits simply contain the complement of bits \\[7:0\\], used by a driver to ascertain that the EIP150 revision register is indeed read"]
    #[inline(always)]
    pub fn comp_eip_num(&self) -> COMP_EIP_NUM_R {
        COMP_EIP_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
These bits encode the hardware patch level for this module they start at value 0 on the first release"]
    #[inline(always)]
    pub fn patch_level(&self) -> PATCH_LEVEL_R {
        PATCH_LEVEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
These bits encode the minor version number for this module"]
    #[inline(always)]
    pub fn minor_revision(&self) -> MINOR_REVISION_R {
        MINOR_REVISION_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
These bits encode the major version number for this module"]
    #[inline(always)]
    pub fn major_revision(&self) -> MAJOR_REVISION_R {
        MAJOR_REVISION_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
These bits should be ignored on read"]
    #[inline(always)]
    pub fn reserved28(&self) -> RESERVED28_R {
        RESERVED28_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
These bits encode the AuthenTec EIP number for the EIP150"]
    #[inline(always)]
    #[must_use]
    pub fn eip_num(&mut self) -> EIP_NUM_W<0> {
        EIP_NUM_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
These bits simply contain the complement of bits \\[7:0\\], used by a driver to ascertain that the EIP150 revision register is indeed read"]
    #[inline(always)]
    #[must_use]
    pub fn comp_eip_num(&mut self) -> COMP_EIP_NUM_W<8> {
        COMP_EIP_NUM_W::new(self)
    }
    #[doc = "Bits 16:19 - 19:16\\]
These bits encode the hardware patch level for this module they start at value 0 on the first release"]
    #[inline(always)]
    #[must_use]
    pub fn patch_level(&mut self) -> PATCH_LEVEL_W<16> {
        PATCH_LEVEL_W::new(self)
    }
    #[doc = "Bits 20:23 - 23:20\\]
These bits encode the minor version number for this module"]
    #[inline(always)]
    #[must_use]
    pub fn minor_revision(&mut self) -> MINOR_REVISION_W<20> {
        MINOR_REVISION_W::new(self)
    }
    #[doc = "Bits 24:27 - 27:24\\]
These bits encode the major version number for this module"]
    #[inline(always)]
    #[must_use]
    pub fn major_revision(&mut self) -> MAJOR_REVISION_W<24> {
        MAJOR_REVISION_W::new(self)
    }
    #[doc = "Bits 28:31 - 31:28\\]
These bits should be ignored on read"]
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
#[doc = "PKA hardware revision register This register allows the host access to the hardware revision number of the PKA engine for software driver matching and diagnostic purposes. It is always located at the highest address in the access space of the module and contains an encoding of the EIP number (with its complement as signature) for recognition of the hardware module.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [revision](index.html) module"]
pub struct REVISION_SPEC;
impl crate::RegisterSpec for REVISION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [revision::R](R) reader structure"]
impl crate::Readable for REVISION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [revision::W](W) writer structure"]
impl crate::Writable for REVISION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REVISION to value 0x0200_6996"]
impl crate::Resettable for REVISION_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200_6996;
}
