#[doc = "Register `CMDBYTEN` reader"]
pub struct R(crate::R<CMDBYTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDBYTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMDBYTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMDBYTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMDBYTEN` writer"]
pub struct W(crate::W<CMDBYTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMDBYTEN_SPEC>;
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
impl From<crate::W<CMDBYTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMDBYTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VAL` reader - 15:0\\]
Command Byte Enable value. A 1-bit per flash word byte value is placed in this register."]
pub type VAL_R = crate::FieldReader<u16, VAL_A>;
#[doc = "15:0\\]
Command Byte Enable value. A 1-bit per flash word byte value is placed in this register.\n\nValue on reset: 65535"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum VAL_A {
    #[doc = "65535: Maximum value of VAL"]
    MAXIMUM = 65535,
    #[doc = "0: Minimum value of VAL"]
    MINIMUM = 0,
}
impl From<VAL_A> for u16 {
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
            65535 => Some(VAL_A::MAXIMUM),
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
#[doc = "Field `VAL` writer - 15:0\\]
Command Byte Enable value. A 1-bit per flash word byte value is placed in this register."]
pub type VAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMDBYTEN_SPEC, u16, VAL_A, 16, O>;
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
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CMDBYTEN_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Command Byte Enable value. A 1-bit per flash word byte value is placed in this register."]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Command Byte Enable value. A 1-bit per flash word byte value is placed in this register."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> VAL_W<0> {
        VAL_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> RESERVED16_W<16> {
        RESERVED16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command Program Byte Enable Register: This register forms a per-byte enable for programming data. For data bytes to be programmed, a 1 must be written to the corresponding bit in this register. Normally, all bits are written to 1, allowing program of full flash words. However, leaving some bits 0 allows programming of 8-bit, 16-bit, 32-bit or 64-bit portions of a flash word. In addtion, the read verify command will ignore data bytes read from the flash in its comparison if the corresponding CMDBYTEN bit is 0. ECC data bytes are protected by the 1-2 MSB bits in this register, depending on the presence of ECC and the flash word data width. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is written to all 0 after the completion of all NoWrapper commands.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdbyten](index.html) module"]
pub struct CMDBYTEN_SPEC;
impl crate::RegisterSpec for CMDBYTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmdbyten::R](R) reader structure"]
impl crate::Readable for CMDBYTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmdbyten::W](W) writer structure"]
impl crate::Writable for CMDBYTEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMDBYTEN to value 0xffff"]
impl crate::Resettable for CMDBYTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
