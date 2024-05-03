#[doc = "Register `OSCCFG` reader"]
pub type R = crate::R<OsccfgSpec>;
#[doc = "Register `OSCCFG` writer"]
pub type W = crate::W<OsccfgSpec>;
#[doc = "Field `PER_E` reader - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type PerER = crate::FieldReader;
#[doc = "Field `PER_E` writer - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type PerEW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PER_M` reader - 7:3\\]
Internal. Only to be used through TI provided API."]
pub type PerMR = crate::FieldReader;
#[doc = "Field `PER_M` writer - 7:3\\]
Internal. Only to be used through TI provided API."]
pub type PerMW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn per_e(&self) -> PerER {
        PerER::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn per_m(&self) -> PerMR {
        PerMR::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn per_e(&mut self) -> PerEW<OsccfgSpec> {
        PerEW::new(self, 0)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn per_m(&mut self) -> PerMW<OsccfgSpec> {
        PerMW::new(self, 3)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<OsccfgSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Oscillator Configuration This register sets the period for Amplitude compensation requests sent to the oscillator control system. The amplitude compensations is only applicable when XOSC_HF is running in low power mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osccfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osccfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OsccfgSpec;
impl crate::RegisterSpec for OsccfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osccfg::R`](R) reader structure"]
impl crate::Readable for OsccfgSpec {}
#[doc = "`write(|w| ..)` method takes [`osccfg::W`](W) writer structure"]
impl crate::Writable for OsccfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSCCFG to value 0"]
impl crate::Resettable for OsccfgSpec {
    const RESET_VALUE: u32 = 0;
}
