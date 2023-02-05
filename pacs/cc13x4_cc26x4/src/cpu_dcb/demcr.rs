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
Enable Reset Vector Catch. This causes a warm reset to halt a running system"]
pub type VC_CORERESET_R = crate::BitReader<bool>;
#[doc = "Field `VC_CORERESET` writer - 0:0\\]
Enable Reset Vector Catch. This causes a warm reset to halt a running system"]
pub type VC_CORERESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED1` writer - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DEMCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `VC_MMERR` reader - 4:4\\]
Enable halting debug trap on a MemManage exception"]
pub type VC_MMERR_R = crate::BitReader<bool>;
#[doc = "Field `VC_MMERR` writer - 4:4\\]
Enable halting debug trap on a MemManage exception"]
pub type VC_MMERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, O>;
#[doc = "Field `VC_NOCPERR` reader - 5:5\\]
Enable halting debug trap on a UsageFault caused by an access to a coprocessor"]
pub type VC_NOCPERR_R = crate::BitReader<bool>;
#[doc = "Field `VC_NOCPERR` writer - 5:5\\]
Enable halting debug trap on a UsageFault caused by an access to a coprocessor"]
pub type VC_NOCPERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, O>;
#[doc = "Field `VC_CHKERR` reader - 6:6\\]
Enable halting debug trap on a UsageFault exception caused by a checking error, for example an alignment check error"]
pub type VC_CHKERR_R = crate::BitReader<bool>;
#[doc = "Field `VC_CHKERR` writer - 6:6\\]
Enable halting debug trap on a UsageFault exception caused by a checking error, for example an alignment check error"]
pub type VC_CHKERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, O>;
#[doc = "Field `VC_STATERR` reader - 7:7\\]
Enable halting debug trap on a UsageFault exception caused by a state information error, for example an Undefined Instruction exception"]
pub type VC_STATERR_R = crate::BitReader<bool>;
#[doc = "Field `VC_STATERR` writer - 7:7\\]
Enable halting debug trap on a UsageFault exception caused by a state information error, for example an Undefined Instruction exception"]
pub type VC_STATERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, O>;
#[doc = "Field `VC_BUSERR` reader - 8:8\\]
BusFault exception halting debug vector catch enable"]
pub type VC_BUSERR_R = crate::BitReader<bool>;
#[doc = "Field `VC_BUSERR` writer - 8:8\\]
BusFault exception halting debug vector catch enable"]
pub type VC_BUSERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, O>;
#[doc = "Field `VC_INTERR` reader - 9:9\\]
Enable halting debug vector catch for faults during exception entry and return"]
pub type VC_INTERR_R = crate::BitReader<bool>;
#[doc = "Field `VC_INTERR` writer - 9:9\\]
Enable halting debug vector catch for faults during exception entry and return"]
pub type VC_INTERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, O>;
#[doc = "Field `VC_HARDERR` reader - 10:10\\]
HardFault exception halting debug vector catch enable"]
pub type VC_HARDERR_R = crate::BitReader<bool>;
#[doc = "Field `VC_HARDERR` writer - 10:10\\]
HardFault exception halting debug vector catch enable"]
pub type VC_HARDERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, O>;
#[doc = "Field `VC_SFERR` reader - 11:11\\]
SecureFault exception halting debug vector catch enable"]
pub type VC_SFERR_R = crate::BitReader<bool>;
#[doc = "Field `VC_SFERR` writer - 11:11\\]
SecureFault exception halting debug vector catch enable"]
pub type VC_SFERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, O>;
#[doc = "Field `RESERVED12` reader - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED12` writer - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DEMCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `MON_EN` reader - 16:16\\]
Enable the DebugMonitor exception"]
pub type MON_EN_R = crate::BitReader<bool>;
#[doc = "Field `MON_EN` writer - 16:16\\]
Enable the DebugMonitor exception"]
pub type MON_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, O>;
#[doc = "Field `MON_PEND` reader - 17:17\\]
Sets or clears the pending state of the DebugMonitor exception"]
pub type MON_PEND_R = crate::BitReader<bool>;
#[doc = "Field `MON_PEND` writer - 17:17\\]
Sets or clears the pending state of the DebugMonitor exception"]
pub type MON_PEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, O>;
#[doc = "Field `MON_STEP` reader - 18:18\\]
Enable DebugMonitor stepping"]
pub type MON_STEP_R = crate::BitReader<bool>;
#[doc = "Field `MON_STEP` writer - 18:18\\]
Enable DebugMonitor stepping"]
pub type MON_STEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, O>;
#[doc = "Field `MON_REQ` reader - 19:19\\]
DebugMonitor semaphore bit"]
pub type MON_REQ_R = crate::BitReader<bool>;
#[doc = "Field `MON_REQ` writer - 19:19\\]
DebugMonitor semaphore bit"]
pub type MON_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, O>;
#[doc = "Field `SDME` reader - 20:20\\]
Indicates whether the DebugMonitor targets the Secure or the Non-secure state and whether debug events are allowed in Secure state"]
pub type SDME_R = crate::BitReader<bool>;
#[doc = "Field `SDME` writer - 20:20\\]
Indicates whether the DebugMonitor targets the Secure or the Non-secure state and whether debug events are allowed in Secure state"]
pub type SDME_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, O>;
#[doc = "Field `RESERVED21` reader - 23:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED21_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED21` writer - 23:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED21_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DEMCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `TRCENA` reader - 24:24\\]
Global enable for all DWT and ITM features"]
pub type TRCENA_R = crate::BitReader<bool>;
#[doc = "Field `TRCENA` writer - 24:24\\]
Global enable for all DWT and ITM features"]
pub type TRCENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEMCR_SPEC, bool, O>;
#[doc = "Field `RESERVED25` reader - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED25_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED25` writer - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED25_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DEMCR_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable Reset Vector Catch. This causes a warm reset to halt a running system"]
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
Enable halting debug trap on a MemManage exception"]
    #[inline(always)]
    pub fn vc_mmerr(&self) -> VC_MMERR_R {
        VC_MMERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Enable halting debug trap on a UsageFault caused by an access to a coprocessor"]
    #[inline(always)]
    pub fn vc_nocperr(&self) -> VC_NOCPERR_R {
        VC_NOCPERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Enable halting debug trap on a UsageFault exception caused by a checking error, for example an alignment check error"]
    #[inline(always)]
    pub fn vc_chkerr(&self) -> VC_CHKERR_R {
        VC_CHKERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Enable halting debug trap on a UsageFault exception caused by a state information error, for example an Undefined Instruction exception"]
    #[inline(always)]
    pub fn vc_staterr(&self) -> VC_STATERR_R {
        VC_STATERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
BusFault exception halting debug vector catch enable"]
    #[inline(always)]
    pub fn vc_buserr(&self) -> VC_BUSERR_R {
        VC_BUSERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Enable halting debug vector catch for faults during exception entry and return"]
    #[inline(always)]
    pub fn vc_interr(&self) -> VC_INTERR_R {
        VC_INTERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
HardFault exception halting debug vector catch enable"]
    #[inline(always)]
    pub fn vc_harderr(&self) -> VC_HARDERR_R {
        VC_HARDERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
SecureFault exception halting debug vector catch enable"]
    #[inline(always)]
    pub fn vc_sferr(&self) -> VC_SFERR_R {
        VC_SFERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable the DebugMonitor exception"]
    #[inline(always)]
    pub fn mon_en(&self) -> MON_EN_R {
        MON_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Sets or clears the pending state of the DebugMonitor exception"]
    #[inline(always)]
    pub fn mon_pend(&self) -> MON_PEND_R {
        MON_PEND_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Enable DebugMonitor stepping"]
    #[inline(always)]
    pub fn mon_step(&self) -> MON_STEP_R {
        MON_STEP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
DebugMonitor semaphore bit"]
    #[inline(always)]
    pub fn mon_req(&self) -> MON_REQ_R {
        MON_REQ_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Indicates whether the DebugMonitor targets the Secure or the Non-secure state and whether debug events are allowed in Secure state"]
    #[inline(always)]
    pub fn sdme(&self) -> SDME_R {
        SDME_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved21(&self) -> RESERVED21_R {
        RESERVED21_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Global enable for all DWT and ITM features"]
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
Enable Reset Vector Catch. This causes a warm reset to halt a running system"]
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
Enable halting debug trap on a MemManage exception"]
    #[inline(always)]
    #[must_use]
    pub fn vc_mmerr(&mut self) -> VC_MMERR_W<4> {
        VC_MMERR_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Enable halting debug trap on a UsageFault caused by an access to a coprocessor"]
    #[inline(always)]
    #[must_use]
    pub fn vc_nocperr(&mut self) -> VC_NOCPERR_W<5> {
        VC_NOCPERR_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Enable halting debug trap on a UsageFault exception caused by a checking error, for example an alignment check error"]
    #[inline(always)]
    #[must_use]
    pub fn vc_chkerr(&mut self) -> VC_CHKERR_W<6> {
        VC_CHKERR_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Enable halting debug trap on a UsageFault exception caused by a state information error, for example an Undefined Instruction exception"]
    #[inline(always)]
    #[must_use]
    pub fn vc_staterr(&mut self) -> VC_STATERR_W<7> {
        VC_STATERR_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
BusFault exception halting debug vector catch enable"]
    #[inline(always)]
    #[must_use]
    pub fn vc_buserr(&mut self) -> VC_BUSERR_W<8> {
        VC_BUSERR_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Enable halting debug vector catch for faults during exception entry and return"]
    #[inline(always)]
    #[must_use]
    pub fn vc_interr(&mut self) -> VC_INTERR_W<9> {
        VC_INTERR_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
HardFault exception halting debug vector catch enable"]
    #[inline(always)]
    #[must_use]
    pub fn vc_harderr(&mut self) -> VC_HARDERR_W<10> {
        VC_HARDERR_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
SecureFault exception halting debug vector catch enable"]
    #[inline(always)]
    #[must_use]
    pub fn vc_sferr(&mut self) -> VC_SFERR_W<11> {
        VC_SFERR_W::new(self)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> RESERVED12_W<12> {
        RESERVED12_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable the DebugMonitor exception"]
    #[inline(always)]
    #[must_use]
    pub fn mon_en(&mut self) -> MON_EN_W<16> {
        MON_EN_W::new(self)
    }
    #[doc = "Bit 17 - 17:17\\]
Sets or clears the pending state of the DebugMonitor exception"]
    #[inline(always)]
    #[must_use]
    pub fn mon_pend(&mut self) -> MON_PEND_W<17> {
        MON_PEND_W::new(self)
    }
    #[doc = "Bit 18 - 18:18\\]
Enable DebugMonitor stepping"]
    #[inline(always)]
    #[must_use]
    pub fn mon_step(&mut self) -> MON_STEP_W<18> {
        MON_STEP_W::new(self)
    }
    #[doc = "Bit 19 - 19:19\\]
DebugMonitor semaphore bit"]
    #[inline(always)]
    #[must_use]
    pub fn mon_req(&mut self) -> MON_REQ_W<19> {
        MON_REQ_W::new(self)
    }
    #[doc = "Bit 20 - 20:20\\]
Indicates whether the DebugMonitor targets the Secure or the Non-secure state and whether debug events are allowed in Secure state"]
    #[inline(always)]
    #[must_use]
    pub fn sdme(&mut self) -> SDME_W<20> {
        SDME_W::new(self)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved21(&mut self) -> RESERVED21_W<21> {
        RESERVED21_W::new(self)
    }
    #[doc = "Bit 24 - 24:24\\]
Global enable for all DWT and ITM features"]
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
#[doc = "Manages vector catch behavior and DebugMonitor handling when debugging\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [demcr](index.html) module"]
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
