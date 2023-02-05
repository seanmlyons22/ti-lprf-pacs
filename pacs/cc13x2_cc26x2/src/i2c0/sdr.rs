#[doc = "Register `SDR` reader"]
pub struct R(crate::R<SDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDR` writer"]
pub struct W(crate::W<SDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDR_SPEC>;
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
impl From<crate::W<SDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - 7:0\\]
Data for transfer This field contains the data for transfer during a slave receive or transmit operation. When written the register data is used as transmit data. When read, this register returns the last data received. Data is stored until next update, either by a system write for transmit or by an external master for receive."]
pub type DATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA` writer - 7:0\\]
Data for transfer This field contains the data for transfer during a slave receive or transmit operation. When written the register data is used as transmit data. When read, this register returns the last data received. Data is stored until next update, either by a system write for transmit or by an external master for receive."]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDR_SPEC, u8, u8, 8, O>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDR_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Data for transfer This field contains the data for transfer during a slave receive or transmit operation. When written the register data is used as transmit data. When read, this register returns the last data received. Data is stored until next update, either by a system write for transmit or by an external master for receive."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Data for transfer This field contains the data for transfer during a slave receive or transmit operation. When written the register data is used as transmit data. When read, this register returns the last data received. Data is stored until next update, either by a system write for transmit or by an external master for receive."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
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
#[doc = "Slave Data This register contains the data to be transmitted when in the Slave Transmit state, and the data received when in the Slave Receive state.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdr](index.html) module"]
pub struct SDR_SPEC;
impl crate::RegisterSpec for SDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdr::R](R) reader structure"]
impl crate::Readable for SDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdr::W](W) writer structure"]
impl crate::Writable for SDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDR to value 0"]
impl crate::Resettable for SDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
