#[doc = "Register `CFGPCNT` reader"]
pub struct R(crate::R<CFGPCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGPCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGPCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGPCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGPCNT` writer"]
pub struct W(crate::W<CFGPCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGPCNT_SPEC>;
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
impl From<crate::W<CFGPCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGPCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAXPCNTOVR` reader - 0:0\\]
Override hard-wired maximum pulse count. If MAXERSPCNTOVR is not set, then setting this value alone will override the max pulse count for both program and erase. If MAXERSPCNTOVR is set, then this bit will only control the max pulse count setting for program. By default, this bit is 0, and a hard-wired max pulse count is used."]
pub type MAXPCNTOVR_R = crate::BitReader<MAXPCNTOVR_A>;
#[doc = "0:0\\]
Override hard-wired maximum pulse count. If MAXERSPCNTOVR is not set, then setting this value alone will override the max pulse count for both program and erase. If MAXERSPCNTOVR is set, then this bit will only control the max pulse count setting for program. By default, this bit is 0, and a hard-wired max pulse count is used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MAXPCNTOVR_A {
    #[doc = "1: Use value from MAXPCNTVAL field as maximum puse count"]
    OVERRIDE = 1,
    #[doc = "0: Use hard-wired (default) value for maximum pulse count"]
    DEFAULT = 0,
}
impl From<MAXPCNTOVR_A> for bool {
    #[inline(always)]
    fn from(variant: MAXPCNTOVR_A) -> Self {
        variant as u8 != 0
    }
}
impl MAXPCNTOVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAXPCNTOVR_A {
        match self.bits {
            true => MAXPCNTOVR_A::OVERRIDE,
            false => MAXPCNTOVR_A::DEFAULT,
        }
    }
    #[doc = "Checks if the value of the field is `OVERRIDE`"]
    #[inline(always)]
    pub fn is_override(&self) -> bool {
        *self == MAXPCNTOVR_A::OVERRIDE
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == MAXPCNTOVR_A::DEFAULT
    }
}
#[doc = "Field `MAXPCNTOVR` writer - 0:0\\]
Override hard-wired maximum pulse count. If MAXERSPCNTOVR is not set, then setting this value alone will override the max pulse count for both program and erase. If MAXERSPCNTOVR is set, then this bit will only control the max pulse count setting for program. By default, this bit is 0, and a hard-wired max pulse count is used."]
pub type MAXPCNTOVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGPCNT_SPEC, MAXPCNTOVR_A, O>;
impl<'a, const O: u8> MAXPCNTOVR_W<'a, O> {
    #[doc = "Use value from MAXPCNTVAL field as maximum puse count"]
    #[inline(always)]
    pub fn override_(self) -> &'a mut W {
        self.variant(MAXPCNTOVR_A::OVERRIDE)
    }
    #[doc = "Use hard-wired (default) value for maximum pulse count"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(MAXPCNTOVR_A::DEFAULT)
    }
}
#[doc = "Field `RESERVED1` reader - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED1` writer - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGPCNT_SPEC, u8, u8, 3, O>;
#[doc = "Field `MAXPCNTVAL` reader - 11:4\\]
Override maximum pulse counter with this value. If MAXPCNTOVR = 0, then this field is ignored. If MAXPCNTOVR = 1 and MAXERSPCNTOVR = 0, then this value will be used to override the max pulse count for both program and erase. Full max value will be {4'h0, MAXPCNTVAL} . If MAXPCNTOVR = 1 and MAXERSPCNTOVR = 1, then this value will be used to override the max pulse count for program only. Full max value will be {4'h0, MAXPCNTVAL}."]
pub type MAXPCNTVAL_R = crate::FieldReader<u8, MAXPCNTVAL_A>;
#[doc = "11:4\\]
Override maximum pulse counter with this value. If MAXPCNTOVR = 0, then this field is ignored. If MAXPCNTOVR = 1 and MAXERSPCNTOVR = 0, then this value will be used to override the max pulse count for both program and erase. Full max value will be {4'h0, MAXPCNTVAL} . If MAXPCNTOVR = 1 and MAXERSPCNTOVR = 1, then this value will be used to override the max pulse count for program only. Full max value will be {4'h0, MAXPCNTVAL}.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MAXPCNTVAL_A {
    #[doc = "255: Maximum value"]
    MAXIMUM = 255,
    #[doc = "0: Minimum value"]
    MINIMUM = 0,
}
impl From<MAXPCNTVAL_A> for u8 {
    #[inline(always)]
    fn from(variant: MAXPCNTVAL_A) -> Self {
        variant as _
    }
}
impl MAXPCNTVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MAXPCNTVAL_A> {
        match self.bits {
            255 => Some(MAXPCNTVAL_A::MAXIMUM),
            0 => Some(MAXPCNTVAL_A::MINIMUM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAXIMUM`"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == MAXPCNTVAL_A::MAXIMUM
    }
    #[doc = "Checks if the value of the field is `MINIMUM`"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == MAXPCNTVAL_A::MINIMUM
    }
}
#[doc = "Field `MAXPCNTVAL` writer - 11:4\\]
Override maximum pulse counter with this value. If MAXPCNTOVR = 0, then this field is ignored. If MAXPCNTOVR = 1 and MAXERSPCNTOVR = 0, then this value will be used to override the max pulse count for both program and erase. Full max value will be {4'h0, MAXPCNTVAL} . If MAXPCNTOVR = 1 and MAXERSPCNTOVR = 1, then this value will be used to override the max pulse count for program only. Full max value will be {4'h0, MAXPCNTVAL}."]
pub type MAXPCNTVAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CFGPCNT_SPEC, u8, MAXPCNTVAL_A, 8, O>;
impl<'a, const O: u8> MAXPCNTVAL_W<'a, O> {
    #[doc = "Maximum value"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut W {
        self.variant(MAXPCNTVAL_A::MAXIMUM)
    }
    #[doc = "Minimum value"]
    #[inline(always)]
    pub fn minimum(self) -> &'a mut W {
        self.variant(MAXPCNTVAL_A::MINIMUM)
    }
}
#[doc = "Field `RESERVED12` reader - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED12_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED12` writer - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGPCNT_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Override hard-wired maximum pulse count. If MAXERSPCNTOVR is not set, then setting this value alone will override the max pulse count for both program and erase. If MAXERSPCNTOVR is set, then this bit will only control the max pulse count setting for program. By default, this bit is 0, and a hard-wired max pulse count is used."]
    #[inline(always)]
    pub fn maxpcntovr(&self) -> MAXPCNTOVR_R {
        MAXPCNTOVR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:11 - 11:4\\]
Override maximum pulse counter with this value. If MAXPCNTOVR = 0, then this field is ignored. If MAXPCNTOVR = 1 and MAXERSPCNTOVR = 0, then this value will be used to override the max pulse count for both program and erase. Full max value will be {4'h0, MAXPCNTVAL} . If MAXPCNTOVR = 1 and MAXERSPCNTOVR = 1, then this value will be used to override the max pulse count for program only. Full max value will be {4'h0, MAXPCNTVAL}."]
    #[inline(always)]
    pub fn maxpcntval(&self) -> MAXPCNTVAL_R {
        MAXPCNTVAL_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Override hard-wired maximum pulse count. If MAXERSPCNTOVR is not set, then setting this value alone will override the max pulse count for both program and erase. If MAXERSPCNTOVR is set, then this bit will only control the max pulse count setting for program. By default, this bit is 0, and a hard-wired max pulse count is used."]
    #[inline(always)]
    #[must_use]
    pub fn maxpcntovr(&mut self) -> MAXPCNTOVR_W<0> {
        MAXPCNTOVR_W::new(self)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Bits 4:11 - 11:4\\]
Override maximum pulse counter with this value. If MAXPCNTOVR = 0, then this field is ignored. If MAXPCNTOVR = 1 and MAXERSPCNTOVR = 0, then this value will be used to override the max pulse count for both program and erase. Full max value will be {4'h0, MAXPCNTVAL} . If MAXPCNTOVR = 1 and MAXERSPCNTOVR = 1, then this value will be used to override the max pulse count for program only. Full max value will be {4'h0, MAXPCNTVAL}."]
    #[inline(always)]
    #[must_use]
    pub fn maxpcntval(&mut self) -> MAXPCNTVAL_W<4> {
        MAXPCNTVAL_W::new(self)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> RESERVED12_W<12> {
        RESERVED12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pulse Counter Configuration Register This register allows further configuration of maximum pulse counts for program and erase operations. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgpcnt](index.html) module"]
pub struct CFGPCNT_SPEC;
impl crate::RegisterSpec for CFGPCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgpcnt::R](R) reader structure"]
impl crate::Readable for CFGPCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgpcnt::W](W) writer structure"]
impl crate::Writable for CFGPCNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGPCNT to value 0"]
impl crate::Resettable for CFGPCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
