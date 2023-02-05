#[doc = "Register `GPTCLKGR` reader"]
pub struct R(crate::R<GPTCLKGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPTCLKGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPTCLKGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPTCLKGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPTCLKGR` writer"]
pub struct W(crate::W<GPTCLKGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPTCLKGR_SPEC>;
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
impl From<crate::W<GPTCLKGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPTCLKGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_EN` reader - 3:0\\]
Each bit below has the following meaning: 0: Disable clock 1: Enable clock Can be forced on by AM_CLK_EN ENUMs can be combined For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type CLK_EN_R = crate::FieldReader<u8, CLK_EN_A>;
#[doc = "3:0\\]
Each bit below has the following meaning: 0: Disable clock 1: Enable clock Can be forced on by AM_CLK_EN ENUMs can be combined For changes to take effect, CLKLOADCTL.LOAD needs to be written\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLK_EN_A {
    #[doc = "8: Enable clock for GPT3"]
    GPT3 = 8,
    #[doc = "4: Enable clock for GPT2"]
    GPT2 = 4,
    #[doc = "2: Enable clock for GPT1"]
    GPT1 = 2,
    #[doc = "1: Enable clock for GPT0"]
    GPT0 = 1,
}
impl From<CLK_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_EN_A) -> Self {
        variant as _
    }
}
impl CLK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLK_EN_A> {
        match self.bits {
            8 => Some(CLK_EN_A::GPT3),
            4 => Some(CLK_EN_A::GPT2),
            2 => Some(CLK_EN_A::GPT1),
            1 => Some(CLK_EN_A::GPT0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GPT3`"]
    #[inline(always)]
    pub fn is_gpt3(&self) -> bool {
        *self == CLK_EN_A::GPT3
    }
    #[doc = "Checks if the value of the field is `GPT2`"]
    #[inline(always)]
    pub fn is_gpt2(&self) -> bool {
        *self == CLK_EN_A::GPT2
    }
    #[doc = "Checks if the value of the field is `GPT1`"]
    #[inline(always)]
    pub fn is_gpt1(&self) -> bool {
        *self == CLK_EN_A::GPT1
    }
    #[doc = "Checks if the value of the field is `GPT0`"]
    #[inline(always)]
    pub fn is_gpt0(&self) -> bool {
        *self == CLK_EN_A::GPT0
    }
}
#[doc = "Field `CLK_EN` writer - 3:0\\]
Each bit below has the following meaning: 0: Disable clock 1: Enable clock Can be forced on by AM_CLK_EN ENUMs can be combined For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type CLK_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPTCLKGR_SPEC, u8, CLK_EN_A, 4, O>;
impl<'a, const O: u8> CLK_EN_W<'a, O> {
    #[doc = "Enable clock for GPT3"]
    #[inline(always)]
    pub fn gpt3(self) -> &'a mut W {
        self.variant(CLK_EN_A::GPT3)
    }
    #[doc = "Enable clock for GPT2"]
    #[inline(always)]
    pub fn gpt2(self) -> &'a mut W {
        self.variant(CLK_EN_A::GPT2)
    }
    #[doc = "Enable clock for GPT1"]
    #[inline(always)]
    pub fn gpt1(self) -> &'a mut W {
        self.variant(CLK_EN_A::GPT1)
    }
    #[doc = "Enable clock for GPT0"]
    #[inline(always)]
    pub fn gpt0(self) -> &'a mut W {
        self.variant(CLK_EN_A::GPT0)
    }
}
#[doc = "Field `RESERVED4` reader - 7:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED4` writer - 7:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPTCLKGR_SPEC, u8, u8, 4, O>;
#[doc = "Field `AM_CLK_EN` reader - 11:8\\]
Each bit below has the following meaning: 0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CLK_EN, GPTCLKGS.CLK_EN and GPTCLKGDS.CLK_EN when enabled. ENUMs can be combined For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type AM_CLK_EN_R = crate::FieldReader<u8, AM_CLK_EN_A>;
#[doc = "11:8\\]
Each bit below has the following meaning: 0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CLK_EN, GPTCLKGS.CLK_EN and GPTCLKGDS.CLK_EN when enabled. ENUMs can be combined For changes to take effect, CLKLOADCTL.LOAD needs to be written\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AM_CLK_EN_A {
    #[doc = "8: Enable clock for GPT3 in all modes"]
    AM_GPT3 = 8,
    #[doc = "4: Enable clock for GPT2 in all modes"]
    AM_GPT2 = 4,
    #[doc = "2: Enable clock for GPT1 in all modes"]
    AM_GPT1 = 2,
    #[doc = "1: Enable clock for GPT0 in all modes"]
    AM_GPT0 = 1,
}
impl From<AM_CLK_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: AM_CLK_EN_A) -> Self {
        variant as _
    }
}
impl AM_CLK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AM_CLK_EN_A> {
        match self.bits {
            8 => Some(AM_CLK_EN_A::AM_GPT3),
            4 => Some(AM_CLK_EN_A::AM_GPT2),
            2 => Some(AM_CLK_EN_A::AM_GPT1),
            1 => Some(AM_CLK_EN_A::AM_GPT0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AM_GPT3`"]
    #[inline(always)]
    pub fn is_am_gpt3(&self) -> bool {
        *self == AM_CLK_EN_A::AM_GPT3
    }
    #[doc = "Checks if the value of the field is `AM_GPT2`"]
    #[inline(always)]
    pub fn is_am_gpt2(&self) -> bool {
        *self == AM_CLK_EN_A::AM_GPT2
    }
    #[doc = "Checks if the value of the field is `AM_GPT1`"]
    #[inline(always)]
    pub fn is_am_gpt1(&self) -> bool {
        *self == AM_CLK_EN_A::AM_GPT1
    }
    #[doc = "Checks if the value of the field is `AM_GPT0`"]
    #[inline(always)]
    pub fn is_am_gpt0(&self) -> bool {
        *self == AM_CLK_EN_A::AM_GPT0
    }
}
#[doc = "Field `AM_CLK_EN` writer - 11:8\\]
Each bit below has the following meaning: 0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CLK_EN, GPTCLKGS.CLK_EN and GPTCLKGDS.CLK_EN when enabled. ENUMs can be combined For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type AM_CLK_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPTCLKGR_SPEC, u8, AM_CLK_EN_A, 4, O>;
impl<'a, const O: u8> AM_CLK_EN_W<'a, O> {
    #[doc = "Enable clock for GPT3 in all modes"]
    #[inline(always)]
    pub fn am_gpt3(self) -> &'a mut W {
        self.variant(AM_CLK_EN_A::AM_GPT3)
    }
    #[doc = "Enable clock for GPT2 in all modes"]
    #[inline(always)]
    pub fn am_gpt2(self) -> &'a mut W {
        self.variant(AM_CLK_EN_A::AM_GPT2)
    }
    #[doc = "Enable clock for GPT1 in all modes"]
    #[inline(always)]
    pub fn am_gpt1(self) -> &'a mut W {
        self.variant(AM_CLK_EN_A::AM_GPT1)
    }
    #[doc = "Enable clock for GPT0 in all modes"]
    #[inline(always)]
    pub fn am_gpt0(self) -> &'a mut W {
        self.variant(AM_CLK_EN_A::AM_GPT0)
    }
}
#[doc = "Field `RESERVED12` reader - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED12_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED12` writer - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED12_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPTCLKGR_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Each bit below has the following meaning: 0: Disable clock 1: Enable clock Can be forced on by AM_CLK_EN ENUMs can be combined For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Each bit below has the following meaning: 0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CLK_EN, GPTCLKGS.CLK_EN and GPTCLKGDS.CLK_EN when enabled. ENUMs can be combined For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn am_clk_en(&self) -> AM_CLK_EN_R {
        AM_CLK_EN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Each bit below has the following meaning: 0: Disable clock 1: Enable clock Can be forced on by AM_CLK_EN ENUMs can be combined For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<0> {
        CLK_EN_W::new(self)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> RESERVED4_W<4> {
        RESERVED4_W::new(self)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Each bit below has the following meaning: 0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CLK_EN, GPTCLKGS.CLK_EN and GPTCLKGDS.CLK_EN when enabled. ENUMs can be combined For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn am_clk_en(&mut self) -> AM_CLK_EN_W<8> {
        AM_CLK_EN_W::new(self)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> RESERVED12_W<12> {
        RESERVED12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPT Clock Gate For Run And All Modes\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptclkgr](index.html) module"]
pub struct GPTCLKGR_SPEC;
impl crate::RegisterSpec for GPTCLKGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gptclkgr::R](R) reader structure"]
impl crate::Readable for GPTCLKGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gptclkgr::W](W) writer structure"]
impl crate::Writable for GPTCLKGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPTCLKGR to value 0"]
impl crate::Resettable for GPTCLKGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
