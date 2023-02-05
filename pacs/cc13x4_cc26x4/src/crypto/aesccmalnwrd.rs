#[doc = "Register `AESCCMALNWRD` reader"]
pub struct R(crate::R<AESCCMALNWRD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AESCCMALNWRD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AESCCMALNWRD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AESCCMALNWRD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AESCCMALNWRD` writer"]
pub struct W(crate::W<AESCCMALNWRD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AESCCMALNWRD_SPEC>;
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
impl From<crate::W<AESCCMALNWRD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AESCCMALNWRD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AES_CCM_ALN_WRD` reader - 31:0\\]
This register provides access to an internal location where the AAD align data is stored for AES-CCM operation. Reading this register provides the current AAD alignment data word that is stored internally, and is used internally to concatenate to the next AAD block. Write this register to restore the AAD alignment data word for a resumed AES-CCM operation Note: This register should only be used for continued processing with AES-CCM. Do not write this register for other processing modes"]
pub type AES_CCM_ALN_WRD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `AES_CCM_ALN_WRD` writer - 31:0\\]
This register provides access to an internal location where the AAD align data is stored for AES-CCM operation. Reading this register provides the current AAD alignment data word that is stored internally, and is used internally to concatenate to the next AAD block. Write this register to restore the AAD alignment data word for a resumed AES-CCM operation Note: This register should only be used for continued processing with AES-CCM. Do not write this register for other processing modes"]
pub type AES_CCM_ALN_WRD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AESCCMALNWRD_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register provides access to an internal location where the AAD align data is stored for AES-CCM operation. Reading this register provides the current AAD alignment data word that is stored internally, and is used internally to concatenate to the next AAD block. Write this register to restore the AAD alignment data word for a resumed AES-CCM operation Note: This register should only be used for continued processing with AES-CCM. Do not write this register for other processing modes"]
    #[inline(always)]
    pub fn aes_ccm_aln_wrd(&self) -> AES_CCM_ALN_WRD_R {
        AES_CCM_ALN_WRD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register provides access to an internal location where the AAD align data is stored for AES-CCM operation. Reading this register provides the current AAD alignment data word that is stored internally, and is used internally to concatenate to the next AAD block. Write this register to restore the AAD alignment data word for a resumed AES-CCM operation Note: This register should only be used for continued processing with AES-CCM. Do not write this register for other processing modes"]
    #[inline(always)]
    #[must_use]
    pub fn aes_ccm_aln_wrd(&mut self) -> AES_CCM_ALN_WRD_W<0> {
        AES_CCM_ALN_WRD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register needs to be read and stored when an AES-CCM operation is interrupted. This value needs to be restored by writing this register, when resuming that AES-CCM operation in a later session\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesccmalnwrd](index.html) module"]
pub struct AESCCMALNWRD_SPEC;
impl crate::RegisterSpec for AESCCMALNWRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aesccmalnwrd::R](R) reader structure"]
impl crate::Readable for AESCCMALNWRD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aesccmalnwrd::W](W) writer structure"]
impl crate::Writable for AESCCMALNWRD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AESCCMALNWRD to value 0"]
impl crate::Resettable for AESCCMALNWRD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
