#[doc = "Register `MAIR0` reader"]
pub type R = crate::R<Mair0Spec>;
#[doc = "Register `MAIR0` writer"]
pub type W = crate::W<Mair0Spec>;
#[doc = "Field `ATTR0` reader - 7:0\\]
Memory attribute encoding for MPU regions with an AttrIndex of 0"]
pub type Attr0R = crate::FieldReader;
#[doc = "Field `ATTR0` writer - 7:0\\]
Memory attribute encoding for MPU regions with an AttrIndex of 0"]
pub type Attr0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ATTR1` reader - 15:8\\]
Memory attribute encoding for MPU regions with an AttrIndex of 1"]
pub type Attr1R = crate::FieldReader;
#[doc = "Field `ATTR1` writer - 15:8\\]
Memory attribute encoding for MPU regions with an AttrIndex of 1"]
pub type Attr1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ATTR2` reader - 23:16\\]
Memory attribute encoding for MPU regions with an AttrIndex of 2"]
pub type Attr2R = crate::FieldReader;
#[doc = "Field `ATTR2` writer - 23:16\\]
Memory attribute encoding for MPU regions with an AttrIndex of 2"]
pub type Attr2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ATTR3` reader - 31:24\\]
Memory attribute encoding for MPU regions with an AttrIndex of 3"]
pub type Attr3R = crate::FieldReader;
#[doc = "Field `ATTR3` writer - 31:24\\]
Memory attribute encoding for MPU regions with an AttrIndex of 3"]
pub type Attr3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Memory attribute encoding for MPU regions with an AttrIndex of 0"]
    #[inline(always)]
    pub fn attr0(&self) -> Attr0R {
        Attr0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Memory attribute encoding for MPU regions with an AttrIndex of 1"]
    #[inline(always)]
    pub fn attr1(&self) -> Attr1R {
        Attr1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Memory attribute encoding for MPU regions with an AttrIndex of 2"]
    #[inline(always)]
    pub fn attr2(&self) -> Attr2R {
        Attr2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Memory attribute encoding for MPU regions with an AttrIndex of 3"]
    #[inline(always)]
    pub fn attr3(&self) -> Attr3R {
        Attr3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Memory attribute encoding for MPU regions with an AttrIndex of 0"]
    #[inline(always)]
    #[must_use]
    pub fn attr0(&mut self) -> Attr0W<Mair0Spec> {
        Attr0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Memory attribute encoding for MPU regions with an AttrIndex of 1"]
    #[inline(always)]
    #[must_use]
    pub fn attr1(&mut self) -> Attr1W<Mair0Spec> {
        Attr1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Memory attribute encoding for MPU regions with an AttrIndex of 2"]
    #[inline(always)]
    #[must_use]
    pub fn attr2(&mut self) -> Attr2W<Mair0Spec> {
        Attr2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Memory attribute encoding for MPU regions with an AttrIndex of 3"]
    #[inline(always)]
    #[must_use]
    pub fn attr3(&mut self) -> Attr3W<Mair0Spec> {
        Attr3W::new(self, 24)
    }
}
#[doc = "Along with MPU_MAIR1, provides the memory attribute encodings corresponding to the AttrIndex values\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mair0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mair0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mair0Spec;
impl crate::RegisterSpec for Mair0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mair0::R`](R) reader structure"]
impl crate::Readable for Mair0Spec {}
#[doc = "`write(|w| ..)` method takes [`mair0::W`](W) writer structure"]
impl crate::Writable for Mair0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAIR0 to value 0"]
impl crate::Resettable for Mair0Spec {
    const RESET_VALUE: u32 = 0;
}
