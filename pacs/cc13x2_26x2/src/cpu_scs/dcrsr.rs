#[doc = "Register `DCRSR` reader"]
pub type R = crate::R<DcrsrSpec>;
#[doc = "Register `DCRSR` writer"]
pub type W = crate::W<DcrsrSpec>;
#[doc = "Field `REGSEL` writer - 4:0\\]
Register select 0x00: R0 0x01: R1 0x02: R2 0x03: R3 0x04: R4 0x05: R5 0x06: R6 0x07: R7 0x08: R8 0x09: R9 0x0A: R10 0x0B: R11 0x0C: R12 0x0D: Current SP 0x0E: LR 0x0F: DebugReturnAddress 0x10: XPSR/flags, execution state information, and exception number 0x11: MSP (Main SP) 0x12: PSP (Process SP) 0x14: CONTROL&lt;&lt;24 | FAULTMASK&lt;&lt;16 | BASEPRI&lt;&lt;8 | PRIMASK"]
pub type RegselW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RESERVED5` writer - 15:5\\]
Software should not rely on the value of a reserved. Write 0."]
pub type Reserved5W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `REGWNR` writer - 16:16\\]
1: Write 0: Read"]
pub type RegwnrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED17` writer - 31:17\\]
Software should not rely on the value of a reserved. Write 0."]
pub type Reserved17W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Register select 0x00: R0 0x01: R1 0x02: R2 0x03: R3 0x04: R4 0x05: R5 0x06: R6 0x07: R7 0x08: R8 0x09: R9 0x0A: R10 0x0B: R11 0x0C: R12 0x0D: Current SP 0x0E: LR 0x0F: DebugReturnAddress 0x10: XPSR/flags, execution state information, and exception number 0x11: MSP (Main SP) 0x12: PSP (Process SP) 0x14: CONTROL&lt;&lt;24 | FAULTMASK&lt;&lt;16 | BASEPRI&lt;&lt;8 | PRIMASK"]
    #[inline(always)]
    #[must_use]
    pub fn regsel(&mut self) -> RegselW<DcrsrSpec> {
        RegselW::new(self, 0)
    }
    #[doc = "Bits 5:15 - 15:5\\]
Software should not rely on the value of a reserved. Write 0."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> Reserved5W<DcrsrSpec> {
        Reserved5W::new(self, 5)
    }
    #[doc = "Bit 16 - 16:16\\]
1: Write 0: Read"]
    #[inline(always)]
    #[must_use]
    pub fn regwnr(&mut self) -> RegwnrW<DcrsrSpec> {
        RegwnrW::new(self, 16)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Software should not rely on the value of a reserved. Write 0."]
    #[inline(always)]
    #[must_use]
    pub fn reserved17(&mut self) -> Reserved17W<DcrsrSpec> {
        Reserved17W::new(self, 17)
    }
}
#[doc = "Deubg Core Register Selector The purpose of this register is to select the processor register to transfer data to or from. This write-only register generates a handshake to the core to transfer data to or from Debug Core Register Data Register and the selected register. Until this core transaction is complete, DHCSR.S_REGRDY is 0. Note that writes to this register in any size but word are Unpredictable. Note that PSR registers are fully accessible this way, whereas some read as 0 when using MRS instructions. Note that all bits can be written, but some combinations cause a fault when execution is resumed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcrsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcrsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcrsrSpec;
impl crate::RegisterSpec for DcrsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcrsr::R`](R) reader structure"]
impl crate::Readable for DcrsrSpec {}
#[doc = "`write(|w| ..)` method takes [`dcrsr::W`](W) writer structure"]
impl crate::Writable for DcrsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCRSR to value 0"]
impl crate::Resettable for DcrsrSpec {
    const RESET_VALUE: u32 = 0;
}
