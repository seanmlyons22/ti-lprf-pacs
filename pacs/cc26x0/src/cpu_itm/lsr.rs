#[doc = "Register `LSR` reader"]
pub struct R(crate::R<LSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LSR` writer"]
pub struct W(crate::W<LSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LSR_SPEC>;
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
impl From<crate::W<LSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRESENT` reader - 0:0\\]
Indicates that a lock mechanism exists for this component."]
pub type PRESENT_R = crate::BitReader<bool>;
#[doc = "Field `PRESENT` writer - 0:0\\]
Indicates that a lock mechanism exists for this component."]
pub type PRESENT_W<'a, const O: u8> = crate::BitWriter<'a, u32, LSR_SPEC, bool, O>;
#[doc = "Field `ACCESS` reader - 1:1\\]
Write access to component is blocked. All writes are ignored, reads are permitted."]
pub type ACCESS_R = crate::BitReader<bool>;
#[doc = "Field `ACCESS` writer - 1:1\\]
Write access to component is blocked. All writes are ignored, reads are permitted."]
pub type ACCESS_W<'a, const O: u8> = crate::BitWriter<'a, u32, LSR_SPEC, bool, O>;
#[doc = "Field `BYTEACC` reader - 2:2\\]
Reads 0 which means 8-bit lock access is not be implemented."]
pub type BYTEACC_R = crate::BitReader<bool>;
#[doc = "Field `BYTEACC` writer - 2:2\\]
Reads 0 which means 8-bit lock access is not be implemented."]
pub type BYTEACC_W<'a, const O: u8> = crate::BitWriter<'a, u32, LSR_SPEC, bool, O>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LSR_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates that a lock mechanism exists for this component."]
    #[inline(always)]
    pub fn present(&self) -> PRESENT_R {
        PRESENT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Write access to component is blocked. All writes are ignored, reads are permitted."]
    #[inline(always)]
    pub fn access(&self) -> ACCESS_R {
        ACCESS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Reads 0 which means 8-bit lock access is not be implemented."]
    #[inline(always)]
    pub fn byteacc(&self) -> BYTEACC_R {
        BYTEACC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates that a lock mechanism exists for this component."]
    #[inline(always)]
    #[must_use]
    pub fn present(&mut self) -> PRESENT_W<0> {
        PRESENT_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Write access to component is blocked. All writes are ignored, reads are permitted."]
    #[inline(always)]
    #[must_use]
    pub fn access(&mut self) -> ACCESS_W<1> {
        ACCESS_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Reads 0 which means 8-bit lock access is not be implemented."]
    #[inline(always)]
    #[must_use]
    pub fn byteacc(&mut self) -> BYTEACC_W<2> {
        BYTEACC_W::new(self)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<3> {
        RESERVED3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Lock Status Use this register to enable write accesses to the Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsr](index.html) module"]
pub struct LSR_SPEC;
impl crate::RegisterSpec for LSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lsr::R](R) reader structure"]
impl crate::Readable for LSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lsr::W](W) writer structure"]
impl crate::Writable for LSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LSR to value 0x03"]
impl crate::Resettable for LSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
