#[doc = "Register `CPUIRQSEL42` reader"]
pub type R = crate::R<Cpuirqsel42Spec>;
#[doc = "Register `CPUIRQSEL42` writer"]
pub type W = crate::W<Cpuirqsel42Spec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 126"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "126: Interrupt event from I2C1"]
    I2c1Irq = 126,
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
            126 => Some(Ev::I2c1Irq),
            _ => None,
        }
    }
    #[doc = "Interrupt event from I2C1"]
    #[inline(always)]
    pub fn is_i2c1_irq(&self) -> bool {
        *self == Ev::I2c1Irq
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
    #[doc = "Interrupt event from I2C1"]
    #[inline(always)]
    pub fn i2c1_irq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::I2c1Irq)
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
    pub fn ev(&mut self) -> EvW<Cpuirqsel42Spec> {
        EvW::new(self, 0)
    }
}
#[doc = "Output Selection for CPU Interrupt 42\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel42::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel42::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpuirqsel42Spec;
impl crate::RegisterSpec for Cpuirqsel42Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuirqsel42::R`](R) reader structure"]
impl crate::Readable for Cpuirqsel42Spec {}
#[doc = "`write(|w| ..)` method takes [`cpuirqsel42::W`](W) writer structure"]
impl crate::Writable for Cpuirqsel42Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUIRQSEL42 to value 0x7e"]
impl crate::Resettable for Cpuirqsel42Spec {
    const RESET_VALUE: u32 = 0x7e;
}
