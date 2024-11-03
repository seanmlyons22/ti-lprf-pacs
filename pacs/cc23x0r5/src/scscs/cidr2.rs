#[doc = "Register `CIDR2` reader"]
pub type R = crate::R<Cidr2Spec>;
#[doc = "Register `CIDR2` writer"]
pub type W = crate::W<Cidr2Spec>;
#[doc = "Field `PRMBL_2` reader - 7:0\\]
Contains bits \\[23:16\\]
of the component identification"]
pub type Prmbl2R = crate::FieldReader;
#[doc = "Field `PRMBL_2` writer - 7:0\\]
Contains bits \\[23:16\\]
of the component identification"]
pub type Prmbl2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Contains bits \\[23:16\\]
of the component identification"]
    #[inline(always)]
    pub fn prmbl_2(&self) -> Prmbl2R {
        Prmbl2R::new((self.bits & 0xff) as u8)
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
Contains bits \\[23:16\\]
of the component identification"]
    #[inline(always)]
    #[must_use]
    pub fn prmbl_2(&mut self) -> Prmbl2W<Cidr2Spec> {
        Prmbl2W::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<Cidr2Spec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "A component identification register, that indicates that the identification registers are present.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cidr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cidr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cidr2Spec;
impl crate::RegisterSpec for Cidr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cidr2::R`](R) reader structure"]
impl crate::Readable for Cidr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cidr2::W`](W) writer structure"]
impl crate::Writable for Cidr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CIDR2 to value 0x05"]
impl crate::Resettable for Cidr2Spec {
    const RESET_VALUE: u32 = 0x05;
}
