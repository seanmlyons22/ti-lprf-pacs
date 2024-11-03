#[doc = "Register `FLWS2T` reader"]
pub type R = crate::R<Flws2tSpec>;
#[doc = "Register `FLWS2T` writer"]
pub type W = crate::W<Flws2tSpec>;
#[doc = "2:0\\]
Specifies the waitstate value.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Val {
    #[doc = "7: Wait state value 7"]
    Ws7 = 7,
    #[doc = "6: Wait state value 6"]
    Ws6 = 6,
    #[doc = "5: Wait state value 5"]
    Ws5 = 5,
    #[doc = "4: Wait state value 4"]
    Ws4 = 4,
    #[doc = "3: Wait state value 3"]
    Ws3 = 3,
    #[doc = "2: Wait state value 2"]
    Ws2 = 2,
    #[doc = "1: Wait state value 1"]
    Ws1 = 1,
    #[doc = "0: Wait state value 0"]
    Ws0 = 0,
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
#[doc = "Field `VAL` reader - 2:0\\]
Specifies the waitstate value."]
pub type ValR = crate::FieldReader<Val>;
impl ValR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Val {
        match self.bits {
            7 => Val::Ws7,
            6 => Val::Ws6,
            5 => Val::Ws5,
            4 => Val::Ws4,
            3 => Val::Ws3,
            2 => Val::Ws2,
            1 => Val::Ws1,
            0 => Val::Ws0,
            _ => unreachable!(),
        }
    }
    #[doc = "Wait state value 7"]
    #[inline(always)]
    pub fn is_ws7(&self) -> bool {
        *self == Val::Ws7
    }
    #[doc = "Wait state value 6"]
    #[inline(always)]
    pub fn is_ws6(&self) -> bool {
        *self == Val::Ws6
    }
    #[doc = "Wait state value 5"]
    #[inline(always)]
    pub fn is_ws5(&self) -> bool {
        *self == Val::Ws5
    }
    #[doc = "Wait state value 4"]
    #[inline(always)]
    pub fn is_ws4(&self) -> bool {
        *self == Val::Ws4
    }
    #[doc = "Wait state value 3"]
    #[inline(always)]
    pub fn is_ws3(&self) -> bool {
        *self == Val::Ws3
    }
    #[doc = "Wait state value 2"]
    #[inline(always)]
    pub fn is_ws2(&self) -> bool {
        *self == Val::Ws2
    }
    #[doc = "Wait state value 1"]
    #[inline(always)]
    pub fn is_ws1(&self) -> bool {
        *self == Val::Ws1
    }
    #[doc = "Wait state value 0"]
    #[inline(always)]
    pub fn is_ws0(&self) -> bool {
        *self == Val::Ws0
    }
}
#[doc = "Field `VAL` writer - 2:0\\]
Specifies the waitstate value."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 3, Val, crate::Safe>;
impl<'a, REG> ValW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Wait state value 7"]
    #[inline(always)]
    pub fn ws7(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Ws7)
    }
    #[doc = "Wait state value 6"]
    #[inline(always)]
    pub fn ws6(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Ws6)
    }
    #[doc = "Wait state value 5"]
    #[inline(always)]
    pub fn ws5(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Ws5)
    }
    #[doc = "Wait state value 4"]
    #[inline(always)]
    pub fn ws4(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Ws4)
    }
    #[doc = "Wait state value 3"]
    #[inline(always)]
    pub fn ws3(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Ws3)
    }
    #[doc = "Wait state value 2"]
    #[inline(always)]
    pub fn ws2(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Ws2)
    }
    #[doc = "Wait state value 1"]
    #[inline(always)]
    pub fn ws1(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Ws1)
    }
    #[doc = "Wait state value 0"]
    #[inline(always)]
    pub fn ws0(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Ws0)
    }
}
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Specifies the waitstate value."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Specifies the waitstate value."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<Flws2tSpec> {
        ValW::new(self, 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<Flws2tSpec> {
        Reserved3W::new(self, 3)
    }
}
#[doc = "This register is used to specify the number of waitstates necessary for accessing the flash in 2T mode. This register is retained.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flws2t::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flws2t::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Flws2tSpec;
impl crate::RegisterSpec for Flws2tSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flws2t::R`](R) reader structure"]
impl crate::Readable for Flws2tSpec {}
#[doc = "`write(|w| ..)` method takes [`flws2t::W`](W) writer structure"]
impl crate::Writable for Flws2tSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLWS2T to value 0x07"]
impl crate::Resettable for Flws2tSpec {
    const RESET_VALUE: u32 = 0x07;
}
