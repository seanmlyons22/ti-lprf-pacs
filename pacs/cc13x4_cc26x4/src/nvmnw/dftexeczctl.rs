#[doc = "Register `DFTEXECZCTL` reader"]
pub type R = crate::R<DftexeczctlSpec>;
#[doc = "Register `DFTEXECZCTL` writer"]
pub type W = crate::W<DftexeczctlSpec>;
#[doc = "0:0\\]
Enable override of EXECUTEZ Note that when this bit is set, NoWrapper has control of the bank pins.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exezovren {
    #[doc = "1: Enable"]
    Enable = 1,
    #[doc = "0: Disable"]
    Disable = 0,
}
impl From<Exezovren> for bool {
    #[inline(always)]
    fn from(variant: Exezovren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXEZOVREN` reader - 0:0\\]
Enable override of EXECUTEZ Note that when this bit is set, NoWrapper has control of the bank pins."]
pub type ExezovrenR = crate::BitReader<Exezovren>;
impl ExezovrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exezovren {
        match self.bits {
            true => Exezovren::Enable,
            false => Exezovren::Disable,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Exezovren::Enable
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Exezovren::Disable
    }
}
#[doc = "Field `EXEZOVREN` writer - 0:0\\]
Enable override of EXECUTEZ Note that when this bit is set, NoWrapper has control of the bank pins."]
pub type ExezovrenW<'a, REG> = crate::BitWriter<'a, REG, Exezovren>;
impl<'a, REG> ExezovrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Exezovren::Enable)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Exezovren::Disable)
    }
}
#[doc = "1:1\\]
Override value to be applied to EXECUTEZ\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExezOvr {
    #[doc = "1: Set EXECUTEZ to 1"]
    One = 1,
    #[doc = "0: Set EXECUTEZ to 0"]
    Zero = 0,
}
impl From<ExezOvr> for bool {
    #[inline(always)]
    fn from(variant: ExezOvr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXEZ_OVR` reader - 1:1\\]
Override value to be applied to EXECUTEZ"]
pub type ExezOvrR = crate::BitReader<ExezOvr>;
impl ExezOvrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ExezOvr {
        match self.bits {
            true => ExezOvr::One,
            false => ExezOvr::Zero,
        }
    }
    #[doc = "Set EXECUTEZ to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == ExezOvr::One
    }
    #[doc = "Set EXECUTEZ to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == ExezOvr::Zero
    }
}
#[doc = "Field `EXEZ_OVR` writer - 1:1\\]
Override value to be applied to EXECUTEZ"]
pub type ExezOvrW<'a, REG> = crate::BitWriter<'a, REG, ExezOvr>;
impl<'a, REG> ExezOvrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set EXECUTEZ to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(ExezOvr::One)
    }
    #[doc = "Set EXECUTEZ to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(ExezOvr::Zero)
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable override of EXECUTEZ Note that when this bit is set, NoWrapper has control of the bank pins."]
    #[inline(always)]
    pub fn exezovren(&self) -> ExezovrenR {
        ExezovrenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Override value to be applied to EXECUTEZ"]
    #[inline(always)]
    pub fn exez_ovr(&self) -> ExezOvrR {
        ExezOvrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable override of EXECUTEZ Note that when this bit is set, NoWrapper has control of the bank pins."]
    #[inline(always)]
    #[must_use]
    pub fn exezovren(&mut self) -> ExezovrenW<DftexeczctlSpec> {
        ExezovrenW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Override value to be applied to EXECUTEZ"]
    #[inline(always)]
    #[must_use]
    pub fn exez_ovr(&mut self) -> ExezOvrW<DftexeczctlSpec> {
        ExezOvrW::new(self, 1)
    }
}
#[doc = "DFT EXECUTEZ control register. This register allows direct control of the EXECUTEZ to bank and pump for test. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dftexeczctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dftexeczctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DftexeczctlSpec;
impl crate::RegisterSpec for DftexeczctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dftexeczctl::R`](R) reader structure"]
impl crate::Readable for DftexeczctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dftexeczctl::W`](W) writer structure"]
impl crate::Writable for DftexeczctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFTEXECZCTL to value 0x02"]
impl crate::Resettable for DftexeczctlSpec {
    const RESET_VALUE: u32 = 0x02;
}
