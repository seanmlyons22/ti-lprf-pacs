#[doc = "Register `ALARMCNT` reader"]
pub type R = crate::R<AlarmcntSpec>;
#[doc = "Register `ALARMCNT` writer"]
pub type W = crate::W<AlarmcntSpec>;
#[doc = "Field `ALARM_THR` reader - 7:0\\]
Alarm detection threshold for the repeating pattern detectors on each FRO. An FRO 'alarm event' is declared when a repeating pattern (of up to four samples length) is detected continuously for the number of samples defined by this field's value. Reset value 0xFF should keep the number of 'alarm events' to a manageable level."]
pub type AlarmThrR = crate::FieldReader;
#[doc = "Field `ALARM_THR` writer - 7:0\\]
Alarm detection threshold for the repeating pattern detectors on each FRO. An FRO 'alarm event' is declared when a repeating pattern (of up to four samples length) is detected continuously for the number of samples defined by this field's value. Reset value 0xFF should keep the number of 'alarm events' to a manageable level."]
pub type AlarmThrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader;
#[doc = "Field `SHUTDOWN_THR` reader - 20:16\\]
Threshold setting for generating IRQFLAGSTAT.SHUTDOWN_OVF interrupt. The interrupt is triggered when SHUTDOWN_CNT value exceeds this bit field."]
pub type ShutdownThrR = crate::FieldReader;
#[doc = "Field `SHUTDOWN_THR` writer - 20:16\\]
Threshold setting for generating IRQFLAGSTAT.SHUTDOWN_OVF interrupt. The interrupt is triggered when SHUTDOWN_CNT value exceeds this bit field."]
pub type ShutdownThrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RESERVED21` reader - 23:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved21R = crate::FieldReader;
#[doc = "Field `SHUTDOWN_CNT` reader - 29:24\\]
Read-only, indicates the number of '1' bits in ALARMSTOP register. The maximum value equals the number of FROs."]
pub type ShutdownCntR = crate::FieldReader;
#[doc = "Field `SHUTDOWN_CNT` writer - 29:24\\]
Read-only, indicates the number of '1' bits in ALARMSTOP register. The maximum value equals the number of FROs."]
pub type ShutdownCntW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RESERVED30` reader - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved30R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Alarm detection threshold for the repeating pattern detectors on each FRO. An FRO 'alarm event' is declared when a repeating pattern (of up to four samples length) is detected continuously for the number of samples defined by this field's value. Reset value 0xFF should keep the number of 'alarm events' to a manageable level."]
    #[inline(always)]
    pub fn alarm_thr(&self) -> AlarmThrR {
        AlarmThrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Threshold setting for generating IRQFLAGSTAT.SHUTDOWN_OVF interrupt. The interrupt is triggered when SHUTDOWN_CNT value exceeds this bit field."]
    #[inline(always)]
    pub fn shutdown_thr(&self) -> ShutdownThrR {
        ShutdownThrR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved21(&self) -> Reserved21R {
        Reserved21R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Read-only, indicates the number of '1' bits in ALARMSTOP register. The maximum value equals the number of FROs."]
    #[inline(always)]
    pub fn shutdown_cnt(&self) -> ShutdownCntR {
        ShutdownCntR::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved30(&self) -> Reserved30R {
        Reserved30R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Alarm detection threshold for the repeating pattern detectors on each FRO. An FRO 'alarm event' is declared when a repeating pattern (of up to four samples length) is detected continuously for the number of samples defined by this field's value. Reset value 0xFF should keep the number of 'alarm events' to a manageable level."]
    #[inline(always)]
    #[must_use]
    pub fn alarm_thr(&mut self) -> AlarmThrW<AlarmcntSpec> {
        AlarmThrW::new(self, 0)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Threshold setting for generating IRQFLAGSTAT.SHUTDOWN_OVF interrupt. The interrupt is triggered when SHUTDOWN_CNT value exceeds this bit field."]
    #[inline(always)]
    #[must_use]
    pub fn shutdown_thr(&mut self) -> ShutdownThrW<AlarmcntSpec> {
        ShutdownThrW::new(self, 16)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Read-only, indicates the number of '1' bits in ALARMSTOP register. The maximum value equals the number of FROs."]
    #[inline(always)]
    #[must_use]
    pub fn shutdown_cnt(&mut self) -> ShutdownCntW<AlarmcntSpec> {
        ShutdownCntW::new(self, 24)
    }
}
#[doc = "Alarm Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alarmcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarmcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlarmcntSpec;
impl crate::RegisterSpec for AlarmcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alarmcnt::R`](R) reader structure"]
impl crate::Readable for AlarmcntSpec {}
#[doc = "`write(|w| ..)` method takes [`alarmcnt::W`](W) writer structure"]
impl crate::Writable for AlarmcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALARMCNT to value 0xff"]
impl crate::Resettable for AlarmcntSpec {
    const RESET_VALUE: u32 = 0xff;
}
