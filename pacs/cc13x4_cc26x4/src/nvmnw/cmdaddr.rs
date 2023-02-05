#[doc = "Register `CMDADDR` reader"]
pub struct R(crate::R<CMDADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMDADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMDADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMDADDR` writer"]
pub struct W(crate::W<CMDADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMDADDR_SPEC>;
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
impl From<crate::W<CMDADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMDADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VAL` reader - 31:0\\]
Address value"]
pub type VAL_R = crate::FieldReader<u32, VAL_A>;
#[doc = "31:0\\]
Address value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum VAL_A {
    #[doc = "4294967295: Maximum value of VAL"]
    MAXIMUM = 4294967295,
    #[doc = "0: Minimum value of VAL"]
    MINIMUM = 0,
}
impl From<VAL_A> for u32 {
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
            4294967295 => Some(VAL_A::MAXIMUM),
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
#[doc = "Field `VAL` writer - 31:0\\]
Address value"]
pub type VAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMDADDR_SPEC, u32, VAL_A, 32, O>;
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
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Address value"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Address value"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> VAL_W<0> {
        VAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command Address Register: This register forms the target address of a command. The use cases are as follows: 1)For single-word program, this address indicates the flash bank word to be programmed. 2)For multi-word program, this address indicates the first flash bank address for the program. The address will be incremented for further words. 3)For sector erase, this address indicates the sector to be erased. 4)For bank erase, the address indicates the bank to be erased. 5)For read verify, the address indications follow program/erase listed above. Note the address written to this register will be submitted for translation to the NoWrapper address translation interface, and the translated address will be used to access the bank. However, if the CMDCTL.ADDRXLATEOVR bit is set, then the address written to this register will be used directly as the bank address. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdaddr](index.html) module"]
pub struct CMDADDR_SPEC;
impl crate::RegisterSpec for CMDADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmdaddr::R](R) reader structure"]
impl crate::Readable for CMDADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmdaddr::W](W) writer structure"]
impl crate::Writable for CMDADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMDADDR to value 0"]
impl crate::Resettable for CMDADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
