#[doc = "Register `GPTCLKGR` reader"]
pub type R = crate::R<GptclkgrSpec>;
#[doc = "Register `GPTCLKGR` writer"]
pub type W = crate::W<GptclkgrSpec>;
#[doc = "3:0\\]
Each bit below has the following meaning: 0: Disable clock 1: Enable clock ENUMs can be combined For changes to take effect, CLKLOADCTL.LOAD needs to be written\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkEn {
    #[doc = "8: Enable clock for GPT3"]
    Gpt3 = 8,
    #[doc = "4: Enable clock for GPT2"]
    Gpt2 = 4,
    #[doc = "2: Enable clock for GPT1"]
    Gpt1 = 2,
    #[doc = "1: Enable clock for GPT0"]
    Gpt0 = 1,
}
impl From<ClkEn> for u8 {
    #[inline(always)]
    fn from(variant: ClkEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkEn {
    type Ux = u8;
}
impl crate::IsEnum for ClkEn {}
#[doc = "Field `CLK_EN` reader - 3:0\\]
Each bit below has the following meaning: 0: Disable clock 1: Enable clock ENUMs can be combined For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type ClkEnR = crate::FieldReader<ClkEn>;
impl ClkEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkEn> {
        match self.bits {
            8 => Some(ClkEn::Gpt3),
            4 => Some(ClkEn::Gpt2),
            2 => Some(ClkEn::Gpt1),
            1 => Some(ClkEn::Gpt0),
            _ => None,
        }
    }
    #[doc = "Enable clock for GPT3"]
    #[inline(always)]
    pub fn is_gpt3(&self) -> bool {
        *self == ClkEn::Gpt3
    }
    #[doc = "Enable clock for GPT2"]
    #[inline(always)]
    pub fn is_gpt2(&self) -> bool {
        *self == ClkEn::Gpt2
    }
    #[doc = "Enable clock for GPT1"]
    #[inline(always)]
    pub fn is_gpt1(&self) -> bool {
        *self == ClkEn::Gpt1
    }
    #[doc = "Enable clock for GPT0"]
    #[inline(always)]
    pub fn is_gpt0(&self) -> bool {
        *self == ClkEn::Gpt0
    }
}
#[doc = "Field `CLK_EN` writer - 3:0\\]
Each bit below has the following meaning: 0: Disable clock 1: Enable clock ENUMs can be combined For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type ClkEnW<'a, REG> = crate::FieldWriter<'a, REG, 4, ClkEn>;
impl<'a, REG> ClkEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Enable clock for GPT3"]
    #[inline(always)]
    pub fn gpt3(self) -> &'a mut crate::W<REG> {
        self.variant(ClkEn::Gpt3)
    }
    #[doc = "Enable clock for GPT2"]
    #[inline(always)]
    pub fn gpt2(self) -> &'a mut crate::W<REG> {
        self.variant(ClkEn::Gpt2)
    }
    #[doc = "Enable clock for GPT1"]
    #[inline(always)]
    pub fn gpt1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkEn::Gpt1)
    }
    #[doc = "Enable clock for GPT0"]
    #[inline(always)]
    pub fn gpt0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkEn::Gpt0)
    }
}
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Each bit below has the following meaning: 0: Disable clock 1: Enable clock ENUMs can be combined For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn clk_en(&self) -> ClkEnR {
        ClkEnR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Each bit below has the following meaning: 0: Disable clock 1: Enable clock ENUMs can be combined For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> ClkEnW<GptclkgrSpec> {
        ClkEnW::new(self, 0)
    }
}
#[doc = "GPT Clock Gate For Run Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gptclkgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gptclkgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GptclkgrSpec;
impl crate::RegisterSpec for GptclkgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gptclkgr::R`](R) reader structure"]
impl crate::Readable for GptclkgrSpec {}
#[doc = "`write(|w| ..)` method takes [`gptclkgr::W`](W) writer structure"]
impl crate::Writable for GptclkgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPTCLKGR to value 0"]
impl crate::Resettable for GptclkgrSpec {
    const RESET_VALUE: u32 = 0;
}
