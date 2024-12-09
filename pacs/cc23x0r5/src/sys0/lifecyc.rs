#[doc = "Register `LIFECYC` reader"]
pub type R = crate::R<LifecycSpec>;
#[doc = "Register `LIFECYC` writer"]
pub type W = crate::W<LifecycSpec>;
#[doc = "7:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Val {
    #[doc = "255: Internal. Only to be used through TI provided API."]
    LcycleIllegal = 255,
    #[doc = "6: Internal. Only to be used through TI provided API."]
    LcycleRetest = 6,
    #[doc = "5: Internal. Only to be used through TI provided API."]
    LcycleProddev = 5,
    #[doc = "4: Internal. Only to be used through TI provided API."]
    LcycleEngrdev = 4,
    #[doc = "3: Internal. Only to be used through TI provided API."]
    LcycleTestft = 3,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    LcycleTestpt = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Lcycle1stbday = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    LcyclePossible1stbday = 0,
}
impl From<Val> for u8 {
    #[inline(always)]
    fn from(variant: Val) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Val {
    type Ux = u8;
}
impl crate::IsEnum for Val {}
#[doc = "Field `VAL` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type ValR = crate::FieldReader<Val>;
impl ValR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Val> {
        match self.bits {
            255 => Some(Val::LcycleIllegal),
            6 => Some(Val::LcycleRetest),
            5 => Some(Val::LcycleProddev),
            4 => Some(Val::LcycleEngrdev),
            3 => Some(Val::LcycleTestft),
            2 => Some(Val::LcycleTestpt),
            1 => Some(Val::Lcycle1stbday),
            0 => Some(Val::LcyclePossible1stbday),
            _ => None,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_lcycle_illegal(&self) -> bool {
        *self == Val::LcycleIllegal
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_lcycle_retest(&self) -> bool {
        *self == Val::LcycleRetest
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_lcycle_proddev(&self) -> bool {
        *self == Val::LcycleProddev
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_lcycle_engrdev(&self) -> bool {
        *self == Val::LcycleEngrdev
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_lcycle_testft(&self) -> bool {
        *self == Val::LcycleTestft
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_lcycle_testpt(&self) -> bool {
        *self == Val::LcycleTestpt
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_lcycle_1stbday(&self) -> bool {
        *self == Val::Lcycle1stbday
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_lcycle_possible_1stbday(&self) -> bool {
        *self == Val::LcyclePossible1stbday
    }
}
#[doc = "Field `VAL` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 8, Val>;
impl<'a, REG> ValW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lcycle_illegal(self) -> &'a mut crate::W<REG> {
        self.variant(Val::LcycleIllegal)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lcycle_retest(self) -> &'a mut crate::W<REG> {
        self.variant(Val::LcycleRetest)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lcycle_proddev(self) -> &'a mut crate::W<REG> {
        self.variant(Val::LcycleProddev)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lcycle_engrdev(self) -> &'a mut crate::W<REG> {
        self.variant(Val::LcycleEngrdev)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lcycle_testft(self) -> &'a mut crate::W<REG> {
        self.variant(Val::LcycleTestft)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lcycle_testpt(self) -> &'a mut crate::W<REG> {
        self.variant(Val::LcycleTestpt)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lcycle_1stbday(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Lcycle1stbday)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lcycle_possible_1stbday(self) -> &'a mut crate::W<REG> {
        self.variant(Val::LcyclePossible1stbday)
    }
}
#[doc = "Field `RESERVED8` reader - 31:8\\]
Internal. Only to be used through TI provided API."]
pub type Reserved8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<LifecycSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lifecyc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lifecyc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LifecycSpec;
impl crate::RegisterSpec for LifecycSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lifecyc::R`](R) reader structure"]
impl crate::Readable for LifecycSpec {}
#[doc = "`write(|w| ..)` method takes [`lifecyc::W`](W) writer structure"]
impl crate::Writable for LifecycSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LIFECYC to value 0"]
impl crate::Resettable for LifecycSpec {
    const RESET_VALUE: u32 = 0;
}
