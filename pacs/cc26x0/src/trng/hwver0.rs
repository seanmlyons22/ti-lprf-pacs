#[doc = "Register `HWVER0` reader"]
pub struct R(crate::R<HWVER0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWVER0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWVER0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWVER0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HWVER0` writer"]
pub struct W(crate::W<HWVER0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HWVER0_SPEC>;
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
impl From<crate::W<HWVER0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HWVER0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EIP_NUM` reader - 7:0\\]
8 bits binary encoding of the module number. This TRNG gives 0x4B."]
pub type EIP_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EIP_NUM` writer - 7:0\\]
8 bits binary encoding of the module number. This TRNG gives 0x4B."]
pub type EIP_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWVER0_SPEC, u8, u8, 8, O>;
#[doc = "Field `EIP_NUM_COMPL` reader - 15:8\\]
Bit-by-bit logic complement of bits \\[7:0\\]. This TRNG gives 0xB4."]
pub type EIP_NUM_COMPL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EIP_NUM_COMPL` writer - 15:8\\]
Bit-by-bit logic complement of bits \\[7:0\\]. This TRNG gives 0xB4."]
pub type EIP_NUM_COMPL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWVER0_SPEC, u8, u8, 8, O>;
#[doc = "Field `HW_PATCH_LVL` reader - 19:16\\]
4 bits binary encoding of the hardware patch level, initial release will carry value zero."]
pub type HW_PATCH_LVL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HW_PATCH_LVL` writer - 19:16\\]
4 bits binary encoding of the hardware patch level, initial release will carry value zero."]
pub type HW_PATCH_LVL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWVER0_SPEC, u8, u8, 4, O>;
#[doc = "Field `HW_MINOR_VER` reader - 23:20\\]
4 bits binary encoding of the minor hardware revision number."]
pub type HW_MINOR_VER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HW_MINOR_VER` writer - 23:20\\]
4 bits binary encoding of the minor hardware revision number."]
pub type HW_MINOR_VER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWVER0_SPEC, u8, u8, 4, O>;
#[doc = "Field `HW_MAJOR_VER` reader - 27:24\\]
4 bits binary encoding of the major hardware revision number."]
pub type HW_MAJOR_VER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HW_MAJOR_VER` writer - 27:24\\]
4 bits binary encoding of the major hardware revision number."]
pub type HW_MAJOR_VER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWVER0_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED28` reader - 31:28\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED28_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED28` writer - 31:28\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED28_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWVER0_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
8 bits binary encoding of the module number. This TRNG gives 0x4B."]
    #[inline(always)]
    pub fn eip_num(&self) -> EIP_NUM_R {
        EIP_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Bit-by-bit logic complement of bits \\[7:0\\]. This TRNG gives 0xB4."]
    #[inline(always)]
    pub fn eip_num_compl(&self) -> EIP_NUM_COMPL_R {
        EIP_NUM_COMPL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
4 bits binary encoding of the hardware patch level, initial release will carry value zero."]
    #[inline(always)]
    pub fn hw_patch_lvl(&self) -> HW_PATCH_LVL_R {
        HW_PATCH_LVL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
4 bits binary encoding of the minor hardware revision number."]
    #[inline(always)]
    pub fn hw_minor_ver(&self) -> HW_MINOR_VER_R {
        HW_MINOR_VER_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
4 bits binary encoding of the major hardware revision number."]
    #[inline(always)]
    pub fn hw_major_ver(&self) -> HW_MAJOR_VER_R {
        HW_MAJOR_VER_R::new(((self.bits >> 24) & 0x0f) as u8)
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
8 bits binary encoding of the module number. This TRNG gives 0x4B."]
    #[inline(always)]
    #[must_use]
    pub fn eip_num(&mut self) -> EIP_NUM_W<0> {
        EIP_NUM_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Bit-by-bit logic complement of bits \\[7:0\\]. This TRNG gives 0xB4."]
    #[inline(always)]
    #[must_use]
    pub fn eip_num_compl(&mut self) -> EIP_NUM_COMPL_W<8> {
        EIP_NUM_COMPL_W::new(self)
    }
    #[doc = "Bits 16:19 - 19:16\\]
4 bits binary encoding of the hardware patch level, initial release will carry value zero."]
    #[inline(always)]
    #[must_use]
    pub fn hw_patch_lvl(&mut self) -> HW_PATCH_LVL_W<16> {
        HW_PATCH_LVL_W::new(self)
    }
    #[doc = "Bits 20:23 - 23:20\\]
4 bits binary encoding of the minor hardware revision number."]
    #[inline(always)]
    #[must_use]
    pub fn hw_minor_ver(&mut self) -> HW_MINOR_VER_W<20> {
        HW_MINOR_VER_W::new(self)
    }
    #[doc = "Bits 24:27 - 27:24\\]
4 bits binary encoding of the major hardware revision number."]
    #[inline(always)]
    #[must_use]
    pub fn hw_major_ver(&mut self) -> HW_MAJOR_VER_W<24> {
        HW_MAJOR_VER_W::new(self)
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
#[doc = "HW Version 0 EIP Number And Core Revision\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwver0](index.html) module"]
pub struct HWVER0_SPEC;
impl crate::RegisterSpec for HWVER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwver0::R](R) reader structure"]
impl crate::Readable for HWVER0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hwver0::W](W) writer structure"]
impl crate::Writable for HWVER0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HWVER0 to value 0x0200_b44b"]
impl crate::Resettable for HWVER0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200_b44b;
}
