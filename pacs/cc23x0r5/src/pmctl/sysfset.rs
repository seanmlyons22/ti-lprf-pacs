#[doc = "Register `SYSFSET` reader"]
pub type R = crate::R<SysfsetSpec>;
#[doc = "Register `SYSFSET` writer"]
pub type W = crate::W<SysfsetSpec>;
#[doc = "0:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flag0 {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Set = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Noeff = 0,
}
impl From<Flag0> for bool {
    #[inline(always)]
    fn from(variant: Flag0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLAG0` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type Flag0W<'a, REG> = crate::BitWriter<'a, REG, Flag0>;
impl<'a, REG> Flag0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Flag0::Set)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Flag0::Noeff)
    }
}
#[doc = "1:1\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flag1 {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Set = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Noeff = 0,
}
impl From<Flag1> for bool {
    #[inline(always)]
    fn from(variant: Flag1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLAG1` writer - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type Flag1W<'a, REG> = crate::BitWriter<'a, REG, Flag1>;
impl<'a, REG> Flag1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Flag1::Set)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Flag1::Noeff)
    }
}
#[doc = "2:2\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flag2 {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Set = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Noeff = 0,
}
impl From<Flag2> for bool {
    #[inline(always)]
    fn from(variant: Flag2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLAG2` writer - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type Flag2W<'a, REG> = crate::BitWriter<'a, REG, Flag2>;
impl<'a, REG> Flag2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Flag2::Set)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Flag2::Noeff)
    }
}
#[doc = "Field `RESERVED3` reader - 31:3\\]
Internal. Only to be used through TI provided API."]
pub type Reserved3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 3:31 - 31:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn flag0(&mut self) -> Flag0W<SysfsetSpec> {
        Flag0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn flag1(&mut self) -> Flag1W<SysfsetSpec> {
        Flag1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn flag2(&mut self) -> Flag2W<SysfsetSpec> {
        Flag2W::new(self, 2)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysfset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysfset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysfsetSpec;
impl crate::RegisterSpec for SysfsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysfset::R`](R) reader structure"]
impl crate::Readable for SysfsetSpec {}
#[doc = "`write(|w| ..)` method takes [`sysfset::W`](W) writer structure"]
impl crate::Writable for SysfsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSFSET to value 0"]
impl crate::Resettable for SysfsetSpec {
    const RESET_VALUE: u32 = 0;
}
