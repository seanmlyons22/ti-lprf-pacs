#[doc = "Register `AESIV` reader"]
pub type R = crate::R<AesivSpec>;
#[doc = "Register `AESIV` writer"]
pub type W = crate::W<AesivSpec>;
#[doc = "Field `IV` reader - 31:0\\]
The interpretation of this field depends on the crypto operation mode."]
pub type IvR = crate::FieldReader<u32>;
#[doc = "Field `IV` writer - 31:0\\]
The interpretation of this field depends on the crypto operation mode."]
pub type IvW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
The interpretation of this field depends on the crypto operation mode."]
    #[inline(always)]
    pub fn iv(&self) -> IvR {
        IvR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
The interpretation of this field depends on the crypto operation mode."]
    #[inline(always)]
    #[must_use]
    pub fn iv(&mut self) -> IvW<AesivSpec> {
        IvW::new(self, 0)
    }
}
#[doc = "AES Initialization Vector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesiv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesiv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesivSpec;
impl crate::RegisterSpec for AesivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesiv::R`](R) reader structure"]
impl crate::Readable for AesivSpec {}
#[doc = "`write(|w| ..)` method takes [`aesiv::W`](W) writer structure"]
impl crate::Writable for AesivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESIV to value 0"]
impl crate::Resettable for AesivSpec {
    const RESET_VALUE: u32 = 0;
}
