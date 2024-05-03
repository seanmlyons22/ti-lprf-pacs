#[doc = "Register `AESCCMALNWRD` reader"]
pub type R = crate::R<AesccmalnwrdSpec>;
#[doc = "Register `AESCCMALNWRD` writer"]
pub type W = crate::W<AesccmalnwrdSpec>;
#[doc = "Field `AES_CCM_ALN_WRD` reader - 31:0\\]
This register provides access to an internal location where the AAD align data is stored for AES-CCM operation. Reading this register provides the current AAD alignment data word that is stored internally, and is used internally to concatenate to the next AAD block. Write this register to restore the AAD alignment data word for a resumed AES-CCM operation Note: This register should only be used for continued processing with AES-CCM. Do not write this register for other processing modes"]
pub type AesCcmAlnWrdR = crate::FieldReader<u32>;
#[doc = "Field `AES_CCM_ALN_WRD` writer - 31:0\\]
This register provides access to an internal location where the AAD align data is stored for AES-CCM operation. Reading this register provides the current AAD alignment data word that is stored internally, and is used internally to concatenate to the next AAD block. Write this register to restore the AAD alignment data word for a resumed AES-CCM operation Note: This register should only be used for continued processing with AES-CCM. Do not write this register for other processing modes"]
pub type AesCcmAlnWrdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register provides access to an internal location where the AAD align data is stored for AES-CCM operation. Reading this register provides the current AAD alignment data word that is stored internally, and is used internally to concatenate to the next AAD block. Write this register to restore the AAD alignment data word for a resumed AES-CCM operation Note: This register should only be used for continued processing with AES-CCM. Do not write this register for other processing modes"]
    #[inline(always)]
    pub fn aes_ccm_aln_wrd(&self) -> AesCcmAlnWrdR {
        AesCcmAlnWrdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register provides access to an internal location where the AAD align data is stored for AES-CCM operation. Reading this register provides the current AAD alignment data word that is stored internally, and is used internally to concatenate to the next AAD block. Write this register to restore the AAD alignment data word for a resumed AES-CCM operation Note: This register should only be used for continued processing with AES-CCM. Do not write this register for other processing modes"]
    #[inline(always)]
    #[must_use]
    pub fn aes_ccm_aln_wrd(&mut self) -> AesCcmAlnWrdW<AesccmalnwrdSpec> {
        AesCcmAlnWrdW::new(self, 0)
    }
}
#[doc = "This register needs to be read and stored when an AES-CCM operation is interrupted. This value needs to be restored by writing this register, when resuming that AES-CCM operation in a later session\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesccmalnwrd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesccmalnwrd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesccmalnwrdSpec;
impl crate::RegisterSpec for AesccmalnwrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesccmalnwrd::R`](R) reader structure"]
impl crate::Readable for AesccmalnwrdSpec {}
#[doc = "`write(|w| ..)` method takes [`aesccmalnwrd::W`](W) writer structure"]
impl crate::Writable for AesccmalnwrdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESCCMALNWRD to value 0"]
impl crate::Resettable for AesccmalnwrdSpec {
    const RESET_VALUE: u32 = 0;
}
