#[doc = "Register `ID_ISAR2` reader"]
pub type R = crate::R<IdIsar2Spec>;
#[doc = "Register `ID_ISAR2` writer"]
pub type W = crate::W<IdIsar2Spec>;
#[doc = "Field `RESERVED0` reader - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED0` writer - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<IdIsar2Spec> {
        Reserved0W::new(self, 0)
    }
}
#[doc = "ISA Feature 2 Information on the instruction set attributes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id_isar2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`id_isar2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdIsar2Spec;
impl crate::RegisterSpec for IdIsar2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id_isar2::R`](R) reader structure"]
impl crate::Readable for IdIsar2Spec {}
#[doc = "`write(|w| ..)` method takes [`id_isar2::W`](W) writer structure"]
impl crate::Writable for IdIsar2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ID_ISAR2 to value 0x2123_2231"]
impl crate::Resettable for IdIsar2Spec {
    const RESET_VALUE: u32 = 0x2123_2231;
}
