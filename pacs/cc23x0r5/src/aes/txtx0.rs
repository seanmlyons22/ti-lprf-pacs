#[doc = "Register `TXTX0` reader"]
pub type R = crate::R<Txtx0Spec>;
#[doc = "Register `TXTX0` writer"]
pub type W = crate::W<Txtx0Spec>;
#[doc = "Field `VAL` reader - 31:0\\]
Value in TXT0 will be TXT0.VAL = VAL XOR TXT0.VAL"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - 31:0\\]
Value in TXT0 will be TXT0.VAL = VAL XOR TXT0.VAL"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Value in TXT0 will be TXT0.VAL = VAL XOR TXT0.VAL"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Value in TXT0 will be TXT0.VAL = VAL XOR TXT0.VAL"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<Txtx0Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "Text Word 0 XOR Write data to this register to XOR data with contents in TXT0.VAL.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txtx0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txtx0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txtx0Spec;
impl crate::RegisterSpec for Txtx0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txtx0::R`](R) reader structure"]
impl crate::Readable for Txtx0Spec {}
#[doc = "`write(|w| ..)` method takes [`txtx0::W`](W) writer structure"]
impl crate::Writable for Txtx0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXTX0 to value 0"]
impl crate::Resettable for Txtx0Spec {
    const RESET_VALUE: u32 = 0;
}
