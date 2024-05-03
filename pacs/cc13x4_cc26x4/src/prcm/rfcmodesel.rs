#[doc = "Register `RFCMODESEL` reader"]
pub type R = crate::R<RfcmodeselSpec>;
#[doc = "Register `RFCMODESEL` writer"]
pub type W = crate::W<RfcmodeselSpec>;
#[doc = "2:0\\]
Selects the set of commands that the RFC will accept. Only modes permitted by RFCMODEHWOPT.AVAIL are writable. See the technical reference manual for details.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Curr {
    #[doc = "7: Select Mode 7"]
    Mode7 = 7,
    #[doc = "6: Select Mode 6"]
    Mode6 = 6,
    #[doc = "5: Select Mode 5"]
    Mode5 = 5,
    #[doc = "4: Select Mode 4"]
    Mode4 = 4,
    #[doc = "3: Select Mode 3"]
    Mode3 = 3,
    #[doc = "2: Select Mode 2"]
    Mode2 = 2,
    #[doc = "1: Select Mode 1"]
    Mode1 = 1,
    #[doc = "0: Select Mode 0"]
    Mode0 = 0,
}
impl From<Curr> for u8 {
    #[inline(always)]
    fn from(variant: Curr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Curr {
    type Ux = u8;
}
impl crate::IsEnum for Curr {}
#[doc = "Field `CURR` reader - 2:0\\]
Selects the set of commands that the RFC will accept. Only modes permitted by RFCMODEHWOPT.AVAIL are writable. See the technical reference manual for details."]
pub type CurrR = crate::FieldReader<Curr>;
impl CurrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Curr {
        match self.bits {
            7 => Curr::Mode7,
            6 => Curr::Mode6,
            5 => Curr::Mode5,
            4 => Curr::Mode4,
            3 => Curr::Mode3,
            2 => Curr::Mode2,
            1 => Curr::Mode1,
            0 => Curr::Mode0,
            _ => unreachable!(),
        }
    }
    #[doc = "Select Mode 7"]
    #[inline(always)]
    pub fn is_mode7(&self) -> bool {
        *self == Curr::Mode7
    }
    #[doc = "Select Mode 6"]
    #[inline(always)]
    pub fn is_mode6(&self) -> bool {
        *self == Curr::Mode6
    }
    #[doc = "Select Mode 5"]
    #[inline(always)]
    pub fn is_mode5(&self) -> bool {
        *self == Curr::Mode5
    }
    #[doc = "Select Mode 4"]
    #[inline(always)]
    pub fn is_mode4(&self) -> bool {
        *self == Curr::Mode4
    }
    #[doc = "Select Mode 3"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == Curr::Mode3
    }
    #[doc = "Select Mode 2"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == Curr::Mode2
    }
    #[doc = "Select Mode 1"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == Curr::Mode1
    }
    #[doc = "Select Mode 0"]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == Curr::Mode0
    }
}
#[doc = "Field `CURR` writer - 2:0\\]
Selects the set of commands that the RFC will accept. Only modes permitted by RFCMODEHWOPT.AVAIL are writable. See the technical reference manual for details."]
pub type CurrW<'a, REG> = crate::FieldWriter<'a, REG, 3, Curr, crate::Safe>;
impl<'a, REG> CurrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select Mode 7"]
    #[inline(always)]
    pub fn mode7(self) -> &'a mut crate::W<REG> {
        self.variant(Curr::Mode7)
    }
    #[doc = "Select Mode 6"]
    #[inline(always)]
    pub fn mode6(self) -> &'a mut crate::W<REG> {
        self.variant(Curr::Mode6)
    }
    #[doc = "Select Mode 5"]
    #[inline(always)]
    pub fn mode5(self) -> &'a mut crate::W<REG> {
        self.variant(Curr::Mode5)
    }
    #[doc = "Select Mode 4"]
    #[inline(always)]
    pub fn mode4(self) -> &'a mut crate::W<REG> {
        self.variant(Curr::Mode4)
    }
    #[doc = "Select Mode 3"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut crate::W<REG> {
        self.variant(Curr::Mode3)
    }
    #[doc = "Select Mode 2"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut crate::W<REG> {
        self.variant(Curr::Mode2)
    }
    #[doc = "Select Mode 1"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut crate::W<REG> {
        self.variant(Curr::Mode1)
    }
    #[doc = "Select Mode 0"]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut crate::W<REG> {
        self.variant(Curr::Mode0)
    }
}
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Selects the set of commands that the RFC will accept. Only modes permitted by RFCMODEHWOPT.AVAIL are writable. See the technical reference manual for details."]
    #[inline(always)]
    pub fn curr(&self) -> CurrR {
        CurrR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Selects the set of commands that the RFC will accept. Only modes permitted by RFCMODEHWOPT.AVAIL are writable. See the technical reference manual for details."]
    #[inline(always)]
    #[must_use]
    pub fn curr(&mut self) -> CurrW<RfcmodeselSpec> {
        CurrW::new(self, 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<RfcmodeselSpec> {
        Reserved3W::new(self, 3)
    }
}
#[doc = "Selected RFC Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfcmodesel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfcmodesel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfcmodeselSpec;
impl crate::RegisterSpec for RfcmodeselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfcmodesel::R`](R) reader structure"]
impl crate::Readable for RfcmodeselSpec {}
#[doc = "`write(|w| ..)` method takes [`rfcmodesel::W`](W) writer structure"]
impl crate::Writable for RfcmodeselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RFCMODESEL to value 0"]
impl crate::Resettable for RfcmodeselSpec {
    const RESET_VALUE: u32 = 0;
}
