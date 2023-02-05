#[doc = "Register `DFTPCLKTESTCTL` reader"]
pub struct R(crate::R<DFTPCLKTESTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFTPCLKTESTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFTPCLKTESTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFTPCLKTESTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFTPCLKTESTCTL` writer"]
pub struct W(crate::W<DFTPCLKTESTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFTPCLKTESTCTL_SPEC>;
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
impl From<crate::W<DFTPCLKTESTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFTPCLKTESTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - 0:0\\]
Enable the state machine which sequences measurement of pump clock frequency."]
pub type ENABLE_R = crate::BitReader<ENABLE_A>;
#[doc = "0:0\\]
Enable the state machine which sequences measurement of pump clock frequency.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE_A {
    #[doc = "1: Enable"]
    ENABLE = 1,
    #[doc = "0: Disable"]
    DISABLE = 0,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            true => ENABLE_A::ENABLE,
            false => ENABLE_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENABLE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENABLE_A::DISABLE
    }
}
#[doc = "Field `ENABLE` writer - 0:0\\]
Enable the state machine which sequences measurement of pump clock frequency."]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFTPCLKTESTCTL_SPEC, ENABLE_A, O>;
impl<'a, const O: u8> ENABLE_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLE)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENABLE_A::DISABLE)
    }
}
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFTPCLKTESTCTL_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable the state machine which sequences measurement of pump clock frequency."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
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
Enable the state machine which sequences measurement of pump clock frequency."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
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
#[doc = "DFT Pump Clock Test Control Register. This register controls hardware features that allow the pump clock to be characterized for trim development. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dftpclktestctl](index.html) module"]
pub struct DFTPCLKTESTCTL_SPEC;
impl crate::RegisterSpec for DFTPCLKTESTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dftpclktestctl::R](R) reader structure"]
impl crate::Readable for DFTPCLKTESTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dftpclktestctl::W](W) writer structure"]
impl crate::Writable for DFTPCLKTESTCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFTPCLKTESTCTL to value 0"]
impl crate::Resettable for DFTPCLKTESTCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
