#[doc = "Register `CMDWEPROTNM` reader"]
pub struct R(crate::R<CMDWEPROTNM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDWEPROTNM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMDWEPROTNM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMDWEPROTNM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMDWEPROTNM` writer"]
pub struct W(crate::W<CMDWEPROTNM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMDWEPROTNM_SPEC>;
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
impl From<crate::W<CMDWEPROTNM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMDWEPROTNM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VAL` reader - 0:0\\]
Each bit protects 1 sector. bit \\[0\\]:When 1, sector 0 of the non-main region will be protected from program and erase. bit \\[1\\]:When 1, sector 1 of the non-main region will be protected from program and erase. : : bit \\[31\\]:When 1, sector 31 of the non-main will be protected from program and erase."]
pub type VAL_R = crate::BitReader<VAL_A>;
#[doc = "0:0\\]
Each bit protects 1 sector. bit \\[0\\]:When 1, sector 0 of the non-main region will be protected from program and erase. bit \\[1\\]:When 1, sector 1 of the non-main region will be protected from program and erase. : : bit \\[31\\]:When 1, sector 31 of the non-main will be protected from program and erase.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VAL_A {
    #[doc = "1: Maximum value of VAL"]
    MAXIMUM = 1,
    #[doc = "0: Minimum value of VAL"]
    MINIMUM = 0,
}
impl From<VAL_A> for bool {
    #[inline(always)]
    fn from(variant: VAL_A) -> Self {
        variant as u8 != 0
    }
}
impl VAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VAL_A {
        match self.bits {
            true => VAL_A::MAXIMUM,
            false => VAL_A::MINIMUM,
        }
    }
    #[doc = "Checks if the value of the field is `MAXIMUM`"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == VAL_A::MAXIMUM
    }
    #[doc = "Checks if the value of the field is `MINIMUM`"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == VAL_A::MINIMUM
    }
}
#[doc = "Field `VAL` writer - 0:0\\]
Each bit protects 1 sector. bit \\[0\\]:When 1, sector 0 of the non-main region will be protected from program and erase. bit \\[1\\]:When 1, sector 1 of the non-main region will be protected from program and erase. : : bit \\[31\\]:When 1, sector 31 of the non-main will be protected from program and erase."]
pub type VAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDWEPROTNM_SPEC, VAL_A, O>;
impl<'a, const O: u8> VAL_W<'a, O> {
    #[doc = "Maximum value of VAL"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut W {
        self.variant(VAL_A::MAXIMUM)
    }
    #[doc = "Minimum value of VAL"]
    #[inline(always)]
    pub fn minimum(self) -> &'a mut W {
        self.variant(VAL_A::MINIMUM)
    }
}
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CMDWEPROTNM_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Each bit protects 1 sector. bit \\[0\\]:When 1, sector 0 of the non-main region will be protected from program and erase. bit \\[1\\]:When 1, sector 1 of the non-main region will be protected from program and erase. : : bit \\[31\\]:When 1, sector 31 of the non-main will be protected from program and erase."]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new((self.bits & 1) != 0)
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
Each bit protects 1 sector. bit \\[0\\]:When 1, sector 0 of the non-main region will be protected from program and erase. bit \\[1\\]:When 1, sector 1 of the non-main region will be protected from program and erase. : : bit \\[31\\]:When 1, sector 31 of the non-main will be protected from program and erase."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> VAL_W<0> {
        VAL_W::new(self)
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
#[doc = "Command WriteErase Protect Non-Main Register This register allows non-main region region sectors to be protected from program and erase. Each bit corresponds to 1 sector. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all NoWrapper commands.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdweprotnm](index.html) module"]
pub struct CMDWEPROTNM_SPEC;
impl crate::RegisterSpec for CMDWEPROTNM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmdweprotnm::R](R) reader structure"]
impl crate::Readable for CMDWEPROTNM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmdweprotnm::W](W) writer structure"]
impl crate::Writable for CMDWEPROTNM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMDWEPROTNM to value 0x01"]
impl crate::Resettable for CMDWEPROTNM_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
