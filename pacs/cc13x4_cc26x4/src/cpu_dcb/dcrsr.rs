#[doc = "Register `DCRSR` reader"]
pub struct R(crate::R<DCRSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCRSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCRSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCRSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCRSR` writer"]
pub struct W(crate::W<DCRSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCRSR_SPEC>;
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
impl From<crate::W<DCRSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCRSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGSEL` reader - 6:0\\]
Specifies the general-purpose register, special-purpose register, or FP register to transfer"]
pub type REGSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REGSEL` writer - 6:0\\]
Specifies the general-purpose register, special-purpose register, or FP register to transfer"]
pub type REGSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCRSR_SPEC, u8, u8, 7, O>;
#[doc = "Field `RESERVED7` reader - 15:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED7_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED7` writer - 15:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCRSR_SPEC, u16, u16, 9, O>;
#[doc = "Field `REGWnR` reader - 16:16\\]
Specifies the access type for the transfer"]
pub type REGWN_R_R = crate::BitReader<bool>;
#[doc = "Field `REGWnR` writer - 16:16\\]
Specifies the access type for the transfer"]
pub type REGWN_R_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCRSR_SPEC, bool, O>;
#[doc = "Field `RESERVED17` reader - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED17_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED17` writer - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED17_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCRSR_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Specifies the general-purpose register, special-purpose register, or FP register to transfer"]
    #[inline(always)]
    pub fn regsel(&self) -> REGSEL_R {
        REGSEL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:15 - 15:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new(((self.bits >> 7) & 0x01ff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Specifies the access type for the transfer"]
    #[inline(always)]
    pub fn regwn_r(&self) -> REGWN_R_R {
        REGWN_R_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> RESERVED17_R {
        RESERVED17_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Specifies the general-purpose register, special-purpose register, or FP register to transfer"]
    #[inline(always)]
    #[must_use]
    pub fn regsel(&mut self) -> REGSEL_W<0> {
        REGSEL_W::new(self)
    }
    #[doc = "Bits 7:15 - 15:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> RESERVED7_W<7> {
        RESERVED7_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
Specifies the access type for the transfer"]
    #[inline(always)]
    #[must_use]
    pub fn regwn_r(&mut self) -> REGWN_R_W<16> {
        REGWN_R_W::new(self)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved17(&mut self) -> RESERVED17_W<17> {
        RESERVED17_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "With the DCRDR, provides debug access to the general-purpose registers, special-purpose registers, and the FP extension registers. A write to the DCRSR specifies the register to transfer, whether the transfer is a read or write, and starts the transfer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcrsr](index.html) module"]
pub struct DCRSR_SPEC;
impl crate::RegisterSpec for DCRSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcrsr::R](R) reader structure"]
impl crate::Readable for DCRSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcrsr::W](W) writer structure"]
impl crate::Writable for DCRSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCRSR to value 0"]
impl crate::Resettable for DCRSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
