#[doc = "Register `ARMCLR` reader"]
pub type R = crate::R<ArmclrSpec>;
#[doc = "Register `ARMCLR` writer"]
pub type W = crate::W<ArmclrSpec>;
#[doc = "0:0\\]
Disarming channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0 {
    #[doc = "1: Set channel in UNARMED state without triggering event unless a compare/capture event happens in the same cycle"]
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
Disarming channel 0"]
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
    #[doc = "Set channel in UNARMED state without triggering event unless a compare/capture event happens in the same cycle"]
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
Disarming channel 0"]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG, Ch0>;
impl<'a, REG> Ch0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set channel in UNARMED state without triggering event unless a compare/capture event happens in the same cycle"]
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
Disarming channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1 {
    #[doc = "1: Set channel in UNARMED state without triggering event unless a compare/capture event happens in the same cycle"]
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
Disarming channel 1"]
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
    #[doc = "Set channel in UNARMED state without triggering event unless a compare/capture event happens in the same cycle"]
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
Disarming channel 1"]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG, Ch1>;
impl<'a, REG> Ch1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set channel in UNARMED state without triggering event unless a compare/capture event happens in the same cycle"]
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
#[doc = "2:2\\]
Disarming channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch2 {
    #[doc = "1: Set channel in UNARMED state without triggering event unless a compare/capture event happens in the same cycle"]
    Clr = 1,
    #[doc = "0: No effect on the channel"]
    Noeff = 0,
}
impl From<Ch2> for bool {
    #[inline(always)]
    fn from(variant: Ch2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2` reader - 2:2\\]
Disarming channel 2"]
pub type Ch2R = crate::BitReader<Ch2>;
impl Ch2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch2 {
        match self.bits {
            true => Ch2::Clr,
            false => Ch2::Noeff,
        }
    }
    #[doc = "Set channel in UNARMED state without triggering event unless a compare/capture event happens in the same cycle"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ch2::Clr
    }
    #[doc = "No effect on the channel"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Ch2::Noeff
    }
}
#[doc = "Field `CH2` writer - 2:2\\]
Disarming channel 2"]
pub type Ch2W<'a, REG> = crate::BitWriter<'a, REG, Ch2>;
impl<'a, REG> Ch2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set channel in UNARMED state without triggering event unless a compare/capture event happens in the same cycle"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2::Clr)
    }
    #[doc = "No effect on the channel"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2::Noeff)
    }
}
#[doc = "3:3\\]
Disarming channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch3 {
    #[doc = "1: Set channel in UNARMED state without triggering event unless a compare/capture event happens in the same cycle"]
    Clr = 1,
    #[doc = "0: No effect on the channel"]
    Noeff = 0,
}
impl From<Ch3> for bool {
    #[inline(always)]
    fn from(variant: Ch3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3` reader - 3:3\\]
Disarming channel 3"]
pub type Ch3R = crate::BitReader<Ch3>;
impl Ch3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch3 {
        match self.bits {
            true => Ch3::Clr,
            false => Ch3::Noeff,
        }
    }
    #[doc = "Set channel in UNARMED state without triggering event unless a compare/capture event happens in the same cycle"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ch3::Clr
    }
    #[doc = "No effect on the channel"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Ch3::Noeff
    }
}
#[doc = "Field `CH3` writer - 3:3\\]
Disarming channel 3"]
pub type Ch3W<'a, REG> = crate::BitWriter<'a, REG, Ch3>;
impl<'a, REG> Ch3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set channel in UNARMED state without triggering event unless a compare/capture event happens in the same cycle"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3::Clr)
    }
    #[doc = "No effect on the channel"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3::Noeff)
    }
}
#[doc = "4:4\\]
Disarming channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch4 {
    #[doc = "1: Set channel in UNARMED state without triggering event unless a compare/capture event happens in the same cycle"]
    Clr = 1,
    #[doc = "0: No effect on the channel"]
    Noeff = 0,
}
impl From<Ch4> for bool {
    #[inline(always)]
    fn from(variant: Ch4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH4` reader - 4:4\\]
Disarming channel 4"]
pub type Ch4R = crate::BitReader<Ch4>;
impl Ch4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch4 {
        match self.bits {
            true => Ch4::Clr,
            false => Ch4::Noeff,
        }
    }
    #[doc = "Set channel in UNARMED state without triggering event unless a compare/capture event happens in the same cycle"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ch4::Clr
    }
    #[doc = "No effect on the channel"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Ch4::Noeff
    }
}
#[doc = "Field `CH4` writer - 4:4\\]
Disarming channel 4"]
pub type Ch4W<'a, REG> = crate::BitWriter<'a, REG, Ch4>;
impl<'a, REG> Ch4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set channel in UNARMED state without triggering event unless a compare/capture event happens in the same cycle"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ch4::Clr)
    }
    #[doc = "No effect on the channel"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Ch4::Noeff)
    }
}
#[doc = "Field `RESERVED5` reader - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved5R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED5` writer - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved5W<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Disarming channel 0"]
    #[inline(always)]
    pub fn ch0(&self) -> Ch0R {
        Ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Disarming channel 1"]
    #[inline(always)]
    pub fn ch1(&self) -> Ch1R {
        Ch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Disarming channel 2"]
    #[inline(always)]
    pub fn ch2(&self) -> Ch2R {
        Ch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Disarming channel 3"]
    #[inline(always)]
    pub fn ch3(&self) -> Ch3R {
        Ch3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Disarming channel 4"]
    #[inline(always)]
    pub fn ch4(&self) -> Ch4R {
        Ch4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Disarming channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> Ch0W<ArmclrSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Disarming channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> Ch1W<ArmclrSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Disarming channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> Ch2W<ArmclrSpec> {
        Ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Disarming channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> Ch3W<ArmclrSpec> {
        Ch3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Disarming channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn ch4(&mut self) -> Ch4W<ArmclrSpec> {
        Ch4W::new(self, 4)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> Reserved5W<ArmclrSpec> {
        Reserved5W::new(self, 5)
    }
}
#[doc = "ARMCLR on read gives out the status of the 5 channels 1. channel state UNARMED returns 0 2. channel state CAPTURE or COMPARE returns 1 A write to ARMCLR has for each channel the following effect: 1. If ARMCLR\\[x\\]==0 no effect 2. Else Set channel in UNARMED state without triggering event unless a compare/capture event happens in the same cycle\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`armclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`armclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
