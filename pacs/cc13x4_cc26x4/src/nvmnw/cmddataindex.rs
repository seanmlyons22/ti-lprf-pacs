#[doc = "Register `CMDDATAINDEX` reader"]
pub struct R(crate::R<CMDDATAINDEX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDDATAINDEX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMDDATAINDEX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMDDATAINDEX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMDDATAINDEX` writer"]
pub struct W(crate::W<CMDDATAINDEX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMDDATAINDEX_SPEC>;
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
impl From<crate::W<CMDDATAINDEX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMDDATAINDEX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VAL` reader - 1:0\\]
Data register index"]
pub type VAL_R = crate::FieldReader<u8, VAL_A>;
#[doc = "1:0\\]
Data register index\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VAL_A {
    #[doc = "3: Maximum value of VAL"]
    MAXIMUM = 3,
    #[doc = "0: Minimum value of VAL"]
    MINIMUM = 0,
}
impl From<VAL_A> for u8 {
    #[inline(always)]
    fn from(variant: VAL_A) -> Self {
        variant as _
    }
}
impl VAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VAL_A> {
        match self.bits {
            3 => Some(VAL_A::MAXIMUM),
            0 => Some(VAL_A::MINIMUM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAXIMUM`"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == VAL_A::MAXIMUM
    }
    #[doc = "Checks if the value of the field is `MINIMUM`"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == VAL_A::MINIMUM
    }
}
#[doc = "Field `VAL` writer - 1:0\\]
Data register index"]
pub type VAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMDDATAINDEX_SPEC, u8, VAL_A, 2, O>;
impl<'a, const O: u8> VAL_W<'a, O> {
    #[doc = "Maximum value of VAL"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut W {
        self.variant(VAL_A::MAXIMUM)
    }
    #[doc = "Minimum value of VAL"]
    #[inline(always)]
    pub fn minimum(self) -> &'a mut W {
        self.variant(VAL_A::MINIMUM)
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CMDDATAINDEX_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Data register index"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Data register index"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> VAL_W<0> {
        VAL_W::new(self)
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
#[doc = "Command Program Data Index Register: When multiple data registers are available for multi-word program, this register can be written with an index which points to one of the data registers. When a write to CMDDATA* is done, the data will be written to the physical data register indexed by the value in this register. Up to 8 data registers can be present, so this register can be written with 0x0 to 0x7. If less than 8 data registers are present, successive MSB bits of this register are ignored when indexing the CMDDATA* registers. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmddataindex](index.html) module"]
pub struct CMDDATAINDEX_SPEC;
impl crate::RegisterSpec for CMDDATAINDEX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmddataindex::R](R) reader structure"]
impl crate::Readable for CMDDATAINDEX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmddataindex::W](W) writer structure"]
impl crate::Writable for CMDDATAINDEX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMDDATAINDEX to value 0"]
impl crate::Resettable for CMDDATAINDEX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
