#[doc = "Register `FLASH_PP` reader"]
pub type R = crate::R<FlashPpSpec>;
#[doc = "Register `FLASH_PP` writer"]
pub type W = crate::W<FlashPpSpec>;
#[doc = "Field `MAX_PP` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type MaxPpR = crate::FieldReader<u16>;
#[doc = "Field `MAX_PP` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type MaxPpW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TRIM3P4` reader - 23:16\\]
Internal. Only to be used through TI provided API."]
pub type Trim3p4R = crate::FieldReader;
#[doc = "Field `TRIM3P4` writer - 23:16\\]
Internal. Only to be used through TI provided API."]
pub type Trim3p4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PUMP_SU` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type PumpSuR = crate::FieldReader;
#[doc = "Field `PUMP_SU` writer - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type PumpSuW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn max_pp(&self) -> MaxPpR {
        MaxPpR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim3p4(&self) -> Trim3p4R {
        Trim3p4R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pump_su(&self) -> PumpSuR {
        PumpSuR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn max_pp(&mut self) -> MaxPpW<FlashPpSpec> {
        MaxPpW::new(self, 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn trim3p4(&mut self) -> Trim3p4W<FlashPpSpec> {
        Trim3p4W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn pump_su(&mut self) -> PumpSuW<FlashPpSpec> {
        PumpSuW::new(self, 24)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_pp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_pp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashPpSpec;
impl crate::RegisterSpec for FlashPpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_pp::R`](R) reader structure"]
impl crate::Readable for FlashPpSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_pp::W`](W) writer structure"]
impl crate::Writable for FlashPpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_PP to value 0x14"]
impl crate::Resettable for FlashPpSpec {
    const RESET_VALUE: u32 = 0x14;
}
