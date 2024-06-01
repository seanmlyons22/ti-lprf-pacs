#[doc = "Register `ERROR` reader"]
pub type R = crate::R<ErrorSpec>;
#[doc = "Register `ERROR` writer"]
pub type W = crate::W<ErrorSpec>;
#[doc = "Field `STATUS` reader - 0:0\\]
Returns the status of bus error flag in uDMA, or clears this bit Read as: 0: No bus error detected 1: Bus error detected Write as: 0: No effect, status of bus error flag is unchanged. 1: Clears the bus error flag."]
pub type StatusR = crate::BitReader;
#[doc = "Field `STATUS` writer - 0:0\\]
Returns the status of bus error flag in uDMA, or clears this bit Read as: 0: No bus error detected 1: Bus error detected Write as: 0: No effect, status of bus error flag is unchanged. 1: Clears the bus error flag."]
pub type StatusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Returns the status of bus error flag in uDMA, or clears this bit Read as: 0: No bus error detected 1: Bus error detected Write as: 0: No effect, status of bus error flag is unchanged. 1: Clears the bus error flag."]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Returns the status of bus error flag in uDMA, or clears this bit Read as: 0: No bus error detected 1: Bus error detected Write as: 0: No effect, status of bus error flag is unchanged. 1: Clears the bus error flag."]
    #[inline(always)]
    #[must_use]
    pub fn status(&mut self) -> StatusW<ErrorSpec> {
        StatusW::new(self, 0)
    }
}
#[doc = "Error Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`error::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`error::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrorSpec;
impl crate::RegisterSpec for ErrorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`error::R`](R) reader structure"]
impl crate::Readable for ErrorSpec {}
#[doc = "`write(|w| ..)` method takes [`error::W`](W) writer structure"]
impl crate::Writable for ErrorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERROR to value 0"]
impl crate::Resettable for ErrorSpec {
    const RESET_VALUE: u32 = 0;
}
