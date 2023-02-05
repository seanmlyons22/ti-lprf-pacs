#[doc = "Register `CFGCMD` reader"]
pub struct R(crate::R<CFGCMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGCMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGCMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGCMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGCMD` writer"]
pub struct W(crate::W<CFGCMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGCMD_SPEC>;
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
impl From<crate::W<CFGCMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGCMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAITSTATE` reader - 3:0\\]
Wait State setting for program verify, erase verify and read verify"]
pub type WAITSTATE_R = crate::FieldReader<u8, WAITSTATE_A>;
#[doc = "3:0\\]
Wait State setting for program verify, erase verify and read verify\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WAITSTATE_A {
    #[doc = "15: Maximum value"]
    MAXIMUM = 15,
    #[doc = "0: Minimum value"]
    MINIMUM = 0,
}
impl From<WAITSTATE_A> for u8 {
    #[inline(always)]
    fn from(variant: WAITSTATE_A) -> Self {
        variant as _
    }
}
impl WAITSTATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WAITSTATE_A> {
        match self.bits {
            15 => Some(WAITSTATE_A::MAXIMUM),
            0 => Some(WAITSTATE_A::MINIMUM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAXIMUM`"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == WAITSTATE_A::MAXIMUM
    }
    #[doc = "Checks if the value of the field is `MINIMUM`"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == WAITSTATE_A::MINIMUM
    }
}
#[doc = "Field `WAITSTATE` writer - 3:0\\]
Wait State setting for program verify, erase verify and read verify"]
pub type WAITSTATE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CFGCMD_SPEC, u8, WAITSTATE_A, 4, O>;
impl<'a, const O: u8> WAITSTATE_W<'a, O> {
    #[doc = "Maximum value"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut W {
        self.variant(WAITSTATE_A::MAXIMUM)
    }
    #[doc = "Minimum value"]
    #[inline(always)]
    pub fn minimum(self) -> &'a mut W {
        self.variant(WAITSTATE_A::MINIMUM)
    }
}
#[doc = "Field `RESERVED4` reader - 32:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED4` writer - 32:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGCMD_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Wait State setting for program verify, erase verify and read verify"]
    #[inline(always)]
    pub fn waitstate(&self) -> WAITSTATE_R {
        WAITSTATE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - 32:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Wait State setting for program verify, erase verify and read verify"]
    #[inline(always)]
    #[must_use]
    pub fn waitstate(&mut self) -> WAITSTATE_W<0> {
        WAITSTATE_W::new(self)
    }
    #[doc = "Bits 4:31 - 32:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> RESERVED4_W<4> {
        RESERVED4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command Configuration Register This register configures specific capabilities of the state machine for related to the execution of a command. This register is blocked for writes after CMDEXEC is written to a 1 and prior to STATCMD.DONE being set by the hardware to indicate that command execution has completed.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgcmd](index.html) module"]
pub struct CFGCMD_SPEC;
impl crate::RegisterSpec for CFGCMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgcmd::R](R) reader structure"]
impl crate::Readable for CFGCMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgcmd::W](W) writer structure"]
impl crate::Writable for CFGCMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGCMD to value 0x02"]
impl crate::Resettable for CFGCMD_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
