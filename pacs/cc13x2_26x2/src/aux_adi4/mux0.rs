#[doc = "Register `MUX0` reader"]
pub type R = crate::R<Mux0Spec>;
#[doc = "Register `MUX0` writer"]
pub type W = crate::W<Mux0Spec>;
#[doc = "3:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CompaRef {
    #[doc = "8: Internal. Only to be used through TI provided API."]
    Adcvrefp = 8,
    #[doc = "4: Internal. Only to be used through TI provided API."]
    Vdds = 4,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    Vss = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Dcoupl = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Nc = 0,
}
impl From<CompaRef> for u8 {
    #[inline(always)]
    fn from(variant: CompaRef) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CompaRef {
    type Ux = u8;
}
impl crate::IsEnum for CompaRef {}
#[doc = "Field `COMPA_REF` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type CompaRefR = crate::FieldReader<CompaRef>;
impl CompaRefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CompaRef> {
        match self.bits {
            8 => Some(CompaRef::Adcvrefp),
            4 => Some(CompaRef::Vdds),
            2 => Some(CompaRef::Vss),
            1 => Some(CompaRef::Dcoupl),
            0 => Some(CompaRef::Nc),
            _ => None,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_adcvrefp(&self) -> bool {
        *self == CompaRef::Adcvrefp
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_vdds(&self) -> bool {
        *self == CompaRef::Vdds
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_vss(&self) -> bool {
        *self == CompaRef::Vss
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_dcoupl(&self) -> bool {
        *self == CompaRef::Dcoupl
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == CompaRef::Nc
    }
}
#[doc = "Field `COMPA_REF` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type CompaRefW<'a, REG> = crate::FieldWriter<'a, REG, 4, CompaRef>;
impl<'a, REG> CompaRefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adcvrefp(self) -> &'a mut crate::W<REG> {
        self.variant(CompaRef::Adcvrefp)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vdds(self) -> &'a mut crate::W<REG> {
        self.variant(CompaRef::Vdds)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vss(self) -> &'a mut crate::W<REG> {
        self.variant(CompaRef::Vss)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dcoupl(self) -> &'a mut crate::W<REG> {
        self.variant(CompaRef::Dcoupl)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn nc(self) -> &'a mut crate::W<REG> {
        self.variant(CompaRef::Nc)
    }
}
#[doc = "Field `RESERVED4` reader - 5:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `RESERVED4` writer - 5:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "6:6\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdccompbIn {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Vddr1p8v = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Nc = 0,
}
impl From<AdccompbIn> for bool {
    #[inline(always)]
    fn from(variant: AdccompbIn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCCOMPB_IN` reader - 6:6\\]
Internal. Only to be used through TI provided API."]
pub type AdccompbInR = crate::BitReader<AdccompbIn>;
impl AdccompbInR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AdccompbIn {
        match self.bits {
            true => AdccompbIn::Vddr1p8v,
            false => AdccompbIn::Nc,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_vddr_1p8v(&self) -> bool {
        *self == AdccompbIn::Vddr1p8v
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == AdccompbIn::Nc
    }
}
#[doc = "Field `ADCCOMPB_IN` writer - 6:6\\]
Internal. Only to be used through TI provided API."]
pub type AdccompbInW<'a, REG> = crate::BitWriter<'a, REG, AdccompbIn>;
impl<'a, REG> AdccompbInW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_1p8v(self) -> &'a mut crate::W<REG> {
        self.variant(AdccompbIn::Vddr1p8v)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn nc(self) -> &'a mut crate::W<REG> {
        self.variant(AdccompbIn::Nc)
    }
}
#[doc = "Field `RESERVED7` reader - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `RESERVED7` writer - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn compa_ref(&self) -> CompaRefR {
        CompaRefR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adccompb_in(&self) -> AdccompbInR {
        AdccompbInR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn compa_ref(&mut self) -> CompaRefW<Mux0Spec> {
        CompaRefW::new(self, 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<Mux0Spec> {
        Reserved4W::new(self, 4)
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn adccompb_in(&mut self) -> AdccompbInW<Mux0Spec> {
        AdccompbInW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> Reserved7W<Mux0Spec> {
        Reserved7W::new(self, 7)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mux0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mux0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mux0Spec;
impl crate::RegisterSpec for Mux0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mux0::R`](R) reader structure"]
impl crate::Readable for Mux0Spec {}
#[doc = "`write(|w| ..)` method takes [`mux0::W`](W) writer structure"]
impl crate::Writable for Mux0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets MUX0 to value 0"]
impl crate::Resettable for Mux0Spec {
    const RESET_VALUE: u8 = 0;
}
