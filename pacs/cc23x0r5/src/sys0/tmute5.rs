#[doc = "Register `TMUTE5` reader"]
pub type R = crate::R<Tmute5Spec>;
#[doc = "Register `TMUTE5` writer"]
pub type W = crate::W<Tmute5Spec>;
#[doc = "Field `GLDOISSET` writer - 4:0\\]
GLDO current source trim set value"]
pub type GldoissetW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RESERVED0` reader - 4:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `GLDOISCLR` writer - 9:5\\]
GLDO current source trim clear value"]
pub type GldoisclrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DCDCDRVDS` reader - 12:10\\]
DCDC: Driver drive strength configuration"]
pub type DcdcdrvdsR = crate::FieldReader;
#[doc = "Field `DCDCDRVDS` writer - 12:10\\]
DCDC: Driver drive strength configuration"]
pub type DcdcdrvdsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESERVED13` reader - 31:13\\]
RESERVED"]
pub type Reserved13R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 10:12 - 12:10\\]
DCDC: Driver drive strength configuration"]
    #[inline(always)]
    pub fn dcdcdrvds(&self) -> DcdcdrvdsR {
        DcdcdrvdsR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:31 - 31:13\\]
RESERVED"]
    #[inline(always)]
    pub fn reserved13(&self) -> Reserved13R {
        Reserved13R::new((self.bits >> 13) & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
GLDO current source trim set value"]
    #[inline(always)]
    #[must_use]
    pub fn gldoisset(&mut self) -> GldoissetW<Tmute5Spec> {
        GldoissetW::new(self, 0)
    }
    #[doc = "Bits 5:9 - 9:5\\]
GLDO current source trim clear value"]
    #[inline(always)]
    #[must_use]
    pub fn gldoisclr(&mut self) -> GldoisclrW<Tmute5Spec> {
        GldoisclrW::new(self, 5)
    }
    #[doc = "Bits 10:12 - 12:10\\]
DCDC: Driver drive strength configuration"]
    #[inline(always)]
    #[must_use]
    pub fn dcdcdrvds(&mut self) -> DcdcdrvdsW<Tmute5Spec> {
        DcdcdrvdsW::new(self, 10)
    }
}
#[doc = "TMUTE5 trim Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmute5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmute5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmute5Spec;
impl crate::RegisterSpec for Tmute5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmute5::R`](R) reader structure"]
impl crate::Readable for Tmute5Spec {}
#[doc = "`write(|w| ..)` method takes [`tmute5::W`](W) writer structure"]
impl crate::Writable for Tmute5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMUTE5 to value 0"]
impl crate::Resettable for Tmute5Spec {
    const RESET_VALUE: u32 = 0;
}
