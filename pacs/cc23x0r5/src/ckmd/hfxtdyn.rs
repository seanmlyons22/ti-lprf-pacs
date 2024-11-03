#[doc = "Register `HFXTDYN` reader"]
pub type R = crate::R<HfxtdynSpec>;
#[doc = "Register `HFXTDYN` writer"]
pub type W = crate::W<HfxtdynSpec>;
#[doc = "Field `Q1CAP` reader - 5:0\\]
Target HFXT Q1 cap trim"]
pub type Q1capR = crate::FieldReader;
#[doc = "Field `Q1CAP` writer - 5:0\\]
Target HFXT Q1 cap trim"]
pub type Q1capW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Q2CAP` reader - 11:6\\]
Target HFXT Q2 cap trim"]
pub type Q2capR = crate::FieldReader;
#[doc = "Field `Q2CAP` writer - 11:6\\]
Target HFXT Q2 cap trim"]
pub type Q2capW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `IREF` reader - 15:12\\]
Target HFXT IREF current"]
pub type IrefR = crate::FieldReader;
#[doc = "Field `IREF` writer - 15:12\\]
Target HFXT IREF current"]
pub type IrefW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IDAC` reader - 22:16\\]
Minimum IDAC current"]
pub type IdacR = crate::FieldReader;
#[doc = "Field `IDAC` writer - 22:16\\]
Minimum IDAC current"]
pub type IdacW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `AMPTHR` reader - 29:23\\]
Minimum HFXT amplitude"]
pub type AmpthrR = crate::FieldReader;
#[doc = "Field `AMPTHR` writer - 29:23\\]
Minimum HFXT amplitude"]
pub type AmpthrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RESERVED30` reader - 30:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved30R = crate::BitReader;
#[doc = "Field `RESERVED30` writer - 30:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "31:31\\]
Select the dynamic configuration. Amplitude ramping will always happen using the values in HFXTINIT, and HFXTTARG. Afterwards, this bit can be used to select between HFXTTARG and HFXTDYN. Hardware will ensure a smooth transition of analog control signals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sel {
    #[doc = "1: Select configuration in HFXTDYN."]
    Dynamic = 1,
    #[doc = "0: Select configuration in HFXTTARG."]
    Target = 0,
}
impl From<Sel> for bool {
    #[inline(always)]
    fn from(variant: Sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEL` reader - 31:31\\]
Select the dynamic configuration. Amplitude ramping will always happen using the values in HFXTINIT, and HFXTTARG. Afterwards, this bit can be used to select between HFXTTARG and HFXTDYN. Hardware will ensure a smooth transition of analog control signals."]
pub type SelR = crate::BitReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sel {
        match self.bits {
            true => Sel::Dynamic,
            false => Sel::Target,
        }
    }
    #[doc = "Select configuration in HFXTDYN."]
    #[inline(always)]
    pub fn is_dynamic(&self) -> bool {
        *self == Sel::Dynamic
    }
    #[doc = "Select configuration in HFXTTARG."]
    #[inline(always)]
    pub fn is_target(&self) -> bool {
        *self == Sel::Target
    }
}
#[doc = "Field `SEL` writer - 31:31\\]
Select the dynamic configuration. Amplitude ramping will always happen using the values in HFXTINIT, and HFXTTARG. Afterwards, this bit can be used to select between HFXTTARG and HFXTDYN. Hardware will ensure a smooth transition of analog control signals."]
pub type SelW<'a, REG> = crate::BitWriter<'a, REG, Sel>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select configuration in HFXTDYN."]
    #[inline(always)]
    pub fn dynamic(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Dynamic)
    }
    #[doc = "Select configuration in HFXTTARG."]
    #[inline(always)]
    pub fn target(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Target)
    }
}
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Target HFXT Q1 cap trim"]
    #[inline(always)]
    pub fn q1cap(&self) -> Q1capR {
        Q1capR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - 11:6\\]
Target HFXT Q2 cap trim"]
    #[inline(always)]
    pub fn q2cap(&self) -> Q2capR {
        Q2capR::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Target HFXT IREF current"]
    #[inline(always)]
    pub fn iref(&self) -> IrefR {
        IrefR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Minimum IDAC current"]
    #[inline(always)]
    pub fn idac(&self) -> IdacR {
        IdacR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 23:29 - 29:23\\]
Minimum HFXT amplitude"]
    #[inline(always)]
    pub fn ampthr(&self) -> AmpthrR {
        AmpthrR::new(((self.bits >> 23) & 0x7f) as u8)
    }
    #[doc = "Bit 30 - 30:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved30(&self) -> Reserved30R {
        Reserved30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Select the dynamic configuration. Amplitude ramping will always happen using the values in HFXTINIT, and HFXTTARG. Afterwards, this bit can be used to select between HFXTTARG and HFXTDYN. Hardware will ensure a smooth transition of analog control signals."]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Target HFXT Q1 cap trim"]
    #[inline(always)]
    #[must_use]
    pub fn q1cap(&mut self) -> Q1capW<HfxtdynSpec> {
        Q1capW::new(self, 0)
    }
    #[doc = "Bits 6:11 - 11:6\\]
Target HFXT Q2 cap trim"]
    #[inline(always)]
    #[must_use]
    pub fn q2cap(&mut self) -> Q2capW<HfxtdynSpec> {
        Q2capW::new(self, 6)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Target HFXT IREF current"]
    #[inline(always)]
    #[must_use]
    pub fn iref(&mut self) -> IrefW<HfxtdynSpec> {
        IrefW::new(self, 12)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Minimum IDAC current"]
    #[inline(always)]
    #[must_use]
    pub fn idac(&mut self) -> IdacW<HfxtdynSpec> {
        IdacW::new(self, 16)
    }
    #[doc = "Bits 23:29 - 29:23\\]
Minimum HFXT amplitude"]
    #[inline(always)]
    #[must_use]
    pub fn ampthr(&mut self) -> AmpthrW<HfxtdynSpec> {
        AmpthrW::new(self, 23)
    }
    #[doc = "Bit 30 - 30:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved30(&mut self) -> Reserved30W<HfxtdynSpec> {
        Reserved30W::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Select the dynamic configuration. Amplitude ramping will always happen using the values in HFXTINIT, and HFXTTARG. Afterwards, this bit can be used to select between HFXTTARG and HFXTDYN. Hardware will ensure a smooth transition of analog control signals."]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<HfxtdynSpec> {
        SelW::new(self, 31)
    }
}
#[doc = "Alternative target values for HFXT configuration Software can change these values to dynamically transition the HFXT configuration while HFXT is running. Set SEL to select the alternative set of target values.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfxtdyn::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfxtdyn::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfxtdynSpec;
impl crate::RegisterSpec for HfxtdynSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfxtdyn::R`](R) reader structure"]
impl crate::Readable for HfxtdynSpec {}
#[doc = "`write(|w| ..)` method takes [`hfxtdyn::W`](W) writer structure"]
impl crate::Writable for HfxtdynSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HFXTDYN to value 0x1446_4b6d"]
impl crate::Resettable for HfxtdynSpec {
    const RESET_VALUE: u32 = 0x1446_4b6d;
}
