#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "Field `MCU_WARM_RESET` reader - 0:0\\]
Indicates type of last MCU Voltage Domain reset: 0: Last MCU reset was not a warm reset 1: Last MCU reset was a warm reset (requested from MCU or JTAG as indicated in MCU_RESET_SRC) This bit can only be cleared by writing a 1 to it"]
pub type McuWarmResetR = crate::BitReader;
#[doc = "Field `MCU_WARM_RESET` writer - 0:0\\]
Indicates type of last MCU Voltage Domain reset: 0: Last MCU reset was not a warm reset 1: Last MCU reset was a warm reset (requested from MCU or JTAG as indicated in MCU_RESET_SRC) This bit can only be cleared by writing a 1 to it"]
pub type McuWarmResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_RESET_SRC` reader - 1:1\\]
Indicates source of last MCU Voltage Domain warm reset request: 0: MCU SW reset 1: JTAG reset This bit can only be cleared by writing a 1 to it"]
pub type McuResetSrcR = crate::BitReader;
#[doc = "Field `MCU_RESET_SRC` writer - 1:1\\]
Indicates source of last MCU Voltage Domain warm reset request: 0: MCU SW reset 1: JTAG reset This bit can only be cleared by writing a 1 to it"]
pub type McuResetSrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates type of last MCU Voltage Domain reset: 0: Last MCU reset was not a warm reset 1: Last MCU reset was a warm reset (requested from MCU or JTAG as indicated in MCU_RESET_SRC) This bit can only be cleared by writing a 1 to it"]
    #[inline(always)]
    pub fn mcu_warm_reset(&self) -> McuWarmResetR {
        McuWarmResetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates source of last MCU Voltage Domain warm reset request: 0: MCU SW reset 1: JTAG reset This bit can only be cleared by writing a 1 to it"]
    #[inline(always)]
    pub fn mcu_reset_src(&self) -> McuResetSrcR {
        McuResetSrcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates type of last MCU Voltage Domain reset: 0: Last MCU reset was not a warm reset 1: Last MCU reset was a warm reset (requested from MCU or JTAG as indicated in MCU_RESET_SRC) This bit can only be cleared by writing a 1 to it"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_warm_reset(&mut self) -> McuWarmResetW<Ctl1Spec> {
        McuWarmResetW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates source of last MCU Voltage Domain warm reset request: 0: MCU SW reset 1: JTAG reset This bit can only be cleared by writing a 1 to it"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_reset_src(&mut self) -> McuResetSrcW<Ctl1Spec> {
        McuResetSrcW::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<Ctl1Spec> {
        Reserved2W::new(self, 2)
    }
}
#[doc = "Control 1 This register contains various chip level control and debug bitfields.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl1Spec;
impl crate::RegisterSpec for Ctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for Ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for Ctl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for Ctl1Spec {
    const RESET_VALUE: u32 = 0;
}
