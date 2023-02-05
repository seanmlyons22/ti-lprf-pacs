#[doc = "Register `ALARMCNT` reader"]
pub struct R(crate::R<ALARMCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALARMCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALARMCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALARMCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALARMCNT` writer"]
pub struct W(crate::W<ALARMCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALARMCNT_SPEC>;
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
impl From<crate::W<ALARMCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALARMCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALARM_THR` reader - 7:0\\]
Alarm detection threshold for the repeating pattern detectors on each FRO. An FRO 'alarm event' is declared when a repeating pattern (of up to four samples length) is detected continuously for the number of samples defined by this field's value. Reset value 0xFF should keep the number of 'alarm events' to a manageable level."]
pub type ALARM_THR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALARM_THR` writer - 7:0\\]
Alarm detection threshold for the repeating pattern detectors on each FRO. An FRO 'alarm event' is declared when a repeating pattern (of up to four samples length) is detected continuously for the number of samples defined by this field's value. Reset value 0xFF should keep the number of 'alarm events' to a manageable level."]
pub type ALARM_THR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALARMCNT_SPEC, u8, u8, 8, O>;
#[doc = "Field `RESERVED8` reader - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED8` writer - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALARMCNT_SPEC, u8, u8, 8, O>;
#[doc = "Field `SHUTDOWN_THR` reader - 20:16\\]
Threshold setting for generating IRQFLAGSTAT.SHUTDOWN_OVF interrupt. The interrupt is triggered when SHUTDOWN_CNT value exceeds this bit field."]
pub type SHUTDOWN_THR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHUTDOWN_THR` writer - 20:16\\]
Threshold setting for generating IRQFLAGSTAT.SHUTDOWN_OVF interrupt. The interrupt is triggered when SHUTDOWN_CNT value exceeds this bit field."]
pub type SHUTDOWN_THR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALARMCNT_SPEC, u8, u8, 5, O>;
#[doc = "Field `RESERVED21` reader - 23:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED21_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED21` writer - 23:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED21_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALARMCNT_SPEC, u8, u8, 3, O>;
#[doc = "Field `SHUTDOWN_CNT` reader - 29:24\\]
Read-only, indicates the number of '1' bits in ALARMSTOP register. The maximum value equals the number of FROs."]
pub type SHUTDOWN_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHUTDOWN_CNT` writer - 29:24\\]
Read-only, indicates the number of '1' bits in ALARMSTOP register. The maximum value equals the number of FROs."]
pub type SHUTDOWN_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALARMCNT_SPEC, u8, u8, 6, O>;
#[doc = "Field `RESERVED30` reader - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED30_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED30` writer - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED30_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALARMCNT_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Alarm detection threshold for the repeating pattern detectors on each FRO. An FRO 'alarm event' is declared when a repeating pattern (of up to four samples length) is detected continuously for the number of samples defined by this field's value. Reset value 0xFF should keep the number of 'alarm events' to a manageable level."]
    #[inline(always)]
    pub fn alarm_thr(&self) -> ALARM_THR_R {
        ALARM_THR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Threshold setting for generating IRQFLAGSTAT.SHUTDOWN_OVF interrupt. The interrupt is triggered when SHUTDOWN_CNT value exceeds this bit field."]
    #[inline(always)]
    pub fn shutdown_thr(&self) -> SHUTDOWN_THR_R {
        SHUTDOWN_THR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved21(&self) -> RESERVED21_R {
        RESERVED21_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Read-only, indicates the number of '1' bits in ALARMSTOP register. The maximum value equals the number of FROs."]
    #[inline(always)]
    pub fn shutdown_cnt(&self) -> SHUTDOWN_CNT_R {
        SHUTDOWN_CNT_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved30(&self) -> RESERVED30_R {
        RESERVED30_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Alarm detection threshold for the repeating pattern detectors on each FRO. An FRO 'alarm event' is declared when a repeating pattern (of up to four samples length) is detected continuously for the number of samples defined by this field's value. Reset value 0xFF should keep the number of 'alarm events' to a manageable level."]
    #[inline(always)]
    #[must_use]
    pub fn alarm_thr(&mut self) -> ALARM_THR_W<0> {
        ALARM_THR_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> RESERVED8_W<8> {
        RESERVED8_W::new(self)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Threshold setting for generating IRQFLAGSTAT.SHUTDOWN_OVF interrupt. The interrupt is triggered when SHUTDOWN_CNT value exceeds this bit field."]
    #[inline(always)]
    #[must_use]
    pub fn shutdown_thr(&mut self) -> SHUTDOWN_THR_W<16> {
        SHUTDOWN_THR_W::new(self)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved21(&mut self) -> RESERVED21_W<21> {
        RESERVED21_W::new(self)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Read-only, indicates the number of '1' bits in ALARMSTOP register. The maximum value equals the number of FROs."]
    #[inline(always)]
    #[must_use]
    pub fn shutdown_cnt(&mut self) -> SHUTDOWN_CNT_W<24> {
        SHUTDOWN_CNT_W::new(self)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved30(&mut self) -> RESERVED30_W<30> {
        RESERVED30_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alarm Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alarmcnt](index.html) module"]
pub struct ALARMCNT_SPEC;
impl crate::RegisterSpec for ALARMCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alarmcnt::R](R) reader structure"]
impl crate::Readable for ALARMCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alarmcnt::W](W) writer structure"]
impl crate::Writable for ALARMCNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALARMCNT to value 0xff"]
impl crate::Resettable for ALARMCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
