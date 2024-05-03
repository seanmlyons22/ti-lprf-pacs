#[doc = "Register `ID_ISAR3` reader"]
pub type R = crate::R<IdIsar3Spec>;
#[doc = "Register `ID_ISAR3` writer"]
pub type W = crate::W<IdIsar3Spec>;
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
    pub fn reserved0(&mut self) -> Reserved0W<IdIsar3Spec> {
        Reserved0W::new(self, 0)
    }
}
#[doc = "ISA Feature 3 Information on the instruction set attributes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id_isar3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`id_isar3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdIsar3Spec;
impl crate::RegisterSpec for IdIsar3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id_isar3::R`](R) reader structure"]
impl crate::Readable for IdIsar3Spec {}
#[doc = "`write(|w| ..)` method takes [`id_isar3::W`](W) writer structure"]
impl crate::Writable for IdIsar3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ID_ISAR3 to value 0x0111_1131"]
impl crate::Resettable for IdIsar3Spec {
    const RESET_VALUE: u32 = 0x0111_1131;
}
