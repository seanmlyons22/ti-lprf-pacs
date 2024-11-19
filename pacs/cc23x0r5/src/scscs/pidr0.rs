#[doc = "Register `PIDR0` reader"]
pub type R = crate::R<Pidr0Spec>;
#[doc = "Register `PIDR0` writer"]
pub type W = crate::W<Pidr0Spec>;
#[doc = "Field `PART_0` reader - 7:0\\]
Bits \\[7:0\\]
of the component's part number. This is selected by the designer of the component."]
pub type Part0R = crate::FieldReader;
#[doc = "Field `PART_0` writer - 7:0\\]
Bits \\[7:0\\]
of the component's part number. This is selected by the designer of the component."]
pub type Part0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Bits \\[7:0\\]
of the component's part number. This is selected by the designer of the component."]
    #[inline(always)]
    pub fn part_0(&self) -> Part0R {
        Part0R::new((self.bits & 0xff) as u8)
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
Bits \\[7:0\\]
of the component's part number. This is selected by the designer of the component."]
    #[inline(always)]
    #[must_use]
    pub fn part_0(&mut self) -> Part0W<Pidr0Spec> {
        Part0W::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<Pidr0Spec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Part of the set of Peripheral Identification registers. Contains part of the designer specific part number.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pidr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pidr0Spec;
impl crate::RegisterSpec for Pidr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr0::R`](R) reader structure"]
impl crate::Readable for Pidr0Spec {}
#[doc = "`write(|w| ..)` method takes [`pidr0::W`](W) writer structure"]
impl crate::Writable for Pidr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PIDR0 to value 0x08"]
impl crate::Resettable for Pidr0Spec {
    const RESET_VALUE: u32 = 0x08;
}