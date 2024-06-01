#[doc = "Register `TEMPUL` reader"]
pub type R = crate::R<TempulSpec>;
#[doc = "Register `TEMPUL` writer"]
pub type W = crate::W<TempulSpec>;
#[doc = "Field `RESERVED0` reader - 5:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `RESERVED0` writer - 5:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `FRAC` reader - 7:6\\]
Fractional part of temperature upper limit. Total value = INTEGER + FRACTIONAL The encoding is an extension of the 2's complement encoding. 00: 0.0C 01: 0.25C 10: 0.5C 11: 0.75C For example: 000000001,00 = ( 1+0,00) = 1,00 000000000,11 = ( 0+0,75) = 0,75 000000000,10 = ( 0+0,50) = 0,50 000000000,01 = ( 0+0,25) = 0,25 000000000,00 = ( 0+0,00) = 0,00 111111111,11 = (-1+0,75) = -0,25 111111111,10 = (-1+0,50) = -0,50 111111111,01 = (-1+0,25) = -0,75 111111111,00 = (-1+0,00) = -1,00 111111110,11 = (-2+0,75) = -1,25"]
pub type FracR = crate::FieldReader;
#[doc = "Field `FRAC` writer - 7:6\\]
Fractional part of temperature upper limit. Total value = INTEGER + FRACTIONAL The encoding is an extension of the 2's complement encoding. 00: 0.0C 01: 0.25C 10: 0.5C 11: 0.75C For example: 000000001,00 = ( 1+0,00) = 1,00 000000000,11 = ( 0+0,75) = 0,75 000000000,10 = ( 0+0,50) = 0,50 000000000,01 = ( 0+0,25) = 0,25 000000000,00 = ( 0+0,00) = 0,00 111111111,11 = (-1+0,75) = -0,25 111111111,10 = (-1+0,50) = -0,50 111111111,01 = (-1+0,25) = -0,75 111111111,00 = (-1+0,00) = -1,00 111111110,11 = (-2+0,75) = -1,25"]
pub type FracW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INT` reader - 16:8\\]
Integer part (signed) of temperature upper limit. Total value = INTEGER + FRACTIONAL 2's complement encoding 0x100: Min value 0x1D8: -40C 0x1FF: -1C 0x00: 0C 0x1B: 27C 0x55: 85C 0xFF: Max value"]
pub type IntR = crate::FieldReader<u16>;
#[doc = "Field `INT` writer - 16:8\\]
Integer part (signed) of temperature upper limit. Total value = INTEGER + FRACTIONAL 2's complement encoding 0x100: Min value 0x1D8: -40C 0x1FF: -1C 0x00: 0C 0x1B: 27C 0x55: 85C 0xFF: Max value"]
pub type IntW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `RESERVED17` reader - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved17R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED17` writer - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved17W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Fractional part of temperature upper limit. Total value = INTEGER + FRACTIONAL The encoding is an extension of the 2's complement encoding. 00: 0.0C 01: 0.25C 10: 0.5C 11: 0.75C For example: 000000001,00 = ( 1+0,00) = 1,00 000000000,11 = ( 0+0,75) = 0,75 000000000,10 = ( 0+0,50) = 0,50 000000000,01 = ( 0+0,25) = 0,25 000000000,00 = ( 0+0,00) = 0,00 111111111,11 = (-1+0,75) = -0,25 111111111,10 = (-1+0,50) = -0,50 111111111,01 = (-1+0,25) = -0,75 111111111,00 = (-1+0,00) = -1,00 111111110,11 = (-2+0,75) = -1,25"]
    #[inline(always)]
    pub fn frac(&self) -> FracR {
        FracR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:16 - 16:8\\]
Integer part (signed) of temperature upper limit. Total value = INTEGER + FRACTIONAL 2's complement encoding 0x100: Min value 0x1D8: -40C 0x1FF: -1C 0x00: 0C 0x1B: 27C 0x55: 85C 0xFF: Max value"]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new(((self.bits >> 8) & 0x01ff) as u16)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> Reserved17R {
        Reserved17R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<TempulSpec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Fractional part of temperature upper limit. Total value = INTEGER + FRACTIONAL The encoding is an extension of the 2's complement encoding. 00: 0.0C 01: 0.25C 10: 0.5C 11: 0.75C For example: 000000001,00 = ( 1+0,00) = 1,00 000000000,11 = ( 0+0,75) = 0,75 000000000,10 = ( 0+0,50) = 0,50 000000000,01 = ( 0+0,25) = 0,25 000000000,00 = ( 0+0,00) = 0,00 111111111,11 = (-1+0,75) = -0,25 111111111,10 = (-1+0,50) = -0,50 111111111,01 = (-1+0,25) = -0,75 111111111,00 = (-1+0,00) = -1,00 111111110,11 = (-2+0,75) = -1,25"]
    #[inline(always)]
    #[must_use]
    pub fn frac(&mut self) -> FracW<TempulSpec> {
        FracW::new(self, 6)
    }
    #[doc = "Bits 8:16 - 16:8\\]
Integer part (signed) of temperature upper limit. Total value = INTEGER + FRACTIONAL 2's complement encoding 0x100: Min value 0x1D8: -40C 0x1FF: -1C 0x00: 0C 0x1B: 27C 0x55: 85C 0xFF: Max value"]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> IntW<TempulSpec> {
        IntW::new(self, 8)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved17(&mut self) -> Reserved17W<TempulSpec> {
        Reserved17W::new(self, 17)
    }
}
#[doc = "Temperature Upper Limit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tempul::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tempul::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TempulSpec;
impl crate::RegisterSpec for TempulSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tempul::R`](R) reader structure"]
impl crate::Readable for TempulSpec {}
#[doc = "`write(|w| ..)` method takes [`tempul::W`](W) writer structure"]
impl crate::Writable for TempulSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TEMPUL to value 0xffc0"]
impl crate::Resettable for TempulSpec {
    const RESET_VALUE: u32 = 0xffc0;
}
