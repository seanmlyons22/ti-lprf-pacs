#[doc = "Register `IOCLATCH` reader"]
pub type R = crate::R<IoclatchSpec>;
#[doc = "Register `IOCLATCH` writer"]
pub type W = crate::W<IoclatchSpec>;
#[doc = "0:0\\]
Controls latches between MCU IOC and AON_IOC. The latches are transparent by default. They must be closed prior to power off the domain(s) controlling the IOs in order to preserve IO values on external pins.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "1: Latches are transparent, meaning the value of the IO is directly controlled by the GPIO or peripheral value"]
    Transp = 1,
    #[doc = "0: Latches are static, meaning the current value on the IO pin is frozen by latches and kept even if GPIO module or a peripheral module is turned off"]
    Static = 0,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - 0:0\\]
Controls latches between MCU IOC and AON_IOC. The latches are transparent by default. They must be closed prior to power off the domain(s) controlling the IOs in order to preserve IO values on external pins."]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            true => En::Transp,
            false => En::Static,
        }
    }
    #[doc = "Latches are transparent, meaning the value of the IO is directly controlled by the GPIO or peripheral value"]
    #[inline(always)]
    pub fn is_transp(&self) -> bool {
        *self == En::Transp
    }
    #[doc = "Latches are static, meaning the current value on the IO pin is frozen by latches and kept even if GPIO module or a peripheral module is turned off"]
    #[inline(always)]
    pub fn is_static(&self) -> bool {
        *self == En::Static
    }
}
#[doc = "Field `EN` writer - 0:0\\]
Controls latches between MCU IOC and AON_IOC. The latches are transparent by default. They must be closed prior to power off the domain(s) controlling the IOs in order to preserve IO values on external pins."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Latches are transparent, meaning the value of the IO is directly controlled by the GPIO or peripheral value"]
    #[inline(always)]
    pub fn transp(self) -> &'a mut crate::W<REG> {
        self.variant(En::Transp)
    }
    #[doc = "Latches are static, meaning the current value on the IO pin is frozen by latches and kept even if GPIO module or a peripheral module is turned off"]
    #[inline(always)]
    pub fn static_(self) -> &'a mut crate::W<REG> {
        self.variant(En::Static)
    }
}
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Controls latches between MCU IOC and AON_IOC. The latches are transparent by default. They must be closed prior to power off the domain(s) controlling the IOs in order to preserve IO values on external pins."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Controls latches between MCU IOC and AON_IOC. The latches are transparent by default. They must be closed prior to power off the domain(s) controlling the IOs in order to preserve IO values on external pins."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<IoclatchSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<IoclatchSpec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "IO Latch Control Controls transparency of all latches holding I/O or configuration state from the MCU IOC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioclatch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioclatch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoclatchSpec;
impl crate::RegisterSpec for IoclatchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioclatch::R`](R) reader structure"]
impl crate::Readable for IoclatchSpec {}
#[doc = "`write(|w| ..)` method takes [`ioclatch::W`](W) writer structure"]
impl crate::Writable for IoclatchSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOCLATCH to value 0x01"]
impl crate::Resettable for IoclatchSpec {
    const RESET_VALUE: u32 = 0x01;
}
