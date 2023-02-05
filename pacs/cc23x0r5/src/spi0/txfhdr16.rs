#[doc = "Register `TXFHDR16` reader"]
pub type R = crate::R<Txfhdr16Spec>;
#[doc = "Register `TXFHDR16` writer"]
pub type W = crate::W<Txfhdr16Spec>;
#[doc = "Field `DATA` writer - 31:0\\]
This field can be used to write two bytes of header data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This field can be used to write two bytes of header data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<Txfhdr16Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Header update reigster for 16 bits of data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfhdr16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txfhdr16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txfhdr16Spec;
impl crate::RegisterSpec for Txfhdr16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txfhdr16::R`](R) reader structure"]
impl crate::Readable for Txfhdr16Spec {}
#[doc = "`write(|w| ..)` method takes [`txfhdr16::W`](W) writer structure"]
impl crate::Writable for Txfhdr16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXFHDR16 to value 0"]
impl crate::Resettable for Txfhdr16Spec {
    const RESET_VALUE: u32 = 0;
}
