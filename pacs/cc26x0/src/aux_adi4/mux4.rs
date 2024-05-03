#[doc = "Register `MUX4` reader"]
pub type R = crate::R<Mux4Spec>;
#[doc = "Register `MUX4` writer"]
pub type W = crate::W<Mux4Spec>;
#[doc = "7:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CompaRef {
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
#[doc = "Field `COMPA_REF` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type CompaRefR = crate::FieldReader<CompaRef>;
impl CompaRefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CompaRef> {
        match self.bits {
            128 => Some(CompaRef::Auxio0),
            64 => Some(CompaRef::Auxio1),
            32 => Some(CompaRef::Auxio2),
            16 => Some(CompaRef::Auxio3),
            8 => Some(CompaRef::Auxio4),
            4 => Some(CompaRef::Auxio5),
            2 => Some(CompaRef::Auxio6),
            1 => Some(CompaRef::Auxio7),
            0 => Some(CompaRef::Nc),
            _ => None,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio0(&self) -> bool {
        *self == CompaRef::Auxio0
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio1(&self) -> bool {
        *self == CompaRef::Auxio1
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio2(&self) -> bool {
        *self == CompaRef::Auxio2
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio3(&self) -> bool {
        *self == CompaRef::Auxio3
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio4(&self) -> bool {
        *self == CompaRef::Auxio4
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio5(&self) -> bool {
        *self == CompaRef::Auxio5
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio6(&self) -> bool {
        *self == CompaRef::Auxio6
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_auxio7(&self) -> bool {
        *self == CompaRef::Auxio7
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == CompaRef::Nc
    }
}
#[doc = "Field `COMPA_REF` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type CompaRefW<'a, REG> = crate::FieldWriter<'a, REG, 8, CompaRef>;
impl<'a, REG> CompaRefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio0(self) -> &'a mut crate::W<REG> {
        self.variant(CompaRef::Auxio0)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio1(self) -> &'a mut crate::W<REG> {
        self.variant(CompaRef::Auxio1)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio2(self) -> &'a mut crate::W<REG> {
        self.variant(CompaRef::Auxio2)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio3(self) -> &'a mut crate::W<REG> {
        self.variant(CompaRef::Auxio3)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio4(self) -> &'a mut crate::W<REG> {
        self.variant(CompaRef::Auxio4)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio5(self) -> &'a mut crate::W<REG> {
        self.variant(CompaRef::Auxio5)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio6(self) -> &'a mut crate::W<REG> {
        self.variant(CompaRef::Auxio6)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio7(self) -> &'a mut crate::W<REG> {
        self.variant(CompaRef::Auxio7)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn nc(self) -> &'a mut crate::W<REG> {
        self.variant(CompaRef::Nc)
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn compa_ref(&self) -> CompaRefR {
        CompaRefR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn compa_ref(&mut self) -> CompaRefW<Mux4Spec> {
        CompaRefW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mux4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mux4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mux4Spec;
impl crate::RegisterSpec for Mux4Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mux4::R`](R) reader structure"]
impl crate::Readable for Mux4Spec {}
#[doc = "`write(|w| ..)` method takes [`mux4::W`](W) writer structure"]
impl crate::Writable for Mux4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets MUX4 to value 0"]
impl crate::Resettable for Mux4Spec {
    const RESET_VALUE: u8 = 0;
}
