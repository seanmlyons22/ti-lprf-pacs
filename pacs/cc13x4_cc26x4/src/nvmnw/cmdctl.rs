#[doc = "Register `CMDCTL` reader"]
pub struct R(crate::R<CMDCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMDCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMDCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMDCTL` writer"]
pub struct W(crate::W<CMDCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMDCTL_SPEC>;
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
impl From<crate::W<CMDCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMDCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODESEL` reader - 3:0\\]
Mode This field is only used for the Mode Change command type. Otherwise, bank and pump modes are set automaticlly via the NW hardware."]
pub type MODESEL_R = crate::FieldReader<u8, MODESEL_A>;
#[doc = "3:0\\]
Mode This field is only used for the Mode Change command type. Otherwise, bank and pump modes are set automaticlly via the NW hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODESEL_A {
    #[doc = "15: Erase Bank"]
    ERASEBNK = 15,
    #[doc = "14: Program Multiple Word"]
    PGMMW = 14,
    #[doc = "12: Erase Sector"]
    ERASESECT = 12,
    #[doc = "11: Erase Verify Mode"]
    ERASEVER = 11,
    #[doc = "10: Program Single Word"]
    PGMSW = 10,
    #[doc = "9: Program Verify Mode"]
    PGMVER = 9,
    #[doc = "7: Read Margin 1B Mode"]
    RDMARG1B = 7,
    #[doc = "6: Read Margin 0B Mode"]
    RDMARG0B = 6,
    #[doc = "4: Read Margin 1 Mode"]
    RDMARG1 = 4,
    #[doc = "2: Read Margin 0 Mode"]
    RDMARG0 = 2,
    #[doc = "0: Read Mode"]
    READ = 0,
}
impl From<MODESEL_A> for u8 {
    #[inline(always)]
    fn from(variant: MODESEL_A) -> Self {
        variant as _
    }
}
impl MODESEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODESEL_A> {
        match self.bits {
            15 => Some(MODESEL_A::ERASEBNK),
            14 => Some(MODESEL_A::PGMMW),
            12 => Some(MODESEL_A::ERASESECT),
            11 => Some(MODESEL_A::ERASEVER),
            10 => Some(MODESEL_A::PGMSW),
            9 => Some(MODESEL_A::PGMVER),
            7 => Some(MODESEL_A::RDMARG1B),
            6 => Some(MODESEL_A::RDMARG0B),
            4 => Some(MODESEL_A::RDMARG1),
            2 => Some(MODESEL_A::RDMARG0),
            0 => Some(MODESEL_A::READ),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ERASEBNK`"]
    #[inline(always)]
    pub fn is_erasebnk(&self) -> bool {
        *self == MODESEL_A::ERASEBNK
    }
    #[doc = "Checks if the value of the field is `PGMMW`"]
    #[inline(always)]
    pub fn is_pgmmw(&self) -> bool {
        *self == MODESEL_A::PGMMW
    }
    #[doc = "Checks if the value of the field is `ERASESECT`"]
    #[inline(always)]
    pub fn is_erasesect(&self) -> bool {
        *self == MODESEL_A::ERASESECT
    }
    #[doc = "Checks if the value of the field is `ERASEVER`"]
    #[inline(always)]
    pub fn is_erasever(&self) -> bool {
        *self == MODESEL_A::ERASEVER
    }
    #[doc = "Checks if the value of the field is `PGMSW`"]
    #[inline(always)]
    pub fn is_pgmsw(&self) -> bool {
        *self == MODESEL_A::PGMSW
    }
    #[doc = "Checks if the value of the field is `PGMVER`"]
    #[inline(always)]
    pub fn is_pgmver(&self) -> bool {
        *self == MODESEL_A::PGMVER
    }
    #[doc = "Checks if the value of the field is `RDMARG1B`"]
    #[inline(always)]
    pub fn is_rdmarg1b(&self) -> bool {
        *self == MODESEL_A::RDMARG1B
    }
    #[doc = "Checks if the value of the field is `RDMARG0B`"]
    #[inline(always)]
    pub fn is_rdmarg0b(&self) -> bool {
        *self == MODESEL_A::RDMARG0B
    }
    #[doc = "Checks if the value of the field is `RDMARG1`"]
    #[inline(always)]
    pub fn is_rdmarg1(&self) -> bool {
        *self == MODESEL_A::RDMARG1
    }
    #[doc = "Checks if the value of the field is `RDMARG0`"]
    #[inline(always)]
    pub fn is_rdmarg0(&self) -> bool {
        *self == MODESEL_A::RDMARG0
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == MODESEL_A::READ
    }
}
#[doc = "Field `MODESEL` writer - 3:0\\]
Mode This field is only used for the Mode Change command type. Otherwise, bank and pump modes are set automaticlly via the NW hardware."]
pub type MODESEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMDCTL_SPEC, u8, MODESEL_A, 4, O>;
impl<'a, const O: u8> MODESEL_W<'a, O> {
    #[doc = "Erase Bank"]
    #[inline(always)]
    pub fn erasebnk(self) -> &'a mut W {
        self.variant(MODESEL_A::ERASEBNK)
    }
    #[doc = "Program Multiple Word"]
    #[inline(always)]
    pub fn pgmmw(self) -> &'a mut W {
        self.variant(MODESEL_A::PGMMW)
    }
    #[doc = "Erase Sector"]
    #[inline(always)]
    pub fn erasesect(self) -> &'a mut W {
        self.variant(MODESEL_A::ERASESECT)
    }
    #[doc = "Erase Verify Mode"]
    #[inline(always)]
    pub fn erasever(self) -> &'a mut W {
        self.variant(MODESEL_A::ERASEVER)
    }
    #[doc = "Program Single Word"]
    #[inline(always)]
    pub fn pgmsw(self) -> &'a mut W {
        self.variant(MODESEL_A::PGMSW)
    }
    #[doc = "Program Verify Mode"]
    #[inline(always)]
    pub fn pgmver(self) -> &'a mut W {
        self.variant(MODESEL_A::PGMVER)
    }
    #[doc = "Read Margin 1B Mode"]
    #[inline(always)]
    pub fn rdmarg1b(self) -> &'a mut W {
        self.variant(MODESEL_A::RDMARG1B)
    }
    #[doc = "Read Margin 0B Mode"]
    #[inline(always)]
    pub fn rdmarg0b(self) -> &'a mut W {
        self.variant(MODESEL_A::RDMARG0B)
    }
    #[doc = "Read Margin 1 Mode"]
    #[inline(always)]
    pub fn rdmarg1(self) -> &'a mut W {
        self.variant(MODESEL_A::RDMARG1)
    }
    #[doc = "Read Margin 0 Mode"]
    #[inline(always)]
    pub fn rdmarg0(self) -> &'a mut W {
        self.variant(MODESEL_A::RDMARG0)
    }
    #[doc = "Read Mode"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(MODESEL_A::READ)
    }
}
#[doc = "Field `BANKSEL` reader - 4:8\\]
Bank Select A specific Bank ID can be written to this field to indicate to which bank an operation should be applied if CMDCTL.ADDRXLATEOVR is set."]
pub type BANKSEL_R = crate::FieldReader<u8, BANKSEL_A>;
#[doc = "4:8\\]
Bank Select A specific Bank ID can be written to this field to indicate to which bank an operation should be applied if CMDCTL.ADDRXLATEOVR is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BANKSEL_A {
    #[doc = "16: Bank 4"]
    BANK4 = 16,
    #[doc = "8: Bank 3"]
    BANK3 = 8,
    #[doc = "4: Bank 2"]
    BANK2 = 4,
    #[doc = "2: Bank 1"]
    BANK1 = 2,
    #[doc = "1: Bank 0"]
    BANK0 = 1,
}
impl From<BANKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: BANKSEL_A) -> Self {
        variant as _
    }
}
impl BANKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BANKSEL_A> {
        match self.bits {
            16 => Some(BANKSEL_A::BANK4),
            8 => Some(BANKSEL_A::BANK3),
            4 => Some(BANKSEL_A::BANK2),
            2 => Some(BANKSEL_A::BANK1),
            1 => Some(BANKSEL_A::BANK0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BANK4`"]
    #[inline(always)]
    pub fn is_bank4(&self) -> bool {
        *self == BANKSEL_A::BANK4
    }
    #[doc = "Checks if the value of the field is `BANK3`"]
    #[inline(always)]
    pub fn is_bank3(&self) -> bool {
        *self == BANKSEL_A::BANK3
    }
    #[doc = "Checks if the value of the field is `BANK2`"]
    #[inline(always)]
    pub fn is_bank2(&self) -> bool {
        *self == BANKSEL_A::BANK2
    }
    #[doc = "Checks if the value of the field is `BANK1`"]
    #[inline(always)]
    pub fn is_bank1(&self) -> bool {
        *self == BANKSEL_A::BANK1
    }
    #[doc = "Checks if the value of the field is `BANK0`"]
    #[inline(always)]
    pub fn is_bank0(&self) -> bool {
        *self == BANKSEL_A::BANK0
    }
}
#[doc = "Field `BANKSEL` writer - 4:8\\]
Bank Select A specific Bank ID can be written to this field to indicate to which bank an operation should be applied if CMDCTL.ADDRXLATEOVR is set."]
pub type BANKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMDCTL_SPEC, u8, BANKSEL_A, 5, O>;
impl<'a, const O: u8> BANKSEL_W<'a, O> {
    #[doc = "Bank 4"]
    #[inline(always)]
    pub fn bank4(self) -> &'a mut W {
        self.variant(BANKSEL_A::BANK4)
    }
    #[doc = "Bank 3"]
    #[inline(always)]
    pub fn bank3(self) -> &'a mut W {
        self.variant(BANKSEL_A::BANK3)
    }
    #[doc = "Bank 2"]
    #[inline(always)]
    pub fn bank2(self) -> &'a mut W {
        self.variant(BANKSEL_A::BANK2)
    }
    #[doc = "Bank 1"]
    #[inline(always)]
    pub fn bank1(self) -> &'a mut W {
        self.variant(BANKSEL_A::BANK1)
    }
    #[doc = "Bank 0"]
    #[inline(always)]
    pub fn bank0(self) -> &'a mut W {
        self.variant(BANKSEL_A::BANK0)
    }
}
#[doc = "Field `REGIONSEL` reader - 12:9\\]
Bank Region A specific region ID can be written to this field to indicate to which region an operation should be applied if CMDCTL.ADDRXLATEOVR is set."]
pub type REGIONSEL_R = crate::FieldReader<u8, REGIONSEL_A>;
#[doc = "12:9\\]
Bank Region A specific region ID can be written to this field to indicate to which region an operation should be applied if CMDCTL.ADDRXLATEOVR is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REGIONSEL_A {
    #[doc = "8: Engr Region"]
    ENGR = 8,
    #[doc = "4: Trim Region"]
    TRIM = 4,
    #[doc = "2: Non-Main Region"]
    NONMAIN = 2,
    #[doc = "1: Main Region"]
    MAIN = 1,
}
impl From<REGIONSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REGIONSEL_A) -> Self {
        variant as _
    }
}
impl REGIONSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REGIONSEL_A> {
        match self.bits {
            8 => Some(REGIONSEL_A::ENGR),
            4 => Some(REGIONSEL_A::TRIM),
            2 => Some(REGIONSEL_A::NONMAIN),
            1 => Some(REGIONSEL_A::MAIN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENGR`"]
    #[inline(always)]
    pub fn is_engr(&self) -> bool {
        *self == REGIONSEL_A::ENGR
    }
    #[doc = "Checks if the value of the field is `TRIM`"]
    #[inline(always)]
    pub fn is_trim(&self) -> bool {
        *self == REGIONSEL_A::TRIM
    }
    #[doc = "Checks if the value of the field is `NONMAIN`"]
    #[inline(always)]
    pub fn is_nonmain(&self) -> bool {
        *self == REGIONSEL_A::NONMAIN
    }
    #[doc = "Checks if the value of the field is `MAIN`"]
    #[inline(always)]
    pub fn is_main(&self) -> bool {
        *self == REGIONSEL_A::MAIN
    }
}
#[doc = "Field `REGIONSEL` writer - 12:9\\]
Bank Region A specific region ID can be written to this field to indicate to which region an operation should be applied if CMDCTL.ADDRXLATEOVR is set."]
pub type REGIONSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CMDCTL_SPEC, u8, REGIONSEL_A, 4, O>;
impl<'a, const O: u8> REGIONSEL_W<'a, O> {
    #[doc = "Engr Region"]
    #[inline(always)]
    pub fn engr(self) -> &'a mut W {
        self.variant(REGIONSEL_A::ENGR)
    }
    #[doc = "Trim Region"]
    #[inline(always)]
    pub fn trim(self) -> &'a mut W {
        self.variant(REGIONSEL_A::TRIM)
    }
    #[doc = "Non-Main Region"]
    #[inline(always)]
    pub fn nonmain(self) -> &'a mut W {
        self.variant(REGIONSEL_A::NONMAIN)
    }
    #[doc = "Main Region"]
    #[inline(always)]
    pub fn main(self) -> &'a mut W {
        self.variant(REGIONSEL_A::MAIN)
    }
}
#[doc = "Field `RESERVED13` reader - 13:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED13_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED13` writer - 13:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED13_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDCTL_SPEC, bool, O>;
#[doc = "Field `PREVEREN` reader - 14:14\\]
Enable verify before program or erase. For program, bits already programmed to the requested value will be masked. For erase, sectors already erased will be masked."]
pub type PREVEREN_R = crate::BitReader<PREVEREN_A>;
#[doc = "14:14\\]
Enable verify before program or erase. For program, bits already programmed to the requested value will be masked. For erase, sectors already erased will be masked.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PREVEREN_A {
    #[doc = "1: Enable"]
    ENABLE = 1,
    #[doc = "0: Disable"]
    DISABLE = 0,
}
impl From<PREVEREN_A> for bool {
    #[inline(always)]
    fn from(variant: PREVEREN_A) -> Self {
        variant as u8 != 0
    }
}
impl PREVEREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PREVEREN_A {
        match self.bits {
            true => PREVEREN_A::ENABLE,
            false => PREVEREN_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PREVEREN_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PREVEREN_A::DISABLE
    }
}
#[doc = "Field `PREVEREN` writer - 14:14\\]
Enable verify before program or erase. For program, bits already programmed to the requested value will be masked. For erase, sectors already erased will be masked."]
pub type PREVEREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDCTL_SPEC, PREVEREN_A, O>;
impl<'a, const O: u8> PREVEREN_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PREVEREN_A::ENABLE)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PREVEREN_A::DISABLE)
    }
}
#[doc = "Field `POSTVEREN` reader - 15:15\\]
Enable verify after program or erase"]
pub type POSTVEREN_R = crate::BitReader<POSTVEREN_A>;
#[doc = "15:15\\]
Enable verify after program or erase\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POSTVEREN_A {
    #[doc = "1: Enable"]
    ENABLE = 1,
    #[doc = "0: Disable"]
    DISABLE = 0,
}
impl From<POSTVEREN_A> for bool {
    #[inline(always)]
    fn from(variant: POSTVEREN_A) -> Self {
        variant as u8 != 0
    }
}
impl POSTVEREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POSTVEREN_A {
        match self.bits {
            true => POSTVEREN_A::ENABLE,
            false => POSTVEREN_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == POSTVEREN_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == POSTVEREN_A::DISABLE
    }
}
#[doc = "Field `POSTVEREN` writer - 15:15\\]
Enable verify after program or erase"]
pub type POSTVEREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDCTL_SPEC, POSTVEREN_A, O>;
impl<'a, const O: u8> POSTVEREN_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(POSTVEREN_A::ENABLE)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(POSTVEREN_A::DISABLE)
    }
}
#[doc = "Field `ADDRXLATEOVR` reader - 16:16\\]
Override hardware address translation of address in CMDADDR from a system address to a bank address and bank ID. Use data written to CMDADDR directly as the bank address. Use the value written to CMDCTL.BANKSEL directly as the bank ID. Use the value written to CMDCTL.REGIONSEL directly as the region ID."]
pub type ADDRXLATEOVR_R = crate::BitReader<ADDRXLATEOVR_A>;
#[doc = "16:16\\]
Override hardware address translation of address in CMDADDR from a system address to a bank address and bank ID. Use data written to CMDADDR directly as the bank address. Use the value written to CMDCTL.BANKSEL directly as the bank ID. Use the value written to CMDCTL.REGIONSEL directly as the region ID.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDRXLATEOVR_A {
    #[doc = "1: Override"]
    OVERRIDE = 1,
    #[doc = "0: Do not override"]
    NOOVERRIDE = 0,
}
impl From<ADDRXLATEOVR_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRXLATEOVR_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDRXLATEOVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDRXLATEOVR_A {
        match self.bits {
            true => ADDRXLATEOVR_A::OVERRIDE,
            false => ADDRXLATEOVR_A::NOOVERRIDE,
        }
    }
    #[doc = "Checks if the value of the field is `OVERRIDE`"]
    #[inline(always)]
    pub fn is_override(&self) -> bool {
        *self == ADDRXLATEOVR_A::OVERRIDE
    }
    #[doc = "Checks if the value of the field is `NOOVERRIDE`"]
    #[inline(always)]
    pub fn is_nooverride(&self) -> bool {
        *self == ADDRXLATEOVR_A::NOOVERRIDE
    }
}
#[doc = "Field `ADDRXLATEOVR` writer - 16:16\\]
Override hardware address translation of address in CMDADDR from a system address to a bank address and bank ID. Use data written to CMDADDR directly as the bank address. Use the value written to CMDCTL.BANKSEL directly as the bank ID. Use the value written to CMDCTL.REGIONSEL directly as the region ID."]
pub type ADDRXLATEOVR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CMDCTL_SPEC, ADDRXLATEOVR_A, O>;
impl<'a, const O: u8> ADDRXLATEOVR_W<'a, O> {
    #[doc = "Override"]
    #[inline(always)]
    pub fn override_(self) -> &'a mut W {
        self.variant(ADDRXLATEOVR_A::OVERRIDE)
    }
    #[doc = "Do not override"]
    #[inline(always)]
    pub fn nooverride(self) -> &'a mut W {
        self.variant(ADDRXLATEOVR_A::NOOVERRIDE)
    }
}
#[doc = "Field `RESERVED17` reader - 17:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED17_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED17` writer - 17:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED17_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDCTL_SPEC, bool, O>;
#[doc = "Field `PROGMASKDIS` reader - 18:18\\]
Disable use of program mask for programming. Bit masking will not be used during program verify. If any bits fail the verify either before (prever) or after (postver) the operation, then all specified flash entries will receive subsequent program pulse."]
pub type PROGMASKDIS_R = crate::BitReader<PROGMASKDIS_A>;
#[doc = "18:18\\]
Disable use of program mask for programming. Bit masking will not be used during program verify. If any bits fail the verify either before (prever) or after (postver) the operation, then all specified flash entries will receive subsequent program pulse.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROGMASKDIS_A {
    #[doc = "1: Disable"]
    DISABLE = 1,
    #[doc = "0: Enable"]
    ENABLE = 0,
}
impl From<PROGMASKDIS_A> for bool {
    #[inline(always)]
    fn from(variant: PROGMASKDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl PROGMASKDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROGMASKDIS_A {
        match self.bits {
            true => PROGMASKDIS_A::DISABLE,
            false => PROGMASKDIS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PROGMASKDIS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PROGMASKDIS_A::ENABLE
    }
}
#[doc = "Field `PROGMASKDIS` writer - 18:18\\]
Disable use of program mask for programming. Bit masking will not be used during program verify. If any bits fail the verify either before (prever) or after (postver) the operation, then all specified flash entries will receive subsequent program pulse."]
pub type PROGMASKDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDCTL_SPEC, PROGMASKDIS_A, O>;
impl<'a, const O: u8> PROGMASKDIS_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PROGMASKDIS_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PROGMASKDIS_A::ENABLE)
    }
}
#[doc = "Field `ERASEMASKDIS` reader - 19:19\\]
Disable use of erase mask for erase Bit masking will not be used during erase verify. If any sectors fail the verify either before (prever) or after (postver) the operation, then all specified flash sectors will receive subsequent erase pulse."]
pub type ERASEMASKDIS_R = crate::BitReader<ERASEMASKDIS_A>;
#[doc = "19:19\\]
Disable use of erase mask for erase Bit masking will not be used during erase verify. If any sectors fail the verify either before (prever) or after (postver) the operation, then all specified flash sectors will receive subsequent erase pulse.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERASEMASKDIS_A {
    #[doc = "1: Disable"]
    DISABLE = 1,
    #[doc = "0: Enable"]
    ENABLE = 0,
}
impl From<ERASEMASKDIS_A> for bool {
    #[inline(always)]
    fn from(variant: ERASEMASKDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl ERASEMASKDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERASEMASKDIS_A {
        match self.bits {
            true => ERASEMASKDIS_A::DISABLE,
            false => ERASEMASKDIS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERASEMASKDIS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERASEMASKDIS_A::ENABLE
    }
}
#[doc = "Field `ERASEMASKDIS` writer - 19:19\\]
Disable use of erase mask for erase Bit masking will not be used during erase verify. If any sectors fail the verify either before (prever) or after (postver) the operation, then all specified flash sectors will receive subsequent erase pulse."]
pub type ERASEMASKDIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CMDCTL_SPEC, ERASEMASKDIS_A, O>;
impl<'a, const O: u8> ERASEMASKDIS_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERASEMASKDIS_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERASEMASKDIS_A::ENABLE)
    }
}
#[doc = "Field `SSERASEDIS` reader - 20:20\\]
Disable Stair-Step Erase. If set, the default VHV trim voltage setting will be used for all erase pulses. By default, this bit is reset, meaning that the VHV voltage will be stepped during successive erase pulses. The step count, step voltage, begin and end voltages are all hard-wired."]
pub type SSERASEDIS_R = crate::BitReader<SSERASEDIS_A>;
#[doc = "20:20\\]
Disable Stair-Step Erase. If set, the default VHV trim voltage setting will be used for all erase pulses. By default, this bit is reset, meaning that the VHV voltage will be stepped during successive erase pulses. The step count, step voltage, begin and end voltages are all hard-wired.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSERASEDIS_A {
    #[doc = "1: Disable"]
    DISABLE = 1,
    #[doc = "0: Enable"]
    ENABLE = 0,
}
impl From<SSERASEDIS_A> for bool {
    #[inline(always)]
    fn from(variant: SSERASEDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl SSERASEDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSERASEDIS_A {
        match self.bits {
            true => SSERASEDIS_A::DISABLE,
            false => SSERASEDIS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SSERASEDIS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SSERASEDIS_A::ENABLE
    }
}
#[doc = "Field `SSERASEDIS` writer - 20:20\\]
Disable Stair-Step Erase. If set, the default VHV trim voltage setting will be used for all erase pulses. By default, this bit is reset, meaning that the VHV voltage will be stepped during successive erase pulses. The step count, step voltage, begin and end voltages are all hard-wired."]
pub type SSERASEDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDCTL_SPEC, SSERASEDIS_A, O>;
impl<'a, const O: u8> SSERASEDIS_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SSERASEDIS_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SSERASEDIS_A::ENABLE)
    }
}
#[doc = "Field `DATAVEREN` reader - 21:21\\]
Enable invalid data verify. This checks for 0->1 transitions in the memory when a program operation is initiated. If such a transition is found, the program will fail with an error without doing any programming."]
pub type DATAVEREN_R = crate::BitReader<DATAVEREN_A>;
#[doc = "21:21\\]
Enable invalid data verify. This checks for 0->1 transitions in the memory when a program operation is initiated. If such a transition is found, the program will fail with an error without doing any programming.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATAVEREN_A {
    #[doc = "1: Enable"]
    ENABLE = 1,
    #[doc = "0: Disable"]
    DISABLE = 0,
}
impl From<DATAVEREN_A> for bool {
    #[inline(always)]
    fn from(variant: DATAVEREN_A) -> Self {
        variant as u8 != 0
    }
}
impl DATAVEREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATAVEREN_A {
        match self.bits {
            true => DATAVEREN_A::ENABLE,
            false => DATAVEREN_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DATAVEREN_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DATAVEREN_A::DISABLE
    }
}
#[doc = "Field `DATAVEREN` writer - 21:21\\]
Enable invalid data verify. This checks for 0->1 transitions in the memory when a program operation is initiated. If such a transition is found, the program will fail with an error without doing any programming."]
pub type DATAVEREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDCTL_SPEC, DATAVEREN_A, O>;
impl<'a, const O: u8> DATAVEREN_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DATAVEREN_A::ENABLE)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DATAVEREN_A::DISABLE)
    }
}
#[doc = "Field `RESERVED22` reader - 31:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED22_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED22` writer - 31:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED22_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMDCTL_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Mode This field is only used for the Mode Change command type. Otherwise, bank and pump modes are set automaticlly via the NW hardware."]
    #[inline(always)]
    pub fn modesel(&self) -> MODESEL_R {
        MODESEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - 4:8\\]
Bank Select A specific Bank ID can be written to this field to indicate to which bank an operation should be applied if CMDCTL.ADDRXLATEOVR is set."]
    #[inline(always)]
    pub fn banksel(&self) -> BANKSEL_R {
        BANKSEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 9:12 - 12:9\\]
Bank Region A specific region ID can be written to this field to indicate to which region an operation should be applied if CMDCTL.ADDRXLATEOVR is set."]
    #[inline(always)]
    pub fn regionsel(&self) -> REGIONSEL_R {
        REGIONSEL_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - 13:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved13(&self) -> RESERVED13_R {
        RESERVED13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Enable verify before program or erase. For program, bits already programmed to the requested value will be masked. For erase, sectors already erased will be masked."]
    #[inline(always)]
    pub fn preveren(&self) -> PREVEREN_R {
        PREVEREN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Enable verify after program or erase"]
    #[inline(always)]
    pub fn postveren(&self) -> POSTVEREN_R {
        POSTVEREN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Override hardware address translation of address in CMDADDR from a system address to a bank address and bank ID. Use data written to CMDADDR directly as the bank address. Use the value written to CMDCTL.BANKSEL directly as the bank ID. Use the value written to CMDCTL.REGIONSEL directly as the region ID."]
    #[inline(always)]
    pub fn addrxlateovr(&self) -> ADDRXLATEOVR_R {
        ADDRXLATEOVR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> RESERVED17_R {
        RESERVED17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Disable use of program mask for programming. Bit masking will not be used during program verify. If any bits fail the verify either before (prever) or after (postver) the operation, then all specified flash entries will receive subsequent program pulse."]
    #[inline(always)]
    pub fn progmaskdis(&self) -> PROGMASKDIS_R {
        PROGMASKDIS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Disable use of erase mask for erase Bit masking will not be used during erase verify. If any sectors fail the verify either before (prever) or after (postver) the operation, then all specified flash sectors will receive subsequent erase pulse."]
    #[inline(always)]
    pub fn erasemaskdis(&self) -> ERASEMASKDIS_R {
        ERASEMASKDIS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Disable Stair-Step Erase. If set, the default VHV trim voltage setting will be used for all erase pulses. By default, this bit is reset, meaning that the VHV voltage will be stepped during successive erase pulses. The step count, step voltage, begin and end voltages are all hard-wired."]
    #[inline(always)]
    pub fn sserasedis(&self) -> SSERASEDIS_R {
        SSERASEDIS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Enable invalid data verify. This checks for 0->1 transitions in the memory when a program operation is initiated. If such a transition is found, the program will fail with an error without doing any programming."]
    #[inline(always)]
    pub fn dataveren(&self) -> DATAVEREN_R {
        DATAVEREN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:31 - 31:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved22(&self) -> RESERVED22_R {
        RESERVED22_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Mode This field is only used for the Mode Change command type. Otherwise, bank and pump modes are set automaticlly via the NW hardware."]
    #[inline(always)]
    #[must_use]
    pub fn modesel(&mut self) -> MODESEL_W<0> {
        MODESEL_W::new(self)
    }
    #[doc = "Bits 8:12 - 4:8\\]
Bank Select A specific Bank ID can be written to this field to indicate to which bank an operation should be applied if CMDCTL.ADDRXLATEOVR is set."]
    #[inline(always)]
    #[must_use]
    pub fn banksel(&mut self) -> BANKSEL_W<8> {
        BANKSEL_W::new(self)
    }
    #[doc = "Bits 9:12 - 12:9\\]
Bank Region A specific region ID can be written to this field to indicate to which region an operation should be applied if CMDCTL.ADDRXLATEOVR is set."]
    #[inline(always)]
    #[must_use]
    pub fn regionsel(&mut self) -> REGIONSEL_W<9> {
        REGIONSEL_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved13(&mut self) -> RESERVED13_W<13> {
        RESERVED13_W::new(self)
    }
    #[doc = "Bit 14 - 14:14\\]
Enable verify before program or erase. For program, bits already programmed to the requested value will be masked. For erase, sectors already erased will be masked."]
    #[inline(always)]
    #[must_use]
    pub fn preveren(&mut self) -> PREVEREN_W<14> {
        PREVEREN_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
Enable verify after program or erase"]
    #[inline(always)]
    #[must_use]
    pub fn postveren(&mut self) -> POSTVEREN_W<15> {
        POSTVEREN_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
Override hardware address translation of address in CMDADDR from a system address to a bank address and bank ID. Use data written to CMDADDR directly as the bank address. Use the value written to CMDCTL.BANKSEL directly as the bank ID. Use the value written to CMDCTL.REGIONSEL directly as the region ID."]
    #[inline(always)]
    #[must_use]
    pub fn addrxlateovr(&mut self) -> ADDRXLATEOVR_W<16> {
        ADDRXLATEOVR_W::new(self)
    }
    #[doc = "Bit 17 - 17:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved17(&mut self) -> RESERVED17_W<17> {
        RESERVED17_W::new(self)
    }
    #[doc = "Bit 18 - 18:18\\]
Disable use of program mask for programming. Bit masking will not be used during program verify. If any bits fail the verify either before (prever) or after (postver) the operation, then all specified flash entries will receive subsequent program pulse."]
    #[inline(always)]
    #[must_use]
    pub fn progmaskdis(&mut self) -> PROGMASKDIS_W<18> {
        PROGMASKDIS_W::new(self)
    }
    #[doc = "Bit 19 - 19:19\\]
Disable use of erase mask for erase Bit masking will not be used during erase verify. If any sectors fail the verify either before (prever) or after (postver) the operation, then all specified flash sectors will receive subsequent erase pulse."]
    #[inline(always)]
    #[must_use]
    pub fn erasemaskdis(&mut self) -> ERASEMASKDIS_W<19> {
        ERASEMASKDIS_W::new(self)
    }
    #[doc = "Bit 20 - 20:20\\]
Disable Stair-Step Erase. If set, the default VHV trim voltage setting will be used for all erase pulses. By default, this bit is reset, meaning that the VHV voltage will be stepped during successive erase pulses. The step count, step voltage, begin and end voltages are all hard-wired."]
    #[inline(always)]
    #[must_use]
    pub fn sserasedis(&mut self) -> SSERASEDIS_W<20> {
        SSERASEDIS_W::new(self)
    }
    #[doc = "Bit 21 - 21:21\\]
Enable invalid data verify. This checks for 0->1 transitions in the memory when a program operation is initiated. If such a transition is found, the program will fail with an error without doing any programming."]
    #[inline(always)]
    #[must_use]
    pub fn dataveren(&mut self) -> DATAVEREN_W<21> {
        DATAVEREN_W::new(self)
    }
    #[doc = "Bits 22:31 - 31:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved22(&mut self) -> RESERVED22_W<22> {
        RESERVED22_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command Control Register This register configures specific capabilities of the state machine for related to the execution of a command. This register is blocked for writes after CMDEXEC is written to a 1 and prior to STATCMD.DONE being set by the hardware to indicate that command execution has completed.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdctl](index.html) module"]
pub struct CMDCTL_SPEC;
impl crate::RegisterSpec for CMDCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmdctl::R](R) reader structure"]
impl crate::Readable for CMDCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmdctl::W](W) writer structure"]
impl crate::Writable for CMDCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMDCTL to value 0xc000"]
impl crate::Resettable for CMDCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0xc000;
}
