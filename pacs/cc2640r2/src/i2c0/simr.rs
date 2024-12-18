#[doc = "Register `SIMR` reader"]
pub type R = crate::R<SimrSpec>;
#[doc = "Register `SIMR` writer"]
pub type W = crate::W<SimrSpec>;
#[doc = "Field `DATAIM` reader - 0:0\\]
Data interrupt mask 0: The SRIS.DATARIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.DATARIS interrupt is enabled and sent to the interrupt controller."]
pub type DataimR = crate::BitReader;
#[doc = "Field `DATAIM` writer - 0:0\\]
Data interrupt mask 0: The SRIS.DATARIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.DATARIS interrupt is enabled and sent to the interrupt controller."]
pub type DataimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "1:1\\]
Start condition interrupt mask 0: The SRIS.STARTRIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.STARTRIS interrupt is enabled and sent to the interrupt controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Startim {
    #[doc = "1: Enable Interrupt"]
    En = 1,
    #[doc = "0: Disable Interrupt"]
    Dis = 0,
}
impl From<Startim> for bool {
    #[inline(always)]
    fn from(variant: Startim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STARTIM` reader - 1:1\\]
Start condition interrupt mask 0: The SRIS.STARTRIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.STARTRIS interrupt is enabled and sent to the interrupt controller."]
pub type StartimR = crate::BitReader<Startim>;
impl StartimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Startim {
        match self.bits {
            true => Startim::En,
            false => Startim::Dis,
        }
    }
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Startim::En
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Startim::Dis
    }
}
#[doc = "Field `STARTIM` writer - 1:1\\]
Start condition interrupt mask 0: The SRIS.STARTRIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.STARTRIS interrupt is enabled and sent to the interrupt controller."]
pub type StartimW<'a, REG> = crate::BitWriter<'a, REG, Startim>;
impl<'a, REG> StartimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Startim::En)
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Startim::Dis)
    }
}
#[doc = "2:2\\]
Stop condition interrupt mask 0: The SRIS.STOPRIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.STOPRIS interrupt is enabled and sent to the interrupt controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stopim {
    #[doc = "1: Enable Interrupt"]
    En = 1,
    #[doc = "0: Disable Interrupt"]
    Dis = 0,
}
impl From<Stopim> for bool {
    #[inline(always)]
    fn from(variant: Stopim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPIM` reader - 2:2\\]
Stop condition interrupt mask 0: The SRIS.STOPRIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.STOPRIS interrupt is enabled and sent to the interrupt controller."]
pub type StopimR = crate::BitReader<Stopim>;
impl StopimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stopim {
        match self.bits {
            true => Stopim::En,
            false => Stopim::Dis,
        }
    }
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Stopim::En
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Stopim::Dis
    }
}
#[doc = "Field `STOPIM` writer - 2:2\\]
Stop condition interrupt mask 0: The SRIS.STOPRIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.STOPRIS interrupt is enabled and sent to the interrupt controller."]
pub type StopimW<'a, REG> = crate::BitWriter<'a, REG, Stopim>;
impl<'a, REG> StopimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Stopim::En)
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Stopim::Dis)
    }
}
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Data interrupt mask 0: The SRIS.DATARIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.DATARIS interrupt is enabled and sent to the interrupt controller."]
    #[inline(always)]
    pub fn dataim(&self) -> DataimR {
        DataimR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Start condition interrupt mask 0: The SRIS.STARTRIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.STARTRIS interrupt is enabled and sent to the interrupt controller."]
    #[inline(always)]
    pub fn startim(&self) -> StartimR {
        StartimR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Stop condition interrupt mask 0: The SRIS.STOPRIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.STOPRIS interrupt is enabled and sent to the interrupt controller."]
    #[inline(always)]
    pub fn stopim(&self) -> StopimR {
        StopimR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Data interrupt mask 0: The SRIS.DATARIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.DATARIS interrupt is enabled and sent to the interrupt controller."]
    #[inline(always)]
    #[must_use]
    pub fn dataim(&mut self) -> DataimW<SimrSpec> {
        DataimW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Start condition interrupt mask 0: The SRIS.STARTRIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.STARTRIS interrupt is enabled and sent to the interrupt controller."]
    #[inline(always)]
    #[must_use]
    pub fn startim(&mut self) -> StartimW<SimrSpec> {
        StartimW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Stop condition interrupt mask 0: The SRIS.STOPRIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.STOPRIS interrupt is enabled and sent to the interrupt controller."]
    #[inline(always)]
    #[must_use]
    pub fn stopim(&mut self) -> StopimW<SimrSpec> {
        StopimW::new(self, 2)
    }
}
#[doc = "Slave Interrupt Mask This register controls whether a raw interrupt is promoted to a controller interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`simr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`simr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SimrSpec;
impl crate::RegisterSpec for SimrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`simr::R`](R) reader structure"]
impl crate::Readable for SimrSpec {}
#[doc = "`write(|w| ..)` method takes [`simr::W`](W) writer structure"]
impl crate::Writable for SimrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIMR to value 0"]
impl crate::Resettable for SimrSpec {
    const RESET_VALUE: u32 = 0;
}
