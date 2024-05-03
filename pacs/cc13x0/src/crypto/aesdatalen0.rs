#[doc = "Register `AESDATALEN0` reader"]
pub type R = crate::R<Aesdatalen0Spec>;
#[doc = "Register `AESDATALEN0` writer"]
pub type W = crate::W<Aesdatalen0Spec>;
#[doc = "Field `LEN_LSW` reader - 31:0\\]
Used to write the Length values to the Crypto peripheral. This register contains bits \\[31:0\\]
of the combined data length."]
pub type LenLswR = crate::FieldReader<u32>;
#[doc = "Field `LEN_LSW` writer - 31:0\\]
Used to write the Length values to the Crypto peripheral. This register contains bits \\[31:0\\]
of the combined data length."]
pub type LenLswW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Used to write the Length values to the Crypto peripheral. This register contains bits \\[31:0\\]
of the combined data length."]
    #[inline(always)]
    pub fn len_lsw(&self) -> LenLswR {
        LenLswR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Used to write the Length values to the Crypto peripheral. This register contains bits \\[31:0\\]
of the combined data length."]
    #[inline(always)]
    #[must_use]
    pub fn len_lsw(&mut self) -> LenLswW<Aesdatalen0Spec> {
        LenLswW::new(self, 0)
    }
}
#[doc = "Crypto Data Length LSW\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesdatalen0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesdatalen0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aesdatalen0Spec;
impl crate::RegisterSpec for Aesdatalen0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesdatalen0::R`](R) reader structure"]
impl crate::Readable for Aesdatalen0Spec {}
#[doc = "`write(|w| ..)` method takes [`aesdatalen0::W`](W) writer structure"]
impl crate::Writable for Aesdatalen0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESDATALEN0 to value 0"]
impl crate::Resettable for Aesdatalen0Spec {
    const RESET_VALUE: u32 = 0;
}
