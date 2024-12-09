#[doc = "Register `CIDR3` reader"]
pub type R = crate::R<Cidr3Spec>;
#[doc = "Register `CIDR3` writer"]
pub type W = crate::W<Cidr3Spec>;
#[doc = "Field `PRMBL_3` reader - 7:0\\]
See CoreSight Architecture Specification"]
pub type Prmbl3R = crate::FieldReader;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn prmbl_3(&self) -> Prmbl3R {
        Prmbl3R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "Provides CoreSight discovery information for the ITM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cidr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cidr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cidr3Spec;
impl crate::RegisterSpec for Cidr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cidr3::R`](R) reader structure"]
impl crate::Readable for Cidr3Spec {}
#[doc = "`write(|w| ..)` method takes [`cidr3::W`](W) writer structure"]
impl crate::Writable for Cidr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CIDR3 to value 0"]
impl crate::Resettable for Cidr3Spec {
    const RESET_VALUE: u32 = 0;
}
