#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTEN` reader - 0:0\\]
WDT Interrupt Enable 0: Interrupt event disabled. 1: Interrupt event enabled. Once set, this bit can only be cleared by a hardware reset."]
pub type INTEN_R = crate::BitReader<INTEN_A>;
#[doc = "0:0\\]
WDT Interrupt Enable 0: Interrupt event disabled. 1: Interrupt event enabled. Once set, this bit can only be cleared by a hardware reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN_A {
    #[doc = "1: Interrupt Enabled"]
    EN = 1,
    #[doc = "0: Interrupt Disabled"]
    DIS = 0,
}
impl From<INTEN_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN_A {
        match self.bits {
            true => INTEN_A::EN,
            false => INTEN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == INTEN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == INTEN_A::DIS
    }
}
#[doc = "Field `INTEN` writer - 0:0\\]
WDT Interrupt Enable 0: Interrupt event disabled. 1: Interrupt event enabled. Once set, this bit can only be cleared by a hardware reset."]
pub type INTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, INTEN_A, O>;
impl<'a, const O: u8> INTEN_W<'a, O> {
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(INTEN_A::EN)
    }
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(INTEN_A::DIS)
    }
}
#[doc = "Field `RESEN` reader - 1:1\\]
WDT Reset Enable. Defines the function of the WDT reset source (see PRCM:WARMRESET.WDT_STAT if enabled) 0: Disabled. 1: Enable the Watchdog reset output."]
pub type RESEN_R = crate::BitReader<RESEN_A>;
#[doc = "1:1\\]
WDT Reset Enable. Defines the function of the WDT reset source (see PRCM:WARMRESET.WDT_STAT if enabled) 0: Disabled. 1: Enable the Watchdog reset output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESEN_A {
    #[doc = "1: Reset output Enabled"]
    EN = 1,
    #[doc = "0: Reset output Disabled"]
    DIS = 0,
}
impl From<RESEN_A> for bool {
    #[inline(always)]
    fn from(variant: RESEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RESEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESEN_A {
        match self.bits {
            true => RESEN_A::EN,
            false => RESEN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RESEN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RESEN_A::DIS
    }
}
#[doc = "Field `RESEN` writer - 1:1\\]
WDT Reset Enable. Defines the function of the WDT reset source (see PRCM:WARMRESET.WDT_STAT if enabled) 0: Disabled. 1: Enable the Watchdog reset output."]
pub type RESEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, RESEN_A, O>;
impl<'a, const O: u8> RESEN_W<'a, O> {
    #[doc = "Reset output Enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RESEN_A::EN)
    }
    #[doc = "Reset output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RESEN_A::DIS)
    }
}
#[doc = "Field `INTTYPE` reader - 2:2\\]
WDT Interrupt Type 0: WDT interrupt is a standard interrupt. 1: WDT interrupt is a non-maskable interrupt."]
pub type INTTYPE_R = crate::BitReader<INTTYPE_A>;
#[doc = "2:2\\]
WDT Interrupt Type 0: WDT interrupt is a standard interrupt. 1: WDT interrupt is a non-maskable interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTTYPE_A {
    #[doc = "1: Non-maskable interrupt"]
    NONMASKABLE = 1,
    #[doc = "0: Maskable interrupt"]
    MASKABLE = 0,
}
impl From<INTTYPE_A> for bool {
    #[inline(always)]
    fn from(variant: INTTYPE_A) -> Self {
        variant as u8 != 0
    }
}
impl INTTYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTTYPE_A {
        match self.bits {
            true => INTTYPE_A::NONMASKABLE,
            false => INTTYPE_A::MASKABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NONMASKABLE`"]
    #[inline(always)]
    pub fn is_nonmaskable(&self) -> bool {
        *self == INTTYPE_A::NONMASKABLE
    }
    #[doc = "Checks if the value of the field is `MASKABLE`"]
    #[inline(always)]
    pub fn is_maskable(&self) -> bool {
        *self == INTTYPE_A::MASKABLE
    }
}
#[doc = "Field `INTTYPE` writer - 2:2\\]
WDT Interrupt Type 0: WDT interrupt is a standard interrupt. 1: WDT interrupt is a non-maskable interrupt."]
pub type INTTYPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, INTTYPE_A, O>;
impl<'a, const O: u8> INTTYPE_W<'a, O> {
    #[doc = "Non-maskable interrupt"]
    #[inline(always)]
    pub fn nonmaskable(self) -> &'a mut W {
        self.variant(INTTYPE_A::NONMASKABLE)
    }
    #[doc = "Maskable interrupt"]
    #[inline(always)]
    pub fn maskable(self) -> &'a mut W {
        self.variant(INTTYPE_A::MASKABLE)
    }
}
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
WDT Interrupt Enable 0: Interrupt event disabled. 1: Interrupt event enabled. Once set, this bit can only be cleared by a hardware reset."]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
WDT Reset Enable. Defines the function of the WDT reset source (see PRCM:WARMRESET.WDT_STAT if enabled) 0: Disabled. 1: Enable the Watchdog reset output."]
    #[inline(always)]
    pub fn resen(&self) -> RESEN_R {
        RESEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
WDT Interrupt Type 0: WDT interrupt is a standard interrupt. 1: WDT interrupt is a non-maskable interrupt."]
    #[inline(always)]
    pub fn inttype(&self) -> INTTYPE_R {
        INTTYPE_R::new(((self.bits >> 2) & 1) != 0)
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
WDT Interrupt Enable 0: Interrupt event disabled. 1: Interrupt event enabled. Once set, this bit can only be cleared by a hardware reset."]
    #[inline(always)]
    #[must_use]
    pub fn inten(&mut self) -> INTEN_W<0> {
        INTEN_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
WDT Reset Enable. Defines the function of the WDT reset source (see PRCM:WARMRESET.WDT_STAT if enabled) 0: Disabled. 1: Enable the Watchdog reset output."]
    #[inline(always)]
    #[must_use]
    pub fn resen(&mut self) -> RESEN_W<1> {
        RESEN_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
WDT Interrupt Type 0: WDT interrupt is a standard interrupt. 1: WDT interrupt is a non-maskable interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn inttype(&mut self) -> INTTYPE_W<2> {
        INTTYPE_W::new(self)
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
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
