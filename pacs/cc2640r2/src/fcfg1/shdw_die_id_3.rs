#[doc = "Register `SHDW_DIE_ID_3` reader"]
pub type R = crate::R<ShdwDieId3Spec>;
#[doc = "Register `SHDW_DIE_ID_3` writer"]
pub type W = crate::W<ShdwDieId3Spec>;
#[doc = "Field `ID_127_96` reader - 31:0\\]
Shadow of DIE_ID_3 register in eFuse row number 6"]
pub type Id127_96R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Shadow of DIE_ID_3 register in eFuse row number 6"]
    #[inline(always)]
    pub fn id_127_96(&self) -> Id127_96R {
        Id127_96R::new(self.bits)
    }
}
impl W {}
#[doc = "Shadow of DIE_ID_3 register in eFuse\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shdw_die_id_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shdw_die_id_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShdwDieId3Spec;
impl crate::RegisterSpec for ShdwDieId3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shdw_die_id_3::R`](R) reader structure"]
impl crate::Readable for ShdwDieId3Spec {}
#[doc = "`write(|w| ..)` method takes [`shdw_die_id_3::W`](W) writer structure"]
impl crate::Writable for ShdwDieId3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHDW_DIE_ID_3 to value 0"]
impl crate::Resettable for ShdwDieId3Spec {
    const RESET_VALUE: u32 = 0;
}
