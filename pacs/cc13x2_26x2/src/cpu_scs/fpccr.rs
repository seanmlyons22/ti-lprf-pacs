#[doc = "Register `FPCCR` reader"]
pub type R = crate::R<FpccrSpec>;
#[doc = "Register `FPCCR` writer"]
pub type W = crate::W<FpccrSpec>;
#[doc = "Field `LSPACT` reader - 0:0\\]
Indicates whether Lazy preservation of the FP state is active: 0: Lazy state preservation is not active. 1: Lazy state preservation is active. floating-point stack frame has been allocated but saving state to it has been deferred."]
pub type LspactR = crate::BitReader;
#[doc = "Field `LSPACT` writer - 0:0\\]
Indicates whether Lazy preservation of the FP state is active: 0: Lazy state preservation is not active. 1: Lazy state preservation is active. floating-point stack frame has been allocated but saving state to it has been deferred."]
pub type LspactW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USER` reader - 1:1\\]
Indicates the privilege level of the software executing was User (Unpriviledged) when the processor allocated the FP stack frame: 0: Privilege level was not user when the floating-point stack frame was allocated. 1: Privilege level was user when the floating-point stack frame was allocated."]
pub type UserR = crate::BitReader;
#[doc = "Field `USER` writer - 1:1\\]
Indicates the privilege level of the software executing was User (Unpriviledged) when the processor allocated the FP stack frame: 0: Privilege level was not user when the floating-point stack frame was allocated. 1: Privilege level was user when the floating-point stack frame was allocated."]
pub type UserW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `RESERVED2` writer - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THREAD` reader - 3:3\\]
Indicates the processor mode was Thread when it allocated the FP stack frame. 0: Mode was not Thread Mode when the floating-point stack frame was allocated. 1: Mode was Thread Mode when the floating-point stack frame was allocated."]
pub type ThreadR = crate::BitReader;
#[doc = "Field `THREAD` writer - 3:3\\]
Indicates the processor mode was Thread when it allocated the FP stack frame. 0: Mode was not Thread Mode when the floating-point stack frame was allocated. 1: Mode was Thread Mode when the floating-point stack frame was allocated."]
pub type ThreadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFRDY` reader - 4:4\\]
Indicates whether the software executing when the processor allocated the FP stack frame was able to set the HardFault exception to pending. 0: Priority did not permit setting the HardFault handler to the pending state when the floating-point stack frame was allocated. 1: Priority permitted setting the HardFault handler to the pending state when the floating-point stack frame was allocated."]
pub type HfrdyR = crate::BitReader;
#[doc = "Field `HFRDY` writer - 4:4\\]
Indicates whether the software executing when the processor allocated the FP stack frame was able to set the HardFault exception to pending. 0: Priority did not permit setting the HardFault handler to the pending state when the floating-point stack frame was allocated. 1: Priority permitted setting the HardFault handler to the pending state when the floating-point stack frame was allocated."]
pub type HfrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMRDY` reader - 5:5\\]
Indicates whether the software executing when the processor allocated the FP stack frame was able to set the MemManage exception to pending. 0: MemManage is disabled or priority did not permit setting the MemManage handler to the pending state when the floating-point stack frame was allocated. 1: MemManage is enabled and priority permitted setting the MemManage handler to the pending state when the floating-point stack frame was allocated."]
pub type MmrdyR = crate::BitReader;
#[doc = "Field `MMRDY` writer - 5:5\\]
Indicates whether the software executing when the processor allocated the FP stack frame was able to set the MemManage exception to pending. 0: MemManage is disabled or priority did not permit setting the MemManage handler to the pending state when the floating-point stack frame was allocated. 1: MemManage is enabled and priority permitted setting the MemManage handler to the pending state when the floating-point stack frame was allocated."]
pub type MmrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BFRDY` reader - 6:6\\]
Indicates whether the software executing when the processor allocated the FP stack frame was able to set the BusFault exception to pending. 0: BusFault is disabled or priority did not permit setting the BusFault handler to the pending state when the floating-point stack frame was allocated. 1: BusFault is enabled and priority permitted setting the BusFault handler to the pending state when the floating-point stack frame was allocated."]
pub type BfrdyR = crate::BitReader;
#[doc = "Field `BFRDY` writer - 6:6\\]
Indicates whether the software executing when the processor allocated the FP stack frame was able to set the BusFault exception to pending. 0: BusFault is disabled or priority did not permit setting the BusFault handler to the pending state when the floating-point stack frame was allocated. 1: BusFault is enabled and priority permitted setting the BusFault handler to the pending state when the floating-point stack frame was allocated."]
pub type BfrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED7` reader - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `RESERVED7` writer - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MONRDY` reader - 8:8\\]
Indicates whether the the software executing when the processor allocated the FP stack frame was able to set the DebugMonitor exception to pending. 0: DebugMonitor is disabled or priority did not permit setting DEMCR.MON_PEND when the floating-point stack frame was allocated. 1: DebugMonitor is enabled and priority permits setting DEMCR.MON_PEND when the floating-point stack frame was allocated."]
pub type MonrdyR = crate::BitReader;
#[doc = "Field `MONRDY` writer - 8:8\\]
Indicates whether the the software executing when the processor allocated the FP stack frame was able to set the DebugMonitor exception to pending. 0: DebugMonitor is disabled or priority did not permit setting DEMCR.MON_PEND when the floating-point stack frame was allocated. 1: DebugMonitor is enabled and priority permits setting DEMCR.MON_PEND when the floating-point stack frame was allocated."]
pub type MonrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED9` reader - 29:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED9` writer - 29:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
#[doc = "Field `LSPEN` reader - 30:30\\]
Lazy State Preservation enable. Lazy state preservation is when the processor performs a context save, space on the stack is reserved for the floating point state but it is not stacked until the new context performs a floating point operation. 0: Disable automatic lazy state preservation for floating-point context. 1: Enable automatic lazy state preservation for floating-point context."]
pub type LspenR = crate::BitReader;
#[doc = "Field `LSPEN` writer - 30:30\\]
Lazy State Preservation enable. Lazy state preservation is when the processor performs a context save, space on the stack is reserved for the floating point state but it is not stacked until the new context performs a floating point operation. 0: Disable automatic lazy state preservation for floating-point context. 1: Enable automatic lazy state preservation for floating-point context."]
pub type LspenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASPEN` reader - 31:31\\]
Automatic State Preservation enable. When this bit is set is will cause bit \\[2\\]
of the Special CONTROL register to be set (FPCA) on execution of a floating point instruction which results in the floating point state automatically being preserved on exception entry."]
pub type AspenR = crate::BitReader;
#[doc = "Field `ASPEN` writer - 31:31\\]
Automatic State Preservation enable. When this bit is set is will cause bit \\[2\\]
of the Special CONTROL register to be set (FPCA) on execution of a floating point instruction which results in the floating point state automatically being preserved on exception entry."]
pub type AspenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates whether Lazy preservation of the FP state is active: 0: Lazy state preservation is not active. 1: Lazy state preservation is active. floating-point stack frame has been allocated but saving state to it has been deferred."]
    #[inline(always)]
    pub fn lspact(&self) -> LspactR {
        LspactR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates the privilege level of the software executing was User (Unpriviledged) when the processor allocated the FP stack frame: 0: Privilege level was not user when the floating-point stack frame was allocated. 1: Privilege level was user when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn user(&self) -> UserR {
        UserR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Indicates the processor mode was Thread when it allocated the FP stack frame. 0: Mode was not Thread Mode when the floating-point stack frame was allocated. 1: Mode was Thread Mode when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn thread(&self) -> ThreadR {
        ThreadR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Indicates whether the software executing when the processor allocated the FP stack frame was able to set the HardFault exception to pending. 0: Priority did not permit setting the HardFault handler to the pending state when the floating-point stack frame was allocated. 1: Priority permitted setting the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn hfrdy(&self) -> HfrdyR {
        HfrdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Indicates whether the software executing when the processor allocated the FP stack frame was able to set the MemManage exception to pending. 0: MemManage is disabled or priority did not permit setting the MemManage handler to the pending state when the floating-point stack frame was allocated. 1: MemManage is enabled and priority permitted setting the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn mmrdy(&self) -> MmrdyR {
        MmrdyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Indicates whether the software executing when the processor allocated the FP stack frame was able to set the BusFault exception to pending. 0: BusFault is disabled or priority did not permit setting the BusFault handler to the pending state when the floating-point stack frame was allocated. 1: BusFault is enabled and priority permitted setting the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn bfrdy(&self) -> BfrdyR {
        BfrdyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Indicates whether the the software executing when the processor allocated the FP stack frame was able to set the DebugMonitor exception to pending. 0: DebugMonitor is disabled or priority did not permit setting DEMCR.MON_PEND when the floating-point stack frame was allocated. 1: DebugMonitor is enabled and priority permits setting DEMCR.MON_PEND when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn monrdy(&self) -> MonrdyR {
        MonrdyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:29 - 29:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new((self.bits >> 9) & 0x001f_ffff)
    }
    #[doc = "Bit 30 - 30:30\\]
Lazy State Preservation enable. Lazy state preservation is when the processor performs a context save, space on the stack is reserved for the floating point state but it is not stacked until the new context performs a floating point operation. 0: Disable automatic lazy state preservation for floating-point context. 1: Enable automatic lazy state preservation for floating-point context."]
    #[inline(always)]
    pub fn lspen(&self) -> LspenR {
        LspenR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Automatic State Preservation enable. When this bit is set is will cause bit \\[2\\]
of the Special CONTROL register to be set (FPCA) on execution of a floating point instruction which results in the floating point state automatically being preserved on exception entry."]
    #[inline(always)]
    pub fn aspen(&self) -> AspenR {
        AspenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates whether Lazy preservation of the FP state is active: 0: Lazy state preservation is not active. 1: Lazy state preservation is active. floating-point stack frame has been allocated but saving state to it has been deferred."]
    #[inline(always)]
    #[must_use]
    pub fn lspact(&mut self) -> LspactW<FpccrSpec> {
        LspactW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates the privilege level of the software executing was User (Unpriviledged) when the processor allocated the FP stack frame: 0: Privilege level was not user when the floating-point stack frame was allocated. 1: Privilege level was user when the floating-point stack frame was allocated."]
    #[inline(always)]
    #[must_use]
    pub fn user(&mut self) -> UserW<FpccrSpec> {
        UserW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<FpccrSpec> {
        Reserved2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Indicates the processor mode was Thread when it allocated the FP stack frame. 0: Mode was not Thread Mode when the floating-point stack frame was allocated. 1: Mode was Thread Mode when the floating-point stack frame was allocated."]
    #[inline(always)]
    #[must_use]
    pub fn thread(&mut self) -> ThreadW<FpccrSpec> {
        ThreadW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Indicates whether the software executing when the processor allocated the FP stack frame was able to set the HardFault exception to pending. 0: Priority did not permit setting the HardFault handler to the pending state when the floating-point stack frame was allocated. 1: Priority permitted setting the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    #[must_use]
    pub fn hfrdy(&mut self) -> HfrdyW<FpccrSpec> {
        HfrdyW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Indicates whether the software executing when the processor allocated the FP stack frame was able to set the MemManage exception to pending. 0: MemManage is disabled or priority did not permit setting the MemManage handler to the pending state when the floating-point stack frame was allocated. 1: MemManage is enabled and priority permitted setting the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    #[must_use]
    pub fn mmrdy(&mut self) -> MmrdyW<FpccrSpec> {
        MmrdyW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Indicates whether the software executing when the processor allocated the FP stack frame was able to set the BusFault exception to pending. 0: BusFault is disabled or priority did not permit setting the BusFault handler to the pending state when the floating-point stack frame was allocated. 1: BusFault is enabled and priority permitted setting the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    #[must_use]
    pub fn bfrdy(&mut self) -> BfrdyW<FpccrSpec> {
        BfrdyW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> Reserved7W<FpccrSpec> {
        Reserved7W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Indicates whether the the software executing when the processor allocated the FP stack frame was able to set the DebugMonitor exception to pending. 0: DebugMonitor is disabled or priority did not permit setting DEMCR.MON_PEND when the floating-point stack frame was allocated. 1: DebugMonitor is enabled and priority permits setting DEMCR.MON_PEND when the floating-point stack frame was allocated."]
    #[inline(always)]
    #[must_use]
    pub fn monrdy(&mut self) -> MonrdyW<FpccrSpec> {
        MonrdyW::new(self, 8)
    }
    #[doc = "Bits 9:29 - 29:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> Reserved9W<FpccrSpec> {
        Reserved9W::new(self, 9)
    }
    #[doc = "Bit 30 - 30:30\\]
Lazy State Preservation enable. Lazy state preservation is when the processor performs a context save, space on the stack is reserved for the floating point state but it is not stacked until the new context performs a floating point operation. 0: Disable automatic lazy state preservation for floating-point context. 1: Enable automatic lazy state preservation for floating-point context."]
    #[inline(always)]
    #[must_use]
    pub fn lspen(&mut self) -> LspenW<FpccrSpec> {
        LspenW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Automatic State Preservation enable. When this bit is set is will cause bit \\[2\\]
of the Special CONTROL register to be set (FPCA) on execution of a floating point instruction which results in the floating point state automatically being preserved on exception entry."]
    #[inline(always)]
    #[must_use]
    pub fn aspen(&mut self) -> AspenW<FpccrSpec> {
        AspenW::new(self, 31)
    }
}
#[doc = "Floating Point Context Control This register holds control data for the floating-point unit. Accessible only by privileged software.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FpccrSpec;
impl crate::RegisterSpec for FpccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpccr::R`](R) reader structure"]
impl crate::Readable for FpccrSpec {}
#[doc = "`write(|w| ..)` method takes [`fpccr::W`](W) writer structure"]
impl crate::Writable for FpccrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FPCCR to value 0xc000_0000"]
impl crate::Resettable for FpccrSpec {
    const RESET_VALUE: u32 = 0xc000_0000;
}
