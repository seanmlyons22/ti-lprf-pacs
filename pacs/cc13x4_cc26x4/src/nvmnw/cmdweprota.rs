#[doc = "Register `CMDWEPROTA` reader"]
pub struct R(crate::R<CMDWEPROTA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDWEPROTA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMDWEPROTA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMDWEPROTA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMDWEPROTA` writer"]
pub struct W(crate::W<CMDWEPROTA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMDWEPROTA_SPEC>;
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
impl From<crate::W<CMDWEPROTA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMDWEPROTA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VAL` reader - 31:0\\]
Each bit protects 1 sector. bit \\[0\\]:When 1, sector 0 of the flash memory will be protected from program and erase. bit \\[1\\]:When 1, sector 1 of the flash memory will be protected from program and erase. : : bit \\[31\\]:When 1, sector 31 of the flash memory will be protected from program and erase."]
pub type VAL_R = crate::FieldReader<u32, VAL_A>;
#[doc = "31:0\\]
Each bit protects 1 sector. bit \\[0\\]:When 1, sector 0 of the flash memory will be protected from program and erase. bit \\[1\\]:When 1, sector 1 of the flash memory will be protected from program and erase. : : bit \\[31\\]:When 1, sector 31 of the flash memory will be protected from program and erase.\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum VAL_A {
    #[doc = "4294967295: Maximum value of VAL"]
    MAXIMUM = 4294967295,
    #[doc = "0: Minimum value of VAL"]
    MINIMUM = 0,
}
impl From<VAL_A> for u32 {
    #[inline(always)]
    fn from(variant: VAL_A) -> Self {
        variant as _
    }
}
impl VAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VAL_A> {
        match self.bits {
            4294967295 => Some(VAL_A::MAXIMUM),
            0 => Some(VAL_A::MINIMUM),
            _ => None,
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
#[doc = "Field `VAL` writer - 31:0\\]
Each bit protects 1 sector. bit \\[0\\]:When 1, sector 0 of the flash memory will be protected from program and erase. bit \\[1\\]:When 1, sector 1 of the flash memory will be protected from program and erase. : : bit \\[31\\]:When 1, sector 31 of the flash memory will be protected from program and erase."]
pub type VAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMDWEPROTA_SPEC, u32, VAL_A, 32, O>;
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
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Each bit protects 1 sector. bit \\[0\\]:When 1, sector 0 of the flash memory will be protected from program and erase. bit \\[1\\]:When 1, sector 1 of the flash memory will be protected from program and erase. : : bit \\[31\\]:When 1, sector 31 of the flash memory will be protected from program and erase."]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Each bit protects 1 sector. bit \\[0\\]:When 1, sector 0 of the flash memory will be protected from program and erase. bit \\[1\\]:When 1, sector 1 of the flash memory will be protected from program and erase. : : bit \\[31\\]:When 1, sector 31 of the flash memory will be protected from program and erase."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> VAL_W<0> {
        VAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command WriteErase Protect A Register This register allows the first 32 sectors of the main region to be protected from program or erase, with 1 bit protecting each sector. If the main region size is smaller than 32 sectors, then this register provides protection for the whole region. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all NoWrapper commands.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdweprota](index.html) module"]
pub struct CMDWEPROTA_SPEC;
impl crate::RegisterSpec for CMDWEPROTA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmdweprota::R](R) reader structure"]
impl crate::Readable for CMDWEPROTA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmdweprota::W](W) writer structure"]
impl crate::Writable for CMDWEPROTA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMDWEPROTA to value 0xffff_ffff"]
impl crate::Resettable for CMDWEPROTA_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
