#[doc = "Register `FPCCR` reader"]
pub type R = crate::R<FpccrSpec>;
#[doc = "Register `FPCCR` writer"]
pub type W = crate::W<FpccrSpec>;
#[doc = "Field `LSPACT` reader - 0:0\\]
Indicates whether lazy preservation of the floating-point state is active"]
pub type LspactR = crate::BitReader;
#[doc = "Field `LSPACT` writer - 0:0\\]
Indicates whether lazy preservation of the floating-point state is active"]
pub type LspactW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USER` reader - 1:1\\]
Indicates the privilege level of the software executing when the PE allocated the floating-point stack frame"]
pub type UserR = crate::BitReader;
#[doc = "Field `USER` writer - 1:1\\]
Indicates the privilege level of the software executing when the PE allocated the floating-point stack frame"]
pub type UserW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S` reader - 2:2\\]
Security status of the floating-point context. This bit is only present in the Secure version of the register, and behaves as RAZ/WI when accessed from the Non-secure state. This bit is updated whenever lazy state preservation is activated, or when a floating-point instruction is executed"]
pub type SR = crate::BitReader;
#[doc = "Field `S` writer - 2:2\\]
Security status of the floating-point context. This bit is only present in the Secure version of the register, and behaves as RAZ/WI when accessed from the Non-secure state. This bit is updated whenever lazy state preservation is activated, or when a floating-point instruction is executed"]
pub type SW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THREAD` reader - 3:3\\]
Indicates the PE mode when it allocated the floating-point stack frame"]
pub type ThreadR = crate::BitReader;
#[doc = "Field `THREAD` writer - 3:3\\]
Indicates the PE mode when it allocated the floating-point stack frame"]
pub type ThreadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFRDY` reader - 4:4\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the HardFault exception to pending"]
pub type HfrdyR = crate::BitReader;
#[doc = "Field `HFRDY` writer - 4:4\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the HardFault exception to pending"]
pub type HfrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMRDY` reader - 5:5\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the MemManage exception to pending"]
pub type MmrdyR = crate::BitReader;
#[doc = "Field `MMRDY` writer - 5:5\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the MemManage exception to pending"]
pub type MmrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BFRDY` reader - 6:6\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the BusFault exception to pending"]
pub type BfrdyR = crate::BitReader;
#[doc = "Field `BFRDY` writer - 6:6\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the BusFault exception to pending"]
pub type BfrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFRDY` reader - 7:7\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the SecureFault exception to pending. This bit is only present in the Secure version of the register, and behaves as RAZ/WI when accessed from the Non-secure state"]
pub type SfrdyR = crate::BitReader;
#[doc = "Field `SFRDY` writer - 7:7\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the SecureFault exception to pending. This bit is only present in the Secure version of the register, and behaves as RAZ/WI when accessed from the Non-secure state"]
pub type SfrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MONRDY` reader - 8:8\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the DebugMonitor exception to pending"]
pub type MonrdyR = crate::BitReader;
#[doc = "Field `MONRDY` writer - 8:8\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the DebugMonitor exception to pending"]
pub type MonrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPLIMVIOL` reader - 9:9\\]
This bit is banked between the Security states and indicates whether the floating-point context violates the stack pointer limit that was active when lazy state preservation was activated. SPLIMVIOL modifies the lazy floating-point state preservation behavior"]
pub type SplimviolR = crate::BitReader;
#[doc = "Field `SPLIMVIOL` writer - 9:9\\]
This bit is banked between the Security states and indicates whether the floating-point context violates the stack pointer limit that was active when lazy state preservation was activated. SPLIMVIOL modifies the lazy floating-point state preservation behavior"]
pub type SplimviolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UFRDY` reader - 10:10\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the UsageFault exception to pending"]
pub type UfrdyR = crate::BitReader;
#[doc = "Field `UFRDY` writer - 10:10\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the UsageFault exception to pending"]
pub type UfrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED11` reader - 25:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11R = crate::FieldReader<u16>;
#[doc = "Field `TS` reader - 26:26\\]
Treat floating-point registers as Secure enable"]
pub type TsR = crate::BitReader;
#[doc = "Field `TS` writer - 26:26\\]
Treat floating-point registers as Secure enable"]
pub type TsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRONRETS` reader - 27:27\\]
This bit controls whether the CLRONRET bit is writeable from the Non-secure state"]
pub type ClronretsR = crate::BitReader;
#[doc = "Field `CLRONRETS` writer - 27:27\\]
This bit controls whether the CLRONRET bit is writeable from the Non-secure state"]
pub type ClronretsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRONRET` reader - 28:28\\]
Clear floating-point caller saved registers on exception return"]
pub type ClronretR = crate::BitReader;
#[doc = "Field `CLRONRET` writer - 28:28\\]
Clear floating-point caller saved registers on exception return"]
pub type ClronretW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSPENS` reader - 29:29\\]
This bit controls whether the LSPEN bit is writeable from the Non-secure state"]
pub type LspensR = crate::BitReader;
#[doc = "Field `LSPENS` writer - 29:29\\]
This bit controls whether the LSPEN bit is writeable from the Non-secure state"]
pub type LspensW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSPEN` reader - 30:30\\]
Enables lazy context save of floating-point state"]
pub type LspenR = crate::BitReader;
#[doc = "Field `LSPEN` writer - 30:30\\]
Enables lazy context save of floating-point state"]
pub type LspenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASPEN` reader - 31:31\\]
When this bit is set to 1, execution of a floating-point instruction sets the CONTROL.FPCA bit to 1"]
pub type AspenR = crate::BitReader;
#[doc = "Field `ASPEN` writer - 31:31\\]
When this bit is set to 1, execution of a floating-point instruction sets the CONTROL.FPCA bit to 1"]
pub type AspenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates whether lazy preservation of the floating-point state is active"]
    #[inline(always)]
    pub fn lspact(&self) -> LspactR {
        LspactR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates the privilege level of the software executing when the PE allocated the floating-point stack frame"]
    #[inline(always)]
    pub fn user(&self) -> UserR {
        UserR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Security status of the floating-point context. This bit is only present in the Secure version of the register, and behaves as RAZ/WI when accessed from the Non-secure state. This bit is updated whenever lazy state preservation is activated, or when a floating-point instruction is executed"]
    #[inline(always)]
    pub fn s(&self) -> SR {
        SR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Indicates the PE mode when it allocated the floating-point stack frame"]
    #[inline(always)]
    pub fn thread(&self) -> ThreadR {
        ThreadR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the HardFault exception to pending"]
    #[inline(always)]
    pub fn hfrdy(&self) -> HfrdyR {
        HfrdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the MemManage exception to pending"]
    #[inline(always)]
    pub fn mmrdy(&self) -> MmrdyR {
        MmrdyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the BusFault exception to pending"]
    #[inline(always)]
    pub fn bfrdy(&self) -> BfrdyR {
        BfrdyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the SecureFault exception to pending. This bit is only present in the Secure version of the register, and behaves as RAZ/WI when accessed from the Non-secure state"]
    #[inline(always)]
    pub fn sfrdy(&self) -> SfrdyR {
        SfrdyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the DebugMonitor exception to pending"]
    #[inline(always)]
    pub fn monrdy(&self) -> MonrdyR {
        MonrdyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
This bit is banked between the Security states and indicates whether the floating-point context violates the stack pointer limit that was active when lazy state preservation was activated. SPLIMVIOL modifies the lazy floating-point state preservation behavior"]
    #[inline(always)]
    pub fn splimviol(&self) -> SplimviolR {
        SplimviolR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the UsageFault exception to pending"]
    #[inline(always)]
    pub fn ufrdy(&self) -> UfrdyR {
        UfrdyR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:25 - 25:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new(((self.bits >> 11) & 0x7fff) as u16)
    }
    #[doc = "Bit 26 - 26:26\\]
Treat floating-point registers as Secure enable"]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
This bit controls whether the CLRONRET bit is writeable from the Non-secure state"]
    #[inline(always)]
    pub fn clronrets(&self) -> ClronretsR {
        ClronretsR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Clear floating-point caller saved registers on exception return"]
    #[inline(always)]
    pub fn clronret(&self) -> ClronretR {
        ClronretR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
This bit controls whether the LSPEN bit is writeable from the Non-secure state"]
    #[inline(always)]
    pub fn lspens(&self) -> LspensR {
        LspensR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Enables lazy context save of floating-point state"]
    #[inline(always)]
    pub fn lspen(&self) -> LspenR {
        LspenR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
When this bit is set to 1, execution of a floating-point instruction sets the CONTROL.FPCA bit to 1"]
    #[inline(always)]
    pub fn aspen(&self) -> AspenR {
        AspenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates whether lazy preservation of the floating-point state is active"]
    #[inline(always)]
    #[must_use]
    pub fn lspact(&mut self) -> LspactW<FpccrSpec> {
        LspactW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates the privilege level of the software executing when the PE allocated the floating-point stack frame"]
    #[inline(always)]
    #[must_use]
    pub fn user(&mut self) -> UserW<FpccrSpec> {
        UserW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Security status of the floating-point context. This bit is only present in the Secure version of the register, and behaves as RAZ/WI when accessed from the Non-secure state. This bit is updated whenever lazy state preservation is activated, or when a floating-point instruction is executed"]
    #[inline(always)]
    #[must_use]
    pub fn s(&mut self) -> SW<FpccrSpec> {
        SW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Indicates the PE mode when it allocated the floating-point stack frame"]
    #[inline(always)]
    #[must_use]
    pub fn thread(&mut self) -> ThreadW<FpccrSpec> {
        ThreadW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the HardFault exception to pending"]
    #[inline(always)]
    #[must_use]
    pub fn hfrdy(&mut self) -> HfrdyW<FpccrSpec> {
        HfrdyW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the MemManage exception to pending"]
    #[inline(always)]
    #[must_use]
    pub fn mmrdy(&mut self) -> MmrdyW<FpccrSpec> {
        MmrdyW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the BusFault exception to pending"]
    #[inline(always)]
    #[must_use]
    pub fn bfrdy(&mut self) -> BfrdyW<FpccrSpec> {
        BfrdyW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the SecureFault exception to pending. This bit is only present in the Secure version of the register, and behaves as RAZ/WI when accessed from the Non-secure state"]
    #[inline(always)]
    #[must_use]
    pub fn sfrdy(&mut self) -> SfrdyW<FpccrSpec> {
        SfrdyW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the DebugMonitor exception to pending"]
    #[inline(always)]
    #[must_use]
    pub fn monrdy(&mut self) -> MonrdyW<FpccrSpec> {
        MonrdyW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
This bit is banked between the Security states and indicates whether the floating-point context violates the stack pointer limit that was active when lazy state preservation was activated. SPLIMVIOL modifies the lazy floating-point state preservation behavior"]
    #[inline(always)]
    #[must_use]
    pub fn splimviol(&mut self) -> SplimviolW<FpccrSpec> {
        SplimviolW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the UsageFault exception to pending"]
    #[inline(always)]
    #[must_use]
    pub fn ufrdy(&mut self) -> UfrdyW<FpccrSpec> {
        UfrdyW::new(self, 10)
    }
    #[doc = "Bit 26 - 26:26\\]
Treat floating-point registers as Secure enable"]
    #[inline(always)]
    #[must_use]
    pub fn ts(&mut self) -> TsW<FpccrSpec> {
        TsW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
This bit controls whether the CLRONRET bit is writeable from the Non-secure state"]
    #[inline(always)]
    #[must_use]
    pub fn clronrets(&mut self) -> ClronretsW<FpccrSpec> {
        ClronretsW::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Clear floating-point caller saved registers on exception return"]
    #[inline(always)]
    #[must_use]
    pub fn clronret(&mut self) -> ClronretW<FpccrSpec> {
        ClronretW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
This bit controls whether the LSPEN bit is writeable from the Non-secure state"]
    #[inline(always)]
    #[must_use]
    pub fn lspens(&mut self) -> LspensW<FpccrSpec> {
        LspensW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Enables lazy context save of floating-point state"]
    #[inline(always)]
    #[must_use]
    pub fn lspen(&mut self) -> LspenW<FpccrSpec> {
        LspenW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
When this bit is set to 1, execution of a floating-point instruction sets the CONTROL.FPCA bit to 1"]
    #[inline(always)]
    #[must_use]
    pub fn aspen(&mut self) -> AspenW<FpccrSpec> {
        AspenW::new(self, 31)
    }
}
#[doc = "Holds control data for the Floating-point extension\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets FPCCR to value 0"]
impl crate::Resettable for FpccrSpec {
    const RESET_VALUE: u32 = 0;
}
