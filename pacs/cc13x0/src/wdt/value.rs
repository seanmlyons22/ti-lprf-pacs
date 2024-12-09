#[doc = "Register `VALUE` reader"]
pub type R = crate::R<ValueSpec>;
#[doc = "Register `VALUE` writer"]
pub type W = crate::W<ValueSpec>;
#[doc = "Field `WDTVALUE` reader - 31:0\\]
This register contains the current count value of the timer."]
pub type WdtvalueR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register contains the current count value of the timer."]
    #[inline(always)]
    pub fn wdtvalue(&self) -> WdtvalueR {
        WdtvalueR::new(self.bits)
    }
}
impl W {}
#[doc = "Current Count Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`value::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`value::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ValueSpec;
impl crate::RegisterSpec for ValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`value::R`](R) reader structure"]
impl crate::Readable for ValueSpec {}
#[doc = "`write(|w| ..)` method takes [`value::W`](W) writer structure"]
impl crate::Writable for ValueSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VALUE to value 0xffff_ffff"]
impl crate::Resettable for ValueSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
