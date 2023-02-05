#[doc = "Register `CMDTYPE` reader"]
pub struct R(crate::R<CMDTYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDTYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMDTYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMDTYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMDTYPE` writer"]
pub struct W(crate::W<CMDTYPE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMDTYPE_SPEC>;
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
impl From<crate::W<CMDTYPE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMDTYPE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMAND` reader - 2:0\\]
Command type"]
pub type COMMAND_R = crate::FieldReader<u8, COMMAND_A>;
#[doc = "2:0\\]
Command type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMMAND_A {
    #[doc = "6: Blank Verify - Check whether a flash word is in the erased state. This command may only be used with CMDTYPE.SIZE = ONEWORD"]
    BLANKVERIFY = 6,
    #[doc = "5: Clear Status - Clear status bits in FW_SMSTAT only."]
    CLEARSTATUS = 5,
    #[doc = "4: Mode Change - Perform a mode change only, no other operation."]
    MODECHANGE = 4,
    #[doc = "3: Read Verify - Perform a standalone read verify operation."]
    READVERIFY = 3,
    #[doc = "2: Erase"]
    ERASE = 2,
    #[doc = "1: Program"]
    PROGRAM = 1,
    #[doc = "0: No Operation"]
    NOOP = 0,
}
impl From<COMMAND_A> for u8 {
    #[inline(always)]
    fn from(variant: COMMAND_A) -> Self {
        variant as _
    }
}
impl COMMAND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMMAND_A> {
        match self.bits {
            6 => Some(COMMAND_A::BLANKVERIFY),
            5 => Some(COMMAND_A::CLEARSTATUS),
            4 => Some(COMMAND_A::MODECHANGE),
            3 => Some(COMMAND_A::READVERIFY),
            2 => Some(COMMAND_A::ERASE),
            1 => Some(COMMAND_A::PROGRAM),
            0 => Some(COMMAND_A::NOOP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BLANKVERIFY`"]
    #[inline(always)]
    pub fn is_blankverify(&self) -> bool {
        *self == COMMAND_A::BLANKVERIFY
    }
    #[doc = "Checks if the value of the field is `CLEARSTATUS`"]
    #[inline(always)]
    pub fn is_clearstatus(&self) -> bool {
        *self == COMMAND_A::CLEARSTATUS
    }
    #[doc = "Checks if the value of the field is `MODECHANGE`"]
    #[inline(always)]
    pub fn is_modechange(&self) -> bool {
        *self == COMMAND_A::MODECHANGE
    }
    #[doc = "Checks if the value of the field is `READVERIFY`"]
    #[inline(always)]
    pub fn is_readverify(&self) -> bool {
        *self == COMMAND_A::READVERIFY
    }
    #[doc = "Checks if the value of the field is `ERASE`"]
    #[inline(always)]
    pub fn is_erase(&self) -> bool {
        *self == COMMAND_A::ERASE
    }
    #[doc = "Checks if the value of the field is `PROGRAM`"]
    #[inline(always)]
    pub fn is_program(&self) -> bool {
        *self == COMMAND_A::PROGRAM
    }
    #[doc = "Checks if the value of the field is `NOOP`"]
    #[inline(always)]
    pub fn is_noop(&self) -> bool {
        *self == COMMAND_A::NOOP
    }
}
#[doc = "Field `COMMAND` writer - 2:0\\]
Command type"]
pub type COMMAND_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CMDTYPE_SPEC, u8, COMMAND_A, 3, O>;
impl<'a, const O: u8> COMMAND_W<'a, O> {
    #[doc = "Blank Verify - Check whether a flash word is in the erased state. This command may only be used with CMDTYPE.SIZE = ONEWORD"]
    #[inline(always)]
    pub fn blankverify(self) -> &'a mut W {
        self.variant(COMMAND_A::BLANKVERIFY)
    }
    #[doc = "Clear Status - Clear status bits in FW_SMSTAT only."]
    #[inline(always)]
    pub fn clearstatus(self) -> &'a mut W {
        self.variant(COMMAND_A::CLEARSTATUS)
    }
    #[doc = "Mode Change - Perform a mode change only, no other operation."]
    #[inline(always)]
    pub fn modechange(self) -> &'a mut W {
        self.variant(COMMAND_A::MODECHANGE)
    }
    #[doc = "Read Verify - Perform a standalone read verify operation."]
    #[inline(always)]
    pub fn readverify(self) -> &'a mut W {
        self.variant(COMMAND_A::READVERIFY)
    }
    #[doc = "Erase"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut W {
        self.variant(COMMAND_A::ERASE)
    }
    #[doc = "Program"]
    #[inline(always)]
    pub fn program(self) -> &'a mut W {
        self.variant(COMMAND_A::PROGRAM)
    }
    #[doc = "No Operation"]
    #[inline(always)]
    pub fn noop(self) -> &'a mut W {
        self.variant(COMMAND_A::NOOP)
    }
}
#[doc = "Field `RESERVED3` reader - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED3` writer - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDTYPE_SPEC, bool, O>;
#[doc = "Field `SIZE` reader - 6:4\\]
Command size"]
pub type SIZE_R = crate::FieldReader<u8, SIZE_A>;
#[doc = "6:4\\]
Command size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SIZE_A {
    #[doc = "5: Operate on an entire flash bank"]
    BANK = 5,
    #[doc = "4: Operate on a flash sector"]
    SECTOR = 4,
    #[doc = "3: Operate on 8 flash words"]
    EIGHTWORD = 3,
    #[doc = "2: Operate on 4 flash words"]
    FOURWORD = 2,
    #[doc = "1: Operate on 2 flash words"]
    TWOWORD = 1,
    #[doc = "0: Operate on 1 flash word"]
    ONEWORD = 0,
}
impl From<SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SIZE_A) -> Self {
        variant as _
    }
}
impl SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SIZE_A> {
        match self.bits {
            5 => Some(SIZE_A::BANK),
            4 => Some(SIZE_A::SECTOR),
            3 => Some(SIZE_A::EIGHTWORD),
            2 => Some(SIZE_A::FOURWORD),
            1 => Some(SIZE_A::TWOWORD),
            0 => Some(SIZE_A::ONEWORD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BANK`"]
    #[inline(always)]
    pub fn is_bank(&self) -> bool {
        *self == SIZE_A::BANK
    }
    #[doc = "Checks if the value of the field is `SECTOR`"]
    #[inline(always)]
    pub fn is_sector(&self) -> bool {
        *self == SIZE_A::SECTOR
    }
    #[doc = "Checks if the value of the field is `EIGHTWORD`"]
    #[inline(always)]
    pub fn is_eightword(&self) -> bool {
        *self == SIZE_A::EIGHTWORD
    }
    #[doc = "Checks if the value of the field is `FOURWORD`"]
    #[inline(always)]
    pub fn is_fourword(&self) -> bool {
        *self == SIZE_A::FOURWORD
    }
    #[doc = "Checks if the value of the field is `TWOWORD`"]
    #[inline(always)]
    pub fn is_twoword(&self) -> bool {
        *self == SIZE_A::TWOWORD
    }
    #[doc = "Checks if the value of the field is `ONEWORD`"]
    #[inline(always)]
    pub fn is_oneword(&self) -> bool {
        *self == SIZE_A::ONEWORD
    }
}
#[doc = "Field `SIZE` writer - 6:4\\]
Command size"]
pub type SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMDTYPE_SPEC, u8, SIZE_A, 3, O>;
impl<'a, const O: u8> SIZE_W<'a, O> {
    #[doc = "Operate on an entire flash bank"]
    #[inline(always)]
    pub fn bank(self) -> &'a mut W {
        self.variant(SIZE_A::BANK)
    }
    #[doc = "Operate on a flash sector"]
    #[inline(always)]
    pub fn sector(self) -> &'a mut W {
        self.variant(SIZE_A::SECTOR)
    }
    #[doc = "Operate on 8 flash words"]
    #[inline(always)]
    pub fn eightword(self) -> &'a mut W {
        self.variant(SIZE_A::EIGHTWORD)
    }
    #[doc = "Operate on 4 flash words"]
    #[inline(always)]
    pub fn fourword(self) -> &'a mut W {
        self.variant(SIZE_A::FOURWORD)
    }
    #[doc = "Operate on 2 flash words"]
    #[inline(always)]
    pub fn twoword(self) -> &'a mut W {
        self.variant(SIZE_A::TWOWORD)
    }
    #[doc = "Operate on 1 flash word"]
    #[inline(always)]
    pub fn oneword(self) -> &'a mut W {
        self.variant(SIZE_A::ONEWORD)
    }
}
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Command type"]
    #[inline(always)]
    pub fn command(&self) -> COMMAND_R {
        COMMAND_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Command size"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Command type"]
    #[inline(always)]
    #[must_use]
    pub fn command(&mut self) -> COMMAND_W<0> {
        COMMAND_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<3> {
        RESERVED3_W::new(self)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Command size"]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SIZE_W<4> {
        SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command Type Register This register specifies the type of command to be executed by the NoWrapper hardware. This register is blocked for writes after CMDEXEC is written to a 1 and prior to STATCMD.DONE being set by the hardware to indicate that command execution has completed.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdtype](index.html) module"]
pub struct CMDTYPE_SPEC;
impl crate::RegisterSpec for CMDTYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmdtype::R](R) reader structure"]
impl crate::Readable for CMDTYPE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmdtype::W](W) writer structure"]
impl crate::Writable for CMDTYPE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMDTYPE to value 0"]
impl crate::Resettable for CMDTYPE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
