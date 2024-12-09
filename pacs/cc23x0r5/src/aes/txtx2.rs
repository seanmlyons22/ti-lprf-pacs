#[doc = "Register `TXTX2` reader"]
pub type R = crate::R<Txtx2Spec>;
#[doc = "Register `TXTX2` writer"]
pub type W = crate::W<Txtx2Spec>;
#[doc = "Field `VAL` writer - 31:0\\]
Value in TXT2 will be TXT2.VAL = VAL XOR TXT2.VAL"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Value in TXT2 will be TXT2.VAL = VAL XOR TXT2.VAL"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<Txtx2Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "Text Word 2 XOR Write data to this register to XOR data with contents in TXT2.VAL.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txtx2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txtx2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txtx2Spec;
impl crate::RegisterSpec for Txtx2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txtx2::R`](R) reader structure"]
impl crate::Readable for Txtx2Spec {}
#[doc = "`write(|w| ..)` method takes [`txtx2::W`](W) writer structure"]
impl crate::Writable for Txtx2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXTX2 to value 0"]
impl crate::Resettable for Txtx2Spec {
    const RESET_VALUE: u32 = 0;
}
