#[doc = "Register `MUX1` reader"]
pub type R = crate::R<Mux1Spec>;
#[doc = "Register `MUX1` writer"]
pub type W = crate::W<Mux1Spec>;
#[doc = "7:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CompaIn {
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
impl From<CompaIn> for u8 {
    #[inline(always)]
    fn from(variant: CompaIn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CompaIn {
    type Ux = u8;
}
impl crate::IsEnum for CompaIn {}
#[doc = "Field `COMPA_IN` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type CompaInR = crate::FieldReader<CompaIn>;
impl CompaInR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CompaIn> {
        match self.bits {
            128 => Some(CompaIn::Auxio19),
            64 => Some(CompaIn::Auxio20),
            32 => Some(CompaIn::Auxio21),
            16 => Some(CompaIn::Auxio22),
            8 => Some(CompaIn::Auxio23),
            4 => Some(CompaIn::Auxio24),
            2 => Some(CompaIn::Auxio25),
            1 => Some(CompaIn::Auxio26),
            0 => Some(CompaIn::Nc),
            _ => None,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio19(&self) -> bool {
        *self == CompaIn::Auxio19
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio20(&self) -> bool {
        *self == CompaIn::Auxio20
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio21(&self) -> bool {
        *self == CompaIn::Auxio21
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio22(&self) -> bool {
        *self == CompaIn::Auxio22
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio23(&self) -> bool {
        *self == CompaIn::Auxio23
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio24(&self) -> bool {
        *self == CompaIn::Auxio24
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio25(&self) -> bool {
        *self == CompaIn::Auxio25
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio26(&self) -> bool {
        *self == CompaIn::Auxio26
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == CompaIn::Nc
    }
}
#[doc = "Field `COMPA_IN` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type CompaInW<'a, REG> = crate::FieldWriter<'a, REG, 8, CompaIn>;
impl<'a, REG> CompaInW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio19(self) -> &'a mut crate::W<REG> {
        self.variant(CompaIn::Auxio19)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio20(self) -> &'a mut crate::W<REG> {
        self.variant(CompaIn::Auxio20)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio21(self) -> &'a mut crate::W<REG> {
        self.variant(CompaIn::Auxio21)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio22(self) -> &'a mut crate::W<REG> {
        self.variant(CompaIn::Auxio22)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio23(self) -> &'a mut crate::W<REG> {
        self.variant(CompaIn::Auxio23)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio24(self) -> &'a mut crate::W<REG> {
        self.variant(CompaIn::Auxio24)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio25(self) -> &'a mut crate::W<REG> {
        self.variant(CompaIn::Auxio25)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio26(self) -> &'a mut crate::W<REG> {
        self.variant(CompaIn::Auxio26)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn nc(self) -> &'a mut crate::W<REG> {
        self.variant(CompaIn::Nc)
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn compa_in(&self) -> CompaInR {
        CompaInR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn compa_in(&mut self) -> CompaInW<Mux1Spec> {
        CompaInW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mux1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mux1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mux1Spec;
impl crate::RegisterSpec for Mux1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mux1::R`](R) reader structure"]
impl crate::Readable for Mux1Spec {}
#[doc = "`write(|w| ..)` method takes [`mux1::W`](W) writer structure"]
impl crate::Writable for Mux1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets MUX1 to value 0"]
impl crate::Resettable for Mux1Spec {
    const RESET_VALUE: u8 = 0;
}
