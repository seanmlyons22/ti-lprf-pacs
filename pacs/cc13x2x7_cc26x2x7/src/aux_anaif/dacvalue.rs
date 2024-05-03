#[doc = "Register `DACVALUE` reader"]
pub type R = crate::R<DacvalueSpec>;
#[doc = "Register `DACVALUE` writer"]
pub type W = crate::W<DacvalueSpec>;
#[doc = "Field `VALUE` reader - 7:0\\]
DAC value. Digital data word for the DAC. Only change VALUE when DACCTL.DAC_EN is 0. Then wait 1 us before you enable the DAC."]
pub type ValueR = crate::FieldReader;
#[doc = "Field `VALUE` writer - 7:0\\]
DAC value. Digital data word for the DAC. Only change VALUE when DACCTL.DAC_EN is 0. Then wait 1 us before you enable the DAC."]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
DAC value. Digital data word for the DAC. Only change VALUE when DACCTL.DAC_EN is 0. Then wait 1 us before you enable the DAC."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
DAC value. Digital data word for the DAC. Only change VALUE when DACCTL.DAC_EN is 0. Then wait 1 us before you enable the DAC."]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> ValueW<DacvalueSpec> {
        ValueW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<DacvalueSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "DAC Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dacvalue::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dacvalue::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacvalueSpec;
impl crate::RegisterSpec for DacvalueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dacvalue::R`](R) reader structure"]
impl crate::Readable for DacvalueSpec {}
#[doc = "`write(|w| ..)` method takes [`dacvalue::W`](W) writer structure"]
impl crate::Writable for DacvalueSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DACVALUE to value 0"]
impl crate::Resettable for DacvalueSpec {
    const RESET_VALUE: u32 = 0;
}
