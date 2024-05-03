#[doc = "Register `CPU_LOCK_CFG` reader"]
pub type R = crate::R<CpuLockCfgSpec>;
#[doc = "Register `CPU_LOCK_CFG` writer"]
pub type W = crate::W<CpuLockCfgSpec>;
#[doc = "Field `LOCKSMPU_N` reader - 0:0\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKSMPU by ROM boot FW"]
pub type LocksmpuNR = crate::BitReader;
#[doc = "Field `LOCKSMPU_N` writer - 0:0\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKSMPU by ROM boot FW"]
pub type LocksmpuNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKNSMPU_N` reader - 1:1\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKNSMPU by ROM boot FW"]
pub type LocknsmpuNR = crate::BitReader;
#[doc = "Field `LOCKNSMPU_N` writer - 1:1\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKNSMPU by ROM boot FW"]
pub type LocknsmpuNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKSAU_N` reader - 2:2\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKSAU by ROM boot FW"]
pub type LocksauNR = crate::BitReader;
#[doc = "Field `LOCKSAU_N` writer - 2:2\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKSAU by ROM boot FW"]
pub type LocksauNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKSVTAIRCR_N` reader - 3:3\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKSVTAIRCR by ROM boot FW"]
pub type LocksvtaircrNR = crate::BitReader;
#[doc = "Field `LOCKSVTAIRCR_N` writer - 3:3\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKSVTAIRCR by ROM boot FW"]
pub type LocksvtaircrNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKNSVTOR_N` reader - 4:4\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKNSVTOR by ROM boot FW"]
pub type LocknsvtorNR = crate::BitReader;
#[doc = "Field `LOCKNSVTOR_N` writer - 4:4\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKNSVTOR by ROM boot FW"]
pub type LocknsvtorNW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKSMPU by ROM boot FW"]
    #[inline(always)]
    pub fn locksmpu_n(&self) -> LocksmpuNR {
        LocksmpuNR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKNSMPU by ROM boot FW"]
    #[inline(always)]
    pub fn locknsmpu_n(&self) -> LocknsmpuNR {
        LocknsmpuNR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKSAU by ROM boot FW"]
    #[inline(always)]
    pub fn locksau_n(&self) -> LocksauNR {
        LocksauNR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKSVTAIRCR by ROM boot FW"]
    #[inline(always)]
    pub fn locksvtaircr_n(&self) -> LocksvtaircrNR {
        LocksvtaircrNR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKNSVTOR by ROM boot FW"]
    #[inline(always)]
    pub fn locknsvtor_n(&self) -> LocknsvtorNR {
        LocknsvtorNR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKSMPU by ROM boot FW"]
    #[inline(always)]
    #[must_use]
    pub fn locksmpu_n(&mut self) -> LocksmpuNW<CpuLockCfgSpec> {
        LocksmpuNW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKNSMPU by ROM boot FW"]
    #[inline(always)]
    #[must_use]
    pub fn locknsmpu_n(&mut self) -> LocknsmpuNW<CpuLockCfgSpec> {
        LocknsmpuNW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKSAU by ROM boot FW"]
    #[inline(always)]
    #[must_use]
    pub fn locksau_n(&mut self) -> LocksauNW<CpuLockCfgSpec> {
        LocksauNW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKSVTAIRCR by ROM boot FW"]
    #[inline(always)]
    #[must_use]
    pub fn locksvtaircr_n(&mut self) -> LocksvtaircrNW<CpuLockCfgSpec> {
        LocksvtaircrNW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKNSVTOR by ROM boot FW"]
    #[inline(always)]
    #[must_use]
    pub fn locknsvtor_n(&mut self) -> LocknsvtorNW<CpuLockCfgSpec> {
        LocknsvtorNW::new(self, 4)
    }
}
#[doc = "Configuration register for MCU CPU lock options\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_lock_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_lock_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuLockCfgSpec;
impl crate::RegisterSpec for CpuLockCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_lock_cfg::R`](R) reader structure"]
impl crate::Readable for CpuLockCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cpu_lock_cfg::W`](W) writer structure"]
impl crate::Writable for CpuLockCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPU_LOCK_CFG to value 0xffff_ffff"]
impl crate::Resettable for CpuLockCfgSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
