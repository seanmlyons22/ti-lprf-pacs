#[doc = "Register `CLKCFG1` reader"]
pub type R = crate::R<Clkcfg1Spec>;
#[doc = "Register `CLKCFG1` writer"]
pub type W = crate::W<Clkcfg1Spec>;
#[doc = "Field `SCR` reader - 9:0\\]
Serial clock divider. This is used to generate the transmit and receive bit rate of the SPI. The SPI bit rate: (SPI's functional clock frequency)/((SCR+1)*2). SCR value can be from 0 to1023."]
pub type ScrR = crate::FieldReader<u16>;
#[doc = "Field `SCR` writer - 9:0\\]
Serial clock divider. This is used to generate the transmit and receive bit rate of the SPI. The SPI bit rate: (SPI's functional clock frequency)/((SCR+1)*2). SCR value can be from 0 to1023."]
pub type ScrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `RESERVED10` reader - 15:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved10R = crate::FieldReader;
#[doc = "Field `DSAMPLE` reader - 19:16\\]
Delayed sampling. In controller mode the data on the input pin will be delayed sampled by the defined clock cycles. DSAMPLE values can range from 0 to SCR+1. Typically, values of 1 or 2 would suffice."]
pub type DsampleR = crate::FieldReader;
#[doc = "Field `DSAMPLE` writer - 19:16\\]
Delayed sampling. In controller mode the data on the input pin will be delayed sampled by the defined clock cycles. DSAMPLE values can range from 0 to SCR+1. Typically, values of 1 or 2 would suffice."]
pub type DsampleW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED20` reader - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved20R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Serial clock divider. This is used to generate the transmit and receive bit rate of the SPI. The SPI bit rate: (SPI's functional clock frequency)/((SCR+1)*2). SCR value can be from 0 to1023."]
    #[inline(always)]
    pub fn scr(&self) -> ScrR {
        ScrR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> Reserved10R {
        Reserved10R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Delayed sampling. In controller mode the data on the input pin will be delayed sampled by the defined clock cycles. DSAMPLE values can range from 0 to SCR+1. Typically, values of 1 or 2 would suffice."]
    #[inline(always)]
    pub fn dsample(&self) -> DsampleR {
        DsampleR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved20(&self) -> Reserved20R {
        Reserved20R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Serial clock divider. This is used to generate the transmit and receive bit rate of the SPI. The SPI bit rate: (SPI's functional clock frequency)/((SCR+1)*2). SCR value can be from 0 to1023."]
    #[inline(always)]
    #[must_use]
    pub fn scr(&mut self) -> ScrW<Clkcfg1Spec> {
        ScrW::new(self, 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Delayed sampling. In controller mode the data on the input pin will be delayed sampled by the defined clock cycles. DSAMPLE values can range from 0 to SCR+1. Typically, values of 1 or 2 would suffice."]
    #[inline(always)]
    #[must_use]
    pub fn dsample(&mut self) -> DsampleW<Clkcfg1Spec> {
        DsampleW::new(self, 16)
    }
}
#[doc = "Clock configuration register 1. This register is used to configure serial clock rate and clock count for delayed sampling in controller mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkcfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkcfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Clkcfg1Spec;
impl crate::RegisterSpec for Clkcfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkcfg1::R`](R) reader structure"]
impl crate::Readable for Clkcfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`clkcfg1::W`](W) writer structure"]
impl crate::Writable for Clkcfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKCFG1 to value 0"]
impl crate::Resettable for Clkcfg1Spec {
    const RESET_VALUE: u32 = 0;
}
