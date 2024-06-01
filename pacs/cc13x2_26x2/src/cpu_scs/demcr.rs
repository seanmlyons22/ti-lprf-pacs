#[doc = "Register `DEMCR` reader"]
pub type R = crate::R<DemcrSpec>;
#[doc = "Register `DEMCR` writer"]
pub type W = crate::W<DemcrSpec>;
#[doc = "Field `VC_CORERESET` reader - 0:0\\]
Reset Vector Catch. Halt running system if Core reset occurs. Ignored when DHCSR.C_DEBUGEN is cleared."]
pub type VcCoreresetR = crate::BitReader;
#[doc = "Field `VC_CORERESET` writer - 0:0\\]
Reset Vector Catch. Halt running system if Core reset occurs. Ignored when DHCSR.C_DEBUGEN is cleared."]
pub type VcCoreresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `RESERVED1` writer - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `VC_MMERR` reader - 4:4\\]
Debug trap on Memory Management faults. Ignored when DHCSR.C_DEBUGEN is cleared."]
pub type VcMmerrR = crate::BitReader;
#[doc = "Field `VC_MMERR` writer - 4:4\\]
Debug trap on Memory Management faults. Ignored when DHCSR.C_DEBUGEN is cleared."]
pub type VcMmerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VC_NOCPERR` reader - 5:5\\]
Debug trap on a UsageFault access to a Coprocessor. Ignored when DHCSR.C_DEBUGEN is cleared."]
pub type VcNocperrR = crate::BitReader;
#[doc = "Field `VC_NOCPERR` writer - 5:5\\]
Debug trap on a UsageFault access to a Coprocessor. Ignored when DHCSR.C_DEBUGEN is cleared."]
pub type VcNocperrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VC_CHKERR` reader - 6:6\\]
Debug trap on Usage Fault enabled checking errors. Ignored when DHCSR.C_DEBUGEN is cleared."]
pub type VcChkerrR = crate::BitReader;
#[doc = "Field `VC_CHKERR` writer - 6:6\\]
Debug trap on Usage Fault enabled checking errors. Ignored when DHCSR.C_DEBUGEN is cleared."]
pub type VcChkerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VC_STATERR` reader - 7:7\\]
Debug trap on Usage Fault state errors. Ignored when DHCSR.C_DEBUGEN is cleared."]
pub type VcStaterrR = crate::BitReader;
#[doc = "Field `VC_STATERR` writer - 7:7\\]
Debug trap on Usage Fault state errors. Ignored when DHCSR.C_DEBUGEN is cleared."]
pub type VcStaterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VC_BUSERR` reader - 8:8\\]
Debug Trap on normal Bus error. Ignored when DHCSR.C_DEBUGEN is cleared."]
pub type VcBuserrR = crate::BitReader;
#[doc = "Field `VC_BUSERR` writer - 8:8\\]
Debug Trap on normal Bus error. Ignored when DHCSR.C_DEBUGEN is cleared."]
pub type VcBuserrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VC_INTERR` reader - 9:9\\]
Debug trap on a fault occurring during an exception entry or return sequence. Ignored when DHCSR.C_DEBUGEN is cleared."]
pub type VcInterrR = crate::BitReader;
#[doc = "Field `VC_INTERR` writer - 9:9\\]
Debug trap on a fault occurring during an exception entry or return sequence. Ignored when DHCSR.C_DEBUGEN is cleared."]
pub type VcInterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VC_HARDERR` reader - 10:10\\]
Debug trap on Hard Fault. Ignored when DHCSR.C_DEBUGEN is cleared."]
pub type VcHarderrR = crate::BitReader;
#[doc = "Field `VC_HARDERR` writer - 10:10\\]
Debug trap on Hard Fault. Ignored when DHCSR.C_DEBUGEN is cleared."]
pub type VcHarderrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED11` reader - 15:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11R = crate::FieldReader;
#[doc = "Field `RESERVED11` writer - 15:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `MON_EN` reader - 16:16\\]
Enable the debug monitor. When enabled, the System handler priority register controls its priority level. If disabled, then all debug events go to Hard fault. DHCSR.C_DEBUGEN overrides this bit. Vector catching is semi-synchronous. When a matching event is seen, a Halt is requested. Because the processor can only halt on an instruction boundary, it must wait until the next instruction boundary. As a result, it stops on the first instruction of the exception handler. However, two special cases exist when a vector catch has triggered: 1. If a fault is taken during vectoring, vector read or stack push error, the halt occurs on the corresponding fault handler, for the vector error or stack push. 2. If a late arriving interrupt comes in during vectoring, it is not taken. That is, an implementation that supports the late arrival optimization must suppress it in this case."]
pub type MonEnR = crate::BitReader;
#[doc = "Field `MON_EN` writer - 16:16\\]
Enable the debug monitor. When enabled, the System handler priority register controls its priority level. If disabled, then all debug events go to Hard fault. DHCSR.C_DEBUGEN overrides this bit. Vector catching is semi-synchronous. When a matching event is seen, a Halt is requested. Because the processor can only halt on an instruction boundary, it must wait until the next instruction boundary. As a result, it stops on the first instruction of the exception handler. However, two special cases exist when a vector catch has triggered: 1. If a fault is taken during vectoring, vector read or stack push error, the halt occurs on the corresponding fault handler, for the vector error or stack push. 2. If a late arriving interrupt comes in during vectoring, it is not taken. That is, an implementation that supports the late arrival optimization must suppress it in this case."]
pub type MonEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MON_PEND` reader - 17:17\\]
Pend the monitor to activate when priority permits. This can wake up the monitor through the AHB-AP port. It is the equivalent to DHCSR.C_HALT for Monitor debug. This register does not reset on a system reset. It is only reset by a power-on reset. Software in the reset handler or later, or by the DAP must enable the debug monitor."]
pub type MonPendR = crate::BitReader;
#[doc = "Field `MON_PEND` writer - 17:17\\]
Pend the monitor to activate when priority permits. This can wake up the monitor through the AHB-AP port. It is the equivalent to DHCSR.C_HALT for Monitor debug. This register does not reset on a system reset. It is only reset by a power-on reset. Software in the reset handler or later, or by the DAP must enable the debug monitor."]
pub type MonPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MON_STEP` reader - 18:18\\]
When MON_EN = 1, this steps the core. When MON_EN = 0, this bit is ignored. This is the equivalent to DHCSR.C_STEP. Interrupts are only stepped according to the priority of the monitor and settings of PRIMASK, FAULTMASK, or BASEPRI."]
pub type MonStepR = crate::BitReader;
#[doc = "Field `MON_STEP` writer - 18:18\\]
When MON_EN = 1, this steps the core. When MON_EN = 0, this bit is ignored. This is the equivalent to DHCSR.C_STEP. Interrupts are only stepped according to the priority of the monitor and settings of PRIMASK, FAULTMASK, or BASEPRI."]
pub type MonStepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MON_REQ` reader - 19:19\\]
This enables the monitor to identify how it wakes up. This bit clears on a Core Reset. 0x0: Woken up by debug exception. 0x1: Woken up by MON_PEND"]
pub type MonReqR = crate::BitReader;
#[doc = "Field `MON_REQ` writer - 19:19\\]
This enables the monitor to identify how it wakes up. This bit clears on a Core Reset. 0x0: Woken up by debug exception. 0x1: Woken up by MON_PEND"]
pub type MonReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED20` reader - 23:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved20R = crate::FieldReader;
#[doc = "Field `RESERVED20` writer - 23:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved20W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRCENA` reader - 24:24\\]
This bit must be set to 1 to enable use of the trace and debug blocks: DWT, ITM, ETM and TPIU. This enables control of power usage unless tracing is required. The application can enable this, for ITM use, or use by a debugger."]
pub type TrcenaR = crate::BitReader;
#[doc = "Field `TRCENA` writer - 24:24\\]
This bit must be set to 1 to enable use of the trace and debug blocks: DWT, ITM, ETM and TPIU. This enables control of power usage unless tracing is required. The application can enable this, for ITM use, or use by a debugger."]
pub type TrcenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED25` reader - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved25R = crate::FieldReader;
#[doc = "Field `RESERVED25` writer - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved25W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Reset Vector Catch. Halt running system if Core reset occurs. Ignored when DHCSR.C_DEBUGEN is cleared."]
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
Debug trap on Memory Management faults. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    pub fn vc_mmerr(&self) -> VcMmerrR {
        VcMmerrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Debug trap on a UsageFault access to a Coprocessor. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    pub fn vc_nocperr(&self) -> VcNocperrR {
        VcNocperrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Debug trap on Usage Fault enabled checking errors. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    pub fn vc_chkerr(&self) -> VcChkerrR {
        VcChkerrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Debug trap on Usage Fault state errors. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    pub fn vc_staterr(&self) -> VcStaterrR {
        VcStaterrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Debug Trap on normal Bus error. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    pub fn vc_buserr(&self) -> VcBuserrR {
        VcBuserrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Debug trap on a fault occurring during an exception entry or return sequence. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    pub fn vc_interr(&self) -> VcInterrR {
        VcInterrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Debug trap on Hard Fault. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    pub fn vc_harderr(&self) -> VcHarderrR {
        VcHarderrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable the debug monitor. When enabled, the System handler priority register controls its priority level. If disabled, then all debug events go to Hard fault. DHCSR.C_DEBUGEN overrides this bit. Vector catching is semi-synchronous. When a matching event is seen, a Halt is requested. Because the processor can only halt on an instruction boundary, it must wait until the next instruction boundary. As a result, it stops on the first instruction of the exception handler. However, two special cases exist when a vector catch has triggered: 1. If a fault is taken during vectoring, vector read or stack push error, the halt occurs on the corresponding fault handler, for the vector error or stack push. 2. If a late arriving interrupt comes in during vectoring, it is not taken. That is, an implementation that supports the late arrival optimization must suppress it in this case."]
    #[inline(always)]
    pub fn mon_en(&self) -> MonEnR {
        MonEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pend the monitor to activate when priority permits. This can wake up the monitor through the AHB-AP port. It is the equivalent to DHCSR.C_HALT for Monitor debug. This register does not reset on a system reset. It is only reset by a power-on reset. Software in the reset handler or later, or by the DAP must enable the debug monitor."]
    #[inline(always)]
    pub fn mon_pend(&self) -> MonPendR {
        MonPendR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
When MON_EN = 1, this steps the core. When MON_EN = 0, this bit is ignored. This is the equivalent to DHCSR.C_STEP. Interrupts are only stepped according to the priority of the monitor and settings of PRIMASK, FAULTMASK, or BASEPRI."]
    #[inline(always)]
    pub fn mon_step(&self) -> MonStepR {
        MonStepR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
This enables the monitor to identify how it wakes up. This bit clears on a Core Reset. 0x0: Woken up by debug exception. 0x1: Woken up by MON_PEND"]
    #[inline(always)]
    pub fn mon_req(&self) -> MonReqR {
        MonReqR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved20(&self) -> Reserved20R {
        Reserved20R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
This bit must be set to 1 to enable use of the trace and debug blocks: DWT, ITM, ETM and TPIU. This enables control of power usage unless tracing is required. The application can enable this, for ITM use, or use by a debugger."]
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
Reset Vector Catch. Halt running system if Core reset occurs. Ignored when DHCSR.C_DEBUGEN is cleared."]
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
Debug trap on Memory Management faults. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn vc_mmerr(&mut self) -> VcMmerrW<DemcrSpec> {
        VcMmerrW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Debug trap on a UsageFault access to a Coprocessor. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn vc_nocperr(&mut self) -> VcNocperrW<DemcrSpec> {
        VcNocperrW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Debug trap on Usage Fault enabled checking errors. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn vc_chkerr(&mut self) -> VcChkerrW<DemcrSpec> {
        VcChkerrW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Debug trap on Usage Fault state errors. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn vc_staterr(&mut self) -> VcStaterrW<DemcrSpec> {
        VcStaterrW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Debug Trap on normal Bus error. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn vc_buserr(&mut self) -> VcBuserrW<DemcrSpec> {
        VcBuserrW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Debug trap on a fault occurring during an exception entry or return sequence. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn vc_interr(&mut self) -> VcInterrW<DemcrSpec> {
        VcInterrW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Debug trap on Hard Fault. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn vc_harderr(&mut self) -> VcHarderrW<DemcrSpec> {
        VcHarderrW::new(self, 10)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved11(&mut self) -> Reserved11W<DemcrSpec> {
        Reserved11W::new(self, 11)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable the debug monitor. When enabled, the System handler priority register controls its priority level. If disabled, then all debug events go to Hard fault. DHCSR.C_DEBUGEN overrides this bit. Vector catching is semi-synchronous. When a matching event is seen, a Halt is requested. Because the processor can only halt on an instruction boundary, it must wait until the next instruction boundary. As a result, it stops on the first instruction of the exception handler. However, two special cases exist when a vector catch has triggered: 1. If a fault is taken during vectoring, vector read or stack push error, the halt occurs on the corresponding fault handler, for the vector error or stack push. 2. If a late arriving interrupt comes in during vectoring, it is not taken. That is, an implementation that supports the late arrival optimization must suppress it in this case."]
    #[inline(always)]
    #[must_use]
    pub fn mon_en(&mut self) -> MonEnW<DemcrSpec> {
        MonEnW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pend the monitor to activate when priority permits. This can wake up the monitor through the AHB-AP port. It is the equivalent to DHCSR.C_HALT for Monitor debug. This register does not reset on a system reset. It is only reset by a power-on reset. Software in the reset handler or later, or by the DAP must enable the debug monitor."]
    #[inline(always)]
    #[must_use]
    pub fn mon_pend(&mut self) -> MonPendW<DemcrSpec> {
        MonPendW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
When MON_EN = 1, this steps the core. When MON_EN = 0, this bit is ignored. This is the equivalent to DHCSR.C_STEP. Interrupts are only stepped according to the priority of the monitor and settings of PRIMASK, FAULTMASK, or BASEPRI."]
    #[inline(always)]
    #[must_use]
    pub fn mon_step(&mut self) -> MonStepW<DemcrSpec> {
        MonStepW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
This enables the monitor to identify how it wakes up. This bit clears on a Core Reset. 0x0: Woken up by debug exception. 0x1: Woken up by MON_PEND"]
    #[inline(always)]
    #[must_use]
    pub fn mon_req(&mut self) -> MonReqW<DemcrSpec> {
        MonReqW::new(self, 19)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved20(&mut self) -> Reserved20W<DemcrSpec> {
        Reserved20W::new(self, 20)
    }
    #[doc = "Bit 24 - 24:24\\]
This bit must be set to 1 to enable use of the trace and debug blocks: DWT, ITM, ETM and TPIU. This enables control of power usage unless tracing is required. The application can enable this, for ITM use, or use by a debugger."]
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
#[doc = "Debug Exception and Monitor Control The purpose of this register is vector catching and debug monitor control. This register manages exception behavior under debug. Vector catching is only available to halting debug. The upper halfword is for monitor controls and the lower halfword is for halting exception support. This register is not reset on a system reset. This register is reset by a power-on reset. The fields MON_EN, MON_PEND, MON_STEP and MON_REQ are always cleared on a core reset. The debug monitor is enabled by software in the reset handler or later, or by the **AHB-AP** port. Vector catching is semi-synchronous. When a matching event is seen, a Halt is requested. Because the processor can only halt on an instruction boundary, it must wait until the next instruction boundary. As a result, it stops on the first instruction of the exception handler. However, two special cases exist when a vector catch has triggered: 1. If a fault is taken during a vector read or stack push error the halt occurs on the corresponding fault handler for the vector error or stack push. 2. If a late arriving interrupt detected during a vector read or stack push error it is not taken. That is, an implementation that supports the late arrival optimization must suppress it in this case.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`demcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`demcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
