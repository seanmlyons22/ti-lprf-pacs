#[doc = "Register `PDSTAT0` reader"]
pub struct R(crate::R<PDSTAT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDSTAT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDSTAT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDSTAT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDSTAT0` writer"]
pub struct W(crate::W<PDSTAT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDSTAT0_SPEC>;
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
impl From<crate::W<PDSTAT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDSTAT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFC_ON` reader - 0:0\\]
RFC Power domain 0: Domain may be powered down 1: Domain powered up (guaranteed)"]
pub type RFC_ON_R = crate::BitReader<bool>;
#[doc = "Field `RFC_ON` writer - 0:0\\]
RFC Power domain 0: Domain may be powered down 1: Domain powered up (guaranteed)"]
pub type RFC_ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDSTAT0_SPEC, bool, O>;
#[doc = "Field `SERIAL_ON` reader - 1:1\\]
SERIAL Power domain. 0: Domain may be powered down 1: Domain powered up (guaranteed)"]
pub type SERIAL_ON_R = crate::BitReader<bool>;
#[doc = "Field `SERIAL_ON` writer - 1:1\\]
SERIAL Power domain. 0: Domain may be powered down 1: Domain powered up (guaranteed)"]
pub type SERIAL_ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDSTAT0_SPEC, bool, O>;
#[doc = "Field `PERIPH_ON` reader - 2:2\\]
PERIPH Power domain. 0: Domain may be powered down 1: Domain powered up (guaranteed)"]
pub type PERIPH_ON_R = crate::BitReader<bool>;
#[doc = "Field `PERIPH_ON` writer - 2:2\\]
PERIPH Power domain. 0: Domain may be powered down 1: Domain powered up (guaranteed)"]
pub type PERIPH_ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDSTAT0_SPEC, bool, O>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PDSTAT0_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
RFC Power domain 0: Domain may be powered down 1: Domain powered up (guaranteed)"]
    #[inline(always)]
    pub fn rfc_on(&self) -> RFC_ON_R {
        RFC_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
SERIAL Power domain. 0: Domain may be powered down 1: Domain powered up (guaranteed)"]
    #[inline(always)]
    pub fn serial_on(&self) -> SERIAL_ON_R {
        SERIAL_ON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
PERIPH Power domain. 0: Domain may be powered down 1: Domain powered up (guaranteed)"]
    #[inline(always)]
    pub fn periph_on(&self) -> PERIPH_ON_R {
        PERIPH_ON_R::new(((self.bits >> 2) & 1) != 0)
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
RFC Power domain 0: Domain may be powered down 1: Domain powered up (guaranteed)"]
    #[inline(always)]
    #[must_use]
    pub fn rfc_on(&mut self) -> RFC_ON_W<0> {
        RFC_ON_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
SERIAL Power domain. 0: Domain may be powered down 1: Domain powered up (guaranteed)"]
    #[inline(always)]
    #[must_use]
    pub fn serial_on(&mut self) -> SERIAL_ON_W<1> {
        SERIAL_ON_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
PERIPH Power domain. 0: Domain may be powered down 1: Domain powered up (guaranteed)"]
    #[inline(always)]
    #[must_use]
    pub fn periph_on(&mut self) -> PERIPH_ON_W<2> {
        PERIPH_ON_W::new(self)
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
#[doc = "Power Domain Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdstat0](index.html) module"]
pub struct PDSTAT0_SPEC;
impl crate::RegisterSpec for PDSTAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdstat0::R](R) reader structure"]
impl crate::Readable for PDSTAT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdstat0::W](W) writer structure"]
impl crate::Writable for PDSTAT0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDSTAT0 to value 0"]
impl crate::Resettable for PDSTAT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
