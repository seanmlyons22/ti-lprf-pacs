#[doc = "Register `STATCMD` reader"]
pub type R = crate::R<StatcmdSpec>;
#[doc = "Register `STATCMD` writer"]
pub type W = crate::W<StatcmdSpec>;
#[doc = "0:0\\]
Command Done\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmddone {
    #[doc = "1: Done"]
    Statdone = 1,
    #[doc = "0: Not Done"]
    Statnotdone = 0,
}
impl From<Cmddone> for bool {
    #[inline(always)]
    fn from(variant: Cmddone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDDONE` reader - 0:0\\]
Command Done"]
pub type CmddoneR = crate::BitReader<Cmddone>;
impl CmddoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmddone {
        match self.bits {
            true => Cmddone::Statdone,
            false => Cmddone::Statnotdone,
        }
    }
    #[doc = "Done"]
    #[inline(always)]
    pub fn is_statdone(&self) -> bool {
        *self == Cmddone::Statdone
    }
    #[doc = "Not Done"]
    #[inline(always)]
    pub fn is_statnotdone(&self) -> bool {
        *self == Cmddone::Statnotdone
    }
}
#[doc = "1:1\\]
Command Pass - valid when CMD_DONE field is 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdpass {
    #[doc = "1: Pass"]
    Statpass = 1,
    #[doc = "0: Fail"]
    Statfail = 0,
}
impl From<Cmdpass> for bool {
    #[inline(always)]
    fn from(variant: Cmdpass) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDPASS` reader - 1:1\\]
Command Pass - valid when CMD_DONE field is 1"]
pub type CmdpassR = crate::BitReader<Cmdpass>;
impl CmdpassR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdpass {
        match self.bits {
            true => Cmdpass::Statpass,
            false => Cmdpass::Statfail,
        }
    }
    #[doc = "Pass"]
    #[inline(always)]
    pub fn is_statpass(&self) -> bool {
        *self == Cmdpass::Statpass
    }
    #[doc = "Fail"]
    #[inline(always)]
    pub fn is_statfail(&self) -> bool {
        *self == Cmdpass::Statfail
    }
}
#[doc = "2:2\\]
Command In Progress\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdinprogress {
    #[doc = "1: In Progress"]
    Statinprogress = 1,
    #[doc = "0: Complete"]
    Statcomplete = 0,
}
impl From<Cmdinprogress> for bool {
    #[inline(always)]
    fn from(variant: Cmdinprogress) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDINPROGRESS` reader - 2:2\\]
Command In Progress"]
pub type CmdinprogressR = crate::BitReader<Cmdinprogress>;
impl CmdinprogressR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdinprogress {
        match self.bits {
            true => Cmdinprogress::Statinprogress,
            false => Cmdinprogress::Statcomplete,
        }
    }
    #[doc = "In Progress"]
    #[inline(always)]
    pub fn is_statinprogress(&self) -> bool {
        *self == Cmdinprogress::Statinprogress
    }
    #[doc = "Complete"]
    #[inline(always)]
    pub fn is_statcomplete(&self) -> bool {
        *self == Cmdinprogress::Statcomplete
    }
}
#[doc = "Field `RESERVED3` reader - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::BitReader;
#[doc = "4:4\\]
Command failed due to Write/Erase Protect Sector Violation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Failweprot {
    #[doc = "1: Fail"]
    Statfail = 1,
    #[doc = "0: No Fail"]
    Statnofail = 0,
}
impl From<Failweprot> for bool {
    #[inline(always)]
    fn from(variant: Failweprot) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAILWEPROT` reader - 4:4\\]
Command failed due to Write/Erase Protect Sector Violation"]
pub type FailweprotR = crate::BitReader<Failweprot>;
impl FailweprotR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Failweprot {
        match self.bits {
            true => Failweprot::Statfail,
            false => Failweprot::Statnofail,
        }
    }
    #[doc = "Fail"]
    #[inline(always)]
    pub fn is_statfail(&self) -> bool {
        *self == Failweprot::Statfail
    }
    #[doc = "No Fail"]
    #[inline(always)]
    pub fn is_statnofail(&self) -> bool {
        *self == Failweprot::Statnofail
    }
}
#[doc = "5:5\\]
Command failed due to verify error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Failverify {
    #[doc = "1: Fail"]
    Statfail = 1,
    #[doc = "0: No Fail"]
    Statnofail = 0,
}
impl From<Failverify> for bool {
    #[inline(always)]
    fn from(variant: Failverify) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAILVERIFY` reader - 5:5\\]
Command failed due to verify error"]
pub type FailverifyR = crate::BitReader<Failverify>;
impl FailverifyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Failverify {
        match self.bits {
            true => Failverify::Statfail,
            false => Failverify::Statnofail,
        }
    }
    #[doc = "Fail"]
    #[inline(always)]
    pub fn is_statfail(&self) -> bool {
        *self == Failverify::Statfail
    }
    #[doc = "No Fail"]
    #[inline(always)]
    pub fn is_statnofail(&self) -> bool {
        *self == Failverify::Statnofail
    }
}
#[doc = "6:6\\]
Command failed due to the use of an illegal address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Faililladdr {
    #[doc = "1: Fail"]
    Statfail = 1,
    #[doc = "0: No Fail"]
    Statnofail = 0,
}
impl From<Faililladdr> for bool {
    #[inline(always)]
    fn from(variant: Faililladdr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAILILLADDR` reader - 6:6\\]
Command failed due to the use of an illegal address"]
pub type FaililladdrR = crate::BitReader<Faililladdr>;
impl FaililladdrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Faililladdr {
        match self.bits {
            true => Faililladdr::Statfail,
            false => Faililladdr::Statnofail,
        }
    }
    #[doc = "Fail"]
    #[inline(always)]
    pub fn is_statfail(&self) -> bool {
        *self == Faililladdr::Statfail
    }
    #[doc = "No Fail"]
    #[inline(always)]
    pub fn is_statnofail(&self) -> bool {
        *self == Faililladdr::Statnofail
    }
}
#[doc = "7:7\\]
Command failed because a bank has been set to a mode other than READ. Program and Erase commands cannot be initiated unless all banks are in READ mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Failmode {
    #[doc = "1: Fail"]
    Statfail = 1,
    #[doc = "0: No Fail"]
    Statnofail = 0,
}
impl From<Failmode> for bool {
    #[inline(always)]
    fn from(variant: Failmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAILMODE` reader - 7:7\\]
Command failed because a bank has been set to a mode other than READ. Program and Erase commands cannot be initiated unless all banks are in READ mode."]
pub type FailmodeR = crate::BitReader<Failmode>;
impl FailmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Failmode {
        match self.bits {
            true => Failmode::Statfail,
            false => Failmode::Statnofail,
        }
    }
    #[doc = "Fail"]
    #[inline(always)]
    pub fn is_statfail(&self) -> bool {
        *self == Failmode::Statfail
    }
    #[doc = "No Fail"]
    #[inline(always)]
    pub fn is_statnofail(&self) -> bool {
        *self == Failmode::Statnofail
    }
}
#[doc = "8:8\\]
Program command failed because an attempt was made to program a stored 0 value to a 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Failinvdata {
    #[doc = "1: Fail"]
    Statfail = 1,
    #[doc = "0: No Fail"]
    Statnofail = 0,
}
impl From<Failinvdata> for bool {
    #[inline(always)]
    fn from(variant: Failinvdata) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAILINVDATA` reader - 8:8\\]
Program command failed because an attempt was made to program a stored 0 value to a 1."]
pub type FailinvdataR = crate::BitReader<Failinvdata>;
impl FailinvdataR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Failinvdata {
        match self.bits {
            true => Failinvdata::Statfail,
            false => Failinvdata::Statnofail,
        }
    }
    #[doc = "Fail"]
    #[inline(always)]
    pub fn is_statfail(&self) -> bool {
        *self == Failinvdata::Statfail
    }
    #[doc = "No Fail"]
    #[inline(always)]
    pub fn is_statnofail(&self) -> bool {
        *self == Failinvdata::Statnofail
    }
}
#[doc = "12:12\\]
Command failed due to error other than write/erase protect violation or verify error. This is an extra bit in case a new failure mechanism is added which requires a status bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Failmisc {
    #[doc = "1: Fail"]
    Statfail = 1,
    #[doc = "0: No Fail"]
    Statnofail = 0,
}
impl From<Failmisc> for bool {
    #[inline(always)]
    fn from(variant: Failmisc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAILMISC` reader - 12:12\\]
Command failed due to error other than write/erase protect violation or verify error. This is an extra bit in case a new failure mechanism is added which requires a status bit."]
pub type FailmiscR = crate::BitReader<Failmisc>;
impl FailmiscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Failmisc {
        match self.bits {
            true => Failmisc::Statfail,
            false => Failmisc::Statnofail,
        }
    }
    #[doc = "Fail"]
    #[inline(always)]
    pub fn is_statfail(&self) -> bool {
        *self == Failmisc::Statfail
    }
    #[doc = "No Fail"]
    #[inline(always)]
    pub fn is_statnofail(&self) -> bool {
        *self == Failmisc::Statnofail
    }
}
#[doc = "Field `RESERVED13` reader - 31:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved13R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Command Done"]
    #[inline(always)]
    pub fn cmddone(&self) -> CmddoneR {
        CmddoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Command Pass - valid when CMD_DONE field is 1"]
    #[inline(always)]
    pub fn cmdpass(&self) -> CmdpassR {
        CmdpassR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Command In Progress"]
    #[inline(always)]
    pub fn cmdinprogress(&self) -> CmdinprogressR {
        CmdinprogressR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Command failed due to Write/Erase Protect Sector Violation"]
    #[inline(always)]
    pub fn failweprot(&self) -> FailweprotR {
        FailweprotR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Command failed due to verify error"]
    #[inline(always)]
    pub fn failverify(&self) -> FailverifyR {
        FailverifyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Command failed due to the use of an illegal address"]
    #[inline(always)]
    pub fn faililladdr(&self) -> FaililladdrR {
        FaililladdrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Command failed because a bank has been set to a mode other than READ. Program and Erase commands cannot be initiated unless all banks are in READ mode."]
    #[inline(always)]
    pub fn failmode(&self) -> FailmodeR {
        FailmodeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Program command failed because an attempt was made to program a stored 0 value to a 1."]
    #[inline(always)]
    pub fn failinvdata(&self) -> FailinvdataR {
        FailinvdataR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Command failed due to error other than write/erase protect violation or verify error. This is an extra bit in case a new failure mechanism is added which requires a status bit."]
    #[inline(always)]
    pub fn failmisc(&self) -> FailmiscR {
        FailmiscR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:31 - 31:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved13(&self) -> Reserved13R {
        Reserved13R::new((self.bits >> 13) & 0x0007_ffff)
    }
}
impl W {}
#[doc = "Command Status Register This register contains status regarding completion and errors of command execution.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statcmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`statcmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatcmdSpec;
impl crate::RegisterSpec for StatcmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statcmd::R`](R) reader structure"]
impl crate::Readable for StatcmdSpec {}
#[doc = "`write(|w| ..)` method takes [`statcmd::W`](W) writer structure"]
impl crate::Writable for StatcmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATCMD to value 0"]
impl crate::Resettable for StatcmdSpec {
    const RESET_VALUE: u32 = 0;
}
