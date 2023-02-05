#[doc = "Register `IMASK` reader"]
pub type R = crate::R<ImaskSpec>;
#[doc = "Register `IMASK` writer"]
pub type W = crate::W<ImaskSpec>;
#[doc = "0:0\\]
Channel 0 Event Interrupt Mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ev0 {
    #[doc = "1: Enable Interrrupt Mask"]
    En = 1,
    #[doc = "0: Disable Interrupt Mask"]
    Dis = 0,
}
impl From<Ev0> for bool {
    #[inline(always)]
    fn from(variant: Ev0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV0` reader - 0:0\\]
Channel 0 Event Interrupt Mask."]
pub type Ev0R = crate::BitReader<Ev0>;
impl Ev0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ev0 {
        match self.bits {
            true => Ev0::En,
            false => Ev0::Dis,
        }
    }
    #[doc = "Enable Interrrupt Mask"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Ev0::En
    }
    #[doc = "Disable Interrupt Mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ev0::Dis
    }
}
#[doc = "Field `EV0` writer - 0:0\\]
Channel 0 Event Interrupt Mask."]
pub type Ev0W<'a, REG> = crate::BitWriter<'a, REG, Ev0>;
impl<'a, REG> Ev0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Interrrupt Mask"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Ev0::En)
    }
    #[doc = "Disable Interrupt Mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ev0::Dis)
    }
}
#[doc = "1:1\\]
Channel 1 Event Interrupt Mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ev1 {
    #[doc = "1: Enable Interrrupt Mask"]
    En = 1,
    #[doc = "0: Clear Interrupt Mask"]
    Dis = 0,
}
impl From<Ev1> for bool {
    #[inline(always)]
    fn from(variant: Ev1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV1` reader - 1:1\\]
Channel 1 Event Interrupt Mask."]
pub type Ev1R = crate::BitReader<Ev1>;
impl Ev1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ev1 {
        match self.bits {
            true => Ev1::En,
            false => Ev1::Dis,
        }
    }
    #[doc = "Enable Interrrupt Mask"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Ev1::En
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ev1::Dis
    }
}
#[doc = "Field `EV1` writer - 1:1\\]
Channel 1 Event Interrupt Mask."]
pub type Ev1W<'a, REG> = crate::BitWriter<'a, REG, Ev1>;
impl<'a, REG> Ev1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Interrrupt Mask"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1::En)
    }
    #[doc = "Clear Interrupt Mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1::Dis)
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Channel 0 Event Interrupt Mask."]
    #[inline(always)]
    pub fn ev0(&self) -> Ev0R {
        Ev0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Channel 1 Event Interrupt Mask."]
    #[inline(always)]
    pub fn ev1(&self) -> Ev1R {
        Ev1R::new(((self.bits >> 1) & 1) != 0)
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
Channel 0 Event Interrupt Mask."]
    #[inline(always)]
    #[must_use]
    pub fn ev0(&mut self) -> Ev0W<ImaskSpec> {
        Ev0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Channel 1 Event Interrupt Mask."]
    #[inline(always)]
    #[must_use]
    pub fn ev1(&mut self) -> Ev1W<ImaskSpec> {
        Ev1W::new(self, 1)
    }
}
#[doc = "Interrupt Mask. If a bit is set, then corresponding interrupt is un-masked. Un-masking the interrupt causes the raw interrupt to be visible in IIDX, as well as MIS.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImaskSpec;
impl crate::RegisterSpec for ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imask::R`](R) reader structure"]
impl crate::Readable for ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`imask::W`](W) writer structure"]
impl crate::Writable for ImaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMASK to value 0"]
impl crate::Resettable for ImaskSpec {
    const RESET_VALUE: u32 = 0;
}
