#[doc = "Register `PIDR1` reader"]
pub type R = crate::R<Pidr1Spec>;
#[doc = "Register `PIDR1` writer"]
pub type W = crate::W<Pidr1Spec>;
#[doc = "Field `PART_1` reader - 3:0\\]
See CoreSight Architecture Specification"]
pub type Part1R = crate::FieldReader;
#[doc = "Field `DES_0` reader - 7:4\\]
See CoreSight Architecture Specification"]
pub type Des0R = crate::FieldReader;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn part_1(&self) -> Part1R {
        Part1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn des_0(&self) -> Des0R {
        Des0R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "Provides CoreSight discovery information for the ITM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pidr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pidr1Spec;
impl crate::RegisterSpec for Pidr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr1::R`](R) reader structure"]
impl crate::Readable for Pidr1Spec {}
#[doc = "`write(|w| ..)` method takes [`pidr1::W`](W) writer structure"]
impl crate::Writable for Pidr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PIDR1 to value 0"]
impl crate::Resettable for Pidr1Spec {
    const RESET_VALUE: u32 = 0;
}
