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
    Auxio0 = 128,
    #[doc = "64: Internal. Only to be used through TI provided API."]
    Auxio1 = 64,
    #[doc = "32: Internal. Only to be used through TI provided API."]
    Auxio2 = 32,
    #[doc = "16: Internal. Only to be used through TI provided API."]
    Auxio3 = 16,
    #[doc = "8: Internal. Only to be used through TI provided API."]
    Auxio4 = 8,
    #[doc = "4: Internal. Only to be used through TI provided API."]
    Auxio5 = 4,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    Auxio6 = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Auxio7 = 1,
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
            128 => Some(AdccompbIn::Auxio0),
            64 => Some(AdccompbIn::Auxio1),
            32 => Some(AdccompbIn::Auxio2),
            16 => Some(AdccompbIn::Auxio3),
            8 => Some(AdccompbIn::Auxio4),
            4 => Some(AdccompbIn::Auxio5),
            2 => Some(AdccompbIn::Auxio6),
            1 => Some(AdccompbIn::Auxio7),
            0 => Some(AdccompbIn::Nc),
            _ => None,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio0(&self) -> bool {
        *self == AdccompbIn::Auxio0
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio1(&self) -> bool {
        *self == AdccompbIn::Auxio1
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio2(&self) -> bool {
        *self == AdccompbIn::Auxio2
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio3(&self) -> bool {
        *self == AdccompbIn::Auxio3
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio4(&self) -> bool {
        *self == AdccompbIn::Auxio4
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio5(&self) -> bool {
        *self == AdccompbIn::Auxio5
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio6(&self) -> bool {
        *self == AdccompbIn::Auxio6
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio7(&self) -> bool {
        *self == AdccompbIn::Auxio7
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
    pub fn auxio0(self) -> &'a mut crate::W<REG> {
        self.variant(AdccompbIn::Auxio0)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio1(self) -> &'a mut crate::W<REG> {
        self.variant(AdccompbIn::Auxio1)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio2(self) -> &'a mut crate::W<REG> {
        self.variant(AdccompbIn::Auxio2)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio3(self) -> &'a mut crate::W<REG> {
        self.variant(AdccompbIn::Auxio3)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio4(self) -> &'a mut crate::W<REG> {
        self.variant(AdccompbIn::Auxio4)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio5(self) -> &'a mut crate::W<REG> {
        self.variant(AdccompbIn::Auxio5)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio6(self) -> &'a mut crate::W<REG> {
        self.variant(AdccompbIn::Auxio6)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio7(self) -> &'a mut crate::W<REG> {
        self.variant(AdccompbIn::Auxio7)
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
