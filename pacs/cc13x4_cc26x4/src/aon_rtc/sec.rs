#[doc = "Register `SEC` reader"]
pub type R = crate::R<SecSpec>;
#[doc = "Register `SEC` writer"]
pub type W = crate::W<SecSpec>;
#[doc = "Field `VALUE` reader - 31:0\\]
Unsigned integer representing Real Time Clock in seconds. When reading this register the content of SUBSEC.VALUE is simultaneously latched. A consistent reading of the combined Real Time Clock can be obtained by first reading this register, then reading SUBSEC register."]
pub type ValueR = crate::FieldReader<u32>;
#[doc = "Field `VALUE` writer - 31:0\\]
Unsigned integer representing Real Time Clock in seconds. When reading this register the content of SUBSEC.VALUE is simultaneously latched. A consistent reading of the combined Real Time Clock can be obtained by first reading this register, then reading SUBSEC register."]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Unsigned integer representing Real Time Clock in seconds. When reading this register the content of SUBSEC.VALUE is simultaneously latched. A consistent reading of the combined Real Time Clock can be obtained by first reading this register, then reading SUBSEC register."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Unsigned integer representing Real Time Clock in seconds. When reading this register the content of SUBSEC.VALUE is simultaneously latched. A consistent reading of the combined Real Time Clock can be obtained by first reading this register, then reading SUBSEC register."]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> ValueW<SecSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Second Counter Value, Integer Part\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sec::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sec::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecSpec;
impl crate::RegisterSpec for SecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec::R`](R) reader structure"]
impl crate::Readable for SecSpec {}
#[doc = "`write(|w| ..)` method takes [`sec::W`](W) writer structure"]
impl crate::Writable for SecSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEC to value 0"]
impl crate::Resettable for SecSpec {
    const RESET_VALUE: u32 = 0;
}
