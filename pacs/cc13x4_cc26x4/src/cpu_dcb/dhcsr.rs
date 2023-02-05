#[doc = "Register `DHCSR` reader"]
pub struct R(crate::R<DHCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DHCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DHCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DHCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DHCSR` writer"]
pub struct W(crate::W<DHCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DHCSR_SPEC>;
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
impl From<crate::W<DHCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DHCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `C_DEBUGEN` reader - 0:0\\]
Enable Halting debug"]
pub type C_DEBUGEN_R = crate::BitReader<bool>;
#[doc = "Field `C_DEBUGEN` writer - 0:0\\]
Enable Halting debug"]
pub type C_DEBUGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DHCSR_SPEC, bool, O>;
#[doc = "Field `C_HALT` reader - 1:1\\]
PE enter Debug state halt request"]
pub type C_HALT_R = crate::BitReader<bool>;
#[doc = "Field `C_HALT` writer - 1:1\\]
PE enter Debug state halt request"]
pub type C_HALT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DHCSR_SPEC, bool, O>;
#[doc = "Field `C_STEP` reader - 2:2\\]
Enable single instruction step"]
pub type C_STEP_R = crate::BitReader<bool>;
#[doc = "Field `C_STEP` writer - 2:2\\]
Enable single instruction step"]
pub type C_STEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DHCSR_SPEC, bool, O>;
#[doc = "Field `C_MASKINTS` reader - 3:3\\]
When debug is enabled, the debugger can write to this bit to mask PendSV, SysTick and external configurable interrupts"]
pub type C_MASKINTS_R = crate::BitReader<bool>;
#[doc = "Field `C_MASKINTS` writer - 3:3\\]
When debug is enabled, the debugger can write to this bit to mask PendSV, SysTick and external configurable interrupts"]
pub type C_MASKINTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DHCSR_SPEC, bool, O>;
#[doc = "Field `RESERVED4` reader - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED4` writer - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_W<'a, const O: u8> = crate::BitWriter<'a, u32, DHCSR_SPEC, bool, O>;
#[doc = "Field `C_SNAPSTALL` reader - 5:5\\]
Allow imprecise entry to Debug state"]
pub type C_SNAPSTALL_R = crate::BitReader<bool>;
#[doc = "Field `C_SNAPSTALL` writer - 5:5\\]
Allow imprecise entry to Debug state"]
pub type C_SNAPSTALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DHCSR_SPEC, bool, O>;
#[doc = "Field `RESERVED6` reader - 15:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED6` writer - 15:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DHCSR_SPEC, u16, u16, 10, O>;
#[doc = "Field `DBGKEY` reader - 31:16\\]
A debugger must write 0xA05F to this field to enable write access to the remaining bits, otherwise the PE ignores the write access"]
pub type DBGKEY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DBGKEY` writer - 31:16\\]
A debugger must write 0xA05F to this field to enable write access to the remaining bits, otherwise the PE ignores the write access"]
pub type DBGKEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DHCSR_SPEC, u16, u16, 16, O>;
#[doc = "Field `S_HALT` reader - 17:17\\]
Indicates whether the PE is in Debug state"]
pub type S_HALT_R = crate::BitReader<bool>;
#[doc = "Field `S_HALT` writer - 17:17\\]
Indicates whether the PE is in Debug state"]
pub type S_HALT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DHCSR_SPEC, bool, O>;
#[doc = "Field `S_SLEEP` reader - 18:18\\]
Indicates whether the PE is sleeping"]
pub type S_SLEEP_R = crate::BitReader<bool>;
#[doc = "Field `S_SLEEP` writer - 18:18\\]
Indicates whether the PE is sleeping"]
pub type S_SLEEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DHCSR_SPEC, bool, O>;
#[doc = "Field `S_LOCKUP` reader - 19:19\\]
Indicates whether the PE is in Lockup state"]
pub type S_LOCKUP_R = crate::BitReader<bool>;
#[doc = "Field `S_LOCKUP` writer - 19:19\\]
Indicates whether the PE is in Lockup state"]
pub type S_LOCKUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DHCSR_SPEC, bool, O>;
#[doc = "Field `S_SDE` reader - 20:20\\]
Indicates whether Secure invasive debug is allowed"]
pub type S_SDE_R = crate::BitReader<bool>;
#[doc = "Field `S_SDE` writer - 20:20\\]
Indicates whether Secure invasive debug is allowed"]
pub type S_SDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DHCSR_SPEC, bool, O>;
#[doc = "Field `RESERVED21` reader - 23:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED21_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED21` writer - 23:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED21_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DHCSR_SPEC, u8, u8, 3, O>;
#[doc = "Field `S_RETIRE_ST` reader - 24:24\\]
Set to 1 every time the PE retires one of more instructions"]
pub type S_RETIRE_ST_R = crate::BitReader<bool>;
#[doc = "Field `S_RETIRE_ST` writer - 24:24\\]
Set to 1 every time the PE retires one of more instructions"]
pub type S_RETIRE_ST_W<'a, const O: u8> = crate::BitWriter<'a, u32, DHCSR_SPEC, bool, O>;
#[doc = "Field `S_RESET_ST` reader - 25:25\\]
Indicates whether the PE has been reset since the last read of the DHCSR"]
pub type S_RESET_ST_R = crate::BitReader<bool>;
#[doc = "Field `S_RESET_ST` writer - 25:25\\]
Indicates whether the PE has been reset since the last read of the DHCSR"]
pub type S_RESET_ST_W<'a, const O: u8> = crate::BitWriter<'a, u32, DHCSR_SPEC, bool, O>;
#[doc = "Field `S_RESTART_ST` reader - 26:26\\]
Indicates the PE has processed a request to clear DHCSR.C_HALT to 0. That is, either a write to DHCSR that clears DHCSR.C_HALT from 1 to 0, or an External Restart Request"]
pub type S_RESTART_ST_R = crate::BitReader<bool>;
#[doc = "Field `S_RESTART_ST` writer - 26:26\\]
Indicates the PE has processed a request to clear DHCSR.C_HALT to 0. That is, either a write to DHCSR that clears DHCSR.C_HALT from 1 to 0, or an External Restart Request"]
pub type S_RESTART_ST_W<'a, const O: u8> = crate::BitWriter<'a, u32, DHCSR_SPEC, bool, O>;
#[doc = "Field `RESERVED27` reader - 31:27\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED27_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED27` writer - 31:27\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED27_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DHCSR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable Halting debug"]
    #[inline(always)]
    pub fn c_debugen(&self) -> C_DEBUGEN_R {
        C_DEBUGEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
PE enter Debug state halt request"]
    #[inline(always)]
    pub fn c_halt(&self) -> C_HALT_R {
        C_HALT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable single instruction step"]
    #[inline(always)]
    pub fn c_step(&self) -> C_STEP_R {
        C_STEP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
When debug is enabled, the debugger can write to this bit to mask PendSV, SysTick and external configurable interrupts"]
    #[inline(always)]
    pub fn c_maskints(&self) -> C_MASKINTS_R {
        C_MASKINTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Allow imprecise entry to Debug state"]
    #[inline(always)]
    pub fn c_snapstall(&self) -> C_SNAPSTALL_R {
        C_SNAPSTALL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:15 - 15:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 0x03ff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
A debugger must write 0xA05F to this field to enable write access to the remaining bits, otherwise the PE ignores the write access"]
    #[inline(always)]
    pub fn dbgkey(&self) -> DBGKEY_R {
        DBGKEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bit 17 - 17:17\\]
Indicates whether the PE is in Debug state"]
    #[inline(always)]
    pub fn s_halt(&self) -> S_HALT_R {
        S_HALT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Indicates whether the PE is sleeping"]
    #[inline(always)]
    pub fn s_sleep(&self) -> S_SLEEP_R {
        S_SLEEP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Indicates whether the PE is in Lockup state"]
    #[inline(always)]
    pub fn s_lockup(&self) -> S_LOCKUP_R {
        S_LOCKUP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Indicates whether Secure invasive debug is allowed"]
    #[inline(always)]
    pub fn s_sde(&self) -> S_SDE_R {
        S_SDE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved21(&self) -> RESERVED21_R {
        RESERVED21_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Set to 1 every time the PE retires one of more instructions"]
    #[inline(always)]
    pub fn s_retire_st(&self) -> S_RETIRE_ST_R {
        S_RETIRE_ST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Indicates whether the PE has been reset since the last read of the DHCSR"]
    #[inline(always)]
    pub fn s_reset_st(&self) -> S_RESET_ST_R {
        S_RESET_ST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Indicates the PE has processed a request to clear DHCSR.C_HALT to 0. That is, either a write to DHCSR that clears DHCSR.C_HALT from 1 to 0, or an External Restart Request"]
    #[inline(always)]
    pub fn s_restart_st(&self) -> S_RESTART_ST_R {
        S_RESTART_ST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:31 - 31:27\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved27(&self) -> RESERVED27_R {
        RESERVED27_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable Halting debug"]
    #[inline(always)]
    #[must_use]
    pub fn c_debugen(&mut self) -> C_DEBUGEN_W<0> {
        C_DEBUGEN_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
PE enter Debug state halt request"]
    #[inline(always)]
    #[must_use]
    pub fn c_halt(&mut self) -> C_HALT_W<1> {
        C_HALT_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable single instruction step"]
    #[inline(always)]
    #[must_use]
    pub fn c_step(&mut self) -> C_STEP_W<2> {
        C_STEP_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
When debug is enabled, the debugger can write to this bit to mask PendSV, SysTick and external configurable interrupts"]
    #[inline(always)]
    #[must_use]
    pub fn c_maskints(&mut self) -> C_MASKINTS_W<3> {
        C_MASKINTS_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> RESERVED4_W<4> {
        RESERVED4_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Allow imprecise entry to Debug state"]
    #[inline(always)]
    #[must_use]
    pub fn c_snapstall(&mut self) -> C_SNAPSTALL_W<5> {
        C_SNAPSTALL_W::new(self)
    }
    #[doc = "Bits 6:15 - 15:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<6> {
        RESERVED6_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
A debugger must write 0xA05F to this field to enable write access to the remaining bits, otherwise the PE ignores the write access"]
    #[inline(always)]
    #[must_use]
    pub fn dbgkey(&mut self) -> DBGKEY_W<16> {
        DBGKEY_W::new(self)
    }
    #[doc = "Bit 17 - 17:17\\]
Indicates whether the PE is in Debug state"]
    #[inline(always)]
    #[must_use]
    pub fn s_halt(&mut self) -> S_HALT_W<17> {
        S_HALT_W::new(self)
    }
    #[doc = "Bit 18 - 18:18\\]
Indicates whether the PE is sleeping"]
    #[inline(always)]
    #[must_use]
    pub fn s_sleep(&mut self) -> S_SLEEP_W<18> {
        S_SLEEP_W::new(self)
    }
    #[doc = "Bit 19 - 19:19\\]
Indicates whether the PE is in Lockup state"]
    #[inline(always)]
    #[must_use]
    pub fn s_lockup(&mut self) -> S_LOCKUP_W<19> {
        S_LOCKUP_W::new(self)
    }
    #[doc = "Bit 20 - 20:20\\]
Indicates whether Secure invasive debug is allowed"]
    #[inline(always)]
    #[must_use]
    pub fn s_sde(&mut self) -> S_SDE_W<20> {
        S_SDE_W::new(self)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved21(&mut self) -> RESERVED21_W<21> {
        RESERVED21_W::new(self)
    }
    #[doc = "Bit 24 - 24:24\\]
Set to 1 every time the PE retires one of more instructions"]
    #[inline(always)]
    #[must_use]
    pub fn s_retire_st(&mut self) -> S_RETIRE_ST_W<24> {
        S_RETIRE_ST_W::new(self)
    }
    #[doc = "Bit 25 - 25:25\\]
Indicates whether the PE has been reset since the last read of the DHCSR"]
    #[inline(always)]
    #[must_use]
    pub fn s_reset_st(&mut self) -> S_RESET_ST_W<25> {
        S_RESET_ST_W::new(self)
    }
    #[doc = "Bit 26 - 26:26\\]
Indicates the PE has processed a request to clear DHCSR.C_HALT to 0. That is, either a write to DHCSR that clears DHCSR.C_HALT from 1 to 0, or an External Restart Request"]
    #[inline(always)]
    #[must_use]
    pub fn s_restart_st(&mut self) -> S_RESTART_ST_W<26> {
        S_RESTART_ST_W::new(self)
    }
    #[doc = "Bits 27:31 - 31:27\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved27(&mut self) -> RESERVED27_W<27> {
        RESERVED27_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls halting debug\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhcsr](index.html) module"]
pub struct DHCSR_SPEC;
impl crate::RegisterSpec for DHCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dhcsr::R](R) reader structure"]
impl crate::Readable for DHCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dhcsr::W](W) writer structure"]
impl crate::Writable for DHCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DHCSR to value 0"]
impl crate::Resettable for DHCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
