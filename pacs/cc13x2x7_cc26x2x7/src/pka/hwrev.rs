#[doc = "Register `HWREV` reader"]
pub type R = crate::R<HwrevSpec>;
#[doc = "Register `HWREV` writer"]
pub type W = crate::W<HwrevSpec>;
#[doc = "Field `BASIC_EIP_NUMBER` reader - 7:0\\]
8-bit binary encoding of the EIP number, EIP-28 gives 0x1C"]
pub type BasicEipNumberR = crate::FieldReader;
#[doc = "Field `COMPLEMENT_OF_BASIC_EIP_NUMBER` reader - 15:8\\]
Bit-by-bit logic complement of bits \\[7:0\\], EIP-28 gives 0xE3"]
pub type ComplementOfBasicEipNumberR = crate::FieldReader;
#[doc = "Field `HW_PATCH_LEVEL` reader - 19:16\\]
4-bit binary encoding of the hardware patch level, initial release will carry value zero Patches are used to remove bugs without changing the functionality or interface of a module."]
pub type HwPatchLevelR = crate::FieldReader;
#[doc = "Field `MINOR_HW_REVISION` reader - 23:20\\]
4-bit binary encoding of the minor hardware revision number"]
pub type MinorHwRevisionR = crate::FieldReader;
#[doc = "Field `MAJOR_HW_REVISION` reader - 27:24\\]
4-bit binary encoding of the major hardware revision number"]
pub type MajorHwRevisionR = crate::FieldReader;
#[doc = "Field `RESERVED28` reader - 31:28\\]
Ignore on read"]
pub type Reserved28R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
8-bit binary encoding of the EIP number, EIP-28 gives 0x1C"]
    #[inline(always)]
    pub fn basic_eip_number(&self) -> BasicEipNumberR {
        BasicEipNumberR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Bit-by-bit logic complement of bits \\[7:0\\], EIP-28 gives 0xE3"]
    #[inline(always)]
    pub fn complement_of_basic_eip_number(&self) -> ComplementOfBasicEipNumberR {
        ComplementOfBasicEipNumberR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
4-bit binary encoding of the hardware patch level, initial release will carry value zero Patches are used to remove bugs without changing the functionality or interface of a module."]
    #[inline(always)]
    pub fn hw_patch_level(&self) -> HwPatchLevelR {
        HwPatchLevelR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
4-bit binary encoding of the minor hardware revision number"]
    #[inline(always)]
    pub fn minor_hw_revision(&self) -> MinorHwRevisionR {
        MinorHwRevisionR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
4-bit binary encoding of the major hardware revision number"]
    #[inline(always)]
    pub fn major_hw_revision(&self) -> MajorHwRevisionR {
        MajorHwRevisionR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Ignore on read"]
    #[inline(always)]
    pub fn reserved28(&self) -> Reserved28R {
        Reserved28R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "PKA hardware revision register This register allows the host access to the hardware revision number of the PKA engine for software driver matching and diagnostic purposes. It is always located at the highest address in the access space of the module and contains an encoding of the EIP number (with its complement as signature) for recognition of the hardware module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwrev::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwrev::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwrevSpec;
impl crate::RegisterSpec for HwrevSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwrev::R`](R) reader structure"]
impl crate::Readable for HwrevSpec {}
#[doc = "`write(|w| ..)` method takes [`hwrev::W`](W) writer structure"]
impl crate::Writable for HwrevSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWREV to value 0x0151_e31c"]
impl crate::Resettable for HwrevSpec {
    const RESET_VALUE: u32 = 0x0151_e31c;
}
