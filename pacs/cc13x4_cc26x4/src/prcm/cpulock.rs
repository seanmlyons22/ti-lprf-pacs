#[doc = "Register `CPULOCK` reader"]
pub type R = crate::R<CpulockSpec>;
#[doc = "Register `CPULOCK` writer"]
pub type W = crate::W<CpulockSpec>;
#[doc = "Field `LOCKSMPU` reader - 0:0\\]
When set will lock secure MPU Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
pub type LocksmpuR = crate::BitReader;
#[doc = "Field `LOCKSMPU` writer - 0:0\\]
When set will lock secure MPU Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
pub type LocksmpuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKNSMPU` reader - 1:1\\]
When set will lock non-secure MPU Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
pub type LocknsmpuR = crate::BitReader;
#[doc = "Field `LOCKNSMPU` writer - 1:1\\]
When set will lock non-secure MPU Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
pub type LocknsmpuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKSAU` reader - 2:2\\]
When set will lock SAU regions Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
pub type LocksauR = crate::BitReader;
#[doc = "Field `LOCKSAU` writer - 2:2\\]
When set will lock SAU regions Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
pub type LocksauW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKSVTAIRCR` reader - 3:3\\]
When set will lock - Secure vector table base address - Secure interrupt priority - Busfault - Hardfault NMI security target Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
pub type LocksvtaircrR = crate::BitReader;
#[doc = "Field `LOCKSVTAIRCR` writer - 3:3\\]
When set will lock - Secure vector table base address - Secure interrupt priority - Busfault - Hardfault NMI security target Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
pub type LocksvtaircrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKNSVTOR` reader - 4:4\\]
When set will lock non-secure vector table base address Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
pub type LocknsvtorR = crate::BitReader;
#[doc = "Field `LOCKNSVTOR` writer - 4:4\\]
When set will lock non-secure vector table base address Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
pub type LocknsvtorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED5` reader - 30:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved5R = crate::FieldReader<u32>;
#[doc = "Field `PARITY` reader - 31:31\\]
Register parity bit"]
pub type ParityR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 0:0\\]
When set will lock secure MPU Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
    #[inline(always)]
    pub fn locksmpu(&self) -> LocksmpuR {
        LocksmpuR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
When set will lock non-secure MPU Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
    #[inline(always)]
    pub fn locknsmpu(&self) -> LocknsmpuR {
        LocknsmpuR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
When set will lock SAU regions Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
    #[inline(always)]
    pub fn locksau(&self) -> LocksauR {
        LocksauR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
When set will lock - Secure vector table base address - Secure interrupt priority - Busfault - Hardfault NMI security target Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
    #[inline(always)]
    pub fn locksvtaircr(&self) -> LocksvtaircrR {
        LocksvtaircrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
When set will lock non-secure vector table base address Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
    #[inline(always)]
    pub fn locknsvtor(&self) -> LocknsvtorR {
        LocknsvtorR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:30 - 30:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new((self.bits >> 5) & 0x03ff_ffff)
    }
    #[doc = "Bit 31 - 31:31\\]
Register parity bit"]
    #[inline(always)]
    pub fn parity(&self) -> ParityR {
        ParityR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
When set will lock secure MPU Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn locksmpu(&mut self) -> LocksmpuW<CpulockSpec> {
        LocksmpuW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
When set will lock non-secure MPU Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn locknsmpu(&mut self) -> LocknsmpuW<CpulockSpec> {
        LocknsmpuW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
When set will lock SAU regions Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn locksau(&mut self) -> LocksauW<CpulockSpec> {
        LocksauW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
When set will lock - Secure vector table base address - Secure interrupt priority - Busfault - Hardfault NMI security target Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn locksvtaircr(&mut self) -> LocksvtaircrW<CpulockSpec> {
        LocksvtaircrW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
When set will lock non-secure vector table base address Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn locknsvtor(&mut self) -> LocknsvtorW<CpulockSpec> {
        LocknsvtorW::new(self, 4)
    }
}
#[doc = "CPU Lock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpulock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpulock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpulockSpec;
impl crate::RegisterSpec for CpulockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpulock::R`](R) reader structure"]
impl crate::Readable for CpulockSpec {}
#[doc = "`write(|w| ..)` method takes [`cpulock::W`](W) writer structure"]
impl crate::Writable for CpulockSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPULOCK to value 0x8000_001f"]
impl crate::Resettable for CpulockSpec {
    const RESET_VALUE: u32 = 0x8000_001f;
}
