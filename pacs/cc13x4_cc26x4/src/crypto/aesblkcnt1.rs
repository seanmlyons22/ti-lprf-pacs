#[doc = "Register `AESBLKCNT1` reader"]
pub type R = crate::R<Aesblkcnt1Spec>;
#[doc = "Register `AESBLKCNT1` writer"]
pub type W = crate::W<Aesblkcnt1Spec>;
#[doc = "Field `AES_BLK_CNT_56_32` reader - 24:0\\]
\\[56:32\\]
of Internal block counter for AES GCM and CCM operations. These bits read the block count value that represents the number of blocks to go. This value is valid with saved_context_ready after a request for an intermediate GCM/CCM digest. Writing these registers will restore the internal block counter to the programmed value. This only needs to be done to prepare the engine to continue processing of an interrupted GCM or CCM operation. Also refer to the get_digest and gcm_ccm_continue bits in AES_CTRL register."]
pub type AesBlkCnt56_32R = crate::FieldReader<u32>;
#[doc = "Field `AES_BLK_CNT_56_32` writer - 24:0\\]
\\[56:32\\]
of Internal block counter for AES GCM and CCM operations. These bits read the block count value that represents the number of blocks to go. This value is valid with saved_context_ready after a request for an intermediate GCM/CCM digest. Writing these registers will restore the internal block counter to the programmed value. This only needs to be done to prepare the engine to continue processing of an interrupted GCM or CCM operation. Also refer to the get_digest and gcm_ccm_continue bits in AES_CTRL register."]
pub type AesBlkCnt56_32W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
#[doc = "Field `RESERVED25` reader - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved25R = crate::FieldReader;
#[doc = "Field `RESERVED25` writer - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved25W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:24 - 24:0\\]
\\[56:32\\]
of Internal block counter for AES GCM and CCM operations. These bits read the block count value that represents the number of blocks to go. This value is valid with saved_context_ready after a request for an intermediate GCM/CCM digest. Writing these registers will restore the internal block counter to the programmed value. This only needs to be done to prepare the engine to continue processing of an interrupted GCM or CCM operation. Also refer to the get_digest and gcm_ccm_continue bits in AES_CTRL register."]
    #[inline(always)]
    pub fn aes_blk_cnt_56_32(&self) -> AesBlkCnt56_32R {
        AesBlkCnt56_32R::new(self.bits & 0x01ff_ffff)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved25(&self) -> Reserved25R {
        Reserved25R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:24 - 24:0\\]
\\[56:32\\]
of Internal block counter for AES GCM and CCM operations. These bits read the block count value that represents the number of blocks to go. This value is valid with saved_context_ready after a request for an intermediate GCM/CCM digest. Writing these registers will restore the internal block counter to the programmed value. This only needs to be done to prepare the engine to continue processing of an interrupted GCM or CCM operation. Also refer to the get_digest and gcm_ccm_continue bits in AES_CTRL register."]
    #[inline(always)]
    #[must_use]
    pub fn aes_blk_cnt_56_32(&mut self) -> AesBlkCnt56_32W<Aesblkcnt1Spec> {
        AesBlkCnt56_32W::new(self, 0)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved25(&mut self) -> Reserved25W<Aesblkcnt1Spec> {
        Reserved25W::new(self, 25)
    }
}
#[doc = "This counter keeps track of the number of data blocks during AES-CCM and AES-GCM operations. Reading and writing this counter allows interruption and resuming of long CCM or GCM operations. Note that internally, the block counter is used for AAD data as well as encryption/decryption data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesblkcnt1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesblkcnt1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aesblkcnt1Spec;
impl crate::RegisterSpec for Aesblkcnt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesblkcnt1::R`](R) reader structure"]
impl crate::Readable for Aesblkcnt1Spec {}
#[doc = "`write(|w| ..)` method takes [`aesblkcnt1::W`](W) writer structure"]
impl crate::Writable for Aesblkcnt1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESBLKCNT1 to value 0"]
impl crate::Resettable for Aesblkcnt1Spec {
    const RESET_VALUE: u32 = 0;
}
