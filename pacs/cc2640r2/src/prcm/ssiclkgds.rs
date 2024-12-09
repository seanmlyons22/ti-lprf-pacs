#[doc = "Register `SSICLKGDS` reader"]
pub type R = crate::R<SsiclkgdsSpec>;
#[doc = "Register `SSICLKGDS` writer"]
pub type W = crate::W<SsiclkgdsSpec>;
#[doc = "1:0\\]
0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkEn {
    #[doc = "2: Enable clock for SSI1"]
    Ssi1 = 2,
    #[doc = "1: Enable clock for SSI0"]
    Ssi0 = 1,
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
#[doc = "Field `CLK_EN` reader - 1:0\\]
0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type ClkEnR = crate::FieldReader<ClkEn>;
impl ClkEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkEn> {
        match self.bits {
            2 => Some(ClkEn::Ssi1),
            1 => Some(ClkEn::Ssi0),
            _ => None,
        }
    }
    #[doc = "Enable clock for SSI1"]
    #[inline(always)]
    pub fn is_ssi1(&self) -> bool {
        *self == ClkEn::Ssi1
    }
    #[doc = "Enable clock for SSI0"]
    #[inline(always)]
    pub fn is_ssi0(&self) -> bool {
        *self == ClkEn::Ssi0
    }
}
#[doc = "Field `CLK_EN` writer - 1:0\\]
0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type ClkEnW<'a, REG> = crate::FieldWriter<'a, REG, 2, ClkEn>;
impl<'a, REG> ClkEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Enable clock for SSI1"]
    #[inline(always)]
    pub fn ssi1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkEn::Ssi1)
    }
    #[doc = "Enable clock for SSI0"]
    #[inline(always)]
    pub fn ssi0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkEn::Ssi0)
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn clk_en(&self) -> ClkEnR {
        ClkEnR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> ClkEnW<SsiclkgdsSpec> {
        ClkEnW::new(self, 0)
    }
}
#[doc = "SSI Clock Gate For Deep Sleep Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssiclkgds::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssiclkgds::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsiclkgdsSpec;
impl crate::RegisterSpec for SsiclkgdsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssiclkgds::R`](R) reader structure"]
impl crate::Readable for SsiclkgdsSpec {}
#[doc = "`write(|w| ..)` method takes [`ssiclkgds::W`](W) writer structure"]
impl crate::Writable for SsiclkgdsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSICLKGDS to value 0"]
impl crate::Resettable for SsiclkgdsSpec {
    const RESET_VALUE: u32 = 0;
}
