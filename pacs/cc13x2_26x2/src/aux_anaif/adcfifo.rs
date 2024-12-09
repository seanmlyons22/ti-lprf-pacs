#[doc = "Register `ADCFIFO` reader"]
pub type R = crate::R<AdcfifoSpec>;
#[doc = "Register `ADCFIFO` writer"]
pub type W = crate::W<AdcfifoSpec>;
#[doc = "Field `DATA` reader - 11:0\\]
FIFO data. Read: Get oldest ADC sample from FIFO. Write: Write dummy sample to FIFO. This is useful for code development when you do not have real ADC samples."]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - 11:0\\]
FIFO data. Read: Get oldest ADC sample from FIFO. Write: Write dummy sample to FIFO. This is useful for code development when you do not have real ADC samples."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `RESERVED12` reader - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
FIFO data. Read: Get oldest ADC sample from FIFO. Write: Write dummy sample to FIFO. This is useful for code development when you do not have real ADC samples."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
FIFO data. Read: Get oldest ADC sample from FIFO. Write: Write dummy sample to FIFO. This is useful for code development when you do not have real ADC samples."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<AdcfifoSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "ADC FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcfifo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcfifo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcfifoSpec;
impl crate::RegisterSpec for AdcfifoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcfifo::R`](R) reader structure"]
impl crate::Readable for AdcfifoSpec {}
#[doc = "`write(|w| ..)` method takes [`adcfifo::W`](W) writer structure"]
impl crate::Writable for AdcfifoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCFIFO to value 0"]
impl crate::Resettable for AdcfifoSpec {
    const RESET_VALUE: u32 = 0;
}
