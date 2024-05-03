#[doc = "Register `SPPR` reader"]
pub type R = crate::R<SpprSpec>;
#[doc = "Register `SPPR` writer"]
pub type W = crate::W<SpprSpec>;
#[doc = "1:0\\]
Trace output protocol\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Protocol {
    #[doc = "2: SerialWire Output (NRZ)"]
    SwoNrz = 2,
    #[doc = "1: SerialWire Output (Manchester). This is the reset value."]
    SwoManchester = 1,
    #[doc = "0: TracePort mode"]
    Traceport = 0,
}
impl From<Protocol> for u8 {
    #[inline(always)]
    fn from(variant: Protocol) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Protocol {
    type Ux = u8;
}
impl crate::IsEnum for Protocol {}
#[doc = "Field `PROTOCOL` reader - 1:0\\]
Trace output protocol"]
pub type ProtocolR = crate::FieldReader<Protocol>;
impl ProtocolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Protocol> {
        match self.bits {
            2 => Some(Protocol::SwoNrz),
            1 => Some(Protocol::SwoManchester),
            0 => Some(Protocol::Traceport),
            _ => None,
        }
    }
    #[doc = "SerialWire Output (NRZ)"]
    #[inline(always)]
    pub fn is_swo_nrz(&self) -> bool {
        *self == Protocol::SwoNrz
    }
    #[doc = "SerialWire Output (Manchester). This is the reset value."]
    #[inline(always)]
    pub fn is_swo_manchester(&self) -> bool {
        *self == Protocol::SwoManchester
    }
    #[doc = "TracePort mode"]
    #[inline(always)]
    pub fn is_traceport(&self) -> bool {
        *self == Protocol::Traceport
    }
}
#[doc = "Field `PROTOCOL` writer - 1:0\\]
Trace output protocol"]
pub type ProtocolW<'a, REG> = crate::FieldWriter<'a, REG, 2, Protocol>;
impl<'a, REG> ProtocolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SerialWire Output (NRZ)"]
    #[inline(always)]
    pub fn swo_nrz(self) -> &'a mut crate::W<REG> {
        self.variant(Protocol::SwoNrz)
    }
    #[doc = "SerialWire Output (Manchester). This is the reset value."]
    #[inline(always)]
    pub fn swo_manchester(self) -> &'a mut crate::W<REG> {
        self.variant(Protocol::SwoManchester)
    }
    #[doc = "TracePort mode"]
    #[inline(always)]
    pub fn traceport(self) -> &'a mut crate::W<REG> {
        self.variant(Protocol::Traceport)
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Trace output protocol"]
    #[inline(always)]
    pub fn protocol(&self) -> ProtocolR {
        ProtocolR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Trace output protocol"]
    #[inline(always)]
    #[must_use]
    pub fn protocol(&mut self) -> ProtocolW<SpprSpec> {
        ProtocolW::new(self, 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<SpprSpec> {
        Reserved2W::new(self, 2)
    }
}
#[doc = "Selected Pin Protocol This register selects the protocol to be used for trace output. Note: If this register is changed while trace data is being output, data corruption occurs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sppr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sppr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpprSpec;
impl crate::RegisterSpec for SpprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sppr::R`](R) reader structure"]
impl crate::Readable for SpprSpec {}
#[doc = "`write(|w| ..)` method takes [`sppr::W`](W) writer structure"]
impl crate::Writable for SpprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPPR to value 0x01"]
impl crate::Resettable for SpprSpec {
    const RESET_VALUE: u32 = 0x01;
}
