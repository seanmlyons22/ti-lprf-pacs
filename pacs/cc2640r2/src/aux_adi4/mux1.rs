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
            128 => Some(CompaIn::Auxio0),
            64 => Some(CompaIn::Auxio1),
            32 => Some(CompaIn::Auxio2),
            16 => Some(CompaIn::Auxio3),
            8 => Some(CompaIn::Auxio4),
            4 => Some(CompaIn::Auxio5),
            2 => Some(CompaIn::Auxio6),
            1 => Some(CompaIn::Auxio7),
            0 => Some(CompaIn::Nc),
            _ => None,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio0(&self) -> bool {
        *self == CompaIn::Auxio0
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio1(&self) -> bool {
        *self == CompaIn::Auxio1
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio2(&self) -> bool {
        *self == CompaIn::Auxio2
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio3(&self) -> bool {
        *self == CompaIn::Auxio3
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio4(&self) -> bool {
        *self == CompaIn::Auxio4
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio5(&self) -> bool {
        *self == CompaIn::Auxio5
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio6(&self) -> bool {
        *self == CompaIn::Auxio6
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio7(&self) -> bool {
        *self == CompaIn::Auxio7
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
    pub fn auxio0(self) -> &'a mut crate::W<REG> {
        self.variant(CompaIn::Auxio0)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio1(self) -> &'a mut crate::W<REG> {
        self.variant(CompaIn::Auxio1)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio2(self) -> &'a mut crate::W<REG> {
        self.variant(CompaIn::Auxio2)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio3(self) -> &'a mut crate::W<REG> {
        self.variant(CompaIn::Auxio3)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio4(self) -> &'a mut crate::W<REG> {
        self.variant(CompaIn::Auxio4)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio5(self) -> &'a mut crate::W<REG> {
        self.variant(CompaIn::Auxio5)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio6(self) -> &'a mut crate::W<REG> {
        self.variant(CompaIn::Auxio6)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio7(self) -> &'a mut crate::W<REG> {
        self.variant(CompaIn::Auxio7)
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
