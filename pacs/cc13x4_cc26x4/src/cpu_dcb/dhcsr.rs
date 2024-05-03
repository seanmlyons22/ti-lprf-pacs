#[doc = "Register `DHCSR` reader"]
pub type R = crate::R<DhcsrSpec>;
#[doc = "Register `DHCSR` writer"]
pub type W = crate::W<DhcsrSpec>;
#[doc = "Field `C_DEBUGEN` reader - 0:0\\]
Enable Halting debug"]
pub type CDebugenR = crate::BitReader;
#[doc = "Field `C_DEBUGEN` writer - 0:0\\]
Enable Halting debug"]
pub type CDebugenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_HALT` reader - 1:1\\]
PE enter Debug state halt request"]
pub type CHaltR = crate::BitReader;
#[doc = "Field `C_HALT` writer - 1:1\\]
PE enter Debug state halt request"]
pub type CHaltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_STEP` reader - 2:2\\]
Enable single instruction step"]
pub type CStepR = crate::BitReader;
#[doc = "Field `C_STEP` writer - 2:2\\]
Enable single instruction step"]
pub type CStepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_MASKINTS` reader - 3:3\\]
When debug is enabled, the debugger can write to this bit to mask PendSV, SysTick and external configurable interrupts"]
pub type CMaskintsR = crate::BitReader;
#[doc = "Field `C_MASKINTS` writer - 3:3\\]
When debug is enabled, the debugger can write to this bit to mask PendSV, SysTick and external configurable interrupts"]
pub type CMaskintsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED4` reader - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `RESERVED4` writer - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_SNAPSTALL` reader - 5:5\\]
Allow imprecise entry to Debug state"]
pub type CSnapstallR = crate::BitReader;
#[doc = "Field `C_SNAPSTALL` writer - 5:5\\]
Allow imprecise entry to Debug state"]
pub type CSnapstallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED6` reader - 15:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED6` writer - 15:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DBGKEY` reader - 31:16\\]
A debugger must write 0xA05F to this field to enable write access to the remaining bits, otherwise the PE ignores the write access"]
pub type DbgkeyR = crate::FieldReader<u16>;
#[doc = "Field `DBGKEY` writer - 31:16\\]
A debugger must write 0xA05F to this field to enable write access to the remaining bits, otherwise the PE ignores the write access"]
pub type DbgkeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `S_HALT` reader - 17:17\\]
Indicates whether the PE is in Debug state"]
pub type SHaltR = crate::BitReader;
#[doc = "Field `S_HALT` writer - 17:17\\]
Indicates whether the PE is in Debug state"]
pub type SHaltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_SLEEP` reader - 18:18\\]
Indicates whether the PE is sleeping"]
pub type SSleepR = crate::BitReader;
#[doc = "Field `S_SLEEP` writer - 18:18\\]
Indicates whether the PE is sleeping"]
pub type SSleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_LOCKUP` reader - 19:19\\]
Indicates whether the PE is in Lockup state"]
pub type SLockupR = crate::BitReader;
#[doc = "Field `S_LOCKUP` writer - 19:19\\]
Indicates whether the PE is in Lockup state"]
pub type SLockupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_SDE` reader - 20:20\\]
Indicates whether Secure invasive debug is allowed"]
pub type SSdeR = crate::BitReader;
#[doc = "Field `S_SDE` writer - 20:20\\]
Indicates whether Secure invasive debug is allowed"]
pub type SSdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED21` reader - 23:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved21R = crate::FieldReader;
#[doc = "Field `RESERVED21` writer - 23:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved21W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `S_RETIRE_ST` reader - 24:24\\]
Set to 1 every time the PE retires one of more instructions"]
pub type SRetireStR = crate::BitReader;
#[doc = "Field `S_RETIRE_ST` writer - 24:24\\]
Set to 1 every time the PE retires one of more instructions"]
pub type SRetireStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_RESET_ST` reader - 25:25\\]
Indicates whether the PE has been reset since the last read of the DHCSR"]
pub type SResetStR = crate::BitReader;
#[doc = "Field `S_RESET_ST` writer - 25:25\\]
Indicates whether the PE has been reset since the last read of the DHCSR"]
pub type SResetStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_RESTART_ST` reader - 26:26\\]
Indicates the PE has processed a request to clear DHCSR.C_HALT to 0. That is, either a write to DHCSR that clears DHCSR.C_HALT from 1 to 0, or an External Restart Request"]
pub type SRestartStR = crate::BitReader;
#[doc = "Field `S_RESTART_ST` writer - 26:26\\]
Indicates the PE has processed a request to clear DHCSR.C_HALT to 0. That is, either a write to DHCSR that clears DHCSR.C_HALT from 1 to 0, or an External Restart Request"]
pub type SRestartStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED27` reader - 31:27\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved27R = crate::FieldReader;
#[doc = "Field `RESERVED27` writer - 31:27\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved27W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable Halting debug"]
    #[inline(always)]
    pub fn c_debugen(&self) -> CDebugenR {
        CDebugenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
PE enter Debug state halt request"]
    #[inline(always)]
    pub fn c_halt(&self) -> CHaltR {
        CHaltR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable single instruction step"]
    #[inline(always)]
    pub fn c_step(&self) -> CStepR {
        CStepR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
When debug is enabled, the debugger can write to this bit to mask PendSV, SysTick and external configurable interrupts"]
    #[inline(always)]
    pub fn c_maskints(&self) -> CMaskintsR {
        CMaskintsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Allow imprecise entry to Debug state"]
    #[inline(always)]
    pub fn c_snapstall(&self) -> CSnapstallR {
        CSnapstallR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:15 - 15:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 6) & 0x03ff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
A debugger must write 0xA05F to this field to enable write access to the remaining bits, otherwise the PE ignores the write access"]
    #[inline(always)]
    pub fn dbgkey(&self) -> DbgkeyR {
        DbgkeyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bit 17 - 17:17\\]
Indicates whether the PE is in Debug state"]
    #[inline(always)]
    pub fn s_halt(&self) -> SHaltR {
        SHaltR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Indicates whether the PE is sleeping"]
    #[inline(always)]
    pub fn s_sleep(&self) -> SSleepR {
        SSleepR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Indicates whether the PE is in Lockup state"]
    #[inline(always)]
    pub fn s_lockup(&self) -> SLockupR {
        SLockupR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Indicates whether Secure invasive debug is allowed"]
    #[inline(always)]
    pub fn s_sde(&self) -> SSdeR {
        SSdeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved21(&self) -> Reserved21R {
        Reserved21R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Set to 1 every time the PE retires one of more instructions"]
    #[inline(always)]
    pub fn s_retire_st(&self) -> SRetireStR {
        SRetireStR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Indicates whether the PE has been reset since the last read of the DHCSR"]
    #[inline(always)]
    pub fn s_reset_st(&self) -> SResetStR {
        SResetStR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Indicates the PE has processed a request to clear DHCSR.C_HALT to 0. That is, either a write to DHCSR that clears DHCSR.C_HALT from 1 to 0, or an External Restart Request"]
    #[inline(always)]
    pub fn s_restart_st(&self) -> SRestartStR {
        SRestartStR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:31 - 31:27\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved27(&self) -> Reserved27R {
        Reserved27R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable Halting debug"]
    #[inline(always)]
    #[must_use]
    pub fn c_debugen(&mut self) -> CDebugenW<DhcsrSpec> {
        CDebugenW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
PE enter Debug state halt request"]
    #[inline(always)]
    #[must_use]
    pub fn c_halt(&mut self) -> CHaltW<DhcsrSpec> {
        CHaltW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable single instruction step"]
    #[inline(always)]
    #[must_use]
    pub fn c_step(&mut self) -> CStepW<DhcsrSpec> {
        CStepW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
When debug is enabled, the debugger can write to this bit to mask PendSV, SysTick and external configurable interrupts"]
    #[inline(always)]
    #[must_use]
    pub fn c_maskints(&mut self) -> CMaskintsW<DhcsrSpec> {
        CMaskintsW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<DhcsrSpec> {
        Reserved4W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Allow imprecise entry to Debug state"]
    #[inline(always)]
    #[must_use]
    pub fn c_snapstall(&mut self) -> CSnapstallW<DhcsrSpec> {
        CSnapstallW::new(self, 5)
    }
    #[doc = "Bits 6:15 - 15:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<DhcsrSpec> {
        Reserved6W::new(self, 6)
    }
    #[doc = "Bits 16:31 - 31:16\\]
A debugger must write 0xA05F to this field to enable write access to the remaining bits, otherwise the PE ignores the write access"]
    #[inline(always)]
    #[must_use]
    pub fn dbgkey(&mut self) -> DbgkeyW<DhcsrSpec> {
        DbgkeyW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Indicates whether the PE is in Debug state"]
    #[inline(always)]
    #[must_use]
    pub fn s_halt(&mut self) -> SHaltW<DhcsrSpec> {
        SHaltW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Indicates whether the PE is sleeping"]
    #[inline(always)]
    #[must_use]
    pub fn s_sleep(&mut self) -> SSleepW<DhcsrSpec> {
        SSleepW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Indicates whether the PE is in Lockup state"]
    #[inline(always)]
    #[must_use]
    pub fn s_lockup(&mut self) -> SLockupW<DhcsrSpec> {
        SLockupW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Indicates whether Secure invasive debug is allowed"]
    #[inline(always)]
    #[must_use]
    pub fn s_sde(&mut self) -> SSdeW<DhcsrSpec> {
        SSdeW::new(self, 20)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved21(&mut self) -> Reserved21W<DhcsrSpec> {
        Reserved21W::new(self, 21)
    }
    #[doc = "Bit 24 - 24:24\\]
Set to 1 every time the PE retires one of more instructions"]
    #[inline(always)]
    #[must_use]
    pub fn s_retire_st(&mut self) -> SRetireStW<DhcsrSpec> {
        SRetireStW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Indicates whether the PE has been reset since the last read of the DHCSR"]
    #[inline(always)]
    #[must_use]
    pub fn s_reset_st(&mut self) -> SResetStW<DhcsrSpec> {
        SResetStW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Indicates the PE has processed a request to clear DHCSR.C_HALT to 0. That is, either a write to DHCSR that clears DHCSR.C_HALT from 1 to 0, or an External Restart Request"]
    #[inline(always)]
    #[must_use]
    pub fn s_restart_st(&mut self) -> SRestartStW<DhcsrSpec> {
        SRestartStW::new(self, 26)
    }
    #[doc = "Bits 27:31 - 31:27\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved27(&mut self) -> Reserved27W<DhcsrSpec> {
        Reserved27W::new(self, 27)
    }
}
#[doc = "Controls halting debug\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dhcsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dhcsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DhcsrSpec;
impl crate::RegisterSpec for DhcsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dhcsr::R`](R) reader structure"]
impl crate::Readable for DhcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`dhcsr::W`](W) writer structure"]
impl crate::Writable for DhcsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DHCSR to value 0"]
impl crate::Resettable for DhcsrSpec {
    const RESET_VALUE: u32 = 0;
}
