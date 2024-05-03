#[doc = "Register `SFSR` reader"]
pub type R = crate::R<SfsrSpec>;
#[doc = "Register `SFSR` writer"]
pub type W = crate::W<SfsrSpec>;
#[doc = "Field `INVEP` reader - 0:0\\]
This bit is set if a function call from the Non-secure state or exception targets a non-SG instruction in the Secure state. This bit is also set if the target address is a SG instruction, but there is no matching SAU/IDAU region with the NSC flag set"]
pub type InvepR = crate::BitReader;
#[doc = "Field `INVEP` writer - 0:0\\]
This bit is set if a function call from the Non-secure state or exception targets a non-SG instruction in the Secure state. This bit is also set if the target address is a SG instruction, but there is no matching SAU/IDAU region with the NSC flag set"]
pub type InvepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVIS` reader - 1:1\\]
This bit is set if the integrity signature in an exception stack frame is found to be invalid during the unstacking operation"]
pub type InvisR = crate::BitReader;
#[doc = "Field `INVIS` writer - 1:1\\]
This bit is set if the integrity signature in an exception stack frame is found to be invalid during the unstacking operation"]
pub type InvisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVER` reader - 2:2\\]
This can be caused by EXC_RETURN.DCRS being set to 0 when returning from an exception in the Non-secure state, or by EXC_RETURN.ES being set to 1 when returning from an exception in the Non-secure state"]
pub type InverR = crate::BitReader;
#[doc = "Field `INVER` writer - 2:2\\]
This can be caused by EXC_RETURN.DCRS being set to 0 when returning from an exception in the Non-secure state, or by EXC_RETURN.ES being set to 1 when returning from an exception in the Non-secure state"]
pub type InverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUVIOL` reader - 3:3\\]
Sticky flag indicating that an attempt was made to access parts of the address space that are marked as Secure with NS-Req for the transaction set to Non-secure. This bit is not set if the violation occurred during lazy state preservation. See LSPERR"]
pub type AuviolR = crate::BitReader;
#[doc = "Field `AUVIOL` writer - 3:3\\]
Sticky flag indicating that an attempt was made to access parts of the address space that are marked as Secure with NS-Req for the transaction set to Non-secure. This bit is not set if the violation occurred during lazy state preservation. See LSPERR"]
pub type AuviolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVTRAN` reader - 4:4\\]
Sticky flag indicating that an exception was raised due to a branch that was not flagged as being domain crossing causing a transition from Secure to Non-secure memory"]
pub type InvtranR = crate::BitReader;
#[doc = "Field `INVTRAN` writer - 4:4\\]
Sticky flag indicating that an exception was raised due to a branch that was not flagged as being domain crossing causing a transition from Secure to Non-secure memory"]
pub type InvtranW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSPERR` reader - 5:5\\]
Stick flag indicating that an SAU or IDAU violation occurred during the lazy preservation of floating-point state"]
pub type LsperrR = crate::BitReader;
#[doc = "Field `LSPERR` writer - 5:5\\]
Stick flag indicating that an SAU or IDAU violation occurred during the lazy preservation of floating-point state"]
pub type LsperrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFARVALID` reader - 6:6\\]
This bit is set when the SFAR register contains a valid value. As with similar fields, such as BFSR.BFARVALID and MMFSR.MMARVALID, this bit can be cleared by other exceptions, such as BusFault"]
pub type SfarvalidR = crate::BitReader;
#[doc = "Field `SFARVALID` writer - 6:6\\]
This bit is set when the SFAR register contains a valid value. As with similar fields, such as BFSR.BFARVALID and MMFSR.MMARVALID, this bit can be cleared by other exceptions, such as BusFault"]
pub type SfarvalidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERR` reader - 7:7\\]
Sticky flag indicating that an error occurred during lazy state activation or deactivation"]
pub type LserrR = crate::BitReader;
#[doc = "Field `LSERR` writer - 7:7\\]
Sticky flag indicating that an error occurred during lazy state activation or deactivation"]
pub type LserrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This bit is set if a function call from the Non-secure state or exception targets a non-SG instruction in the Secure state. This bit is also set if the target address is a SG instruction, but there is no matching SAU/IDAU region with the NSC flag set"]
    #[inline(always)]
    pub fn invep(&self) -> InvepR {
        InvepR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit is set if the integrity signature in an exception stack frame is found to be invalid during the unstacking operation"]
    #[inline(always)]
    pub fn invis(&self) -> InvisR {
        InvisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
This can be caused by EXC_RETURN.DCRS being set to 0 when returning from an exception in the Non-secure state, or by EXC_RETURN.ES being set to 1 when returning from an exception in the Non-secure state"]
    #[inline(always)]
    pub fn inver(&self) -> InverR {
        InverR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Sticky flag indicating that an attempt was made to access parts of the address space that are marked as Secure with NS-Req for the transaction set to Non-secure. This bit is not set if the violation occurred during lazy state preservation. See LSPERR"]
    #[inline(always)]
    pub fn auviol(&self) -> AuviolR {
        AuviolR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Sticky flag indicating that an exception was raised due to a branch that was not flagged as being domain crossing causing a transition from Secure to Non-secure memory"]
    #[inline(always)]
    pub fn invtran(&self) -> InvtranR {
        InvtranR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Stick flag indicating that an SAU or IDAU violation occurred during the lazy preservation of floating-point state"]
    #[inline(always)]
    pub fn lsperr(&self) -> LsperrR {
        LsperrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
This bit is set when the SFAR register contains a valid value. As with similar fields, such as BFSR.BFARVALID and MMFSR.MMARVALID, this bit can be cleared by other exceptions, such as BusFault"]
    #[inline(always)]
    pub fn sfarvalid(&self) -> SfarvalidR {
        SfarvalidR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Sticky flag indicating that an error occurred during lazy state activation or deactivation"]
    #[inline(always)]
    pub fn lserr(&self) -> LserrR {
        LserrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This bit is set if a function call from the Non-secure state or exception targets a non-SG instruction in the Secure state. This bit is also set if the target address is a SG instruction, but there is no matching SAU/IDAU region with the NSC flag set"]
    #[inline(always)]
    #[must_use]
    pub fn invep(&mut self) -> InvepW<SfsrSpec> {
        InvepW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit is set if the integrity signature in an exception stack frame is found to be invalid during the unstacking operation"]
    #[inline(always)]
    #[must_use]
    pub fn invis(&mut self) -> InvisW<SfsrSpec> {
        InvisW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
This can be caused by EXC_RETURN.DCRS being set to 0 when returning from an exception in the Non-secure state, or by EXC_RETURN.ES being set to 1 when returning from an exception in the Non-secure state"]
    #[inline(always)]
    #[must_use]
    pub fn inver(&mut self) -> InverW<SfsrSpec> {
        InverW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Sticky flag indicating that an attempt was made to access parts of the address space that are marked as Secure with NS-Req for the transaction set to Non-secure. This bit is not set if the violation occurred during lazy state preservation. See LSPERR"]
    #[inline(always)]
    #[must_use]
    pub fn auviol(&mut self) -> AuviolW<SfsrSpec> {
        AuviolW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Sticky flag indicating that an exception was raised due to a branch that was not flagged as being domain crossing causing a transition from Secure to Non-secure memory"]
    #[inline(always)]
    #[must_use]
    pub fn invtran(&mut self) -> InvtranW<SfsrSpec> {
        InvtranW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Stick flag indicating that an SAU or IDAU violation occurred during the lazy preservation of floating-point state"]
    #[inline(always)]
    #[must_use]
    pub fn lsperr(&mut self) -> LsperrW<SfsrSpec> {
        LsperrW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
This bit is set when the SFAR register contains a valid value. As with similar fields, such as BFSR.BFARVALID and MMFSR.MMARVALID, this bit can be cleared by other exceptions, such as BusFault"]
    #[inline(always)]
    #[must_use]
    pub fn sfarvalid(&mut self) -> SfarvalidW<SfsrSpec> {
        SfarvalidW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Sticky flag indicating that an error occurred during lazy state activation or deactivation"]
    #[inline(always)]
    #[must_use]
    pub fn lserr(&mut self) -> LserrW<SfsrSpec> {
        LserrW::new(self, 7)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<SfsrSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Provides information about any security related faults\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfsrSpec;
impl crate::RegisterSpec for SfsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfsr::R`](R) reader structure"]
impl crate::Readable for SfsrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfsr::W`](W) writer structure"]
impl crate::Writable for SfsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SFSR to value 0"]
impl crate::Resettable for SfsrSpec {
    const RESET_VALUE: u32 = 0;
}
