#[doc = "Register `UARTILPR` reader"]
pub type R = crate::R<UartilprSpec>;
#[doc = "Register `UARTILPR` writer"]
pub type W = crate::W<UartilprSpec>;
#[doc = "Field `ILPDVSR` reader - 7:0\\]
8 bit low-power divisor value"]
pub type IlpdvsrR = crate::FieldReader;
#[doc = "Field `ILPDVSR` writer - 7:0\\]
8 bit low-power divisor value"]
pub type IlpdvsrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Reads to this field return zero, writes to this field are ignored."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Reads to this field return zero, writes to this field are ignored."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
8 bit low-power divisor value"]
    #[inline(always)]
    pub fn ilpdvsr(&self) -> IlpdvsrR {
        IlpdvsrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reads to this field return zero, writes to this field are ignored."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
8 bit low-power divisor value"]
    #[inline(always)]
    #[must_use]
    pub fn ilpdvsr(&mut self) -> IlpdvsrW<UartilprSpec> {
        IlpdvsrW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reads to this field return zero, writes to this field are ignored."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<UartilprSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "IrDA $lt;Low-Power Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uartilpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uartilpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartilprSpec;
impl crate::RegisterSpec for UartilprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartilpr::R`](R) reader structure"]
impl crate::Readable for UartilprSpec {}
#[doc = "`write(|w| ..)` method takes [`uartilpr::W`](W) writer structure"]
impl crate::Writable for UartilprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UARTILPR to value 0"]
impl crate::Resettable for UartilprSpec {
    const RESET_VALUE: u32 = 0;
}
