#[doc = "Register `TCR` reader"]
pub type R = crate::R<TcrSpec>;
#[doc = "Register `TCR` writer"]
pub type W = crate::W<TcrSpec>;
#[doc = "Field `ITMENA` reader - 0:0\\]
Enables the ITM"]
pub type ItmenaR = crate::BitReader;
#[doc = "Field `ITMENA` writer - 0:0\\]
Enables the ITM"]
pub type ItmenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSENA` reader - 1:1\\]
Enables Local timestamp generation"]
pub type TsenaR = crate::BitReader;
#[doc = "Field `TSENA` writer - 1:1\\]
Enables Local timestamp generation"]
pub type TsenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCENA` reader - 2:2\\]
Enables Synchronization packet transmission for a synchronous TPIU"]
pub type SyncenaR = crate::BitReader;
#[doc = "Field `SYNCENA` writer - 2:2\\]
Enables Synchronization packet transmission for a synchronous TPIU"]
pub type SyncenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXENA` reader - 3:3\\]
Enables forwarding of hardware event packet from the DWT unit to the ITM for output to the TPIU"]
pub type TxenaR = crate::BitReader;
#[doc = "Field `TXENA` writer - 3:3\\]
Enables forwarding of hardware event packet from the DWT unit to the ITM for output to the TPIU"]
pub type TxenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWOENA` reader - 4:4\\]
Enables asynchronous clocking of the timestamp counter"]
pub type SwoenaR = crate::BitReader;
#[doc = "Field `SWOENA` writer - 4:4\\]
Enables asynchronous clocking of the timestamp counter"]
pub type SwoenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLENA` reader - 5:5\\]
Stall the PE to guarantee delivery of Data Trace packets."]
pub type StallenaR = crate::BitReader;
#[doc = "Field `STALLENA` writer - 5:5\\]
Stall the PE to guarantee delivery of Data Trace packets."]
pub type StallenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED6` reader - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader;
#[doc = "Field `TSPrescale` reader - 9:8\\]
Local timestamp prescaler, used with the trace packet reference clock"]
pub type TsprescaleR = crate::FieldReader;
#[doc = "Field `TSPrescale` writer - 9:8\\]
Local timestamp prescaler, used with the trace packet reference clock"]
pub type TsprescaleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GTSFREQ` reader - 11:10\\]
Defines how often the ITM generates a global timestamp, based on the global timestamp clock frequency, or disables generation of global timestamps"]
pub type GtsfreqR = crate::FieldReader;
#[doc = "Field `GTSFREQ` writer - 11:10\\]
Defines how often the ITM generates a global timestamp, based on the global timestamp clock frequency, or disables generation of global timestamps"]
pub type GtsfreqW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESERVED12` reader - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12R = crate::FieldReader;
#[doc = "Field `TraceBusID` reader - 22:16\\]
Identifier for multi-source trace stream formatting. If multi-source trace is in use, the debugger must write a unique non-zero trace ID value to this field"]
pub type TraceBusIdR = crate::FieldReader;
#[doc = "Field `TraceBusID` writer - 22:16\\]
Identifier for multi-source trace stream formatting. If multi-source trace is in use, the debugger must write a unique non-zero trace ID value to this field"]
pub type TraceBusIdW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `BUSY` reader - 23:23\\]
Indicates whether the ITM is currently processing events"]
pub type BusyR = crate::BitReader;
#[doc = "Field `RESERVED24` reader - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables the ITM"]
    #[inline(always)]
    pub fn itmena(&self) -> ItmenaR {
        ItmenaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enables Local timestamp generation"]
    #[inline(always)]
    pub fn tsena(&self) -> TsenaR {
        TsenaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enables Synchronization packet transmission for a synchronous TPIU"]
    #[inline(always)]
    pub fn syncena(&self) -> SyncenaR {
        SyncenaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Enables forwarding of hardware event packet from the DWT unit to the ITM for output to the TPIU"]
    #[inline(always)]
    pub fn txena(&self) -> TxenaR {
        TxenaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Enables asynchronous clocking of the timestamp counter"]
    #[inline(always)]
    pub fn swoena(&self) -> SwoenaR {
        SwoenaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Stall the PE to guarantee delivery of Data Trace packets."]
    #[inline(always)]
    pub fn stallena(&self) -> StallenaR {
        StallenaR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Local timestamp prescaler, used with the trace packet reference clock"]
    #[inline(always)]
    pub fn tsprescale(&self) -> TsprescaleR {
        TsprescaleR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Defines how often the ITM generates a global timestamp, based on the global timestamp clock frequency, or disables generation of global timestamps"]
    #[inline(always)]
    pub fn gtsfreq(&self) -> GtsfreqR {
        GtsfreqR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Identifier for multi-source trace stream formatting. If multi-source trace is in use, the debugger must write a unique non-zero trace ID value to this field"]
    #[inline(always)]
    pub fn trace_bus_id(&self) -> TraceBusIdR {
        TraceBusIdR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - 23:23\\]
Indicates whether the ITM is currently processing events"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> Reserved24R {
        Reserved24R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enables the ITM"]
    #[inline(always)]
    #[must_use]
    pub fn itmena(&mut self) -> ItmenaW<TcrSpec> {
        ItmenaW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enables Local timestamp generation"]
    #[inline(always)]
    #[must_use]
    pub fn tsena(&mut self) -> TsenaW<TcrSpec> {
        TsenaW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Enables Synchronization packet transmission for a synchronous TPIU"]
    #[inline(always)]
    #[must_use]
    pub fn syncena(&mut self) -> SyncenaW<TcrSpec> {
        SyncenaW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Enables forwarding of hardware event packet from the DWT unit to the ITM for output to the TPIU"]
    #[inline(always)]
    #[must_use]
    pub fn txena(&mut self) -> TxenaW<TcrSpec> {
        TxenaW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Enables asynchronous clocking of the timestamp counter"]
    #[inline(always)]
    #[must_use]
    pub fn swoena(&mut self) -> SwoenaW<TcrSpec> {
        SwoenaW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Stall the PE to guarantee delivery of Data Trace packets."]
    #[inline(always)]
    #[must_use]
    pub fn stallena(&mut self) -> StallenaW<TcrSpec> {
        StallenaW::new(self, 5)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Local timestamp prescaler, used with the trace packet reference clock"]
    #[inline(always)]
    #[must_use]
    pub fn tsprescale(&mut self) -> TsprescaleW<TcrSpec> {
        TsprescaleW::new(self, 8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Defines how often the ITM generates a global timestamp, based on the global timestamp clock frequency, or disables generation of global timestamps"]
    #[inline(always)]
    #[must_use]
    pub fn gtsfreq(&mut self) -> GtsfreqW<TcrSpec> {
        GtsfreqW::new(self, 10)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Identifier for multi-source trace stream formatting. If multi-source trace is in use, the debugger must write a unique non-zero trace ID value to this field"]
    #[inline(always)]
    #[must_use]
    pub fn trace_bus_id(&mut self) -> TraceBusIdW<TcrSpec> {
        TraceBusIdW::new(self, 16)
    }
}
#[doc = "Configures and controls transfers through the ITM interface\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcrSpec;
impl crate::RegisterSpec for TcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcr::R`](R) reader structure"]
impl crate::Readable for TcrSpec {}
#[doc = "`write(|w| ..)` method takes [`tcr::W`](W) writer structure"]
impl crate::Writable for TcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TCR to value 0"]
impl crate::Resettable for TcrSpec {
    const RESET_VALUE: u32 = 0;
}
