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
Indicates whether lazy preservation of the floating-point state is active"]
pub type LSPACT_R = crate::BitReader<bool>;
#[doc = "Field `LSPACT` writer - 0:0\\]
Indicates whether lazy preservation of the floating-point state is active"]
pub type LSPACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPCCR_SPEC, bool, O>;
#[doc = "Field `USER` reader - 1:1\\]
Indicates the privilege level of the software executing when the PE allocated the floating-point stack frame"]
pub type USER_R = crate::BitReader<bool>;
#[doc = "Field `USER` writer - 1:1\\]
Indicates the privilege level of the software executing when the PE allocated the floating-point stack frame"]
pub type USER_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPCCR_SPEC, bool, O>;
#[doc = "Field `S` reader - 2:2\\]
Security status of the floating-point context. This bit is only present in the Secure version of the register, and behaves as RAZ/WI when accessed from the Non-secure state. This bit is updated whenever lazy state preservation is activated, or when a floating-point instruction is executed"]
pub type S_R = crate::BitReader<bool>;
#[doc = "Field `S` writer - 2:2\\]
Security status of the floating-point context. This bit is only present in the Secure version of the register, and behaves as RAZ/WI when accessed from the Non-secure state. This bit is updated whenever lazy state preservation is activated, or when a floating-point instruction is executed"]
pub type S_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPCCR_SPEC, bool, O>;
#[doc = "Field `THREAD` reader - 3:3\\]
Indicates the PE mode when it allocated the floating-point stack frame"]
pub type THREAD_R = crate::BitReader<bool>;
#[doc = "Field `THREAD` writer - 3:3\\]
Indicates the PE mode when it allocated the floating-point stack frame"]
pub type THREAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPCCR_SPEC, bool, O>;
#[doc = "Field `HFRDY` reader - 4:4\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the HardFault exception to pending"]
pub type HFRDY_R = crate::BitReader<bool>;
#[doc = "Field `HFRDY` writer - 4:4\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the HardFault exception to pending"]
pub type HFRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPCCR_SPEC, bool, O>;
#[doc = "Field `MMRDY` reader - 5:5\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the MemManage exception to pending"]
pub type MMRDY_R = crate::BitReader<bool>;
#[doc = "Field `MMRDY` writer - 5:5\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the MemManage exception to pending"]
pub type MMRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPCCR_SPEC, bool, O>;
#[doc = "Field `BFRDY` reader - 6:6\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the BusFault exception to pending"]
pub type BFRDY_R = crate::BitReader<bool>;
#[doc = "Field `BFRDY` writer - 6:6\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the BusFault exception to pending"]
pub type BFRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPCCR_SPEC, bool, O>;
#[doc = "Field `SFRDY` reader - 7:7\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the SecureFault exception to pending. This bit is only present in the Secure version of the register, and behaves as RAZ/WI when accessed from the Non-secure state"]
pub type SFRDY_R = crate::BitReader<bool>;
#[doc = "Field `SFRDY` writer - 7:7\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the SecureFault exception to pending. This bit is only present in the Secure version of the register, and behaves as RAZ/WI when accessed from the Non-secure state"]
pub type SFRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPCCR_SPEC, bool, O>;
#[doc = "Field `MONRDY` reader - 8:8\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the DebugMonitor exception to pending"]
pub type MONRDY_R = crate::BitReader<bool>;
#[doc = "Field `MONRDY` writer - 8:8\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the DebugMonitor exception to pending"]
pub type MONRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPCCR_SPEC, bool, O>;
#[doc = "Field `SPLIMVIOL` reader - 9:9\\]
This bit is banked between the Security states and indicates whether the floating-point context violates the stack pointer limit that was active when lazy state preservation was activated. SPLIMVIOL modifies the lazy floating-point state preservation behavior"]
pub type SPLIMVIOL_R = crate::BitReader<bool>;
#[doc = "Field `SPLIMVIOL` writer - 9:9\\]
This bit is banked between the Security states and indicates whether the floating-point context violates the stack pointer limit that was active when lazy state preservation was activated. SPLIMVIOL modifies the lazy floating-point state preservation behavior"]
pub type SPLIMVIOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPCCR_SPEC, bool, O>;
#[doc = "Field `UFRDY` reader - 10:10\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the UsageFault exception to pending"]
pub type UFRDY_R = crate::BitReader<bool>;
#[doc = "Field `UFRDY` writer - 10:10\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the UsageFault exception to pending"]
pub type UFRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPCCR_SPEC, bool, O>;
#[doc = "Field `RESERVED11` reader - 25:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED11_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED11` writer - 25:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FPCCR_SPEC, u16, u16, 15, O>;
#[doc = "Field `TS` reader - 26:26\\]
Treat floating-point registers as Secure enable"]
pub type TS_R = crate::BitReader<bool>;
#[doc = "Field `TS` writer - 26:26\\]
Treat floating-point registers as Secure enable"]
pub type TS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPCCR_SPEC, bool, O>;
#[doc = "Field `CLRONRETS` reader - 27:27\\]
This bit controls whether the CLRONRET bit is writeable from the Non-secure state"]
pub type CLRONRETS_R = crate::BitReader<bool>;
#[doc = "Field `CLRONRETS` writer - 27:27\\]
This bit controls whether the CLRONRET bit is writeable from the Non-secure state"]
pub type CLRONRETS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPCCR_SPEC, bool, O>;
#[doc = "Field `CLRONRET` reader - 28:28\\]
Clear floating-point caller saved registers on exception return"]
pub type CLRONRET_R = crate::BitReader<bool>;
#[doc = "Field `CLRONRET` writer - 28:28\\]
Clear floating-point caller saved registers on exception return"]
pub type CLRONRET_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPCCR_SPEC, bool, O>;
#[doc = "Field `LSPENS` reader - 29:29\\]
This bit controls whether the LSPEN bit is writeable from the Non-secure state"]
pub type LSPENS_R = crate::BitReader<bool>;
#[doc = "Field `LSPENS` writer - 29:29\\]
This bit controls whether the LSPEN bit is writeable from the Non-secure state"]
pub type LSPENS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPCCR_SPEC, bool, O>;
#[doc = "Field `LSPEN` reader - 30:30\\]
Enables lazy context save of floating-point state"]
pub type LSPEN_R = crate::BitReader<bool>;
#[doc = "Field `LSPEN` writer - 30:30\\]
Enables lazy context save of floating-point state"]
pub type LSPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPCCR_SPEC, bool, O>;
#[doc = "Field `ASPEN` reader - 31:31\\]
When this bit is set to 1, execution of a floating-point instruction sets the CONTROL.FPCA bit to 1"]
pub type ASPEN_R = crate::BitReader<bool>;
#[doc = "Field `ASPEN` writer - 31:31\\]
When this bit is set to 1, execution of a floating-point instruction sets the CONTROL.FPCA bit to 1"]
pub type ASPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPCCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates whether lazy preservation of the floating-point state is active"]
    #[inline(always)]
    pub fn lspact(&self) -> LSPACT_R {
        LSPACT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates the privilege level of the software executing when the PE allocated the floating-point stack frame"]
    #[inline(always)]
    pub fn user(&self) -> USER_R {
        USER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Security status of the floating-point context. This bit is only present in the Secure version of the register, and behaves as RAZ/WI when accessed from the Non-secure state. This bit is updated whenever lazy state preservation is activated, or when a floating-point instruction is executed"]
    #[inline(always)]
    pub fn s(&self) -> S_R {
        S_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Indicates the PE mode when it allocated the floating-point stack frame"]
    #[inline(always)]
    pub fn thread(&self) -> THREAD_R {
        THREAD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the HardFault exception to pending"]
    #[inline(always)]
    pub fn hfrdy(&self) -> HFRDY_R {
        HFRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the MemManage exception to pending"]
    #[inline(always)]
    pub fn mmrdy(&self) -> MMRDY_R {
        MMRDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the BusFault exception to pending"]
    #[inline(always)]
    pub fn bfrdy(&self) -> BFRDY_R {
        BFRDY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the SecureFault exception to pending. This bit is only present in the Secure version of the register, and behaves as RAZ/WI when accessed from the Non-secure state"]
    #[inline(always)]
    pub fn sfrdy(&self) -> SFRDY_R {
        SFRDY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the DebugMonitor exception to pending"]
    #[inline(always)]
    pub fn monrdy(&self) -> MONRDY_R {
        MONRDY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
This bit is banked between the Security states and indicates whether the floating-point context violates the stack pointer limit that was active when lazy state preservation was activated. SPLIMVIOL modifies the lazy floating-point state preservation behavior"]
    #[inline(always)]
    pub fn splimviol(&self) -> SPLIMVIOL_R {
        SPLIMVIOL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the UsageFault exception to pending"]
    #[inline(always)]
    pub fn ufrdy(&self) -> UFRDY_R {
        UFRDY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:25 - 25:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> RESERVED11_R {
        RESERVED11_R::new(((self.bits >> 11) & 0x7fff) as u16)
    }
    #[doc = "Bit 26 - 26:26\\]
Treat floating-point registers as Secure enable"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
This bit controls whether the CLRONRET bit is writeable from the Non-secure state"]
    #[inline(always)]
    pub fn clronrets(&self) -> CLRONRETS_R {
        CLRONRETS_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Clear floating-point caller saved registers on exception return"]
    #[inline(always)]
    pub fn clronret(&self) -> CLRONRET_R {
        CLRONRET_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
This bit controls whether the LSPEN bit is writeable from the Non-secure state"]
    #[inline(always)]
    pub fn lspens(&self) -> LSPENS_R {
        LSPENS_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Enables lazy context save of floating-point state"]
    #[inline(always)]
    pub fn lspen(&self) -> LSPEN_R {
        LSPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
When this bit is set to 1, execution of a floating-point instruction sets the CONTROL.FPCA bit to 1"]
    #[inline(always)]
    pub fn aspen(&self) -> ASPEN_R {
        ASPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates whether lazy preservation of the floating-point state is active"]
    #[inline(always)]
    #[must_use]
    pub fn lspact(&mut self) -> LSPACT_W<0> {
        LSPACT_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates the privilege level of the software executing when the PE allocated the floating-point stack frame"]
    #[inline(always)]
    #[must_use]
    pub fn user(&mut self) -> USER_W<1> {
        USER_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Security status of the floating-point context. This bit is only present in the Secure version of the register, and behaves as RAZ/WI when accessed from the Non-secure state. This bit is updated whenever lazy state preservation is activated, or when a floating-point instruction is executed"]
    #[inline(always)]
    #[must_use]
    pub fn s(&mut self) -> S_W<2> {
        S_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Indicates the PE mode when it allocated the floating-point stack frame"]
    #[inline(always)]
    #[must_use]
    pub fn thread(&mut self) -> THREAD_W<3> {
        THREAD_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the HardFault exception to pending"]
    #[inline(always)]
    #[must_use]
    pub fn hfrdy(&mut self) -> HFRDY_W<4> {
        HFRDY_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the MemManage exception to pending"]
    #[inline(always)]
    #[must_use]
    pub fn mmrdy(&mut self) -> MMRDY_W<5> {
        MMRDY_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the BusFault exception to pending"]
    #[inline(always)]
    #[must_use]
    pub fn bfrdy(&mut self) -> BFRDY_W<6> {
        BFRDY_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the SecureFault exception to pending. This bit is only present in the Secure version of the register, and behaves as RAZ/WI when accessed from the Non-secure state"]
    #[inline(always)]
    #[must_use]
    pub fn sfrdy(&mut self) -> SFRDY_W<7> {
        SFRDY_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the DebugMonitor exception to pending"]
    #[inline(always)]
    #[must_use]
    pub fn monrdy(&mut self) -> MONRDY_W<8> {
        MONRDY_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
This bit is banked between the Security states and indicates whether the floating-point context violates the stack pointer limit that was active when lazy state preservation was activated. SPLIMVIOL modifies the lazy floating-point state preservation behavior"]
    #[inline(always)]
    #[must_use]
    pub fn splimviol(&mut self) -> SPLIMVIOL_W<9> {
        SPLIMVIOL_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the UsageFault exception to pending"]
    #[inline(always)]
    #[must_use]
    pub fn ufrdy(&mut self) -> UFRDY_W<10> {
        UFRDY_W::new(self)
    }
    #[doc = "Bits 11:25 - 25:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved11(&mut self) -> RESERVED11_W<11> {
        RESERVED11_W::new(self)
    }
    #[doc = "Bit 26 - 26:26\\]
Treat floating-point registers as Secure enable"]
    #[inline(always)]
    #[must_use]
    pub fn ts(&mut self) -> TS_W<26> {
        TS_W::new(self)
    }
    #[doc = "Bit 27 - 27:27\\]
This bit controls whether the CLRONRET bit is writeable from the Non-secure state"]
    #[inline(always)]
    #[must_use]
    pub fn clronrets(&mut self) -> CLRONRETS_W<27> {
        CLRONRETS_W::new(self)
    }
    #[doc = "Bit 28 - 28:28\\]
Clear floating-point caller saved registers on exception return"]
    #[inline(always)]
    #[must_use]
    pub fn clronret(&mut self) -> CLRONRET_W<28> {
        CLRONRET_W::new(self)
    }
    #[doc = "Bit 29 - 29:29\\]
This bit controls whether the LSPEN bit is writeable from the Non-secure state"]
    #[inline(always)]
    #[must_use]
    pub fn lspens(&mut self) -> LSPENS_W<29> {
        LSPENS_W::new(self)
    }
    #[doc = "Bit 30 - 30:30\\]
Enables lazy context save of floating-point state"]
    #[inline(always)]
    #[must_use]
    pub fn lspen(&mut self) -> LSPEN_W<30> {
        LSPEN_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
When this bit is set to 1, execution of a floating-point instruction sets the CONTROL.FPCA bit to 1"]
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
#[doc = "Holds control data for the Floating-point extension\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpccr](index.html) module"]
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
#[doc = "`reset()` method sets FPCCR to value 0"]
impl crate::Resettable for FPCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
