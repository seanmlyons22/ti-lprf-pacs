#[doc = "Register `CIDR0` reader"]
pub type R = crate::R<Cidr0Spec>;
#[doc = "Register `CIDR0` writer"]
pub type W = crate::W<Cidr0Spec>;
#[doc = "Field `PRMBL_0` reader - 7:0\\]
See CoreSight Architecture Specification"]
pub type Prmbl0R = crate::FieldReader;
#[doc = "Field `PRMBL_0` writer - 7:0\\]
See CoreSight Architecture Specification"]
pub type Prmbl0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn prmbl_0(&self) -> Prmbl0R {
        Prmbl0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
See CoreSight Architecture Specification"]
    #[inline(always)]
    #[must_use]
    pub fn prmbl_0(&mut self) -> Prmbl0W<Cidr0Spec> {
        Prmbl0W::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<Cidr0Spec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Provides CoreSight discovery information for the ITM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cidr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cidr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cidr0Spec;
impl crate::RegisterSpec for Cidr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cidr0::R`](R) reader structure"]
impl crate::Readable for Cidr0Spec {}
#[doc = "`write(|w| ..)` method takes [`cidr0::W`](W) writer structure"]
impl crate::Writable for Cidr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CIDR0 to value 0"]
impl crate::Resettable for Cidr0Spec {
    const RESET_VALUE: u32 = 0;
}
