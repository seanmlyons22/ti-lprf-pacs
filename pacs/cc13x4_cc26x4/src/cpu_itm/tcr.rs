#[doc = "Register `TCR` reader"]
pub struct R(crate::R<TCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCR` writer"]
pub struct W(crate::W<TCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCR_SPEC>;
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
impl From<crate::W<TCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ITMENA` reader - 0:0\\]
Enables the ITM"]
pub type ITMENA_R = crate::BitReader<bool>;
#[doc = "Field `ITMENA` writer - 0:0\\]
Enables the ITM"]
pub type ITMENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCR_SPEC, bool, O>;
#[doc = "Field `TSENA` reader - 1:1\\]
Enables Local timestamp generation"]
pub type TSENA_R = crate::BitReader<bool>;
#[doc = "Field `TSENA` writer - 1:1\\]
Enables Local timestamp generation"]
pub type TSENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCR_SPEC, bool, O>;
#[doc = "Field `SYNCENA` reader - 2:2\\]
Enables Synchronization packet transmission for a synchronous TPIU"]
pub type SYNCENA_R = crate::BitReader<bool>;
#[doc = "Field `SYNCENA` writer - 2:2\\]
Enables Synchronization packet transmission for a synchronous TPIU"]
pub type SYNCENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCR_SPEC, bool, O>;
#[doc = "Field `TXENA` reader - 3:3\\]
Enables forwarding of hardware event packet from the DWT unit to the ITM for output to the TPIU"]
pub type TXENA_R = crate::BitReader<bool>;
#[doc = "Field `TXENA` writer - 3:3\\]
Enables forwarding of hardware event packet from the DWT unit to the ITM for output to the TPIU"]
pub type TXENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCR_SPEC, bool, O>;
#[doc = "Field `SWOENA` reader - 4:4\\]
Enables asynchronous clocking of the timestamp counter"]
pub type SWOENA_R = crate::BitReader<bool>;
#[doc = "Field `SWOENA` writer - 4:4\\]
Enables asynchronous clocking of the timestamp counter"]
pub type SWOENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCR_SPEC, bool, O>;
#[doc = "Field `STALLENA` reader - 5:5\\]
Stall the PE to guarantee delivery of Data Trace packets."]
pub type STALLENA_R = crate::BitReader<bool>;
#[doc = "Field `STALLENA` writer - 5:5\\]
Stall the PE to guarantee delivery of Data Trace packets."]
pub type STALLENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCR_SPEC, bool, O>;
#[doc = "Field `RESERVED6` reader - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED6` writer - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TSPrescale` reader - 9:8\\]
Local timestamp prescaler, used with the trace packet reference clock"]
pub type TSPRESCALE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TSPrescale` writer - 9:8\\]
Local timestamp prescaler, used with the trace packet reference clock"]
pub type TSPRESCALE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `GTSFREQ` reader - 11:10\\]
Defines how often the ITM generates a global timestamp, based on the global timestamp clock frequency, or disables generation of global timestamps"]
pub type GTSFREQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GTSFREQ` writer - 11:10\\]
Defines how often the ITM generates a global timestamp, based on the global timestamp clock frequency, or disables generation of global timestamps"]
pub type GTSFREQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `RESERVED12` reader - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED12` writer - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `TraceBusID` reader - 22:16\\]
Identifier for multi-source trace stream formatting. If multi-source trace is in use, the debugger must write a unique non-zero trace ID value to this field"]
pub type TRACE_BUS_ID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TraceBusID` writer - 22:16\\]
Identifier for multi-source trace stream formatting. If multi-source trace is in use, the debugger must write a unique non-zero trace ID value to this field"]
pub type TRACE_BUS_ID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCR_SPEC, u8, u8, 7, O>;
#[doc = "Field `BUSY` reader - 23:23\\]
Indicates whether the ITM is currently processing events"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` writer - 23:23\\]
Indicates whether the ITM is currently processing events"]
pub type BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCR_SPEC, bool, O>;
#[doc = "Field `RESERVED24` reader - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED24_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED24` writer - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED24_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables the ITM"]
    #[inline(always)]
    pub fn itmena(&self) -> ITMENA_R {
        ITMENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enables Local timestamp generation"]
    #[inline(always)]
    pub fn tsena(&self) -> TSENA_R {
        TSENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enables Synchronization packet transmission for a synchronous TPIU"]
    #[inline(always)]
    pub fn syncena(&self) -> SYNCENA_R {
        SYNCENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Enables forwarding of hardware event packet from the DWT unit to the ITM for output to the TPIU"]
    #[inline(always)]
    pub fn txena(&self) -> TXENA_R {
        TXENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Enables asynchronous clocking of the timestamp counter"]
    #[inline(always)]
    pub fn swoena(&self) -> SWOENA_R {
        SWOENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Stall the PE to guarantee delivery of Data Trace packets."]
    #[inline(always)]
    pub fn stallena(&self) -> STALLENA_R {
        STALLENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Local timestamp prescaler, used with the trace packet reference clock"]
    #[inline(always)]
    pub fn tsprescale(&self) -> TSPRESCALE_R {
        TSPRESCALE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Defines how often the ITM generates a global timestamp, based on the global timestamp clock frequency, or disables generation of global timestamps"]
    #[inline(always)]
    pub fn gtsfreq(&self) -> GTSFREQ_R {
        GTSFREQ_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Identifier for multi-source trace stream formatting. If multi-source trace is in use, the debugger must write a unique non-zero trace ID value to this field"]
    #[inline(always)]
    pub fn trace_bus_id(&self) -> TRACE_BUS_ID_R {
        TRACE_BUS_ID_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - 23:23\\]
Indicates whether the ITM is currently processing events"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enables the ITM"]
    #[inline(always)]
    #[must_use]
    pub fn itmena(&mut self) -> ITMENA_W<0> {
        ITMENA_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Enables Local timestamp generation"]
    #[inline(always)]
    #[must_use]
    pub fn tsena(&mut self) -> TSENA_W<1> {
        TSENA_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Enables Synchronization packet transmission for a synchronous TPIU"]
    #[inline(always)]
    #[must_use]
    pub fn syncena(&mut self) -> SYNCENA_W<2> {
        SYNCENA_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Enables forwarding of hardware event packet from the DWT unit to the ITM for output to the TPIU"]
    #[inline(always)]
    #[must_use]
    pub fn txena(&mut self) -> TXENA_W<3> {
        TXENA_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Enables asynchronous clocking of the timestamp counter"]
    #[inline(always)]
    #[must_use]
    pub fn swoena(&mut self) -> SWOENA_W<4> {
        SWOENA_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Stall the PE to guarantee delivery of Data Trace packets."]
    #[inline(always)]
    #[must_use]
    pub fn stallena(&mut self) -> STALLENA_W<5> {
        STALLENA_W::new(self)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<6> {
        RESERVED6_W::new(self)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Local timestamp prescaler, used with the trace packet reference clock"]
    #[inline(always)]
    #[must_use]
    pub fn tsprescale(&mut self) -> TSPRESCALE_W<8> {
        TSPRESCALE_W::new(self)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Defines how often the ITM generates a global timestamp, based on the global timestamp clock frequency, or disables generation of global timestamps"]
    #[inline(always)]
    #[must_use]
    pub fn gtsfreq(&mut self) -> GTSFREQ_W<10> {
        GTSFREQ_W::new(self)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> RESERVED12_W<12> {
        RESERVED12_W::new(self)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Identifier for multi-source trace stream formatting. If multi-source trace is in use, the debugger must write a unique non-zero trace ID value to this field"]
    #[inline(always)]
    #[must_use]
    pub fn trace_bus_id(&mut self) -> TRACE_BUS_ID_W<16> {
        TRACE_BUS_ID_W::new(self)
    }
    #[doc = "Bit 23 - 23:23\\]
Indicates whether the ITM is currently processing events"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BUSY_W<23> {
        BUSY_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved24(&mut self) -> RESERVED24_W<24> {
        RESERVED24_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configures and controls transfers through the ITM interface\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcr](index.html) module"]
pub struct TCR_SPEC;
impl crate::RegisterSpec for TCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcr::R](R) reader structure"]
impl crate::Readable for TCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcr::W](W) writer structure"]
impl crate::Writable for TCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCR to value 0"]
impl crate::Resettable for TCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
