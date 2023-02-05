#[doc = "Register `EVSYNCRATE` reader"]
pub struct R(crate::R<EVSYNCRATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVSYNCRATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVSYNCRATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVSYNCRATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVSYNCRATE` writer"]
pub struct W(crate::W<EVSYNCRATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVSYNCRATE_SPEC>;
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
impl From<crate::W<EVSYNCRATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVSYNCRATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED0` writer - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSYNCRATE_SPEC, bool, O>;
#[doc = "Field `AUX_COMPB_SYNC_RATE` reader - 1:1\\]
Select synchronization rate for AUX_EVCTL:EVSTAT2.AUX_COMPB event."]
pub type AUX_COMPB_SYNC_RATE_R = crate::BitReader<AUX_COMPB_SYNC_RATE_A>;
#[doc = "1:1\\]
Select synchronization rate for AUX_EVCTL:EVSTAT2.AUX_COMPB event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUX_COMPB_SYNC_RATE_A {
    #[doc = "1: AUX bus rate"]
    BUS_RATE = 1,
    #[doc = "0: SCE rate"]
    SCE_RATE = 0,
}
impl From<AUX_COMPB_SYNC_RATE_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_COMPB_SYNC_RATE_A) -> Self {
        variant as u8 != 0
    }
}
impl AUX_COMPB_SYNC_RATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_COMPB_SYNC_RATE_A {
        match self.bits {
            true => AUX_COMPB_SYNC_RATE_A::BUS_RATE,
            false => AUX_COMPB_SYNC_RATE_A::SCE_RATE,
        }
    }
    #[doc = "Checks if the value of the field is `BUS_RATE`"]
    #[inline(always)]
    pub fn is_bus_rate(&self) -> bool {
        *self == AUX_COMPB_SYNC_RATE_A::BUS_RATE
    }
    #[doc = "Checks if the value of the field is `SCE_RATE`"]
    #[inline(always)]
    pub fn is_sce_rate(&self) -> bool {
        *self == AUX_COMPB_SYNC_RATE_A::SCE_RATE
    }
}
#[doc = "Field `AUX_COMPB_SYNC_RATE` writer - 1:1\\]
Select synchronization rate for AUX_EVCTL:EVSTAT2.AUX_COMPB event."]
pub type AUX_COMPB_SYNC_RATE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVSYNCRATE_SPEC, AUX_COMPB_SYNC_RATE_A, O>;
impl<'a, const O: u8> AUX_COMPB_SYNC_RATE_W<'a, O> {
    #[doc = "AUX bus rate"]
    #[inline(always)]
    pub fn bus_rate(self) -> &'a mut W {
        self.variant(AUX_COMPB_SYNC_RATE_A::BUS_RATE)
    }
    #[doc = "SCE rate"]
    #[inline(always)]
    pub fn sce_rate(self) -> &'a mut W {
        self.variant(AUX_COMPB_SYNC_RATE_A::SCE_RATE)
    }
}
#[doc = "Field `AUX_COMPA_SYNC_RATE` reader - 2:2\\]
Select synchronization rate for AUX_EVCTL:EVSTAT2.AUX_COMPA event."]
pub type AUX_COMPA_SYNC_RATE_R = crate::BitReader<AUX_COMPA_SYNC_RATE_A>;
#[doc = "2:2\\]
Select synchronization rate for AUX_EVCTL:EVSTAT2.AUX_COMPA event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUX_COMPA_SYNC_RATE_A {
    #[doc = "1: AUX bus rate"]
    BUS_RATE = 1,
    #[doc = "0: SCE rate"]
    SCE_RATE = 0,
}
impl From<AUX_COMPA_SYNC_RATE_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_COMPA_SYNC_RATE_A) -> Self {
        variant as u8 != 0
    }
}
impl AUX_COMPA_SYNC_RATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_COMPA_SYNC_RATE_A {
        match self.bits {
            true => AUX_COMPA_SYNC_RATE_A::BUS_RATE,
            false => AUX_COMPA_SYNC_RATE_A::SCE_RATE,
        }
    }
    #[doc = "Checks if the value of the field is `BUS_RATE`"]
    #[inline(always)]
    pub fn is_bus_rate(&self) -> bool {
        *self == AUX_COMPA_SYNC_RATE_A::BUS_RATE
    }
    #[doc = "Checks if the value of the field is `SCE_RATE`"]
    #[inline(always)]
    pub fn is_sce_rate(&self) -> bool {
        *self == AUX_COMPA_SYNC_RATE_A::SCE_RATE
    }
}
#[doc = "Field `AUX_COMPA_SYNC_RATE` writer - 2:2\\]
Select synchronization rate for AUX_EVCTL:EVSTAT2.AUX_COMPA event."]
pub type AUX_COMPA_SYNC_RATE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVSYNCRATE_SPEC, AUX_COMPA_SYNC_RATE_A, O>;
impl<'a, const O: u8> AUX_COMPA_SYNC_RATE_W<'a, O> {
    #[doc = "AUX bus rate"]
    #[inline(always)]
    pub fn bus_rate(self) -> &'a mut W {
        self.variant(AUX_COMPA_SYNC_RATE_A::BUS_RATE)
    }
    #[doc = "SCE rate"]
    #[inline(always)]
    pub fn sce_rate(self) -> &'a mut W {
        self.variant(AUX_COMPA_SYNC_RATE_A::SCE_RATE)
    }
}
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EVSYNCRATE_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Select synchronization rate for AUX_EVCTL:EVSTAT2.AUX_COMPB event."]
    #[inline(always)]
    pub fn aux_compb_sync_rate(&self) -> AUX_COMPB_SYNC_RATE_R {
        AUX_COMPB_SYNC_RATE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Select synchronization rate for AUX_EVCTL:EVSTAT2.AUX_COMPA event."]
    #[inline(always)]
    pub fn aux_compa_sync_rate(&self) -> AUX_COMPA_SYNC_RATE_R {
        AUX_COMPA_SYNC_RATE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<0> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Select synchronization rate for AUX_EVCTL:EVSTAT2.AUX_COMPB event."]
    #[inline(always)]
    #[must_use]
    pub fn aux_compb_sync_rate(&mut self) -> AUX_COMPB_SYNC_RATE_W<1> {
        AUX_COMPB_SYNC_RATE_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Select synchronization rate for AUX_EVCTL:EVSTAT2.AUX_COMPA event."]
    #[inline(always)]
    #[must_use]
    pub fn aux_compa_sync_rate(&mut self) -> AUX_COMPA_SYNC_RATE_W<2> {
        AUX_COMPA_SYNC_RATE_W::new(self)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<3> {
        RESERVED3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Synchronization Rate Configure synchronization rate for certain events to the synchronous AUX event bus. Select AUX bus rate when system CPU uses the event. SCE rate equals rate configured in AON_PMCTL:AUXSCECLK. AUX bus rate equals SCE rate, or SCLK_HF divided by two when MCU domain is active.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evsyncrate](index.html) module"]
pub struct EVSYNCRATE_SPEC;
impl crate::RegisterSpec for EVSYNCRATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evsyncrate::R](R) reader structure"]
impl crate::Readable for EVSYNCRATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evsyncrate::W](W) writer structure"]
impl crate::Writable for EVSYNCRATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVSYNCRATE to value 0"]
impl crate::Resettable for EVSYNCRATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
