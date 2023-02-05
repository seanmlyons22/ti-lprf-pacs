#[doc = "Register `SPIMCFG` reader"]
pub struct R(crate::R<SPIMCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPIMCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPIMCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPIMCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPIMCFG` writer"]
pub struct W(crate::W<SPIMCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPIMCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SPIMCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPIMCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POL` reader - 0:0\\]
Polarity of the SCLK signal. 0: SCLK is low when idle, first clock edge rises. 1: SCLK is high when idle, first clock edge falls."]
pub type POL_R = crate::BitReader<bool>;
#[doc = "Field `POL` writer - 0:0\\]
Polarity of the SCLK signal. 0: SCLK is low when idle, first clock edge rises. 1: SCLK is high when idle, first clock edge falls."]
pub type POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPIMCFG_SPEC, bool, O>;
#[doc = "Field `PHA` reader - 1:1\\]
Phase of the MOSI and MISO data signals. 0: Sample MISO at leading (odd) edges and shift MOSI at trailing (even) edges of SCLK. 1: Sample MISO at trailing (even) edges and shift MOSI at leading (odd) edges of SCLK."]
pub type PHA_R = crate::BitReader<bool>;
#[doc = "Field `PHA` writer - 1:1\\]
Phase of the MOSI and MISO data signals. 0: Sample MISO at leading (odd) edges and shift MOSI at trailing (even) edges of SCLK. 1: Sample MISO at trailing (even) edges and shift MOSI at leading (odd) edges of SCLK."]
pub type PHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPIMCFG_SPEC, bool, O>;
#[doc = "Field `DIV` reader - 7:2\\]
SCLK divider. Peripheral clock frequency division gives the SCLK clock frequency. The division factor equals (2 * (DIV+1)): 0x00: Divide by 2. 0x01: Divide by 4. 0x02: Divide by 6. ... 0x3F: Divide by 128."]
pub type DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIV` writer - 7:2\\]
SCLK divider. Peripheral clock frequency division gives the SCLK clock frequency. The division factor equals (2 * (DIV+1)): 0x00: Divide by 2. 0x01: Divide by 4. 0x02: Divide by 6. ... 0x3F: Divide by 128."]
pub type DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPIMCFG_SPEC, u8, u8, 6, O>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPIMCFG_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Polarity of the SCLK signal. 0: SCLK is low when idle, first clock edge rises. 1: SCLK is high when idle, first clock edge falls."]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Phase of the MOSI and MISO data signals. 0: Sample MISO at leading (odd) edges and shift MOSI at trailing (even) edges of SCLK. 1: Sample MISO at trailing (even) edges and shift MOSI at leading (odd) edges of SCLK."]
    #[inline(always)]
    pub fn pha(&self) -> PHA_R {
        PHA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - 7:2\\]
SCLK divider. Peripheral clock frequency division gives the SCLK clock frequency. The division factor equals (2 * (DIV+1)): 0x00: Divide by 2. 0x01: Divide by 4. 0x02: Divide by 6. ... 0x3F: Divide by 128."]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Polarity of the SCLK signal. 0: SCLK is low when idle, first clock edge rises. 1: SCLK is high when idle, first clock edge falls."]
    #[inline(always)]
    #[must_use]
    pub fn pol(&mut self) -> POL_W<0> {
        POL_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Phase of the MOSI and MISO data signals. 0: Sample MISO at leading (odd) edges and shift MOSI at trailing (even) edges of SCLK. 1: Sample MISO at trailing (even) edges and shift MOSI at leading (odd) edges of SCLK."]
    #[inline(always)]
    #[must_use]
    pub fn pha(&mut self) -> PHA_W<1> {
        PHA_W::new(self)
    }
    #[doc = "Bits 2:7 - 7:2\\]
SCLK divider. Peripheral clock frequency division gives the SCLK clock frequency. The division factor equals (2 * (DIV+1)): 0x00: Divide by 2. 0x01: Divide by 4. 0x02: Divide by 6. ... 0x3F: Divide by 128."]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<2> {
        DIV_W::new(self)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> RESERVED8_W<8> {
        RESERVED8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Master Configuration Write operation stalls until current transfer completes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spimcfg](index.html) module"]
pub struct SPIMCFG_SPEC;
impl crate::RegisterSpec for SPIMCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spimcfg::R](R) reader structure"]
impl crate::Readable for SPIMCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spimcfg::W](W) writer structure"]
impl crate::Writable for SPIMCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPIMCFG to value 0"]
impl crate::Resettable for SPIMCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
