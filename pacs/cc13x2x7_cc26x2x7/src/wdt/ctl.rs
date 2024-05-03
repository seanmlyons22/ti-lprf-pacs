#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "0:0\\]
WDT Interrupt Enable 0: Interrupt event disabled. 1: Interrupt event enabled. Once set, this bit can only be cleared by a hardware reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inten {
    #[doc = "1: Interrupt Enabled"]
    En = 1,
    #[doc = "0: Interrupt Disabled"]
    Dis = 0,
}
impl From<Inten> for bool {
    #[inline(always)]
    fn from(variant: Inten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTEN` reader - 0:0\\]
WDT Interrupt Enable 0: Interrupt event disabled. 1: Interrupt event enabled. Once set, this bit can only be cleared by a hardware reset."]
pub type IntenR = crate::BitReader<Inten>;
impl IntenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inten {
        match self.bits {
            true => Inten::En,
            false => Inten::Dis,
        }
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Inten::En
    }
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Inten::Dis
    }
}
#[doc = "Field `INTEN` writer - 0:0\\]
WDT Interrupt Enable 0: Interrupt event disabled. 1: Interrupt event enabled. Once set, this bit can only be cleared by a hardware reset."]
pub type IntenW<'a, REG> = crate::BitWriter<'a, REG, Inten>;
impl<'a, REG> IntenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Inten::En)
    }
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Inten::Dis)
    }
}
#[doc = "1:1\\]
WDT Reset Enable. Defines the function of the WDT reset source (see PRCM:WARMRESET.WDT_STAT if enabled) 0: Disabled. 1: Enable the Watchdog reset output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resen {
    #[doc = "1: Reset output Enabled"]
    En = 1,
    #[doc = "0: Reset output Disabled"]
    Dis = 0,
}
impl From<Resen> for bool {
    #[inline(always)]
    fn from(variant: Resen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESEN` reader - 1:1\\]
WDT Reset Enable. Defines the function of the WDT reset source (see PRCM:WARMRESET.WDT_STAT if enabled) 0: Disabled. 1: Enable the Watchdog reset output."]
pub type ResenR = crate::BitReader<Resen>;
impl ResenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Resen {
        match self.bits {
            true => Resen::En,
            false => Resen::Dis,
        }
    }
    #[doc = "Reset output Enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Resen::En
    }
    #[doc = "Reset output Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Resen::Dis
    }
}
#[doc = "Field `RESEN` writer - 1:1\\]
WDT Reset Enable. Defines the function of the WDT reset source (see PRCM:WARMRESET.WDT_STAT if enabled) 0: Disabled. 1: Enable the Watchdog reset output."]
pub type ResenW<'a, REG> = crate::BitWriter<'a, REG, Resen>;
impl<'a, REG> ResenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset output Enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Resen::En)
    }
    #[doc = "Reset output Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Resen::Dis)
    }
}
#[doc = "2:2\\]
WDT Interrupt Type 0: WDT interrupt is a standard interrupt. 1: WDT interrupt is a non-maskable interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inttype {
    #[doc = "1: Non-maskable interrupt"]
    Nonmaskable = 1,
    #[doc = "0: Maskable interrupt"]
    Maskable = 0,
}
impl From<Inttype> for bool {
    #[inline(always)]
    fn from(variant: Inttype) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTTYPE` reader - 2:2\\]
WDT Interrupt Type 0: WDT interrupt is a standard interrupt. 1: WDT interrupt is a non-maskable interrupt."]
pub type InttypeR = crate::BitReader<Inttype>;
impl InttypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inttype {
        match self.bits {
            true => Inttype::Nonmaskable,
            false => Inttype::Maskable,
        }
    }
    #[doc = "Non-maskable interrupt"]
    #[inline(always)]
    pub fn is_nonmaskable(&self) -> bool {
        *self == Inttype::Nonmaskable
    }
    #[doc = "Maskable interrupt"]
    #[inline(always)]
    pub fn is_maskable(&self) -> bool {
        *self == Inttype::Maskable
    }
}
#[doc = "Field `INTTYPE` writer - 2:2\\]
WDT Interrupt Type 0: WDT interrupt is a standard interrupt. 1: WDT interrupt is a non-maskable interrupt."]
pub type InttypeW<'a, REG> = crate::BitWriter<'a, REG, Inttype>;
impl<'a, REG> InttypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Non-maskable interrupt"]
    #[inline(always)]
    pub fn nonmaskable(self) -> &'a mut crate::W<REG> {
        self.variant(Inttype::Nonmaskable)
    }
    #[doc = "Maskable interrupt"]
    #[inline(always)]
    pub fn maskable(self) -> &'a mut crate::W<REG> {
        self.variant(Inttype::Maskable)
    }
}
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
WDT Interrupt Enable 0: Interrupt event disabled. 1: Interrupt event enabled. Once set, this bit can only be cleared by a hardware reset."]
    #[inline(always)]
    pub fn inten(&self) -> IntenR {
        IntenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
WDT Reset Enable. Defines the function of the WDT reset source (see PRCM:WARMRESET.WDT_STAT if enabled) 0: Disabled. 1: Enable the Watchdog reset output."]
    #[inline(always)]
    pub fn resen(&self) -> ResenR {
        ResenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
WDT Interrupt Type 0: WDT interrupt is a standard interrupt. 1: WDT interrupt is a non-maskable interrupt."]
    #[inline(always)]
    pub fn inttype(&self) -> InttypeR {
        InttypeR::new(((self.bits >> 2) & 1) != 0)
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
WDT Interrupt Enable 0: Interrupt event disabled. 1: Interrupt event enabled. Once set, this bit can only be cleared by a hardware reset."]
    #[inline(always)]
    #[must_use]
    pub fn inten(&mut self) -> IntenW<CtlSpec> {
        IntenW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
WDT Reset Enable. Defines the function of the WDT reset source (see PRCM:WARMRESET.WDT_STAT if enabled) 0: Disabled. 1: Enable the Watchdog reset output."]
    #[inline(always)]
    #[must_use]
    pub fn resen(&mut self) -> ResenW<CtlSpec> {
        ResenW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
WDT Interrupt Type 0: WDT interrupt is a standard interrupt. 1: WDT interrupt is a non-maskable interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn inttype(&mut self) -> InttypeW<CtlSpec> {
        InttypeW::new(self, 2)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<CtlSpec> {
        Reserved3W::new(self, 3)
    }
}
#[doc = "Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0;
}
