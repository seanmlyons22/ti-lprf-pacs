#[doc = "Register `DFTPCLKTESTSTAT` reader"]
pub struct R(crate::R<DFTPCLKTESTSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFTPCLKTESTSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFTPCLKTESTSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFTPCLKTESTSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFTPCLKTESTSTAT` writer"]
pub struct W(crate::W<DFTPCLKTESTSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFTPCLKTESTSTAT_SPEC>;
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
impl From<crate::W<DFTPCLKTESTSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFTPCLKTESTSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUSY` reader - 0:0\\]
Indicates that a pump clock measurement is in progress."]
pub type BUSY_R = crate::BitReader<BUSY_A>;
#[doc = "0:0\\]
Indicates that a pump clock measurement is in progress.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY_A {
    #[doc = "1: Indicates test in progress"]
    INPROGRESS = 1,
    #[doc = "0: Indicates test complete"]
    COMPLETE = 0,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            true => BUSY_A::INPROGRESS,
            false => BUSY_A::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `INPROGRESS`"]
    #[inline(always)]
    pub fn is_inprogress(&self) -> bool {
        *self == BUSY_A::INPROGRESS
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == BUSY_A::COMPLETE
    }
}
#[doc = "Field `BUSY` writer - 0:0\\]
Indicates that a pump clock measurement is in progress."]
pub type BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFTPCLKTESTSTAT_SPEC, BUSY_A, O>;
impl<'a, const O: u8> BUSY_W<'a, O> {
    #[doc = "Indicates test in progress"]
    #[inline(always)]
    pub fn inprogress(self) -> &'a mut W {
        self.variant(BUSY_A::INPROGRESS)
    }
    #[doc = "Indicates test complete"]
    #[inline(always)]
    pub fn complete(self) -> &'a mut W {
        self.variant(BUSY_A::COMPLETE)
    }
}
#[doc = "Field `RESERVED1` reader - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED1` writer - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFTPCLKTESTSTAT_SPEC, u8, u8, 3, O>;
#[doc = "Field `CLOCKCNT` reader - 15:4\\]
Indicates the core clock count captured during the pump clock measurement."]
pub type CLOCKCNT_R = crate::FieldReader<u16, CLOCKCNT_A>;
#[doc = "15:4\\]
Indicates the core clock count captured during the pump clock measurement.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum CLOCKCNT_A {
    #[doc = "4095: Maximum count value"]
    MAXIMUM = 4095,
    #[doc = "0: Minimum count value"]
    MINIMUM = 0,
}
impl From<CLOCKCNT_A> for u16 {
    #[inline(always)]
    fn from(variant: CLOCKCNT_A) -> Self {
        variant as _
    }
}
impl CLOCKCNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLOCKCNT_A> {
        match self.bits {
            4095 => Some(CLOCKCNT_A::MAXIMUM),
            0 => Some(CLOCKCNT_A::MINIMUM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAXIMUM`"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == CLOCKCNT_A::MAXIMUM
    }
    #[doc = "Checks if the value of the field is `MINIMUM`"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == CLOCKCNT_A::MINIMUM
    }
}
#[doc = "Field `CLOCKCNT` writer - 15:4\\]
Indicates the core clock count captured during the pump clock measurement."]
pub type CLOCKCNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFTPCLKTESTSTAT_SPEC, u16, CLOCKCNT_A, 12, O>;
impl<'a, const O: u8> CLOCKCNT_W<'a, O> {
    #[doc = "Maximum count value"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut W {
        self.variant(CLOCKCNT_A::MAXIMUM)
    }
    #[doc = "Minimum count value"]
    #[inline(always)]
    pub fn minimum(self) -> &'a mut W {
        self.variant(CLOCKCNT_A::MINIMUM)
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates that a pump clock measurement is in progress."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Indicates the core clock count captured during the pump clock measurement."]
    #[inline(always)]
    pub fn clockcnt(&self) -> CLOCKCNT_R {
        CLOCKCNT_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates that a pump clock measurement is in progress."]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BUSY_W<0> {
        BUSY_W::new(self)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Indicates the core clock count captured during the pump clock measurement."]
    #[inline(always)]
    #[must_use]
    pub fn clockcnt(&mut self) -> CLOCKCNT_W<4> {
        CLOCKCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFT Pump Clock Test Status Register. This register shows status reported by the hardware features that allow the pump clock to be characterized for trim development. This register is only writable when DFTEN.ENABLE is set.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dftpclkteststat](index.html) module"]
pub struct DFTPCLKTESTSTAT_SPEC;
impl crate::RegisterSpec for DFTPCLKTESTSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dftpclkteststat::R](R) reader structure"]
impl crate::Readable for DFTPCLKTESTSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dftpclkteststat::W](W) writer structure"]
impl crate::Writable for DFTPCLKTESTSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFTPCLKTESTSTAT to value 0"]
impl crate::Resettable for DFTPCLKTESTSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
