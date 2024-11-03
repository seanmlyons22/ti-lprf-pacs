#[doc = "Register `CMDADDR` reader"]
pub type R = crate::R<CmdaddrSpec>;
#[doc = "Register `CMDADDR` writer"]
pub type W = crate::W<CmdaddrSpec>;
#[doc = "31:0\\]
Address value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Val {
    #[doc = "4294967295: Maximum value of VAL"]
    Maximum = 4294967295,
    #[doc = "0: Minimum value of VAL"]
    Minimum = 0,
}
impl From<Val> for u32 {
    #[inline(always)]
    fn from(variant: Val) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Val {
    type Ux = u32;
}
impl crate::IsEnum for Val {}
#[doc = "Field `VAL` reader - 31:0\\]
Address value"]
pub type ValR = crate::FieldReader<Val>;
impl ValR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Val> {
        match self.bits {
            4294967295 => Some(Val::Maximum),
            0 => Some(Val::Minimum),
            _ => None,
        }
    }
    #[doc = "Maximum value of VAL"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == Val::Maximum
    }
    #[doc = "Minimum value of VAL"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == Val::Minimum
    }
}
#[doc = "Field `VAL` writer - 31:0\\]
Address value"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, Val>;
impl<'a, REG> ValW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Maximum value of VAL"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Maximum)
    }
    #[doc = "Minimum value of VAL"]
    #[inline(always)]
    pub fn minimum(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Minimum)
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Address value"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Address value"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<CmdaddrSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Command Address Register: This register forms the target address of a command. The use cases are as follows: 1) For single-word program, this address indicates the flash bank word to be programmed. 2) For multi-word program, this address indicates the first flash bank address for the program. The address will be incremented for further words. 3) For sector erase, this address indicates the sector to be erased. 4) For bank erase, the address indicates the bank to be erased. 5) For read verify, the address indications follow program/erase listed above. Note the address written to this register will be submitted for translation to the flash wrapper address translation interface, and the translated address will be used to access the bank. However, if the CMDCTL.ADDRXLATEOVR bit is set, then the address written to this register will be used directly as the bank address. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the flash wrapper hardware.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdaddrSpec;
impl crate::RegisterSpec for CmdaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdaddr::R`](R) reader structure"]
impl crate::Readable for CmdaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`cmdaddr::W`](W) writer structure"]
impl crate::Writable for CmdaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMDADDR to value 0"]
impl crate::Resettable for CmdaddrSpec {
    const RESET_VALUE: u32 = 0;
}
