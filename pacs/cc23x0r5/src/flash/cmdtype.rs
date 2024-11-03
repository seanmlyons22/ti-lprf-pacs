#[doc = "Register `CMDTYPE` reader"]
pub type R = crate::R<CmdtypeSpec>;
#[doc = "Register `CMDTYPE` writer"]
pub type W = crate::W<CmdtypeSpec>;
#[doc = "2:0\\]
Command type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Command {
    #[doc = "6: Blank Verify - Check whether a flash word is in the erased state. This command may only be used with CMDTYPE.SIZE = ONEWORD"]
    Blankverify = 6,
    #[doc = "5: Clear Status - Clear status bits in FW_SMSTAT only."]
    Clearstatus = 5,
    #[doc = "4: Mode Change - Perform a mode change only, no other operation."]
    Modechange = 4,
    #[doc = "3: Read Verify - Perform a standalone read verify operation."]
    Readverify = 3,
    #[doc = "2: Erase"]
    Erase = 2,
    #[doc = "1: Program"]
    Program = 1,
    #[doc = "0: No Operation"]
    Noop = 0,
}
impl From<Command> for u8 {
    #[inline(always)]
    fn from(variant: Command) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Command {
    type Ux = u8;
}
impl crate::IsEnum for Command {}
#[doc = "Field `COMMAND` reader - 2:0\\]
Command type"]
pub type CommandR = crate::FieldReader<Command>;
impl CommandR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Command> {
        match self.bits {
            6 => Some(Command::Blankverify),
            5 => Some(Command::Clearstatus),
            4 => Some(Command::Modechange),
            3 => Some(Command::Readverify),
            2 => Some(Command::Erase),
            1 => Some(Command::Program),
            0 => Some(Command::Noop),
            _ => None,
        }
    }
    #[doc = "Blank Verify - Check whether a flash word is in the erased state. This command may only be used with CMDTYPE.SIZE = ONEWORD"]
    #[inline(always)]
    pub fn is_blankverify(&self) -> bool {
        *self == Command::Blankverify
    }
    #[doc = "Clear Status - Clear status bits in FW_SMSTAT only."]
    #[inline(always)]
    pub fn is_clearstatus(&self) -> bool {
        *self == Command::Clearstatus
    }
    #[doc = "Mode Change - Perform a mode change only, no other operation."]
    #[inline(always)]
    pub fn is_modechange(&self) -> bool {
        *self == Command::Modechange
    }
    #[doc = "Read Verify - Perform a standalone read verify operation."]
    #[inline(always)]
    pub fn is_readverify(&self) -> bool {
        *self == Command::Readverify
    }
    #[doc = "Erase"]
    #[inline(always)]
    pub fn is_erase(&self) -> bool {
        *self == Command::Erase
    }
    #[doc = "Program"]
    #[inline(always)]
    pub fn is_program(&self) -> bool {
        *self == Command::Program
    }
    #[doc = "No Operation"]
    #[inline(always)]
    pub fn is_noop(&self) -> bool {
        *self == Command::Noop
    }
}
#[doc = "Field `COMMAND` writer - 2:0\\]
Command type"]
pub type CommandW<'a, REG> = crate::FieldWriter<'a, REG, 3, Command>;
impl<'a, REG> CommandW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Blank Verify - Check whether a flash word is in the erased state. This command may only be used with CMDTYPE.SIZE = ONEWORD"]
    #[inline(always)]
    pub fn blankverify(self) -> &'a mut crate::W<REG> {
        self.variant(Command::Blankverify)
    }
    #[doc = "Clear Status - Clear status bits in FW_SMSTAT only."]
    #[inline(always)]
    pub fn clearstatus(self) -> &'a mut crate::W<REG> {
        self.variant(Command::Clearstatus)
    }
    #[doc = "Mode Change - Perform a mode change only, no other operation."]
    #[inline(always)]
    pub fn modechange(self) -> &'a mut crate::W<REG> {
        self.variant(Command::Modechange)
    }
    #[doc = "Read Verify - Perform a standalone read verify operation."]
    #[inline(always)]
    pub fn readverify(self) -> &'a mut crate::W<REG> {
        self.variant(Command::Readverify)
    }
    #[doc = "Erase"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut crate::W<REG> {
        self.variant(Command::Erase)
    }
    #[doc = "Program"]
    #[inline(always)]
    pub fn program(self) -> &'a mut crate::W<REG> {
        self.variant(Command::Program)
    }
    #[doc = "No Operation"]
    #[inline(always)]
    pub fn noop(self) -> &'a mut crate::W<REG> {
        self.variant(Command::Noop)
    }
}
#[doc = "Field `RESERVED_3` reader - 3:3\\]
Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `RESERVED_3` writer - 3:3\\]
Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "6:4\\]
Command size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Size {
    #[doc = "5: Operate on an entire flash bank"]
    Bank = 5,
    #[doc = "4: Operate on a flash sector"]
    Sector = 4,
    #[doc = "3: Operate on 8 flash words"]
    Eightword = 3,
    #[doc = "2: Operate on 4 flash words"]
    Fourword = 2,
    #[doc = "1: Operate on 2 flash words"]
    Twoword = 1,
    #[doc = "0: Operate on 1 flash word"]
    Oneword = 0,
}
impl From<Size> for u8 {
    #[inline(always)]
    fn from(variant: Size) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Size {
    type Ux = u8;
}
impl crate::IsEnum for Size {}
#[doc = "Field `SIZE` reader - 6:4\\]
Command size"]
pub type SizeR = crate::FieldReader<Size>;
impl SizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Size> {
        match self.bits {
            5 => Some(Size::Bank),
            4 => Some(Size::Sector),
            3 => Some(Size::Eightword),
            2 => Some(Size::Fourword),
            1 => Some(Size::Twoword),
            0 => Some(Size::Oneword),
            _ => None,
        }
    }
    #[doc = "Operate on an entire flash bank"]
    #[inline(always)]
    pub fn is_bank(&self) -> bool {
        *self == Size::Bank
    }
    #[doc = "Operate on a flash sector"]
    #[inline(always)]
    pub fn is_sector(&self) -> bool {
        *self == Size::Sector
    }
    #[doc = "Operate on 8 flash words"]
    #[inline(always)]
    pub fn is_eightword(&self) -> bool {
        *self == Size::Eightword
    }
    #[doc = "Operate on 4 flash words"]
    #[inline(always)]
    pub fn is_fourword(&self) -> bool {
        *self == Size::Fourword
    }
    #[doc = "Operate on 2 flash words"]
    #[inline(always)]
    pub fn is_twoword(&self) -> bool {
        *self == Size::Twoword
    }
    #[doc = "Operate on 1 flash word"]
    #[inline(always)]
    pub fn is_oneword(&self) -> bool {
        *self == Size::Oneword
    }
}
#[doc = "Field `SIZE` writer - 6:4\\]
Command size"]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Size>;
impl<'a, REG> SizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Operate on an entire flash bank"]
    #[inline(always)]
    pub fn bank(self) -> &'a mut crate::W<REG> {
        self.variant(Size::Bank)
    }
    #[doc = "Operate on a flash sector"]
    #[inline(always)]
    pub fn sector(self) -> &'a mut crate::W<REG> {
        self.variant(Size::Sector)
    }
    #[doc = "Operate on 8 flash words"]
    #[inline(always)]
    pub fn eightword(self) -> &'a mut crate::W<REG> {
        self.variant(Size::Eightword)
    }
    #[doc = "Operate on 4 flash words"]
    #[inline(always)]
    pub fn fourword(self) -> &'a mut crate::W<REG> {
        self.variant(Size::Fourword)
    }
    #[doc = "Operate on 2 flash words"]
    #[inline(always)]
    pub fn twoword(self) -> &'a mut crate::W<REG> {
        self.variant(Size::Twoword)
    }
    #[doc = "Operate on 1 flash word"]
    #[inline(always)]
    pub fn oneword(self) -> &'a mut crate::W<REG> {
        self.variant(Size::Oneword)
    }
}
#[doc = "Field `RESERVED_31_7` reader - 31:7\\]
Reserved"]
pub type Reserved31_7R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED_31_7` writer - 31:7\\]
Reserved"]
pub type Reserved31_7W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Command type"]
    #[inline(always)]
    pub fn command(&self) -> CommandR {
        CommandR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
Reserved"]
    #[inline(always)]
    pub fn reserved_3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Command size"]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Reserved"]
    #[inline(always)]
    pub fn reserved_31_7(&self) -> Reserved31_7R {
        Reserved31_7R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Command type"]
    #[inline(always)]
    #[must_use]
    pub fn command(&mut self) -> CommandW<CmdtypeSpec> {
        CommandW::new(self, 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved_3(&mut self) -> Reserved3W<CmdtypeSpec> {
        Reserved3W::new(self, 3)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Command size"]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SizeW<CmdtypeSpec> {
        SizeW::new(self, 4)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved_31_7(&mut self) -> Reserved31_7W<CmdtypeSpec> {
        Reserved31_7W::new(self, 7)
    }
}
#[doc = "Command Type Register This register specifies the type of command to be executed by the flash wrapper hardware. This register is blocked for writes after CMDEXEC is written to a 1 and prior to STATCMD.DONE being set by the hardware to indicate that command execution has completed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdtype::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdtype::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdtypeSpec;
impl crate::RegisterSpec for CmdtypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdtype::R`](R) reader structure"]
impl crate::Readable for CmdtypeSpec {}
#[doc = "`write(|w| ..)` method takes [`cmdtype::W`](W) writer structure"]
impl crate::Writable for CmdtypeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMDTYPE to value 0"]
impl crate::Resettable for CmdtypeSpec {
    const RESET_VALUE: u32 = 0;
}
