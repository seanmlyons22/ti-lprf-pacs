#[doc = "Register `REVISION` reader"]
pub type R = crate::R<RevisionSpec>;
#[doc = "Register `REVISION` writer"]
pub type W = crate::W<RevisionSpec>;
#[doc = "Field `EIP_NUM` reader - 7:0\\]
These bits encode the AuthenTec EIP number for the EIP150"]
pub type EipNumR = crate::FieldReader;
#[doc = "Field `EIP_NUM` writer - 7:0\\]
These bits encode the AuthenTec EIP number for the EIP150"]
pub type EipNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `COMP_EIP_NUM` reader - 15:8\\]
These bits simply contain the complement of bits \\[7:0\\], used by a driver to ascertain that the EIP150 revision register is indeed read"]
pub type CompEipNumR = crate::FieldReader;
#[doc = "Field `COMP_EIP_NUM` writer - 15:8\\]
These bits simply contain the complement of bits \\[7:0\\], used by a driver to ascertain that the EIP150 revision register is indeed read"]
pub type CompEipNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PATCH_LEVEL` reader - 19:16\\]
These bits encode the hardware patch level for this module they start at value 0 on the first release"]
pub type PatchLevelR = crate::FieldReader;
#[doc = "Field `PATCH_LEVEL` writer - 19:16\\]
These bits encode the hardware patch level for this module they start at value 0 on the first release"]
pub type PatchLevelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MINOR_REVISION` reader - 23:20\\]
These bits encode the minor version number for this module"]
pub type MinorRevisionR = crate::FieldReader;
#[doc = "Field `MINOR_REVISION` writer - 23:20\\]
These bits encode the minor version number for this module"]
pub type MinorRevisionW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MAJOR_REVISION` reader - 27:24\\]
These bits encode the major version number for this module"]
pub type MajorRevisionR = crate::FieldReader;
#[doc = "Field `MAJOR_REVISION` writer - 27:24\\]
These bits encode the major version number for this module"]
pub type MajorRevisionW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED28` reader - 31:28\\]
These bits should be ignored on read"]
pub type Reserved28R = crate::FieldReader;
#[doc = "Field `RESERVED28` writer - 31:28\\]
These bits should be ignored on read"]
pub type Reserved28W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
These bits encode the AuthenTec EIP number for the EIP150"]
    #[inline(always)]
    pub fn eip_num(&self) -> EipNumR {
        EipNumR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
These bits simply contain the complement of bits \\[7:0\\], used by a driver to ascertain that the EIP150 revision register is indeed read"]
    #[inline(always)]
    pub fn comp_eip_num(&self) -> CompEipNumR {
        CompEipNumR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
These bits encode the hardware patch level for this module they start at value 0 on the first release"]
    #[inline(always)]
    pub fn patch_level(&self) -> PatchLevelR {
        PatchLevelR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
These bits encode the minor version number for this module"]
    #[inline(always)]
    pub fn minor_revision(&self) -> MinorRevisionR {
        MinorRevisionR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
These bits encode the major version number for this module"]
    #[inline(always)]
    pub fn major_revision(&self) -> MajorRevisionR {
        MajorRevisionR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
These bits should be ignored on read"]
    #[inline(always)]
    pub fn reserved28(&self) -> Reserved28R {
        Reserved28R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
These bits encode the AuthenTec EIP number for the EIP150"]
    #[inline(always)]
    #[must_use]
    pub fn eip_num(&mut self) -> EipNumW<RevisionSpec> {
        EipNumW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
These bits simply contain the complement of bits \\[7:0\\], used by a driver to ascertain that the EIP150 revision register is indeed read"]
    #[inline(always)]
    #[must_use]
    pub fn comp_eip_num(&mut self) -> CompEipNumW<RevisionSpec> {
        CompEipNumW::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
These bits encode the hardware patch level for this module they start at value 0 on the first release"]
    #[inline(always)]
    #[must_use]
    pub fn patch_level(&mut self) -> PatchLevelW<RevisionSpec> {
        PatchLevelW::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
These bits encode the minor version number for this module"]
    #[inline(always)]
    #[must_use]
    pub fn minor_revision(&mut self) -> MinorRevisionW<RevisionSpec> {
        MinorRevisionW::new(self, 20)
    }
    #[doc = "Bits 24:27 - 27:24\\]
These bits encode the major version number for this module"]
    #[inline(always)]
    #[must_use]
    pub fn major_revision(&mut self) -> MajorRevisionW<RevisionSpec> {
        MajorRevisionW::new(self, 24)
    }
    #[doc = "Bits 28:31 - 31:28\\]
These bits should be ignored on read"]
    #[inline(always)]
    #[must_use]
    pub fn reserved28(&mut self) -> Reserved28W<RevisionSpec> {
        Reserved28W::new(self, 28)
    }
}
#[doc = "PKA hardware revision register This register allows the host access to the hardware revision number of the PKA engine for software driver matching and diagnostic purposes. It is always located at the highest address in the access space of the module and contains an encoding of the EIP number (with its complement as signature) for recognition of the hardware module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`revision::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`revision::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RevisionSpec;
impl crate::RegisterSpec for RevisionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`revision::R`](R) reader structure"]
impl crate::Readable for RevisionSpec {}
#[doc = "`write(|w| ..)` method takes [`revision::W`](W) writer structure"]
impl crate::Writable for RevisionSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REVISION to value 0x0201_6996"]
impl crate::Resettable for RevisionSpec {
    const RESET_VALUE: u32 = 0x0201_6996;
}
