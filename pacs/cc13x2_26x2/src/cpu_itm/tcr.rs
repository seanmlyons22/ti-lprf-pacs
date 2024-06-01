#[doc = "Register `TCR` reader"]
pub type R = crate::R<TcrSpec>;
#[doc = "Register `TCR` writer"]
pub type W = crate::W<TcrSpec>;
#[doc = "Field `ITMENA` reader - 0:0\\]
Enables ITM. This is the master enable, and must be set before ITM Stimulus and Trace Enable registers can be written."]
pub type ItmenaR = crate::BitReader;
#[doc = "Field `ITMENA` writer - 0:0\\]
Enables ITM. This is the master enable, and must be set before ITM Stimulus and Trace Enable registers can be written."]
pub type ItmenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSENA` reader - 1:1\\]
Enables differential timestamps. Differential timestamps are emitted when a packet is written to the FIFO with a non-zero timestamp counter, and when the timestamp counter overflows. Timestamps are emitted during idle times after a fixed number of two million cycles. This provides a time reference for packets and inter-packet gaps. If SWOENA (bit \\[4\\]) is set, timestamps are triggered by activity on the internal trace bus only. In this case there is no regular timestamp output when the ITM is idle."]
pub type TsenaR = crate::BitReader;
#[doc = "Field `TSENA` writer - 1:1\\]
Enables differential timestamps. Differential timestamps are emitted when a packet is written to the FIFO with a non-zero timestamp counter, and when the timestamp counter overflows. Timestamps are emitted during idle times after a fixed number of two million cycles. This provides a time reference for packets and inter-packet gaps. If SWOENA (bit \\[4\\]) is set, timestamps are triggered by activity on the internal trace bus only. In this case there is no regular timestamp output when the ITM is idle."]
pub type TsenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCENA` reader - 2:2\\]
Enables synchronization packet transmission for a synchronous TPIU. CPU_DWT:CTRL.SYNCTAP must be configured for the correct synchronization speed."]
pub type SyncenaR = crate::BitReader;
#[doc = "Field `SYNCENA` writer - 2:2\\]
Enables synchronization packet transmission for a synchronous TPIU. CPU_DWT:CTRL.SYNCTAP must be configured for the correct synchronization speed."]
pub type SyncenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DWTENA` reader - 3:3\\]
Enables the DWT stimulus (hardware event packet emission to the TPIU from the DWT)"]
pub type DwtenaR = crate::BitReader;
#[doc = "Field `DWTENA` writer - 3:3\\]
Enables the DWT stimulus (hardware event packet emission to the TPIU from the DWT)"]
pub type DwtenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWOENA` reader - 4:4\\]
Enables asynchronous clocking of the timestamp counter (when TSENA = 1). If TSENA = 0, writing this bit to 1 does not enable asynchronous clocking of the timestamp counter. 0x0: Mode disabled. Timestamp counter uses system clock from the core and counts continuously. 0x1: Timestamp counter uses lineout (data related) clock from TPIU interface. The timestamp counter is held in reset while the output line is idle."]
pub type SwoenaR = crate::BitReader;
#[doc = "Field `SWOENA` writer - 4:4\\]
Enables asynchronous clocking of the timestamp counter (when TSENA = 1). If TSENA = 0, writing this bit to 1 does not enable asynchronous clocking of the timestamp counter. 0x0: Mode disabled. Timestamp counter uses system clock from the core and counts continuously. 0x1: Timestamp counter uses lineout (data related) clock from TPIU interface. The timestamp counter is held in reset while the output line is idle."]
pub type SwoenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED5` reader - 7:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved5R = crate::FieldReader;
#[doc = "Field `RESERVED5` writer - 7:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "9:8\\]
Timestamp prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tsprescale {
    #[doc = "3: Divide by 64"]
    Div64 = 3,
    #[doc = "2: Divide by 16"]
    Div16 = 2,
    #[doc = "1: Divide by 4"]
    Div4 = 1,
    #[doc = "0: No prescaling"]
    Noprescaling = 0,
}
impl From<Tsprescale> for u8 {
    #[inline(always)]
    fn from(variant: Tsprescale) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tsprescale {
    type Ux = u8;
}
impl crate::IsEnum for Tsprescale {}
#[doc = "Field `TSPRESCALE` reader - 9:8\\]
Timestamp prescaler"]
pub type TsprescaleR = crate::FieldReader<Tsprescale>;
impl TsprescaleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsprescale {
        match self.bits {
            3 => Tsprescale::Div64,
            2 => Tsprescale::Div16,
            1 => Tsprescale::Div4,
            0 => Tsprescale::Noprescaling,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Tsprescale::Div64
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Tsprescale::Div16
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Tsprescale::Div4
    }
    #[doc = "No prescaling"]
    #[inline(always)]
    pub fn is_noprescaling(&self) -> bool {
        *self == Tsprescale::Noprescaling
    }
}
#[doc = "Field `TSPRESCALE` writer - 9:8\\]
Timestamp prescaler"]
pub type TsprescaleW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tsprescale, crate::Safe>;
impl<'a, REG> TsprescaleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Tsprescale::Div64)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Tsprescale::Div16)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Tsprescale::Div4)
    }
    #[doc = "No prescaling"]
    #[inline(always)]
    pub fn noprescaling(self) -> &'a mut crate::W<REG> {
        self.variant(Tsprescale::Noprescaling)
    }
}
#[doc = "Field `RESERVED10` reader - 15:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved10R = crate::FieldReader;
#[doc = "Field `RESERVED10` writer - 15:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved10W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ATBID` reader - 22:16\\]
Trace Bus ID for CoreSight system. Optional identifier for multi-source trace stream formatting. If multi-source trace is in use, this field must be written with a non-zero value."]
pub type AtbidR = crate::FieldReader;
#[doc = "Field `ATBID` writer - 22:16\\]
Trace Bus ID for CoreSight system. Optional identifier for multi-source trace stream formatting. If multi-source trace is in use, this field must be written with a non-zero value."]
pub type AtbidW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `BUSY` reader - 23:23\\]
Set when ITM events present and being drained."]
pub type BusyR = crate::BitReader;
#[doc = "Field `BUSY` writer - 23:23\\]
Set when ITM events present and being drained."]
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED24` reader - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24R = crate::FieldReader;
#[doc = "Field `RESERVED24` writer - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables ITM. This is the master enable, and must be set before ITM Stimulus and Trace Enable registers can be written."]
    #[inline(always)]
    pub fn itmena(&self) -> ItmenaR {
        ItmenaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enables differential timestamps. Differential timestamps are emitted when a packet is written to the FIFO with a non-zero timestamp counter, and when the timestamp counter overflows. Timestamps are emitted during idle times after a fixed number of two million cycles. This provides a time reference for packets and inter-packet gaps. If SWOENA (bit \\[4\\]) is set, timestamps are triggered by activity on the internal trace bus only. In this case there is no regular timestamp output when the ITM is idle."]
    #[inline(always)]
    pub fn tsena(&self) -> TsenaR {
        TsenaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enables synchronization packet transmission for a synchronous TPIU. CPU_DWT:CTRL.SYNCTAP must be configured for the correct synchronization speed."]
    #[inline(always)]
    pub fn syncena(&self) -> SyncenaR {
        SyncenaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Enables the DWT stimulus (hardware event packet emission to the TPIU from the DWT)"]
    #[inline(always)]
    pub fn dwtena(&self) -> DwtenaR {
        DwtenaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Enables asynchronous clocking of the timestamp counter (when TSENA = 1). If TSENA = 0, writing this bit to 1 does not enable asynchronous clocking of the timestamp counter. 0x0: Mode disabled. Timestamp counter uses system clock from the core and counts continuously. 0x1: Timestamp counter uses lineout (data related) clock from TPIU interface. The timestamp counter is held in reset while the output line is idle."]
    #[inline(always)]
    pub fn swoena(&self) -> SwoenaR {
        SwoenaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Timestamp prescaler"]
    #[inline(always)]
    pub fn tsprescale(&self) -> TsprescaleR {
        TsprescaleR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> Reserved10R {
        Reserved10R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Trace Bus ID for CoreSight system. Optional identifier for multi-source trace stream formatting. If multi-source trace is in use, this field must be written with a non-zero value."]
    #[inline(always)]
    pub fn atbid(&self) -> AtbidR {
        AtbidR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - 23:23\\]
Set when ITM events present and being drained."]
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
Enables ITM. This is the master enable, and must be set before ITM Stimulus and Trace Enable registers can be written."]
    #[inline(always)]
    #[must_use]
    pub fn itmena(&mut self) -> ItmenaW<TcrSpec> {
        ItmenaW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enables differential timestamps. Differential timestamps are emitted when a packet is written to the FIFO with a non-zero timestamp counter, and when the timestamp counter overflows. Timestamps are emitted during idle times after a fixed number of two million cycles. This provides a time reference for packets and inter-packet gaps. If SWOENA (bit \\[4\\]) is set, timestamps are triggered by activity on the internal trace bus only. In this case there is no regular timestamp output when the ITM is idle."]
    #[inline(always)]
    #[must_use]
    pub fn tsena(&mut self) -> TsenaW<TcrSpec> {
        TsenaW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Enables synchronization packet transmission for a synchronous TPIU. CPU_DWT:CTRL.SYNCTAP must be configured for the correct synchronization speed."]
    #[inline(always)]
    #[must_use]
    pub fn syncena(&mut self) -> SyncenaW<TcrSpec> {
        SyncenaW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Enables the DWT stimulus (hardware event packet emission to the TPIU from the DWT)"]
    #[inline(always)]
    #[must_use]
    pub fn dwtena(&mut self) -> DwtenaW<TcrSpec> {
        DwtenaW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Enables asynchronous clocking of the timestamp counter (when TSENA = 1). If TSENA = 0, writing this bit to 1 does not enable asynchronous clocking of the timestamp counter. 0x0: Mode disabled. Timestamp counter uses system clock from the core and counts continuously. 0x1: Timestamp counter uses lineout (data related) clock from TPIU interface. The timestamp counter is held in reset while the output line is idle."]
    #[inline(always)]
    #[must_use]
    pub fn swoena(&mut self) -> SwoenaW<TcrSpec> {
        SwoenaW::new(self, 4)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> Reserved5W<TcrSpec> {
        Reserved5W::new(self, 5)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Timestamp prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn tsprescale(&mut self) -> TsprescaleW<TcrSpec> {
        TsprescaleW::new(self, 8)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved10(&mut self) -> Reserved10W<TcrSpec> {
        Reserved10W::new(self, 10)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Trace Bus ID for CoreSight system. Optional identifier for multi-source trace stream formatting. If multi-source trace is in use, this field must be written with a non-zero value."]
    #[inline(always)]
    #[must_use]
    pub fn atbid(&mut self) -> AtbidW<TcrSpec> {
        AtbidW::new(self, 16)
    }
    #[doc = "Bit 23 - 23:23\\]
Set when ITM events present and being drained."]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BusyW<TcrSpec> {
        BusyW::new(self, 23)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved24(&mut self) -> Reserved24W<TcrSpec> {
        Reserved24W::new(self, 24)
    }
}
#[doc = "Trace Control Use this register to configure and control ITM transfers. This register can only be written in privilege mode. DWT is not enabled in the ITM block. However, DWT stimulus entry into the FIFO is controlled by DWTENA. If DWT requires timestamping, the TSENA bit must be set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
