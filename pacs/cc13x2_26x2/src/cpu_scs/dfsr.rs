#[doc = "Register `DFSR` reader"]
pub type R = crate::R<DfsrSpec>;
#[doc = "Register `DFSR` writer"]
pub type W = crate::W<DfsrSpec>;
#[doc = "Field `HALTED` reader - 0:0\\]
Halt request flag. The processor is halted on the next instruction. 0x0: No halt request 0x1: Halt requested by NVIC, including step"]
pub type HaltedR = crate::BitReader;
#[doc = "Field `HALTED` writer - 0:0\\]
Halt request flag. The processor is halted on the next instruction. 0x0: No halt request 0x1: Halt requested by NVIC, including step"]
pub type HaltedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKPT` reader - 1:1\\]
BKPT flag. The BKPT flag is set by a BKPT instruction in flash patch code, and also by normal code. Return PC points to breakpoint containing instruction. 0x0: No BKPT instruction execution 0x1: BKPT instruction execution"]
pub type BkptR = crate::BitReader;
#[doc = "Field `BKPT` writer - 1:1\\]
BKPT flag. The BKPT flag is set by a BKPT instruction in flash patch code, and also by normal code. Return PC points to breakpoint containing instruction. 0x0: No BKPT instruction execution 0x1: BKPT instruction execution"]
pub type BkptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DWTTRAP` reader - 2:2\\]
Data Watchpoint and Trace (DWT) flag. The processor stops at the current instruction or at the next instruction. 0x0: No DWT match 0x1: DWT match"]
pub type DwttrapR = crate::BitReader;
#[doc = "Field `DWTTRAP` writer - 2:2\\]
Data Watchpoint and Trace (DWT) flag. The processor stops at the current instruction or at the next instruction. 0x0: No DWT match 0x1: DWT match"]
pub type DwttrapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VCATCH` reader - 3:3\\]
Vector catch flag. When this flag is set, a flag in one of the local fault status registers is also set to indicate the type of fault. 0x0: No vector catch occurred 0x1: Vector catch occurred"]
pub type VcatchR = crate::BitReader;
#[doc = "Field `VCATCH` writer - 3:3\\]
Vector catch flag. When this flag is set, a flag in one of the local fault status registers is also set to indicate the type of fault. 0x0: No vector catch occurred 0x1: Vector catch occurred"]
pub type VcatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTERNAL` reader - 4:4\\]
External debug request flag. The processor stops on next instruction boundary. 0x0: External debug request signal not asserted 0x1: External debug request signal asserted"]
pub type ExternalR = crate::BitReader;
#[doc = "Field `EXTERNAL` writer - 4:4\\]
External debug request flag. The processor stops on next instruction boundary. 0x0: External debug request signal not asserted 0x1: External debug request signal asserted"]
pub type ExternalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED5` reader - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved5R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED5` writer - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved5W<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Halt request flag. The processor is halted on the next instruction. 0x0: No halt request 0x1: Halt requested by NVIC, including step"]
    #[inline(always)]
    pub fn halted(&self) -> HaltedR {
        HaltedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
BKPT flag. The BKPT flag is set by a BKPT instruction in flash patch code, and also by normal code. Return PC points to breakpoint containing instruction. 0x0: No BKPT instruction execution 0x1: BKPT instruction execution"]
    #[inline(always)]
    pub fn bkpt(&self) -> BkptR {
        BkptR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Data Watchpoint and Trace (DWT) flag. The processor stops at the current instruction or at the next instruction. 0x0: No DWT match 0x1: DWT match"]
    #[inline(always)]
    pub fn dwttrap(&self) -> DwttrapR {
        DwttrapR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Vector catch flag. When this flag is set, a flag in one of the local fault status registers is also set to indicate the type of fault. 0x0: No vector catch occurred 0x1: Vector catch occurred"]
    #[inline(always)]
    pub fn vcatch(&self) -> VcatchR {
        VcatchR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
External debug request flag. The processor stops on next instruction boundary. 0x0: External debug request signal not asserted 0x1: External debug request signal asserted"]
    #[inline(always)]
    pub fn external(&self) -> ExternalR {
        ExternalR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Halt request flag. The processor is halted on the next instruction. 0x0: No halt request 0x1: Halt requested by NVIC, including step"]
    #[inline(always)]
    #[must_use]
    pub fn halted(&mut self) -> HaltedW<DfsrSpec> {
        HaltedW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
BKPT flag. The BKPT flag is set by a BKPT instruction in flash patch code, and also by normal code. Return PC points to breakpoint containing instruction. 0x0: No BKPT instruction execution 0x1: BKPT instruction execution"]
    #[inline(always)]
    #[must_use]
    pub fn bkpt(&mut self) -> BkptW<DfsrSpec> {
        BkptW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Data Watchpoint and Trace (DWT) flag. The processor stops at the current instruction or at the next instruction. 0x0: No DWT match 0x1: DWT match"]
    #[inline(always)]
    #[must_use]
    pub fn dwttrap(&mut self) -> DwttrapW<DfsrSpec> {
        DwttrapW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Vector catch flag. When this flag is set, a flag in one of the local fault status registers is also set to indicate the type of fault. 0x0: No vector catch occurred 0x1: Vector catch occurred"]
    #[inline(always)]
    #[must_use]
    pub fn vcatch(&mut self) -> VcatchW<DfsrSpec> {
        VcatchW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
External debug request flag. The processor stops on next instruction boundary. 0x0: External debug request signal not asserted 0x1: External debug request signal asserted"]
    #[inline(always)]
    #[must_use]
    pub fn external(&mut self) -> ExternalW<DfsrSpec> {
        ExternalW::new(self, 4)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> Reserved5W<DfsrSpec> {
        Reserved5W::new(self, 5)
    }
}
#[doc = "Debug Fault Status This register is used to monitor external debug requests, vector catches, data watchpoint match, BKPT instruction execution, halt requests. Multiple flags in the Debug Fault Status Register can be set when multiple fault conditions occur. The register is read/write clear. This means that it can be read normally. Writing a 1 to a bit clears that bit. Note that these bits are not set unless the event is caught. This means that it causes a stop of some sort. If halting debug is enabled, these events stop the processor into debug. If debug is disabled and the debug monitor is enabled, then this becomes a debug monitor handler call, if priority permits. If debug and the monitor are both disabled, some of these events are Hard Faults, and some are ignored.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfsrSpec;
impl crate::RegisterSpec for DfsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsr::R`](R) reader structure"]
impl crate::Readable for DfsrSpec {}
#[doc = "`write(|w| ..)` method takes [`dfsr::W`](W) writer structure"]
impl crate::Writable for DfsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFSR to value 0"]
impl crate::Resettable for DfsrSpec {
    const RESET_VALUE: u32 = 0;
}
