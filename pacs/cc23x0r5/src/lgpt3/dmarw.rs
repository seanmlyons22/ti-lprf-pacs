#[doc = "Register `DMARW` reader"]
pub type R = crate::R<DmarwSpec>;
#[doc = "Register `DMARW` writer"]
pub type W = crate::W<DmarwSpec>;
#[doc = "Field `VAL` reader - 23:0\\]
DMA read write value. The value that is read/written from/to the registers."]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - 23:0\\]
DMA read write value. The value that is read/written from/to the registers."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `RESERVED24` reader - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24R = crate::FieldReader;
#[doc = "Field `RESERVED24` writer - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
DMA read write value. The value that is read/written from/to the registers."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> Reserved24R {
        Reserved24R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
DMA read write value. The value that is read/written from/to the registers."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<DmarwSpec> {
        ValW::new(self, 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved24(&mut self) -> Reserved24W<DmarwSpec> {
        Reserved24W::new(self, 24)
    }
}
#[doc = "Direct Memory Access This register is used by the DMA to access (read/write) register inside this LGPT module. Each access to this register will increment the internal DMA address counter. See DMA for description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmarw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmarw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmarwSpec;
impl crate::RegisterSpec for DmarwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmarw::R`](R) reader structure"]
impl crate::Readable for DmarwSpec {}
#[doc = "`write(|w| ..)` method takes [`dmarw::W`](W) writer structure"]
impl crate::Writable for DmarwSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMARW to value 0"]
impl crate::Resettable for DmarwSpec {
    const RESET_VALUE: u32 = 0;
}