#[doc = "Register `DFTPCLKTESTCTL` reader"]
pub type R = crate::R<DftpclktestctlSpec>;
#[doc = "Register `DFTPCLKTESTCTL` writer"]
pub type W = crate::W<DftpclktestctlSpec>;
#[doc = "0:0\\]
Enable the state machine which sequences measurement of pump clock frequency.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "1: Enable"]
    Enable = 1,
    #[doc = "0: Disable"]
    Disable = 0,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - 0:0\\]
Enable the state machine which sequences measurement of pump clock frequency."]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            true => Enable::Enable,
            false => Enable::Disable,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enable::Enable
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Enable::Disable
    }
}
#[doc = "Field `ENABLE` writer - 0:0\\]
Enable the state machine which sequences measurement of pump clock frequency."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Enable)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Disable)
    }
}
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable the state machine which sequences measurement of pump clock frequency."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
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
Enable the state machine which sequences measurement of pump clock frequency."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<DftpclktestctlSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "DFT Pump Clock Test Control Register. This register controls hardware features that allow the pump clock to be characterized for trim development. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dftpclktestctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dftpclktestctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DftpclktestctlSpec;
impl crate::RegisterSpec for DftpclktestctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dftpclktestctl::R`](R) reader structure"]
impl crate::Readable for DftpclktestctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dftpclktestctl::W`](W) writer structure"]
impl crate::Writable for DftpclktestctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFTPCLKTESTCTL to value 0"]
impl crate::Resettable for DftpclktestctlSpec {
    const RESET_VALUE: u32 = 0;
}
