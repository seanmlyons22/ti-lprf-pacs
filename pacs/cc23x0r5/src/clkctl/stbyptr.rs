#[doc = "Register `STBYPTR` reader"]
pub type R = crate::R<StbyptrSpec>;
#[doc = "Register `STBYPTR` writer"]
pub type W = crate::W<StbyptrSpec>;
#[doc = "31:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Val {
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Min = 0,
}
impl From<Val> for u32 {
    #[inline(always)]
    fn from(variant: Val) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Val {
    type Ux = u32;
}
impl crate::IsEnum for Val {}
#[doc = "Field `VAL` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type ValR = crate::FieldReader<Val>;
impl ValR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Val> {
        match self.bits {
            0 => Some(Val::Min),
            _ => None,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == Val::Min
    }
}
#[doc = "Field `VAL` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, Val>;
impl<'a, REG> ValW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn min(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Min)
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<StbyptrSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stbyptr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stbyptr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StbyptrSpec;
impl crate::RegisterSpec for StbyptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stbyptr::R`](R) reader structure"]
impl crate::Readable for StbyptrSpec {}
#[doc = "`write(|w| ..)` method takes [`stbyptr::W`](W) writer structure"]
impl crate::Writable for StbyptrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STBYPTR to value 0"]
impl crate::Resettable for StbyptrSpec {
    const RESET_VALUE: u32 = 0;
}
