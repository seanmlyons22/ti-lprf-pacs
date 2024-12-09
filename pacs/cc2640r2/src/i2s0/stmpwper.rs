#[doc = "Register `STMPWPER` reader"]
pub type R = crate::R<StmpwperSpec>;
#[doc = "Register `STMPWPER` writer"]
pub type W = crate::W<StmpwperSpec>;
#[doc = "Field `VALUE` reader - 15:0\\]
Used to define when STMPWCNT is to be reset so number of WCLK edges are found for the size of the sample buffer. This is thus a modulo value for the WCLK counter. This number must correspond to the size of the sample buffer used by the system (that is the index of the last sample plus 1)."]
pub type ValueR = crate::FieldReader<u16>;
#[doc = "Field `VALUE` writer - 15:0\\]
Used to define when STMPWCNT is to be reset so number of WCLK edges are found for the size of the sample buffer. This is thus a modulo value for the WCLK counter. This number must correspond to the size of the sample buffer used by the system (that is the index of the last sample plus 1)."]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Used to define when STMPWCNT is to be reset so number of WCLK edges are found for the size of the sample buffer. This is thus a modulo value for the WCLK counter. This number must correspond to the size of the sample buffer used by the system (that is the index of the last sample plus 1)."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Used to define when STMPWCNT is to be reset so number of WCLK edges are found for the size of the sample buffer. This is thus a modulo value for the WCLK counter. This number must correspond to the size of the sample buffer used by the system (that is the index of the last sample plus 1)."]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> ValueW<StmpwperSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "WCLK Counter Period Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stmpwper::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stmpwper::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StmpwperSpec;
impl crate::RegisterSpec for StmpwperSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stmpwper::R`](R) reader structure"]
impl crate::Readable for StmpwperSpec {}
#[doc = "`write(|w| ..)` method takes [`stmpwper::W`](W) writer structure"]
impl crate::Writable for StmpwperSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STMPWPER to value 0"]
impl crate::Resettable for StmpwperSpec {
    const RESET_VALUE: u32 = 0;
}
