#[doc = "Register `SRAMNSCADDR` reader"]
pub struct R(crate::R<SRAMNSCADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAMNSCADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAMNSCADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAMNSCADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRAMNSCADDR` writer"]
pub struct W(crate::W<SRAMNSCADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAMNSCADDR_SPEC>;
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
impl From<crate::W<SRAMNSCADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAMNSCADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 9:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED0` writer - 9:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SRAMNSCADDR_SPEC, u16, u16, 10, O>;
#[doc = "Field `BOUNDARY` reader - 18:10\\]
Non-Secure callable boundary address. Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
pub type BOUNDARY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BOUNDARY` writer - 18:10\\]
Non-Secure callable boundary address. Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
pub type BOUNDARY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SRAMNSCADDR_SPEC, u16, u16, 9, O>;
#[doc = "Field `RESERVED19` reader - 30:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED19_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED19` writer - 30:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED19_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SRAMNSCADDR_SPEC, u16, u16, 12, O>;
#[doc = "Field `PARITY` reader - 31:31\\]
Register parity bit"]
pub type PARITY_R = crate::BitReader<bool>;
#[doc = "Field `PARITY` writer - 31:31\\]
Register parity bit"]
pub type PARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRAMNSCADDR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:18 - 18:10\\]
Non-Secure callable boundary address. Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
    #[inline(always)]
    pub fn boundary(&self) -> BOUNDARY_R {
        BOUNDARY_R::new(((self.bits >> 10) & 0x01ff) as u16)
    }
    #[doc = "Bits 19:30 - 30:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved19(&self) -> RESERVED19_R {
        RESERVED19_R::new(((self.bits >> 19) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - 31:31\\]
Register parity bit"]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<0> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bits 10:18 - 18:10\\]
Non-Secure callable boundary address. Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn boundary(&mut self) -> BOUNDARY_W<10> {
        BOUNDARY_W::new(self)
    }
    #[doc = "Bits 19:30 - 30:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved19(&mut self) -> RESERVED19_W<19> {
        RESERVED19_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
Register parity bit"]
    #[inline(always)]
    #[must_use]
    pub fn parity(&mut self) -> PARITY_W<31> {
        PARITY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM Non-Secure Callable boundary Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sramnscaddr](index.html) module"]
pub struct SRAMNSCADDR_SPEC;
impl crate::RegisterSpec for SRAMNSCADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sramnscaddr::R](R) reader structure"]
impl crate::Readable for SRAMNSCADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sramnscaddr::W](W) writer structure"]
impl crate::Writable for SRAMNSCADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRAMNSCADDR to value 0"]
impl crate::Resettable for SRAMNSCADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
