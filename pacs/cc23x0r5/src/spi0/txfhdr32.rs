#[doc = "Register `TXFHDR32` reader"]
pub type R = crate::R<Txfhdr32Spec>;
#[doc = "Register `TXFHDR32` writer"]
pub type W = crate::W<Txfhdr32Spec>;
#[doc = "Field `DATA` writer - 31:0\\]
This field can be used to write four bytes of header data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This field can be used to write four bytes of header data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<Txfhdr32Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Header update reigster for 32 bits of header data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfhdr32::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txfhdr32::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txfhdr32Spec;
impl crate::RegisterSpec for Txfhdr32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txfhdr32::R`](R) reader structure"]
impl crate::Readable for Txfhdr32Spec {}
#[doc = "`write(|w| ..)` method takes [`txfhdr32::W`](W) writer structure"]
impl crate::Writable for Txfhdr32Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXFHDR32 to value 0"]
impl crate::Resettable for Txfhdr32Spec {
    const RESET_VALUE: u32 = 0;
}
