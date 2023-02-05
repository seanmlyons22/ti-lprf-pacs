#[doc = "Register `CMDEXEC` reader"]
pub struct R(crate::R<CMDEXEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDEXEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMDEXEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMDEXEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMDEXEC` writer"]
pub struct W(crate::W<CMDEXEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMDEXEC_SPEC>;
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
impl From<crate::W<CMDEXEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMDEXEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VAL` reader - 0:0\\]
Command Execute value Initiates execution of the command specified in the CMDTYPE register."]
pub type VAL_R = crate::BitReader<VAL_A>;
#[doc = "0:0\\]
Command Execute value Initiates execution of the command specified in the CMDTYPE register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VAL_A {
    #[doc = "1: Command will execute or is executing in NoWrapper"]
    EXECUTE = 1,
    #[doc = "0: Command will not execute or is not executing in NoWrapper"]
    NOEXECUTE = 0,
}
impl From<VAL_A> for bool {
    #[inline(always)]
    fn from(variant: VAL_A) -> Self {
        variant as u8 != 0
    }
}
impl VAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VAL_A {
        match self.bits {
            true => VAL_A::EXECUTE,
            false => VAL_A::NOEXECUTE,
        }
    }
    #[doc = "Checks if the value of the field is `EXECUTE`"]
    #[inline(always)]
    pub fn is_execute(&self) -> bool {
        *self == VAL_A::EXECUTE
    }
    #[doc = "Checks if the value of the field is `NOEXECUTE`"]
    #[inline(always)]
    pub fn is_noexecute(&self) -> bool {
        *self == VAL_A::NOEXECUTE
    }
}
#[doc = "Field `VAL` writer - 0:0\\]
Command Execute value Initiates execution of the command specified in the CMDTYPE register."]
pub type VAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDEXEC_SPEC, VAL_A, O>;
impl<'a, const O: u8> VAL_W<'a, O> {
    #[doc = "Command will execute or is executing in NoWrapper"]
    #[inline(always)]
    pub fn execute(self) -> &'a mut W {
        self.variant(VAL_A::EXECUTE)
    }
    #[doc = "Command will not execute or is not executing in NoWrapper"]
    #[inline(always)]
    pub fn noexecute(self) -> &'a mut W {
        self.variant(VAL_A::NOEXECUTE)
    }
}
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMDEXEC_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Command Execute value Initiates execution of the command specified in the CMDTYPE register."]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Command Execute value Initiates execution of the command specified in the CMDTYPE register."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> VAL_W<0> {
        VAL_W::new(self)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command Execute Register: Initiates execution of the command specified in the CMDTYPE register. This register is blocked for writes after being written to 1 and prior to STATCMD.DONE being set by the NoWrapper hardware. NoWrapper hardware clears this register after the processing of the command has completed.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdexec](index.html) module"]
pub struct CMDEXEC_SPEC;
impl crate::RegisterSpec for CMDEXEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmdexec::R](R) reader structure"]
impl crate::Readable for CMDEXEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmdexec::W](W) writer structure"]
impl crate::Writable for CMDEXEC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMDEXEC to value 0"]
impl crate::Resettable for CMDEXEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
