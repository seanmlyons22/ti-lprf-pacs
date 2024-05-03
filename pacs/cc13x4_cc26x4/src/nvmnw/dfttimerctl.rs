#[doc = "Register `DFTTIMERCTL` reader"]
pub type R = crate::R<DfttimerctlSpec>;
#[doc = "Register `DFTTIMERCTL` writer"]
pub type W = crate::W<DfttimerctlSpec>;
#[doc = "0:0\\]
Program/Erase Setup Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pesetuptime {
    #[doc = "1: Use 2x the hard-wired (functional) time value"]
    Twoxfunctional = 1,
    #[doc = "0: Use hard-wired (Functional) timer value"]
    Functional = 0,
}
impl From<Pesetuptime> for bool {
    #[inline(always)]
    fn from(variant: Pesetuptime) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PESETUPTIME` reader - 0:0\\]
Program/Erase Setup Time"]
pub type PesetuptimeR = crate::BitReader<Pesetuptime>;
impl PesetuptimeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pesetuptime {
        match self.bits {
            true => Pesetuptime::Twoxfunctional,
            false => Pesetuptime::Functional,
        }
    }
    #[doc = "Use 2x the hard-wired (functional) time value"]
    #[inline(always)]
    pub fn is_twoxfunctional(&self) -> bool {
        *self == Pesetuptime::Twoxfunctional
    }
    #[doc = "Use hard-wired (Functional) timer value"]
    #[inline(always)]
    pub fn is_functional(&self) -> bool {
        *self == Pesetuptime::Functional
    }
}
#[doc = "Field `PESETUPTIME` writer - 0:0\\]
Program/Erase Setup Time"]
pub type PesetuptimeW<'a, REG> = crate::BitWriter<'a, REG, Pesetuptime>;
impl<'a, REG> PesetuptimeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use 2x the hard-wired (functional) time value"]
    #[inline(always)]
    pub fn twoxfunctional(self) -> &'a mut crate::W<REG> {
        self.variant(Pesetuptime::Twoxfunctional)
    }
    #[doc = "Use hard-wired (Functional) timer value"]
    #[inline(always)]
    pub fn functional(self) -> &'a mut crate::W<REG> {
        self.variant(Pesetuptime::Functional)
    }
}
#[doc = "1:1\\]
Program VHV Setup Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pvhvsetuptime {
    #[doc = "1: Use 2x the hard-wired (functional) time value"]
    Twoxfunctional = 1,
    #[doc = "0: Use hard-wired (Functional) timer value"]
    Functional = 0,
}
impl From<Pvhvsetuptime> for bool {
    #[inline(always)]
    fn from(variant: Pvhvsetuptime) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVHVSETUPTIME` reader - 1:1\\]
Program VHV Setup Time"]
pub type PvhvsetuptimeR = crate::BitReader<Pvhvsetuptime>;
impl PvhvsetuptimeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pvhvsetuptime {
        match self.bits {
            true => Pvhvsetuptime::Twoxfunctional,
            false => Pvhvsetuptime::Functional,
        }
    }
    #[doc = "Use 2x the hard-wired (functional) time value"]
    #[inline(always)]
    pub fn is_twoxfunctional(&self) -> bool {
        *self == Pvhvsetuptime::Twoxfunctional
    }
    #[doc = "Use hard-wired (Functional) timer value"]
    #[inline(always)]
    pub fn is_functional(&self) -> bool {
        *self == Pvhvsetuptime::Functional
    }
}
#[doc = "Field `PVHVSETUPTIME` writer - 1:1\\]
Program VHV Setup Time"]
pub type PvhvsetuptimeW<'a, REG> = crate::BitWriter<'a, REG, Pvhvsetuptime>;
impl<'a, REG> PvhvsetuptimeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use 2x the hard-wired (functional) time value"]
    #[inline(always)]
    pub fn twoxfunctional(self) -> &'a mut crate::W<REG> {
        self.variant(Pvhvsetuptime::Twoxfunctional)
    }
    #[doc = "Use hard-wired (Functional) timer value"]
    #[inline(always)]
    pub fn functional(self) -> &'a mut crate::W<REG> {
        self.variant(Pvhvsetuptime::Functional)
    }
}
#[doc = "2:2\\]
Program and Program Verify Wordline Switching Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ppvwordlinetime {
    #[doc = "1: Use 2x the hard-wired (functional) time value"]
    Twoxfunctional = 1,
    #[doc = "0: Use hard-wired (Functional) timer value"]
    Functional = 0,
}
impl From<Ppvwordlinetime> for bool {
    #[inline(always)]
    fn from(variant: Ppvwordlinetime) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPVWORDLINETIME` reader - 2:2\\]
Program and Program Verify Wordline Switching Time"]
pub type PpvwordlinetimeR = crate::BitReader<Ppvwordlinetime>;
impl PpvwordlinetimeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ppvwordlinetime {
        match self.bits {
            true => Ppvwordlinetime::Twoxfunctional,
            false => Ppvwordlinetime::Functional,
        }
    }
    #[doc = "Use 2x the hard-wired (functional) time value"]
    #[inline(always)]
    pub fn is_twoxfunctional(&self) -> bool {
        *self == Ppvwordlinetime::Twoxfunctional
    }
    #[doc = "Use hard-wired (Functional) timer value"]
    #[inline(always)]
    pub fn is_functional(&self) -> bool {
        *self == Ppvwordlinetime::Functional
    }
}
#[doc = "Field `PPVWORDLINETIME` writer - 2:2\\]
Program and Program Verify Wordline Switching Time"]
pub type PpvwordlinetimeW<'a, REG> = crate::BitWriter<'a, REG, Ppvwordlinetime>;
impl<'a, REG> PpvwordlinetimeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use 2x the hard-wired (functional) time value"]
    #[inline(always)]
    pub fn twoxfunctional(self) -> &'a mut crate::W<REG> {
        self.variant(Ppvwordlinetime::Twoxfunctional)
    }
    #[doc = "Use hard-wired (Functional) timer value"]
    #[inline(always)]
    pub fn functional(self) -> &'a mut crate::W<REG> {
        self.variant(Ppvwordlinetime::Functional)
    }
}
#[doc = "3:3\\]
Program/Erase Hold Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Peholdtime {
    #[doc = "1: Use 2x the hard-wired (functional) time value"]
    Twoxfunctional = 1,
    #[doc = "0: Use hard-wired (Functional) timer value"]
    Functional = 0,
}
impl From<Peholdtime> for bool {
    #[inline(always)]
    fn from(variant: Peholdtime) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEHOLDTIME` reader - 3:3\\]
Program/Erase Hold Time"]
pub type PeholdtimeR = crate::BitReader<Peholdtime>;
impl PeholdtimeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Peholdtime {
        match self.bits {
            true => Peholdtime::Twoxfunctional,
            false => Peholdtime::Functional,
        }
    }
    #[doc = "Use 2x the hard-wired (functional) time value"]
    #[inline(always)]
    pub fn is_twoxfunctional(&self) -> bool {
        *self == Peholdtime::Twoxfunctional
    }
    #[doc = "Use hard-wired (Functional) timer value"]
    #[inline(always)]
    pub fn is_functional(&self) -> bool {
        *self == Peholdtime::Functional
    }
}
#[doc = "Field `PEHOLDTIME` writer - 3:3\\]
Program/Erase Hold Time"]
pub type PeholdtimeW<'a, REG> = crate::BitWriter<'a, REG, Peholdtime>;
impl<'a, REG> PeholdtimeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use 2x the hard-wired (functional) time value"]
    #[inline(always)]
    pub fn twoxfunctional(self) -> &'a mut crate::W<REG> {
        self.variant(Peholdtime::Twoxfunctional)
    }
    #[doc = "Use hard-wired (Functional) timer value"]
    #[inline(always)]
    pub fn functional(self) -> &'a mut crate::W<REG> {
        self.variant(Peholdtime::Functional)
    }
}
#[doc = "4:4\\]
Program/Erase Verify Mode Change Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pevmodetime {
    #[doc = "1: Use 2x the hard-wired (functional) time value"]
    Twoxfunctional = 1,
    #[doc = "0: Use hard-wired (Functional) timer value"]
    Functional = 0,
}
impl From<Pevmodetime> for bool {
    #[inline(always)]
    fn from(variant: Pevmodetime) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEVMODETIME` reader - 4:4\\]
Program/Erase Verify Mode Change Time"]
pub type PevmodetimeR = crate::BitReader<Pevmodetime>;
impl PevmodetimeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pevmodetime {
        match self.bits {
            true => Pevmodetime::Twoxfunctional,
            false => Pevmodetime::Functional,
        }
    }
    #[doc = "Use 2x the hard-wired (functional) time value"]
    #[inline(always)]
    pub fn is_twoxfunctional(&self) -> bool {
        *self == Pevmodetime::Twoxfunctional
    }
    #[doc = "Use hard-wired (Functional) timer value"]
    #[inline(always)]
    pub fn is_functional(&self) -> bool {
        *self == Pevmodetime::Functional
    }
}
#[doc = "Field `PEVMODETIME` writer - 4:4\\]
Program/Erase Verify Mode Change Time"]
pub type PevmodetimeW<'a, REG> = crate::BitWriter<'a, REG, Pevmodetime>;
impl<'a, REG> PevmodetimeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use 2x the hard-wired (functional) time value"]
    #[inline(always)]
    pub fn twoxfunctional(self) -> &'a mut crate::W<REG> {
        self.variant(Pevmodetime::Twoxfunctional)
    }
    #[doc = "Use hard-wired (Functional) timer value"]
    #[inline(always)]
    pub fn functional(self) -> &'a mut crate::W<REG> {
        self.variant(Pevmodetime::Functional)
    }
}
#[doc = "5:5\\]
Program/Erase Verify Setup Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pevsetuptime {
    #[doc = "1: Use 2x the hard-wired (functional) time value"]
    Twoxfunctional = 1,
    #[doc = "0: Use hard-wired (Functional) timer value"]
    Functional = 0,
}
impl From<Pevsetuptime> for bool {
    #[inline(always)]
    fn from(variant: Pevsetuptime) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEVSETUPTIME` reader - 5:5\\]
Program/Erase Verify Setup Time"]
pub type PevsetuptimeR = crate::BitReader<Pevsetuptime>;
impl PevsetuptimeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pevsetuptime {
        match self.bits {
            true => Pevsetuptime::Twoxfunctional,
            false => Pevsetuptime::Functional,
        }
    }
    #[doc = "Use 2x the hard-wired (functional) time value"]
    #[inline(always)]
    pub fn is_twoxfunctional(&self) -> bool {
        *self == Pevsetuptime::Twoxfunctional
    }
    #[doc = "Use hard-wired (Functional) timer value"]
    #[inline(always)]
    pub fn is_functional(&self) -> bool {
        *self == Pevsetuptime::Functional
    }
}
#[doc = "Field `PEVSETUPTIME` writer - 5:5\\]
Program/Erase Verify Setup Time"]
pub type PevsetuptimeW<'a, REG> = crate::BitWriter<'a, REG, Pevsetuptime>;
impl<'a, REG> PevsetuptimeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use 2x the hard-wired (functional) time value"]
    #[inline(always)]
    pub fn twoxfunctional(self) -> &'a mut crate::W<REG> {
        self.variant(Pevsetuptime::Twoxfunctional)
    }
    #[doc = "Use hard-wired (Functional) timer value"]
    #[inline(always)]
    pub fn functional(self) -> &'a mut crate::W<REG> {
        self.variant(Pevsetuptime::Functional)
    }
}
#[doc = "6:6\\]
Program/Erase Verify Hold Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pevholdtime {
    #[doc = "1: Use 2x the hard-wired (functional) time value"]
    Twoxfunctional = 1,
    #[doc = "0: Use hard-wired (Functional) timer value"]
    Functional = 0,
}
impl From<Pevholdtime> for bool {
    #[inline(always)]
    fn from(variant: Pevholdtime) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEVHOLDTIME` reader - 6:6\\]
Program/Erase Verify Hold Time"]
pub type PevholdtimeR = crate::BitReader<Pevholdtime>;
impl PevholdtimeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pevholdtime {
        match self.bits {
            true => Pevholdtime::Twoxfunctional,
            false => Pevholdtime::Functional,
        }
    }
    #[doc = "Use 2x the hard-wired (functional) time value"]
    #[inline(always)]
    pub fn is_twoxfunctional(&self) -> bool {
        *self == Pevholdtime::Twoxfunctional
    }
    #[doc = "Use hard-wired (Functional) timer value"]
    #[inline(always)]
    pub fn is_functional(&self) -> bool {
        *self == Pevholdtime::Functional
    }
}
#[doc = "Field `PEVHOLDTIME` writer - 6:6\\]
Program/Erase Verify Hold Time"]
pub type PevholdtimeW<'a, REG> = crate::BitWriter<'a, REG, Pevholdtime>;
impl<'a, REG> PevholdtimeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use 2x the hard-wired (functional) time value"]
    #[inline(always)]
    pub fn twoxfunctional(self) -> &'a mut crate::W<REG> {
        self.variant(Pevholdtime::Twoxfunctional)
    }
    #[doc = "Use hard-wired (Functional) timer value"]
    #[inline(always)]
    pub fn functional(self) -> &'a mut crate::W<REG> {
        self.variant(Pevholdtime::Functional)
    }
}
#[doc = "7:7\\]
Read Mode Change Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Readmodetime {
    #[doc = "1: Use 2x the hard-wired (functional) time value"]
    Twoxfunctional = 1,
    #[doc = "0: Use hard-wired (Functional) timer value"]
    Functional = 0,
}
impl From<Readmodetime> for bool {
    #[inline(always)]
    fn from(variant: Readmodetime) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READMODETIME` reader - 7:7\\]
Read Mode Change Time"]
pub type ReadmodetimeR = crate::BitReader<Readmodetime>;
impl ReadmodetimeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Readmodetime {
        match self.bits {
            true => Readmodetime::Twoxfunctional,
            false => Readmodetime::Functional,
        }
    }
    #[doc = "Use 2x the hard-wired (functional) time value"]
    #[inline(always)]
    pub fn is_twoxfunctional(&self) -> bool {
        *self == Readmodetime::Twoxfunctional
    }
    #[doc = "Use hard-wired (Functional) timer value"]
    #[inline(always)]
    pub fn is_functional(&self) -> bool {
        *self == Readmodetime::Functional
    }
}
#[doc = "Field `READMODETIME` writer - 7:7\\]
Read Mode Change Time"]
pub type ReadmodetimeW<'a, REG> = crate::BitWriter<'a, REG, Readmodetime>;
impl<'a, REG> ReadmodetimeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use 2x the hard-wired (functional) time value"]
    #[inline(always)]
    pub fn twoxfunctional(self) -> &'a mut crate::W<REG> {
        self.variant(Readmodetime::Twoxfunctional)
    }
    #[doc = "Use hard-wired (Functional) timer value"]
    #[inline(always)]
    pub fn functional(self) -> &'a mut crate::W<REG> {
        self.variant(Readmodetime::Functional)
    }
}
#[doc = "8:8\\]
Override Program/Erase Pulse Time If set, this will force the program or erase pulse time to be overridden with the value in the PEPULSETIMEVAL field. If not set, then a hard-coded value will be used for this pulse time.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pepulsetimeovr {
    #[doc = "1: Use value from the PE_PULSE_TIME field for time value"]
    Override = 1,
    #[doc = "0: Use hard-wired (Functional) timer value"]
    Functional = 0,
}
impl From<Pepulsetimeovr> for bool {
    #[inline(always)]
    fn from(variant: Pepulsetimeovr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEPULSETIMEOVR` reader - 8:8\\]
Override Program/Erase Pulse Time If set, this will force the program or erase pulse time to be overridden with the value in the PEPULSETIMEVAL field. If not set, then a hard-coded value will be used for this pulse time."]
pub type PepulsetimeovrR = crate::BitReader<Pepulsetimeovr>;
impl PepulsetimeovrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pepulsetimeovr {
        match self.bits {
            true => Pepulsetimeovr::Override,
            false => Pepulsetimeovr::Functional,
        }
    }
    #[doc = "Use value from the PE_PULSE_TIME field for time value"]
    #[inline(always)]
    pub fn is_override(&self) -> bool {
        *self == Pepulsetimeovr::Override
    }
    #[doc = "Use hard-wired (Functional) timer value"]
    #[inline(always)]
    pub fn is_functional(&self) -> bool {
        *self == Pepulsetimeovr::Functional
    }
}
#[doc = "Field `PEPULSETIMEOVR` writer - 8:8\\]
Override Program/Erase Pulse Time If set, this will force the program or erase pulse time to be overridden with the value in the PEPULSETIMEVAL field. If not set, then a hard-coded value will be used for this pulse time."]
pub type PepulsetimeovrW<'a, REG> = crate::BitWriter<'a, REG, Pepulsetimeovr>;
impl<'a, REG> PepulsetimeovrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use value from the PE_PULSE_TIME field for time value"]
    #[inline(always)]
    pub fn override_(self) -> &'a mut crate::W<REG> {
        self.variant(Pepulsetimeovr::Override)
    }
    #[doc = "Use hard-wired (Functional) timer value"]
    #[inline(always)]
    pub fn functional(self) -> &'a mut crate::W<REG> {
        self.variant(Pepulsetimeovr::Functional)
    }
}
#[doc = "Field `RESERVED9` reader - 11:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader;
#[doc = "Field `RESERVED9` writer - 11:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "27:12\\]
Program/Erase Pulse Time Value If operation is a program, this value gets loaded into bits \\[15:0\\]
of the timer when the PEPULSETIMEVALOVR field is set to 1. If operation is an erase, this value gets loaded into bits \\[19:4\\]
of the timer when the PEPULSETIMEVALOVR field is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Pepulsetimeval {
    #[doc = "65535: Maximum value"]
    Maximum = 65535,
    #[doc = "1: Minimum value"]
    Minimum = 1,
}
impl From<Pepulsetimeval> for u16 {
    #[inline(always)]
    fn from(variant: Pepulsetimeval) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pepulsetimeval {
    type Ux = u16;
}
impl crate::IsEnum for Pepulsetimeval {}
#[doc = "Field `PEPULSETIMEVAL` reader - 27:12\\]
Program/Erase Pulse Time Value If operation is a program, this value gets loaded into bits \\[15:0\\]
of the timer when the PEPULSETIMEVALOVR field is set to 1. If operation is an erase, this value gets loaded into bits \\[19:4\\]
of the timer when the PEPULSETIMEVALOVR field is set to 1."]
pub type PepulsetimevalR = crate::FieldReader<Pepulsetimeval>;
impl PepulsetimevalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pepulsetimeval> {
        match self.bits {
            65535 => Some(Pepulsetimeval::Maximum),
            1 => Some(Pepulsetimeval::Minimum),
            _ => None,
        }
    }
    #[doc = "Maximum value"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == Pepulsetimeval::Maximum
    }
    #[doc = "Minimum value"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == Pepulsetimeval::Minimum
    }
}
#[doc = "Field `PEPULSETIMEVAL` writer - 27:12\\]
Program/Erase Pulse Time Value If operation is a program, this value gets loaded into bits \\[15:0\\]
of the timer when the PEPULSETIMEVALOVR field is set to 1. If operation is an erase, this value gets loaded into bits \\[19:4\\]
of the timer when the PEPULSETIMEVALOVR field is set to 1."]
pub type PepulsetimevalW<'a, REG> = crate::FieldWriter<'a, REG, 16, Pepulsetimeval>;
impl<'a, REG> PepulsetimevalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Maximum value"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut crate::W<REG> {
        self.variant(Pepulsetimeval::Maximum)
    }
    #[doc = "Minimum value"]
    #[inline(always)]
    pub fn minimum(self) -> &'a mut crate::W<REG> {
        self.variant(Pepulsetimeval::Minimum)
    }
}
#[doc = "30:28\\]
Override Timer clock frequency using an ICG-based clock divide mechanism. To divide the timer clock, pulses can be skipped based on settings in this field. By default, this field is 0, which corresponds to no division on the timer clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Timerclockovr {
    #[doc = "7: Divide timer clock by 8"]
    Divideby8 = 7,
    #[doc = "6: Divide timer clock by 7"]
    Divideby7 = 6,
    #[doc = "5: Divide timer clock by 6"]
    Divideby6 = 5,
    #[doc = "4: Divide timer clock by 5"]
    Divideby5 = 4,
    #[doc = "3: Divide timer clock by 4"]
    Divideby4 = 3,
    #[doc = "2: Divide timer clock by 3"]
    Divideby3 = 2,
    #[doc = "1: Divide timer clock by 2"]
    Divideby2 = 1,
    #[doc = "0: No divide on timer clock."]
    Nodivide = 0,
}
impl From<Timerclockovr> for u8 {
    #[inline(always)]
    fn from(variant: Timerclockovr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Timerclockovr {
    type Ux = u8;
}
impl crate::IsEnum for Timerclockovr {}
#[doc = "Field `TIMERCLOCKOVR` reader - 30:28\\]
Override Timer clock frequency using an ICG-based clock divide mechanism. To divide the timer clock, pulses can be skipped based on settings in this field. By default, this field is 0, which corresponds to no division on the timer clock."]
pub type TimerclockovrR = crate::FieldReader<Timerclockovr>;
impl TimerclockovrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timerclockovr {
        match self.bits {
            7 => Timerclockovr::Divideby8,
            6 => Timerclockovr::Divideby7,
            5 => Timerclockovr::Divideby6,
            4 => Timerclockovr::Divideby5,
            3 => Timerclockovr::Divideby4,
            2 => Timerclockovr::Divideby3,
            1 => Timerclockovr::Divideby2,
            0 => Timerclockovr::Nodivide,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide timer clock by 8"]
    #[inline(always)]
    pub fn is_divideby8(&self) -> bool {
        *self == Timerclockovr::Divideby8
    }
    #[doc = "Divide timer clock by 7"]
    #[inline(always)]
    pub fn is_divideby7(&self) -> bool {
        *self == Timerclockovr::Divideby7
    }
    #[doc = "Divide timer clock by 6"]
    #[inline(always)]
    pub fn is_divideby6(&self) -> bool {
        *self == Timerclockovr::Divideby6
    }
    #[doc = "Divide timer clock by 5"]
    #[inline(always)]
    pub fn is_divideby5(&self) -> bool {
        *self == Timerclockovr::Divideby5
    }
    #[doc = "Divide timer clock by 4"]
    #[inline(always)]
    pub fn is_divideby4(&self) -> bool {
        *self == Timerclockovr::Divideby4
    }
    #[doc = "Divide timer clock by 3"]
    #[inline(always)]
    pub fn is_divideby3(&self) -> bool {
        *self == Timerclockovr::Divideby3
    }
    #[doc = "Divide timer clock by 2"]
    #[inline(always)]
    pub fn is_divideby2(&self) -> bool {
        *self == Timerclockovr::Divideby2
    }
    #[doc = "No divide on timer clock."]
    #[inline(always)]
    pub fn is_nodivide(&self) -> bool {
        *self == Timerclockovr::Nodivide
    }
}
#[doc = "Field `TIMERCLOCKOVR` writer - 30:28\\]
Override Timer clock frequency using an ICG-based clock divide mechanism. To divide the timer clock, pulses can be skipped based on settings in this field. By default, this field is 0, which corresponds to no division on the timer clock."]
pub type TimerclockovrW<'a, REG> = crate::FieldWriter<'a, REG, 3, Timerclockovr, crate::Safe>;
impl<'a, REG> TimerclockovrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide timer clock by 8"]
    #[inline(always)]
    pub fn divideby8(self) -> &'a mut crate::W<REG> {
        self.variant(Timerclockovr::Divideby8)
    }
    #[doc = "Divide timer clock by 7"]
    #[inline(always)]
    pub fn divideby7(self) -> &'a mut crate::W<REG> {
        self.variant(Timerclockovr::Divideby7)
    }
    #[doc = "Divide timer clock by 6"]
    #[inline(always)]
    pub fn divideby6(self) -> &'a mut crate::W<REG> {
        self.variant(Timerclockovr::Divideby6)
    }
    #[doc = "Divide timer clock by 5"]
    #[inline(always)]
    pub fn divideby5(self) -> &'a mut crate::W<REG> {
        self.variant(Timerclockovr::Divideby5)
    }
    #[doc = "Divide timer clock by 4"]
    #[inline(always)]
    pub fn divideby4(self) -> &'a mut crate::W<REG> {
        self.variant(Timerclockovr::Divideby4)
    }
    #[doc = "Divide timer clock by 3"]
    #[inline(always)]
    pub fn divideby3(self) -> &'a mut crate::W<REG> {
        self.variant(Timerclockovr::Divideby3)
    }
    #[doc = "Divide timer clock by 2"]
    #[inline(always)]
    pub fn divideby2(self) -> &'a mut crate::W<REG> {
        self.variant(Timerclockovr::Divideby2)
    }
    #[doc = "No divide on timer clock."]
    #[inline(always)]
    pub fn nodivide(self) -> &'a mut crate::W<REG> {
        self.variant(Timerclockovr::Nodivide)
    }
}
#[doc = "Field `RESERVED31` reader - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved31R = crate::BitReader;
#[doc = "Field `RESERVED31` writer - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved31W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Program/Erase Setup Time"]
    #[inline(always)]
    pub fn pesetuptime(&self) -> PesetuptimeR {
        PesetuptimeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Program VHV Setup Time"]
    #[inline(always)]
    pub fn pvhvsetuptime(&self) -> PvhvsetuptimeR {
        PvhvsetuptimeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Program and Program Verify Wordline Switching Time"]
    #[inline(always)]
    pub fn ppvwordlinetime(&self) -> PpvwordlinetimeR {
        PpvwordlinetimeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Program/Erase Hold Time"]
    #[inline(always)]
    pub fn peholdtime(&self) -> PeholdtimeR {
        PeholdtimeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Program/Erase Verify Mode Change Time"]
    #[inline(always)]
    pub fn pevmodetime(&self) -> PevmodetimeR {
        PevmodetimeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Program/Erase Verify Setup Time"]
    #[inline(always)]
    pub fn pevsetuptime(&self) -> PevsetuptimeR {
        PevsetuptimeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Program/Erase Verify Hold Time"]
    #[inline(always)]
    pub fn pevholdtime(&self) -> PevholdtimeR {
        PevholdtimeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Read Mode Change Time"]
    #[inline(always)]
    pub fn readmodetime(&self) -> ReadmodetimeR {
        ReadmodetimeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Override Program/Erase Pulse Time If set, this will force the program or erase pulse time to be overridden with the value in the PEPULSETIMEVAL field. If not set, then a hard-coded value will be used for this pulse time."]
    #[inline(always)]
    pub fn pepulsetimeovr(&self) -> PepulsetimeovrR {
        PepulsetimeovrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - 11:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:27 - 27:12\\]
Program/Erase Pulse Time Value If operation is a program, this value gets loaded into bits \\[15:0\\]
of the timer when the PEPULSETIMEVALOVR field is set to 1. If operation is an erase, this value gets loaded into bits \\[19:4\\]
of the timer when the PEPULSETIMEVALOVR field is set to 1."]
    #[inline(always)]
    pub fn pepulsetimeval(&self) -> PepulsetimevalR {
        PepulsetimevalR::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bits 28:30 - 30:28\\]
Override Timer clock frequency using an ICG-based clock divide mechanism. To divide the timer clock, pulses can be skipped based on settings in this field. By default, this field is 0, which corresponds to no division on the timer clock."]
    #[inline(always)]
    pub fn timerclockovr(&self) -> TimerclockovrR {
        TimerclockovrR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved31(&self) -> Reserved31R {
        Reserved31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Program/Erase Setup Time"]
    #[inline(always)]
    #[must_use]
    pub fn pesetuptime(&mut self) -> PesetuptimeW<DfttimerctlSpec> {
        PesetuptimeW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Program VHV Setup Time"]
    #[inline(always)]
    #[must_use]
    pub fn pvhvsetuptime(&mut self) -> PvhvsetuptimeW<DfttimerctlSpec> {
        PvhvsetuptimeW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Program and Program Verify Wordline Switching Time"]
    #[inline(always)]
    #[must_use]
    pub fn ppvwordlinetime(&mut self) -> PpvwordlinetimeW<DfttimerctlSpec> {
        PpvwordlinetimeW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Program/Erase Hold Time"]
    #[inline(always)]
    #[must_use]
    pub fn peholdtime(&mut self) -> PeholdtimeW<DfttimerctlSpec> {
        PeholdtimeW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Program/Erase Verify Mode Change Time"]
    #[inline(always)]
    #[must_use]
    pub fn pevmodetime(&mut self) -> PevmodetimeW<DfttimerctlSpec> {
        PevmodetimeW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Program/Erase Verify Setup Time"]
    #[inline(always)]
    #[must_use]
    pub fn pevsetuptime(&mut self) -> PevsetuptimeW<DfttimerctlSpec> {
        PevsetuptimeW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Program/Erase Verify Hold Time"]
    #[inline(always)]
    #[must_use]
    pub fn pevholdtime(&mut self) -> PevholdtimeW<DfttimerctlSpec> {
        PevholdtimeW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Read Mode Change Time"]
    #[inline(always)]
    #[must_use]
    pub fn readmodetime(&mut self) -> ReadmodetimeW<DfttimerctlSpec> {
        ReadmodetimeW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Override Program/Erase Pulse Time If set, this will force the program or erase pulse time to be overridden with the value in the PEPULSETIMEVAL field. If not set, then a hard-coded value will be used for this pulse time."]
    #[inline(always)]
    #[must_use]
    pub fn pepulsetimeovr(&mut self) -> PepulsetimeovrW<DfttimerctlSpec> {
        PepulsetimeovrW::new(self, 8)
    }
    #[doc = "Bits 9:11 - 11:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> Reserved9W<DfttimerctlSpec> {
        Reserved9W::new(self, 9)
    }
    #[doc = "Bits 12:27 - 27:12\\]
Program/Erase Pulse Time Value If operation is a program, this value gets loaded into bits \\[15:0\\]
of the timer when the PEPULSETIMEVALOVR field is set to 1. If operation is an erase, this value gets loaded into bits \\[19:4\\]
of the timer when the PEPULSETIMEVALOVR field is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn pepulsetimeval(&mut self) -> PepulsetimevalW<DfttimerctlSpec> {
        PepulsetimevalW::new(self, 12)
    }
    #[doc = "Bits 28:30 - 30:28\\]
Override Timer clock frequency using an ICG-based clock divide mechanism. To divide the timer clock, pulses can be skipped based on settings in this field. By default, this field is 0, which corresponds to no division on the timer clock."]
    #[inline(always)]
    #[must_use]
    pub fn timerclockovr(&mut self) -> TimerclockovrW<DfttimerctlSpec> {
        TimerclockovrW::new(self, 28)
    }
    #[doc = "Bit 31 - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved31(&mut self) -> Reserved31W<DfttimerctlSpec> {
        Reserved31W::new(self, 31)
    }
}
#[doc = "DFT Timer Control Register This allows some configuration of timing values for various phases of flash operations for test. These time values are hard-coded for functional execution. This register is only writable when DFT.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfttimerctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfttimerctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfttimerctlSpec;
impl crate::RegisterSpec for DfttimerctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfttimerctl::R`](R) reader structure"]
impl crate::Readable for DfttimerctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dfttimerctl::W`](W) writer structure"]
impl crate::Writable for DfttimerctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFTTIMERCTL to value 0"]
impl crate::Resettable for DfttimerctlSpec {
    const RESET_VALUE: u32 = 0;
}
