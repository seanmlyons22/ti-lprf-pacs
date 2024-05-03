#[doc = "Register `IOCONF` reader"]
pub type R = crate::R<IoconfSpec>;
#[doc = "Register `IOCONF` writer"]
pub type W = crate::W<IoconfSpec>;
#[doc = "Field `GPIO_CNT` reader - 6:0\\]
Number of available DIOs."]
pub type GpioCntR = crate::FieldReader;
#[doc = "Field `GPIO_CNT` writer - 6:0\\]
Number of available DIOs."]
pub type GpioCntW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RESERVED7` reader - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED7` writer - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Number of available DIOs."]
    #[inline(always)]
    pub fn gpio_cnt(&self) -> GpioCntR {
        GpioCntR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Number of available DIOs."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_cnt(&mut self) -> GpioCntW<IoconfSpec> {
        GpioCntW::new(self, 0)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> Reserved7W<IoconfSpec> {
        Reserved7W::new(self, 7)
    }
}
#[doc = "IO Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioconf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioconf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoconfSpec;
impl crate::RegisterSpec for IoconfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioconf::R`](R) reader structure"]
impl crate::Readable for IoconfSpec {}
#[doc = "`write(|w| ..)` method takes [`ioconf::W`](W) writer structure"]
impl crate::Writable for IoconfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOCONF to value 0xffff_ff00"]
impl crate::Resettable for IoconfSpec {
    const RESET_VALUE: u32 = 0xffff_ff00;
}
