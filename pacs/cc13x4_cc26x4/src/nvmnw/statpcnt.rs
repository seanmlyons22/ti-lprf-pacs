#[doc = "Register `STATPCNT` reader"]
pub type R = crate::R<StatpcntSpec>;
#[doc = "Register `STATPCNT` writer"]
pub type W = crate::W<StatpcntSpec>;
#[doc = "11:0\\]
Current Pulse Counter Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Pulsecnt {
    #[doc = "4095: Maximum value"]
    Maximum = 4095,
    #[doc = "0: Minimum value"]
    Minimum = 0,
}
impl From<Pulsecnt> for u16 {
    #[inline(always)]
    fn from(variant: Pulsecnt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pulsecnt {
    type Ux = u16;
}
impl crate::IsEnum for Pulsecnt {}
#[doc = "Field `PULSECNT` reader - 11:0\\]
Current Pulse Counter Value"]
pub type PulsecntR = crate::FieldReader<Pulsecnt>;
impl PulsecntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pulsecnt> {
        match self.bits {
            4095 => Some(Pulsecnt::Maximum),
            0 => Some(Pulsecnt::Minimum),
            _ => None,
        }
    }
    #[doc = "Maximum value"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == Pulsecnt::Maximum
    }
    #[doc = "Minimum value"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == Pulsecnt::Minimum
    }
}
#[doc = "Field `RESERVED12` reader - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Current Pulse Counter Value"]
    #[inline(always)]
    pub fn pulsecnt(&self) -> PulsecntR {
        PulsecntR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {}
#[doc = "Current Pulse Count Register: Read only register giving read access to the state machine current pulse count value for program/erase operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statpcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`statpcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatpcntSpec;
impl crate::RegisterSpec for StatpcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statpcnt::R`](R) reader structure"]
impl crate::Readable for StatpcntSpec {}
#[doc = "`write(|w| ..)` method takes [`statpcnt::W`](W) writer structure"]
impl crate::Writable for StatpcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATPCNT to value 0"]
impl crate::Resettable for StatpcntSpec {
    const RESET_VALUE: u32 = 0;
}
