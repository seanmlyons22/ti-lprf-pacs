#[doc = "Register `PEROPRATE` reader"]
pub struct R(crate::R<PEROPRATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEROPRATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEROPRATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEROPRATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PEROPRATE` writer"]
pub struct W(crate::W<PEROPRATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PEROPRATE_SPEC>;
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
impl From<crate::W<PEROPRATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PEROPRATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAC_OP_RATE` reader - 0:0\\]
Select operational rate for AUX_MAC."]
pub type MAC_OP_RATE_R = crate::BitReader<MAC_OP_RATE_A>;
#[doc = "0:0\\]
Select operational rate for AUX_MAC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MAC_OP_RATE_A {
    #[doc = "1: AUX bus rate"]
    BUS_RATE = 1,
    #[doc = "0: SCE rate"]
    SCE_RATE = 0,
}
impl From<MAC_OP_RATE_A> for bool {
    #[inline(always)]
    fn from(variant: MAC_OP_RATE_A) -> Self {
        variant as u8 != 0
    }
}
impl MAC_OP_RATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAC_OP_RATE_A {
        match self.bits {
            true => MAC_OP_RATE_A::BUS_RATE,
            false => MAC_OP_RATE_A::SCE_RATE,
        }
    }
    #[doc = "Checks if the value of the field is `BUS_RATE`"]
    #[inline(always)]
    pub fn is_bus_rate(&self) -> bool {
        *self == MAC_OP_RATE_A::BUS_RATE
    }
    #[doc = "Checks if the value of the field is `SCE_RATE`"]
    #[inline(always)]
    pub fn is_sce_rate(&self) -> bool {
        *self == MAC_OP_RATE_A::SCE_RATE
    }
}
#[doc = "Field `MAC_OP_RATE` writer - 0:0\\]
Select operational rate for AUX_MAC."]
pub type MAC_OP_RATE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PEROPRATE_SPEC, MAC_OP_RATE_A, O>;
impl<'a, const O: u8> MAC_OP_RATE_W<'a, O> {
    #[doc = "AUX bus rate"]
    #[inline(always)]
    pub fn bus_rate(self) -> &'a mut W {
        self.variant(MAC_OP_RATE_A::BUS_RATE)
    }
    #[doc = "SCE rate"]
    #[inline(always)]
    pub fn sce_rate(self) -> &'a mut W {
        self.variant(MAC_OP_RATE_A::SCE_RATE)
    }
}
#[doc = "Field `SPIM_OP_RATE` reader - 1:1\\]
Select operational rate for AUX_SPIM."]
pub type SPIM_OP_RATE_R = crate::BitReader<SPIM_OP_RATE_A>;
#[doc = "1:1\\]
Select operational rate for AUX_SPIM.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPIM_OP_RATE_A {
    #[doc = "1: AUX bus rate"]
    BUS_RATE = 1,
    #[doc = "0: SCE rate"]
    SCE_RATE = 0,
}
impl From<SPIM_OP_RATE_A> for bool {
    #[inline(always)]
    fn from(variant: SPIM_OP_RATE_A) -> Self {
        variant as u8 != 0
    }
}
impl SPIM_OP_RATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIM_OP_RATE_A {
        match self.bits {
            true => SPIM_OP_RATE_A::BUS_RATE,
            false => SPIM_OP_RATE_A::SCE_RATE,
        }
    }
    #[doc = "Checks if the value of the field is `BUS_RATE`"]
    #[inline(always)]
    pub fn is_bus_rate(&self) -> bool {
        *self == SPIM_OP_RATE_A::BUS_RATE
    }
    #[doc = "Checks if the value of the field is `SCE_RATE`"]
    #[inline(always)]
    pub fn is_sce_rate(&self) -> bool {
        *self == SPIM_OP_RATE_A::SCE_RATE
    }
}
#[doc = "Field `SPIM_OP_RATE` writer - 1:1\\]
Select operational rate for AUX_SPIM."]
pub type SPIM_OP_RATE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PEROPRATE_SPEC, SPIM_OP_RATE_A, O>;
impl<'a, const O: u8> SPIM_OP_RATE_W<'a, O> {
    #[doc = "AUX bus rate"]
    #[inline(always)]
    pub fn bus_rate(self) -> &'a mut W {
        self.variant(SPIM_OP_RATE_A::BUS_RATE)
    }
    #[doc = "SCE rate"]
    #[inline(always)]
    pub fn sce_rate(self) -> &'a mut W {
        self.variant(SPIM_OP_RATE_A::SCE_RATE)
    }
}
#[doc = "Field `TIMER01_OP_RATE` reader - 2:2\\]
Select operational rate for AUX_TIMER01."]
pub type TIMER01_OP_RATE_R = crate::BitReader<TIMER01_OP_RATE_A>;
#[doc = "2:2\\]
Select operational rate for AUX_TIMER01.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMER01_OP_RATE_A {
    #[doc = "1: AUX bus rate"]
    BUS_RATE = 1,
    #[doc = "0: SCE rate"]
    SCE_RATE = 0,
}
impl From<TIMER01_OP_RATE_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER01_OP_RATE_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMER01_OP_RATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER01_OP_RATE_A {
        match self.bits {
            true => TIMER01_OP_RATE_A::BUS_RATE,
            false => TIMER01_OP_RATE_A::SCE_RATE,
        }
    }
    #[doc = "Checks if the value of the field is `BUS_RATE`"]
    #[inline(always)]
    pub fn is_bus_rate(&self) -> bool {
        *self == TIMER01_OP_RATE_A::BUS_RATE
    }
    #[doc = "Checks if the value of the field is `SCE_RATE`"]
    #[inline(always)]
    pub fn is_sce_rate(&self) -> bool {
        *self == TIMER01_OP_RATE_A::SCE_RATE
    }
}
#[doc = "Field `TIMER01_OP_RATE` writer - 2:2\\]
Select operational rate for AUX_TIMER01."]
pub type TIMER01_OP_RATE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PEROPRATE_SPEC, TIMER01_OP_RATE_A, O>;
impl<'a, const O: u8> TIMER01_OP_RATE_W<'a, O> {
    #[doc = "AUX bus rate"]
    #[inline(always)]
    pub fn bus_rate(self) -> &'a mut W {
        self.variant(TIMER01_OP_RATE_A::BUS_RATE)
    }
    #[doc = "SCE rate"]
    #[inline(always)]
    pub fn sce_rate(self) -> &'a mut W {
        self.variant(TIMER01_OP_RATE_A::SCE_RATE)
    }
}
#[doc = "Field `ANAIF_DAC_OP_RATE` reader - 3:3\\]
Select operational rate for AUX_ANAIF DAC sample clock state machine."]
pub type ANAIF_DAC_OP_RATE_R = crate::BitReader<ANAIF_DAC_OP_RATE_A>;
#[doc = "3:3\\]
Select operational rate for AUX_ANAIF DAC sample clock state machine.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANAIF_DAC_OP_RATE_A {
    #[doc = "1: AUX bus rate"]
    BUS_RATE = 1,
    #[doc = "0: SCE rate"]
    SCE_RATE = 0,
}
impl From<ANAIF_DAC_OP_RATE_A> for bool {
    #[inline(always)]
    fn from(variant: ANAIF_DAC_OP_RATE_A) -> Self {
        variant as u8 != 0
    }
}
impl ANAIF_DAC_OP_RATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANAIF_DAC_OP_RATE_A {
        match self.bits {
            true => ANAIF_DAC_OP_RATE_A::BUS_RATE,
            false => ANAIF_DAC_OP_RATE_A::SCE_RATE,
        }
    }
    #[doc = "Checks if the value of the field is `BUS_RATE`"]
    #[inline(always)]
    pub fn is_bus_rate(&self) -> bool {
        *self == ANAIF_DAC_OP_RATE_A::BUS_RATE
    }
    #[doc = "Checks if the value of the field is `SCE_RATE`"]
    #[inline(always)]
    pub fn is_sce_rate(&self) -> bool {
        *self == ANAIF_DAC_OP_RATE_A::SCE_RATE
    }
}
#[doc = "Field `ANAIF_DAC_OP_RATE` writer - 3:3\\]
Select operational rate for AUX_ANAIF DAC sample clock state machine."]
pub type ANAIF_DAC_OP_RATE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PEROPRATE_SPEC, ANAIF_DAC_OP_RATE_A, O>;
impl<'a, const O: u8> ANAIF_DAC_OP_RATE_W<'a, O> {
    #[doc = "AUX bus rate"]
    #[inline(always)]
    pub fn bus_rate(self) -> &'a mut W {
        self.variant(ANAIF_DAC_OP_RATE_A::BUS_RATE)
    }
    #[doc = "SCE rate"]
    #[inline(always)]
    pub fn sce_rate(self) -> &'a mut W {
        self.variant(ANAIF_DAC_OP_RATE_A::SCE_RATE)
    }
}
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PEROPRATE_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Select operational rate for AUX_MAC."]
    #[inline(always)]
    pub fn mac_op_rate(&self) -> MAC_OP_RATE_R {
        MAC_OP_RATE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Select operational rate for AUX_SPIM."]
    #[inline(always)]
    pub fn spim_op_rate(&self) -> SPIM_OP_RATE_R {
        SPIM_OP_RATE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Select operational rate for AUX_TIMER01."]
    #[inline(always)]
    pub fn timer01_op_rate(&self) -> TIMER01_OP_RATE_R {
        TIMER01_OP_RATE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Select operational rate for AUX_ANAIF DAC sample clock state machine."]
    #[inline(always)]
    pub fn anaif_dac_op_rate(&self) -> ANAIF_DAC_OP_RATE_R {
        ANAIF_DAC_OP_RATE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Select operational rate for AUX_MAC."]
    #[inline(always)]
    #[must_use]
    pub fn mac_op_rate(&mut self) -> MAC_OP_RATE_W<0> {
        MAC_OP_RATE_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Select operational rate for AUX_SPIM."]
    #[inline(always)]
    #[must_use]
    pub fn spim_op_rate(&mut self) -> SPIM_OP_RATE_W<1> {
        SPIM_OP_RATE_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Select operational rate for AUX_TIMER01."]
    #[inline(always)]
    #[must_use]
    pub fn timer01_op_rate(&mut self) -> TIMER01_OP_RATE_W<2> {
        TIMER01_OP_RATE_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Select operational rate for AUX_ANAIF DAC sample clock state machine."]
    #[inline(always)]
    #[must_use]
    pub fn anaif_dac_op_rate(&mut self) -> ANAIF_DAC_OP_RATE_W<3> {
        ANAIF_DAC_OP_RATE_W::new(self)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> RESERVED4_W<4> {
        RESERVED4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Operational Rate Some AUX peripherals are operated at either SCE or at AUX bus rate. You must select SCE rate when AUX_SCE uses such peripheral or an event produced by it. You must select AUX bus rate when system CPU uses such peripheral. SCE rate equals rate configured in AON_PMCTL:AUXSCECLK. AUX bus rate equals SCE rate, or SCLK_HF divided by 2 when MCU domain is active.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peroprate](index.html) module"]
pub struct PEROPRATE_SPEC;
impl crate::RegisterSpec for PEROPRATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [peroprate::R](R) reader structure"]
impl crate::Readable for PEROPRATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [peroprate::W](W) writer structure"]
impl crate::Writable for PEROPRATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PEROPRATE to value 0"]
impl crate::Resettable for PEROPRATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
