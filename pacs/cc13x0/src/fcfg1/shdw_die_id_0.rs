#[doc = "Register `SHDW_DIE_ID_0` reader"]
pub type R = crate::R<ShdwDieId0Spec>;
#[doc = "Register `SHDW_DIE_ID_0` writer"]
pub type W = crate::W<ShdwDieId0Spec>;
#[doc = "Field `ID_31_0` reader - 31:0\\]
Shadow of the DIE_ID_0 register in eFuse row number 3"]
pub type Id31_0R = crate::FieldReader<u32>;
#[doc = "Field `ID_31_0` writer - 31:0\\]
Shadow of the DIE_ID_0 register in eFuse row number 3"]
pub type Id31_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Shadow of the DIE_ID_0 register in eFuse row number 3"]
    #[inline(always)]
    pub fn id_31_0(&self) -> Id31_0R {
        Id31_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Shadow of the DIE_ID_0 register in eFuse row number 3"]
    #[inline(always)]
    #[must_use]
    pub fn id_31_0(&mut self) -> Id31_0W<ShdwDieId0Spec> {
        Id31_0W::new(self, 0)
    }
}
#[doc = "Shadow of the DIE_ID_0 register in eFuse\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shdw_die_id_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shdw_die_id_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShdwDieId0Spec;
impl crate::RegisterSpec for ShdwDieId0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shdw_die_id_0::R`](R) reader structure"]
impl crate::Readable for ShdwDieId0Spec {}
#[doc = "`write(|w| ..)` method takes [`shdw_die_id_0::W`](W) writer structure"]
impl crate::Writable for ShdwDieId0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHDW_DIE_ID_0 to value 0"]
impl crate::Resettable for ShdwDieId0Spec {
    const RESET_VALUE: u32 = 0;
}
