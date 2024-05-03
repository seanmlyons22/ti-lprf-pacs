#[doc = "Register `WARMRESET` reader"]
pub type R = crate::R<WarmresetSpec>;
#[doc = "Register `WARMRESET` writer"]
pub type W = crate::W<WarmresetSpec>;
#[doc = "Field `WDT_STAT` reader - 0:0\\]
0: No registered event 1: A WDT event has occured since last SW clear of the register. A read of this register clears both WDT_STAT and LOCKUP_STAT."]
pub type WdtStatR = crate::BitReader;
#[doc = "Field `WDT_STAT` writer - 0:0\\]
0: No registered event 1: A WDT event has occured since last SW clear of the register. A read of this register clears both WDT_STAT and LOCKUP_STAT."]
pub type WdtStatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKUP_STAT` reader - 1:1\\]
0: No registred event 1: A system CPU LOCKUP event has occured since last SW clear of the register. A read of this register clears both WDT_STAT and LOCKUP_STAT."]
pub type LockupStatR = crate::BitReader;
#[doc = "Field `LOCKUP_STAT` writer - 1:1\\]
0: No registred event 1: A system CPU LOCKUP event has occured since last SW clear of the register. A read of this register clears both WDT_STAT and LOCKUP_STAT."]
pub type LockupStatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_TO_PINRESET` reader - 2:2\\]
0: No action 1: A warm system reset event triggered by the below listed sources will result in an emulated pin reset. Warm reset sources included: ICEPick sysreset System CPU reset request, CPU_SCS:AIRCR.SYSRESETREQ System CPU Lockup WDT timeout An active ICEPick block system reset will gate all sources except ICEPick sysreset SW can read AON_SYSCTL:RESETCTL.RESET_SRC to find the source of the last reset resulting in a full power up sequence. WARMRESET in this register is set in the scenario that WR_TO_PINRESET=1 and one of the above listed sources is triggered."]
pub type WrToPinresetR = crate::BitReader;
#[doc = "Field `WR_TO_PINRESET` writer - 2:2\\]
0: No action 1: A warm system reset event triggered by the below listed sources will result in an emulated pin reset. Warm reset sources included: ICEPick sysreset System CPU reset request, CPU_SCS:AIRCR.SYSRESETREQ System CPU Lockup WDT timeout An active ICEPick block system reset will gate all sources except ICEPick sysreset SW can read AON_SYSCTL:RESETCTL.RESET_SRC to find the source of the last reset resulting in a full power up sequence. WARMRESET in this register is set in the scenario that WR_TO_PINRESET=1 and one of the above listed sources is triggered."]
pub type WrToPinresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0: No registered event 1: A WDT event has occured since last SW clear of the register. A read of this register clears both WDT_STAT and LOCKUP_STAT."]
    #[inline(always)]
    pub fn wdt_stat(&self) -> WdtStatR {
        WdtStatR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: No registred event 1: A system CPU LOCKUP event has occured since last SW clear of the register. A read of this register clears both WDT_STAT and LOCKUP_STAT."]
    #[inline(always)]
    pub fn lockup_stat(&self) -> LockupStatR {
        LockupStatR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
0: No action 1: A warm system reset event triggered by the below listed sources will result in an emulated pin reset. Warm reset sources included: ICEPick sysreset System CPU reset request, CPU_SCS:AIRCR.SYSRESETREQ System CPU Lockup WDT timeout An active ICEPick block system reset will gate all sources except ICEPick sysreset SW can read AON_SYSCTL:RESETCTL.RESET_SRC to find the source of the last reset resulting in a full power up sequence. WARMRESET in this register is set in the scenario that WR_TO_PINRESET=1 and one of the above listed sources is triggered."]
    #[inline(always)]
    pub fn wr_to_pinreset(&self) -> WrToPinresetR {
        WrToPinresetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
0: No registered event 1: A WDT event has occured since last SW clear of the register. A read of this register clears both WDT_STAT and LOCKUP_STAT."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_stat(&mut self) -> WdtStatW<WarmresetSpec> {
        WdtStatW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: No registred event 1: A system CPU LOCKUP event has occured since last SW clear of the register. A read of this register clears both WDT_STAT and LOCKUP_STAT."]
    #[inline(always)]
    #[must_use]
    pub fn lockup_stat(&mut self) -> LockupStatW<WarmresetSpec> {
        LockupStatW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
0: No action 1: A warm system reset event triggered by the below listed sources will result in an emulated pin reset. Warm reset sources included: ICEPick sysreset System CPU reset request, CPU_SCS:AIRCR.SYSRESETREQ System CPU Lockup WDT timeout An active ICEPick block system reset will gate all sources except ICEPick sysreset SW can read AON_SYSCTL:RESETCTL.RESET_SRC to find the source of the last reset resulting in a full power up sequence. WARMRESET in this register is set in the scenario that WR_TO_PINRESET=1 and one of the above listed sources is triggered."]
    #[inline(always)]
    #[must_use]
    pub fn wr_to_pinreset(&mut self) -> WrToPinresetW<WarmresetSpec> {
        WrToPinresetW::new(self, 2)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<WarmresetSpec> {
        Reserved3W::new(self, 3)
    }
}
#[doc = "WARM Reset Control And Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`warmreset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`warmreset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WarmresetSpec;
impl crate::RegisterSpec for WarmresetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`warmreset::R`](R) reader structure"]
impl crate::Readable for WarmresetSpec {}
#[doc = "`write(|w| ..)` method takes [`warmreset::W`](W) writer structure"]
impl crate::Writable for WarmresetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WARMRESET to value 0"]
impl crate::Resettable for WarmresetSpec {
    const RESET_VALUE: u32 = 0;
}
