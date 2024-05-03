#[doc = "Register `CHCTL` reader"]
pub type R = crate::R<ChctlSpec>;
#[doc = "Register `CHCTL` writer"]
pub type W = crate::W<ChctlSpec>;
#[doc = "Field `CH0_EN` reader - 0:0\\]
RTC Channel 0 Enable 0: Disable RTC Channel 0 1: Enable RTC Channel 0"]
pub type Ch0EnR = crate::BitReader;
#[doc = "Field `CH0_EN` writer - 0:0\\]
RTC Channel 0 Enable 0: Disable RTC Channel 0 1: Enable RTC Channel 0"]
pub type Ch0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `RESERVED1` writer - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `CH1_EN` reader - 8:8\\]
RTC Channel 1 Enable 0: Disable RTC Channel 1 1: Enable RTC Channel 1"]
pub type Ch1EnR = crate::BitReader;
#[doc = "Field `CH1_EN` writer - 8:8\\]
RTC Channel 1 Enable 0: Disable RTC Channel 1 1: Enable RTC Channel 1"]
pub type Ch1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CAPT_EN` reader - 9:9\\]
Set Channel 1 mode 0: Compare mode (default) 1: Capture mode"]
pub type Ch1CaptEnR = crate::BitReader;
#[doc = "Field `CH1_CAPT_EN` writer - 9:9\\]
Set Channel 1 mode 0: Compare mode (default) 1: Capture mode"]
pub type Ch1CaptEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED10` reader - 15:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved10R = crate::FieldReader;
#[doc = "Field `RESERVED10` writer - 15:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved10W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CH2_EN` reader - 16:16\\]
RTC Channel 2 Enable 0: Disable RTC Channel 2 1: Enable RTC Channel 2"]
pub type Ch2EnR = crate::BitReader;
#[doc = "Field `CH2_EN` writer - 16:16\\]
RTC Channel 2 Enable 0: Disable RTC Channel 2 1: Enable RTC Channel 2"]
pub type Ch2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED17` reader - 17:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved17R = crate::BitReader;
#[doc = "Field `RESERVED17` writer - 17:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_CONT_EN` reader - 18:18\\]
Set to enable continuous operation of Channel 2"]
pub type Ch2ContEnR = crate::BitReader;
#[doc = "Field `CH2_CONT_EN` writer - 18:18\\]
Set to enable continuous operation of Channel 2"]
pub type Ch2ContEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED19` reader - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved19R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED19` writer - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved19W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
RTC Channel 0 Enable 0: Disable RTC Channel 0 1: Enable RTC Channel 0"]
    #[inline(always)]
    pub fn ch0_en(&self) -> Ch0EnR {
        Ch0EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
RTC Channel 1 Enable 0: Disable RTC Channel 1 1: Enable RTC Channel 1"]
    #[inline(always)]
    pub fn ch1_en(&self) -> Ch1EnR {
        Ch1EnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Set Channel 1 mode 0: Compare mode (default) 1: Capture mode"]
    #[inline(always)]
    pub fn ch1_capt_en(&self) -> Ch1CaptEnR {
        Ch1CaptEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> Reserved10R {
        Reserved10R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
RTC Channel 2 Enable 0: Disable RTC Channel 2 1: Enable RTC Channel 2"]
    #[inline(always)]
    pub fn ch2_en(&self) -> Ch2EnR {
        Ch2EnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> Reserved17R {
        Reserved17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Set to enable continuous operation of Channel 2"]
    #[inline(always)]
    pub fn ch2_cont_en(&self) -> Ch2ContEnR {
        Ch2ContEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:31 - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved19(&self) -> Reserved19R {
        Reserved19R::new(((self.bits >> 19) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
RTC Channel 0 Enable 0: Disable RTC Channel 0 1: Enable RTC Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_en(&mut self) -> Ch0EnW<ChctlSpec> {
        Ch0EnW::new(self, 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<ChctlSpec> {
        Reserved1W::new(self, 1)
    }
    #[doc = "Bit 8 - 8:8\\]
RTC Channel 1 Enable 0: Disable RTC Channel 1 1: Enable RTC Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_en(&mut self) -> Ch1EnW<ChctlSpec> {
        Ch1EnW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Set Channel 1 mode 0: Compare mode (default) 1: Capture mode"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_capt_en(&mut self) -> Ch1CaptEnW<ChctlSpec> {
        Ch1CaptEnW::new(self, 9)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved10(&mut self) -> Reserved10W<ChctlSpec> {
        Reserved10W::new(self, 10)
    }
    #[doc = "Bit 16 - 16:16\\]
RTC Channel 2 Enable 0: Disable RTC Channel 2 1: Enable RTC Channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_en(&mut self) -> Ch2EnW<ChctlSpec> {
        Ch2EnW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved17(&mut self) -> Reserved17W<ChctlSpec> {
        Reserved17W::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Set to enable continuous operation of Channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_cont_en(&mut self) -> Ch2ContEnW<ChctlSpec> {
        Ch2ContEnW::new(self, 18)
    }
    #[doc = "Bits 19:31 - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved19(&mut self) -> Reserved19W<ChctlSpec> {
        Reserved19W::new(self, 19)
    }
}
#[doc = "Channel Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChctlSpec;
impl crate::RegisterSpec for ChctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chctl::R`](R) reader structure"]
impl crate::Readable for ChctlSpec {}
#[doc = "`write(|w| ..)` method takes [`chctl::W`](W) writer structure"]
impl crate::Writable for ChctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHCTL to value 0"]
impl crate::Resettable for ChctlSpec {
    const RESET_VALUE: u32 = 0;
}
