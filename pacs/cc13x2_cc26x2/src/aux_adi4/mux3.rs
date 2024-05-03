#[doc = "Register `MUX3` reader"]
pub type R = crate::R<Mux3Spec>;
#[doc = "Register `MUX3` writer"]
pub type W = crate::W<Mux3Spec>;
#[doc = "7:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AdccompbIn {
    #[doc = "128: Internal. Only to be used through TI provided API."]
    Auxio19 = 128,
    #[doc = "64: Internal. Only to be used through TI provided API."]
    Auxio20 = 64,
    #[doc = "32: Internal. Only to be used through TI provided API."]
    Auxio21 = 32,
    #[doc = "16: Internal. Only to be used through TI provided API."]
    Auxio22 = 16,
    #[doc = "8: Internal. Only to be used through TI provided API."]
    Auxio23 = 8,
    #[doc = "4: Internal. Only to be used through TI provided API."]
    Auxio24 = 4,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    Auxio25 = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Auxio26 = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Nc = 0,
}
impl From<AdccompbIn> for u8 {
    #[inline(always)]
    fn from(variant: AdccompbIn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AdccompbIn {
    type Ux = u8;
}
impl crate::IsEnum for AdccompbIn {}
#[doc = "Field `ADCCOMPB_IN` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type AdccompbInR = crate::FieldReader<AdccompbIn>;
impl AdccompbInR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AdccompbIn> {
        match self.bits {
            128 => Some(AdccompbIn::Auxio19),
            64 => Some(AdccompbIn::Auxio20),
            32 => Some(AdccompbIn::Auxio21),
            16 => Some(AdccompbIn::Auxio22),
            8 => Some(AdccompbIn::Auxio23),
            4 => Some(AdccompbIn::Auxio24),
            2 => Some(AdccompbIn::Auxio25),
            1 => Some(AdccompbIn::Auxio26),
            0 => Some(AdccompbIn::Nc),
            _ => None,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio19(&self) -> bool {
        *self == AdccompbIn::Auxio19
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio20(&self) -> bool {
        *self == AdccompbIn::Auxio20
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio21(&self) -> bool {
        *self == AdccompbIn::Auxio21
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio22(&self) -> bool {
        *self == AdccompbIn::Auxio22
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio23(&self) -> bool {
        *self == AdccompbIn::Auxio23
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio24(&self) -> bool {
        *self == AdccompbIn::Auxio24
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio25(&self) -> bool {
        *self == AdccompbIn::Auxio25
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio26(&self) -> bool {
        *self == AdccompbIn::Auxio26
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == AdccompbIn::Nc
    }
}
#[doc = "Field `ADCCOMPB_IN` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type AdccompbInW<'a, REG> = crate::FieldWriter<'a, REG, 8, AdccompbIn>;
impl<'a, REG> AdccompbInW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio19(self) -> &'a mut crate::W<REG> {
        self.variant(AdccompbIn::Auxio19)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio20(self) -> &'a mut crate::W<REG> {
        self.variant(AdccompbIn::Auxio20)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio21(self) -> &'a mut crate::W<REG> {
        self.variant(AdccompbIn::Auxio21)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio22(self) -> &'a mut crate::W<REG> {
        self.variant(AdccompbIn::Auxio22)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio23(self) -> &'a mut crate::W<REG> {
        self.variant(AdccompbIn::Auxio23)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio24(self) -> &'a mut crate::W<REG> {
        self.variant(AdccompbIn::Auxio24)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio25(self) -> &'a mut crate::W<REG> {
        self.variant(AdccompbIn::Auxio25)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio26(self) -> &'a mut crate::W<REG> {
        self.variant(AdccompbIn::Auxio26)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn nc(self) -> &'a mut crate::W<REG> {
        self.variant(AdccompbIn::Nc)
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adccompb_in(&self) -> AdccompbInR {
        AdccompbInR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn adccompb_in(&mut self) -> AdccompbInW<Mux3Spec> {
        AdccompbInW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mux3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mux3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mux3Spec;
impl crate::RegisterSpec for Mux3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mux3::R`](R) reader structure"]
impl crate::Readable for Mux3Spec {}
#[doc = "`write(|w| ..)` method takes [`mux3::W`](W) writer structure"]
impl crate::Writable for Mux3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets MUX3 to value 0"]
impl crate::Resettable for Mux3Spec {
    const RESET_VALUE: u8 = 0;
}
