#[doc = "Register `DFTBANKCTL` reader"]
pub struct R(crate::R<DFTBANKCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFTBANKCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFTBANKCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFTBANKCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFTBANKCTL` writer"]
pub struct W(crate::W<DFTBANKCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFTBANKCTL_SPEC>;
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
impl From<crate::W<DFTBANKCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFTBANKCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCR` reader - 6:0\\]
TCR test mode to be applied to the bank"]
pub type TCR_R = crate::FieldReader<u8, TCR_A>;
#[doc = "6:0\\]
TCR test mode to be applied to the bank\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCR_A {
    #[doc = "127: Maximum value"]
    MAXIMUM = 127,
    #[doc = "0: Minimum value"]
    MINIMUM = 0,
}
impl From<TCR_A> for u8 {
    #[inline(always)]
    fn from(variant: TCR_A) -> Self {
        variant as _
    }
}
impl TCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TCR_A> {
        match self.bits {
            127 => Some(TCR_A::MAXIMUM),
            0 => Some(TCR_A::MINIMUM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAXIMUM`"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == TCR_A::MAXIMUM
    }
    #[doc = "Checks if the value of the field is `MINIMUM`"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == TCR_A::MINIMUM
    }
}
#[doc = "Field `TCR` writer - 6:0\\]
TCR test mode to be applied to the bank"]
pub type TCR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFTBANKCTL_SPEC, u8, TCR_A, 7, O>;
impl<'a, const O: u8> TCR_W<'a, O> {
    #[doc = "Maximum value"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut W {
        self.variant(TCR_A::MAXIMUM)
    }
    #[doc = "Minimum value"]
    #[inline(always)]
    pub fn minimum(self) -> &'a mut W {
        self.variant(TCR_A::MINIMUM)
    }
}
#[doc = "Field `RESERVED7` reader - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED7_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED7` writer - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED7_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFTBANKCTL_SPEC, bool, O>;
#[doc = "Field `TEZ` reader - 8:8\\]
When set, TEZ is asserted to the flash banks. Which banks get the asserted signal is determined by the BANKSELECT field in CMDCTL. 0x0 Do no assert TEZ 0x1 Assert TEZ"]
pub type TEZ_R = crate::BitReader<TEZ_A>;
#[doc = "8:8\\]
When set, TEZ is asserted to the flash banks. Which banks get the asserted signal is determined by the BANKSELECT field in CMDCTL. 0x0 Do no assert TEZ 0x1 Assert TEZ\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEZ_A {
    #[doc = "1: Do not assert TEZ"]
    NEGATE = 1,
    #[doc = "0: Assert TEZ"]
    ASSERT = 0,
}
impl From<TEZ_A> for bool {
    #[inline(always)]
    fn from(variant: TEZ_A) -> Self {
        variant as u8 != 0
    }
}
impl TEZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEZ_A {
        match self.bits {
            true => TEZ_A::NEGATE,
            false => TEZ_A::ASSERT,
        }
    }
    #[doc = "Checks if the value of the field is `NEGATE`"]
    #[inline(always)]
    pub fn is_negate(&self) -> bool {
        *self == TEZ_A::NEGATE
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == TEZ_A::ASSERT
    }
}
#[doc = "Field `TEZ` writer - 8:8\\]
When set, TEZ is asserted to the flash banks. Which banks get the asserted signal is determined by the BANKSELECT field in CMDCTL. 0x0 Do no assert TEZ 0x1 Assert TEZ"]
pub type TEZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFTBANKCTL_SPEC, TEZ_A, O>;
impl<'a, const O: u8> TEZ_W<'a, O> {
    #[doc = "Do not assert TEZ"]
    #[inline(always)]
    pub fn negate(self) -> &'a mut W {
        self.variant(TEZ_A::NEGATE)
    }
    #[doc = "Assert TEZ"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(TEZ_A::ASSERT)
    }
}
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
TCR test mode to be applied to the bank"]
    #[inline(always)]
    pub fn tcr(&self) -> TCR_R {
        TCR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
When set, TEZ is asserted to the flash banks. Which banks get the asserted signal is determined by the BANKSELECT field in CMDCTL. 0x0 Do no assert TEZ 0x1 Assert TEZ"]
    #[inline(always)]
    pub fn tez(&self) -> TEZ_R {
        TEZ_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
TCR test mode to be applied to the bank"]
    #[inline(always)]
    #[must_use]
    pub fn tcr(&mut self) -> TCR_W<0> {
        TCR_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> RESERVED7_W<7> {
        RESERVED7_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
When set, TEZ is asserted to the flash banks. Which banks get the asserted signal is determined by the BANKSELECT field in CMDCTL. 0x0 Do no assert TEZ 0x1 Assert TEZ"]
    #[inline(always)]
    #[must_use]
    pub fn tez(&mut self) -> TEZ_W<8> {
        TEZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFT Bank Control Register This allows some configuration of bank parameters during test. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dftbankctl](index.html) module"]
pub struct DFTBANKCTL_SPEC;
impl crate::RegisterSpec for DFTBANKCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dftbankctl::R](R) reader structure"]
impl crate::Readable for DFTBANKCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dftbankctl::W](W) writer structure"]
impl crate::Writable for DFTBANKCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFTBANKCTL to value 0x0100"]
impl crate::Resettable for DFTBANKCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
