#[doc = "Register `CTL0` reader"]
pub type R = crate::R<Ctl0Spec>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<Ctl0Spec>;
#[doc = "Field `RESERVED0` reader - 1:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `MCU_SRAM_ERASE` writer - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type McuSramEraseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_SRAM_ERASE` writer - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type AuxSramEraseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED4` reader - 7:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `RESERVED4` writer - 7:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PWR_DWN_DIS` reader - 8:8\\]
Controls whether MCU and AUX requesting to be powered off will enable a transition to powerdown: 0: Enabled 1: Disabled"]
pub type PwrDwnDisR = crate::BitReader;
#[doc = "Field `PWR_DWN_DIS` writer - 8:8\\]
Controls whether MCU and AUX requesting to be powered off will enable a transition to powerdown: 0: Enabled 1: Disabled"]
pub type PwrDwnDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED9` reader - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Controls whether MCU and AUX requesting to be powered off will enable a transition to powerdown: 0: Enabled 1: Disabled"]
    #[inline(always)]
    pub fn pwr_dwn_dis(&self) -> PwrDwnDisR {
        PwrDwnDisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_sram_erase(&mut self) -> McuSramEraseW<Ctl0Spec> {
        McuSramEraseW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn aux_sram_erase(&mut self) -> AuxSramEraseW<Ctl0Spec> {
        AuxSramEraseW::new(self, 3)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<Ctl0Spec> {
        Reserved4W::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
Controls whether MCU and AUX requesting to be powered off will enable a transition to powerdown: 0: Enabled 1: Disabled"]
    #[inline(always)]
    #[must_use]
    pub fn pwr_dwn_dis(&mut self) -> PwrDwnDisW<Ctl0Spec> {
        PwrDwnDisW::new(self, 8)
    }
}
#[doc = "Control 0 This register contains various chip level control and debug bitfields.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl0Spec;
impl crate::RegisterSpec for Ctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for Ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for Ctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for Ctl0Spec {
    const RESET_VALUE: u32 = 0;
}
