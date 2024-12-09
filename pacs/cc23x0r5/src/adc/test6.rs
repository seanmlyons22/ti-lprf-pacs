#[doc = "Register `TEST6` reader"]
pub type R = crate::R<Test6Spec>;
#[doc = "Register `TEST6` writer"]
pub type W = crate::W<Test6Spec>;
#[doc = "3:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Atestsel {
    #[doc = "8: Internal. Only to be used through TI provided API."]
    Val8 = 8,
    #[doc = "4: Internal. Only to be used through TI provided API."]
    Val4 = 4,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    Val2 = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Val1 = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Val0 = 0,
}
impl From<Atestsel> for u8 {
    #[inline(always)]
    fn from(variant: Atestsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Atestsel {
    type Ux = u8;
}
impl crate::IsEnum for Atestsel {}
#[doc = "Field `ATESTSEL` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type AtestselR = crate::FieldReader<Atestsel>;
impl AtestselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Atestsel> {
        match self.bits {
            8 => Some(Atestsel::Val8),
            4 => Some(Atestsel::Val4),
            2 => Some(Atestsel::Val2),
            1 => Some(Atestsel::Val1),
            0 => Some(Atestsel::Val0),
            _ => None,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_val8(&self) -> bool {
        *self == Atestsel::Val8
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_val4(&self) -> bool {
        *self == Atestsel::Val4
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_val2(&self) -> bool {
        *self == Atestsel::Val2
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_val1(&self) -> bool {
        *self == Atestsel::Val1
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_val0(&self) -> bool {
        *self == Atestsel::Val0
    }
}
#[doc = "Field `ATESTSEL` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type AtestselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Atestsel>;
impl<'a, REG> AtestselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn val8(self) -> &'a mut crate::W<REG> {
        self.variant(Atestsel::Val8)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn val4(self) -> &'a mut crate::W<REG> {
        self.variant(Atestsel::Val4)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn val2(self) -> &'a mut crate::W<REG> {
        self.variant(Atestsel::Val2)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn val1(self) -> &'a mut crate::W<REG> {
        self.variant(Atestsel::Val1)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn val0(self) -> &'a mut crate::W<REG> {
        self.variant(Atestsel::Val0)
    }
}
#[doc = "Field `RESERVED4` reader - 31:4\\]
Internal. Only to be used through TI provided API."]
pub type Reserved4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn atestsel(&self) -> AtestselR {
        AtestselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn atestsel(&mut self) -> AtestselW<Test6Spec> {
        AtestselW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Test6Spec;
impl crate::RegisterSpec for Test6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test6::R`](R) reader structure"]
impl crate::Readable for Test6Spec {}
#[doc = "`write(|w| ..)` method takes [`test6::W`](W) writer structure"]
impl crate::Writable for Test6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TEST6 to value 0"]
impl crate::Resettable for Test6Spec {
    const RESET_VALUE: u32 = 0;
}
