#[doc = "Register `SSTAT` reader"]
pub struct R(crate::R<SSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSTAT` writer"]
pub struct W(crate::W<SSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSTAT_SPEC>;
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
impl From<crate::W<SSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RREQ` reader - 0:0\\]
Receive request 0: No outstanding receive data 1: The I2C controller has outstanding receive data from the I2C master and is using clock stretching to delay the master until data has been read from the SDR register."]
pub type RREQ_R = crate::BitReader<bool>;
#[doc = "Field `RREQ` writer - 0:0\\]
Receive request 0: No outstanding receive data 1: The I2C controller has outstanding receive data from the I2C master and is using clock stretching to delay the master until data has been read from the SDR register."]
pub type RREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSTAT_SPEC, bool, O>;
#[doc = "Field `TREQ` reader - 1:1\\]
Transmit request 0: No outstanding transmit request. 1: The I2C controller has been addressed as a slave transmitter and is using clock stretching to delay the master until data has been written to the SDR register."]
pub type TREQ_R = crate::BitReader<bool>;
#[doc = "Field `TREQ` writer - 1:1\\]
Transmit request 0: No outstanding transmit request. 1: The I2C controller has been addressed as a slave transmitter and is using clock stretching to delay the master until data has been written to the SDR register."]
pub type TREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSTAT_SPEC, bool, O>;
#[doc = "Field `FBR` reader - 2:2\\]
First byte received 0: The first byte has not been received. 1: The first byte following the slave's own address has been received. This bit is only valid when the RREQ bit is set and is automatically cleared when data has been read from the SDR register. Note: This bit is not used for slave transmit operations."]
pub type FBR_R = crate::BitReader<bool>;
#[doc = "Field `FBR` writer - 2:2\\]
First byte received 0: The first byte has not been received. 1: The first byte following the slave's own address has been received. This bit is only valid when the RREQ bit is set and is automatically cleared when data has been read from the SDR register. Note: This bit is not used for slave transmit operations."]
pub type FBR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSTAT_SPEC, bool, O>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SSTAT_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Receive request 0: No outstanding receive data 1: The I2C controller has outstanding receive data from the I2C master and is using clock stretching to delay the master until data has been read from the SDR register."]
    #[inline(always)]
    pub fn rreq(&self) -> RREQ_R {
        RREQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Transmit request 0: No outstanding transmit request. 1: The I2C controller has been addressed as a slave transmitter and is using clock stretching to delay the master until data has been written to the SDR register."]
    #[inline(always)]
    pub fn treq(&self) -> TREQ_R {
        TREQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
First byte received 0: The first byte has not been received. 1: The first byte following the slave's own address has been received. This bit is only valid when the RREQ bit is set and is automatically cleared when data has been read from the SDR register. Note: This bit is not used for slave transmit operations."]
    #[inline(always)]
    pub fn fbr(&self) -> FBR_R {
        FBR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Receive request 0: No outstanding receive data 1: The I2C controller has outstanding receive data from the I2C master and is using clock stretching to delay the master until data has been read from the SDR register."]
    #[inline(always)]
    #[must_use]
    pub fn rreq(&mut self) -> RREQ_W<0> {
        RREQ_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Transmit request 0: No outstanding transmit request. 1: The I2C controller has been addressed as a slave transmitter and is using clock stretching to delay the master until data has been written to the SDR register."]
    #[inline(always)]
    #[must_use]
    pub fn treq(&mut self) -> TREQ_W<1> {
        TREQ_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
First byte received 0: The first byte has not been received. 1: The first byte following the slave's own address has been received. This bit is only valid when the RREQ bit is set and is automatically cleared when data has been read from the SDR register. Note: This bit is not used for slave transmit operations."]
    #[inline(always)]
    #[must_use]
    pub fn fbr(&mut self) -> FBR_W<2> {
        FBR_W::new(self)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<3> {
        RESERVED3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Status Note: This register shares address with SCTL, meaning that this register functions as a control register when written, and a status register when read.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sstat](index.html) module"]
pub struct SSTAT_SPEC;
impl crate::RegisterSpec for SSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sstat::R](R) reader structure"]
impl crate::Readable for SSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sstat::W](W) writer structure"]
impl crate::Writable for SSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSTAT to value 0"]
impl crate::Resettable for SSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
