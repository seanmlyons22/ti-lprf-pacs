#[doc = "Register `GLOCK` reader"]
pub type R = crate::R<GlockSpec>;
#[doc = "Register `GLOCK` writer"]
pub type W = crate::W<GlockSpec>;
#[doc = "0:0\\]
When LOCK is set, write access to registers in mutable and immutable sections are not allowed. Registers in mutable section can be temporarily unlocked by writing the KEY to MUNLOCK.KEY Following immutable registers are locked by GLOCK.LOCK TESTCFG I2VCFG TSENSCFG LPCMPCFG TIMMUTE0 TIMMUTE1 TIMMUTE2 PARTID LIFECYC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lock {
    #[doc = "1: Global Lock is set"]
    Set = 1,
    #[doc = "0: No effect"]
    Noeff = 0,
}
impl From<Lock> for bool {
    #[inline(always)]
    fn from(variant: Lock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - 0:0\\]
When LOCK is set, write access to registers in mutable and immutable sections are not allowed. Registers in mutable section can be temporarily unlocked by writing the KEY to MUNLOCK.KEY Following immutable registers are locked by GLOCK.LOCK TESTCFG I2VCFG TSENSCFG LPCMPCFG TIMMUTE0 TIMMUTE1 TIMMUTE2 PARTID LIFECYC"]
pub type LockR = crate::BitReader<Lock>;
impl LockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lock {
        match self.bits {
            true => Lock::Set,
            false => Lock::Noeff,
        }
    }
    #[doc = "Global Lock is set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Lock::Set
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Lock::Noeff
    }
}
#[doc = "Field `LOCK` writer - 0:0\\]
When LOCK is set, write access to registers in mutable and immutable sections are not allowed. Registers in mutable section can be temporarily unlocked by writing the KEY to MUNLOCK.KEY Following immutable registers are locked by GLOCK.LOCK TESTCFG I2VCFG TSENSCFG LPCMPCFG TIMMUTE0 TIMMUTE1 TIMMUTE2 PARTID LIFECYC"]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG, Lock>;
impl<'a, REG> LockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Global Lock is set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::Set)
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::Noeff)
    }
}
#[doc = "Field `RESERVED1` reader - 31:1\\]
RESERVED"]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
RESERVED"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
When LOCK is set, write access to registers in mutable and immutable sections are not allowed. Registers in mutable section can be temporarily unlocked by writing the KEY to MUNLOCK.KEY Following immutable registers are locked by GLOCK.LOCK TESTCFG I2VCFG TSENSCFG LPCMPCFG TIMMUTE0 TIMMUTE1 TIMMUTE2 PARTID LIFECYC"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
RESERVED"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
When LOCK is set, write access to registers in mutable and immutable sections are not allowed. Registers in mutable section can be temporarily unlocked by writing the KEY to MUNLOCK.KEY Following immutable registers are locked by GLOCK.LOCK TESTCFG I2VCFG TSENSCFG LPCMPCFG TIMMUTE0 TIMMUTE1 TIMMUTE2 PARTID LIFECYC"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LockW<GlockSpec> {
        LockW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
RESERVED"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<GlockSpec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "Global Lock Register. Locks registers in both mutable and immutable sections\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GlockSpec;
impl crate::RegisterSpec for GlockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`glock::R`](R) reader structure"]
impl crate::Readable for GlockSpec {}
#[doc = "`write(|w| ..)` method takes [`glock::W`](W) writer structure"]
impl crate::Writable for GlockSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GLOCK to value 0"]
impl crate::Resettable for GlockSpec {
    const RESET_VALUE: u32 = 0;
}
