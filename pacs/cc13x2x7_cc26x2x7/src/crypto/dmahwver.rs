#[doc = "Register `DMAHWVER` reader"]
pub type R = crate::R<DmahwverSpec>;
#[doc = "Register `DMAHWVER` writer"]
pub type W = crate::W<DmahwverSpec>;
#[doc = "Field `EIP_NUMBER` reader - 7:0\\]
Binary encoding of the EIP-number of this DMA controller (209)"]
pub type EipNumberR = crate::FieldReader;
#[doc = "Field `EIP_NUMBER` writer - 7:0\\]
Binary encoding of the EIP-number of this DMA controller (209)"]
pub type EipNumberW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EIP_NUMBER_COMPL` reader - 15:8\\]
Bit-by-bit complement of the EIP_NUMBER field bits."]
pub type EipNumberComplR = crate::FieldReader;
#[doc = "Field `EIP_NUMBER_COMPL` writer - 15:8\\]
Bit-by-bit complement of the EIP_NUMBER field bits."]
pub type EipNumberComplW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HW_PATCH_LEVEL` reader - 19:16\\]
Patch level Starts at 0 at first delivery of this version"]
pub type HwPatchLevelR = crate::FieldReader;
#[doc = "Field `HW_PATCH_LEVEL` writer - 19:16\\]
Patch level Starts at 0 at first delivery of this version"]
pub type HwPatchLevelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HW_MINOR_VERSION` reader - 23:20\\]
Minor version number"]
pub type HwMinorVersionR = crate::FieldReader;
#[doc = "Field `HW_MINOR_VERSION` writer - 23:20\\]
Minor version number"]
pub type HwMinorVersionW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HW_MAJOR_VERSION` reader - 27:24\\]
Major version number"]
pub type HwMajorVersionR = crate::FieldReader;
#[doc = "Field `HW_MAJOR_VERSION` writer - 27:24\\]
Major version number"]
pub type HwMajorVersionW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED28` reader - 31:28\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved28R = crate::FieldReader;
#[doc = "Field `RESERVED28` writer - 31:28\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved28W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Binary encoding of the EIP-number of this DMA controller (209)"]
    #[inline(always)]
    pub fn eip_number(&self) -> EipNumberR {
        EipNumberR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Bit-by-bit complement of the EIP_NUMBER field bits."]
    #[inline(always)]
    pub fn eip_number_compl(&self) -> EipNumberComplR {
        EipNumberComplR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Patch level Starts at 0 at first delivery of this version"]
    #[inline(always)]
    pub fn hw_patch_level(&self) -> HwPatchLevelR {
        HwPatchLevelR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Minor version number"]
    #[inline(always)]
    pub fn hw_minor_version(&self) -> HwMinorVersionR {
        HwMinorVersionR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Major version number"]
    #[inline(always)]
    pub fn hw_major_version(&self) -> HwMajorVersionR {
        HwMajorVersionR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved28(&self) -> Reserved28R {
        Reserved28R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Binary encoding of the EIP-number of this DMA controller (209)"]
    #[inline(always)]
    #[must_use]
    pub fn eip_number(&mut self) -> EipNumberW<DmahwverSpec> {
        EipNumberW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Bit-by-bit complement of the EIP_NUMBER field bits."]
    #[inline(always)]
    #[must_use]
    pub fn eip_number_compl(&mut self) -> EipNumberComplW<DmahwverSpec> {
        EipNumberComplW::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Patch level Starts at 0 at first delivery of this version"]
    #[inline(always)]
    #[must_use]
    pub fn hw_patch_level(&mut self) -> HwPatchLevelW<DmahwverSpec> {
        HwPatchLevelW::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Minor version number"]
    #[inline(always)]
    #[must_use]
    pub fn hw_minor_version(&mut self) -> HwMinorVersionW<DmahwverSpec> {
        HwMinorVersionW::new(self, 20)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Major version number"]
    #[inline(always)]
    #[must_use]
    pub fn hw_major_version(&mut self) -> HwMajorVersionW<DmahwverSpec> {
        HwMajorVersionW::new(self, 24)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved28(&mut self) -> Reserved28W<DmahwverSpec> {
        Reserved28W::new(self, 28)
    }
}
#[doc = "DMAC Version This register contains an indication (or signature) of the EIP type of this DMAC, as well as the hardware version/patch numbers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmahwver::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmahwver::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmahwverSpec;
impl crate::RegisterSpec for DmahwverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmahwver::R`](R) reader structure"]
impl crate::Readable for DmahwverSpec {}
#[doc = "`write(|w| ..)` method takes [`dmahwver::W`](W) writer structure"]
impl crate::Writable for DmahwverSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAHWVER to value 0x0101_2ed1"]
impl crate::Resettable for DmahwverSpec {
    const RESET_VALUE: u32 = 0x0101_2ed1;
}
