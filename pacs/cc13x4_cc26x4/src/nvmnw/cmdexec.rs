#[doc = "Register `CMDEXEC` reader"]
pub type R = crate::R<CmdexecSpec>;
#[doc = "Register `CMDEXEC` writer"]
pub type W = crate::W<CmdexecSpec>;
#[doc = "0:0\\]
Command Execute value Initiates execution of the command specified in the CMDTYPE register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Val {
    #[doc = "1: Command will execute or is executing in NoWrapper"]
    Execute = 1,
    #[doc = "0: Command will not execute or is not executing in NoWrapper"]
    Noexecute = 0,
}
impl From<Val> for bool {
    #[inline(always)]
    fn from(variant: Val) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VAL` reader - 0:0\\]
Command Execute value Initiates execution of the command specified in the CMDTYPE register."]
pub type ValR = crate::BitReader<Val>;
impl ValR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Val {
        match self.bits {
            true => Val::Execute,
            false => Val::Noexecute,
        }
    }
    #[doc = "Command will execute or is executing in NoWrapper"]
    #[inline(always)]
    pub fn is_execute(&self) -> bool {
        *self == Val::Execute
    }
    #[doc = "Command will not execute or is not executing in NoWrapper"]
    #[inline(always)]
    pub fn is_noexecute(&self) -> bool {
        *self == Val::Noexecute
    }
}
#[doc = "Field `VAL` writer - 0:0\\]
Command Execute value Initiates execution of the command specified in the CMDTYPE register."]
pub type ValW<'a, REG> = crate::BitWriter<'a, REG, Val>;
impl<'a, REG> ValW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command will execute or is executing in NoWrapper"]
    #[inline(always)]
    pub fn execute(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Execute)
    }
    #[doc = "Command will not execute or is not executing in NoWrapper"]
    #[inline(always)]
    pub fn noexecute(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Noexecute)
    }
}
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Command Execute value Initiates execution of the command specified in the CMDTYPE register."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Command Execute value Initiates execution of the command specified in the CMDTYPE register."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<CmdexecSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Command Execute Register: Initiates execution of the command specified in the CMDTYPE register. This register is blocked for writes after being written to 1 and prior to STATCMD.DONE being set by the NoWrapper hardware. NoWrapper hardware clears this register after the processing of the command has completed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdexec::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdexec::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdexecSpec;
impl crate::RegisterSpec for CmdexecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdexec::R`](R) reader structure"]
impl crate::Readable for CmdexecSpec {}
#[doc = "`write(|w| ..)` method takes [`cmdexec::W`](W) writer structure"]
impl crate::Writable for CmdexecSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMDEXEC to value 0"]
impl crate::Resettable for CmdexecSpec {
    const RESET_VALUE: u32 = 0;
}
