#[doc = "Register `TXTX3` reader"]
pub type R = crate::R<Txtx3Spec>;
#[doc = "Register `TXTX3` writer"]
pub type W = crate::W<Txtx3Spec>;
#[doc = "Field `VAL` writer - 31:0\\]
Value in TXT3 will be TXT3.VAL = VAL XOR TXT3.VAL"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Value in TXT3 will be TXT3.VAL = VAL XOR TXT3.VAL"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<Txtx3Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "Text Word 3 XOR Write data to this register to XOR data with contents in TXT3.VAL. AUTOCFG.TRGECB decides if a write to or a read of this field triggers an encryption.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txtx3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txtx3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txtx3Spec;
impl crate::RegisterSpec for Txtx3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txtx3::R`](R) reader structure"]
impl crate::Readable for Txtx3Spec {}
#[doc = "`write(|w| ..)` method takes [`txtx3::W`](W) writer structure"]
impl crate::Writable for Txtx3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXTX3 to value 0"]
impl crate::Resettable for Txtx3Spec {
    const RESET_VALUE: u32 = 0;
}
