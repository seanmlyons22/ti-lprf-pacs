#[doc = "Register `SHDW_DIE_ID_3` reader"]
pub struct R(crate::R<SHDW_DIE_ID_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHDW_DIE_ID_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHDW_DIE_ID_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHDW_DIE_ID_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHDW_DIE_ID_3` writer"]
pub struct W(crate::W<SHDW_DIE_ID_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHDW_DIE_ID_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SHDW_DIE_ID_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHDW_DIE_ID_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ID_127_96` reader - 31:0\\]
Shadow of DIE_ID_3 register in eFuse row number 8"]
pub type ID_127_96_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ID_127_96` writer - 31:0\\]
Shadow of DIE_ID_3 register in eFuse row number 8"]
pub type ID_127_96_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SHDW_DIE_ID_3_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Shadow of DIE_ID_3 register in eFuse row number 8"]
    #[inline(always)]
    pub fn id_127_96(&self) -> ID_127_96_R {
        ID_127_96_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Shadow of DIE_ID_3 register in eFuse row number 8"]
    #[inline(always)]
    #[must_use]
    pub fn id_127_96(&mut self) -> ID_127_96_W<0> {
        ID_127_96_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shadow of DIE_ID_3 register in eFuse\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shdw_die_id_3](index.html) module"]
pub struct SHDW_DIE_ID_3_SPEC;
impl crate::RegisterSpec for SHDW_DIE_ID_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shdw_die_id_3::R](R) reader structure"]
impl crate::Readable for SHDW_DIE_ID_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shdw_die_id_3::W](W) writer structure"]
impl crate::Writable for SHDW_DIE_ID_3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHDW_DIE_ID_3 to value 0"]
impl crate::Resettable for SHDW_DIE_ID_3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
