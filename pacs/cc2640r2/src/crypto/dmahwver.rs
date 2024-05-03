#[doc = "Register `DMAHWVER` reader"]
pub type R = crate::R<DmahwverSpec>;
#[doc = "Register `DMAHWVER` writer"]
pub type W = crate::W<DmahwverSpec>;
#[doc = "Field `VER_NUM` reader - 7:0\\]
Version number of the DMA Controller (209)"]
pub type VerNumR = crate::FieldReader;
#[doc = "Field `VER_NUM` writer - 7:0\\]
Version number of the DMA Controller (209)"]
pub type VerNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `VER_NUM_COMPL` reader - 15:8\\]
Bit-by-bit complement of the VER_NUM field bits."]
pub type VerNumComplR = crate::FieldReader;
#[doc = "Field `VER_NUM_COMPL` writer - 15:8\\]
Bit-by-bit complement of the VER_NUM field bits."]
pub type VerNumComplW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HW_PATCH_LVL` reader - 19:16\\]
Patch level."]
pub type HwPatchLvlR = crate::FieldReader;
#[doc = "Field `HW_PATCH_LVL` writer - 19:16\\]
Patch level."]
pub type HwPatchLvlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HW_MINOR_VER` reader - 23:20\\]
Minor version number"]
pub type HwMinorVerR = crate::FieldReader;
#[doc = "Field `HW_MINOR_VER` writer - 23:20\\]
Minor version number"]
pub type HwMinorVerW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HW_MAJOR_VER` reader - 27:24\\]
Major version number"]
pub type HwMajorVerR = crate::FieldReader;
#[doc = "Field `HW_MAJOR_VER` writer - 27:24\\]
Major version number"]
pub type HwMajorVerW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED28` reader - 31:28\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved28R = crate::FieldReader;
#[doc = "Field `RESERVED28` writer - 31:28\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved28W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Version number of the DMA Controller (209)"]
    #[inline(always)]
    pub fn ver_num(&self) -> VerNumR {
        VerNumR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Bit-by-bit complement of the VER_NUM field bits."]
    #[inline(always)]
    pub fn ver_num_compl(&self) -> VerNumComplR {
        VerNumComplR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Patch level."]
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
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Version number of the DMA Controller (209)"]
    #[inline(always)]
    #[must_use]
    pub fn ver_num(&mut self) -> VerNumW<DmahwverSpec> {
        VerNumW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Bit-by-bit complement of the VER_NUM field bits."]
    #[inline(always)]
    #[must_use]
    pub fn ver_num_compl(&mut self) -> VerNumComplW<DmahwverSpec> {
        VerNumComplW::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Patch level."]
    #[inline(always)]
    #[must_use]
    pub fn hw_patch_lvl(&mut self) -> HwPatchLvlW<DmahwverSpec> {
        HwPatchLvlW::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Minor version number"]
    #[inline(always)]
    #[must_use]
    pub fn hw_minor_ver(&mut self) -> HwMinorVerW<DmahwverSpec> {
        HwMinorVerW::new(self, 20)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Major version number"]
    #[inline(always)]
    #[must_use]
    pub fn hw_major_ver(&mut self) -> HwMajorVerW<DmahwverSpec> {
        HwMajorVerW::new(self, 24)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved28(&mut self) -> Reserved28W<DmahwverSpec> {
        Reserved28W::new(self, 28)
    }
}
#[doc = "DMA Controller Version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmahwver::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmahwver::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
