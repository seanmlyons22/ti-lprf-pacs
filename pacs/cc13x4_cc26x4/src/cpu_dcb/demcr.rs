#[doc = "Register `DEMCR` reader"]
pub type R = crate::R<DemcrSpec>;
#[doc = "Register `DEMCR` writer"]
pub type W = crate::W<DemcrSpec>;
#[doc = "Field `VC_CORERESET` reader - 0:0\\]
Enable Reset Vector Catch. This causes a warm reset to halt a running system"]
pub type VcCoreresetR = crate::BitReader;
#[doc = "Field `VC_CORERESET` writer - 0:0\\]
Enable Reset Vector Catch. This causes a warm reset to halt a running system"]
pub type VcCoreresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `RESERVED1` writer - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `VC_MMERR` reader - 4:4\\]
Enable halting debug trap on a MemManage exception"]
pub type VcMmerrR = crate::BitReader;
#[doc = "Field `VC_MMERR` writer - 4:4\\]
Enable halting debug trap on a MemManage exception"]
pub type VcMmerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VC_NOCPERR` reader - 5:5\\]
Enable halting debug trap on a UsageFault caused by an access to a coprocessor"]
pub type VcNocperrR = crate::BitReader;
#[doc = "Field `VC_NOCPERR` writer - 5:5\\]
Enable halting debug trap on a UsageFault caused by an access to a coprocessor"]
pub type VcNocperrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VC_CHKERR` reader - 6:6\\]
Enable halting debug trap on a UsageFault exception caused by a checking error, for example an alignment check error"]
pub type VcChkerrR = crate::BitReader;
#[doc = "Field `VC_CHKERR` writer - 6:6\\]
Enable halting debug trap on a UsageFault exception caused by a checking error, for example an alignment check error"]
pub type VcChkerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VC_STATERR` reader - 7:7\\]
Enable halting debug trap on a UsageFault exception caused by a state information error, for example an Undefined Instruction exception"]
pub type VcStaterrR = crate::BitReader;
#[doc = "Field `VC_STATERR` writer - 7:7\\]
Enable halting debug trap on a UsageFault exception caused by a state information error, for example an Undefined Instruction exception"]
pub type VcStaterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VC_BUSERR` reader - 8:8\\]
BusFault exception halting debug vector catch enable"]
pub type VcBuserrR = crate::BitReader;
#[doc = "Field `VC_BUSERR` writer - 8:8\\]
BusFault exception halting debug vector catch enable"]
pub type VcBuserrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VC_INTERR` reader - 9:9\\]
Enable halting debug vector catch for faults during exception entry and return"]
pub type VcInterrR = crate::BitReader;
#[doc = "Field `VC_INTERR` writer - 9:9\\]
Enable halting debug vector catch for faults during exception entry and return"]
pub type VcInterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VC_HARDERR` reader - 10:10\\]
HardFault exception halting debug vector catch enable"]
pub type VcHarderrR = crate::BitReader;
#[doc = "Field `VC_HARDERR` writer - 10:10\\]
HardFault exception halting debug vector catch enable"]
pub type VcHarderrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VC_SFERR` reader - 11:11\\]
SecureFault exception halting debug vector catch enable"]
pub type VcSferrR = crate::BitReader;
#[doc = "Field `VC_SFERR` writer - 11:11\\]
SecureFault exception halting debug vector catch enable"]
pub type VcSferrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED12` reader - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12R = crate::FieldReader;
#[doc = "Field `RESERVED12` writer - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MON_EN` reader - 16:16\\]
Enable the DebugMonitor exception"]
pub type MonEnR = crate::BitReader;
#[doc = "Field `MON_EN` writer - 16:16\\]
Enable the DebugMonitor exception"]
pub type MonEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MON_PEND` reader - 17:17\\]
Sets or clears the pending state of the DebugMonitor exception"]
pub type MonPendR = crate::BitReader;
#[doc = "Field `MON_PEND` writer - 17:17\\]
Sets or clears the pending state of the DebugMonitor exception"]
pub type MonPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MON_STEP` reader - 18:18\\]
Enable DebugMonitor stepping"]
pub type MonStepR = crate::BitReader;
#[doc = "Field `MON_STEP` writer - 18:18\\]
Enable DebugMonitor stepping"]
pub type MonStepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MON_REQ` reader - 19:19\\]
DebugMonitor semaphore bit"]
pub type MonReqR = crate::BitReader;
#[doc = "Field `MON_REQ` writer - 19:19\\]
DebugMonitor semaphore bit"]
pub type MonReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDME` reader - 20:20\\]
Indicates whether the DebugMonitor targets the Secure or the Non-secure state and whether debug events are allowed in Secure state"]
pub type SdmeR = crate::BitReader;
#[doc = "Field `SDME` writer - 20:20\\]
Indicates whether the DebugMonitor targets the Secure or the Non-secure state and whether debug events are allowed in Secure state"]
pub type SdmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED21` reader - 23:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved21R = crate::FieldReader;
#[doc = "Field `RESERVED21` writer - 23:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved21W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TRCENA` reader - 24:24\\]
Global enable for all DWT and ITM features"]
pub type TrcenaR = crate::BitReader;
#[doc = "Field `TRCENA` writer - 24:24\\]
Global enable for all DWT and ITM features"]
pub type TrcenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED25` reader - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved25R = crate::FieldReader;
#[doc = "Field `RESERVED25` writer - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved25W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable Reset Vector Catch. This causes a warm reset to halt a running system"]
    #[inline(always)]
    pub fn vc_corereset(&self) -> VcCoreresetR {
        VcCoreresetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Enable halting debug trap on a MemManage exception"]
    #[inline(always)]
    pub fn vc_mmerr(&self) -> VcMmerrR {
        VcMmerrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Enable halting debug trap on a UsageFault caused by an access to a coprocessor"]
    #[inline(always)]
    pub fn vc_nocperr(&self) -> VcNocperrR {
        VcNocperrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Enable halting debug trap on a UsageFault exception caused by a checking error, for example an alignment check error"]
    #[inline(always)]
    pub fn vc_chkerr(&self) -> VcChkerrR {
        VcChkerrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Enable halting debug trap on a UsageFault exception caused by a state information error, for example an Undefined Instruction exception"]
    #[inline(always)]
    pub fn vc_staterr(&self) -> VcStaterrR {
        VcStaterrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
BusFault exception halting debug vector catch enable"]
    #[inline(always)]
    pub fn vc_buserr(&self) -> VcBuserrR {
        VcBuserrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Enable halting debug vector catch for faults during exception entry and return"]
    #[inline(always)]
    pub fn vc_interr(&self) -> VcInterrR {
        VcInterrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
HardFault exception halting debug vector catch enable"]
    #[inline(always)]
    pub fn vc_harderr(&self) -> VcHarderrR {
        VcHarderrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
SecureFault exception halting debug vector catch enable"]
    #[inline(always)]
    pub fn vc_sferr(&self) -> VcSferrR {
        VcSferrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable the DebugMonitor exception"]
    #[inline(always)]
    pub fn mon_en(&self) -> MonEnR {
        MonEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Sets or clears the pending state of the DebugMonitor exception"]
    #[inline(always)]
    pub fn mon_pend(&self) -> MonPendR {
        MonPendR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Enable DebugMonitor stepping"]
    #[inline(always)]
    pub fn mon_step(&self) -> MonStepR {
        MonStepR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
DebugMonitor semaphore bit"]
    #[inline(always)]
    pub fn mon_req(&self) -> MonReqR {
        MonReqR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Indicates whether the DebugMonitor targets the Secure or the Non-secure state and whether debug events are allowed in Secure state"]
    #[inline(always)]
    pub fn sdme(&self) -> SdmeR {
        SdmeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved21(&self) -> Reserved21R {
        Reserved21R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Global enable for all DWT and ITM features"]
    #[inline(always)]
    pub fn trcena(&self) -> TrcenaR {
        TrcenaR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved25(&self) -> Reserved25R {
        Reserved25R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable Reset Vector Catch. This causes a warm reset to halt a running system"]
    #[inline(always)]
    #[must_use]
    pub fn vc_corereset(&mut self) -> VcCoreresetW<DemcrSpec> {
        VcCoreresetW::new(self, 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<DemcrSpec> {
        Reserved1W::new(self, 1)
    }
    #[doc = "Bit 4 - 4:4\\]
Enable halting debug trap on a MemManage exception"]
    #[inline(always)]
    #[must_use]
    pub fn vc_mmerr(&mut self) -> VcMmerrW<DemcrSpec> {
        VcMmerrW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Enable halting debug trap on a UsageFault caused by an access to a coprocessor"]
    #[inline(always)]
    #[must_use]
    pub fn vc_nocperr(&mut self) -> VcNocperrW<DemcrSpec> {
        VcNocperrW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Enable halting debug trap on a UsageFault exception caused by a checking error, for example an alignment check error"]
    #[inline(always)]
    #[must_use]
    pub fn vc_chkerr(&mut self) -> VcChkerrW<DemcrSpec> {
        VcChkerrW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Enable halting debug trap on a UsageFault exception caused by a state information error, for example an Undefined Instruction exception"]
    #[inline(always)]
    #[must_use]
    pub fn vc_staterr(&mut self) -> VcStaterrW<DemcrSpec> {
        VcStaterrW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
BusFault exception halting debug vector catch enable"]
    #[inline(always)]
    #[must_use]
    pub fn vc_buserr(&mut self) -> VcBuserrW<DemcrSpec> {
        VcBuserrW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Enable halting debug vector catch for faults during exception entry and return"]
    #[inline(always)]
    #[must_use]
    pub fn vc_interr(&mut self) -> VcInterrW<DemcrSpec> {
        VcInterrW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
HardFault exception halting debug vector catch enable"]
    #[inline(always)]
    #[must_use]
    pub fn vc_harderr(&mut self) -> VcHarderrW<DemcrSpec> {
        VcHarderrW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
SecureFault exception halting debug vector catch enable"]
    #[inline(always)]
    #[must_use]
    pub fn vc_sferr(&mut self) -> VcSferrW<DemcrSpec> {
        VcSferrW::new(self, 11)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> Reserved12W<DemcrSpec> {
        Reserved12W::new(self, 12)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable the DebugMonitor exception"]
    #[inline(always)]
    #[must_use]
    pub fn mon_en(&mut self) -> MonEnW<DemcrSpec> {
        MonEnW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Sets or clears the pending state of the DebugMonitor exception"]
    #[inline(always)]
    #[must_use]
    pub fn mon_pend(&mut self) -> MonPendW<DemcrSpec> {
        MonPendW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Enable DebugMonitor stepping"]
    #[inline(always)]
    #[must_use]
    pub fn mon_step(&mut self) -> MonStepW<DemcrSpec> {
        MonStepW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
DebugMonitor semaphore bit"]
    #[inline(always)]
    #[must_use]
    pub fn mon_req(&mut self) -> MonReqW<DemcrSpec> {
        MonReqW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Indicates whether the DebugMonitor targets the Secure or the Non-secure state and whether debug events are allowed in Secure state"]
    #[inline(always)]
    #[must_use]
    pub fn sdme(&mut self) -> SdmeW<DemcrSpec> {
        SdmeW::new(self, 20)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved21(&mut self) -> Reserved21W<DemcrSpec> {
        Reserved21W::new(self, 21)
    }
    #[doc = "Bit 24 - 24:24\\]
Global enable for all DWT and ITM features"]
    #[inline(always)]
    #[must_use]
    pub fn trcena(&mut self) -> TrcenaW<DemcrSpec> {
        TrcenaW::new(self, 24)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved25(&mut self) -> Reserved25W<DemcrSpec> {
        Reserved25W::new(self, 25)
    }
}
#[doc = "Manages vector catch behavior and DebugMonitor handling when debugging\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`demcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`demcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DemcrSpec;
impl crate::RegisterSpec for DemcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`demcr::R`](R) reader structure"]
impl crate::Readable for DemcrSpec {}
#[doc = "`write(|w| ..)` method takes [`demcr::W`](W) writer structure"]
impl crate::Writable for DemcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEMCR to value 0"]
impl crate::Resettable for DemcrSpec {
    const RESET_VALUE: u32 = 0;
}
