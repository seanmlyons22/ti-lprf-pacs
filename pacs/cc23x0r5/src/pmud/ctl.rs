#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `MEAS_EN` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type MeasEnR = crate::BitReader;
#[doc = "Field `MEAS_EN` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type MeasEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALC_EN` reader - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type CalcEnR = crate::BitReader;
#[doc = "Field `CALC_EN` writer - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type CalcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYST_EN` reader - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type HystEnR = crate::BitReader;
#[doc = "Field `HYST_EN` writer - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type HystEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Internal. Only to be used through TI provided API."]
pub type Reserved3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn meas_en(&self) -> MeasEnR {
        MeasEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn calc_en(&self) -> CalcEnR {
        CalcEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hyst_en(&self) -> HystEnR {
        HystEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn meas_en(&mut self) -> MeasEnW<CtlSpec> {
        MeasEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn calc_en(&mut self) -> CalcEnW<CtlSpec> {
        CalcEnW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hyst_en(&mut self) -> HystEnW<CtlSpec> {
        HystEnW::new(self, 2)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0x04"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0x04;
}
