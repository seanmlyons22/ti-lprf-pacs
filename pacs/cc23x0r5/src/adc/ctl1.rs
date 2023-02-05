#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "0:0\\]
Sample trigger source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trigsrc {
    #[doc = "1: Hardware event trigger"]
    Event = 1,
    #[doc = "0: Software trigger"]
    Software = 0,
}
impl From<Trigsrc> for bool {
    #[inline(always)]
    fn from(variant: Trigsrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGSRC` reader - 0:0\\]
Sample trigger source"]
pub type TrigsrcR = crate::BitReader<Trigsrc>;
impl TrigsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trigsrc {
        match self.bits {
            true => Trigsrc::Event,
            false => Trigsrc::Software,
        }
    }
    #[doc = "Hardware event trigger"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == Trigsrc::Event
    }
    #[doc = "Software trigger"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == Trigsrc::Software
    }
}
#[doc = "Field `TRIGSRC` writer - 0:0\\]
Sample trigger source"]
pub type TrigsrcW<'a, REG> = crate::BitWriter<'a, REG, Trigsrc>;
impl<'a, REG> TrigsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware event trigger"]
    #[inline(always)]
    pub fn event(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrc::Event)
    }
    #[doc = "Software trigger"]
    #[inline(always)]
    pub fn software(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrc::Software)
    }
}
#[doc = "Field `RESERVED1` reader - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader;
#[doc = "8:8\\]
Start of conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sc {
    #[doc = "1: When SAMPMODE is set to MANUAL, setting this bit will start the sample phase. Sample phase will last as long as this bit is set. When SAMPMODE is set to AUTO, setting this bit will trigger the timer based sample time."]
    Start = 1,
    #[doc = "0: When SAMPMODE is set to MANUAL, clearing this bit will end the sample phase and the conversion phase will start. When SAMPMODE is set to AUTO, writing 0 has no effect."]
    Stop = 0,
}
impl From<Sc> for bool {
    #[inline(always)]
    fn from(variant: Sc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SC` reader - 8:8\\]
Start of conversion"]
pub type ScR = crate::BitReader<Sc>;
impl ScR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sc {
        match self.bits {
            true => Sc::Start,
            false => Sc::Stop,
        }
    }
    #[doc = "When SAMPMODE is set to MANUAL, setting this bit will start the sample phase. Sample phase will last as long as this bit is set. When SAMPMODE is set to AUTO, setting this bit will trigger the timer based sample time."]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == Sc::Start
    }
    #[doc = "When SAMPMODE is set to MANUAL, clearing this bit will end the sample phase and the conversion phase will start. When SAMPMODE is set to AUTO, writing 0 has no effect."]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Sc::Stop
    }
}
#[doc = "Field `SC` writer - 8:8\\]
Start of conversion"]
pub type ScW<'a, REG> = crate::BitWriter<'a, REG, Sc>;
impl<'a, REG> ScW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When SAMPMODE is set to MANUAL, setting this bit will start the sample phase. Sample phase will last as long as this bit is set. When SAMPMODE is set to AUTO, setting this bit will trigger the timer based sample time."]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(Sc::Start)
    }
    #[doc = "When SAMPMODE is set to MANUAL, clearing this bit will end the sample phase and the conversion phase will start. When SAMPMODE is set to AUTO, writing 0 has no effect."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Sc::Stop)
    }
}
#[doc = "Field `RESERVED9` reader - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader;
#[doc = "17:16\\]
Conversion sequence mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Conseq {
    #[doc = "3: ADC channel sequence pointed by STARTADD and ENDADD will be converted repeatedly"]
    Repeatsequence = 3,
    #[doc = "2: ADC channel in MEMCTLx pointed by STARTADD will be converted repeatedly"]
    Repeatsingle = 2,
    #[doc = "1: ADC channel sequence pointed by STARTADD and ENDADD will be converted once"]
    Sequence = 1,
    #[doc = "0: ADC channel in MEMCTLx pointed by STARTADD will be converted once"]
    Single = 0,
}
impl From<Conseq> for u8 {
    #[inline(always)]
    fn from(variant: Conseq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Conseq {
    type Ux = u8;
}
impl crate::IsEnum for Conseq {}
#[doc = "Field `CONSEQ` reader - 17:16\\]
Conversion sequence mode"]
pub type ConseqR = crate::FieldReader<Conseq>;
impl ConseqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Conseq {
        match self.bits {
            3 => Conseq::Repeatsequence,
            2 => Conseq::Repeatsingle,
            1 => Conseq::Sequence,
            0 => Conseq::Single,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC channel sequence pointed by STARTADD and ENDADD will be converted repeatedly"]
    #[inline(always)]
    pub fn is_repeatsequence(&self) -> bool {
        *self == Conseq::Repeatsequence
    }
    #[doc = "ADC channel in MEMCTLx pointed by STARTADD will be converted repeatedly"]
    #[inline(always)]
    pub fn is_repeatsingle(&self) -> bool {
        *self == Conseq::Repeatsingle
    }
    #[doc = "ADC channel sequence pointed by STARTADD and ENDADD will be converted once"]
    #[inline(always)]
    pub fn is_sequence(&self) -> bool {
        *self == Conseq::Sequence
    }
    #[doc = "ADC channel in MEMCTLx pointed by STARTADD will be converted once"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == Conseq::Single
    }
}
#[doc = "Field `CONSEQ` writer - 17:16\\]
Conversion sequence mode"]
pub type ConseqW<'a, REG> = crate::FieldWriter<'a, REG, 2, Conseq, crate::Safe>;
impl<'a, REG> ConseqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC channel sequence pointed by STARTADD and ENDADD will be converted repeatedly"]
    #[inline(always)]
    pub fn repeatsequence(self) -> &'a mut crate::W<REG> {
        self.variant(Conseq::Repeatsequence)
    }
    #[doc = "ADC channel in MEMCTLx pointed by STARTADD will be converted repeatedly"]
    #[inline(always)]
    pub fn repeatsingle(self) -> &'a mut crate::W<REG> {
        self.variant(Conseq::Repeatsingle)
    }
    #[doc = "ADC channel sequence pointed by STARTADD and ENDADD will be converted once"]
    #[inline(always)]
    pub fn sequence(self) -> &'a mut crate::W<REG> {
        self.variant(Conseq::Sequence)
    }
    #[doc = "ADC channel in MEMCTLx pointed by STARTADD will be converted once"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(Conseq::Single)
    }
}
#[doc = "Field `RESERVED18` reader - 19:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved18R = crate::FieldReader;
#[doc = "20:20\\]
Sample mode. This bit selects the source of the sampling signal. MANUAL option is not valid when TRIGSRC is selected as hardware event trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sampmode {
    #[doc = "1: Software trigger is used as sample signal"]
    Manual = 1,
    #[doc = "0: Sample timer high phase is used as sample signal"]
    Auto = 0,
}
impl From<Sampmode> for bool {
    #[inline(always)]
    fn from(variant: Sampmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAMPMODE` reader - 20:20\\]
Sample mode. This bit selects the source of the sampling signal. MANUAL option is not valid when TRIGSRC is selected as hardware event trigger."]
pub type SampmodeR = crate::BitReader<Sampmode>;
impl SampmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sampmode {
        match self.bits {
            true => Sampmode::Manual,
            false => Sampmode::Auto,
        }
    }
    #[doc = "Software trigger is used as sample signal"]
    #[inline(always)]
    pub fn is_manual(&self) -> bool {
        *self == Sampmode::Manual
    }
    #[doc = "Sample timer high phase is used as sample signal"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == Sampmode::Auto
    }
}
#[doc = "Field `SAMPMODE` writer - 20:20\\]
Sample mode. This bit selects the source of the sampling signal. MANUAL option is not valid when TRIGSRC is selected as hardware event trigger."]
pub type SampmodeW<'a, REG> = crate::BitWriter<'a, REG, Sampmode>;
impl<'a, REG> SampmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software trigger is used as sample signal"]
    #[inline(always)]
    pub fn manual(self) -> &'a mut crate::W<REG> {
        self.variant(Sampmode::Manual)
    }
    #[doc = "Sample timer high phase is used as sample signal"]
    #[inline(always)]
    pub fn auto(self) -> &'a mut crate::W<REG> {
        self.variant(Sampmode::Auto)
    }
}
#[doc = "Field `RESERVED21` reader - 31:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved21R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Sample trigger source"]
    #[inline(always)]
    pub fn trigsrc(&self) -> TrigsrcR {
        TrigsrcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Start of conversion"]
    #[inline(always)]
    pub fn sc(&self) -> ScR {
        ScR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Conversion sequence mode"]
    #[inline(always)]
    pub fn conseq(&self) -> ConseqR {
        ConseqR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - 19:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved18(&self) -> Reserved18R {
        Reserved18R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - 20:20\\]
Sample mode. This bit selects the source of the sampling signal. MANUAL option is not valid when TRIGSRC is selected as hardware event trigger."]
    #[inline(always)]
    pub fn sampmode(&self) -> SampmodeR {
        SampmodeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:31 - 31:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved21(&self) -> Reserved21R {
        Reserved21R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Sample trigger source"]
    #[inline(always)]
    #[must_use]
    pub fn trigsrc(&mut self) -> TrigsrcW<Ctl1Spec> {
        TrigsrcW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Start of conversion"]
    #[inline(always)]
    #[must_use]
    pub fn sc(&mut self) -> ScW<Ctl1Spec> {
        ScW::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Conversion sequence mode"]
    #[inline(always)]
    #[must_use]
    pub fn conseq(&mut self) -> ConseqW<Ctl1Spec> {
        ConseqW::new(self, 16)
    }
    #[doc = "Bit 20 - 20:20\\]
Sample mode. This bit selects the source of the sampling signal. MANUAL option is not valid when TRIGSRC is selected as hardware event trigger."]
    #[inline(always)]
    #[must_use]
    pub fn sampmode(&mut self) -> SampmodeW<Ctl1Spec> {
        SampmodeW::new(self, 20)
    }
}
#[doc = "Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl1Spec;
impl crate::RegisterSpec for Ctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for Ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for Ctl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for Ctl1Spec {
    const RESET_VALUE: u32 = 0;
}
