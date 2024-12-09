#[doc = "Register `CTL0` reader"]
pub type R = crate::R<Ctl0Spec>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<Ctl0Spec>;
#[doc = "0:0\\]
Enable conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enc {
    #[doc = "1: Conversion enabled. ADC sequencer waits for valid trigger (software or hardware)."]
    On = 1,
    #[doc = "0: Conversion disabled. ENC change from ON to OFF will abort single or repeat sequence on a MEMCTLx boundary. The current conversion will finish and result stored in corresponding MEMRESx."]
    Off = 0,
}
impl From<Enc> for bool {
    #[inline(always)]
    fn from(variant: Enc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENC` reader - 0:0\\]
Enable conversion"]
pub type EncR = crate::BitReader<Enc>;
impl EncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enc {
        match self.bits {
            true => Enc::On,
            false => Enc::Off,
        }
    }
    #[doc = "Conversion enabled. ADC sequencer waits for valid trigger (software or hardware)."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Enc::On
    }
    #[doc = "Conversion disabled. ENC change from ON to OFF will abort single or repeat sequence on a MEMCTLx boundary. The current conversion will finish and result stored in corresponding MEMRESx."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Enc::Off
    }
}
#[doc = "Field `ENC` writer - 0:0\\]
Enable conversion"]
pub type EncW<'a, REG> = crate::BitWriter<'a, REG, Enc>;
impl<'a, REG> EncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Conversion enabled. ADC sequencer waits for valid trigger (software or hardware)."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Enc::On)
    }
    #[doc = "Conversion disabled. ENC change from ON to OFF will abort single or repeat sequence on a MEMCTLx boundary. The current conversion will finish and result stored in corresponding MEMRESx."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Enc::Off)
    }
}
#[doc = "Field `RESERVED1` reader - 15:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "16:16\\]
Power down policy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrdn {
    #[doc = "1: ADC remains powered on as long as it is enabled through software."]
    Manual = 1,
    #[doc = "0: ADC is powered down on completion of a conversion if there is no pending trigger"]
    Auto = 0,
}
impl From<Pwrdn> for bool {
    #[inline(always)]
    fn from(variant: Pwrdn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRDN` reader - 16:16\\]
Power down policy"]
pub type PwrdnR = crate::BitReader<Pwrdn>;
impl PwrdnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrdn {
        match self.bits {
            true => Pwrdn::Manual,
            false => Pwrdn::Auto,
        }
    }
    #[doc = "ADC remains powered on as long as it is enabled through software."]
    #[inline(always)]
    pub fn is_manual(&self) -> bool {
        *self == Pwrdn::Manual
    }
    #[doc = "ADC is powered down on completion of a conversion if there is no pending trigger"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == Pwrdn::Auto
    }
}
#[doc = "Field `PWRDN` writer - 16:16\\]
Power down policy"]
pub type PwrdnW<'a, REG> = crate::BitWriter<'a, REG, Pwrdn>;
impl<'a, REG> PwrdnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC remains powered on as long as it is enabled through software."]
    #[inline(always)]
    pub fn manual(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrdn::Manual)
    }
    #[doc = "ADC is powered down on completion of a conversion if there is no pending trigger"]
    #[inline(always)]
    pub fn auto(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrdn::Auto)
    }
}
#[doc = "Field `RESERVED17` reader - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved17R = crate::FieldReader;
#[doc = "26:24\\]
Sample clock divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sclkdiv {
    #[doc = "7: Divide clock source by 48"]
    DivBy48 = 7,
    #[doc = "6: Divide clock source by 32"]
    DivBy32 = 6,
    #[doc = "5: Divide clock source by 24"]
    DivBy24 = 5,
    #[doc = "4: Divide clock source by 16"]
    DivBy16 = 4,
    #[doc = "3: Divide clock source by 8"]
    DivBy8 = 3,
    #[doc = "2: Divide clock source by 4"]
    DivBy4 = 2,
    #[doc = "1: Divide clock source by 2"]
    DivBy2 = 1,
    #[doc = "0: Do not divide clock source"]
    DivBy1 = 0,
}
impl From<Sclkdiv> for u8 {
    #[inline(always)]
    fn from(variant: Sclkdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sclkdiv {
    type Ux = u8;
}
impl crate::IsEnum for Sclkdiv {}
#[doc = "Field `SCLKDIV` reader - 26:24\\]
Sample clock divider"]
pub type SclkdivR = crate::FieldReader<Sclkdiv>;
impl SclkdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sclkdiv {
        match self.bits {
            7 => Sclkdiv::DivBy48,
            6 => Sclkdiv::DivBy32,
            5 => Sclkdiv::DivBy24,
            4 => Sclkdiv::DivBy16,
            3 => Sclkdiv::DivBy8,
            2 => Sclkdiv::DivBy4,
            1 => Sclkdiv::DivBy2,
            0 => Sclkdiv::DivBy1,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide clock source by 48"]
    #[inline(always)]
    pub fn is_div_by_48(&self) -> bool {
        *self == Sclkdiv::DivBy48
    }
    #[doc = "Divide clock source by 32"]
    #[inline(always)]
    pub fn is_div_by_32(&self) -> bool {
        *self == Sclkdiv::DivBy32
    }
    #[doc = "Divide clock source by 24"]
    #[inline(always)]
    pub fn is_div_by_24(&self) -> bool {
        *self == Sclkdiv::DivBy24
    }
    #[doc = "Divide clock source by 16"]
    #[inline(always)]
    pub fn is_div_by_16(&self) -> bool {
        *self == Sclkdiv::DivBy16
    }
    #[doc = "Divide clock source by 8"]
    #[inline(always)]
    pub fn is_div_by_8(&self) -> bool {
        *self == Sclkdiv::DivBy8
    }
    #[doc = "Divide clock source by 4"]
    #[inline(always)]
    pub fn is_div_by_4(&self) -> bool {
        *self == Sclkdiv::DivBy4
    }
    #[doc = "Divide clock source by 2"]
    #[inline(always)]
    pub fn is_div_by_2(&self) -> bool {
        *self == Sclkdiv::DivBy2
    }
    #[doc = "Do not divide clock source"]
    #[inline(always)]
    pub fn is_div_by_1(&self) -> bool {
        *self == Sclkdiv::DivBy1
    }
}
#[doc = "Field `SCLKDIV` writer - 26:24\\]
Sample clock divider"]
pub type SclkdivW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sclkdiv, crate::Safe>;
impl<'a, REG> SclkdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide clock source by 48"]
    #[inline(always)]
    pub fn div_by_48(self) -> &'a mut crate::W<REG> {
        self.variant(Sclkdiv::DivBy48)
    }
    #[doc = "Divide clock source by 32"]
    #[inline(always)]
    pub fn div_by_32(self) -> &'a mut crate::W<REG> {
        self.variant(Sclkdiv::DivBy32)
    }
    #[doc = "Divide clock source by 24"]
    #[inline(always)]
    pub fn div_by_24(self) -> &'a mut crate::W<REG> {
        self.variant(Sclkdiv::DivBy24)
    }
    #[doc = "Divide clock source by 16"]
    #[inline(always)]
    pub fn div_by_16(self) -> &'a mut crate::W<REG> {
        self.variant(Sclkdiv::DivBy16)
    }
    #[doc = "Divide clock source by 8"]
    #[inline(always)]
    pub fn div_by_8(self) -> &'a mut crate::W<REG> {
        self.variant(Sclkdiv::DivBy8)
    }
    #[doc = "Divide clock source by 4"]
    #[inline(always)]
    pub fn div_by_4(self) -> &'a mut crate::W<REG> {
        self.variant(Sclkdiv::DivBy4)
    }
    #[doc = "Divide clock source by 2"]
    #[inline(always)]
    pub fn div_by_2(self) -> &'a mut crate::W<REG> {
        self.variant(Sclkdiv::DivBy2)
    }
    #[doc = "Do not divide clock source"]
    #[inline(always)]
    pub fn div_by_1(self) -> &'a mut crate::W<REG> {
        self.variant(Sclkdiv::DivBy1)
    }
}
#[doc = "Field `RESERVED27` reader - 31:27\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved27R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable conversion"]
    #[inline(always)]
    pub fn enc(&self) -> EncR {
        EncR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:15 - 15:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 0x7fff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Power down policy"]
    #[inline(always)]
    pub fn pwrdn(&self) -> PwrdnR {
        PwrdnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> Reserved17R {
        Reserved17R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Sample clock divider"]
    #[inline(always)]
    pub fn sclkdiv(&self) -> SclkdivR {
        SclkdivR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:31 - 31:27\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved27(&self) -> Reserved27R {
        Reserved27R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable conversion"]
    #[inline(always)]
    #[must_use]
    pub fn enc(&mut self) -> EncW<Ctl0Spec> {
        EncW::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Power down policy"]
    #[inline(always)]
    #[must_use]
    pub fn pwrdn(&mut self) -> PwrdnW<Ctl0Spec> {
        PwrdnW::new(self, 16)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Sample clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn sclkdiv(&mut self) -> SclkdivW<Ctl0Spec> {
        SclkdivW::new(self, 24)
    }
}
#[doc = "Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl0Spec;
impl crate::RegisterSpec for Ctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for Ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for Ctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for Ctl0Spec {
    const RESET_VALUE: u32 = 0;
}
