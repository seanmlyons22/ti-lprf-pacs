#[doc = "Register `SHDW_DIE_ID_1` reader"]
pub type R = crate::R<ShdwDieId1Spec>;
#[doc = "Register `SHDW_DIE_ID_1` writer"]
pub type W = crate::W<ShdwDieId1Spec>;
#[doc = "Field `ID_63_32` reader - 31:0\\]
Shadow of the DIE_ID_1 register in eFuse row number 4"]
pub type Id63_32R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Shadow of the DIE_ID_1 register in eFuse row number 4"]
    #[inline(always)]
    pub fn id_63_32(&self) -> Id63_32R {
        Id63_32R::new(self.bits)
    }
}
impl W {}
#[doc = "Shadow of the DIE_ID_1 register in eFuse\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shdw_die_id_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shdw_die_id_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShdwDieId1Spec;
impl crate::RegisterSpec for ShdwDieId1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shdw_die_id_1::R`](R) reader structure"]
impl crate::Readable for ShdwDieId1Spec {}
#[doc = "`write(|w| ..)` method takes [`shdw_die_id_1::W`](W) writer structure"]
impl crate::Writable for ShdwDieId1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHDW_DIE_ID_1 to value 0"]
impl crate::Resettable for ShdwDieId1Spec {
    const RESET_VALUE: u32 = 0;
}
