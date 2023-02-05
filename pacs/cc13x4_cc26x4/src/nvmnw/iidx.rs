#[doc = "Register `IIDX` reader"]
pub struct R(crate::R<IIDX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IIDX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IIDX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IIDX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IIDX` writer"]
pub struct W(crate::W<IIDX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IIDX_SPEC>;
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
impl From<crate::W<IIDX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IIDX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STAT` reader - 0:0\\]
Indicates which interrupt has fired. 0x0 means no event pending. The priority order is fixed. On each read, only one interrupt is indicated. On a read, the current interrupt (highest priority) is automatically cleared by the hardware and the corresponding interrupt flags in the RIS and MIS are cleared as well. After a read from the CPU (not from the debug interface), the register must be updated with the next highest priority interrupt."]
pub type STAT_R = crate::BitReader<STAT_A>;
#[doc = "0:0\\]
Indicates which interrupt has fired. 0x0 means no event pending. The priority order is fixed. On each read, only one interrupt is indicated. On a read, the current interrupt (highest priority) is automatically cleared by the hardware and the corresponding interrupt flags in the RIS and MIS are cleared as well. After a read from the CPU (not from the debug interface), the register must be updated with the next highest priority interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_A {
    #[doc = "1: DONE Interrupt Pending"]
    DONE = 1,
    #[doc = "0: No Interrupt Pending"]
    NO_INTR = 0,
}
impl From<STAT_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_A {
        match self.bits {
            true => STAT_A::DONE,
            false => STAT_A::NO_INTR,
        }
    }
    #[doc = "Checks if the value of the field is `DONE`"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == STAT_A::DONE
    }
    #[doc = "Checks if the value of the field is `NO_INTR`"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == STAT_A::NO_INTR
    }
}
#[doc = "Field `STAT` writer - 0:0\\]
Indicates which interrupt has fired. 0x0 means no event pending. The priority order is fixed. On each read, only one interrupt is indicated. On a read, the current interrupt (highest priority) is automatically cleared by the hardware and the corresponding interrupt flags in the RIS and MIS are cleared as well. After a read from the CPU (not from the debug interface), the register must be updated with the next highest priority interrupt."]
pub type STAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IIDX_SPEC, STAT_A, O>;
impl<'a, const O: u8> STAT_W<'a, O> {
    #[doc = "DONE Interrupt Pending"]
    #[inline(always)]
    pub fn done(self) -> &'a mut W {
        self.variant(STAT_A::DONE)
    }
    #[doc = "No Interrupt Pending"]
    #[inline(always)]
    pub fn no_intr(self) -> &'a mut W {
        self.variant(STAT_A::NO_INTR)
    }
}
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IIDX_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates which interrupt has fired. 0x0 means no event pending. The priority order is fixed. On each read, only one interrupt is indicated. On a read, the current interrupt (highest priority) is automatically cleared by the hardware and the corresponding interrupt flags in the RIS and MIS are cleared as well. After a read from the CPU (not from the debug interface), the register must be updated with the next highest priority interrupt."]
    #[inline(always)]
    pub fn stat(&self) -> STAT_R {
        STAT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates which interrupt has fired. 0x0 means no event pending. The priority order is fixed. On each read, only one interrupt is indicated. On a read, the current interrupt (highest priority) is automatically cleared by the hardware and the corresponding interrupt flags in the RIS and MIS are cleared as well. After a read from the CPU (not from the debug interface), the register must be updated with the next highest priority interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn stat(&mut self) -> STAT_W<0> {
        STAT_W::new(self)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Index Register: The IIDX register provides the highest priority enabled interrupt index. PSD compliant register. Note that it is not recommended to use this register if the system clock is running at a slower clock frequency than the NoWrapper clock. If this is the case, then reading this register may fail to update the RIS register correctly. The MIS register should be read directly, and a write to ICLR should be used to clear interrupts when this clock relationship is present.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iidx](index.html) module"]
pub struct IIDX_SPEC;
impl crate::RegisterSpec for IIDX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iidx::R](R) reader structure"]
impl crate::Readable for IIDX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iidx::W](W) writer structure"]
impl crate::Writable for IIDX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IIDX to value 0"]
impl crate::Resettable for IIDX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
