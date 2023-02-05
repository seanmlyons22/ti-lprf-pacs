#[doc = "Register `CMDDATA13` reader"]
pub struct R(crate::R<CMDDATA13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDDATA13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMDDATA13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMDDATA13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMDDATA13` writer"]
pub struct W(crate::W<CMDDATA13_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMDDATA13_SPEC>;
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
impl From<crate::W<CMDDATA13_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMDDATA13_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VAL` reader - 31:0\\]
A 32-bit data value is placed in this field."]
pub type VAL_R = crate::FieldReader<u32, VAL_A>;
#[doc = "31:0\\]
A 32-bit data value is placed in this field.\n\nValue on reset: 4294967295"]
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
A 32-bit data value is placed in this field."]
pub type VAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMDDATA13_SPEC, u32, VAL_A, 32, O>;
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
A 32-bit data value is placed in this field."]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
A 32-bit data value is placed in this field."]
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
#[doc = "Command Data Register 13 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 63:32 of flash word data register 3. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 6. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmddata13](index.html) module"]
pub struct CMDDATA13_SPEC;
impl crate::RegisterSpec for CMDDATA13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmddata13::R](R) reader structure"]
impl crate::Readable for CMDDATA13_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmddata13::W](W) writer structure"]
impl crate::Writable for CMDDATA13_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMDDATA13 to value 0xffff_ffff"]
impl crate::Resettable for CMDDATA13_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
