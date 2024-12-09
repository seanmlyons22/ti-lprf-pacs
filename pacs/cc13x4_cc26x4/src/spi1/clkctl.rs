#[doc = "Register `CLKCTL` reader"]
pub type R = crate::R<ClkctlSpec>;
#[doc = "Register `CLKCTL` writer"]
pub type W = crate::W<ClkctlSpec>;
#[doc = "9:0\\]
Serial clock divider: This is used to generate the transmit and receive bit rate of the SPI. The SPI bit rate is (SPI's functional clock frequency)/((SCR+1)*2). SCR is a value from 0-1023.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Scr {
    #[doc = "1023: Highest possible value"]
    Maximum = 1023,
    #[doc = "0: Smallest value"]
    Minimum = 0,
}
impl From<Scr> for u16 {
    #[inline(always)]
    fn from(variant: Scr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Scr {
    type Ux = u16;
}
impl crate::IsEnum for Scr {}
#[doc = "Field `SCR` reader - 9:0\\]
Serial clock divider: This is used to generate the transmit and receive bit rate of the SPI. The SPI bit rate is (SPI's functional clock frequency)/((SCR+1)*2). SCR is a value from 0-1023."]
pub type ScrR = crate::FieldReader<Scr>;
impl ScrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Scr> {
        match self.bits {
            1023 => Some(Scr::Maximum),
            0 => Some(Scr::Minimum),
            _ => None,
        }
    }
    #[doc = "Highest possible value"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == Scr::Maximum
    }
    #[doc = "Smallest value"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == Scr::Minimum
    }
}
#[doc = "Field `SCR` writer - 9:0\\]
Serial clock divider: This is used to generate the transmit and receive bit rate of the SPI. The SPI bit rate is (SPI's functional clock frequency)/((SCR+1)*2). SCR is a value from 0-1023."]
pub type ScrW<'a, REG> = crate::FieldWriter<'a, REG, 10, Scr>;
impl<'a, REG> ScrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Highest possible value"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut crate::W<REG> {
        self.variant(Scr::Maximum)
    }
    #[doc = "Smallest value"]
    #[inline(always)]
    pub fn minimum(self) -> &'a mut crate::W<REG> {
        self.variant(Scr::Minimum)
    }
}
#[doc = "Field `RESERVED10` reader - 27:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved10R = crate::FieldReader<u32>;
#[doc = "Field `DSAMPLE` reader - 31:28\\]
Delayed sampling. In master mode the data on the input pin will be sampled after the defined clock cycles. Note: As an example, if the SPI transmit frequency is set to 12 MHz in the master mode, DSAMPLE should be set to a value of 2"]
pub type DsampleR = crate::FieldReader;
#[doc = "Field `DSAMPLE` writer - 31:28\\]
Delayed sampling. In master mode the data on the input pin will be sampled after the defined clock cycles. Note: As an example, if the SPI transmit frequency is set to 12 MHz in the master mode, DSAMPLE should be set to a value of 2"]
pub type DsampleW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Serial clock divider: This is used to generate the transmit and receive bit rate of the SPI. The SPI bit rate is (SPI's functional clock frequency)/((SCR+1)*2). SCR is a value from 0-1023."]
    #[inline(always)]
    pub fn scr(&self) -> ScrR {
        ScrR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:27 - 27:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> Reserved10R {
        Reserved10R::new((self.bits >> 10) & 0x0003_ffff)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Delayed sampling. In master mode the data on the input pin will be sampled after the defined clock cycles. Note: As an example, if the SPI transmit frequency is set to 12 MHz in the master mode, DSAMPLE should be set to a value of 2"]
    #[inline(always)]
    pub fn dsample(&self) -> DsampleR {
        DsampleR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Serial clock divider: This is used to generate the transmit and receive bit rate of the SPI. The SPI bit rate is (SPI's functional clock frequency)/((SCR+1)*2). SCR is a value from 0-1023."]
    #[inline(always)]
    #[must_use]
    pub fn scr(&mut self) -> ScrW<ClkctlSpec> {
        ScrW::new(self, 0)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Delayed sampling. In master mode the data on the input pin will be sampled after the defined clock cycles. Note: As an example, if the SPI transmit frequency is set to 12 MHz in the master mode, DSAMPLE should be set to a value of 2"]
    #[inline(always)]
    #[must_use]
    pub fn dsample(&mut self) -> DsampleW<ClkctlSpec> {
        DsampleW::new(self, 28)
    }
}
#[doc = "Clock prescaler and divider register. This register contains the settings for the Clock prescaler and divider settings.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkctlSpec;
impl crate::RegisterSpec for ClkctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkctl::R`](R) reader structure"]
impl crate::Readable for ClkctlSpec {}
#[doc = "`write(|w| ..)` method takes [`clkctl::W`](W) writer structure"]
impl crate::Writable for ClkctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKCTL to value 0"]
impl crate::Resettable for ClkctlSpec {
    const RESET_VALUE: u32 = 0;
}
