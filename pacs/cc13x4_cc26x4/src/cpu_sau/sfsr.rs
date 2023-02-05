#[doc = "Register `SFSR` reader"]
pub struct R(crate::R<SFSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFSR` writer"]
pub struct W(crate::W<SFSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFSR_SPEC>;
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
impl From<crate::W<SFSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INVEP` reader - 0:0\\]
This bit is set if a function call from the Non-secure state or exception targets a non-SG instruction in the Secure state. This bit is also set if the target address is a SG instruction, but there is no matching SAU/IDAU region with the NSC flag set"]
pub type INVEP_R = crate::BitReader<bool>;
#[doc = "Field `INVEP` writer - 0:0\\]
This bit is set if a function call from the Non-secure state or exception targets a non-SG instruction in the Secure state. This bit is also set if the target address is a SG instruction, but there is no matching SAU/IDAU region with the NSC flag set"]
pub type INVEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFSR_SPEC, bool, O>;
#[doc = "Field `INVIS` reader - 1:1\\]
This bit is set if the integrity signature in an exception stack frame is found to be invalid during the unstacking operation"]
pub type INVIS_R = crate::BitReader<bool>;
#[doc = "Field `INVIS` writer - 1:1\\]
This bit is set if the integrity signature in an exception stack frame is found to be invalid during the unstacking operation"]
pub type INVIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFSR_SPEC, bool, O>;
#[doc = "Field `INVER` reader - 2:2\\]
This can be caused by EXC_RETURN.DCRS being set to 0 when returning from an exception in the Non-secure state, or by EXC_RETURN.ES being set to 1 when returning from an exception in the Non-secure state"]
pub type INVER_R = crate::BitReader<bool>;
#[doc = "Field `INVER` writer - 2:2\\]
This can be caused by EXC_RETURN.DCRS being set to 0 when returning from an exception in the Non-secure state, or by EXC_RETURN.ES being set to 1 when returning from an exception in the Non-secure state"]
pub type INVER_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFSR_SPEC, bool, O>;
#[doc = "Field `AUVIOL` reader - 3:3\\]
Sticky flag indicating that an attempt was made to access parts of the address space that are marked as Secure with NS-Req for the transaction set to Non-secure. This bit is not set if the violation occurred during lazy state preservation. See LSPERR"]
pub type AUVIOL_R = crate::BitReader<bool>;
#[doc = "Field `AUVIOL` writer - 3:3\\]
Sticky flag indicating that an attempt was made to access parts of the address space that are marked as Secure with NS-Req for the transaction set to Non-secure. This bit is not set if the violation occurred during lazy state preservation. See LSPERR"]
pub type AUVIOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFSR_SPEC, bool, O>;
#[doc = "Field `INVTRAN` reader - 4:4\\]
Sticky flag indicating that an exception was raised due to a branch that was not flagged as being domain crossing causing a transition from Secure to Non-secure memory"]
pub type INVTRAN_R = crate::BitReader<bool>;
#[doc = "Field `INVTRAN` writer - 4:4\\]
Sticky flag indicating that an exception was raised due to a branch that was not flagged as being domain crossing causing a transition from Secure to Non-secure memory"]
pub type INVTRAN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFSR_SPEC, bool, O>;
#[doc = "Field `LSPERR` reader - 5:5\\]
Stick flag indicating that an SAU or IDAU violation occurred during the lazy preservation of floating-point state"]
pub type LSPERR_R = crate::BitReader<bool>;
#[doc = "Field `LSPERR` writer - 5:5\\]
Stick flag indicating that an SAU or IDAU violation occurred during the lazy preservation of floating-point state"]
pub type LSPERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFSR_SPEC, bool, O>;
#[doc = "Field `SFARVALID` reader - 6:6\\]
This bit is set when the SFAR register contains a valid value. As with similar fields, such as BFSR.BFARVALID and MMFSR.MMARVALID, this bit can be cleared by other exceptions, such as BusFault"]
pub type SFARVALID_R = crate::BitReader<bool>;
#[doc = "Field `SFARVALID` writer - 6:6\\]
This bit is set when the SFAR register contains a valid value. As with similar fields, such as BFSR.BFARVALID and MMFSR.MMARVALID, this bit can be cleared by other exceptions, such as BusFault"]
pub type SFARVALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFSR_SPEC, bool, O>;
#[doc = "Field `LSERR` reader - 7:7\\]
Sticky flag indicating that an error occurred during lazy state activation or deactivation"]
pub type LSERR_R = crate::BitReader<bool>;
#[doc = "Field `LSERR` writer - 7:7\\]
Sticky flag indicating that an error occurred during lazy state activation or deactivation"]
pub type LSERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFSR_SPEC, bool, O>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SFSR_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This bit is set if a function call from the Non-secure state or exception targets a non-SG instruction in the Secure state. This bit is also set if the target address is a SG instruction, but there is no matching SAU/IDAU region with the NSC flag set"]
    #[inline(always)]
    pub fn invep(&self) -> INVEP_R {
        INVEP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit is set if the integrity signature in an exception stack frame is found to be invalid during the unstacking operation"]
    #[inline(always)]
    pub fn invis(&self) -> INVIS_R {
        INVIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
This can be caused by EXC_RETURN.DCRS being set to 0 when returning from an exception in the Non-secure state, or by EXC_RETURN.ES being set to 1 when returning from an exception in the Non-secure state"]
    #[inline(always)]
    pub fn inver(&self) -> INVER_R {
        INVER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Sticky flag indicating that an attempt was made to access parts of the address space that are marked as Secure with NS-Req for the transaction set to Non-secure. This bit is not set if the violation occurred during lazy state preservation. See LSPERR"]
    #[inline(always)]
    pub fn auviol(&self) -> AUVIOL_R {
        AUVIOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Sticky flag indicating that an exception was raised due to a branch that was not flagged as being domain crossing causing a transition from Secure to Non-secure memory"]
    #[inline(always)]
    pub fn invtran(&self) -> INVTRAN_R {
        INVTRAN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Stick flag indicating that an SAU or IDAU violation occurred during the lazy preservation of floating-point state"]
    #[inline(always)]
    pub fn lsperr(&self) -> LSPERR_R {
        LSPERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
This bit is set when the SFAR register contains a valid value. As with similar fields, such as BFSR.BFARVALID and MMFSR.MMARVALID, this bit can be cleared by other exceptions, such as BusFault"]
    #[inline(always)]
    pub fn sfarvalid(&self) -> SFARVALID_R {
        SFARVALID_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Sticky flag indicating that an error occurred during lazy state activation or deactivation"]
    #[inline(always)]
    pub fn lserr(&self) -> LSERR_R {
        LSERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This bit is set if a function call from the Non-secure state or exception targets a non-SG instruction in the Secure state. This bit is also set if the target address is a SG instruction, but there is no matching SAU/IDAU region with the NSC flag set"]
    #[inline(always)]
    #[must_use]
    pub fn invep(&mut self) -> INVEP_W<0> {
        INVEP_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit is set if the integrity signature in an exception stack frame is found to be invalid during the unstacking operation"]
    #[inline(always)]
    #[must_use]
    pub fn invis(&mut self) -> INVIS_W<1> {
        INVIS_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
This can be caused by EXC_RETURN.DCRS being set to 0 when returning from an exception in the Non-secure state, or by EXC_RETURN.ES being set to 1 when returning from an exception in the Non-secure state"]
    #[inline(always)]
    #[must_use]
    pub fn inver(&mut self) -> INVER_W<2> {
        INVER_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Sticky flag indicating that an attempt was made to access parts of the address space that are marked as Secure with NS-Req for the transaction set to Non-secure. This bit is not set if the violation occurred during lazy state preservation. See LSPERR"]
    #[inline(always)]
    #[must_use]
    pub fn auviol(&mut self) -> AUVIOL_W<3> {
        AUVIOL_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Sticky flag indicating that an exception was raised due to a branch that was not flagged as being domain crossing causing a transition from Secure to Non-secure memory"]
    #[inline(always)]
    #[must_use]
    pub fn invtran(&mut self) -> INVTRAN_W<4> {
        INVTRAN_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Stick flag indicating that an SAU or IDAU violation occurred during the lazy preservation of floating-point state"]
    #[inline(always)]
    #[must_use]
    pub fn lsperr(&mut self) -> LSPERR_W<5> {
        LSPERR_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
This bit is set when the SFAR register contains a valid value. As with similar fields, such as BFSR.BFARVALID and MMFSR.MMARVALID, this bit can be cleared by other exceptions, such as BusFault"]
    #[inline(always)]
    #[must_use]
    pub fn sfarvalid(&mut self) -> SFARVALID_W<6> {
        SFARVALID_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Sticky flag indicating that an error occurred during lazy state activation or deactivation"]
    #[inline(always)]
    #[must_use]
    pub fn lserr(&mut self) -> LSERR_W<7> {
        LSERR_W::new(self)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> RESERVED8_W<8> {
        RESERVED8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Provides information about any security related faults\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfsr](index.html) module"]
pub struct SFSR_SPEC;
impl crate::RegisterSpec for SFSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sfsr::R](R) reader structure"]
impl crate::Readable for SFSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfsr::W](W) writer structure"]
impl crate::Writable for SFSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SFSR to value 0"]
impl crate::Resettable for SFSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
