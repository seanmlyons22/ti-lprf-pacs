#[doc = "Register `SYSFSTA` reader"]
pub type R = crate::R<SysfstaSpec>;
#[doc = "Register `SYSFSTA` writer"]
pub type W = crate::W<SysfstaSpec>;
#[doc = "0:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flag0 {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Val1 = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Val0 = 0,
}
impl From<Flag0> for bool {
    #[inline(always)]
    fn from(variant: Flag0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLAG0` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type Flag0R = crate::BitReader<Flag0>;
impl Flag0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flag0 {
        match self.bits {
            true => Flag0::Val1,
            false => Flag0::Val0,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_val1(&self) -> bool {
        *self == Flag0::Val1
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_val0(&self) -> bool {
        *self == Flag0::Val0
    }
}
#[doc = "1:1\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flag1 {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Val1 = 1,
}
impl From<Flag1> for bool {
    #[inline(always)]
    fn from(variant: Flag1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLAG1` reader - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type Flag1R = crate::BitReader<Flag1>;
impl Flag1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Flag1> {
        match self.bits {
            true => Some(Flag1::Val1),
            _ => None,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_val1(&self) -> bool {
        *self == Flag1::Val1
    }
}
#[doc = "2:2\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flag2 {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Val1 = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Val0 = 0,
}
impl From<Flag2> for bool {
    #[inline(always)]
    fn from(variant: Flag2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLAG2` reader - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type Flag2R = crate::BitReader<Flag2>;
impl Flag2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flag2 {
        match self.bits {
            true => Flag2::Val1,
            false => Flag2::Val0,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_val1(&self) -> bool {
        *self == Flag2::Val1
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_val0(&self) -> bool {
        *self == Flag2::Val0
    }
}
#[doc = "Field `RESERVED3` reader - 31:3\\]
Internal. Only to be used through TI provided API."]
pub type Reserved3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn flag0(&self) -> Flag0R {
        Flag0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn flag1(&self) -> Flag1R {
        Flag1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn flag2(&self) -> Flag2R {
        Flag2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysfsta::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysfsta::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysfstaSpec;
impl crate::RegisterSpec for SysfstaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysfsta::R`](R) reader structure"]
impl crate::Readable for SysfstaSpec {}
#[doc = "`write(|w| ..)` method takes [`sysfsta::W`](W) writer structure"]
impl crate::Writable for SysfstaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSFSTA to value 0"]
impl crate::Resettable for SysfstaSpec {
    const RESET_VALUE: u32 = 0;
}
