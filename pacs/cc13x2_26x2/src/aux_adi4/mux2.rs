#[doc = "Register `MUX2` reader"]
pub type R = crate::R<Mux2Spec>;
#[doc = "Register `MUX2` writer"]
pub type W = crate::W<Mux2Spec>;
#[doc = "2:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DacVrefSel {
    #[doc = "4: Internal. Only to be used through TI provided API."]
    Vdds = 4,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    Adcref = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Dcoupl = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Nc = 0,
}
impl From<DacVrefSel> for u8 {
    #[inline(always)]
    fn from(variant: DacVrefSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DacVrefSel {
    type Ux = u8;
}
impl crate::IsEnum for DacVrefSel {}
#[doc = "Field `DAC_VREF_SEL` reader - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type DacVrefSelR = crate::FieldReader<DacVrefSel>;
impl DacVrefSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DacVrefSel> {
        match self.bits {
            4 => Some(DacVrefSel::Vdds),
            2 => Some(DacVrefSel::Adcref),
            1 => Some(DacVrefSel::Dcoupl),
            0 => Some(DacVrefSel::Nc),
            _ => None,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_vdds(&self) -> bool {
        *self == DacVrefSel::Vdds
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_adcref(&self) -> bool {
        *self == DacVrefSel::Adcref
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_dcoupl(&self) -> bool {
        *self == DacVrefSel::Dcoupl
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == DacVrefSel::Nc
    }
}
#[doc = "Field `DAC_VREF_SEL` writer - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type DacVrefSelW<'a, REG> = crate::FieldWriter<'a, REG, 3, DacVrefSel>;
impl<'a, REG> DacVrefSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vdds(self) -> &'a mut crate::W<REG> {
        self.variant(DacVrefSel::Vdds)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adcref(self) -> &'a mut crate::W<REG> {
        self.variant(DacVrefSel::Adcref)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dcoupl(self) -> &'a mut crate::W<REG> {
        self.variant(DacVrefSel::Dcoupl)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn nc(self) -> &'a mut crate::W<REG> {
        self.variant(DacVrefSel::Nc)
    }
}
#[doc = "7:3\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AdccompbIn {
    #[doc = "16: Internal. Only to be used through TI provided API."]
    Vdds = 16,
    #[doc = "8: Internal. Only to be used through TI provided API."]
    Vss = 8,
    #[doc = "4: Internal. Only to be used through TI provided API."]
    Dcoupl = 4,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    Atest1 = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Atest0 = 1,
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
#[doc = "Field `ADCCOMPB_IN` reader - 7:3\\]
Internal. Only to be used through TI provided API."]
pub type AdccompbInR = crate::FieldReader<AdccompbIn>;
impl AdccompbInR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AdccompbIn> {
        match self.bits {
            16 => Some(AdccompbIn::Vdds),
            8 => Some(AdccompbIn::Vss),
            4 => Some(AdccompbIn::Dcoupl),
            2 => Some(AdccompbIn::Atest1),
            1 => Some(AdccompbIn::Atest0),
            0 => Some(AdccompbIn::Nc),
            _ => None,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_vdds(&self) -> bool {
        *self == AdccompbIn::Vdds
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_vss(&self) -> bool {
        *self == AdccompbIn::Vss
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_dcoupl(&self) -> bool {
        *self == AdccompbIn::Dcoupl
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_atest1(&self) -> bool {
        *self == AdccompbIn::Atest1
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_atest0(&self) -> bool {
        *self == AdccompbIn::Atest0
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == AdccompbIn::Nc
    }
}
#[doc = "Field `ADCCOMPB_IN` writer - 7:3\\]
Internal. Only to be used through TI provided API."]
pub type AdccompbInW<'a, REG> = crate::FieldWriter<'a, REG, 5, AdccompbIn>;
impl<'a, REG> AdccompbInW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vdds(self) -> &'a mut crate::W<REG> {
        self.variant(AdccompbIn::Vdds)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vss(self) -> &'a mut crate::W<REG> {
        self.variant(AdccompbIn::Vss)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dcoupl(self) -> &'a mut crate::W<REG> {
        self.variant(AdccompbIn::Dcoupl)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn atest1(self) -> &'a mut crate::W<REG> {
        self.variant(AdccompbIn::Atest1)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn atest0(self) -> &'a mut crate::W<REG> {
        self.variant(AdccompbIn::Atest0)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn nc(self) -> &'a mut crate::W<REG> {
        self.variant(AdccompbIn::Nc)
    }
}
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dac_vref_sel(&self) -> DacVrefSelR {
        DacVrefSelR::new(self.bits & 7)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adccompb_in(&self) -> AdccompbInR {
        AdccompbInR::new((self.bits >> 3) & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dac_vref_sel(&mut self) -> DacVrefSelW<Mux2Spec> {
        DacVrefSelW::new(self, 0)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn adccompb_in(&mut self) -> AdccompbInW<Mux2Spec> {
        AdccompbInW::new(self, 3)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mux2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mux2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mux2Spec;
impl crate::RegisterSpec for Mux2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mux2::R`](R) reader structure"]
impl crate::Readable for Mux2Spec {}
#[doc = "`write(|w| ..)` method takes [`mux2::W`](W) writer structure"]
impl crate::Writable for Mux2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets MUX2 to value 0"]
impl crate::Resettable for Mux2Spec {
    const RESET_VALUE: u8 = 0;
}
