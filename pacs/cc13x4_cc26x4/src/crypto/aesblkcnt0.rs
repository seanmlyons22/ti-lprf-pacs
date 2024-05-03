#[doc = "Register `AESBLKCNT0` reader"]
pub type R = crate::R<Aesblkcnt0Spec>;
#[doc = "Register `AESBLKCNT0` writer"]
pub type W = crate::W<Aesblkcnt0Spec>;
#[doc = "Field `AES_BLK_CNT_31_0` reader - 31:0\\]
\\[31:0\\]
of Internal block counter for AES GCM and CCM operations. These bits read the block count value that represents the number of blocks to go. This value is valid with saved_context_ready after a request for an intermediate GCM/CCM digest. Writing these registers will restore the internal block counter to the programmed value. This only needs to be done to prepare the engine to continue processing of an interrupted GCM or CCM operation. Also refer to the get_digest and gcm_ccm_continue bits in AES_CTRL register."]
pub type AesBlkCnt31_0R = crate::FieldReader<u32>;
#[doc = "Field `AES_BLK_CNT_31_0` writer - 31:0\\]
\\[31:0\\]
of Internal block counter for AES GCM and CCM operations. These bits read the block count value that represents the number of blocks to go. This value is valid with saved_context_ready after a request for an intermediate GCM/CCM digest. Writing these registers will restore the internal block counter to the programmed value. This only needs to be done to prepare the engine to continue processing of an interrupted GCM or CCM operation. Also refer to the get_digest and gcm_ccm_continue bits in AES_CTRL register."]
pub type AesBlkCnt31_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
\\[31:0\\]
of Internal block counter for AES GCM and CCM operations. These bits read the block count value that represents the number of blocks to go. This value is valid with saved_context_ready after a request for an intermediate GCM/CCM digest. Writing these registers will restore the internal block counter to the programmed value. This only needs to be done to prepare the engine to continue processing of an interrupted GCM or CCM operation. Also refer to the get_digest and gcm_ccm_continue bits in AES_CTRL register."]
    #[inline(always)]
    pub fn aes_blk_cnt_31_0(&self) -> AesBlkCnt31_0R {
        AesBlkCnt31_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
\\[31:0\\]
of Internal block counter for AES GCM and CCM operations. These bits read the block count value that represents the number of blocks to go. This value is valid with saved_context_ready after a request for an intermediate GCM/CCM digest. Writing these registers will restore the internal block counter to the programmed value. This only needs to be done to prepare the engine to continue processing of an interrupted GCM or CCM operation. Also refer to the get_digest and gcm_ccm_continue bits in AES_CTRL register."]
    #[inline(always)]
    #[must_use]
    pub fn aes_blk_cnt_31_0(&mut self) -> AesBlkCnt31_0W<Aesblkcnt0Spec> {
        AesBlkCnt31_0W::new(self, 0)
    }
}
#[doc = "This counter keeps track of the number of data blocks during AES-CCM and AES-GCM operations. Reading and writing this counter allows interruption and resuming of long CCM or GCM operations. Note that internally, the block counter is used for AAD data as well as encryption/decryption data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesblkcnt0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesblkcnt0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aesblkcnt0Spec;
impl crate::RegisterSpec for Aesblkcnt0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesblkcnt0::R`](R) reader structure"]
impl crate::Readable for Aesblkcnt0Spec {}
#[doc = "`write(|w| ..)` method takes [`aesblkcnt0::W`](W) writer structure"]
impl crate::Writable for Aesblkcnt0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESBLKCNT0 to value 0"]
impl crate::Resettable for Aesblkcnt0Spec {
    const RESET_VALUE: u32 = 0;
}
