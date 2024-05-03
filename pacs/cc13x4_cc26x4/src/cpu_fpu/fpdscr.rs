#[doc = "Register `FPDSCR` reader"]
pub type R = crate::R<FpdscrSpec>;
#[doc = "Register `FPDSCR` writer"]
pub type W = crate::W<FpdscrSpec>;
#[doc = "Field `RESERVED0` reader - 21:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED0` writer - 21:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
#[doc = "Field `RMode` reader - 23:22\\]
Default value for FPSCR.RMode"]
pub type RmodeR = crate::FieldReader;
#[doc = "Field `RMode` writer - 23:22\\]
Default value for FPSCR.RMode"]
pub type RmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FZ` reader - 24:24\\]
Default value for FPSCR.FZ"]
pub type FzR = crate::BitReader;
#[doc = "Field `FZ` writer - 24:24\\]
Default value for FPSCR.FZ"]
pub type FzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DN` reader - 25:25\\]
Default value for FPSCR.DN"]
pub type DnR = crate::BitReader;
#[doc = "Field `DN` writer - 25:25\\]
Default value for FPSCR.DN"]
pub type DnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHP` reader - 26:26\\]
Default value for FPSCR.AHP"]
pub type AhpR = crate::BitReader;
#[doc = "Field `AHP` writer - 26:26\\]
Default value for FPSCR.AHP"]
pub type AhpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED27` reader - 31:27\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved27R = crate::FieldReader;
#[doc = "Field `RESERVED27` writer - 31:27\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved27W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:21 - 21:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(self.bits & 0x003f_ffff)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Default value for FPSCR.RMode"]
    #[inline(always)]
    pub fn rmode(&self) -> RmodeR {
        RmodeR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Default value for FPSCR.FZ"]
    #[inline(always)]
    pub fn fz(&self) -> FzR {
        FzR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Default value for FPSCR.DN"]
    #[inline(always)]
    pub fn dn(&self) -> DnR {
        DnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Default value for FPSCR.AHP"]
    #[inline(always)]
    pub fn ahp(&self) -> AhpR {
        AhpR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:31 - 31:27\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved27(&self) -> Reserved27R {
        Reserved27R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:21 - 21:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<FpdscrSpec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Default value for FPSCR.RMode"]
    #[inline(always)]
    #[must_use]
    pub fn rmode(&mut self) -> RmodeW<FpdscrSpec> {
        RmodeW::new(self, 22)
    }
    #[doc = "Bit 24 - 24:24\\]
Default value for FPSCR.FZ"]
    #[inline(always)]
    #[must_use]
    pub fn fz(&mut self) -> FzW<FpdscrSpec> {
        FzW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Default value for FPSCR.DN"]
    #[inline(always)]
    #[must_use]
    pub fn dn(&mut self) -> DnW<FpdscrSpec> {
        DnW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Default value for FPSCR.AHP"]
    #[inline(always)]
    #[must_use]
    pub fn ahp(&mut self) -> AhpW<FpdscrSpec> {
        AhpW::new(self, 26)
    }
    #[doc = "Bits 27:31 - 31:27\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved27(&mut self) -> Reserved27W<FpdscrSpec> {
        Reserved27W::new(self, 27)
    }
}
#[doc = "Holds the default values for the floating-point status control data that the PE assigns to the FPSCR when it creates a new floating-point context\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpdscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpdscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FpdscrSpec;
impl crate::RegisterSpec for FpdscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpdscr::R`](R) reader structure"]
impl crate::Readable for FpdscrSpec {}
#[doc = "`write(|w| ..)` method takes [`fpdscr::W`](W) writer structure"]
impl crate::Writable for FpdscrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FPDSCR to value 0"]
impl crate::Resettable for FpdscrSpec {
    const RESET_VALUE: u32 = 0;
}
