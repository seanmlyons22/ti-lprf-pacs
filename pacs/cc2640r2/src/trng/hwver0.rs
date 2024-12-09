#[doc = "Register `HWVER0` reader"]
pub type R = crate::R<Hwver0Spec>;
#[doc = "Register `HWVER0` writer"]
pub type W = crate::W<Hwver0Spec>;
#[doc = "Field `EIP_NUM` reader - 7:0\\]
8 bits binary encoding of the module number. This TRNG gives 0x4B."]
pub type EipNumR = crate::FieldReader;
#[doc = "Field `EIP_NUM_COMPL` reader - 15:8\\]
Bit-by-bit logic complement of bits \\[7:0\\]. This TRNG gives 0xB4."]
pub type EipNumComplR = crate::FieldReader;
#[doc = "Field `HW_PATCH_LVL` reader - 19:16\\]
4 bits binary encoding of the hardware patch level, initial release will carry value zero."]
pub type HwPatchLvlR = crate::FieldReader;
#[doc = "Field `HW_MINOR_VER` reader - 23:20\\]
4 bits binary encoding of the minor hardware revision number."]
pub type HwMinorVerR = crate::FieldReader;
#[doc = "Field `HW_MAJOR_VER` reader - 27:24\\]
4 bits binary encoding of the major hardware revision number."]
pub type HwMajorVerR = crate::FieldReader;
#[doc = "Field `RESERVED28` reader - 31:28\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved28R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
8 bits binary encoding of the module number. This TRNG gives 0x4B."]
    #[inline(always)]
    pub fn eip_num(&self) -> EipNumR {
        EipNumR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Bit-by-bit logic complement of bits \\[7:0\\]. This TRNG gives 0xB4."]
    #[inline(always)]
    pub fn eip_num_compl(&self) -> EipNumComplR {
        EipNumComplR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
4 bits binary encoding of the hardware patch level, initial release will carry value zero."]
    #[inline(always)]
    pub fn hw_patch_lvl(&self) -> HwPatchLvlR {
        HwPatchLvlR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
4 bits binary encoding of the minor hardware revision number."]
    #[inline(always)]
    pub fn hw_minor_ver(&self) -> HwMinorVerR {
        HwMinorVerR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
4 bits binary encoding of the major hardware revision number."]
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
#[doc = "HW Version 0 EIP Number And Core Revision\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwver0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwver0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hwver0Spec;
impl crate::RegisterSpec for Hwver0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwver0::R`](R) reader structure"]
impl crate::Readable for Hwver0Spec {}
#[doc = "`write(|w| ..)` method takes [`hwver0::W`](W) writer structure"]
impl crate::Writable for Hwver0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWVER0 to value 0x0200_b44b"]
impl crate::Resettable for Hwver0Spec {
    const RESET_VALUE: u32 = 0x0200_b44b;
}
