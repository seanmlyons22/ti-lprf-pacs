#[doc = "Register `MOSICTL` reader"]
pub type R = crate::R<MosictlSpec>;
#[doc = "Register `MOSICTL` writer"]
pub type W = crate::W<MosictlSpec>;
#[doc = "Field `VALUE` reader - 0:0\\]
MOSI level control. 0: Set MOSI low. 1: Set MOSI high."]
pub type ValueR = crate::BitReader;
#[doc = "Field `VALUE` writer - 0:0\\]
MOSI level control. 0: Set MOSI low. 1: Set MOSI high."]
pub type ValueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
MOSI level control. 0: Set MOSI low. 1: Set MOSI high."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
MOSI level control. 0: Set MOSI low. 1: Set MOSI high."]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> ValueW<MosictlSpec> {
        ValueW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<MosictlSpec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "MOSI Control Write operation stalls until current transfer completes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mosictl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mosictl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MosictlSpec;
impl crate::RegisterSpec for MosictlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mosictl::R`](R) reader structure"]
impl crate::Readable for MosictlSpec {}
#[doc = "`write(|w| ..)` method takes [`mosictl::W`](W) writer structure"]
impl crate::Writable for MosictlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MOSICTL to value 0"]
impl crate::Resettable for MosictlSpec {
    const RESET_VALUE: u32 = 0;
}
