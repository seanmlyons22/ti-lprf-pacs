#[doc = "Register `TXFHDR24` reader"]
pub type R = crate::R<Txfhdr24Spec>;
#[doc = "Register `TXFHDR24` writer"]
pub type W = crate::W<Txfhdr24Spec>;
#[doc = "Field `DATA` writer - 31:0\\]
This field can be used to write three bytes of header data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This field can be used to write three bytes of header data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<Txfhdr24Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Header update reigster for 24 bits of header data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfhdr24::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txfhdr24::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txfhdr24Spec;
impl crate::RegisterSpec for Txfhdr24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txfhdr24::R`](R) reader structure"]
impl crate::Readable for Txfhdr24Spec {}
#[doc = "`write(|w| ..)` method takes [`txfhdr24::W`](W) writer structure"]
impl crate::Writable for Txfhdr24Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXFHDR24 to value 0"]
impl crate::Resettable for Txfhdr24Spec {
    const RESET_VALUE: u32 = 0;
}
