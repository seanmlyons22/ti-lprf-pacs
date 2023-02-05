#[doc = "Register `DEMCR` reader"]
pub struct R(crate::R<DEMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEMCR` writer"]
pub struct W(crate::W<DEMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DEMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VC_CORERESET` reader - 0:0\\]
Reset Vector Catch. Halt running system if Core reset occurs. Ignored when DHCSR.C_DEBUGEN is cleared."]
pub type VC_CORERESET_R = crate::BitReader<bool>;
#[doc = "Field `VC_CORERESET` writer - 0:0\\]
Reset Vector Catch. Halt running system if Core reset occurs. Ignored when DHCSR.C_DEBUGEN is cleared."]
pub type VC_CORERESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED1` writer - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DEMCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `VC_MMERR` reader - 4:4\\]
Debug trap on Memory Management faults. Ignored when DHCSR.C_DEBUGEN is cleared."]
pub type VC_MMERR_R = crate::BitReader<bool>;
#[doc = "Field `VC_MMERR` writer - 4:4\\]
Debug trap on Memory Management faults. Ignored when DHCSR.C_DEBUGEN is cleared."]
pub type VC_MMERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, O>;
#[doc = "Field `VC_NOCPERR` reader - 5:5\\]
Debug trap on a UsageFault access to a Coprocessor. Ignored when DHCSR.C_DEBUGEN is cleared."]
pub type VC_NOCPERR_R = crate::BitReader<bool>;
#[doc = "Field `VC_NOCPERR` writer - 5:5\\]
Debug trap on a UsageFault access to a Coprocessor. Ignored when DHCSR.C_DEBUGEN is cleared."]
pub type VC_NOCPERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, O>;
#[doc = "Field `VC_CHKERR` reader - 6:6\\]
Debug trap on Usage Fault enabled checking errors. Ignored when DHCSR.C_DEBUGEN is cleared."]
pub type VC_CHKERR_R = crate::BitReader<bool>;
#[doc = "Field `VC_CHKERR` writer - 6:6\\]
Debug trap on Usage Fault enabled checking errors. Ignored when DHCSR.C_DEBUGEN is cleared."]
pub type VC_CHKERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, O>;
#[doc = "Field `VC_STATERR` reader - 7:7\\]
Debug trap on Usage Fault state errors. Ignored when DHCSR.C_DEBUGEN is cleared."]
pub type VC_STATERR_R = crate::BitReader<bool>;
#[doc = "Field `VC_STATERR` writer - 7:7\\]
Debug trap on Usage Fault state errors. Ignored when DHCSR.C_DEBUGEN is cleared."]
pub type VC_STATERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, O>;
#[doc = "Field `VC_BUSERR` reader - 8:8\\]
Debug Trap on normal Bus error. Ignored when DHCSR.C_DEBUGEN is cleared."]
pub type VC_BUSERR_R = crate::BitReader<bool>;
#[doc = "Field `VC_BUSERR` writer - 8:8\\]
Debug Trap on normal Bus error. Ignored when DHCSR.C_DEBUGEN is cleared."]
pub type VC_BUSERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, O>;
#[doc = "Field `VC_INTERR` reader - 9:9\\]
Debug trap on a fault occurring during an exception entry or return sequence. Ignored when DHCSR.C_DEBUGEN is cleared."]
pub type VC_INTERR_R = crate::BitReader<bool>;
#[doc = "Field `VC_INTERR` writer - 9:9\\]
Debug trap on a fault occurring during an exception entry or return sequence. Ignored when DHCSR.C_DEBUGEN is cleared."]
pub type VC_INTERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, O>;
#[doc = "Field `VC_HARDERR` reader - 10:10\\]
Debug trap on Hard Fault. Ignored when DHCSR.C_DEBUGEN is cleared."]
pub type VC_HARDERR_R = crate::BitReader<bool>;
#[doc = "Field `VC_HARDERR` writer - 10:10\\]
Debug trap on Hard Fault. Ignored when DHCSR.C_DEBUGEN is cleared."]
pub type VC_HARDERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, O>;
#[doc = "Field `RESERVED11` reader - 15:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED11` writer - 15:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DEMCR_SPEC, u8, u8, 5, O>;
#[doc = "Field `MON_EN` reader - 16:16\\]
Enable the debug monitor. When enabled, the System handler priority register controls its priority level. If disabled, then all debug events go to Hard fault. DHCSR.C_DEBUGEN overrides this bit. Vector catching is semi-synchronous. When a matching event is seen, a Halt is requested. Because the processor can only halt on an instruction boundary, it must wait until the next instruction boundary. As a result, it stops on the first instruction of the exception handler. However, two special cases exist when a vector catch has triggered: 1. If a fault is taken during vectoring, vector read or stack push error, the halt occurs on the corresponding fault handler, for the vector error or stack push. 2. If a late arriving interrupt comes in during vectoring, it is not taken. That is, an implementation that supports the late arrival optimization must suppress it in this case."]
pub type MON_EN_R = crate::BitReader<bool>;
#[doc = "Field `MON_EN` writer - 16:16\\]
Enable the debug monitor. When enabled, the System handler priority register controls its priority level. If disabled, then all debug events go to Hard fault. DHCSR.C_DEBUGEN overrides this bit. Vector catching is semi-synchronous. When a matching event is seen, a Halt is requested. Because the processor can only halt on an instruction boundary, it must wait until the next instruction boundary. As a result, it stops on the first instruction of the exception handler. However, two special cases exist when a vector catch has triggered: 1. If a fault is taken during vectoring, vector read or stack push error, the halt occurs on the corresponding fault handler, for the vector error or stack push. 2. If a late arriving interrupt comes in during vectoring, it is not taken. That is, an implementation that supports the late arrival optimization must suppress it in this case."]
pub type MON_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, O>;
#[doc = "Field `MON_PEND` reader - 17:17\\]
Pend the monitor to activate when priority permits. This can wake up the monitor through the AHB-AP port. It is the equivalent to DHCSR.C_HALT for Monitor debug. This register does not reset on a system reset. It is only reset by a power-on reset. Software in the reset handler or later, or by the DAP must enable the debug monitor."]
pub type MON_PEND_R = crate::BitReader<bool>;
#[doc = "Field `MON_PEND` writer - 17:17\\]
Pend the monitor to activate when priority permits. This can wake up the monitor through the AHB-AP port. It is the equivalent to DHCSR.C_HALT for Monitor debug. This register does not reset on a system reset. It is only reset by a power-on reset. Software in the reset handler or later, or by the DAP must enable the debug monitor."]
pub type MON_PEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, O>;
#[doc = "Field `MON_STEP` reader - 18:18\\]
When MON_EN = 1, this steps the core. When MON_EN = 0, this bit is ignored. This is the equivalent to DHCSR.C_STEP. Interrupts are only stepped according to the priority of the monitor and settings of PRIMASK, FAULTMASK, or BASEPRI."]
pub type MON_STEP_R = crate::BitReader<bool>;
#[doc = "Field `MON_STEP` writer - 18:18\\]
When MON_EN = 1, this steps the core. When MON_EN = 0, this bit is ignored. This is the equivalent to DHCSR.C_STEP. Interrupts are only stepped according to the priority of the monitor and settings of PRIMASK, FAULTMASK, or BASEPRI."]
pub type MON_STEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, O>;
#[doc = "Field `MON_REQ` reader - 19:19\\]
This enables the monitor to identify how it wakes up. This bit clears on a Core Reset. 0x0: Woken up by debug exception. 0x1: Woken up by MON_PEND"]
pub type MON_REQ_R = crate::BitReader<bool>;
#[doc = "Field `MON_REQ` writer - 19:19\\]
This enables the monitor to identify how it wakes up. This bit clears on a Core Reset. 0x0: Woken up by debug exception. 0x1: Woken up by MON_PEND"]
pub type MON_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, O>;
#[doc = "Field `RESERVED20` reader - 23:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED20_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED20` writer - 23:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED20_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DEMCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `TRCENA` reader - 24:24\\]
This bit must be set to 1 to enable use of the trace and debug blocks: DWT, ITM, ETM and TPIU. This enables control of power usage unless tracing is required. The application can enable this, for ITM use, or use by a debugger."]
pub type TRCENA_R = crate::BitReader<bool>;
#[doc = "Field `TRCENA` writer - 24:24\\]
This bit must be set to 1 to enable use of the trace and debug blocks: DWT, ITM, ETM and TPIU. This enables control of power usage unless tracing is required. The application can enable this, for ITM use, or use by a debugger."]
pub type TRCENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, O>;
#[doc = "Field `RESERVED25` reader - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED25_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED25` writer - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED25_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DEMCR_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Reset Vector Catch. Halt running system if Core reset occurs. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    pub fn vc_corereset(&self) -> VC_CORERESET_R {
        VC_CORERESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Debug trap on Memory Management faults. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    pub fn vc_mmerr(&self) -> VC_MMERR_R {
        VC_MMERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Debug trap on a UsageFault access to a Coprocessor. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    pub fn vc_nocperr(&self) -> VC_NOCPERR_R {
        VC_NOCPERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Debug trap on Usage Fault enabled checking errors. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    pub fn vc_chkerr(&self) -> VC_CHKERR_R {
        VC_CHKERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Debug trap on Usage Fault state errors. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    pub fn vc_staterr(&self) -> VC_STATERR_R {
        VC_STATERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Debug Trap on normal Bus error. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    pub fn vc_buserr(&self) -> VC_BUSERR_R {
        VC_BUSERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Debug trap on a fault occurring during an exception entry or return sequence. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    pub fn vc_interr(&self) -> VC_INTERR_R {
        VC_INTERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Debug trap on Hard Fault. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    pub fn vc_harderr(&self) -> VC_HARDERR_R {
        VC_HARDERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> RESERVED11_R {
        RESERVED11_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable the debug monitor. When enabled, the System handler priority register controls its priority level. If disabled, then all debug events go to Hard fault. DHCSR.C_DEBUGEN overrides this bit. Vector catching is semi-synchronous. When a matching event is seen, a Halt is requested. Because the processor can only halt on an instruction boundary, it must wait until the next instruction boundary. As a result, it stops on the first instruction of the exception handler. However, two special cases exist when a vector catch has triggered: 1. If a fault is taken during vectoring, vector read or stack push error, the halt occurs on the corresponding fault handler, for the vector error or stack push. 2. If a late arriving interrupt comes in during vectoring, it is not taken. That is, an implementation that supports the late arrival optimization must suppress it in this case."]
    #[inline(always)]
    pub fn mon_en(&self) -> MON_EN_R {
        MON_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pend the monitor to activate when priority permits. This can wake up the monitor through the AHB-AP port. It is the equivalent to DHCSR.C_HALT for Monitor debug. This register does not reset on a system reset. It is only reset by a power-on reset. Software in the reset handler or later, or by the DAP must enable the debug monitor."]
    #[inline(always)]
    pub fn mon_pend(&self) -> MON_PEND_R {
        MON_PEND_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
When MON_EN = 1, this steps the core. When MON_EN = 0, this bit is ignored. This is the equivalent to DHCSR.C_STEP. Interrupts are only stepped according to the priority of the monitor and settings of PRIMASK, FAULTMASK, or BASEPRI."]
    #[inline(always)]
    pub fn mon_step(&self) -> MON_STEP_R {
        MON_STEP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
This enables the monitor to identify how it wakes up. This bit clears on a Core Reset. 0x0: Woken up by debug exception. 0x1: Woken up by MON_PEND"]
    #[inline(always)]
    pub fn mon_req(&self) -> MON_REQ_R {
        MON_REQ_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved20(&self) -> RESERVED20_R {
        RESERVED20_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
This bit must be set to 1 to enable use of the trace and debug blocks: DWT, ITM, ETM and TPIU. This enables control of power usage unless tracing is required. The application can enable this, for ITM use, or use by a debugger."]
    #[inline(always)]
    pub fn trcena(&self) -> TRCENA_R {
        TRCENA_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved25(&self) -> RESERVED25_R {
        RESERVED25_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Reset Vector Catch. Halt running system if Core reset occurs. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn vc_corereset(&mut self) -> VC_CORERESET_W<0> {
        VC_CORERESET_W::new(self)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Debug trap on Memory Management faults. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn vc_mmerr(&mut self) -> VC_MMERR_W<4> {
        VC_MMERR_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Debug trap on a UsageFault access to a Coprocessor. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn vc_nocperr(&mut self) -> VC_NOCPERR_W<5> {
        VC_NOCPERR_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Debug trap on Usage Fault enabled checking errors. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn vc_chkerr(&mut self) -> VC_CHKERR_W<6> {
        VC_CHKERR_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Debug trap on Usage Fault state errors. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn vc_staterr(&mut self) -> VC_STATERR_W<7> {
        VC_STATERR_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Debug Trap on normal Bus error. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn vc_buserr(&mut self) -> VC_BUSERR_W<8> {
        VC_BUSERR_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Debug trap on a fault occurring during an exception entry or return sequence. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn vc_interr(&mut self) -> VC_INTERR_W<9> {
        VC_INTERR_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
Debug trap on Hard Fault. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn vc_harderr(&mut self) -> VC_HARDERR_W<10> {
        VC_HARDERR_W::new(self)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved11(&mut self) -> RESERVED11_W<11> {
        RESERVED11_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable the debug monitor. When enabled, the System handler priority register controls its priority level. If disabled, then all debug events go to Hard fault. DHCSR.C_DEBUGEN overrides this bit. Vector catching is semi-synchronous. When a matching event is seen, a Halt is requested. Because the processor can only halt on an instruction boundary, it must wait until the next instruction boundary. As a result, it stops on the first instruction of the exception handler. However, two special cases exist when a vector catch has triggered: 1. If a fault is taken during vectoring, vector read or stack push error, the halt occurs on the corresponding fault handler, for the vector error or stack push. 2. If a late arriving interrupt comes in during vectoring, it is not taken. That is, an implementation that supports the late arrival optimization must suppress it in this case."]
    #[inline(always)]
    #[must_use]
    pub fn mon_en(&mut self) -> MON_EN_W<16> {
        MON_EN_W::new(self)
    }
    #[doc = "Bit 17 - 17:17\\]
Pend the monitor to activate when priority permits. This can wake up the monitor through the AHB-AP port. It is the equivalent to DHCSR.C_HALT for Monitor debug. This register does not reset on a system reset. It is only reset by a power-on reset. Software in the reset handler or later, or by the DAP must enable the debug monitor."]
    #[inline(always)]
    #[must_use]
    pub fn mon_pend(&mut self) -> MON_PEND_W<17> {
        MON_PEND_W::new(self)
    }
    #[doc = "Bit 18 - 18:18\\]
When MON_EN = 1, this steps the core. When MON_EN = 0, this bit is ignored. This is the equivalent to DHCSR.C_STEP. Interrupts are only stepped according to the priority of the monitor and settings of PRIMASK, FAULTMASK, or BASEPRI."]
    #[inline(always)]
    #[must_use]
    pub fn mon_step(&mut self) -> MON_STEP_W<18> {
        MON_STEP_W::new(self)
    }
    #[doc = "Bit 19 - 19:19\\]
This enables the monitor to identify how it wakes up. This bit clears on a Core Reset. 0x0: Woken up by debug exception. 0x1: Woken up by MON_PEND"]
    #[inline(always)]
    #[must_use]
    pub fn mon_req(&mut self) -> MON_REQ_W<19> {
        MON_REQ_W::new(self)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved20(&mut self) -> RESERVED20_W<20> {
        RESERVED20_W::new(self)
    }
    #[doc = "Bit 24 - 24:24\\]
This bit must be set to 1 to enable use of the trace and debug blocks: DWT, ITM, ETM and TPIU. This enables control of power usage unless tracing is required. The application can enable this, for ITM use, or use by a debugger."]
    #[inline(always)]
    #[must_use]
    pub fn trcena(&mut self) -> TRCENA_W<24> {
        TRCENA_W::new(self)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved25(&mut self) -> RESERVED25_W<25> {
        RESERVED25_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug Exception and Monitor Control The purpose of this register is vector catching and debug monitor control. This register manages exception behavior under debug. Vector catching is only available to halting debug. The upper halfword is for monitor controls and the lower halfword is for halting exception support. This register is not reset on a system reset. This register is reset by a power-on reset. The fields MON_EN, MON_PEND, MON_STEP and MON_REQ are always cleared on a core reset. The debug monitor is enabled by software in the reset handler or later, or by the **AHB-AP** port. Vector catching is semi-synchronous. When a matching event is seen, a Halt is requested. Because the processor can only halt on an instruction boundary, it must wait until the next instruction boundary. As a result, it stops on the first instruction of the exception handler. However, two special cases exist when a vector catch has triggered: 1. If a fault is taken during a vector read or stack push error the halt occurs on the corresponding fault handler for the vector error or stack push. 2. If a late arriving interrupt detected during a vector read or stack push error it is not taken. That is, an implementation that supports the late arrival optimization must suppress it in this case.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [demcr](index.html) module"]
pub struct DEMCR_SPEC;
impl crate::RegisterSpec for DEMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [demcr::R](R) reader structure"]
impl crate::Readable for DEMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [demcr::W](W) writer structure"]
impl crate::Writable for DEMCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEMCR to value 0"]
impl crate::Resettable for DEMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
