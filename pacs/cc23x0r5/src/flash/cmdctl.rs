#[doc = "Register `CMDCTL` reader"]
pub type R = crate::R<CmdctlSpec>;
#[doc = "Register `CMDCTL` writer"]
pub type W = crate::W<CmdctlSpec>;
#[doc = "3:0\\]
Mode This field is only used for the Mode Change command type. Otherwise, bank and pump modes are set automaticlly via the NW hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Modesel {
    #[doc = "15: Erase Bank"]
    Erasebnk = 15,
    #[doc = "14: Program Multiple Word"]
    Pgmmw = 14,
    #[doc = "12: Erase Sector"]
    Erasesect = 12,
    #[doc = "11: Erase Verify Mode"]
    Erasever = 11,
    #[doc = "10: Program Single Word"]
    Pgmsw = 10,
    #[doc = "9: Program Verify Mode"]
    Pgmver = 9,
    #[doc = "7: Read Margin 1B Mode"]
    Rdmarg1b = 7,
    #[doc = "6: Read Margin 0B Mode"]
    Rdmarg0b = 6,
    #[doc = "4: Read Margin 1 Mode"]
    Rdmarg1 = 4,
    #[doc = "2: Read Margin 0 Mode"]
    Rdmarg0 = 2,
    #[doc = "0: Read Mode"]
    Read = 0,
}
impl From<Modesel> for u8 {
    #[inline(always)]
    fn from(variant: Modesel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Modesel {
    type Ux = u8;
}
impl crate::IsEnum for Modesel {}
#[doc = "Field `MODESEL` reader - 3:0\\]
Mode This field is only used for the Mode Change command type. Otherwise, bank and pump modes are set automaticlly via the NW hardware."]
pub type ModeselR = crate::FieldReader<Modesel>;
impl ModeselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Modesel> {
        match self.bits {
            15 => Some(Modesel::Erasebnk),
            14 => Some(Modesel::Pgmmw),
            12 => Some(Modesel::Erasesect),
            11 => Some(Modesel::Erasever),
            10 => Some(Modesel::Pgmsw),
            9 => Some(Modesel::Pgmver),
            7 => Some(Modesel::Rdmarg1b),
            6 => Some(Modesel::Rdmarg0b),
            4 => Some(Modesel::Rdmarg1),
            2 => Some(Modesel::Rdmarg0),
            0 => Some(Modesel::Read),
            _ => None,
        }
    }
    #[doc = "Erase Bank"]
    #[inline(always)]
    pub fn is_erasebnk(&self) -> bool {
        *self == Modesel::Erasebnk
    }
    #[doc = "Program Multiple Word"]
    #[inline(always)]
    pub fn is_pgmmw(&self) -> bool {
        *self == Modesel::Pgmmw
    }
    #[doc = "Erase Sector"]
    #[inline(always)]
    pub fn is_erasesect(&self) -> bool {
        *self == Modesel::Erasesect
    }
    #[doc = "Erase Verify Mode"]
    #[inline(always)]
    pub fn is_erasever(&self) -> bool {
        *self == Modesel::Erasever
    }
    #[doc = "Program Single Word"]
    #[inline(always)]
    pub fn is_pgmsw(&self) -> bool {
        *self == Modesel::Pgmsw
    }
    #[doc = "Program Verify Mode"]
    #[inline(always)]
    pub fn is_pgmver(&self) -> bool {
        *self == Modesel::Pgmver
    }
    #[doc = "Read Margin 1B Mode"]
    #[inline(always)]
    pub fn is_rdmarg1b(&self) -> bool {
        *self == Modesel::Rdmarg1b
    }
    #[doc = "Read Margin 0B Mode"]
    #[inline(always)]
    pub fn is_rdmarg0b(&self) -> bool {
        *self == Modesel::Rdmarg0b
    }
    #[doc = "Read Margin 1 Mode"]
    #[inline(always)]
    pub fn is_rdmarg1(&self) -> bool {
        *self == Modesel::Rdmarg1
    }
    #[doc = "Read Margin 0 Mode"]
    #[inline(always)]
    pub fn is_rdmarg0(&self) -> bool {
        *self == Modesel::Rdmarg0
    }
    #[doc = "Read Mode"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == Modesel::Read
    }
}
#[doc = "Field `MODESEL` writer - 3:0\\]
Mode This field is only used for the Mode Change command type. Otherwise, bank and pump modes are set automaticlly via the NW hardware."]
pub type ModeselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Modesel>;
impl<'a, REG> ModeselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Erase Bank"]
    #[inline(always)]
    pub fn erasebnk(self) -> &'a mut crate::W<REG> {
        self.variant(Modesel::Erasebnk)
    }
    #[doc = "Program Multiple Word"]
    #[inline(always)]
    pub fn pgmmw(self) -> &'a mut crate::W<REG> {
        self.variant(Modesel::Pgmmw)
    }
    #[doc = "Erase Sector"]
    #[inline(always)]
    pub fn erasesect(self) -> &'a mut crate::W<REG> {
        self.variant(Modesel::Erasesect)
    }
    #[doc = "Erase Verify Mode"]
    #[inline(always)]
    pub fn erasever(self) -> &'a mut crate::W<REG> {
        self.variant(Modesel::Erasever)
    }
    #[doc = "Program Single Word"]
    #[inline(always)]
    pub fn pgmsw(self) -> &'a mut crate::W<REG> {
        self.variant(Modesel::Pgmsw)
    }
    #[doc = "Program Verify Mode"]
    #[inline(always)]
    pub fn pgmver(self) -> &'a mut crate::W<REG> {
        self.variant(Modesel::Pgmver)
    }
    #[doc = "Read Margin 1B Mode"]
    #[inline(always)]
    pub fn rdmarg1b(self) -> &'a mut crate::W<REG> {
        self.variant(Modesel::Rdmarg1b)
    }
    #[doc = "Read Margin 0B Mode"]
    #[inline(always)]
    pub fn rdmarg0b(self) -> &'a mut crate::W<REG> {
        self.variant(Modesel::Rdmarg0b)
    }
    #[doc = "Read Margin 1 Mode"]
    #[inline(always)]
    pub fn rdmarg1(self) -> &'a mut crate::W<REG> {
        self.variant(Modesel::Rdmarg1)
    }
    #[doc = "Read Margin 0 Mode"]
    #[inline(always)]
    pub fn rdmarg0(self) -> &'a mut crate::W<REG> {
        self.variant(Modesel::Rdmarg0)
    }
    #[doc = "Read Mode"]
    #[inline(always)]
    pub fn read(self) -> &'a mut crate::W<REG> {
        self.variant(Modesel::Read)
    }
}
#[doc = "Field `RESERVED4` reader - 8:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader;
#[doc = "12:9\\]
Bank Region A specific region ID can be written to this field to indicate to which region an operation should be applied if CMDCTL.ADDRXLATEOVR is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Regionsel {
    #[doc = "8: Engr Region"]
    Engr = 8,
    #[doc = "4: Trim Region"]
    Trim = 4,
    #[doc = "2: Non-Main Region"]
    Nonmain = 2,
    #[doc = "1: Main Region"]
    Main = 1,
}
impl From<Regionsel> for u8 {
    #[inline(always)]
    fn from(variant: Regionsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Regionsel {
    type Ux = u8;
}
impl crate::IsEnum for Regionsel {}
#[doc = "Field `REGIONSEL` reader - 12:9\\]
Bank Region A specific region ID can be written to this field to indicate to which region an operation should be applied if CMDCTL.ADDRXLATEOVR is set."]
pub type RegionselR = crate::FieldReader<Regionsel>;
impl RegionselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Regionsel> {
        match self.bits {
            8 => Some(Regionsel::Engr),
            4 => Some(Regionsel::Trim),
            2 => Some(Regionsel::Nonmain),
            1 => Some(Regionsel::Main),
            _ => None,
        }
    }
    #[doc = "Engr Region"]
    #[inline(always)]
    pub fn is_engr(&self) -> bool {
        *self == Regionsel::Engr
    }
    #[doc = "Trim Region"]
    #[inline(always)]
    pub fn is_trim(&self) -> bool {
        *self == Regionsel::Trim
    }
    #[doc = "Non-Main Region"]
    #[inline(always)]
    pub fn is_nonmain(&self) -> bool {
        *self == Regionsel::Nonmain
    }
    #[doc = "Main Region"]
    #[inline(always)]
    pub fn is_main(&self) -> bool {
        *self == Regionsel::Main
    }
}
#[doc = "Field `REGIONSEL` writer - 12:9\\]
Bank Region A specific region ID can be written to this field to indicate to which region an operation should be applied if CMDCTL.ADDRXLATEOVR is set."]
pub type RegionselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Regionsel>;
impl<'a, REG> RegionselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Engr Region"]
    #[inline(always)]
    pub fn engr(self) -> &'a mut crate::W<REG> {
        self.variant(Regionsel::Engr)
    }
    #[doc = "Trim Region"]
    #[inline(always)]
    pub fn trim(self) -> &'a mut crate::W<REG> {
        self.variant(Regionsel::Trim)
    }
    #[doc = "Non-Main Region"]
    #[inline(always)]
    pub fn nonmain(self) -> &'a mut crate::W<REG> {
        self.variant(Regionsel::Nonmain)
    }
    #[doc = "Main Region"]
    #[inline(always)]
    pub fn main(self) -> &'a mut crate::W<REG> {
        self.variant(Regionsel::Main)
    }
}
#[doc = "Field `RESERVED_13` reader - 13:13\\]
Reserved"]
pub type Reserved13R = crate::BitReader;
#[doc = "Field `RESERVED_13` writer - 13:13\\]
Reserved"]
pub type Reserved13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED14` reader - 15:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved14R = crate::FieldReader;
#[doc = "16:16\\]
Override hardware address translation of address in CMDADDR from a system address to a bank address and bank ID. Use data written to CMDADDR directly as the bank address. Use the value written to CMDCTL.BANKSEL directly as the bank ID. Use the value written to CMDCTL.REGIONSEL directly as the region ID.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Addrxlateovr {
    #[doc = "1: Override"]
    Override = 1,
    #[doc = "0: Do not override"]
    Nooverride = 0,
}
impl From<Addrxlateovr> for bool {
    #[inline(always)]
    fn from(variant: Addrxlateovr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRXLATEOVR` reader - 16:16\\]
Override hardware address translation of address in CMDADDR from a system address to a bank address and bank ID. Use data written to CMDADDR directly as the bank address. Use the value written to CMDCTL.BANKSEL directly as the bank ID. Use the value written to CMDCTL.REGIONSEL directly as the region ID."]
pub type AddrxlateovrR = crate::BitReader<Addrxlateovr>;
impl AddrxlateovrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Addrxlateovr {
        match self.bits {
            true => Addrxlateovr::Override,
            false => Addrxlateovr::Nooverride,
        }
    }
    #[doc = "Override"]
    #[inline(always)]
    pub fn is_override(&self) -> bool {
        *self == Addrxlateovr::Override
    }
    #[doc = "Do not override"]
    #[inline(always)]
    pub fn is_nooverride(&self) -> bool {
        *self == Addrxlateovr::Nooverride
    }
}
#[doc = "Field `ADDRXLATEOVR` writer - 16:16\\]
Override hardware address translation of address in CMDADDR from a system address to a bank address and bank ID. Use data written to CMDADDR directly as the bank address. Use the value written to CMDCTL.BANKSEL directly as the bank ID. Use the value written to CMDCTL.REGIONSEL directly as the region ID."]
pub type AddrxlateovrW<'a, REG> = crate::BitWriter<'a, REG, Addrxlateovr>;
impl<'a, REG> AddrxlateovrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Override"]
    #[inline(always)]
    pub fn override_(self) -> &'a mut crate::W<REG> {
        self.variant(Addrxlateovr::Override)
    }
    #[doc = "Do not override"]
    #[inline(always)]
    pub fn nooverride(self) -> &'a mut crate::W<REG> {
        self.variant(Addrxlateovr::Nooverride)
    }
}
#[doc = "Field `RESERVED17` reader - 19:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved17R = crate::FieldReader;
#[doc = "20:20\\]
Disable Stair-Step Erase. If set, the default VHV trim voltage setting will be used for all erase pulses. By default, this bit is reset, meaning that the VHV voltage will be stepped during successive erase pulses. The step count, step voltage, begin and end voltages are all hard-wired.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sserasedis {
    #[doc = "1: Disable"]
    Disable = 1,
    #[doc = "0: Enable"]
    Enable = 0,
}
impl From<Sserasedis> for bool {
    #[inline(always)]
    fn from(variant: Sserasedis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSERASEDIS` reader - 20:20\\]
Disable Stair-Step Erase. If set, the default VHV trim voltage setting will be used for all erase pulses. By default, this bit is reset, meaning that the VHV voltage will be stepped during successive erase pulses. The step count, step voltage, begin and end voltages are all hard-wired."]
pub type SserasedisR = crate::BitReader<Sserasedis>;
impl SserasedisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sserasedis {
        match self.bits {
            true => Sserasedis::Disable,
            false => Sserasedis::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Sserasedis::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Sserasedis::Enable
    }
}
#[doc = "Field `SSERASEDIS` writer - 20:20\\]
Disable Stair-Step Erase. If set, the default VHV trim voltage setting will be used for all erase pulses. By default, this bit is reset, meaning that the VHV voltage will be stepped during successive erase pulses. The step count, step voltage, begin and end voltages are all hard-wired."]
pub type SserasedisW<'a, REG> = crate::BitWriter<'a, REG, Sserasedis>;
impl<'a, REG> SserasedisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Sserasedis::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Sserasedis::Enable)
    }
}
#[doc = "21:21\\]
Enable invalid data verify. This checks for 0-$gt;1 transitions in the memory when a program operation is initiated. If such a transition is found, the program will fail with an error without doing any programming.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dataveren {
    #[doc = "1: Enable"]
    Enable = 1,
    #[doc = "0: Disable"]
    Disable = 0,
}
impl From<Dataveren> for bool {
    #[inline(always)]
    fn from(variant: Dataveren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATAVEREN` reader - 21:21\\]
Enable invalid data verify. This checks for 0-$gt;1 transitions in the memory when a program operation is initiated. If such a transition is found, the program will fail with an error without doing any programming."]
pub type DataverenR = crate::BitReader<Dataveren>;
impl DataverenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dataveren {
        match self.bits {
            true => Dataveren::Enable,
            false => Dataveren::Disable,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dataveren::Enable
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dataveren::Disable
    }
}
#[doc = "Field `DATAVEREN` writer - 21:21\\]
Enable invalid data verify. This checks for 0-$gt;1 transitions in the memory when a program operation is initiated. If such a transition is found, the program will fail with an error without doing any programming."]
pub type DataverenW<'a, REG> = crate::BitWriter<'a, REG, Dataveren>;
impl<'a, REG> DataverenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dataveren::Enable)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dataveren::Disable)
    }
}
#[doc = "Field `RESERVED_31_22` reader - 31:22\\]
Reserved"]
pub type Reserved31_22R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED_31_22` writer - 31:22\\]
Reserved"]
pub type Reserved31_22W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Mode This field is only used for the Mode Change command type. Otherwise, bank and pump modes are set automaticlly via the NW hardware."]
    #[inline(always)]
    pub fn modesel(&self) -> ModeselR {
        ModeselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:8 - 8:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 9:12 - 12:9\\]
Bank Region A specific region ID can be written to this field to indicate to which region an operation should be applied if CMDCTL.ADDRXLATEOVR is set."]
    #[inline(always)]
    pub fn regionsel(&self) -> RegionselR {
        RegionselR::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - 13:13\\]
Reserved"]
    #[inline(always)]
    pub fn reserved_13(&self) -> Reserved13R {
        Reserved13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&self) -> Reserved14R {
        Reserved14R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Override hardware address translation of address in CMDADDR from a system address to a bank address and bank ID. Use data written to CMDADDR directly as the bank address. Use the value written to CMDCTL.BANKSEL directly as the bank ID. Use the value written to CMDCTL.REGIONSEL directly as the region ID."]
    #[inline(always)]
    pub fn addrxlateovr(&self) -> AddrxlateovrR {
        AddrxlateovrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - 19:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> Reserved17R {
        Reserved17R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - 20:20\\]
Disable Stair-Step Erase. If set, the default VHV trim voltage setting will be used for all erase pulses. By default, this bit is reset, meaning that the VHV voltage will be stepped during successive erase pulses. The step count, step voltage, begin and end voltages are all hard-wired."]
    #[inline(always)]
    pub fn sserasedis(&self) -> SserasedisR {
        SserasedisR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Enable invalid data verify. This checks for 0-$gt;1 transitions in the memory when a program operation is initiated. If such a transition is found, the program will fail with an error without doing any programming."]
    #[inline(always)]
    pub fn dataveren(&self) -> DataverenR {
        DataverenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:31 - 31:22\\]
Reserved"]
    #[inline(always)]
    pub fn reserved_31_22(&self) -> Reserved31_22R {
        Reserved31_22R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Mode This field is only used for the Mode Change command type. Otherwise, bank and pump modes are set automaticlly via the NW hardware."]
    #[inline(always)]
    #[must_use]
    pub fn modesel(&mut self) -> ModeselW<CmdctlSpec> {
        ModeselW::new(self, 0)
    }
    #[doc = "Bits 9:12 - 12:9\\]
Bank Region A specific region ID can be written to this field to indicate to which region an operation should be applied if CMDCTL.ADDRXLATEOVR is set."]
    #[inline(always)]
    #[must_use]
    pub fn regionsel(&mut self) -> RegionselW<CmdctlSpec> {
        RegionselW::new(self, 9)
    }
    #[doc = "Bit 13 - 13:13\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved_13(&mut self) -> Reserved13W<CmdctlSpec> {
        Reserved13W::new(self, 13)
    }
    #[doc = "Bit 16 - 16:16\\]
Override hardware address translation of address in CMDADDR from a system address to a bank address and bank ID. Use data written to CMDADDR directly as the bank address. Use the value written to CMDCTL.BANKSEL directly as the bank ID. Use the value written to CMDCTL.REGIONSEL directly as the region ID."]
    #[inline(always)]
    #[must_use]
    pub fn addrxlateovr(&mut self) -> AddrxlateovrW<CmdctlSpec> {
        AddrxlateovrW::new(self, 16)
    }
    #[doc = "Bit 20 - 20:20\\]
Disable Stair-Step Erase. If set, the default VHV trim voltage setting will be used for all erase pulses. By default, this bit is reset, meaning that the VHV voltage will be stepped during successive erase pulses. The step count, step voltage, begin and end voltages are all hard-wired."]
    #[inline(always)]
    #[must_use]
    pub fn sserasedis(&mut self) -> SserasedisW<CmdctlSpec> {
        SserasedisW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Enable invalid data verify. This checks for 0-$gt;1 transitions in the memory when a program operation is initiated. If such a transition is found, the program will fail with an error without doing any programming."]
    #[inline(always)]
    #[must_use]
    pub fn dataveren(&mut self) -> DataverenW<CmdctlSpec> {
        DataverenW::new(self, 21)
    }
    #[doc = "Bits 22:31 - 31:22\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved_31_22(&mut self) -> Reserved31_22W<CmdctlSpec> {
        Reserved31_22W::new(self, 22)
    }
}
#[doc = "Command Control Register This register configures specific capabilities of the state machine for related to the execution of a command. This register is blocked for writes after CMDEXEC is written to a 1 and prior to STATCMD.DONE being set by the hardware to indicate that command execution has completed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdctlSpec;
impl crate::RegisterSpec for CmdctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdctl::R`](R) reader structure"]
impl crate::Readable for CmdctlSpec {}
#[doc = "`write(|w| ..)` method takes [`cmdctl::W`](W) writer structure"]
impl crate::Writable for CmdctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMDCTL to value 0"]
impl crate::Resettable for CmdctlSpec {
    const RESET_VALUE: u32 = 0;
}
