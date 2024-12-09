#[doc = "Register `TXTX1` reader"]
pub type R = crate::R<Txtx1Spec>;
#[doc = "Register `TXTX1` writer"]
pub type W = crate::W<Txtx1Spec>;
#[doc = "Field `VAL` writer - 31:0\\]
Value in TXT1 will be TXT1.VAL = VAL XOR TXT1.VAL"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Value in TXT1 will be TXT1.VAL = VAL XOR TXT1.VAL"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<Txtx1Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "Text Word 1 XOR Write data to this register to XOR data with contents in TXT1.VAL.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txtx1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txtx1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txtx1Spec;
impl crate::RegisterSpec for Txtx1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txtx1::R`](R) reader structure"]
impl crate::Readable for Txtx1Spec {}
#[doc = "`write(|w| ..)` method takes [`txtx1::W`](W) writer structure"]
impl crate::Writable for Txtx1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXTX1 to value 0"]
impl crate::Resettable for Txtx1Spec {
    const RESET_VALUE: u32 = 0;
}
