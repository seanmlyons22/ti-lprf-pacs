#[doc = "Register `FPAC1` reader"]
pub type R = crate::R<Fpac1Spec>;
#[doc = "Register `FPAC1` writer"]
pub type W = crate::W<Fpac1Spec>;
#[doc = "Field `PUMPPWR` reader - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type PumppwrR = crate::FieldReader;
#[doc = "Field `PUMPPWR` writer - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type PumppwrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESERVED1` reader - 3:2\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `RESERVED1` writer - 3:2\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUMPRESET_PW` reader - 15:4\\]
Internal. Only to be used through TI provided API."]
pub type PumpresetPwR = crate::FieldReader<u16>;
#[doc = "Field `PUMPRESET_PW` writer - 15:4\\]
Internal. Only to be used through TI provided API."]
pub type PumpresetPwW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PSLEEPTDIS` reader - 27:16\\]
Internal. Only to be used through TI provided API."]
pub type PsleeptdisR = crate::FieldReader<u16>;
#[doc = "Field `PSLEEPTDIS` writer - 27:16\\]
Internal. Only to be used through TI provided API."]
pub type PsleeptdisW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `RESERVED28` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type Reserved28R = crate::FieldReader;
#[doc = "Field `RESERVED28` writer - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type Reserved28W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pumppwr(&self) -> PumppwrR {
        PumppwrR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pumpreset_pw(&self) -> PumpresetPwR {
        PumpresetPwR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn psleeptdis(&self) -> PsleeptdisR {
        PsleeptdisR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved28(&self) -> Reserved28R {
        Reserved28R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn pumppwr(&mut self) -> PumppwrW<Fpac1Spec> {
        PumppwrW::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Fpac1Spec> {
        Reserved1W::new(self, 2)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn pumpreset_pw(&mut self) -> PumpresetPwW<Fpac1Spec> {
        PumpresetPwW::new(self, 4)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn psleeptdis(&mut self) -> PsleeptdisW<Fpac1Spec> {
        PsleeptdisW::new(self, 16)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved28(&mut self) -> Reserved28W<Fpac1Spec> {
        Reserved28W::new(self, 28)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpac1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpac1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fpac1Spec;
impl crate::RegisterSpec for Fpac1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpac1::R`](R) reader structure"]
impl crate::Readable for Fpac1Spec {}
#[doc = "`write(|w| ..)` method takes [`fpac1::W`](W) writer structure"]
impl crate::Writable for Fpac1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FPAC1 to value 0x0208_2081"]
impl crate::Resettable for Fpac1Spec {
    const RESET_VALUE: u32 = 0x0208_2081;
}
