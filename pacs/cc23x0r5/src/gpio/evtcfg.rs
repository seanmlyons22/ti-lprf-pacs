#[doc = "Register `EVTCFG` reader"]
pub type R = crate::R<EvtcfgSpec>;
#[doc = "Register `EVTCFG` writer"]
pub type W = crate::W<EvtcfgSpec>;
#[doc = "5:0\\]
This is used to select DIO for event generation. For example, DIOSEL = 0x0 selects DIO0 and DIOSEL = 0x8 selects DIO8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Diosel {
    #[doc = "63: Maximum value"]
    Maximum = 63,
    #[doc = "0: Minimum value"]
    Minimum = 0,
}
impl From<Diosel> for u8 {
    #[inline(always)]
    fn from(variant: Diosel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Diosel {
    type Ux = u8;
}
impl crate::IsEnum for Diosel {}
#[doc = "Field `DIOSEL` reader - 5:0\\]
This is used to select DIO for event generation. For example, DIOSEL = 0x0 selects DIO0 and DIOSEL = 0x8 selects DIO8."]
pub type DioselR = crate::FieldReader<Diosel>;
impl DioselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Diosel> {
        match self.bits {
            63 => Some(Diosel::Maximum),
            0 => Some(Diosel::Minimum),
            _ => None,
        }
    }
    #[doc = "Maximum value"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == Diosel::Maximum
    }
    #[doc = "Minimum value"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == Diosel::Minimum
    }
}
#[doc = "Field `DIOSEL` writer - 5:0\\]
This is used to select DIO for event generation. For example, DIOSEL = 0x0 selects DIO0 and DIOSEL = 0x8 selects DIO8."]
pub type DioselW<'a, REG> = crate::FieldWriter<'a, REG, 6, Diosel>;
impl<'a, REG> DioselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Maximum value"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut crate::W<REG> {
        self.variant(Diosel::Maximum)
    }
    #[doc = "Minimum value"]
    #[inline(always)]
    pub fn minimum(self) -> &'a mut crate::W<REG> {
        self.variant(Diosel::Minimum)
    }
}
#[doc = "Field `RESERVED6` reader - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader;
#[doc = "Field `RESERVED6` writer - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "8:8\\]
Enables GPIO to publish edge qualified selected DIO event on SVT event fabric. Design note: The edge detector flop is cleared automatically for the selected DIO once the event is published.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Evten {
    #[doc = "1: Enable"]
    Enable = 1,
    #[doc = "0: Disable"]
    Disable = 0,
}
impl From<Evten> for bool {
    #[inline(always)]
    fn from(variant: Evten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVTEN` reader - 8:8\\]
Enables GPIO to publish edge qualified selected DIO event on SVT event fabric. Design note: The edge detector flop is cleared automatically for the selected DIO once the event is published."]
pub type EvtenR = crate::BitReader<Evten>;
impl EvtenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Evten {
        match self.bits {
            true => Evten::Enable,
            false => Evten::Disable,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Evten::Enable
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Evten::Disable
    }
}
#[doc = "Field `EVTEN` writer - 8:8\\]
Enables GPIO to publish edge qualified selected DIO event on SVT event fabric. Design note: The edge detector flop is cleared automatically for the selected DIO once the event is published."]
pub type EvtenW<'a, REG> = crate::BitWriter<'a, REG, Evten>;
impl<'a, REG> EvtenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Evten::Enable)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Evten::Disable)
    }
}
#[doc = "Field `RESERVED9` reader - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED9` writer - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
This is used to select DIO for event generation. For example, DIOSEL = 0x0 selects DIO0 and DIOSEL = 0x8 selects DIO8."]
    #[inline(always)]
    pub fn diosel(&self) -> DioselR {
        DioselR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Enables GPIO to publish edge qualified selected DIO event on SVT event fabric. Design note: The edge detector flop is cleared automatically for the selected DIO once the event is published."]
    #[inline(always)]
    pub fn evten(&self) -> EvtenR {
        EvtenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
This is used to select DIO for event generation. For example, DIOSEL = 0x0 selects DIO0 and DIOSEL = 0x8 selects DIO8."]
    #[inline(always)]
    #[must_use]
    pub fn diosel(&mut self) -> DioselW<EvtcfgSpec> {
        DioselW::new(self, 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<EvtcfgSpec> {
        Reserved6W::new(self, 6)
    }
    #[doc = "Bit 8 - 8:8\\]
Enables GPIO to publish edge qualified selected DIO event on SVT event fabric. Design note: The edge detector flop is cleared automatically for the selected DIO once the event is published."]
    #[inline(always)]
    #[must_use]
    pub fn evten(&mut self) -> EvtenW<EvtcfgSpec> {
        EvtenW::new(self, 8)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> Reserved9W<EvtcfgSpec> {
        Reserved9W::new(self, 9)
    }
}
#[doc = "Event configuration. This register is used to select DIO for GPIO to publish event on SVT event fabric. It also contains enable bit that is used to mask the event.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evtcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evtcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvtcfgSpec;
impl crate::RegisterSpec for EvtcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evtcfg::R`](R) reader structure"]
impl crate::Readable for EvtcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`evtcfg::W`](W) writer structure"]
impl crate::Writable for EvtcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVTCFG to value 0"]
impl crate::Resettable for EvtcfgSpec {
    const RESET_VALUE: u32 = 0;
}
