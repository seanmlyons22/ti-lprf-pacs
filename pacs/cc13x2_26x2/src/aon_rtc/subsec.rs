#[doc = "Register `SUBSEC` reader"]
pub type R = crate::R<SubsecSpec>;
#[doc = "Register `SUBSEC` writer"]
pub type W = crate::W<SubsecSpec>;
#[doc = "Field `VALUE` reader - 31:0\\]
Unsigned integer representing Real Time Clock in fractions of a second (VALUE/2^32 seconds) at the time when SEC register was read. Examples : - 0x0000_0000 = 0.0 sec - 0x4000_0000 = 0.25 sec - 0x8000_0000 = 0.5 sec - 0xC000_0000 = 0.75 sec"]
pub type ValueR = crate::FieldReader<u32>;
#[doc = "Field `VALUE` writer - 31:0\\]
Unsigned integer representing Real Time Clock in fractions of a second (VALUE/2^32 seconds) at the time when SEC register was read. Examples : - 0x0000_0000 = 0.0 sec - 0x4000_0000 = 0.25 sec - 0x8000_0000 = 0.5 sec - 0xC000_0000 = 0.75 sec"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Unsigned integer representing Real Time Clock in fractions of a second (VALUE/2^32 seconds) at the time when SEC register was read. Examples : - 0x0000_0000 = 0.0 sec - 0x4000_0000 = 0.25 sec - 0x8000_0000 = 0.5 sec - 0xC000_0000 = 0.75 sec"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Unsigned integer representing Real Time Clock in fractions of a second (VALUE/2^32 seconds) at the time when SEC register was read. Examples : - 0x0000_0000 = 0.0 sec - 0x4000_0000 = 0.25 sec - 0x8000_0000 = 0.5 sec - 0xC000_0000 = 0.75 sec"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> ValueW<SubsecSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Second Counter Value, Fractional Part\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`subsec::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`subsec::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SubsecSpec;
impl crate::RegisterSpec for SubsecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`subsec::R`](R) reader structure"]
impl crate::Readable for SubsecSpec {}
#[doc = "`write(|w| ..)` method takes [`subsec::W`](W) writer structure"]
impl crate::Writable for SubsecSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SUBSEC to value 0"]
impl crate::Resettable for SubsecSpec {
    const RESET_VALUE: u32 = 0;
}
