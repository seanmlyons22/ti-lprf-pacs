#[doc = "Register `ALARMSTOP` reader"]
pub type R = crate::R<AlarmstopSpec>;
#[doc = "Register `ALARMSTOP` writer"]
pub type W = crate::W<AlarmstopSpec>;
#[doc = "Field `FRO_FLAGS` reader - 23:0\\]
Logging bits for the 'alarm events' of individual FROs. A '1' in bit \\[n\\]
indicates FRO 'n' experienced more than one 'alarm event' in quick succession and has been turned off. A '1' in this field forces the corresponding bit in FROEN.FRO_MASK to '0'."]
pub type FroFlagsR = crate::FieldReader<u32>;
#[doc = "Field `FRO_FLAGS` writer - 23:0\\]
Logging bits for the 'alarm events' of individual FROs. A '1' in bit \\[n\\]
indicates FRO 'n' experienced more than one 'alarm event' in quick succession and has been turned off. A '1' in this field forces the corresponding bit in FROEN.FRO_MASK to '0'."]
pub type FroFlagsW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `RESERVED24` reader - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24R = crate::FieldReader;
#[doc = "Field `RESERVED24` writer - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Logging bits for the 'alarm events' of individual FROs. A '1' in bit \\[n\\]
indicates FRO 'n' experienced more than one 'alarm event' in quick succession and has been turned off. A '1' in this field forces the corresponding bit in FROEN.FRO_MASK to '0'."]
    #[inline(always)]
    pub fn fro_flags(&self) -> FroFlagsR {
        FroFlagsR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> Reserved24R {
        Reserved24R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Logging bits for the 'alarm events' of individual FROs. A '1' in bit \\[n\\]
indicates FRO 'n' experienced more than one 'alarm event' in quick succession and has been turned off. A '1' in this field forces the corresponding bit in FROEN.FRO_MASK to '0'."]
    #[inline(always)]
    #[must_use]
    pub fn fro_flags(&mut self) -> FroFlagsW<AlarmstopSpec> {
        FroFlagsW::new(self, 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved24(&mut self) -> Reserved24W<AlarmstopSpec> {
        Reserved24W::new(self, 24)
    }
}
#[doc = "Alarm Shutdown\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alarmstop::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarmstop::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlarmstopSpec;
impl crate::RegisterSpec for AlarmstopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alarmstop::R`](R) reader structure"]
impl crate::Readable for AlarmstopSpec {}
#[doc = "`write(|w| ..)` method takes [`alarmstop::W`](W) writer structure"]
impl crate::Writable for AlarmstopSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALARMSTOP to value 0"]
impl crate::Resettable for AlarmstopSpec {
    const RESET_VALUE: u32 = 0;
}
