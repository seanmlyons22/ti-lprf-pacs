#[doc = "Register `NVMNSADDR` reader"]
pub struct R(crate::R<NVMNSADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVMNSADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVMNSADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVMNSADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVMNSADDR` writer"]
pub struct W(crate::W<NVMNSADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVMNSADDR_SPEC>;
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
impl From<crate::W<NVMNSADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVMNSADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 12:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED0` writer - 12:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, NVMNSADDR_SPEC, u16, u16, 13, O>;
#[doc = "Field `BOUNDARY` reader - 19:13\\]
Non-Secure boundary address. Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
pub type BOUNDARY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BOUNDARY` writer - 19:13\\]
Non-Secure boundary address. Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
pub type BOUNDARY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVMNSADDR_SPEC, u8, u8, 7, O>;
#[doc = "Field `BOUNDARY_MSB` reader - 20:20\\]
Non-Secure boundary address MSB HW controlled."]
pub type BOUNDARY_MSB_R = crate::BitReader<bool>;
#[doc = "Field `BOUNDARY_MSB` writer - 20:20\\]
Non-Secure boundary address MSB HW controlled."]
pub type BOUNDARY_MSB_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVMNSADDR_SPEC, bool, O>;
#[doc = "Field `RESERVED21` reader - 30:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED21_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED21` writer - 30:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED21_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, NVMNSADDR_SPEC, u16, u16, 10, O>;
#[doc = "Field `PARITY` reader - 31:31\\]
Register parity bit"]
pub type PARITY_R = crate::BitReader<bool>;
#[doc = "Field `PARITY` writer - 31:31\\]
Register parity bit"]
pub type PARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVMNSADDR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:12 - 12:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:19 - 19:13\\]
Non-Secure boundary address. Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
    #[inline(always)]
    pub fn boundary(&self) -> BOUNDARY_R {
        BOUNDARY_R::new(((self.bits >> 13) & 0x7f) as u8)
    }
    #[doc = "Bit 20 - 20:20\\]
Non-Secure boundary address MSB HW controlled."]
    #[inline(always)]
    pub fn boundary_msb(&self) -> BOUNDARY_MSB_R {
        BOUNDARY_MSB_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:30 - 30:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved21(&self) -> RESERVED21_R {
        RESERVED21_R::new(((self.bits >> 21) & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - 31:31\\]
Register parity bit"]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:12 - 12:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<0> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bits 13:19 - 19:13\\]
Non-Secure boundary address. Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn boundary(&mut self) -> BOUNDARY_W<13> {
        BOUNDARY_W::new(self)
    }
    #[doc = "Bit 20 - 20:20\\]
Non-Secure boundary address MSB HW controlled."]
    #[inline(always)]
    #[must_use]
    pub fn boundary_msb(&mut self) -> BOUNDARY_MSB_W<20> {
        BOUNDARY_MSB_W::new(self)
    }
    #[doc = "Bits 21:30 - 30:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved21(&mut self) -> RESERVED21_W<21> {
        RESERVED21_W::new(self)
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
#[doc = "NVM Non-Secure boundary Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvmnsaddr](index.html) module"]
pub struct NVMNSADDR_SPEC;
impl crate::RegisterSpec for NVMNSADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvmnsaddr::R](R) reader structure"]
impl crate::Readable for NVMNSADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvmnsaddr::W](W) writer structure"]
impl crate::Writable for NVMNSADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NVMNSADDR to value 0x8010_0000"]
impl crate::Resettable for NVMNSADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x8010_0000;
}
