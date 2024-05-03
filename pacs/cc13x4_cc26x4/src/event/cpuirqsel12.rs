#[doc = "Register `CPUIRQSEL12` reader"]
pub type R = crate::R<Cpuirqsel12Spec>;
#[doc = "Register `CPUIRQSEL12` writer"]
pub type W = crate::W<Cpuirqsel12Spec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "8: Interrupt event from I2S"]
    I2sIrq = 8,
}
impl From<Ev> for u8 {
    #[inline(always)]
    fn from(variant: Ev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ev {
    type Ux = u8;
}
impl crate::IsEnum for Ev {}
#[doc = "Field `EV` reader - 7:0\\]
Read only selection value"]
pub type EvR = crate::FieldReader<Ev>;
impl EvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ev> {
        match self.bits {
            8 => Some(Ev::I2sIrq),
            _ => None,
        }
    }
    #[doc = "Interrupt event from I2S"]
    #[inline(always)]
    pub fn is_i2s_irq(&self) -> bool {
        *self == Ev::I2sIrq
    }
}
#[doc = "Field `EV` writer - 7:0\\]
Read only selection value"]
pub type EvW<'a, REG> = crate::FieldWriter<'a, REG, 8, Ev>;
impl<'a, REG> EvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt event from I2S"]
    #[inline(always)]
    pub fn i2s_irq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::I2sIrq)
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Read only selection value"]
    #[inline(always)]
    pub fn ev(&self) -> EvR {
        EvR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Read only selection value"]
    #[inline(always)]
    #[must_use]
    pub fn ev(&mut self) -> EvW<Cpuirqsel12Spec> {
        EvW::new(self, 0)
    }
}
#[doc = "Output Selection for CPU Interrupt 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpuirqsel12Spec;
impl crate::RegisterSpec for Cpuirqsel12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuirqsel12::R`](R) reader structure"]
impl crate::Readable for Cpuirqsel12Spec {}
#[doc = "`write(|w| ..)` method takes [`cpuirqsel12::W`](W) writer structure"]
impl crate::Writable for Cpuirqsel12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUIRQSEL12 to value 0x08"]
impl crate::Resettable for Cpuirqsel12Spec {
    const RESET_VALUE: u32 = 0x08;
}
