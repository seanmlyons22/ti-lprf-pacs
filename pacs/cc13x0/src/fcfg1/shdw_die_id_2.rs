#[doc = "Register `SHDW_DIE_ID_2` reader"]
pub type R = crate::R<ShdwDieId2Spec>;
#[doc = "Register `SHDW_DIE_ID_2` writer"]
pub type W = crate::W<ShdwDieId2Spec>;
#[doc = "Field `ID_95_64` reader - 31:0\\]
Shadow of the DIE_ID_2 register in eFuse row number 5"]
pub type Id95_64R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Shadow of the DIE_ID_2 register in eFuse row number 5"]
    #[inline(always)]
    pub fn id_95_64(&self) -> Id95_64R {
        Id95_64R::new(self.bits)
    }
}
impl W {}
#[doc = "Shadow of the DIE_ID_2 register in eFuse\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shdw_die_id_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shdw_die_id_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShdwDieId2Spec;
impl crate::RegisterSpec for ShdwDieId2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shdw_die_id_2::R`](R) reader structure"]
impl crate::Readable for ShdwDieId2Spec {}
#[doc = "`write(|w| ..)` method takes [`shdw_die_id_2::W`](W) writer structure"]
impl crate::Writable for ShdwDieId2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHDW_DIE_ID_2 to value 0"]
impl crate::Resettable for ShdwDieId2Spec {
    const RESET_VALUE: u32 = 0;
}
