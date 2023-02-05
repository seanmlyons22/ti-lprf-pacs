#[doc = "Register `INT_ATREADY` reader"]
pub struct R(crate::R<INT_ATREADY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ATREADY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ATREADY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ATREADY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ATREADY` writer"]
pub struct W(crate::W<INT_ATREADY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ATREADY_SPEC>;
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
impl From<crate::W<INT_ATREADY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ATREADY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATREADY` reader - 0:0\\]
A read of this bit returns the value of ATREADY"]
pub type ATREADY_R = crate::BitReader<bool>;
#[doc = "Field `ATREADY` writer - 0:0\\]
A read of this bit returns the value of ATREADY"]
pub type ATREADY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ATREADY_SPEC, bool, O>;
#[doc = "Field `AFVALID` reader - 1:1\\]
A read of this bit returns the value of AFVALID"]
pub type AFVALID_R = crate::BitReader<bool>;
#[doc = "Field `AFVALID` writer - 1:1\\]
A read of this bit returns the value of AFVALID"]
pub type AFVALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ATREADY_SPEC, bool, O>;
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INT_ATREADY_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
A read of this bit returns the value of ATREADY"]
    #[inline(always)]
    pub fn atready(&self) -> ATREADY_R {
        ATREADY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
A read of this bit returns the value of AFVALID"]
    #[inline(always)]
    pub fn afvalid(&self) -> AFVALID_R {
        AFVALID_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
A read of this bit returns the value of ATREADY"]
    #[inline(always)]
    #[must_use]
    pub fn atready(&mut self) -> ATREADY_W<0> {
        ATREADY_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
A read of this bit returns the value of AFVALID"]
    #[inline(always)]
    #[must_use]
    pub fn afvalid(&mut self) -> AFVALID_W<1> {
        AFVALID_W::new(self)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Integration Mode: Read ATB Ready\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_atready](index.html) module"]
pub struct INT_ATREADY_SPEC;
impl crate::RegisterSpec for INT_ATREADY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_atready::R](R) reader structure"]
impl crate::Readable for INT_ATREADY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_atready::W](W) writer structure"]
impl crate::Writable for INT_ATREADY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_ATREADY to value 0"]
impl crate::Resettable for INT_ATREADY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
