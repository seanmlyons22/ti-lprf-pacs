#[doc = "Register `HWVER1` reader"]
pub type R = crate::R<Hwver1Spec>;
#[doc = "Register `HWVER1` writer"]
pub type W = crate::W<Hwver1Spec>;
#[doc = "Field `REV` reader - 7:0\\]
The revision number of this module is Rev 2.0."]
pub type RevR = crate::FieldReader;
#[doc = "Field `REV` writer - 7:0\\]
The revision number of this module is Rev 2.0."]
pub type RevW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
The revision number of this module is Rev 2.0."]
    #[inline(always)]
    pub fn rev(&self) -> RevR {
        RevR::new((self.bits & 0xff) as u8)
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
The revision number of this module is Rev 2.0."]
    #[inline(always)]
    #[must_use]
    pub fn rev(&mut self) -> RevW<Hwver1Spec> {
        RevW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<Hwver1Spec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "HW Version 1 TRNG Revision Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwver1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwver1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hwver1Spec;
impl crate::RegisterSpec for Hwver1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwver1::R`](R) reader structure"]
impl crate::Readable for Hwver1Spec {}
#[doc = "`write(|w| ..)` method takes [`hwver1::W`](W) writer structure"]
impl crate::Writable for Hwver1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWVER1 to value 0x20"]
impl crate::Resettable for Hwver1Spec {
    const RESET_VALUE: u32 = 0x20;
}
