#[doc = "Register `DFTTIMERCTL` reader"]
pub struct R(crate::R<DFTTIMERCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFTTIMERCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFTTIMERCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFTTIMERCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFTTIMERCTL` writer"]
pub struct W(crate::W<DFTTIMERCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFTTIMERCTL_SPEC>;
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
impl From<crate::W<DFTTIMERCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFTTIMERCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PESETUPTIME` reader - 0:0\\]
Program/Erase Setup Time"]
pub type PESETUPTIME_R = crate::BitReader<PESETUPTIME_A>;
#[doc = "0:0\\]
Program/Erase Setup Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PESETUPTIME_A {
    #[doc = "1: Use 2x the hard-wired (functional) time value"]
    TWOXFUNCTIONAL = 1,
    #[doc = "0: Use hard-wired (Functional) timer value"]
    FUNCTIONAL = 0,
}
impl From<PESETUPTIME_A> for bool {
    #[inline(always)]
    fn from(variant: PESETUPTIME_A) -> Self {
        variant as u8 != 0
    }
}
impl PESETUPTIME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PESETUPTIME_A {
        match self.bits {
            true => PESETUPTIME_A::TWOXFUNCTIONAL,
            false => PESETUPTIME_A::FUNCTIONAL,
        }
    }
    #[doc = "Checks if the value of the field is `TWOXFUNCTIONAL`"]
    #[inline(always)]
    pub fn is_twoxfunctional(&self) -> bool {
        *self == PESETUPTIME_A::TWOXFUNCTIONAL
    }
    #[doc = "Checks if the value of the field is `FUNCTIONAL`"]
    #[inline(always)]
    pub fn is_functional(&self) -> bool {
        *self == PESETUPTIME_A::FUNCTIONAL
    }
}
#[doc = "Field `PESETUPTIME` writer - 0:0\\]
Program/Erase Setup Time"]
pub type PESETUPTIME_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DFTTIMERCTL_SPEC, PESETUPTIME_A, O>;
impl<'a, const O: u8> PESETUPTIME_W<'a, O> {
    #[doc = "Use 2x the hard-wired (functional) time value"]
    #[inline(always)]
    pub fn twoxfunctional(self) -> &'a mut W {
        self.variant(PESETUPTIME_A::TWOXFUNCTIONAL)
    }
    #[doc = "Use hard-wired (Functional) timer value"]
    #[inline(always)]
    pub fn functional(self) -> &'a mut W {
        self.variant(PESETUPTIME_A::FUNCTIONAL)
    }
}
#[doc = "Field `PVHVSETUPTIME` reader - 1:1\\]
Program VHV Setup Time"]
pub type PVHVSETUPTIME_R = crate::BitReader<PVHVSETUPTIME_A>;
#[doc = "1:1\\]
Program VHV Setup Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVHVSETUPTIME_A {
    #[doc = "1: Use 2x the hard-wired (functional) time value"]
    TWOXFUNCTIONAL = 1,
    #[doc = "0: Use hard-wired (Functional) timer value"]
    FUNCTIONAL = 0,
}
impl From<PVHVSETUPTIME_A> for bool {
    #[inline(always)]
    fn from(variant: PVHVSETUPTIME_A) -> Self {
        variant as u8 != 0
    }
}
impl PVHVSETUPTIME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PVHVSETUPTIME_A {
        match self.bits {
            true => PVHVSETUPTIME_A::TWOXFUNCTIONAL,
            false => PVHVSETUPTIME_A::FUNCTIONAL,
        }
    }
    #[doc = "Checks if the value of the field is `TWOXFUNCTIONAL`"]
    #[inline(always)]
    pub fn is_twoxfunctional(&self) -> bool {
        *self == PVHVSETUPTIME_A::TWOXFUNCTIONAL
    }
    #[doc = "Checks if the value of the field is `FUNCTIONAL`"]
    #[inline(always)]
    pub fn is_functional(&self) -> bool {
        *self == PVHVSETUPTIME_A::FUNCTIONAL
    }
}
#[doc = "Field `PVHVSETUPTIME` writer - 1:1\\]
Program VHV Setup Time"]
pub type PVHVSETUPTIME_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DFTTIMERCTL_SPEC, PVHVSETUPTIME_A, O>;
impl<'a, const O: u8> PVHVSETUPTIME_W<'a, O> {
    #[doc = "Use 2x the hard-wired (functional) time value"]
    #[inline(always)]
    pub fn twoxfunctional(self) -> &'a mut W {
        self.variant(PVHVSETUPTIME_A::TWOXFUNCTIONAL)
    }
    #[doc = "Use hard-wired (Functional) timer value"]
    #[inline(always)]
    pub fn functional(self) -> &'a mut W {
        self.variant(PVHVSETUPTIME_A::FUNCTIONAL)
    }
}
#[doc = "Field `PPVWORDLINETIME` reader - 2:2\\]
Program and Program Verify Wordline Switching Time"]
pub type PPVWORDLINETIME_R = crate::BitReader<PPVWORDLINETIME_A>;
#[doc = "2:2\\]
Program and Program Verify Wordline Switching Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PPVWORDLINETIME_A {
    #[doc = "1: Use 2x the hard-wired (functional) time value"]
    TWOXFUNCTIONAL = 1,
    #[doc = "0: Use hard-wired (Functional) timer value"]
    FUNCTIONAL = 0,
}
impl From<PPVWORDLINETIME_A> for bool {
    #[inline(always)]
    fn from(variant: PPVWORDLINETIME_A) -> Self {
        variant as u8 != 0
    }
}
impl PPVWORDLINETIME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPVWORDLINETIME_A {
        match self.bits {
            true => PPVWORDLINETIME_A::TWOXFUNCTIONAL,
            false => PPVWORDLINETIME_A::FUNCTIONAL,
        }
    }
    #[doc = "Checks if the value of the field is `TWOXFUNCTIONAL`"]
    #[inline(always)]
    pub fn is_twoxfunctional(&self) -> bool {
        *self == PPVWORDLINETIME_A::TWOXFUNCTIONAL
    }
    #[doc = "Checks if the value of the field is `FUNCTIONAL`"]
    #[inline(always)]
    pub fn is_functional(&self) -> bool {
        *self == PPVWORDLINETIME_A::FUNCTIONAL
    }
}
#[doc = "Field `PPVWORDLINETIME` writer - 2:2\\]
Program and Program Verify Wordline Switching Time"]
pub type PPVWORDLINETIME_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DFTTIMERCTL_SPEC, PPVWORDLINETIME_A, O>;
impl<'a, const O: u8> PPVWORDLINETIME_W<'a, O> {
    #[doc = "Use 2x the hard-wired (functional) time value"]
    #[inline(always)]
    pub fn twoxfunctional(self) -> &'a mut W {
        self.variant(PPVWORDLINETIME_A::TWOXFUNCTIONAL)
    }
    #[doc = "Use hard-wired (Functional) timer value"]
    #[inline(always)]
    pub fn functional(self) -> &'a mut W {
        self.variant(PPVWORDLINETIME_A::FUNCTIONAL)
    }
}
#[doc = "Field `PEHOLDTIME` reader - 3:3\\]
Program/Erase Hold Time"]
pub type PEHOLDTIME_R = crate::BitReader<PEHOLDTIME_A>;
#[doc = "3:3\\]
Program/Erase Hold Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEHOLDTIME_A {
    #[doc = "1: Use 2x the hard-wired (functional) time value"]
    TWOXFUNCTIONAL = 1,
    #[doc = "0: Use hard-wired (Functional) timer value"]
    FUNCTIONAL = 0,
}
impl From<PEHOLDTIME_A> for bool {
    #[inline(always)]
    fn from(variant: PEHOLDTIME_A) -> Self {
        variant as u8 != 0
    }
}
impl PEHOLDTIME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEHOLDTIME_A {
        match self.bits {
            true => PEHOLDTIME_A::TWOXFUNCTIONAL,
            false => PEHOLDTIME_A::FUNCTIONAL,
        }
    }
    #[doc = "Checks if the value of the field is `TWOXFUNCTIONAL`"]
    #[inline(always)]
    pub fn is_twoxfunctional(&self) -> bool {
        *self == PEHOLDTIME_A::TWOXFUNCTIONAL
    }
    #[doc = "Checks if the value of the field is `FUNCTIONAL`"]
    #[inline(always)]
    pub fn is_functional(&self) -> bool {
        *self == PEHOLDTIME_A::FUNCTIONAL
    }
}
#[doc = "Field `PEHOLDTIME` writer - 3:3\\]
Program/Erase Hold Time"]
pub type PEHOLDTIME_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DFTTIMERCTL_SPEC, PEHOLDTIME_A, O>;
impl<'a, const O: u8> PEHOLDTIME_W<'a, O> {
    #[doc = "Use 2x the hard-wired (functional) time value"]
    #[inline(always)]
    pub fn twoxfunctional(self) -> &'a mut W {
        self.variant(PEHOLDTIME_A::TWOXFUNCTIONAL)
    }
    #[doc = "Use hard-wired (Functional) timer value"]
    #[inline(always)]
    pub fn functional(self) -> &'a mut W {
        self.variant(PEHOLDTIME_A::FUNCTIONAL)
    }
}
#[doc = "Field `PEVMODETIME` reader - 4:4\\]
Program/Erase Verify Mode Change Time"]
pub type PEVMODETIME_R = crate::BitReader<PEVMODETIME_A>;
#[doc = "4:4\\]
Program/Erase Verify Mode Change Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEVMODETIME_A {
    #[doc = "1: Use 2x the hard-wired (functional) time value"]
    TWOXFUNCTIONAL = 1,
    #[doc = "0: Use hard-wired (Functional) timer value"]
    FUNCTIONAL = 0,
}
impl From<PEVMODETIME_A> for bool {
    #[inline(always)]
    fn from(variant: PEVMODETIME_A) -> Self {
        variant as u8 != 0
    }
}
impl PEVMODETIME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEVMODETIME_A {
        match self.bits {
            true => PEVMODETIME_A::TWOXFUNCTIONAL,
            false => PEVMODETIME_A::FUNCTIONAL,
        }
    }
    #[doc = "Checks if the value of the field is `TWOXFUNCTIONAL`"]
    #[inline(always)]
    pub fn is_twoxfunctional(&self) -> bool {
        *self == PEVMODETIME_A::TWOXFUNCTIONAL
    }
    #[doc = "Checks if the value of the field is `FUNCTIONAL`"]
    #[inline(always)]
    pub fn is_functional(&self) -> bool {
        *self == PEVMODETIME_A::FUNCTIONAL
    }
}
#[doc = "Field `PEVMODETIME` writer - 4:4\\]
Program/Erase Verify Mode Change Time"]
pub type PEVMODETIME_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DFTTIMERCTL_SPEC, PEVMODETIME_A, O>;
impl<'a, const O: u8> PEVMODETIME_W<'a, O> {
    #[doc = "Use 2x the hard-wired (functional) time value"]
    #[inline(always)]
    pub fn twoxfunctional(self) -> &'a mut W {
        self.variant(PEVMODETIME_A::TWOXFUNCTIONAL)
    }
    #[doc = "Use hard-wired (Functional) timer value"]
    #[inline(always)]
    pub fn functional(self) -> &'a mut W {
        self.variant(PEVMODETIME_A::FUNCTIONAL)
    }
}
#[doc = "Field `PEVSETUPTIME` reader - 5:5\\]
Program/Erase Verify Setup Time"]
pub type PEVSETUPTIME_R = crate::BitReader<PEVSETUPTIME_A>;
#[doc = "5:5\\]
Program/Erase Verify Setup Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEVSETUPTIME_A {
    #[doc = "1: Use 2x the hard-wired (functional) time value"]
    TWOXFUNCTIONAL = 1,
    #[doc = "0: Use hard-wired (Functional) timer value"]
    FUNCTIONAL = 0,
}
impl From<PEVSETUPTIME_A> for bool {
    #[inline(always)]
    fn from(variant: PEVSETUPTIME_A) -> Self {
        variant as u8 != 0
    }
}
impl PEVSETUPTIME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEVSETUPTIME_A {
        match self.bits {
            true => PEVSETUPTIME_A::TWOXFUNCTIONAL,
            false => PEVSETUPTIME_A::FUNCTIONAL,
        }
    }
    #[doc = "Checks if the value of the field is `TWOXFUNCTIONAL`"]
    #[inline(always)]
    pub fn is_twoxfunctional(&self) -> bool {
        *self == PEVSETUPTIME_A::TWOXFUNCTIONAL
    }
    #[doc = "Checks if the value of the field is `FUNCTIONAL`"]
    #[inline(always)]
    pub fn is_functional(&self) -> bool {
        *self == PEVSETUPTIME_A::FUNCTIONAL
    }
}
#[doc = "Field `PEVSETUPTIME` writer - 5:5\\]
Program/Erase Verify Setup Time"]
pub type PEVSETUPTIME_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DFTTIMERCTL_SPEC, PEVSETUPTIME_A, O>;
impl<'a, const O: u8> PEVSETUPTIME_W<'a, O> {
    #[doc = "Use 2x the hard-wired (functional) time value"]
    #[inline(always)]
    pub fn twoxfunctional(self) -> &'a mut W {
        self.variant(PEVSETUPTIME_A::TWOXFUNCTIONAL)
    }
    #[doc = "Use hard-wired (Functional) timer value"]
    #[inline(always)]
    pub fn functional(self) -> &'a mut W {
        self.variant(PEVSETUPTIME_A::FUNCTIONAL)
    }
}
#[doc = "Field `PEVHOLDTIME` reader - 6:6\\]
Program/Erase Verify Hold Time"]
pub type PEVHOLDTIME_R = crate::BitReader<PEVHOLDTIME_A>;
#[doc = "6:6\\]
Program/Erase Verify Hold Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEVHOLDTIME_A {
    #[doc = "1: Use 2x the hard-wired (functional) time value"]
    TWOXFUNCTIONAL = 1,
    #[doc = "0: Use hard-wired (Functional) timer value"]
    FUNCTIONAL = 0,
}
impl From<PEVHOLDTIME_A> for bool {
    #[inline(always)]
    fn from(variant: PEVHOLDTIME_A) -> Self {
        variant as u8 != 0
    }
}
impl PEVHOLDTIME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEVHOLDTIME_A {
        match self.bits {
            true => PEVHOLDTIME_A::TWOXFUNCTIONAL,
            false => PEVHOLDTIME_A::FUNCTIONAL,
        }
    }
    #[doc = "Checks if the value of the field is `TWOXFUNCTIONAL`"]
    #[inline(always)]
    pub fn is_twoxfunctional(&self) -> bool {
        *self == PEVHOLDTIME_A::TWOXFUNCTIONAL
    }
    #[doc = "Checks if the value of the field is `FUNCTIONAL`"]
    #[inline(always)]
    pub fn is_functional(&self) -> bool {
        *self == PEVHOLDTIME_A::FUNCTIONAL
    }
}
#[doc = "Field `PEVHOLDTIME` writer - 6:6\\]
Program/Erase Verify Hold Time"]
pub type PEVHOLDTIME_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DFTTIMERCTL_SPEC, PEVHOLDTIME_A, O>;
impl<'a, const O: u8> PEVHOLDTIME_W<'a, O> {
    #[doc = "Use 2x the hard-wired (functional) time value"]
    #[inline(always)]
    pub fn twoxfunctional(self) -> &'a mut W {
        self.variant(PEVHOLDTIME_A::TWOXFUNCTIONAL)
    }
    #[doc = "Use hard-wired (Functional) timer value"]
    #[inline(always)]
    pub fn functional(self) -> &'a mut W {
        self.variant(PEVHOLDTIME_A::FUNCTIONAL)
    }
}
#[doc = "Field `READMODETIME` reader - 7:7\\]
Read Mode Change Time"]
pub type READMODETIME_R = crate::BitReader<READMODETIME_A>;
#[doc = "7:7\\]
Read Mode Change Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum READMODETIME_A {
    #[doc = "1: Use 2x the hard-wired (functional) time value"]
    TWOXFUNCTIONAL = 1,
    #[doc = "0: Use hard-wired (Functional) timer value"]
    FUNCTIONAL = 0,
}
impl From<READMODETIME_A> for bool {
    #[inline(always)]
    fn from(variant: READMODETIME_A) -> Self {
        variant as u8 != 0
    }
}
impl READMODETIME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READMODETIME_A {
        match self.bits {
            true => READMODETIME_A::TWOXFUNCTIONAL,
            false => READMODETIME_A::FUNCTIONAL,
        }
    }
    #[doc = "Checks if the value of the field is `TWOXFUNCTIONAL`"]
    #[inline(always)]
    pub fn is_twoxfunctional(&self) -> bool {
        *self == READMODETIME_A::TWOXFUNCTIONAL
    }
    #[doc = "Checks if the value of the field is `FUNCTIONAL`"]
    #[inline(always)]
    pub fn is_functional(&self) -> bool {
        *self == READMODETIME_A::FUNCTIONAL
    }
}
#[doc = "Field `READMODETIME` writer - 7:7\\]
Read Mode Change Time"]
pub type READMODETIME_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DFTTIMERCTL_SPEC, READMODETIME_A, O>;
impl<'a, const O: u8> READMODETIME_W<'a, O> {
    #[doc = "Use 2x the hard-wired (functional) time value"]
    #[inline(always)]
    pub fn twoxfunctional(self) -> &'a mut W {
        self.variant(READMODETIME_A::TWOXFUNCTIONAL)
    }
    #[doc = "Use hard-wired (Functional) timer value"]
    #[inline(always)]
    pub fn functional(self) -> &'a mut W {
        self.variant(READMODETIME_A::FUNCTIONAL)
    }
}
#[doc = "Field `PEPULSETIMEOVR` reader - 8:8\\]
Override Program/Erase Pulse Time If set, this will force the program or erase pulse time to be overridden with the value in the PEPULSETIMEVAL field. If not set, then a hard-coded value will be used for this pulse time."]
pub type PEPULSETIMEOVR_R = crate::BitReader<PEPULSETIMEOVR_A>;
#[doc = "8:8\\]
Override Program/Erase Pulse Time If set, this will force the program or erase pulse time to be overridden with the value in the PEPULSETIMEVAL field. If not set, then a hard-coded value will be used for this pulse time.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEPULSETIMEOVR_A {
    #[doc = "1: Use value from the PE_PULSE_TIME field for time value"]
    OVERRIDE = 1,
    #[doc = "0: Use hard-wired (Functional) timer value"]
    FUNCTIONAL = 0,
}
impl From<PEPULSETIMEOVR_A> for bool {
    #[inline(always)]
    fn from(variant: PEPULSETIMEOVR_A) -> Self {
        variant as u8 != 0
    }
}
impl PEPULSETIMEOVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEPULSETIMEOVR_A {
        match self.bits {
            true => PEPULSETIMEOVR_A::OVERRIDE,
            false => PEPULSETIMEOVR_A::FUNCTIONAL,
        }
    }
    #[doc = "Checks if the value of the field is `OVERRIDE`"]
    #[inline(always)]
    pub fn is_override(&self) -> bool {
        *self == PEPULSETIMEOVR_A::OVERRIDE
    }
    #[doc = "Checks if the value of the field is `FUNCTIONAL`"]
    #[inline(always)]
    pub fn is_functional(&self) -> bool {
        *self == PEPULSETIMEOVR_A::FUNCTIONAL
    }
}
#[doc = "Field `PEPULSETIMEOVR` writer - 8:8\\]
Override Program/Erase Pulse Time If set, this will force the program or erase pulse time to be overridden with the value in the PEPULSETIMEVAL field. If not set, then a hard-coded value will be used for this pulse time."]
pub type PEPULSETIMEOVR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DFTTIMERCTL_SPEC, PEPULSETIMEOVR_A, O>;
impl<'a, const O: u8> PEPULSETIMEOVR_W<'a, O> {
    #[doc = "Use value from the PE_PULSE_TIME field for time value"]
    #[inline(always)]
    pub fn override_(self) -> &'a mut W {
        self.variant(PEPULSETIMEOVR_A::OVERRIDE)
    }
    #[doc = "Use hard-wired (Functional) timer value"]
    #[inline(always)]
    pub fn functional(self) -> &'a mut W {
        self.variant(PEPULSETIMEOVR_A::FUNCTIONAL)
    }
}
#[doc = "Field `RESERVED9` reader - 11:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED9` writer - 11:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFTTIMERCTL_SPEC, u8, u8, 3, O>;
#[doc = "Field `PEPULSETIMEVAL` reader - 27:12\\]
Program/Erase Pulse Time Value If operation is a program, this value gets loaded into bits \\[15:0\\]
of the timer when the PEPULSETIMEVALOVR field is set to 1. If operation is an erase, this value gets loaded into bits \\[19:4\\]
of the timer when the PEPULSETIMEVALOVR field is set to 1."]
pub type PEPULSETIMEVAL_R = crate::FieldReader<u16, PEPULSETIMEVAL_A>;
#[doc = "27:12\\]
Program/Erase Pulse Time Value If operation is a program, this value gets loaded into bits \\[15:0\\]
of the timer when the PEPULSETIMEVALOVR field is set to 1. If operation is an erase, this value gets loaded into bits \\[19:4\\]
of the timer when the PEPULSETIMEVALOVR field is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PEPULSETIMEVAL_A {
    #[doc = "65535: Maximum value"]
    MAXIMUM = 65535,
    #[doc = "1: Minimum value"]
    MINIMUM = 1,
}
impl From<PEPULSETIMEVAL_A> for u16 {
    #[inline(always)]
    fn from(variant: PEPULSETIMEVAL_A) -> Self {
        variant as _
    }
}
impl PEPULSETIMEVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PEPULSETIMEVAL_A> {
        match self.bits {
            65535 => Some(PEPULSETIMEVAL_A::MAXIMUM),
            1 => Some(PEPULSETIMEVAL_A::MINIMUM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAXIMUM`"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == PEPULSETIMEVAL_A::MAXIMUM
    }
    #[doc = "Checks if the value of the field is `MINIMUM`"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == PEPULSETIMEVAL_A::MINIMUM
    }
}
#[doc = "Field `PEPULSETIMEVAL` writer - 27:12\\]
Program/Erase Pulse Time Value If operation is a program, this value gets loaded into bits \\[15:0\\]
of the timer when the PEPULSETIMEVALOVR field is set to 1. If operation is an erase, this value gets loaded into bits \\[19:4\\]
of the timer when the PEPULSETIMEVALOVR field is set to 1."]
pub type PEPULSETIMEVAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFTTIMERCTL_SPEC, u16, PEPULSETIMEVAL_A, 16, O>;
impl<'a, const O: u8> PEPULSETIMEVAL_W<'a, O> {
    #[doc = "Maximum value"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut W {
        self.variant(PEPULSETIMEVAL_A::MAXIMUM)
    }
    #[doc = "Minimum value"]
    #[inline(always)]
    pub fn minimum(self) -> &'a mut W {
        self.variant(PEPULSETIMEVAL_A::MINIMUM)
    }
}
#[doc = "Field `TIMERCLOCKOVR` reader - 30:28\\]
Override Timer clock frequency using an ICG-based clock divide mechanism. To divide the timer clock, pulses can be skipped based on settings in this field. By default, this field is 0, which corresponds to no division on the timer clock."]
pub type TIMERCLOCKOVR_R = crate::FieldReader<u8, TIMERCLOCKOVR_A>;
#[doc = "30:28\\]
Override Timer clock frequency using an ICG-based clock divide mechanism. To divide the timer clock, pulses can be skipped based on settings in this field. By default, this field is 0, which corresponds to no division on the timer clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMERCLOCKOVR_A {
    #[doc = "7: Divide timer clock by 8"]
    DIVIDEBY8 = 7,
    #[doc = "6: Divide timer clock by 7"]
    DIVIDEBY7 = 6,
    #[doc = "5: Divide timer clock by 6"]
    DIVIDEBY6 = 5,
    #[doc = "4: Divide timer clock by 5"]
    DIVIDEBY5 = 4,
    #[doc = "3: Divide timer clock by 4"]
    DIVIDEBY4 = 3,
    #[doc = "2: Divide timer clock by 3"]
    DIVIDEBY3 = 2,
    #[doc = "1: Divide timer clock by 2"]
    DIVIDEBY2 = 1,
    #[doc = "0: No divide on timer clock."]
    NODIVIDE = 0,
}
impl From<TIMERCLOCKOVR_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMERCLOCKOVR_A) -> Self {
        variant as _
    }
}
impl TIMERCLOCKOVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMERCLOCKOVR_A {
        match self.bits {
            7 => TIMERCLOCKOVR_A::DIVIDEBY8,
            6 => TIMERCLOCKOVR_A::DIVIDEBY7,
            5 => TIMERCLOCKOVR_A::DIVIDEBY6,
            4 => TIMERCLOCKOVR_A::DIVIDEBY5,
            3 => TIMERCLOCKOVR_A::DIVIDEBY4,
            2 => TIMERCLOCKOVR_A::DIVIDEBY3,
            1 => TIMERCLOCKOVR_A::DIVIDEBY2,
            0 => TIMERCLOCKOVR_A::NODIVIDE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY8`"]
    #[inline(always)]
    pub fn is_divideby8(&self) -> bool {
        *self == TIMERCLOCKOVR_A::DIVIDEBY8
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY7`"]
    #[inline(always)]
    pub fn is_divideby7(&self) -> bool {
        *self == TIMERCLOCKOVR_A::DIVIDEBY7
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY6`"]
    #[inline(always)]
    pub fn is_divideby6(&self) -> bool {
        *self == TIMERCLOCKOVR_A::DIVIDEBY6
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY5`"]
    #[inline(always)]
    pub fn is_divideby5(&self) -> bool {
        *self == TIMERCLOCKOVR_A::DIVIDEBY5
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY4`"]
    #[inline(always)]
    pub fn is_divideby4(&self) -> bool {
        *self == TIMERCLOCKOVR_A::DIVIDEBY4
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY3`"]
    #[inline(always)]
    pub fn is_divideby3(&self) -> bool {
        *self == TIMERCLOCKOVR_A::DIVIDEBY3
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY2`"]
    #[inline(always)]
    pub fn is_divideby2(&self) -> bool {
        *self == TIMERCLOCKOVR_A::DIVIDEBY2
    }
    #[doc = "Checks if the value of the field is `NODIVIDE`"]
    #[inline(always)]
    pub fn is_nodivide(&self) -> bool {
        *self == TIMERCLOCKOVR_A::NODIVIDE
    }
}
#[doc = "Field `TIMERCLOCKOVR` writer - 30:28\\]
Override Timer clock frequency using an ICG-based clock divide mechanism. To divide the timer clock, pulses can be skipped based on settings in this field. By default, this field is 0, which corresponds to no division on the timer clock."]
pub type TIMERCLOCKOVR_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DFTTIMERCTL_SPEC, u8, TIMERCLOCKOVR_A, 3, O>;
impl<'a, const O: u8> TIMERCLOCKOVR_W<'a, O> {
    #[doc = "Divide timer clock by 8"]
    #[inline(always)]
    pub fn divideby8(self) -> &'a mut W {
        self.variant(TIMERCLOCKOVR_A::DIVIDEBY8)
    }
    #[doc = "Divide timer clock by 7"]
    #[inline(always)]
    pub fn divideby7(self) -> &'a mut W {
        self.variant(TIMERCLOCKOVR_A::DIVIDEBY7)
    }
    #[doc = "Divide timer clock by 6"]
    #[inline(always)]
    pub fn divideby6(self) -> &'a mut W {
        self.variant(TIMERCLOCKOVR_A::DIVIDEBY6)
    }
    #[doc = "Divide timer clock by 5"]
    #[inline(always)]
    pub fn divideby5(self) -> &'a mut W {
        self.variant(TIMERCLOCKOVR_A::DIVIDEBY5)
    }
    #[doc = "Divide timer clock by 4"]
    #[inline(always)]
    pub fn divideby4(self) -> &'a mut W {
        self.variant(TIMERCLOCKOVR_A::DIVIDEBY4)
    }
    #[doc = "Divide timer clock by 3"]
    #[inline(always)]
    pub fn divideby3(self) -> &'a mut W {
        self.variant(TIMERCLOCKOVR_A::DIVIDEBY3)
    }
    #[doc = "Divide timer clock by 2"]
    #[inline(always)]
    pub fn divideby2(self) -> &'a mut W {
        self.variant(TIMERCLOCKOVR_A::DIVIDEBY2)
    }
    #[doc = "No divide on timer clock."]
    #[inline(always)]
    pub fn nodivide(self) -> &'a mut W {
        self.variant(TIMERCLOCKOVR_A::NODIVIDE)
    }
}
#[doc = "Field `RESERVED31` reader - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED31_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED31` writer - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED31_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFTTIMERCTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Program/Erase Setup Time"]
    #[inline(always)]
    pub fn pesetuptime(&self) -> PESETUPTIME_R {
        PESETUPTIME_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Program VHV Setup Time"]
    #[inline(always)]
    pub fn pvhvsetuptime(&self) -> PVHVSETUPTIME_R {
        PVHVSETUPTIME_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Program and Program Verify Wordline Switching Time"]
    #[inline(always)]
    pub fn ppvwordlinetime(&self) -> PPVWORDLINETIME_R {
        PPVWORDLINETIME_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Program/Erase Hold Time"]
    #[inline(always)]
    pub fn peholdtime(&self) -> PEHOLDTIME_R {
        PEHOLDTIME_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Program/Erase Verify Mode Change Time"]
    #[inline(always)]
    pub fn pevmodetime(&self) -> PEVMODETIME_R {
        PEVMODETIME_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Program/Erase Verify Setup Time"]
    #[inline(always)]
    pub fn pevsetuptime(&self) -> PEVSETUPTIME_R {
        PEVSETUPTIME_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Program/Erase Verify Hold Time"]
    #[inline(always)]
    pub fn pevholdtime(&self) -> PEVHOLDTIME_R {
        PEVHOLDTIME_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Read Mode Change Time"]
    #[inline(always)]
    pub fn readmodetime(&self) -> READMODETIME_R {
        READMODETIME_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Override Program/Erase Pulse Time If set, this will force the program or erase pulse time to be overridden with the value in the PEPULSETIMEVAL field. If not set, then a hard-coded value will be used for this pulse time."]
    #[inline(always)]
    pub fn pepulsetimeovr(&self) -> PEPULSETIMEOVR_R {
        PEPULSETIMEOVR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - 11:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:27 - 27:12\\]
Program/Erase Pulse Time Value If operation is a program, this value gets loaded into bits \\[15:0\\]
of the timer when the PEPULSETIMEVALOVR field is set to 1. If operation is an erase, this value gets loaded into bits \\[19:4\\]
of the timer when the PEPULSETIMEVALOVR field is set to 1."]
    #[inline(always)]
    pub fn pepulsetimeval(&self) -> PEPULSETIMEVAL_R {
        PEPULSETIMEVAL_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bits 28:30 - 30:28\\]
Override Timer clock frequency using an ICG-based clock divide mechanism. To divide the timer clock, pulses can be skipped based on settings in this field. By default, this field is 0, which corresponds to no division on the timer clock."]
    #[inline(always)]
    pub fn timerclockovr(&self) -> TIMERCLOCKOVR_R {
        TIMERCLOCKOVR_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved31(&self) -> RESERVED31_R {
        RESERVED31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Program/Erase Setup Time"]
    #[inline(always)]
    #[must_use]
    pub fn pesetuptime(&mut self) -> PESETUPTIME_W<0> {
        PESETUPTIME_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Program VHV Setup Time"]
    #[inline(always)]
    #[must_use]
    pub fn pvhvsetuptime(&mut self) -> PVHVSETUPTIME_W<1> {
        PVHVSETUPTIME_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Program and Program Verify Wordline Switching Time"]
    #[inline(always)]
    #[must_use]
    pub fn ppvwordlinetime(&mut self) -> PPVWORDLINETIME_W<2> {
        PPVWORDLINETIME_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Program/Erase Hold Time"]
    #[inline(always)]
    #[must_use]
    pub fn peholdtime(&mut self) -> PEHOLDTIME_W<3> {
        PEHOLDTIME_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Program/Erase Verify Mode Change Time"]
    #[inline(always)]
    #[must_use]
    pub fn pevmodetime(&mut self) -> PEVMODETIME_W<4> {
        PEVMODETIME_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Program/Erase Verify Setup Time"]
    #[inline(always)]
    #[must_use]
    pub fn pevsetuptime(&mut self) -> PEVSETUPTIME_W<5> {
        PEVSETUPTIME_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Program/Erase Verify Hold Time"]
    #[inline(always)]
    #[must_use]
    pub fn pevholdtime(&mut self) -> PEVHOLDTIME_W<6> {
        PEVHOLDTIME_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Read Mode Change Time"]
    #[inline(always)]
    #[must_use]
    pub fn readmodetime(&mut self) -> READMODETIME_W<7> {
        READMODETIME_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Override Program/Erase Pulse Time If set, this will force the program or erase pulse time to be overridden with the value in the PEPULSETIMEVAL field. If not set, then a hard-coded value will be used for this pulse time."]
    #[inline(always)]
    #[must_use]
    pub fn pepulsetimeovr(&mut self) -> PEPULSETIMEOVR_W<8> {
        PEPULSETIMEOVR_W::new(self)
    }
    #[doc = "Bits 9:11 - 11:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> RESERVED9_W<9> {
        RESERVED9_W::new(self)
    }
    #[doc = "Bits 12:27 - 27:12\\]
Program/Erase Pulse Time Value If operation is a program, this value gets loaded into bits \\[15:0\\]
of the timer when the PEPULSETIMEVALOVR field is set to 1. If operation is an erase, this value gets loaded into bits \\[19:4\\]
of the timer when the PEPULSETIMEVALOVR field is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn pepulsetimeval(&mut self) -> PEPULSETIMEVAL_W<12> {
        PEPULSETIMEVAL_W::new(self)
    }
    #[doc = "Bits 28:30 - 30:28\\]
Override Timer clock frequency using an ICG-based clock divide mechanism. To divide the timer clock, pulses can be skipped based on settings in this field. By default, this field is 0, which corresponds to no division on the timer clock."]
    #[inline(always)]
    #[must_use]
    pub fn timerclockovr(&mut self) -> TIMERCLOCKOVR_W<28> {
        TIMERCLOCKOVR_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved31(&mut self) -> RESERVED31_W<31> {
        RESERVED31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFT Timer Control Register This allows some configuration of timing values for various phases of flash operations for test. These time values are hard-coded for functional execution. This register is only writable when DFT.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfttimerctl](index.html) module"]
pub struct DFTTIMERCTL_SPEC;
impl crate::RegisterSpec for DFTTIMERCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfttimerctl::R](R) reader structure"]
impl crate::Readable for DFTTIMERCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfttimerctl::W](W) writer structure"]
impl crate::Writable for DFTTIMERCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFTTIMERCTL to value 0"]
impl crate::Resettable for DFTTIMERCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
