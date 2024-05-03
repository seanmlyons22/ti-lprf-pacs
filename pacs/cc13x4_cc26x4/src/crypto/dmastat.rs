#[doc = "Register `DMASTAT` reader"]
pub type R = crate::R<DmastatSpec>;
#[doc = "Register `DMASTAT` writer"]
pub type W = crate::W<DmastatSpec>;
#[doc = "Field `CH0_ACT` reader - 0:0\\]
A value of 1 indicates that channel 0 is active (DMA transfer on-going)."]
pub type Ch0ActR = crate::BitReader;
#[doc = "Field `CH0_ACT` writer - 0:0\\]
A value of 1 indicates that channel 0 is active (DMA transfer on-going)."]
pub type Ch0ActW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_ACT` reader - 1:1\\]
A value of 1 indicates that channel 1 is active (DMA transfer on-going)."]
pub type Ch1ActR = crate::BitReader;
#[doc = "Field `CH1_ACT` writer - 1:1\\]
A value of 1 indicates that channel 1 is active (DMA transfer on-going)."]
pub type Ch1ActW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 16:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED2` writer - 16:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `PORT_ERR` reader - 17:17\\]
Reflects possible transfer errors on the AHB port."]
pub type PortErrR = crate::BitReader;
#[doc = "Field `PORT_ERR` writer - 17:17\\]
Reflects possible transfer errors on the AHB port."]
pub type PortErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED18` reader - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved18R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED18` writer - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved18W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
A value of 1 indicates that channel 0 is active (DMA transfer on-going)."]
    #[inline(always)]
    pub fn ch0_act(&self) -> Ch0ActR {
        Ch0ActR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
A value of 1 indicates that channel 1 is active (DMA transfer on-going)."]
    #[inline(always)]
    pub fn ch1_act(&self) -> Ch1ActR {
        Ch1ActR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:16 - 16:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 0x7fff) as u16)
    }
    #[doc = "Bit 17 - 17:17\\]
Reflects possible transfer errors on the AHB port."]
    #[inline(always)]
    pub fn port_err(&self) -> PortErrR {
        PortErrR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved18(&self) -> Reserved18R {
        Reserved18R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
A value of 1 indicates that channel 0 is active (DMA transfer on-going)."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_act(&mut self) -> Ch0ActW<DmastatSpec> {
        Ch0ActW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
A value of 1 indicates that channel 1 is active (DMA transfer on-going)."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_act(&mut self) -> Ch1ActW<DmastatSpec> {
        Ch1ActW::new(self, 1)
    }
    #[doc = "Bits 2:16 - 16:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<DmastatSpec> {
        Reserved2W::new(self, 2)
    }
    #[doc = "Bit 17 - 17:17\\]
Reflects possible transfer errors on the AHB port."]
    #[inline(always)]
    #[must_use]
    pub fn port_err(&mut self) -> PortErrW<DmastatSpec> {
        PortErrW::new(self, 17)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved18(&mut self) -> Reserved18W<DmastatSpec> {
        Reserved18W::new(self, 18)
    }
}
#[doc = "DMAC Status This register provides the actual state of each DMA channel. It also reports port errors in case these were received by the master interface module during the data transfer.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmastat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmastat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmastatSpec;
impl crate::RegisterSpec for DmastatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmastat::R`](R) reader structure"]
impl crate::Readable for DmastatSpec {}
#[doc = "`write(|w| ..)` method takes [`dmastat::W`](W) writer structure"]
impl crate::Writable for DmastatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMASTAT to value 0"]
impl crate::Resettable for DmastatSpec {
    const RESET_VALUE: u32 = 0;
}
