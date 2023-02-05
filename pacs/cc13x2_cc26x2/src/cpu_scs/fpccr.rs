#[doc = "Register `FPCCR` reader"]
pub struct R(crate::R<FPCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPCCR` writer"]
pub struct W(crate::W<FPCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPCCR_SPEC>;
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
impl From<crate::W<FPCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSPACT` reader - 0:0\\]
Indicates whether Lazy preservation of the FP state is active: 0: Lazy state preservation is not active. 1: Lazy state preservation is active. floating-point stack frame has been allocated but saving state to it has been deferred."]
pub type LSPACT_R = crate::BitReader<bool>;
#[doc = "Field `LSPACT` writer - 0:0\\]
Indicates whether Lazy preservation of the FP state is active: 0: Lazy state preservation is not active. 1: Lazy state preservation is active. floating-point stack frame has been allocated but saving state to it has been deferred."]
pub type LSPACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPCCR_SPEC, bool, O>;
#[doc = "Field `USER` reader - 1:1\\]
Indicates the privilege level of the software executing was User (Unpriviledged) when the processor allocated the FP stack frame: 0: Privilege level was not user when the floating-point stack frame was allocated. 1: Privilege level was user when the floating-point stack frame was allocated."]
pub type USER_R = crate::BitReader<bool>;
#[doc = "Field `USER` writer - 1:1\\]
Indicates the privilege level of the software executing was User (Unpriviledged) when the processor allocated the FP stack frame: 0: Privilege level was not user when the floating-point stack frame was allocated. 1: Privilege level was user when the floating-point stack frame was allocated."]
pub type USER_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPCCR_SPEC, bool, O>;
#[doc = "Field `RESERVED2` reader - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED2` writer - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPCCR_SPEC, bool, O>;
#[doc = "Field `THREAD` reader - 3:3\\]
Indicates the processor mode was Thread when it allocated the FP stack frame. 0: Mode was not Thread Mode when the floating-point stack frame was allocated. 1: Mode was Thread Mode when the floating-point stack frame was allocated."]
pub type THREAD_R = crate::BitReader<bool>;
#[doc = "Field `THREAD` writer - 3:3\\]
Indicates the processor mode was Thread when it allocated the FP stack frame. 0: Mode was not Thread Mode when the floating-point stack frame was allocated. 1: Mode was Thread Mode when the floating-point stack frame was allocated."]
pub type THREAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPCCR_SPEC, bool, O>;
#[doc = "Field `HFRDY` reader - 4:4\\]
Indicates whether the software executing when the processor allocated the FP stack frame was able to set the HardFault exception to pending. 0: Priority did not permit setting the HardFault handler to the pending state when the floating-point stack frame was allocated. 1: Priority permitted setting the HardFault handler to the pending state when the floating-point stack frame was allocated."]
pub type HFRDY_R = crate::BitReader<bool>;
#[doc = "Field `HFRDY` writer - 4:4\\]
Indicates whether the software executing when the processor allocated the FP stack frame was able to set the HardFault exception to pending. 0: Priority did not permit setting the HardFault handler to the pending state when the floating-point stack frame was allocated. 1: Priority permitted setting the HardFault handler to the pending state when the floating-point stack frame was allocated."]
pub type HFRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPCCR_SPEC, bool, O>;
#[doc = "Field `MMRDY` reader - 5:5\\]
Indicates whether the software executing when the processor allocated the FP stack frame was able to set the MemManage exception to pending. 0: MemManage is disabled or priority did not permit setting the MemManage handler to the pending state when the floating-point stack frame was allocated. 1: MemManage is enabled and priority permitted setting the MemManage handler to the pending state when the floating-point stack frame was allocated."]
pub type MMRDY_R = crate::BitReader<bool>;
#[doc = "Field `MMRDY` writer - 5:5\\]
Indicates whether the software executing when the processor allocated the FP stack frame was able to set the MemManage exception to pending. 0: MemManage is disabled or priority did not permit setting the MemManage handler to the pending state when the floating-point stack frame was allocated. 1: MemManage is enabled and priority permitted setting the MemManage handler to the pending state when the floating-point stack frame was allocated."]
pub type MMRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPCCR_SPEC, bool, O>;
#[doc = "Field `BFRDY` reader - 6:6\\]
Indicates whether the software executing when the processor allocated the FP stack frame was able to set the BusFault exception to pending. 0: BusFault is disabled or priority did not permit setting the BusFault handler to the pending state when the floating-point stack frame was allocated. 1: BusFault is enabled and priority permitted setting the BusFault handler to the pending state when the floating-point stack frame was allocated."]
pub type BFRDY_R = crate::BitReader<bool>;
#[doc = "Field `BFRDY` writer - 6:6\\]
Indicates whether the software executing when the processor allocated the FP stack frame was able to set the BusFault exception to pending. 0: BusFault is disabled or priority did not permit setting the BusFault handler to the pending state when the floating-point stack frame was allocated. 1: BusFault is enabled and priority permitted setting the BusFault handler to the pending state when the floating-point stack frame was allocated."]
pub type BFRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPCCR_SPEC, bool, O>;
#[doc = "Field `RESERVED7` reader - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED7_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED7` writer - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED7_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPCCR_SPEC, bool, O>;
#[doc = "Field `MONRDY` reader - 8:8\\]
Indicates whether the the software executing when the processor allocated the FP stack frame was able to set the DebugMonitor exception to pending. 0: DebugMonitor is disabled or priority did not permit setting DEMCR.MON_PEND when the floating-point stack frame was allocated. 1: DebugMonitor is enabled and priority permits setting DEMCR.MON_PEND when the floating-point stack frame was allocated."]
pub type MONRDY_R = crate::BitReader<bool>;
#[doc = "Field `MONRDY` writer - 8:8\\]
Indicates whether the the software executing when the processor allocated the FP stack frame was able to set the DebugMonitor exception to pending. 0: DebugMonitor is disabled or priority did not permit setting DEMCR.MON_PEND when the floating-point stack frame was allocated. 1: DebugMonitor is enabled and priority permits setting DEMCR.MON_PEND when the floating-point stack frame was allocated."]
pub type MONRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPCCR_SPEC, bool, O>;
#[doc = "Field `RESERVED9` reader - 29:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED9` writer - 29:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FPCCR_SPEC, u32, u32, 21, O>;
#[doc = "Field `LSPEN` reader - 30:30\\]
Lazy State Preservation enable. Lazy state preservation is when the processor performs a context save, space on the stack is reserved for the floating point state but it is not stacked until the new context performs a floating point operation. 0: Disable automatic lazy state preservation for floating-point context. 1: Enable automatic lazy state preservation for floating-point context."]
pub type LSPEN_R = crate::BitReader<bool>;
#[doc = "Field `LSPEN` writer - 30:30\\]
Lazy State Preservation enable. Lazy state preservation is when the processor performs a context save, space on the stack is reserved for the floating point state but it is not stacked until the new context performs a floating point operation. 0: Disable automatic lazy state preservation for floating-point context. 1: Enable automatic lazy state preservation for floating-point context."]
pub type LSPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPCCR_SPEC, bool, O>;
#[doc = "Field `ASPEN` reader - 31:31\\]
Automatic State Preservation enable. When this bit is set is will cause bit \\[2\\]
of the Special CONTROL register to be set (FPCA) on execution of a floating point instruction which results in the floating point state automatically being preserved on exception entry."]
pub type ASPEN_R = crate::BitReader<bool>;
#[doc = "Field `ASPEN` writer - 31:31\\]
Automatic State Preservation enable. When this bit is set is will cause bit \\[2\\]
of the Special CONTROL register to be set (FPCA) on execution of a floating point instruction which results in the floating point state automatically being preserved on exception entry."]
pub type ASPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPCCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates whether Lazy preservation of the FP state is active: 0: Lazy state preservation is not active. 1: Lazy state preservation is active. floating-point stack frame has been allocated but saving state to it has been deferred."]
    #[inline(always)]
    pub fn lspact(&self) -> LSPACT_R {
        LSPACT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates the privilege level of the software executing was User (Unpriviledged) when the processor allocated the FP stack frame: 0: Privilege level was not user when the floating-point stack frame was allocated. 1: Privilege level was user when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn user(&self) -> USER_R {
        USER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Indicates the processor mode was Thread when it allocated the FP stack frame. 0: Mode was not Thread Mode when the floating-point stack frame was allocated. 1: Mode was Thread Mode when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn thread(&self) -> THREAD_R {
        THREAD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Indicates whether the software executing when the processor allocated the FP stack frame was able to set the HardFault exception to pending. 0: Priority did not permit setting the HardFault handler to the pending state when the floating-point stack frame was allocated. 1: Priority permitted setting the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn hfrdy(&self) -> HFRDY_R {
        HFRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Indicates whether the software executing when the processor allocated the FP stack frame was able to set the MemManage exception to pending. 0: MemManage is disabled or priority did not permit setting the MemManage handler to the pending state when the floating-point stack frame was allocated. 1: MemManage is enabled and priority permitted setting the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn mmrdy(&self) -> MMRDY_R {
        MMRDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Indicates whether the software executing when the processor allocated the FP stack frame was able to set the BusFault exception to pending. 0: BusFault is disabled or priority did not permit setting the BusFault handler to the pending state when the floating-point stack frame was allocated. 1: BusFault is enabled and priority permitted setting the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn bfrdy(&self) -> BFRDY_R {
        BFRDY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Indicates whether the the software executing when the processor allocated the FP stack frame was able to set the DebugMonitor exception to pending. 0: DebugMonitor is disabled or priority did not permit setting DEMCR.MON_PEND when the floating-point stack frame was allocated. 1: DebugMonitor is enabled and priority permits setting DEMCR.MON_PEND when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn monrdy(&self) -> MONRDY_R {
        MONRDY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:29 - 29:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new((self.bits >> 9) & 0x001f_ffff)
    }
    #[doc = "Bit 30 - 30:30\\]
Lazy State Preservation enable. Lazy state preservation is when the processor performs a context save, space on the stack is reserved for the floating point state but it is not stacked until the new context performs a floating point operation. 0: Disable automatic lazy state preservation for floating-point context. 1: Enable automatic lazy state preservation for floating-point context."]
    #[inline(always)]
    pub fn lspen(&self) -> LSPEN_R {
        LSPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Automatic State Preservation enable. When this bit is set is will cause bit \\[2\\]
of the Special CONTROL register to be set (FPCA) on execution of a floating point instruction which results in the floating point state automatically being preserved on exception entry."]
    #[inline(always)]
    pub fn aspen(&self) -> ASPEN_R {
        ASPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates whether Lazy preservation of the FP state is active: 0: Lazy state preservation is not active. 1: Lazy state preservation is active. floating-point stack frame has been allocated but saving state to it has been deferred."]
    #[inline(always)]
    #[must_use]
    pub fn lspact(&mut self) -> LSPACT_W<0> {
        LSPACT_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates the privilege level of the software executing was User (Unpriviledged) when the processor allocated the FP stack frame: 0: Privilege level was not user when the floating-point stack frame was allocated. 1: Privilege level was user when the floating-point stack frame was allocated."]
    #[inline(always)]
    #[must_use]
    pub fn user(&mut self) -> USER_W<1> {
        USER_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Indicates the processor mode was Thread when it allocated the FP stack frame. 0: Mode was not Thread Mode when the floating-point stack frame was allocated. 1: Mode was Thread Mode when the floating-point stack frame was allocated."]
    #[inline(always)]
    #[must_use]
    pub fn thread(&mut self) -> THREAD_W<3> {
        THREAD_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Indicates whether the software executing when the processor allocated the FP stack frame was able to set the HardFault exception to pending. 0: Priority did not permit setting the HardFault handler to the pending state when the floating-point stack frame was allocated. 1: Priority permitted setting the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    #[must_use]
    pub fn hfrdy(&mut self) -> HFRDY_W<4> {
        HFRDY_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Indicates whether the software executing when the processor allocated the FP stack frame was able to set the MemManage exception to pending. 0: MemManage is disabled or priority did not permit setting the MemManage handler to the pending state when the floating-point stack frame was allocated. 1: MemManage is enabled and priority permitted setting the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    #[must_use]
    pub fn mmrdy(&mut self) -> MMRDY_W<5> {
        MMRDY_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Indicates whether the software executing when the processor allocated the FP stack frame was able to set the BusFault exception to pending. 0: BusFault is disabled or priority did not permit setting the BusFault handler to the pending state when the floating-point stack frame was allocated. 1: BusFault is enabled and priority permitted setting the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    #[must_use]
    pub fn bfrdy(&mut self) -> BFRDY_W<6> {
        BFRDY_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> RESERVED7_W<7> {
        RESERVED7_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Indicates whether the the software executing when the processor allocated the FP stack frame was able to set the DebugMonitor exception to pending. 0: DebugMonitor is disabled or priority did not permit setting DEMCR.MON_PEND when the floating-point stack frame was allocated. 1: DebugMonitor is enabled and priority permits setting DEMCR.MON_PEND when the floating-point stack frame was allocated."]
    #[inline(always)]
    #[must_use]
    pub fn monrdy(&mut self) -> MONRDY_W<8> {
        MONRDY_W::new(self)
    }
    #[doc = "Bits 9:29 - 29:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> RESERVED9_W<9> {
        RESERVED9_W::new(self)
    }
    #[doc = "Bit 30 - 30:30\\]
Lazy State Preservation enable. Lazy state preservation is when the processor performs a context save, space on the stack is reserved for the floating point state but it is not stacked until the new context performs a floating point operation. 0: Disable automatic lazy state preservation for floating-point context. 1: Enable automatic lazy state preservation for floating-point context."]
    #[inline(always)]
    #[must_use]
    pub fn lspen(&mut self) -> LSPEN_W<30> {
        LSPEN_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
Automatic State Preservation enable. When this bit is set is will cause bit \\[2\\]
of the Special CONTROL register to be set (FPCA) on execution of a floating point instruction which results in the floating point state automatically being preserved on exception entry."]
    #[inline(always)]
    #[must_use]
    pub fn aspen(&mut self) -> ASPEN_W<31> {
        ASPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Floating Point Context Control This register holds control data for the floating-point unit. Accessible only by privileged software.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpccr](index.html) module"]
pub struct FPCCR_SPEC;
impl crate::RegisterSpec for FPCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fpccr::R](R) reader structure"]
impl crate::Readable for FPCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fpccr::W](W) writer structure"]
impl crate::Writable for FPCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FPCCR to value 0xc000_0000"]
impl crate::Resettable for FPCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0xc000_0000;
}
