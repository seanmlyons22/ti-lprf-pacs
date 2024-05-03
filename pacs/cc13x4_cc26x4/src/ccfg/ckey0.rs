#[doc = "Register `CKEY0` reader"]
pub type R = crate::R<Ckey0Spec>;
#[doc = "Register `CKEY0` writer"]
pub type W = crate::W<Ckey0Spec>;
#[doc = "Field `KEY` reader - 31:0\\]
Bit\\[31:0\\]
of customer key used for XOR of TI unlock code when CCFG_TI_OPTIONS.C_FA_DIS != 0xC5."]
pub type KeyR = crate::FieldReader<u32>;
#[doc = "Field `KEY` writer - 31:0\\]
Bit\\[31:0\\]
of customer key used for XOR of TI unlock code when CCFG_TI_OPTIONS.C_FA_DIS != 0xC5."]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Bit\\[31:0\\]
of customer key used for XOR of TI unlock code when CCFG_TI_OPTIONS.C_FA_DIS != 0xC5."]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Bit\\[31:0\\]
of customer key used for XOR of TI unlock code when CCFG_TI_OPTIONS.C_FA_DIS != 0xC5."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<Ckey0Spec> {
        KeyW::new(self, 0)
    }
}
#[doc = "Customer key\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ckey0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ckey0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ckey0Spec;
impl crate::RegisterSpec for Ckey0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ckey0::R`](R) reader structure"]
impl crate::Readable for Ckey0Spec {}
#[doc = "`write(|w| ..)` method takes [`ckey0::W`](W) writer structure"]
impl crate::Writable for Ckey0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CKEY0 to value 0x0fff_ffff"]
impl crate::Resettable for Ckey0Spec {
    const RESET_VALUE: u32 = 0x0fff_ffff;
}
