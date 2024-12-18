#[doc = "Register `ICPR0` reader"]
pub type R = crate::R<Icpr0Spec>;
#[doc = "Register `ICPR0` writer"]
pub type W = crate::W<Icpr0Spec>;
#[doc = "Field `CLRPEND` reader - 31:0\\]
For CLRPEND\\[m\\]
in NVIC_ICPR*n, indicates whether interrupt 32*n + m is pending"]
pub type ClrpendR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
For CLRPEND\\[m\\]
in NVIC_ICPR*n, indicates whether interrupt 32*n + m is pending"]
    #[inline(always)]
    pub fn clrpend(&self) -> ClrpendR {
        ClrpendR::new(self.bits)
    }
}
impl W {}
#[doc = "Clears or reads the pending state of each group of 32 interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icpr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icpr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Icpr0Spec;
impl crate::RegisterSpec for Icpr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icpr0::R`](R) reader structure"]
impl crate::Readable for Icpr0Spec {}
#[doc = "`write(|w| ..)` method takes [`icpr0::W`](W) writer structure"]
impl crate::Writable for Icpr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICPR0 to value 0"]
impl crate::Resettable for Icpr0Spec {
    const RESET_VALUE: u32 = 0;
}
