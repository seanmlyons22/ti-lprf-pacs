#[doc = "Register `DCRSR` reader"]
pub struct R(crate::R<DCRSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCRSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCRSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCRSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCRSR` writer"]
pub struct W(crate::W<DCRSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCRSR_SPEC>;
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
impl From<crate::W<DCRSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCRSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGSEL` reader - 4:0\\]
Register select 0x00: R0 0x01: R1 0x02: R2 0x03: R3 0x04: R4 0x05: R5 0x06: R6 0x07: R7 0x08: R8 0x09: R9 0x0A: R10 0x0B: R11 0x0C: R12 0x0D: Current SP 0x0E: LR 0x0F: DebugReturnAddress 0x10: XPSR/flags, execution state information, and exception number 0x11: MSP (Main SP) 0x12: PSP (Process SP) 0x14: CONTROL<<24 | FAULTMASK<<16 | BASEPRI<<8 | PRIMASK"]
pub type REGSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REGSEL` writer - 4:0\\]
Register select 0x00: R0 0x01: R1 0x02: R2 0x03: R3 0x04: R4 0x05: R5 0x06: R6 0x07: R7 0x08: R8 0x09: R9 0x0A: R10 0x0B: R11 0x0C: R12 0x0D: Current SP 0x0E: LR 0x0F: DebugReturnAddress 0x10: XPSR/flags, execution state information, and exception number 0x11: MSP (Main SP) 0x12: PSP (Process SP) 0x14: CONTROL<<24 | FAULTMASK<<16 | BASEPRI<<8 | PRIMASK"]
pub type REGSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCRSR_SPEC, u8, u8, 5, O>;
#[doc = "Field `RESERVED5` reader - 15:5\\]
Software should not rely on the value of a reserved. Write 0."]
pub type RESERVED5_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED5` writer - 15:5\\]
Software should not rely on the value of a reserved. Write 0."]
pub type RESERVED5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCRSR_SPEC, u16, u16, 11, O>;
#[doc = "Field `REGWNR` reader - 16:16\\]
1: Write 0: Read"]
pub type REGWNR_R = crate::BitReader<bool>;
#[doc = "Field `REGWNR` writer - 16:16\\]
1: Write 0: Read"]
pub type REGWNR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCRSR_SPEC, bool, O>;
#[doc = "Field `RESERVED17` reader - 31:17\\]
Software should not rely on the value of a reserved. Write 0."]
pub type RESERVED17_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED17` writer - 31:17\\]
Software should not rely on the value of a reserved. Write 0."]
pub type RESERVED17_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCRSR_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Register select 0x00: R0 0x01: R1 0x02: R2 0x03: R3 0x04: R4 0x05: R5 0x06: R6 0x07: R7 0x08: R8 0x09: R9 0x0A: R10 0x0B: R11 0x0C: R12 0x0D: Current SP 0x0E: LR 0x0F: DebugReturnAddress 0x10: XPSR/flags, execution state information, and exception number 0x11: MSP (Main SP) 0x12: PSP (Process SP) 0x14: CONTROL<<24 | FAULTMASK<<16 | BASEPRI<<8 | PRIMASK"]
    #[inline(always)]
    pub fn regsel(&self) -> REGSEL_R {
        REGSEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:15 - 15:5\\]
Software should not rely on the value of a reserved. Write 0."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new(((self.bits >> 5) & 0x07ff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
1: Write 0: Read"]
    #[inline(always)]
    pub fn regwnr(&self) -> REGWNR_R {
        REGWNR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Software should not rely on the value of a reserved. Write 0."]
    #[inline(always)]
    pub fn reserved17(&self) -> RESERVED17_R {
        RESERVED17_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Register select 0x00: R0 0x01: R1 0x02: R2 0x03: R3 0x04: R4 0x05: R5 0x06: R6 0x07: R7 0x08: R8 0x09: R9 0x0A: R10 0x0B: R11 0x0C: R12 0x0D: Current SP 0x0E: LR 0x0F: DebugReturnAddress 0x10: XPSR/flags, execution state information, and exception number 0x11: MSP (Main SP) 0x12: PSP (Process SP) 0x14: CONTROL<<24 | FAULTMASK<<16 | BASEPRI<<8 | PRIMASK"]
    #[inline(always)]
    #[must_use]
    pub fn regsel(&mut self) -> REGSEL_W<0> {
        REGSEL_W::new(self)
    }
    #[doc = "Bits 5:15 - 15:5\\]
Software should not rely on the value of a reserved. Write 0."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> RESERVED5_W<5> {
        RESERVED5_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
1: Write 0: Read"]
    #[inline(always)]
    #[must_use]
    pub fn regwnr(&mut self) -> REGWNR_W<16> {
        REGWNR_W::new(self)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Software should not rely on the value of a reserved. Write 0."]
    #[inline(always)]
    #[must_use]
    pub fn reserved17(&mut self) -> RESERVED17_W<17> {
        RESERVED17_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Deubg Core Register Selector The purpose of this register is to select the processor register to transfer data to or from. This write-only register generates a handshake to the core to transfer data to or from Debug Core Register Data Register and the selected register. Until this core transaction is complete, DHCSR.S_REGRDY is 0. Note that writes to this register in any size but word are Unpredictable. Note that PSR registers are fully accessible this way, whereas some read as 0 when using MRS instructions. Note that all bits can be written, but some combinations cause a fault when execution is resumed.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcrsr](index.html) module"]
pub struct DCRSR_SPEC;
impl crate::RegisterSpec for DCRSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcrsr::R](R) reader structure"]
impl crate::Readable for DCRSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcrsr::W](W) writer structure"]
impl crate::Writable for DCRSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCRSR to value 0"]
impl crate::Resettable for DCRSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
