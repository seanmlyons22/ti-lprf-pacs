#[doc = "Register `ARMCLR` reader"]
pub type R = crate::R<ArmclrSpec>;
#[doc = "Register `ARMCLR` writer"]
pub type W = crate::W<ArmclrSpec>;
#[doc = "0:0\\]
Disarming Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0 {
    #[doc = "1: Set channel in UNARMED state without triggering event unless a compare event happens in the same cycle"]
    Clr = 1,
    #[doc = "0: No effect on the channel"]
    Noeff = 0,
}
impl From<Ch0> for bool {
    #[inline(always)]
    fn from(variant: Ch0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0` reader - 0:0\\]
Disarming Channel 0"]
pub type Ch0R = crate::BitReader<Ch0>;
impl Ch0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0 {
        match self.bits {
            true => Ch0::Clr,
            false => Ch0::Noeff,
        }
    }
    #[doc = "Set channel in UNARMED state without triggering event unless a compare event happens in the same cycle"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ch0::Clr
    }
    #[doc = "No effect on the channel"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Ch0::Noeff
    }
}
#[doc = "Field `CH0` writer - 0:0\\]
Disarming Channel 0"]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG, Ch0>;
impl<'a, REG> Ch0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set channel in UNARMED state without triggering event unless a compare event happens in the same cycle"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0::Clr)
    }
    #[doc = "No effect on the channel"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0::Noeff)
    }
}
#[doc = "1:1\\]
Disarming Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1 {
    #[doc = "1: Set channel in UNARMED state without triggering event unless a capture event happens in the same cycle"]
    Clr = 1,
    #[doc = "0: No effect on the channel"]
    Noeff = 0,
}
impl From<Ch1> for bool {
    #[inline(always)]
    fn from(variant: Ch1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1` reader - 1:1\\]
Disarming Channel 1"]
pub type Ch1R = crate::BitReader<Ch1>;
impl Ch1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1 {
        match self.bits {
            true => Ch1::Clr,
            false => Ch1::Noeff,
        }
    }
    #[doc = "Set channel in UNARMED state without triggering event unless a capture event happens in the same cycle"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ch1::Clr
    }
    #[doc = "No effect on the channel"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Ch1::Noeff
    }
}
#[doc = "Field `CH1` writer - 1:1\\]
Disarming Channel 1"]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG, Ch1>;
impl<'a, REG> Ch1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set channel in UNARMED state without triggering event unless a capture event happens in the same cycle"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1::Clr)
    }
    #[doc = "No effect on the channel"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1::Noeff)
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Disarming Channel 0"]
    #[inline(always)]
    pub fn ch0(&self) -> Ch0R {
        Ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Disarming Channel 1"]
    #[inline(always)]
    pub fn ch1(&self) -> Ch1R {
        Ch1R::new(((self.bits >> 1) & 1) != 0)
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
Disarming Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> Ch0W<ArmclrSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Disarming Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> Ch1W<ArmclrSpec> {
        Ch1W::new(self, 1)
    }
}
#[doc = "RTC channel mode clear register. Read to each bit field of this register provides the current channel mode. - Read of 1'b0 indicates the channel is unarmed. - Read of 1'b1 indicates the channel is either in capture or compare mode. A write to each bitfield of this register the following effect: - Write of 1'b0 has no effect on channel mode. - Write of 1'b1 for capture/compare channel will disarm it without triggering event unless a compare/capture event happens in the same cycle.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`armclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`armclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ArmclrSpec;
impl crate::RegisterSpec for ArmclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`armclr::R`](R) reader structure"]
impl crate::Readable for ArmclrSpec {}
#[doc = "`write(|w| ..)` method takes [`armclr::W`](W) writer structure"]
impl crate::Writable for ArmclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ARMCLR to value 0"]
impl crate::Resettable for ArmclrSpec {
    const RESET_VALUE: u32 = 0;
}
