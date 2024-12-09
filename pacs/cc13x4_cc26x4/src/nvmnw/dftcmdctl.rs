#[doc = "Register `DFTCMDCTL` reader"]
pub type R = crate::R<DftcmdctlSpec>;
#[doc = "Register `DFTCMDCTL` writer"]
pub type W = crate::W<DftcmdctlSpec>;
#[doc = "0:0\\]
Force 1T Enable - Force 1T access to regions that are designated as 2T. Regions designated as 1T will still be accessed as 1T.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Force1ten {
    #[doc = "1: Enable"]
    Enable = 1,
    #[doc = "0: Disable"]
    Disable = 0,
}
impl From<Force1ten> for bool {
    #[inline(always)]
    fn from(variant: Force1ten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCE1TEN` reader - 0:0\\]
Force 1T Enable - Force 1T access to regions that are designated as 2T. Regions designated as 1T will still be accessed as 1T."]
pub type Force1tenR = crate::BitReader<Force1ten>;
impl Force1tenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Force1ten {
        match self.bits {
            true => Force1ten::Enable,
            false => Force1ten::Disable,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Force1ten::Enable
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Force1ten::Disable
    }
}
#[doc = "Field `FORCE1TEN` writer - 0:0\\]
Force 1T Enable - Force 1T access to regions that are designated as 2T. Regions designated as 1T will still be accessed as 1T."]
pub type Force1tenW<'a, REG> = crate::BitWriter<'a, REG, Force1ten>;
impl<'a, REG> Force1tenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Force1ten::Enable)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Force1ten::Disable)
    }
}
#[doc = "1:1\\]
Force 2T Enable - Force 2T access to regions that are designated as 1T. Regions designated as 2T will still be accessed as 2T.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Force2ten {
    #[doc = "1: Enable"]
    Enable = 1,
    #[doc = "0: Disable"]
    Disable = 0,
}
impl From<Force2ten> for bool {
    #[inline(always)]
    fn from(variant: Force2ten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCE2TEN` reader - 1:1\\]
Force 2T Enable - Force 2T access to regions that are designated as 1T. Regions designated as 2T will still be accessed as 2T."]
pub type Force2tenR = crate::BitReader<Force2ten>;
impl Force2tenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Force2ten {
        match self.bits {
            true => Force2ten::Enable,
            false => Force2ten::Disable,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Force2ten::Enable
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Force2ten::Disable
    }
}
#[doc = "Field `FORCE2TEN` writer - 1:1\\]
Force 2T Enable - Force 2T access to regions that are designated as 1T. Regions designated as 2T will still be accessed as 2T."]
pub type Force2tenW<'a, REG> = crate::BitWriter<'a, REG, Force2ten>;
impl<'a, REG> Force2tenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Force2ten::Enable)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Force2ten::Disable)
    }
}
#[doc = "2:2\\]
2T address mux disable control. When set to 1 2T address shifting is disabled. This bit should only be enabled for reads. Indeterminate behavior will result if this bit is set during program/erase. Furthermore, only reads done via a READVERIFY command will be guaranteed to work properly. Reads via the FBAP port are not guaranteed to operate.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Amx2tdis {
    #[doc = "1: Disable"]
    Disable = 1,
    #[doc = "0: Enable"]
    Enable = 0,
}
impl From<Amx2tdis> for bool {
    #[inline(always)]
    fn from(variant: Amx2tdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMX2TDIS` reader - 2:2\\]
2T address mux disable control. When set to 1 2T address shifting is disabled. This bit should only be enabled for reads. Indeterminate behavior will result if this bit is set during program/erase. Furthermore, only reads done via a READVERIFY command will be guaranteed to work properly. Reads via the FBAP port are not guaranteed to operate."]
pub type Amx2tdisR = crate::BitReader<Amx2tdis>;
impl Amx2tdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Amx2tdis {
        match self.bits {
            true => Amx2tdis::Disable,
            false => Amx2tdis::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Amx2tdis::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Amx2tdis::Enable
    }
}
#[doc = "Field `AMX2TDIS` writer - 2:2\\]
2T address mux disable control. When set to 1 2T address shifting is disabled. This bit should only be enabled for reads. Indeterminate behavior will result if this bit is set during program/erase. Furthermore, only reads done via a READVERIFY command will be guaranteed to work properly. Reads via the FBAP port are not guaranteed to operate."]
pub type Amx2tdisW<'a, REG> = crate::BitWriter<'a, REG, Amx2tdis>;
impl<'a, REG> Amx2tdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Amx2tdis::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Amx2tdis::Enable)
    }
}
#[doc = "Field `RESERVED3` reader - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::BitReader;
#[doc = "4:4\\]
Disable redundancy matching. Any repair configuration encoded into the bank trim bits is disabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Redmatchdis {
    #[doc = "1: Disable"]
    Disable = 1,
    #[doc = "0: Enable"]
    Enable = 0,
}
impl From<Redmatchdis> for bool {
    #[inline(always)]
    fn from(variant: Redmatchdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REDMATCHDIS` reader - 4:4\\]
Disable redundancy matching. Any repair configuration encoded into the bank trim bits is disabled."]
pub type RedmatchdisR = crate::BitReader<Redmatchdis>;
impl RedmatchdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Redmatchdis {
        match self.bits {
            true => Redmatchdis::Disable,
            false => Redmatchdis::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Redmatchdis::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Redmatchdis::Enable
    }
}
#[doc = "Field `REDMATCHDIS` writer - 4:4\\]
Disable redundancy matching. Any repair configuration encoded into the bank trim bits is disabled."]
pub type RedmatchdisW<'a, REG> = crate::BitWriter<'a, REG, Redmatchdis>;
impl<'a, REG> RedmatchdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Redmatchdis::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Redmatchdis::Enable)
    }
}
#[doc = "5:5\\]
Force redundancy match. If set to 1, repair configuration encoded in the flash bank trim will be forced for every access.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Redmatchforce {
    #[doc = "1: Enable"]
    Enable = 1,
    #[doc = "0: Disable"]
    Disable = 0,
}
impl From<Redmatchforce> for bool {
    #[inline(always)]
    fn from(variant: Redmatchforce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REDMATCHFORCE` reader - 5:5\\]
Force redundancy match. If set to 1, repair configuration encoded in the flash bank trim will be forced for every access."]
pub type RedmatchforceR = crate::BitReader<Redmatchforce>;
impl RedmatchforceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Redmatchforce {
        match self.bits {
            true => Redmatchforce::Enable,
            false => Redmatchforce::Disable,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Redmatchforce::Enable
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Redmatchforce::Disable
    }
}
#[doc = "Field `REDMATCHFORCE` writer - 5:5\\]
Force redundancy match. If set to 1, repair configuration encoded in the flash bank trim will be forced for every access."]
pub type RedmatchforceW<'a, REG> = crate::BitWriter<'a, REG, Redmatchforce>;
impl<'a, REG> RedmatchforceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Redmatchforce::Enable)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Redmatchforce::Disable)
    }
}
#[doc = "Field `RESERVED6` reader - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader;
#[doc = "8:8\\]
Override address counter enable. When set, the state machine address counter will not be loaded when a command is initiated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Addrcntlddis {
    #[doc = "1: Disable"]
    Disable = 1,
    #[doc = "0: Enable"]
    Enable = 0,
}
impl From<Addrcntlddis> for bool {
    #[inline(always)]
    fn from(variant: Addrcntlddis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRCNTLDDIS` reader - 8:8\\]
Override address counter enable. When set, the state machine address counter will not be loaded when a command is initiated."]
pub type AddrcntlddisR = crate::BitReader<Addrcntlddis>;
impl AddrcntlddisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Addrcntlddis {
        match self.bits {
            true => Addrcntlddis::Disable,
            false => Addrcntlddis::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Addrcntlddis::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Addrcntlddis::Enable
    }
}
#[doc = "Field `ADDRCNTLDDIS` writer - 8:8\\]
Override address counter enable. When set, the state machine address counter will not be loaded when a command is initiated."]
pub type AddrcntlddisW<'a, REG> = crate::BitWriter<'a, REG, Addrcntlddis>;
impl<'a, REG> AddrcntlddisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Addrcntlddis::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Addrcntlddis::Enable)
    }
}
#[doc = "9:9\\]
Override pulse counter enable. When set, the state machine pulse counter will not be loaded when a command is initiated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pulsecntlddis {
    #[doc = "1: Disable"]
    Disable = 1,
    #[doc = "0: Enable"]
    Enable = 0,
}
impl From<Pulsecntlddis> for bool {
    #[inline(always)]
    fn from(variant: Pulsecntlddis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PULSECNTLDDIS` reader - 9:9\\]
Override pulse counter enable. When set, the state machine pulse counter will not be loaded when a command is initiated."]
pub type PulsecntlddisR = crate::BitReader<Pulsecntlddis>;
impl PulsecntlddisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pulsecntlddis {
        match self.bits {
            true => Pulsecntlddis::Disable,
            false => Pulsecntlddis::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pulsecntlddis::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pulsecntlddis::Enable
    }
}
#[doc = "Field `PULSECNTLDDIS` writer - 9:9\\]
Override pulse counter enable. When set, the state machine pulse counter will not be loaded when a command is initiated."]
pub type PulsecntlddisW<'a, REG> = crate::BitWriter<'a, REG, Pulsecntlddis>;
impl<'a, REG> PulsecntlddisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pulsecntlddis::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pulsecntlddis::Enable)
    }
}
#[doc = "Field `RESERVED10` reader - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved10R = crate::FieldReader;
#[doc = "12:12\\]
Enable data pattern. Data pattern select in DATAPATSEL field will override data from CMDDATA registers for use as program or verify data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datapaten {
    #[doc = "1: Enable"]
    Enable = 1,
    #[doc = "0: Disable"]
    Disable = 0,
}
impl From<Datapaten> for bool {
    #[inline(always)]
    fn from(variant: Datapaten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATAPATEN` reader - 12:12\\]
Enable data pattern. Data pattern select in DATAPATSEL field will override data from CMDDATA registers for use as program or verify data."]
pub type DatapatenR = crate::BitReader<Datapaten>;
impl DatapatenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Datapaten {
        match self.bits {
            true => Datapaten::Enable,
            false => Datapaten::Disable,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Datapaten::Enable
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Datapaten::Disable
    }
}
#[doc = "Field `DATAPATEN` writer - 12:12\\]
Enable data pattern. Data pattern select in DATAPATSEL field will override data from CMDDATA registers for use as program or verify data."]
pub type DatapatenW<'a, REG> = crate::BitWriter<'a, REG, Datapaten>;
impl<'a, REG> DatapatenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Datapaten::Enable)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Datapaten::Disable)
    }
}
#[doc = "15:13\\]
Select data pattern. Valid when DATAPATEN bit is set to 1. Overrides CMDDATA registers for program or verify.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Datapatsel {
    #[doc = "2: Set to logical checkerboard (0x01010101...)"]
    Logchkbrd = 2,
    #[doc = "1: Set to all 1"]
    All1 = 1,
    #[doc = "0: Set to all 0"]
    All0 = 0,
}
impl From<Datapatsel> for u8 {
    #[inline(always)]
    fn from(variant: Datapatsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Datapatsel {
    type Ux = u8;
}
impl crate::IsEnum for Datapatsel {}
#[doc = "Field `DATAPATSEL` reader - 15:13\\]
Select data pattern. Valid when DATAPATEN bit is set to 1. Overrides CMDDATA registers for program or verify."]
pub type DatapatselR = crate::FieldReader<Datapatsel>;
impl DatapatselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Datapatsel> {
        match self.bits {
            2 => Some(Datapatsel::Logchkbrd),
            1 => Some(Datapatsel::All1),
            0 => Some(Datapatsel::All0),
            _ => None,
        }
    }
    #[doc = "Set to logical checkerboard (0x01010101...)"]
    #[inline(always)]
    pub fn is_logchkbrd(&self) -> bool {
        *self == Datapatsel::Logchkbrd
    }
    #[doc = "Set to all 1"]
    #[inline(always)]
    pub fn is_all1(&self) -> bool {
        *self == Datapatsel::All1
    }
    #[doc = "Set to all 0"]
    #[inline(always)]
    pub fn is_all0(&self) -> bool {
        *self == Datapatsel::All0
    }
}
#[doc = "Field `DATAPATSEL` writer - 15:13\\]
Select data pattern. Valid when DATAPATEN bit is set to 1. Overrides CMDDATA registers for program or verify."]
pub type DatapatselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Datapatsel>;
impl<'a, REG> DatapatselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Set to logical checkerboard (0x01010101...)"]
    #[inline(always)]
    pub fn logchkbrd(self) -> &'a mut crate::W<REG> {
        self.variant(Datapatsel::Logchkbrd)
    }
    #[doc = "Set to all 1"]
    #[inline(always)]
    pub fn all1(self) -> &'a mut crate::W<REG> {
        self.variant(Datapatsel::All1)
    }
    #[doc = "Set to all 0"]
    #[inline(always)]
    pub fn all0(self) -> &'a mut crate::W<REG> {
        self.variant(Datapatsel::All0)
    }
}
#[doc = "16:16\\]
Invert data always for program or verify. This bit only applies when pattern data is used; i.e. the DATAPATEN bit is set. It will have no effect if CMDDATA is used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alwaysinvdata {
    #[doc = "1: Use inverted data"]
    Invert = 1,
    #[doc = "0: Use true data"]
    True = 0,
}
impl From<Alwaysinvdata> for bool {
    #[inline(always)]
    fn from(variant: Alwaysinvdata) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALWAYSINVDATA` reader - 16:16\\]
Invert data always for program or verify. This bit only applies when pattern data is used; i.e. the DATAPATEN bit is set. It will have no effect if CMDDATA is used."]
pub type AlwaysinvdataR = crate::BitReader<Alwaysinvdata>;
impl AlwaysinvdataR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Alwaysinvdata {
        match self.bits {
            true => Alwaysinvdata::Invert,
            false => Alwaysinvdata::True,
        }
    }
    #[doc = "Use inverted data"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == Alwaysinvdata::Invert
    }
    #[doc = "Use true data"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Alwaysinvdata::True
    }
}
#[doc = "Field `ALWAYSINVDATA` writer - 16:16\\]
Invert data always for program or verify. This bit only applies when pattern data is used; i.e. the DATAPATEN bit is set. It will have no effect if CMDDATA is used."]
pub type AlwaysinvdataW<'a, REG> = crate::BitWriter<'a, REG, Alwaysinvdata>;
impl<'a, REG> AlwaysinvdataW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use inverted data"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut crate::W<REG> {
        self.variant(Alwaysinvdata::Invert)
    }
    #[doc = "Use true data"]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Alwaysinvdata::True)
    }
}
#[doc = "17:17\\]
Invert data at odd bank addresses for program or verify. This bit only applies when pattern data is used; i.e. the DATAPATEN bit is set. It will have no effect if CMDDATA is used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oddwordinvdata {
    #[doc = "1: Use inverted data"]
    Invert = 1,
    #[doc = "0: Use true data"]
    True = 0,
}
impl From<Oddwordinvdata> for bool {
    #[inline(always)]
    fn from(variant: Oddwordinvdata) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ODDWORDINVDATA` reader - 17:17\\]
Invert data at odd bank addresses for program or verify. This bit only applies when pattern data is used; i.e. the DATAPATEN bit is set. It will have no effect if CMDDATA is used."]
pub type OddwordinvdataR = crate::BitReader<Oddwordinvdata>;
impl OddwordinvdataR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oddwordinvdata {
        match self.bits {
            true => Oddwordinvdata::Invert,
            false => Oddwordinvdata::True,
        }
    }
    #[doc = "Use inverted data"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == Oddwordinvdata::Invert
    }
    #[doc = "Use true data"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Oddwordinvdata::True
    }
}
#[doc = "Field `ODDWORDINVDATA` writer - 17:17\\]
Invert data at odd bank addresses for program or verify. This bit only applies when pattern data is used; i.e. the DATAPATEN bit is set. It will have no effect if CMDDATA is used."]
pub type OddwordinvdataW<'a, REG> = crate::BitWriter<'a, REG, Oddwordinvdata>;
impl<'a, REG> OddwordinvdataW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use inverted data"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut crate::W<REG> {
        self.variant(Oddwordinvdata::Invert)
    }
    #[doc = "Use true data"]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Oddwordinvdata::True)
    }
}
#[doc = "18:18\\]
Invert data at odd row addresses for program or verify. The LSB of the row address is bit \\[4\\]
of the bank address. This bit only applies when pattern data is used; i.e. the DATAPATEN bit is set. It will have no effect if CMDDATA is used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oddrowinvdata {
    #[doc = "1: Use inverted data"]
    Invert = 1,
    #[doc = "0: Use true data"]
    True = 0,
}
impl From<Oddrowinvdata> for bool {
    #[inline(always)]
    fn from(variant: Oddrowinvdata) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ODDROWINVDATA` reader - 18:18\\]
Invert data at odd row addresses for program or verify. The LSB of the row address is bit \\[4\\]
of the bank address. This bit only applies when pattern data is used; i.e. the DATAPATEN bit is set. It will have no effect if CMDDATA is used."]
pub type OddrowinvdataR = crate::BitReader<Oddrowinvdata>;
impl OddrowinvdataR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oddrowinvdata {
        match self.bits {
            true => Oddrowinvdata::Invert,
            false => Oddrowinvdata::True,
        }
    }
    #[doc = "Use inverted data"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == Oddrowinvdata::Invert
    }
    #[doc = "Use true data"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Oddrowinvdata::True
    }
}
#[doc = "Field `ODDROWINVDATA` writer - 18:18\\]
Invert data at odd row addresses for program or verify. The LSB of the row address is bit \\[4\\]
of the bank address. This bit only applies when pattern data is used; i.e. the DATAPATEN bit is set. It will have no effect if CMDDATA is used."]
pub type OddrowinvdataW<'a, REG> = crate::BitWriter<'a, REG, Oddrowinvdata>;
impl<'a, REG> OddrowinvdataW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use inverted data"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut crate::W<REG> {
        self.variant(Oddrowinvdata::Invert)
    }
    #[doc = "Use true data"]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Oddrowinvdata::True)
    }
}
#[doc = "20:20\\]
Stop read verify on fail. If this bit is set, read verify will halt when the first verify fail is detected. If command is program or erase, another program or erase pulse will be executed. If command is read verify, comand will terminate.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stopveronfail {
    #[doc = "1: Enable"]
    Enable = 1,
    #[doc = "0: Disable"]
    Disable = 0,
}
impl From<Stopveronfail> for bool {
    #[inline(always)]
    fn from(variant: Stopveronfail) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPVERONFAIL` reader - 20:20\\]
Stop read verify on fail. If this bit is set, read verify will halt when the first verify fail is detected. If command is program or erase, another program or erase pulse will be executed. If command is read verify, comand will terminate."]
pub type StopveronfailR = crate::BitReader<Stopveronfail>;
impl StopveronfailR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stopveronfail {
        match self.bits {
            true => Stopveronfail::Enable,
            false => Stopveronfail::Disable,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Stopveronfail::Enable
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Stopveronfail::Disable
    }
}
#[doc = "Field `STOPVERONFAIL` writer - 20:20\\]
Stop read verify on fail. If this bit is set, read verify will halt when the first verify fail is detected. If command is program or erase, another program or erase pulse will be executed. If command is read verify, comand will terminate."]
pub type StopveronfailW<'a, REG> = crate::BitWriter<'a, REG, Stopveronfail>;
impl<'a, REG> StopveronfailW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Stopveronfail::Enable)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Stopveronfail::Disable)
    }
}
#[doc = "Field `RESERVED21` reader - 27:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved21R = crate::FieldReader;
#[doc = "31:28\\]
DTB Mux Select This field will form the select for the primary DTB mux. This mux selects up to 16 sets of 32-bit fields of internal signals to be present to the 32-bit DTB output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dtbmuxsel {
    #[doc = "15: Maximum value"]
    Maximum = 15,
    #[doc = "0: Minimum value"]
    Minimum = 0,
}
impl From<Dtbmuxsel> for u8 {
    #[inline(always)]
    fn from(variant: Dtbmuxsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dtbmuxsel {
    type Ux = u8;
}
impl crate::IsEnum for Dtbmuxsel {}
#[doc = "Field `DTBMUXSEL` reader - 31:28\\]
DTB Mux Select This field will form the select for the primary DTB mux. This mux selects up to 16 sets of 32-bit fields of internal signals to be present to the 32-bit DTB output."]
pub type DtbmuxselR = crate::FieldReader<Dtbmuxsel>;
impl DtbmuxselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dtbmuxsel> {
        match self.bits {
            15 => Some(Dtbmuxsel::Maximum),
            0 => Some(Dtbmuxsel::Minimum),
            _ => None,
        }
    }
    #[doc = "Maximum value"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == Dtbmuxsel::Maximum
    }
    #[doc = "Minimum value"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == Dtbmuxsel::Minimum
    }
}
#[doc = "Field `DTBMUXSEL` writer - 31:28\\]
DTB Mux Select This field will form the select for the primary DTB mux. This mux selects up to 16 sets of 32-bit fields of internal signals to be present to the 32-bit DTB output."]
pub type DtbmuxselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Dtbmuxsel>;
impl<'a, REG> DtbmuxselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Maximum value"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut crate::W<REG> {
        self.variant(Dtbmuxsel::Maximum)
    }
    #[doc = "Minimum value"]
    #[inline(always)]
    pub fn minimum(self) -> &'a mut crate::W<REG> {
        self.variant(Dtbmuxsel::Minimum)
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
Force 1T Enable - Force 1T access to regions that are designated as 2T. Regions designated as 1T will still be accessed as 1T."]
    #[inline(always)]
    pub fn force1ten(&self) -> Force1tenR {
        Force1tenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Force 2T Enable - Force 2T access to regions that are designated as 1T. Regions designated as 2T will still be accessed as 2T."]
    #[inline(always)]
    pub fn force2ten(&self) -> Force2tenR {
        Force2tenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
2T address mux disable control. When set to 1 2T address shifting is disabled. This bit should only be enabled for reads. Indeterminate behavior will result if this bit is set during program/erase. Furthermore, only reads done via a READVERIFY command will be guaranteed to work properly. Reads via the FBAP port are not guaranteed to operate."]
    #[inline(always)]
    pub fn amx2tdis(&self) -> Amx2tdisR {
        Amx2tdisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Disable redundancy matching. Any repair configuration encoded into the bank trim bits is disabled."]
    #[inline(always)]
    pub fn redmatchdis(&self) -> RedmatchdisR {
        RedmatchdisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Force redundancy match. If set to 1, repair configuration encoded in the flash bank trim will be forced for every access."]
    #[inline(always)]
    pub fn redmatchforce(&self) -> RedmatchforceR {
        RedmatchforceR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Override address counter enable. When set, the state machine address counter will not be loaded when a command is initiated."]
    #[inline(always)]
    pub fn addrcntlddis(&self) -> AddrcntlddisR {
        AddrcntlddisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Override pulse counter enable. When set, the state machine pulse counter will not be loaded when a command is initiated."]
    #[inline(always)]
    pub fn pulsecntlddis(&self) -> PulsecntlddisR {
        PulsecntlddisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> Reserved10R {
        Reserved10R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - 12:12\\]
Enable data pattern. Data pattern select in DATAPATSEL field will override data from CMDDATA registers for use as program or verify data."]
    #[inline(always)]
    pub fn datapaten(&self) -> DatapatenR {
        DatapatenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Select data pattern. Valid when DATAPATEN bit is set to 1. Overrides CMDDATA registers for program or verify."]
    #[inline(always)]
    pub fn datapatsel(&self) -> DatapatselR {
        DatapatselR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Invert data always for program or verify. This bit only applies when pattern data is used; i.e. the DATAPATEN bit is set. It will have no effect if CMDDATA is used."]
    #[inline(always)]
    pub fn alwaysinvdata(&self) -> AlwaysinvdataR {
        AlwaysinvdataR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Invert data at odd bank addresses for program or verify. This bit only applies when pattern data is used; i.e. the DATAPATEN bit is set. It will have no effect if CMDDATA is used."]
    #[inline(always)]
    pub fn oddwordinvdata(&self) -> OddwordinvdataR {
        OddwordinvdataR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Invert data at odd row addresses for program or verify. The LSB of the row address is bit \\[4\\]
of the bank address. This bit only applies when pattern data is used; i.e. the DATAPATEN bit is set. It will have no effect if CMDDATA is used."]
    #[inline(always)]
    pub fn oddrowinvdata(&self) -> OddrowinvdataR {
        OddrowinvdataR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Stop read verify on fail. If this bit is set, read verify will halt when the first verify fail is detected. If command is program or erase, another program or erase pulse will be executed. If command is read verify, comand will terminate."]
    #[inline(always)]
    pub fn stopveronfail(&self) -> StopveronfailR {
        StopveronfailR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:27 - 27:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved21(&self) -> Reserved21R {
        Reserved21R::new(((self.bits >> 21) & 0x7f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
DTB Mux Select This field will form the select for the primary DTB mux. This mux selects up to 16 sets of 32-bit fields of internal signals to be present to the 32-bit DTB output."]
    #[inline(always)]
    pub fn dtbmuxsel(&self) -> DtbmuxselR {
        DtbmuxselR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Force 1T Enable - Force 1T access to regions that are designated as 2T. Regions designated as 1T will still be accessed as 1T."]
    #[inline(always)]
    #[must_use]
    pub fn force1ten(&mut self) -> Force1tenW<DftcmdctlSpec> {
        Force1tenW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Force 2T Enable - Force 2T access to regions that are designated as 1T. Regions designated as 2T will still be accessed as 2T."]
    #[inline(always)]
    #[must_use]
    pub fn force2ten(&mut self) -> Force2tenW<DftcmdctlSpec> {
        Force2tenW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
2T address mux disable control. When set to 1 2T address shifting is disabled. This bit should only be enabled for reads. Indeterminate behavior will result if this bit is set during program/erase. Furthermore, only reads done via a READVERIFY command will be guaranteed to work properly. Reads via the FBAP port are not guaranteed to operate."]
    #[inline(always)]
    #[must_use]
    pub fn amx2tdis(&mut self) -> Amx2tdisW<DftcmdctlSpec> {
        Amx2tdisW::new(self, 2)
    }
    #[doc = "Bit 4 - 4:4\\]
Disable redundancy matching. Any repair configuration encoded into the bank trim bits is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn redmatchdis(&mut self) -> RedmatchdisW<DftcmdctlSpec> {
        RedmatchdisW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Force redundancy match. If set to 1, repair configuration encoded in the flash bank trim will be forced for every access."]
    #[inline(always)]
    #[must_use]
    pub fn redmatchforce(&mut self) -> RedmatchforceW<DftcmdctlSpec> {
        RedmatchforceW::new(self, 5)
    }
    #[doc = "Bit 8 - 8:8\\]
Override address counter enable. When set, the state machine address counter will not be loaded when a command is initiated."]
    #[inline(always)]
    #[must_use]
    pub fn addrcntlddis(&mut self) -> AddrcntlddisW<DftcmdctlSpec> {
        AddrcntlddisW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Override pulse counter enable. When set, the state machine pulse counter will not be loaded when a command is initiated."]
    #[inline(always)]
    #[must_use]
    pub fn pulsecntlddis(&mut self) -> PulsecntlddisW<DftcmdctlSpec> {
        PulsecntlddisW::new(self, 9)
    }
    #[doc = "Bit 12 - 12:12\\]
Enable data pattern. Data pattern select in DATAPATSEL field will override data from CMDDATA registers for use as program or verify data."]
    #[inline(always)]
    #[must_use]
    pub fn datapaten(&mut self) -> DatapatenW<DftcmdctlSpec> {
        DatapatenW::new(self, 12)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Select data pattern. Valid when DATAPATEN bit is set to 1. Overrides CMDDATA registers for program or verify."]
    #[inline(always)]
    #[must_use]
    pub fn datapatsel(&mut self) -> DatapatselW<DftcmdctlSpec> {
        DatapatselW::new(self, 13)
    }
    #[doc = "Bit 16 - 16:16\\]
Invert data always for program or verify. This bit only applies when pattern data is used; i.e. the DATAPATEN bit is set. It will have no effect if CMDDATA is used."]
    #[inline(always)]
    #[must_use]
    pub fn alwaysinvdata(&mut self) -> AlwaysinvdataW<DftcmdctlSpec> {
        AlwaysinvdataW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Invert data at odd bank addresses for program or verify. This bit only applies when pattern data is used; i.e. the DATAPATEN bit is set. It will have no effect if CMDDATA is used."]
    #[inline(always)]
    #[must_use]
    pub fn oddwordinvdata(&mut self) -> OddwordinvdataW<DftcmdctlSpec> {
        OddwordinvdataW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Invert data at odd row addresses for program or verify. The LSB of the row address is bit \\[4\\]
of the bank address. This bit only applies when pattern data is used; i.e. the DATAPATEN bit is set. It will have no effect if CMDDATA is used."]
    #[inline(always)]
    #[must_use]
    pub fn oddrowinvdata(&mut self) -> OddrowinvdataW<DftcmdctlSpec> {
        OddrowinvdataW::new(self, 18)
    }
    #[doc = "Bit 20 - 20:20\\]
Stop read verify on fail. If this bit is set, read verify will halt when the first verify fail is detected. If command is program or erase, another program or erase pulse will be executed. If command is read verify, comand will terminate."]
    #[inline(always)]
    #[must_use]
    pub fn stopveronfail(&mut self) -> StopveronfailW<DftcmdctlSpec> {
        StopveronfailW::new(self, 20)
    }
    #[doc = "Bits 28:31 - 31:28\\]
DTB Mux Select This field will form the select for the primary DTB mux. This mux selects up to 16 sets of 32-bit fields of internal signals to be present to the 32-bit DTB output."]
    #[inline(always)]
    #[must_use]
    pub fn dtbmuxsel(&mut self) -> DtbmuxselW<DftcmdctlSpec> {
        DtbmuxselW::new(self, 28)
    }
}
#[doc = "DFT Command Control Register This register configures specific capabilities for test. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dftcmdctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dftcmdctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DftcmdctlSpec;
impl crate::RegisterSpec for DftcmdctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dftcmdctl::R`](R) reader structure"]
impl crate::Readable for DftcmdctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dftcmdctl::W`](W) writer structure"]
impl crate::Writable for DftcmdctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFTCMDCTL to value 0"]
impl crate::Resettable for DftcmdctlSpec {
    const RESET_VALUE: u32 = 0;
}
