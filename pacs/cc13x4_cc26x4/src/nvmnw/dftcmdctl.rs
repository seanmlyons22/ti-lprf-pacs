#[doc = "Register `DFTCMDCTL` reader"]
pub struct R(crate::R<DFTCMDCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFTCMDCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFTCMDCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFTCMDCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFTCMDCTL` writer"]
pub struct W(crate::W<DFTCMDCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFTCMDCTL_SPEC>;
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
impl From<crate::W<DFTCMDCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFTCMDCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORCE1TEN` reader - 0:0\\]
Force 1T Enable - Force 1T access to regions that are designated as 2T. Regions designated as 1T will still be accessed as 1T."]
pub type FORCE1TEN_R = crate::BitReader<FORCE1TEN_A>;
#[doc = "0:0\\]
Force 1T Enable - Force 1T access to regions that are designated as 2T. Regions designated as 1T will still be accessed as 1T.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FORCE1TEN_A {
    #[doc = "1: Enable"]
    ENABLE = 1,
    #[doc = "0: Disable"]
    DISABLE = 0,
}
impl From<FORCE1TEN_A> for bool {
    #[inline(always)]
    fn from(variant: FORCE1TEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FORCE1TEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCE1TEN_A {
        match self.bits {
            true => FORCE1TEN_A::ENABLE,
            false => FORCE1TEN_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FORCE1TEN_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FORCE1TEN_A::DISABLE
    }
}
#[doc = "Field `FORCE1TEN` writer - 0:0\\]
Force 1T Enable - Force 1T access to regions that are designated as 2T. Regions designated as 1T will still be accessed as 1T."]
pub type FORCE1TEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFTCMDCTL_SPEC, FORCE1TEN_A, O>;
impl<'a, const O: u8> FORCE1TEN_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FORCE1TEN_A::ENABLE)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FORCE1TEN_A::DISABLE)
    }
}
#[doc = "Field `FORCE2TEN` reader - 1:1\\]
Force 2T Enable - Force 2T access to regions that are designated as 1T. Regions designated as 2T will still be accessed as 2T."]
pub type FORCE2TEN_R = crate::BitReader<FORCE2TEN_A>;
#[doc = "1:1\\]
Force 2T Enable - Force 2T access to regions that are designated as 1T. Regions designated as 2T will still be accessed as 2T.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FORCE2TEN_A {
    #[doc = "1: Enable"]
    ENABLE = 1,
    #[doc = "0: Disable"]
    DISABLE = 0,
}
impl From<FORCE2TEN_A> for bool {
    #[inline(always)]
    fn from(variant: FORCE2TEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FORCE2TEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCE2TEN_A {
        match self.bits {
            true => FORCE2TEN_A::ENABLE,
            false => FORCE2TEN_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FORCE2TEN_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FORCE2TEN_A::DISABLE
    }
}
#[doc = "Field `FORCE2TEN` writer - 1:1\\]
Force 2T Enable - Force 2T access to regions that are designated as 1T. Regions designated as 2T will still be accessed as 2T."]
pub type FORCE2TEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFTCMDCTL_SPEC, FORCE2TEN_A, O>;
impl<'a, const O: u8> FORCE2TEN_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FORCE2TEN_A::ENABLE)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FORCE2TEN_A::DISABLE)
    }
}
#[doc = "Field `AMX2TDIS` reader - 2:2\\]
2T address mux disable control. When set to 1 2T address shifting is disabled. This bit should only be enabled for reads. Indeterminate behavior will result if this bit is set during program/erase. Furthermore, only reads done via a READVERIFY command will be guaranteed to work properly. Reads via the FBAP port are not guaranteed to operate."]
pub type AMX2TDIS_R = crate::BitReader<AMX2TDIS_A>;
#[doc = "2:2\\]
2T address mux disable control. When set to 1 2T address shifting is disabled. This bit should only be enabled for reads. Indeterminate behavior will result if this bit is set during program/erase. Furthermore, only reads done via a READVERIFY command will be guaranteed to work properly. Reads via the FBAP port are not guaranteed to operate.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMX2TDIS_A {
    #[doc = "1: Disable"]
    DISABLE = 1,
    #[doc = "0: Enable"]
    ENABLE = 0,
}
impl From<AMX2TDIS_A> for bool {
    #[inline(always)]
    fn from(variant: AMX2TDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl AMX2TDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMX2TDIS_A {
        match self.bits {
            true => AMX2TDIS_A::DISABLE,
            false => AMX2TDIS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AMX2TDIS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == AMX2TDIS_A::ENABLE
    }
}
#[doc = "Field `AMX2TDIS` writer - 2:2\\]
2T address mux disable control. When set to 1 2T address shifting is disabled. This bit should only be enabled for reads. Indeterminate behavior will result if this bit is set during program/erase. Furthermore, only reads done via a READVERIFY command will be guaranteed to work properly. Reads via the FBAP port are not guaranteed to operate."]
pub type AMX2TDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFTCMDCTL_SPEC, AMX2TDIS_A, O>;
impl<'a, const O: u8> AMX2TDIS_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AMX2TDIS_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AMX2TDIS_A::ENABLE)
    }
}
#[doc = "Field `RESERVED3` reader - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED3` writer - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFTCMDCTL_SPEC, bool, O>;
#[doc = "Field `REDMATCHDIS` reader - 4:4\\]
Disable redundancy matching. Any repair configuration encoded into the bank trim bits is disabled."]
pub type REDMATCHDIS_R = crate::BitReader<REDMATCHDIS_A>;
#[doc = "4:4\\]
Disable redundancy matching. Any repair configuration encoded into the bank trim bits is disabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REDMATCHDIS_A {
    #[doc = "1: Disable"]
    DISABLE = 1,
    #[doc = "0: Enable"]
    ENABLE = 0,
}
impl From<REDMATCHDIS_A> for bool {
    #[inline(always)]
    fn from(variant: REDMATCHDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl REDMATCHDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REDMATCHDIS_A {
        match self.bits {
            true => REDMATCHDIS_A::DISABLE,
            false => REDMATCHDIS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == REDMATCHDIS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == REDMATCHDIS_A::ENABLE
    }
}
#[doc = "Field `REDMATCHDIS` writer - 4:4\\]
Disable redundancy matching. Any repair configuration encoded into the bank trim bits is disabled."]
pub type REDMATCHDIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DFTCMDCTL_SPEC, REDMATCHDIS_A, O>;
impl<'a, const O: u8> REDMATCHDIS_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(REDMATCHDIS_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(REDMATCHDIS_A::ENABLE)
    }
}
#[doc = "Field `REDMATCHFORCE` reader - 5:5\\]
Force redundancy match. If set to 1, repair configuration encoded in the flash bank trim will be forced for every access."]
pub type REDMATCHFORCE_R = crate::BitReader<REDMATCHFORCE_A>;
#[doc = "5:5\\]
Force redundancy match. If set to 1, repair configuration encoded in the flash bank trim will be forced for every access.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REDMATCHFORCE_A {
    #[doc = "1: Enable"]
    ENABLE = 1,
    #[doc = "0: Disable"]
    DISABLE = 0,
}
impl From<REDMATCHFORCE_A> for bool {
    #[inline(always)]
    fn from(variant: REDMATCHFORCE_A) -> Self {
        variant as u8 != 0
    }
}
impl REDMATCHFORCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REDMATCHFORCE_A {
        match self.bits {
            true => REDMATCHFORCE_A::ENABLE,
            false => REDMATCHFORCE_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == REDMATCHFORCE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == REDMATCHFORCE_A::DISABLE
    }
}
#[doc = "Field `REDMATCHFORCE` writer - 5:5\\]
Force redundancy match. If set to 1, repair configuration encoded in the flash bank trim will be forced for every access."]
pub type REDMATCHFORCE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DFTCMDCTL_SPEC, REDMATCHFORCE_A, O>;
impl<'a, const O: u8> REDMATCHFORCE_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(REDMATCHFORCE_A::ENABLE)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(REDMATCHFORCE_A::DISABLE)
    }
}
#[doc = "Field `RESERVED6` reader - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED6` writer - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFTCMDCTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `ADDRCNTLDDIS` reader - 8:8\\]
Override address counter enable. When set, the state machine address counter will not be loaded when a command is initiated."]
pub type ADDRCNTLDDIS_R = crate::BitReader<ADDRCNTLDDIS_A>;
#[doc = "8:8\\]
Override address counter enable. When set, the state machine address counter will not be loaded when a command is initiated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDRCNTLDDIS_A {
    #[doc = "1: Disable"]
    DISABLE = 1,
    #[doc = "0: Enable"]
    ENABLE = 0,
}
impl From<ADDRCNTLDDIS_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRCNTLDDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDRCNTLDDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDRCNTLDDIS_A {
        match self.bits {
            true => ADDRCNTLDDIS_A::DISABLE,
            false => ADDRCNTLDDIS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADDRCNTLDDIS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADDRCNTLDDIS_A::ENABLE
    }
}
#[doc = "Field `ADDRCNTLDDIS` writer - 8:8\\]
Override address counter enable. When set, the state machine address counter will not be loaded when a command is initiated."]
pub type ADDRCNTLDDIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DFTCMDCTL_SPEC, ADDRCNTLDDIS_A, O>;
impl<'a, const O: u8> ADDRCNTLDDIS_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADDRCNTLDDIS_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADDRCNTLDDIS_A::ENABLE)
    }
}
#[doc = "Field `PULSECNTLDDIS` reader - 9:9\\]
Override pulse counter enable. When set, the state machine pulse counter will not be loaded when a command is initiated."]
pub type PULSECNTLDDIS_R = crate::BitReader<PULSECNTLDDIS_A>;
#[doc = "9:9\\]
Override pulse counter enable. When set, the state machine pulse counter will not be loaded when a command is initiated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PULSECNTLDDIS_A {
    #[doc = "1: Disable"]
    DISABLE = 1,
    #[doc = "0: Enable"]
    ENABLE = 0,
}
impl From<PULSECNTLDDIS_A> for bool {
    #[inline(always)]
    fn from(variant: PULSECNTLDDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl PULSECNTLDDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PULSECNTLDDIS_A {
        match self.bits {
            true => PULSECNTLDDIS_A::DISABLE,
            false => PULSECNTLDDIS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PULSECNTLDDIS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PULSECNTLDDIS_A::ENABLE
    }
}
#[doc = "Field `PULSECNTLDDIS` writer - 9:9\\]
Override pulse counter enable. When set, the state machine pulse counter will not be loaded when a command is initiated."]
pub type PULSECNTLDDIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DFTCMDCTL_SPEC, PULSECNTLDDIS_A, O>;
impl<'a, const O: u8> PULSECNTLDDIS_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PULSECNTLDDIS_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PULSECNTLDDIS_A::ENABLE)
    }
}
#[doc = "Field `RESERVED10` reader - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED10` writer - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFTCMDCTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `DATAPATEN` reader - 12:12\\]
Enable data pattern. Data pattern select in DATAPATSEL field will override data from CMDDATA registers for use as program or verify data."]
pub type DATAPATEN_R = crate::BitReader<DATAPATEN_A>;
#[doc = "12:12\\]
Enable data pattern. Data pattern select in DATAPATSEL field will override data from CMDDATA registers for use as program or verify data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATAPATEN_A {
    #[doc = "1: Enable"]
    ENABLE = 1,
    #[doc = "0: Disable"]
    DISABLE = 0,
}
impl From<DATAPATEN_A> for bool {
    #[inline(always)]
    fn from(variant: DATAPATEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DATAPATEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATAPATEN_A {
        match self.bits {
            true => DATAPATEN_A::ENABLE,
            false => DATAPATEN_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DATAPATEN_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DATAPATEN_A::DISABLE
    }
}
#[doc = "Field `DATAPATEN` writer - 12:12\\]
Enable data pattern. Data pattern select in DATAPATSEL field will override data from CMDDATA registers for use as program or verify data."]
pub type DATAPATEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFTCMDCTL_SPEC, DATAPATEN_A, O>;
impl<'a, const O: u8> DATAPATEN_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DATAPATEN_A::ENABLE)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DATAPATEN_A::DISABLE)
    }
}
#[doc = "Field `DATAPATSEL` reader - 15:13\\]
Select data pattern. Valid when DATAPATEN bit is set to 1. Overrides CMDDATA registers for program or verify."]
pub type DATAPATSEL_R = crate::FieldReader<u8, DATAPATSEL_A>;
#[doc = "15:13\\]
Select data pattern. Valid when DATAPATEN bit is set to 1. Overrides CMDDATA registers for program or verify.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATAPATSEL_A {
    #[doc = "2: Set to logical checkerboard (0x01010101...)"]
    LOGCHKBRD = 2,
    #[doc = "1: Set to all 1"]
    ALL1 = 1,
    #[doc = "0: Set to all 0"]
    ALL0 = 0,
}
impl From<DATAPATSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DATAPATSEL_A) -> Self {
        variant as _
    }
}
impl DATAPATSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DATAPATSEL_A> {
        match self.bits {
            2 => Some(DATAPATSEL_A::LOGCHKBRD),
            1 => Some(DATAPATSEL_A::ALL1),
            0 => Some(DATAPATSEL_A::ALL0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOGCHKBRD`"]
    #[inline(always)]
    pub fn is_logchkbrd(&self) -> bool {
        *self == DATAPATSEL_A::LOGCHKBRD
    }
    #[doc = "Checks if the value of the field is `ALL1`"]
    #[inline(always)]
    pub fn is_all1(&self) -> bool {
        *self == DATAPATSEL_A::ALL1
    }
    #[doc = "Checks if the value of the field is `ALL0`"]
    #[inline(always)]
    pub fn is_all0(&self) -> bool {
        *self == DATAPATSEL_A::ALL0
    }
}
#[doc = "Field `DATAPATSEL` writer - 15:13\\]
Select data pattern. Valid when DATAPATEN bit is set to 1. Overrides CMDDATA registers for program or verify."]
pub type DATAPATSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFTCMDCTL_SPEC, u8, DATAPATSEL_A, 3, O>;
impl<'a, const O: u8> DATAPATSEL_W<'a, O> {
    #[doc = "Set to logical checkerboard (0x01010101...)"]
    #[inline(always)]
    pub fn logchkbrd(self) -> &'a mut W {
        self.variant(DATAPATSEL_A::LOGCHKBRD)
    }
    #[doc = "Set to all 1"]
    #[inline(always)]
    pub fn all1(self) -> &'a mut W {
        self.variant(DATAPATSEL_A::ALL1)
    }
    #[doc = "Set to all 0"]
    #[inline(always)]
    pub fn all0(self) -> &'a mut W {
        self.variant(DATAPATSEL_A::ALL0)
    }
}
#[doc = "Field `ALWAYSINVDATA` reader - 16:16\\]
Invert data always for program or verify. This bit only applies when pattern data is used; i.e. the DATAPATEN bit is set. It will have no effect if CMDDATA is used."]
pub type ALWAYSINVDATA_R = crate::BitReader<ALWAYSINVDATA_A>;
#[doc = "16:16\\]
Invert data always for program or verify. This bit only applies when pattern data is used; i.e. the DATAPATEN bit is set. It will have no effect if CMDDATA is used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALWAYSINVDATA_A {
    #[doc = "1: Use inverted data"]
    INVERT = 1,
    #[doc = "0: Use true data"]
    TRUE = 0,
}
impl From<ALWAYSINVDATA_A> for bool {
    #[inline(always)]
    fn from(variant: ALWAYSINVDATA_A) -> Self {
        variant as u8 != 0
    }
}
impl ALWAYSINVDATA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALWAYSINVDATA_A {
        match self.bits {
            true => ALWAYSINVDATA_A::INVERT,
            false => ALWAYSINVDATA_A::TRUE,
        }
    }
    #[doc = "Checks if the value of the field is `INVERT`"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == ALWAYSINVDATA_A::INVERT
    }
    #[doc = "Checks if the value of the field is `TRUE`"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == ALWAYSINVDATA_A::TRUE
    }
}
#[doc = "Field `ALWAYSINVDATA` writer - 16:16\\]
Invert data always for program or verify. This bit only applies when pattern data is used; i.e. the DATAPATEN bit is set. It will have no effect if CMDDATA is used."]
pub type ALWAYSINVDATA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DFTCMDCTL_SPEC, ALWAYSINVDATA_A, O>;
impl<'a, const O: u8> ALWAYSINVDATA_W<'a, O> {
    #[doc = "Use inverted data"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut W {
        self.variant(ALWAYSINVDATA_A::INVERT)
    }
    #[doc = "Use true data"]
    #[inline(always)]
    pub fn true_(self) -> &'a mut W {
        self.variant(ALWAYSINVDATA_A::TRUE)
    }
}
#[doc = "Field `ODDWORDINVDATA` reader - 17:17\\]
Invert data at odd bank addresses for program or verify. This bit only applies when pattern data is used; i.e. the DATAPATEN bit is set. It will have no effect if CMDDATA is used."]
pub type ODDWORDINVDATA_R = crate::BitReader<ODDWORDINVDATA_A>;
#[doc = "17:17\\]
Invert data at odd bank addresses for program or verify. This bit only applies when pattern data is used; i.e. the DATAPATEN bit is set. It will have no effect if CMDDATA is used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ODDWORDINVDATA_A {
    #[doc = "1: Use inverted data"]
    INVERT = 1,
    #[doc = "0: Use true data"]
    TRUE = 0,
}
impl From<ODDWORDINVDATA_A> for bool {
    #[inline(always)]
    fn from(variant: ODDWORDINVDATA_A) -> Self {
        variant as u8 != 0
    }
}
impl ODDWORDINVDATA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ODDWORDINVDATA_A {
        match self.bits {
            true => ODDWORDINVDATA_A::INVERT,
            false => ODDWORDINVDATA_A::TRUE,
        }
    }
    #[doc = "Checks if the value of the field is `INVERT`"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == ODDWORDINVDATA_A::INVERT
    }
    #[doc = "Checks if the value of the field is `TRUE`"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == ODDWORDINVDATA_A::TRUE
    }
}
#[doc = "Field `ODDWORDINVDATA` writer - 17:17\\]
Invert data at odd bank addresses for program or verify. This bit only applies when pattern data is used; i.e. the DATAPATEN bit is set. It will have no effect if CMDDATA is used."]
pub type ODDWORDINVDATA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DFTCMDCTL_SPEC, ODDWORDINVDATA_A, O>;
impl<'a, const O: u8> ODDWORDINVDATA_W<'a, O> {
    #[doc = "Use inverted data"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut W {
        self.variant(ODDWORDINVDATA_A::INVERT)
    }
    #[doc = "Use true data"]
    #[inline(always)]
    pub fn true_(self) -> &'a mut W {
        self.variant(ODDWORDINVDATA_A::TRUE)
    }
}
#[doc = "Field `ODDROWINVDATA` reader - 18:18\\]
Invert data at odd row addresses for program or verify. The LSB of the row address is bit \\[4\\]
of the bank address. This bit only applies when pattern data is used; i.e. the DATAPATEN bit is set. It will have no effect if CMDDATA is used."]
pub type ODDROWINVDATA_R = crate::BitReader<ODDROWINVDATA_A>;
#[doc = "18:18\\]
Invert data at odd row addresses for program or verify. The LSB of the row address is bit \\[4\\]
of the bank address. This bit only applies when pattern data is used; i.e. the DATAPATEN bit is set. It will have no effect if CMDDATA is used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ODDROWINVDATA_A {
    #[doc = "1: Use inverted data"]
    INVERT = 1,
    #[doc = "0: Use true data"]
    TRUE = 0,
}
impl From<ODDROWINVDATA_A> for bool {
    #[inline(always)]
    fn from(variant: ODDROWINVDATA_A) -> Self {
        variant as u8 != 0
    }
}
impl ODDROWINVDATA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ODDROWINVDATA_A {
        match self.bits {
            true => ODDROWINVDATA_A::INVERT,
            false => ODDROWINVDATA_A::TRUE,
        }
    }
    #[doc = "Checks if the value of the field is `INVERT`"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == ODDROWINVDATA_A::INVERT
    }
    #[doc = "Checks if the value of the field is `TRUE`"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == ODDROWINVDATA_A::TRUE
    }
}
#[doc = "Field `ODDROWINVDATA` writer - 18:18\\]
Invert data at odd row addresses for program or verify. The LSB of the row address is bit \\[4\\]
of the bank address. This bit only applies when pattern data is used; i.e. the DATAPATEN bit is set. It will have no effect if CMDDATA is used."]
pub type ODDROWINVDATA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DFTCMDCTL_SPEC, ODDROWINVDATA_A, O>;
impl<'a, const O: u8> ODDROWINVDATA_W<'a, O> {
    #[doc = "Use inverted data"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut W {
        self.variant(ODDROWINVDATA_A::INVERT)
    }
    #[doc = "Use true data"]
    #[inline(always)]
    pub fn true_(self) -> &'a mut W {
        self.variant(ODDROWINVDATA_A::TRUE)
    }
}
#[doc = "Field `STOPVERONFAIL` reader - 20:20\\]
Stop read verify on fail. If this bit is set, read verify will halt when the first verify fail is detected. If command is program or erase, another program or erase pulse will be executed. If command is read verify, comand will terminate."]
pub type STOPVERONFAIL_R = crate::BitReader<STOPVERONFAIL_A>;
#[doc = "20:20\\]
Stop read verify on fail. If this bit is set, read verify will halt when the first verify fail is detected. If command is program or erase, another program or erase pulse will be executed. If command is read verify, comand will terminate.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPVERONFAIL_A {
    #[doc = "1: Enable"]
    ENABLE = 1,
    #[doc = "0: Disable"]
    DISABLE = 0,
}
impl From<STOPVERONFAIL_A> for bool {
    #[inline(always)]
    fn from(variant: STOPVERONFAIL_A) -> Self {
        variant as u8 != 0
    }
}
impl STOPVERONFAIL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPVERONFAIL_A {
        match self.bits {
            true => STOPVERONFAIL_A::ENABLE,
            false => STOPVERONFAIL_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STOPVERONFAIL_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STOPVERONFAIL_A::DISABLE
    }
}
#[doc = "Field `STOPVERONFAIL` writer - 20:20\\]
Stop read verify on fail. If this bit is set, read verify will halt when the first verify fail is detected. If command is program or erase, another program or erase pulse will be executed. If command is read verify, comand will terminate."]
pub type STOPVERONFAIL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DFTCMDCTL_SPEC, STOPVERONFAIL_A, O>;
impl<'a, const O: u8> STOPVERONFAIL_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(STOPVERONFAIL_A::ENABLE)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(STOPVERONFAIL_A::DISABLE)
    }
}
#[doc = "Field `RESERVED21` reader - 27:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED21_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED21` writer - 27:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED21_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFTCMDCTL_SPEC, u8, u8, 7, O>;
#[doc = "Field `DTBMUXSEL` reader - 31:28\\]
DTB Mux Select This field will form the select for the primary DTB mux. This mux selects up to 16 sets of 32-bit fields of internal signals to be present to the 32-bit DTB output."]
pub type DTBMUXSEL_R = crate::FieldReader<u8, DTBMUXSEL_A>;
#[doc = "31:28\\]
DTB Mux Select This field will form the select for the primary DTB mux. This mux selects up to 16 sets of 32-bit fields of internal signals to be present to the 32-bit DTB output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DTBMUXSEL_A {
    #[doc = "15: Maximum value"]
    MAXIMUM = 15,
    #[doc = "0: Minimum value"]
    MINIMUM = 0,
}
impl From<DTBMUXSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DTBMUXSEL_A) -> Self {
        variant as _
    }
}
impl DTBMUXSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DTBMUXSEL_A> {
        match self.bits {
            15 => Some(DTBMUXSEL_A::MAXIMUM),
            0 => Some(DTBMUXSEL_A::MINIMUM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAXIMUM`"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == DTBMUXSEL_A::MAXIMUM
    }
    #[doc = "Checks if the value of the field is `MINIMUM`"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == DTBMUXSEL_A::MINIMUM
    }
}
#[doc = "Field `DTBMUXSEL` writer - 31:28\\]
DTB Mux Select This field will form the select for the primary DTB mux. This mux selects up to 16 sets of 32-bit fields of internal signals to be present to the 32-bit DTB output."]
pub type DTBMUXSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFTCMDCTL_SPEC, u8, DTBMUXSEL_A, 4, O>;
impl<'a, const O: u8> DTBMUXSEL_W<'a, O> {
    #[doc = "Maximum value"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut W {
        self.variant(DTBMUXSEL_A::MAXIMUM)
    }
    #[doc = "Minimum value"]
    #[inline(always)]
    pub fn minimum(self) -> &'a mut W {
        self.variant(DTBMUXSEL_A::MINIMUM)
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
Force 1T Enable - Force 1T access to regions that are designated as 2T. Regions designated as 1T will still be accessed as 1T."]
    #[inline(always)]
    pub fn force1ten(&self) -> FORCE1TEN_R {
        FORCE1TEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Force 2T Enable - Force 2T access to regions that are designated as 1T. Regions designated as 2T will still be accessed as 2T."]
    #[inline(always)]
    pub fn force2ten(&self) -> FORCE2TEN_R {
        FORCE2TEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
2T address mux disable control. When set to 1 2T address shifting is disabled. This bit should only be enabled for reads. Indeterminate behavior will result if this bit is set during program/erase. Furthermore, only reads done via a READVERIFY command will be guaranteed to work properly. Reads via the FBAP port are not guaranteed to operate."]
    #[inline(always)]
    pub fn amx2tdis(&self) -> AMX2TDIS_R {
        AMX2TDIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Disable redundancy matching. Any repair configuration encoded into the bank trim bits is disabled."]
    #[inline(always)]
    pub fn redmatchdis(&self) -> REDMATCHDIS_R {
        REDMATCHDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Force redundancy match. If set to 1, repair configuration encoded in the flash bank trim will be forced for every access."]
    #[inline(always)]
    pub fn redmatchforce(&self) -> REDMATCHFORCE_R {
        REDMATCHFORCE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Override address counter enable. When set, the state machine address counter will not be loaded when a command is initiated."]
    #[inline(always)]
    pub fn addrcntlddis(&self) -> ADDRCNTLDDIS_R {
        ADDRCNTLDDIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Override pulse counter enable. When set, the state machine pulse counter will not be loaded when a command is initiated."]
    #[inline(always)]
    pub fn pulsecntlddis(&self) -> PULSECNTLDDIS_R {
        PULSECNTLDDIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> RESERVED10_R {
        RESERVED10_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - 12:12\\]
Enable data pattern. Data pattern select in DATAPATSEL field will override data from CMDDATA registers for use as program or verify data."]
    #[inline(always)]
    pub fn datapaten(&self) -> DATAPATEN_R {
        DATAPATEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Select data pattern. Valid when DATAPATEN bit is set to 1. Overrides CMDDATA registers for program or verify."]
    #[inline(always)]
    pub fn datapatsel(&self) -> DATAPATSEL_R {
        DATAPATSEL_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Invert data always for program or verify. This bit only applies when pattern data is used; i.e. the DATAPATEN bit is set. It will have no effect if CMDDATA is used."]
    #[inline(always)]
    pub fn alwaysinvdata(&self) -> ALWAYSINVDATA_R {
        ALWAYSINVDATA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Invert data at odd bank addresses for program or verify. This bit only applies when pattern data is used; i.e. the DATAPATEN bit is set. It will have no effect if CMDDATA is used."]
    #[inline(always)]
    pub fn oddwordinvdata(&self) -> ODDWORDINVDATA_R {
        ODDWORDINVDATA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Invert data at odd row addresses for program or verify. The LSB of the row address is bit \\[4\\]
of the bank address. This bit only applies when pattern data is used; i.e. the DATAPATEN bit is set. It will have no effect if CMDDATA is used."]
    #[inline(always)]
    pub fn oddrowinvdata(&self) -> ODDROWINVDATA_R {
        ODDROWINVDATA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Stop read verify on fail. If this bit is set, read verify will halt when the first verify fail is detected. If command is program or erase, another program or erase pulse will be executed. If command is read verify, comand will terminate."]
    #[inline(always)]
    pub fn stopveronfail(&self) -> STOPVERONFAIL_R {
        STOPVERONFAIL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:27 - 27:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved21(&self) -> RESERVED21_R {
        RESERVED21_R::new(((self.bits >> 21) & 0x7f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
DTB Mux Select This field will form the select for the primary DTB mux. This mux selects up to 16 sets of 32-bit fields of internal signals to be present to the 32-bit DTB output."]
    #[inline(always)]
    pub fn dtbmuxsel(&self) -> DTBMUXSEL_R {
        DTBMUXSEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Force 1T Enable - Force 1T access to regions that are designated as 2T. Regions designated as 1T will still be accessed as 1T."]
    #[inline(always)]
    #[must_use]
    pub fn force1ten(&mut self) -> FORCE1TEN_W<0> {
        FORCE1TEN_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Force 2T Enable - Force 2T access to regions that are designated as 1T. Regions designated as 2T will still be accessed as 2T."]
    #[inline(always)]
    #[must_use]
    pub fn force2ten(&mut self) -> FORCE2TEN_W<1> {
        FORCE2TEN_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
2T address mux disable control. When set to 1 2T address shifting is disabled. This bit should only be enabled for reads. Indeterminate behavior will result if this bit is set during program/erase. Furthermore, only reads done via a READVERIFY command will be guaranteed to work properly. Reads via the FBAP port are not guaranteed to operate."]
    #[inline(always)]
    #[must_use]
    pub fn amx2tdis(&mut self) -> AMX2TDIS_W<2> {
        AMX2TDIS_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<3> {
        RESERVED3_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Disable redundancy matching. Any repair configuration encoded into the bank trim bits is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn redmatchdis(&mut self) -> REDMATCHDIS_W<4> {
        REDMATCHDIS_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Force redundancy match. If set to 1, repair configuration encoded in the flash bank trim will be forced for every access."]
    #[inline(always)]
    #[must_use]
    pub fn redmatchforce(&mut self) -> REDMATCHFORCE_W<5> {
        REDMATCHFORCE_W::new(self)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<6> {
        RESERVED6_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Override address counter enable. When set, the state machine address counter will not be loaded when a command is initiated."]
    #[inline(always)]
    #[must_use]
    pub fn addrcntlddis(&mut self) -> ADDRCNTLDDIS_W<8> {
        ADDRCNTLDDIS_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Override pulse counter enable. When set, the state machine pulse counter will not be loaded when a command is initiated."]
    #[inline(always)]
    #[must_use]
    pub fn pulsecntlddis(&mut self) -> PULSECNTLDDIS_W<9> {
        PULSECNTLDDIS_W::new(self)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved10(&mut self) -> RESERVED10_W<10> {
        RESERVED10_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
Enable data pattern. Data pattern select in DATAPATSEL field will override data from CMDDATA registers for use as program or verify data."]
    #[inline(always)]
    #[must_use]
    pub fn datapaten(&mut self) -> DATAPATEN_W<12> {
        DATAPATEN_W::new(self)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Select data pattern. Valid when DATAPATEN bit is set to 1. Overrides CMDDATA registers for program or verify."]
    #[inline(always)]
    #[must_use]
    pub fn datapatsel(&mut self) -> DATAPATSEL_W<13> {
        DATAPATSEL_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
Invert data always for program or verify. This bit only applies when pattern data is used; i.e. the DATAPATEN bit is set. It will have no effect if CMDDATA is used."]
    #[inline(always)]
    #[must_use]
    pub fn alwaysinvdata(&mut self) -> ALWAYSINVDATA_W<16> {
        ALWAYSINVDATA_W::new(self)
    }
    #[doc = "Bit 17 - 17:17\\]
Invert data at odd bank addresses for program or verify. This bit only applies when pattern data is used; i.e. the DATAPATEN bit is set. It will have no effect if CMDDATA is used."]
    #[inline(always)]
    #[must_use]
    pub fn oddwordinvdata(&mut self) -> ODDWORDINVDATA_W<17> {
        ODDWORDINVDATA_W::new(self)
    }
    #[doc = "Bit 18 - 18:18\\]
Invert data at odd row addresses for program or verify. The LSB of the row address is bit \\[4\\]
of the bank address. This bit only applies when pattern data is used; i.e. the DATAPATEN bit is set. It will have no effect if CMDDATA is used."]
    #[inline(always)]
    #[must_use]
    pub fn oddrowinvdata(&mut self) -> ODDROWINVDATA_W<18> {
        ODDROWINVDATA_W::new(self)
    }
    #[doc = "Bit 20 - 20:20\\]
Stop read verify on fail. If this bit is set, read verify will halt when the first verify fail is detected. If command is program or erase, another program or erase pulse will be executed. If command is read verify, comand will terminate."]
    #[inline(always)]
    #[must_use]
    pub fn stopveronfail(&mut self) -> STOPVERONFAIL_W<20> {
        STOPVERONFAIL_W::new(self)
    }
    #[doc = "Bits 21:27 - 27:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved21(&mut self) -> RESERVED21_W<21> {
        RESERVED21_W::new(self)
    }
    #[doc = "Bits 28:31 - 31:28\\]
DTB Mux Select This field will form the select for the primary DTB mux. This mux selects up to 16 sets of 32-bit fields of internal signals to be present to the 32-bit DTB output."]
    #[inline(always)]
    #[must_use]
    pub fn dtbmuxsel(&mut self) -> DTBMUXSEL_W<28> {
        DTBMUXSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFT Command Control Register This register configures specific capabilities for test. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dftcmdctl](index.html) module"]
pub struct DFTCMDCTL_SPEC;
impl crate::RegisterSpec for DFTCMDCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dftcmdctl::R](R) reader structure"]
impl crate::Readable for DFTCMDCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dftcmdctl::W](W) writer structure"]
impl crate::Writable for DFTCMDCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFTCMDCTL to value 0"]
impl crate::Resettable for DFTCMDCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
