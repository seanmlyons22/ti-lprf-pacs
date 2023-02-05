#[doc = "Register `STATCMD` reader"]
pub struct R(crate::R<STATCMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATCMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATCMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATCMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATCMD` writer"]
pub struct W(crate::W<STATCMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATCMD_SPEC>;
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
impl From<crate::W<STATCMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATCMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDDONE` reader - 0:0\\]
Command Done"]
pub type CMDDONE_R = crate::BitReader<CMDDONE_A>;
#[doc = "0:0\\]
Command Done\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDDONE_A {
    #[doc = "1: Done"]
    STATDONE = 1,
    #[doc = "0: Not Done"]
    STATNOTDONE = 0,
}
impl From<CMDDONE_A> for bool {
    #[inline(always)]
    fn from(variant: CMDDONE_A) -> Self {
        variant as u8 != 0
    }
}
impl CMDDONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDDONE_A {
        match self.bits {
            true => CMDDONE_A::STATDONE,
            false => CMDDONE_A::STATNOTDONE,
        }
    }
    #[doc = "Checks if the value of the field is `STATDONE`"]
    #[inline(always)]
    pub fn is_statdone(&self) -> bool {
        *self == CMDDONE_A::STATDONE
    }
    #[doc = "Checks if the value of the field is `STATNOTDONE`"]
    #[inline(always)]
    pub fn is_statnotdone(&self) -> bool {
        *self == CMDDONE_A::STATNOTDONE
    }
}
#[doc = "Field `CMDDONE` writer - 0:0\\]
Command Done"]
pub type CMDDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATCMD_SPEC, CMDDONE_A, O>;
impl<'a, const O: u8> CMDDONE_W<'a, O> {
    #[doc = "Done"]
    #[inline(always)]
    pub fn statdone(self) -> &'a mut W {
        self.variant(CMDDONE_A::STATDONE)
    }
    #[doc = "Not Done"]
    #[inline(always)]
    pub fn statnotdone(self) -> &'a mut W {
        self.variant(CMDDONE_A::STATNOTDONE)
    }
}
#[doc = "Field `CMDPASS` reader - 1:1\\]
Command Pass - valid when CMD_DONE field is 1"]
pub type CMDPASS_R = crate::BitReader<CMDPASS_A>;
#[doc = "1:1\\]
Command Pass - valid when CMD_DONE field is 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDPASS_A {
    #[doc = "1: Pass"]
    STATPASS = 1,
    #[doc = "0: Fail"]
    STATFAIL = 0,
}
impl From<CMDPASS_A> for bool {
    #[inline(always)]
    fn from(variant: CMDPASS_A) -> Self {
        variant as u8 != 0
    }
}
impl CMDPASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDPASS_A {
        match self.bits {
            true => CMDPASS_A::STATPASS,
            false => CMDPASS_A::STATFAIL,
        }
    }
    #[doc = "Checks if the value of the field is `STATPASS`"]
    #[inline(always)]
    pub fn is_statpass(&self) -> bool {
        *self == CMDPASS_A::STATPASS
    }
    #[doc = "Checks if the value of the field is `STATFAIL`"]
    #[inline(always)]
    pub fn is_statfail(&self) -> bool {
        *self == CMDPASS_A::STATFAIL
    }
}
#[doc = "Field `CMDPASS` writer - 1:1\\]
Command Pass - valid when CMD_DONE field is 1"]
pub type CMDPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATCMD_SPEC, CMDPASS_A, O>;
impl<'a, const O: u8> CMDPASS_W<'a, O> {
    #[doc = "Pass"]
    #[inline(always)]
    pub fn statpass(self) -> &'a mut W {
        self.variant(CMDPASS_A::STATPASS)
    }
    #[doc = "Fail"]
    #[inline(always)]
    pub fn statfail(self) -> &'a mut W {
        self.variant(CMDPASS_A::STATFAIL)
    }
}
#[doc = "Field `CMDINPROGRESS` reader - 2:2\\]
Command In Progress"]
pub type CMDINPROGRESS_R = crate::BitReader<CMDINPROGRESS_A>;
#[doc = "2:2\\]
Command In Progress\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDINPROGRESS_A {
    #[doc = "1: In Progress"]
    STATINPROGRESS = 1,
    #[doc = "0: Complete"]
    STATCOMPLETE = 0,
}
impl From<CMDINPROGRESS_A> for bool {
    #[inline(always)]
    fn from(variant: CMDINPROGRESS_A) -> Self {
        variant as u8 != 0
    }
}
impl CMDINPROGRESS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDINPROGRESS_A {
        match self.bits {
            true => CMDINPROGRESS_A::STATINPROGRESS,
            false => CMDINPROGRESS_A::STATCOMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `STATINPROGRESS`"]
    #[inline(always)]
    pub fn is_statinprogress(&self) -> bool {
        *self == CMDINPROGRESS_A::STATINPROGRESS
    }
    #[doc = "Checks if the value of the field is `STATCOMPLETE`"]
    #[inline(always)]
    pub fn is_statcomplete(&self) -> bool {
        *self == CMDINPROGRESS_A::STATCOMPLETE
    }
}
#[doc = "Field `CMDINPROGRESS` writer - 2:2\\]
Command In Progress"]
pub type CMDINPROGRESS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STATCMD_SPEC, CMDINPROGRESS_A, O>;
impl<'a, const O: u8> CMDINPROGRESS_W<'a, O> {
    #[doc = "In Progress"]
    #[inline(always)]
    pub fn statinprogress(self) -> &'a mut W {
        self.variant(CMDINPROGRESS_A::STATINPROGRESS)
    }
    #[doc = "Complete"]
    #[inline(always)]
    pub fn statcomplete(self) -> &'a mut W {
        self.variant(CMDINPROGRESS_A::STATCOMPLETE)
    }
}
#[doc = "Field `RESERVED3` reader - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED3` writer - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATCMD_SPEC, bool, O>;
#[doc = "Field `FAILWEPROT` reader - 4:4\\]
Command failed due to Write/Erase Protect Sector Violation"]
pub type FAILWEPROT_R = crate::BitReader<FAILWEPROT_A>;
#[doc = "4:4\\]
Command failed due to Write/Erase Protect Sector Violation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FAILWEPROT_A {
    #[doc = "1: Fail"]
    STATFAIL = 1,
    #[doc = "0: No Fail"]
    STATNOFAIL = 0,
}
impl From<FAILWEPROT_A> for bool {
    #[inline(always)]
    fn from(variant: FAILWEPROT_A) -> Self {
        variant as u8 != 0
    }
}
impl FAILWEPROT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAILWEPROT_A {
        match self.bits {
            true => FAILWEPROT_A::STATFAIL,
            false => FAILWEPROT_A::STATNOFAIL,
        }
    }
    #[doc = "Checks if the value of the field is `STATFAIL`"]
    #[inline(always)]
    pub fn is_statfail(&self) -> bool {
        *self == FAILWEPROT_A::STATFAIL
    }
    #[doc = "Checks if the value of the field is `STATNOFAIL`"]
    #[inline(always)]
    pub fn is_statnofail(&self) -> bool {
        *self == FAILWEPROT_A::STATNOFAIL
    }
}
#[doc = "Field `FAILWEPROT` writer - 4:4\\]
Command failed due to Write/Erase Protect Sector Violation"]
pub type FAILWEPROT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATCMD_SPEC, FAILWEPROT_A, O>;
impl<'a, const O: u8> FAILWEPROT_W<'a, O> {
    #[doc = "Fail"]
    #[inline(always)]
    pub fn statfail(self) -> &'a mut W {
        self.variant(FAILWEPROT_A::STATFAIL)
    }
    #[doc = "No Fail"]
    #[inline(always)]
    pub fn statnofail(self) -> &'a mut W {
        self.variant(FAILWEPROT_A::STATNOFAIL)
    }
}
#[doc = "Field `FAILVERIFY` reader - 5:5\\]
Command failed due to verify error"]
pub type FAILVERIFY_R = crate::BitReader<FAILVERIFY_A>;
#[doc = "5:5\\]
Command failed due to verify error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FAILVERIFY_A {
    #[doc = "1: Fail"]
    STATFAIL = 1,
    #[doc = "0: No Fail"]
    STATNOFAIL = 0,
}
impl From<FAILVERIFY_A> for bool {
    #[inline(always)]
    fn from(variant: FAILVERIFY_A) -> Self {
        variant as u8 != 0
    }
}
impl FAILVERIFY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAILVERIFY_A {
        match self.bits {
            true => FAILVERIFY_A::STATFAIL,
            false => FAILVERIFY_A::STATNOFAIL,
        }
    }
    #[doc = "Checks if the value of the field is `STATFAIL`"]
    #[inline(always)]
    pub fn is_statfail(&self) -> bool {
        *self == FAILVERIFY_A::STATFAIL
    }
    #[doc = "Checks if the value of the field is `STATNOFAIL`"]
    #[inline(always)]
    pub fn is_statnofail(&self) -> bool {
        *self == FAILVERIFY_A::STATNOFAIL
    }
}
#[doc = "Field `FAILVERIFY` writer - 5:5\\]
Command failed due to verify error"]
pub type FAILVERIFY_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATCMD_SPEC, FAILVERIFY_A, O>;
impl<'a, const O: u8> FAILVERIFY_W<'a, O> {
    #[doc = "Fail"]
    #[inline(always)]
    pub fn statfail(self) -> &'a mut W {
        self.variant(FAILVERIFY_A::STATFAIL)
    }
    #[doc = "No Fail"]
    #[inline(always)]
    pub fn statnofail(self) -> &'a mut W {
        self.variant(FAILVERIFY_A::STATNOFAIL)
    }
}
#[doc = "Field `FAILILLADDR` reader - 6:6\\]
Command failed due to the use of an illegal address"]
pub type FAILILLADDR_R = crate::BitReader<FAILILLADDR_A>;
#[doc = "6:6\\]
Command failed due to the use of an illegal address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FAILILLADDR_A {
    #[doc = "1: Fail"]
    STATFAIL = 1,
    #[doc = "0: No Fail"]
    STATNOFAIL = 0,
}
impl From<FAILILLADDR_A> for bool {
    #[inline(always)]
    fn from(variant: FAILILLADDR_A) -> Self {
        variant as u8 != 0
    }
}
impl FAILILLADDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAILILLADDR_A {
        match self.bits {
            true => FAILILLADDR_A::STATFAIL,
            false => FAILILLADDR_A::STATNOFAIL,
        }
    }
    #[doc = "Checks if the value of the field is `STATFAIL`"]
    #[inline(always)]
    pub fn is_statfail(&self) -> bool {
        *self == FAILILLADDR_A::STATFAIL
    }
    #[doc = "Checks if the value of the field is `STATNOFAIL`"]
    #[inline(always)]
    pub fn is_statnofail(&self) -> bool {
        *self == FAILILLADDR_A::STATNOFAIL
    }
}
#[doc = "Field `FAILILLADDR` writer - 6:6\\]
Command failed due to the use of an illegal address"]
pub type FAILILLADDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATCMD_SPEC, FAILILLADDR_A, O>;
impl<'a, const O: u8> FAILILLADDR_W<'a, O> {
    #[doc = "Fail"]
    #[inline(always)]
    pub fn statfail(self) -> &'a mut W {
        self.variant(FAILILLADDR_A::STATFAIL)
    }
    #[doc = "No Fail"]
    #[inline(always)]
    pub fn statnofail(self) -> &'a mut W {
        self.variant(FAILILLADDR_A::STATNOFAIL)
    }
}
#[doc = "Field `FAILMODE` reader - 7:7\\]
Command failed because a bank has been set to a mode other than READ. Program and Erase commands cannot be initiated unless all banks are in READ mode."]
pub type FAILMODE_R = crate::BitReader<FAILMODE_A>;
#[doc = "7:7\\]
Command failed because a bank has been set to a mode other than READ. Program and Erase commands cannot be initiated unless all banks are in READ mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FAILMODE_A {
    #[doc = "1: Fail"]
    STATFAIL = 1,
    #[doc = "0: No Fail"]
    STATNOFAIL = 0,
}
impl From<FAILMODE_A> for bool {
    #[inline(always)]
    fn from(variant: FAILMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl FAILMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAILMODE_A {
        match self.bits {
            true => FAILMODE_A::STATFAIL,
            false => FAILMODE_A::STATNOFAIL,
        }
    }
    #[doc = "Checks if the value of the field is `STATFAIL`"]
    #[inline(always)]
    pub fn is_statfail(&self) -> bool {
        *self == FAILMODE_A::STATFAIL
    }
    #[doc = "Checks if the value of the field is `STATNOFAIL`"]
    #[inline(always)]
    pub fn is_statnofail(&self) -> bool {
        *self == FAILMODE_A::STATNOFAIL
    }
}
#[doc = "Field `FAILMODE` writer - 7:7\\]
Command failed because a bank has been set to a mode other than READ. Program and Erase commands cannot be initiated unless all banks are in READ mode."]
pub type FAILMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATCMD_SPEC, FAILMODE_A, O>;
impl<'a, const O: u8> FAILMODE_W<'a, O> {
    #[doc = "Fail"]
    #[inline(always)]
    pub fn statfail(self) -> &'a mut W {
        self.variant(FAILMODE_A::STATFAIL)
    }
    #[doc = "No Fail"]
    #[inline(always)]
    pub fn statnofail(self) -> &'a mut W {
        self.variant(FAILMODE_A::STATNOFAIL)
    }
}
#[doc = "Field `FAILINVDATA` reader - 8:8\\]
Program command failed because an attempt was made to program a stored 0 value to a 1."]
pub type FAILINVDATA_R = crate::BitReader<FAILINVDATA_A>;
#[doc = "8:8\\]
Program command failed because an attempt was made to program a stored 0 value to a 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FAILINVDATA_A {
    #[doc = "1: Fail"]
    STATFAIL = 1,
    #[doc = "0: No Fail"]
    STATNOFAIL = 0,
}
impl From<FAILINVDATA_A> for bool {
    #[inline(always)]
    fn from(variant: FAILINVDATA_A) -> Self {
        variant as u8 != 0
    }
}
impl FAILINVDATA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAILINVDATA_A {
        match self.bits {
            true => FAILINVDATA_A::STATFAIL,
            false => FAILINVDATA_A::STATNOFAIL,
        }
    }
    #[doc = "Checks if the value of the field is `STATFAIL`"]
    #[inline(always)]
    pub fn is_statfail(&self) -> bool {
        *self == FAILINVDATA_A::STATFAIL
    }
    #[doc = "Checks if the value of the field is `STATNOFAIL`"]
    #[inline(always)]
    pub fn is_statnofail(&self) -> bool {
        *self == FAILINVDATA_A::STATNOFAIL
    }
}
#[doc = "Field `FAILINVDATA` writer - 8:8\\]
Program command failed because an attempt was made to program a stored 0 value to a 1."]
pub type FAILINVDATA_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATCMD_SPEC, FAILINVDATA_A, O>;
impl<'a, const O: u8> FAILINVDATA_W<'a, O> {
    #[doc = "Fail"]
    #[inline(always)]
    pub fn statfail(self) -> &'a mut W {
        self.variant(FAILINVDATA_A::STATFAIL)
    }
    #[doc = "No Fail"]
    #[inline(always)]
    pub fn statnofail(self) -> &'a mut W {
        self.variant(FAILINVDATA_A::STATNOFAIL)
    }
}
#[doc = "Field `FAILMISC` reader - 12:12\\]
Command failed due to error other than write/erase protect violation or verify error. This is an extra bit in case a new failure mechanism is added which requires a status bit."]
pub type FAILMISC_R = crate::BitReader<FAILMISC_A>;
#[doc = "12:12\\]
Command failed due to error other than write/erase protect violation or verify error. This is an extra bit in case a new failure mechanism is added which requires a status bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FAILMISC_A {
    #[doc = "1: Fail"]
    STATFAIL = 1,
    #[doc = "0: No Fail"]
    STATNOFAIL = 0,
}
impl From<FAILMISC_A> for bool {
    #[inline(always)]
    fn from(variant: FAILMISC_A) -> Self {
        variant as u8 != 0
    }
}
impl FAILMISC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAILMISC_A {
        match self.bits {
            true => FAILMISC_A::STATFAIL,
            false => FAILMISC_A::STATNOFAIL,
        }
    }
    #[doc = "Checks if the value of the field is `STATFAIL`"]
    #[inline(always)]
    pub fn is_statfail(&self) -> bool {
        *self == FAILMISC_A::STATFAIL
    }
    #[doc = "Checks if the value of the field is `STATNOFAIL`"]
    #[inline(always)]
    pub fn is_statnofail(&self) -> bool {
        *self == FAILMISC_A::STATNOFAIL
    }
}
#[doc = "Field `FAILMISC` writer - 12:12\\]
Command failed due to error other than write/erase protect violation or verify error. This is an extra bit in case a new failure mechanism is added which requires a status bit."]
pub type FAILMISC_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATCMD_SPEC, FAILMISC_A, O>;
impl<'a, const O: u8> FAILMISC_W<'a, O> {
    #[doc = "Fail"]
    #[inline(always)]
    pub fn statfail(self) -> &'a mut W {
        self.variant(FAILMISC_A::STATFAIL)
    }
    #[doc = "No Fail"]
    #[inline(always)]
    pub fn statnofail(self) -> &'a mut W {
        self.variant(FAILMISC_A::STATNOFAIL)
    }
}
#[doc = "Field `RESERVED13` reader - 31:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED13_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED13` writer - 31:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STATCMD_SPEC, u32, u32, 19, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Command Done"]
    #[inline(always)]
    pub fn cmddone(&self) -> CMDDONE_R {
        CMDDONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Command Pass - valid when CMD_DONE field is 1"]
    #[inline(always)]
    pub fn cmdpass(&self) -> CMDPASS_R {
        CMDPASS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Command In Progress"]
    #[inline(always)]
    pub fn cmdinprogress(&self) -> CMDINPROGRESS_R {
        CMDINPROGRESS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Command failed due to Write/Erase Protect Sector Violation"]
    #[inline(always)]
    pub fn failweprot(&self) -> FAILWEPROT_R {
        FAILWEPROT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Command failed due to verify error"]
    #[inline(always)]
    pub fn failverify(&self) -> FAILVERIFY_R {
        FAILVERIFY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Command failed due to the use of an illegal address"]
    #[inline(always)]
    pub fn faililladdr(&self) -> FAILILLADDR_R {
        FAILILLADDR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Command failed because a bank has been set to a mode other than READ. Program and Erase commands cannot be initiated unless all banks are in READ mode."]
    #[inline(always)]
    pub fn failmode(&self) -> FAILMODE_R {
        FAILMODE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Program command failed because an attempt was made to program a stored 0 value to a 1."]
    #[inline(always)]
    pub fn failinvdata(&self) -> FAILINVDATA_R {
        FAILINVDATA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Command failed due to error other than write/erase protect violation or verify error. This is an extra bit in case a new failure mechanism is added which requires a status bit."]
    #[inline(always)]
    pub fn failmisc(&self) -> FAILMISC_R {
        FAILMISC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:31 - 31:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved13(&self) -> RESERVED13_R {
        RESERVED13_R::new((self.bits >> 13) & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Command Done"]
    #[inline(always)]
    #[must_use]
    pub fn cmddone(&mut self) -> CMDDONE_W<0> {
        CMDDONE_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Command Pass - valid when CMD_DONE field is 1"]
    #[inline(always)]
    #[must_use]
    pub fn cmdpass(&mut self) -> CMDPASS_W<1> {
        CMDPASS_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Command In Progress"]
    #[inline(always)]
    #[must_use]
    pub fn cmdinprogress(&mut self) -> CMDINPROGRESS_W<2> {
        CMDINPROGRESS_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<3> {
        RESERVED3_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Command failed due to Write/Erase Protect Sector Violation"]
    #[inline(always)]
    #[must_use]
    pub fn failweprot(&mut self) -> FAILWEPROT_W<4> {
        FAILWEPROT_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Command failed due to verify error"]
    #[inline(always)]
    #[must_use]
    pub fn failverify(&mut self) -> FAILVERIFY_W<5> {
        FAILVERIFY_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Command failed due to the use of an illegal address"]
    #[inline(always)]
    #[must_use]
    pub fn faililladdr(&mut self) -> FAILILLADDR_W<6> {
        FAILILLADDR_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Command failed because a bank has been set to a mode other than READ. Program and Erase commands cannot be initiated unless all banks are in READ mode."]
    #[inline(always)]
    #[must_use]
    pub fn failmode(&mut self) -> FAILMODE_W<7> {
        FAILMODE_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Program command failed because an attempt was made to program a stored 0 value to a 1."]
    #[inline(always)]
    #[must_use]
    pub fn failinvdata(&mut self) -> FAILINVDATA_W<8> {
        FAILINVDATA_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
Command failed due to error other than write/erase protect violation or verify error. This is an extra bit in case a new failure mechanism is added which requires a status bit."]
    #[inline(always)]
    #[must_use]
    pub fn failmisc(&mut self) -> FAILMISC_W<12> {
        FAILMISC_W::new(self)
    }
    #[doc = "Bits 13:31 - 31:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved13(&mut self) -> RESERVED13_W<13> {
        RESERVED13_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command Status Register This register contains status regarding completion and errors of command execution.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statcmd](index.html) module"]
pub struct STATCMD_SPEC;
impl crate::RegisterSpec for STATCMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [statcmd::R](R) reader structure"]
impl crate::Readable for STATCMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [statcmd::W](W) writer structure"]
impl crate::Writable for STATCMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATCMD to value 0"]
impl crate::Resettable for STATCMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
