#[doc = "Register `DFTEN` reader"]
pub type R = crate::R<DftenSpec>;
#[doc = "Register `DFTEN` writer"]
pub type W = crate::W<DftenSpec>;
#[doc = "0:0\\]
Enable Test Features\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "1: Command"]
    Enabled = 1,
    #[doc = "0: Command"]
    Disabled = 0,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - 0:0\\]
Enable Test Features"]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            true => Enable::Enabled,
            false => Enable::Disabled,
        }
    }
    #[doc = "Command"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Enable::Enabled
    }
    #[doc = "Command"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enable::Disabled
    }
}
#[doc = "Field `ENABLE` writer - 0:0\\]
Enable Test Features"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Enabled)
    }
    #[doc = "Command"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Disabled)
    }
}
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable Test Features"]
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
Enable Test Features"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<DftenSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "DFT Enable Register Allows control of NoWrapper test features. When set, DFT* registers in this aperture open for write access. When cleared, DFT* registers are read-only. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dften::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dften::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DftenSpec;
impl crate::RegisterSpec for DftenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dften::R`](R) reader structure"]
impl crate::Readable for DftenSpec {}
#[doc = "`write(|w| ..)` method takes [`dften::W`](W) writer structure"]
impl crate::Writable for DftenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFTEN to value 0"]
impl crate::Resettable for DftenSpec {
    const RESET_VALUE: u32 = 0;
}
