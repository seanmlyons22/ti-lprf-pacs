#[doc = "Register `CH1CMP` reader"]
pub type R = crate::R<Ch1cmpSpec>;
#[doc = "Register `CH1CMP` writer"]
pub type W = crate::W<Ch1cmpSpec>;
#[doc = "Field `VALUE` reader - 31:0\\]
RTC Channel 1 compare value. Bit 31 to 16 represents seconds and bits 15 to 0 represents subseconds of the compare value. The compare value is compared against SEC.VALUE (15:0) and SUBSEC.VALUE (31:16) values of the Real Time Clock register. A Cannel 0 event is generated when {SEC.VALUE(15:0),SUBSEC.VALUE (31:16)} is reaching or exciting the compare value. Writing to this register can trigger an immediate*) event in case the new compare value matches a Real Time Clock value from 1 second in the past up till current Real Time Clock value. Example: To generate a compare 5.5 seconds RTC start,- set this value = 0x0005_8000 *) It can take up to 2 SCLK_LF clock cycles before event occurs due to synchronization."]
pub type ValueR = crate::FieldReader<u32>;
#[doc = "Field `VALUE` writer - 31:0\\]
RTC Channel 1 compare value. Bit 31 to 16 represents seconds and bits 15 to 0 represents subseconds of the compare value. The compare value is compared against SEC.VALUE (15:0) and SUBSEC.VALUE (31:16) values of the Real Time Clock register. A Cannel 0 event is generated when {SEC.VALUE(15:0),SUBSEC.VALUE (31:16)} is reaching or exciting the compare value. Writing to this register can trigger an immediate*) event in case the new compare value matches a Real Time Clock value from 1 second in the past up till current Real Time Clock value. Example: To generate a compare 5.5 seconds RTC start,- set this value = 0x0005_8000 *) It can take up to 2 SCLK_LF clock cycles before event occurs due to synchronization."]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
RTC Channel 1 compare value. Bit 31 to 16 represents seconds and bits 15 to 0 represents subseconds of the compare value. The compare value is compared against SEC.VALUE (15:0) and SUBSEC.VALUE (31:16) values of the Real Time Clock register. A Cannel 0 event is generated when {SEC.VALUE(15:0),SUBSEC.VALUE (31:16)} is reaching or exciting the compare value. Writing to this register can trigger an immediate*) event in case the new compare value matches a Real Time Clock value from 1 second in the past up till current Real Time Clock value. Example: To generate a compare 5.5 seconds RTC start,- set this value = 0x0005_8000 *) It can take up to 2 SCLK_LF clock cycles before event occurs due to synchronization."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
RTC Channel 1 compare value. Bit 31 to 16 represents seconds and bits 15 to 0 represents subseconds of the compare value. The compare value is compared against SEC.VALUE (15:0) and SUBSEC.VALUE (31:16) values of the Real Time Clock register. A Cannel 0 event is generated when {SEC.VALUE(15:0),SUBSEC.VALUE (31:16)} is reaching or exciting the compare value. Writing to this register can trigger an immediate*) event in case the new compare value matches a Real Time Clock value from 1 second in the past up till current Real Time Clock value. Example: To generate a compare 5.5 seconds RTC start,- set this value = 0x0005_8000 *) It can take up to 2 SCLK_LF clock cycles before event occurs due to synchronization."]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> ValueW<Ch1cmpSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Channel 1 Compare Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1cmp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1cmp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch1cmpSpec;
impl crate::RegisterSpec for Ch1cmpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1cmp::R`](R) reader structure"]
impl crate::Readable for Ch1cmpSpec {}
#[doc = "`write(|w| ..)` method takes [`ch1cmp::W`](W) writer structure"]
impl crate::Writable for Ch1cmpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH1CMP to value 0"]
impl crate::Resettable for Ch1cmpSpec {
    const RESET_VALUE: u32 = 0;
}
