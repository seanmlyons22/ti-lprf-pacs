#[doc = "Register `DFTBANKCTL` reader"]
pub type R = crate::R<DftbankctlSpec>;
#[doc = "Register `DFTBANKCTL` writer"]
pub type W = crate::W<DftbankctlSpec>;
#[doc = "6:0\\]
TCR test mode to be applied to the bank\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tcr {
    #[doc = "127: Maximum value"]
    Maximum = 127,
    #[doc = "0: Minimum value"]
    Minimum = 0,
}
impl From<Tcr> for u8 {
    #[inline(always)]
    fn from(variant: Tcr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tcr {
    type Ux = u8;
}
impl crate::IsEnum for Tcr {}
#[doc = "Field `TCR` reader - 6:0\\]
TCR test mode to be applied to the bank"]
pub type TcrR = crate::FieldReader<Tcr>;
impl TcrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tcr> {
        match self.bits {
            127 => Some(Tcr::Maximum),
            0 => Some(Tcr::Minimum),
            _ => None,
        }
    }
    #[doc = "Maximum value"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == Tcr::Maximum
    }
    #[doc = "Minimum value"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == Tcr::Minimum
    }
}
#[doc = "Field `TCR` writer - 6:0\\]
TCR test mode to be applied to the bank"]
pub type TcrW<'a, REG> = crate::FieldWriter<'a, REG, 7, Tcr>;
impl<'a, REG> TcrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Maximum value"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut crate::W<REG> {
        self.variant(Tcr::Maximum)
    }
    #[doc = "Minimum value"]
    #[inline(always)]
    pub fn minimum(self) -> &'a mut crate::W<REG> {
        self.variant(Tcr::Minimum)
    }
}
#[doc = "Field `RESERVED7` reader - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `RESERVED7` writer - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "8:8\\]
When set, TEZ is asserted to the flash banks. Which banks get the asserted signal is determined by the BANKSELECT field in CMDCTL. 0x0 Do no assert TEZ 0x1 Assert TEZ\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tez {
    #[doc = "1: Do not assert TEZ"]
    Negate = 1,
    #[doc = "0: Assert TEZ"]
    Assert = 0,
}
impl From<Tez> for bool {
    #[inline(always)]
    fn from(variant: Tez) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEZ` reader - 8:8\\]
When set, TEZ is asserted to the flash banks. Which banks get the asserted signal is determined by the BANKSELECT field in CMDCTL. 0x0 Do no assert TEZ 0x1 Assert TEZ"]
pub type TezR = crate::BitReader<Tez>;
impl TezR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tez {
        match self.bits {
            true => Tez::Negate,
            false => Tez::Assert,
        }
    }
    #[doc = "Do not assert TEZ"]
    #[inline(always)]
    pub fn is_negate(&self) -> bool {
        *self == Tez::Negate
    }
    #[doc = "Assert TEZ"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == Tez::Assert
    }
}
#[doc = "Field `TEZ` writer - 8:8\\]
When set, TEZ is asserted to the flash banks. Which banks get the asserted signal is determined by the BANKSELECT field in CMDCTL. 0x0 Do no assert TEZ 0x1 Assert TEZ"]
pub type TezW<'a, REG> = crate::BitWriter<'a, REG, Tez>;
impl<'a, REG> TezW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not assert TEZ"]
    #[inline(always)]
    pub fn negate(self) -> &'a mut crate::W<REG> {
        self.variant(Tez::Negate)
    }
    #[doc = "Assert TEZ"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut crate::W<REG> {
        self.variant(Tez::Assert)
    }
}
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
TCR test mode to be applied to the bank"]
    #[inline(always)]
    pub fn tcr(&self) -> TcrR {
        TcrR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
When set, TEZ is asserted to the flash banks. Which banks get the asserted signal is determined by the BANKSELECT field in CMDCTL. 0x0 Do no assert TEZ 0x1 Assert TEZ"]
    #[inline(always)]
    pub fn tez(&self) -> TezR {
        TezR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
TCR test mode to be applied to the bank"]
    #[inline(always)]
    #[must_use]
    pub fn tcr(&mut self) -> TcrW<DftbankctlSpec> {
        TcrW::new(self, 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> Reserved7W<DftbankctlSpec> {
        Reserved7W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
When set, TEZ is asserted to the flash banks. Which banks get the asserted signal is determined by the BANKSELECT field in CMDCTL. 0x0 Do no assert TEZ 0x1 Assert TEZ"]
    #[inline(always)]
    #[must_use]
    pub fn tez(&mut self) -> TezW<DftbankctlSpec> {
        TezW::new(self, 8)
    }
}
#[doc = "DFT Bank Control Register This allows some configuration of bank parameters during test. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dftbankctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dftbankctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DftbankctlSpec;
impl crate::RegisterSpec for DftbankctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dftbankctl::R`](R) reader structure"]
impl crate::Readable for DftbankctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dftbankctl::W`](W) writer structure"]
impl crate::Writable for DftbankctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFTBANKCTL to value 0x0100"]
impl crate::Resettable for DftbankctlSpec {
    const RESET_VALUE: u32 = 0x0100;
}
