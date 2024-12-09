#[doc = "Register `HWVER` reader"]
pub type R = crate::R<HwverSpec>;
#[doc = "Register `HWVER` writer"]
pub type W = crate::W<HwverSpec>;
#[doc = "Field `VER_NUM` reader - 7:0\\]
These bits encode the EIP number for the EIP-120t, this field contains the value 120 (decimal) or 0x78."]
pub type VerNumR = crate::FieldReader;
#[doc = "Field `VER_NUM_COMPL` reader - 15:8\\]
These bits simply contain the complement of bits \\[7:0\\]
(0x87), used by a driver to ascertain that the EIP-120t register is indeed read."]
pub type VerNumComplR = crate::FieldReader;
#[doc = "Field `HW_PATCH_LVL` reader - 19:16\\]
Patch level Starts at 0 at first delivery of this version"]
pub type HwPatchLvlR = crate::FieldReader;
#[doc = "Field `HW_MINOR_VER` reader - 23:20\\]
Minor version number"]
pub type HwMinorVerR = crate::FieldReader;
#[doc = "Field `HW_MAJOR_VER` reader - 27:24\\]
Major version number"]
pub type HwMajorVerR = crate::FieldReader;
#[doc = "Field `RESERVED28` reader - 31:28\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved28R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
These bits encode the EIP number for the EIP-120t, this field contains the value 120 (decimal) or 0x78."]
    #[inline(always)]
    pub fn ver_num(&self) -> VerNumR {
        VerNumR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
These bits simply contain the complement of bits \\[7:0\\]
(0x87), used by a driver to ascertain that the EIP-120t register is indeed read."]
    #[inline(always)]
    pub fn ver_num_compl(&self) -> VerNumComplR {
        VerNumComplR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Patch level Starts at 0 at first delivery of this version"]
    #[inline(always)]
    pub fn hw_patch_lvl(&self) -> HwPatchLvlR {
        HwPatchLvlR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Minor version number"]
    #[inline(always)]
    pub fn hw_minor_ver(&self) -> HwMinorVerR {
        HwMinorVerR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Major version number"]
    #[inline(always)]
    pub fn hw_major_ver(&self) -> HwMajorVerR {
        HwMajorVerR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved28(&self) -> Reserved28R {
        Reserved28R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Hardware Version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwver::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwver::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwverSpec;
impl crate::RegisterSpec for HwverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwver::R`](R) reader structure"]
impl crate::Readable for HwverSpec {}
#[doc = "`write(|w| ..)` method takes [`hwver::W`](W) writer structure"]
impl crate::Writable for HwverSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWVER to value 0x9200_8778"]
impl crate::Resettable for HwverSpec {
    const RESET_VALUE: u32 = 0x9200_8778;
}
