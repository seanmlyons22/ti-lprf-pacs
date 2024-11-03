#[doc = "Register `ARMSET` reader"]
pub type R = crate::R<ArmsetSpec>;
#[doc = "Register `ARMSET` writer"]
pub type W = crate::W<ArmsetSpec>;
#[doc = "0:0\\]
No effect on arming the channel. Read will give the status of the Channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0 {
    #[doc = "1: No effect on the compare channel"]
    Set = 1,
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
No effect on arming the channel. Read will give the status of the Channel 0."]
pub type Ch0R = crate::BitReader<Ch0>;
impl Ch0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0 {
        match self.bits {
            true => Ch0::Set,
            false => Ch0::Noeff,
        }
    }
    #[doc = "No effect on the compare channel"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ch0::Set
    }
    #[doc = "No effect on the channel"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Ch0::Noeff
    }
}
#[doc = "Field `CH0` writer - 0:0\\]
No effect on arming the channel. Read will give the status of the Channel 0."]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG, Ch0>;
impl<'a, REG> Ch0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on the compare channel"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0::Set)
    }
    #[doc = "No effect on the channel"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0::Noeff)
    }
}
#[doc = "1:1\\]
Arming Channel 1 for capture operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1 {
    #[doc = "1: Enable the Channel 1 for capture operation"]
    Set = 1,
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
Arming Channel 1 for capture operation."]
pub type Ch1R = crate::BitReader<Ch1>;
impl Ch1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1 {
        match self.bits {
            true => Ch1::Set,
            false => Ch1::Noeff,
        }
    }
    #[doc = "Enable the Channel 1 for capture operation"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ch1::Set
    }
    #[doc = "No effect on the channel"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Ch1::Noeff
    }
}
#[doc = "Field `CH1` writer - 1:1\\]
Arming Channel 1 for capture operation."]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG, Ch1>;
impl<'a, REG> Ch1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable the Channel 1 for capture operation"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1::Set)
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
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
No effect on arming the channel. Read will give the status of the Channel 0."]
    #[inline(always)]
    pub fn ch0(&self) -> Ch0R {
        Ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Arming Channel 1 for capture operation."]
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
No effect on arming the channel. Read will give the status of the Channel 0."]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> Ch0W<ArmsetSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Arming Channel 1 for capture operation."]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> Ch1W<ArmsetSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<ArmsetSpec> {
        Reserved2W::new(self, 2)
    }
}
#[doc = "RTC channel mode set register. Read to each bit field of this register provides the current channel mode. - Read of 1'b0 indicates the channel is unarmed. - Read of 1'b1 indicates the channel is either in capture or compare mode. A write to each bitfield of this register the following effect: - Write of 1'b0 has no effect on channel mode. - Write of 1'b1 has no effect on the compare channel. While write of 1'b1 for capture channel will arm it in capture mode if it is disabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`armset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`armset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ArmsetSpec;
impl crate::RegisterSpec for ArmsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`armset::R`](R) reader structure"]
impl crate::Readable for ArmsetSpec {}
#[doc = "`write(|w| ..)` method takes [`armset::W`](W) writer structure"]
impl crate::Writable for ArmsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ARMSET to value 0"]
impl crate::Resettable for ArmsetSpec {
    const RESET_VALUE: u32 = 0;
}
