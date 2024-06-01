#[doc = "Register `HFSR` reader"]
pub type R = crate::R<HfsrSpec>;
#[doc = "Register `HFSR` writer"]
pub type W = crate::W<HfsrSpec>;
#[doc = "Field `RESERVED0` reader - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::BitReader;
#[doc = "Field `RESERVED0` writer - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VECTTBL` reader - 1:1\\]
This bit is set if there is a fault because of vector table read on exception processing (Bus Fault). This case is always a Hard Fault. The return PC points to the pre-empted instruction."]
pub type VecttblR = crate::BitReader;
#[doc = "Field `VECTTBL` writer - 1:1\\]
This bit is set if there is a fault because of vector table read on exception processing (Bus Fault). This case is always a Hard Fault. The return PC points to the pre-empted instruction."]
pub type VecttblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 29:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED2` writer - 29:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
#[doc = "Field `FORCED` reader - 30:30\\]
Hard Fault activated because a Configurable Fault was received and cannot activate because of priority or because the Configurable Fault is disabled. The Hard Fault handler then has to read the other fault status registers to determine cause."]
pub type ForcedR = crate::BitReader;
#[doc = "Field `FORCED` writer - 30:30\\]
Hard Fault activated because a Configurable Fault was received and cannot activate because of priority or because the Configurable Fault is disabled. The Hard Fault handler then has to read the other fault status registers to determine cause."]
pub type ForcedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEBUGEVT` reader - 31:31\\]
This bit is set if there is a fault related to debug. This is only possible when halting debug is not enabled. For monitor enabled debug, it only happens for BKPT when the current priority is higher than the monitor. When both halting and monitor debug are disabled, it only happens for debug events that are not ignored (minimally, BKPT). The Debug Fault Status Register is updated."]
pub type DebugevtR = crate::BitReader;
#[doc = "Field `DEBUGEVT` writer - 31:31\\]
This bit is set if there is a fault related to debug. This is only possible when halting debug is not enabled. For monitor enabled debug, it only happens for BKPT when the current priority is higher than the monitor. When both halting and monitor debug are disabled, it only happens for debug events that are not ignored (minimally, BKPT). The Debug Fault Status Register is updated."]
pub type DebugevtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit is set if there is a fault because of vector table read on exception processing (Bus Fault). This case is always a Hard Fault. The return PC points to the pre-empted instruction."]
    #[inline(always)]
    pub fn vecttbl(&self) -> VecttblR {
        VecttblR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:29 - 29:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x0fff_ffff)
    }
    #[doc = "Bit 30 - 30:30\\]
Hard Fault activated because a Configurable Fault was received and cannot activate because of priority or because the Configurable Fault is disabled. The Hard Fault handler then has to read the other fault status registers to determine cause."]
    #[inline(always)]
    pub fn forced(&self) -> ForcedR {
        ForcedR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
This bit is set if there is a fault related to debug. This is only possible when halting debug is not enabled. For monitor enabled debug, it only happens for BKPT when the current priority is higher than the monitor. When both halting and monitor debug are disabled, it only happens for debug events that are not ignored (minimally, BKPT). The Debug Fault Status Register is updated."]
    #[inline(always)]
    pub fn debugevt(&self) -> DebugevtR {
        DebugevtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<HfsrSpec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit is set if there is a fault because of vector table read on exception processing (Bus Fault). This case is always a Hard Fault. The return PC points to the pre-empted instruction."]
    #[inline(always)]
    #[must_use]
    pub fn vecttbl(&mut self) -> VecttblW<HfsrSpec> {
        VecttblW::new(self, 1)
    }
    #[doc = "Bits 2:29 - 29:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<HfsrSpec> {
        Reserved2W::new(self, 2)
    }
    #[doc = "Bit 30 - 30:30\\]
Hard Fault activated because a Configurable Fault was received and cannot activate because of priority or because the Configurable Fault is disabled. The Hard Fault handler then has to read the other fault status registers to determine cause."]
    #[inline(always)]
    #[must_use]
    pub fn forced(&mut self) -> ForcedW<HfsrSpec> {
        ForcedW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
This bit is set if there is a fault related to debug. This is only possible when halting debug is not enabled. For monitor enabled debug, it only happens for BKPT when the current priority is higher than the monitor. When both halting and monitor debug are disabled, it only happens for debug events that are not ignored (minimally, BKPT). The Debug Fault Status Register is updated."]
    #[inline(always)]
    #[must_use]
    pub fn debugevt(&mut self) -> DebugevtW<HfsrSpec> {
        DebugevtW::new(self, 31)
    }
}
#[doc = "Hard Fault Status This register is used to obtain information about events that activate the Hard Fault handler. This register is a write-clear register. This means that writing a 1 to a bit clears that bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfsrSpec;
impl crate::RegisterSpec for HfsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfsr::R`](R) reader structure"]
impl crate::Readable for HfsrSpec {}
#[doc = "`write(|w| ..)` method takes [`hfsr::W`](W) writer structure"]
impl crate::Writable for HfsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HFSR to value 0"]
impl crate::Resettable for HfsrSpec {
    const RESET_VALUE: u32 = 0;
}
