#[doc = "Register `TMUTE3` reader"]
pub type R = crate::R<Tmute3Spec>;
#[doc = "Register `TMUTE3` writer"]
pub type W = crate::W<Tmute3Spec>;
#[doc = "Field `TEMPC0` reader - 7:0\\]
BATMON: Temperature calculation coefficient 0"]
pub type Tempc0R = crate::FieldReader;
#[doc = "Field `TEMPC0` writer - 7:0\\]
BATMON: Temperature calculation coefficient 0"]
pub type Tempc0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TEMPC1` reader - 13:8\\]
BATMON: Temperature calculation coefficient 1"]
pub type Tempc1R = crate::FieldReader;
#[doc = "Field `TEMPC1` writer - 13:8\\]
BATMON: Temperature calculation coefficient 1"]
pub type Tempc1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TEMPC2` reader - 18:14\\]
BATMON: Temperature calculation coefficient 2"]
pub type Tempc2R = crate::FieldReader;
#[doc = "Field `TEMPC2` writer - 18:14\\]
BATMON: Temperature calculation coefficient 2"]
pub type Tempc2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BATC0` reader - 25:19\\]
BATMON: Battery calculation coefficient 0"]
pub type Batc0R = crate::FieldReader;
#[doc = "Field `BATC0` writer - 25:19\\]
BATMON: Battery calculation coefficient 0"]
pub type Batc0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `BATC1` reader - 31:26\\]
BATMON: Battery calculation coefficient 1"]
pub type Batc1R = crate::FieldReader;
#[doc = "Field `BATC1` writer - 31:26\\]
BATMON: Battery calculation coefficient 1"]
pub type Batc1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
BATMON: Temperature calculation coefficient 0"]
    #[inline(always)]
    pub fn tempc0(&self) -> Tempc0R {
        Tempc0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
BATMON: Temperature calculation coefficient 1"]
    #[inline(always)]
    pub fn tempc1(&self) -> Tempc1R {
        Tempc1R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:18 - 18:14\\]
BATMON: Temperature calculation coefficient 2"]
    #[inline(always)]
    pub fn tempc2(&self) -> Tempc2R {
        Tempc2R::new(((self.bits >> 14) & 0x1f) as u8)
    }
    #[doc = "Bits 19:25 - 25:19\\]
BATMON: Battery calculation coefficient 0"]
    #[inline(always)]
    pub fn batc0(&self) -> Batc0R {
        Batc0R::new(((self.bits >> 19) & 0x7f) as u8)
    }
    #[doc = "Bits 26:31 - 31:26\\]
BATMON: Battery calculation coefficient 1"]
    #[inline(always)]
    pub fn batc1(&self) -> Batc1R {
        Batc1R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
BATMON: Temperature calculation coefficient 0"]
    #[inline(always)]
    #[must_use]
    pub fn tempc0(&mut self) -> Tempc0W<Tmute3Spec> {
        Tempc0W::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
BATMON: Temperature calculation coefficient 1"]
    #[inline(always)]
    #[must_use]
    pub fn tempc1(&mut self) -> Tempc1W<Tmute3Spec> {
        Tempc1W::new(self, 8)
    }
    #[doc = "Bits 14:18 - 18:14\\]
BATMON: Temperature calculation coefficient 2"]
    #[inline(always)]
    #[must_use]
    pub fn tempc2(&mut self) -> Tempc2W<Tmute3Spec> {
        Tempc2W::new(self, 14)
    }
    #[doc = "Bits 19:25 - 25:19\\]
BATMON: Battery calculation coefficient 0"]
    #[inline(always)]
    #[must_use]
    pub fn batc0(&mut self) -> Batc0W<Tmute3Spec> {
        Batc0W::new(self, 19)
    }
    #[doc = "Bits 26:31 - 31:26\\]
BATMON: Battery calculation coefficient 1"]
    #[inline(always)]
    #[must_use]
    pub fn batc1(&mut self) -> Batc1W<Tmute3Spec> {
        Batc1W::new(self, 26)
    }
}
#[doc = "TMUTE3 trim Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmute3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmute3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmute3Spec;
impl crate::RegisterSpec for Tmute3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmute3::R`](R) reader structure"]
impl crate::Readable for Tmute3Spec {}
#[doc = "`write(|w| ..)` method takes [`tmute3::W`](W) writer structure"]
impl crate::Writable for Tmute3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMUTE3 to value 0"]
impl crate::Resettable for Tmute3Spec {
    const RESET_VALUE: u32 = 0;
}
