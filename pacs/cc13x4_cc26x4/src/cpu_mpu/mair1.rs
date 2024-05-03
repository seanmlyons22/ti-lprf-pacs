#[doc = "Register `MAIR1` reader"]
pub type R = crate::R<Mair1Spec>;
#[doc = "Register `MAIR1` writer"]
pub type W = crate::W<Mair1Spec>;
#[doc = "Field `ATTR4` reader - 7:0\\]
Memory attribute encoding for MPU regions with an AttrIndex of 4"]
pub type Attr4R = crate::FieldReader;
#[doc = "Field `ATTR4` writer - 7:0\\]
Memory attribute encoding for MPU regions with an AttrIndex of 4"]
pub type Attr4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ATTR5` reader - 15:8\\]
Memory attribute encoding for MPU regions with an AttrIndex of 5"]
pub type Attr5R = crate::FieldReader;
#[doc = "Field `ATTR5` writer - 15:8\\]
Memory attribute encoding for MPU regions with an AttrIndex of 5"]
pub type Attr5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ATTR6` reader - 23:16\\]
Memory attribute encoding for MPU regions with an AttrIndex of 6"]
pub type Attr6R = crate::FieldReader;
#[doc = "Field `ATTR6` writer - 23:16\\]
Memory attribute encoding for MPU regions with an AttrIndex of 6"]
pub type Attr6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ATTR7` reader - 31:24\\]
Memory attribute encoding for MPU regions with an AttrIndex of 7"]
pub type Attr7R = crate::FieldReader;
#[doc = "Field `ATTR7` writer - 31:24\\]
Memory attribute encoding for MPU regions with an AttrIndex of 7"]
pub type Attr7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Memory attribute encoding for MPU regions with an AttrIndex of 4"]
    #[inline(always)]
    pub fn attr4(&self) -> Attr4R {
        Attr4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Memory attribute encoding for MPU regions with an AttrIndex of 5"]
    #[inline(always)]
    pub fn attr5(&self) -> Attr5R {
        Attr5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Memory attribute encoding for MPU regions with an AttrIndex of 6"]
    #[inline(always)]
    pub fn attr6(&self) -> Attr6R {
        Attr6R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Memory attribute encoding for MPU regions with an AttrIndex of 7"]
    #[inline(always)]
    pub fn attr7(&self) -> Attr7R {
        Attr7R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Memory attribute encoding for MPU regions with an AttrIndex of 4"]
    #[inline(always)]
    #[must_use]
    pub fn attr4(&mut self) -> Attr4W<Mair1Spec> {
        Attr4W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Memory attribute encoding for MPU regions with an AttrIndex of 5"]
    #[inline(always)]
    #[must_use]
    pub fn attr5(&mut self) -> Attr5W<Mair1Spec> {
        Attr5W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Memory attribute encoding for MPU regions with an AttrIndex of 6"]
    #[inline(always)]
    #[must_use]
    pub fn attr6(&mut self) -> Attr6W<Mair1Spec> {
        Attr6W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Memory attribute encoding for MPU regions with an AttrIndex of 7"]
    #[inline(always)]
    #[must_use]
    pub fn attr7(&mut self) -> Attr7W<Mair1Spec> {
        Attr7W::new(self, 24)
    }
}
#[doc = "Along with MPU_MAIR0, provides the memory attribute encodings corresponding to the AttrIndex values\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mair1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mair1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mair1Spec;
impl crate::RegisterSpec for Mair1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mair1::R`](R) reader structure"]
impl crate::Readable for Mair1Spec {}
#[doc = "`write(|w| ..)` method takes [`mair1::W`](W) writer structure"]
impl crate::Writable for Mair1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAIR1 to value 0"]
impl crate::Resettable for Mair1Spec {
    const RESET_VALUE: u32 = 0;
}
