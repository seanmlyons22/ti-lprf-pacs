#[doc = "Register `DESCEX1` reader"]
pub type R = crate::R<Descex1Spec>;
#[doc = "Register `DESCEX1` writer"]
pub type W = crate::W<Descex1Spec>;
#[doc = "Field `RESERVED0` reader - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `RESERVED0` writer - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "15:8\\]
System radio feature availability\n\nValue on reset: 255"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ropt {
    #[doc = "255: All features available"]
    Max = 255,
}
impl From<Ropt> for u8 {
    #[inline(always)]
    fn from(variant: Ropt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ropt {
    type Ux = u8;
}
impl crate::IsEnum for Ropt {}
#[doc = "Field `ROPT` reader - 15:8\\]
System radio feature availability"]
pub type RoptR = crate::FieldReader<Ropt>;
impl RoptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ropt> {
        match self.bits {
            255 => Some(Ropt::Max),
            _ => None,
        }
    }
    #[doc = "All features available"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == Ropt::Max
    }
}
#[doc = "Field `ROPT` writer - 15:8\\]
System radio feature availability"]
pub type RoptW<'a, REG> = crate::FieldWriter<'a, REG, 8, Ropt>;
impl<'a, REG> RoptW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "All features available"]
    #[inline(always)]
    pub fn max(self) -> &'a mut crate::W<REG> {
        self.variant(Ropt::Max)
    }
}
#[doc = "Field `RESERVED16` reader - 27:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 27:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "29:28\\]
System SRAM availability\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sramsz {
    #[doc = "3: SRAM size set to level 3 (Max size)"]
    Sz3 = 3,
    #[doc = "2: SRAM size set to level 2"]
    Sz2 = 2,
    #[doc = "1: SRAM size set to level 1"]
    Sz1 = 1,
    #[doc = "0: SRAM size set to level 0 (Min size)"]
    Sz0 = 0,
}
impl From<Sramsz> for u8 {
    #[inline(always)]
    fn from(variant: Sramsz) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sramsz {
    type Ux = u8;
}
impl crate::IsEnum for Sramsz {}
#[doc = "Field `SRAMSZ` reader - 29:28\\]
System SRAM availability"]
pub type SramszR = crate::FieldReader<Sramsz>;
impl SramszR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sramsz {
        match self.bits {
            3 => Sramsz::Sz3,
            2 => Sramsz::Sz2,
            1 => Sramsz::Sz1,
            0 => Sramsz::Sz0,
            _ => unreachable!(),
        }
    }
    #[doc = "SRAM size set to level 3 (Max size)"]
    #[inline(always)]
    pub fn is_sz3(&self) -> bool {
        *self == Sramsz::Sz3
    }
    #[doc = "SRAM size set to level 2"]
    #[inline(always)]
    pub fn is_sz2(&self) -> bool {
        *self == Sramsz::Sz2
    }
    #[doc = "SRAM size set to level 1"]
    #[inline(always)]
    pub fn is_sz1(&self) -> bool {
        *self == Sramsz::Sz1
    }
    #[doc = "SRAM size set to level 0 (Min size)"]
    #[inline(always)]
    pub fn is_sz0(&self) -> bool {
        *self == Sramsz::Sz0
    }
}
#[doc = "Field `SRAMSZ` writer - 29:28\\]
System SRAM availability"]
pub type SramszW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sramsz, crate::Safe>;
impl<'a, REG> SramszW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SRAM size set to level 3 (Max size)"]
    #[inline(always)]
    pub fn sz3(self) -> &'a mut crate::W<REG> {
        self.variant(Sramsz::Sz3)
    }
    #[doc = "SRAM size set to level 2"]
    #[inline(always)]
    pub fn sz2(self) -> &'a mut crate::W<REG> {
        self.variant(Sramsz::Sz2)
    }
    #[doc = "SRAM size set to level 1"]
    #[inline(always)]
    pub fn sz1(self) -> &'a mut crate::W<REG> {
        self.variant(Sramsz::Sz1)
    }
    #[doc = "SRAM size set to level 0 (Min size)"]
    #[inline(always)]
    pub fn sz0(self) -> &'a mut crate::W<REG> {
        self.variant(Sramsz::Sz0)
    }
}
#[doc = "31:30\\]
System flash availability\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Flashsz {
    #[doc = "3: Flash size set to level 3 (Max size)"]
    Sz3 = 3,
    #[doc = "2: Flash size set to level 2"]
    Sz2 = 2,
    #[doc = "1: Flash size set to level 1"]
    Sz1 = 1,
    #[doc = "0: Flash size set to level 0 (Min size)"]
    Sz0 = 0,
}
impl From<Flashsz> for u8 {
    #[inline(always)]
    fn from(variant: Flashsz) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Flashsz {
    type Ux = u8;
}
impl crate::IsEnum for Flashsz {}
#[doc = "Field `FLASHSZ` reader - 31:30\\]
System flash availability"]
pub type FlashszR = crate::FieldReader<Flashsz>;
impl FlashszR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flashsz {
        match self.bits {
            3 => Flashsz::Sz3,
            2 => Flashsz::Sz2,
            1 => Flashsz::Sz1,
            0 => Flashsz::Sz0,
            _ => unreachable!(),
        }
    }
    #[doc = "Flash size set to level 3 (Max size)"]
    #[inline(always)]
    pub fn is_sz3(&self) -> bool {
        *self == Flashsz::Sz3
    }
    #[doc = "Flash size set to level 2"]
    #[inline(always)]
    pub fn is_sz2(&self) -> bool {
        *self == Flashsz::Sz2
    }
    #[doc = "Flash size set to level 1"]
    #[inline(always)]
    pub fn is_sz1(&self) -> bool {
        *self == Flashsz::Sz1
    }
    #[doc = "Flash size set to level 0 (Min size)"]
    #[inline(always)]
    pub fn is_sz0(&self) -> bool {
        *self == Flashsz::Sz0
    }
}
#[doc = "Field `FLASHSZ` writer - 31:30\\]
System flash availability"]
pub type FlashszW<'a, REG> = crate::FieldWriter<'a, REG, 2, Flashsz, crate::Safe>;
impl<'a, REG> FlashszW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Flash size set to level 3 (Max size)"]
    #[inline(always)]
    pub fn sz3(self) -> &'a mut crate::W<REG> {
        self.variant(Flashsz::Sz3)
    }
    #[doc = "Flash size set to level 2"]
    #[inline(always)]
    pub fn sz2(self) -> &'a mut crate::W<REG> {
        self.variant(Flashsz::Sz2)
    }
    #[doc = "Flash size set to level 1"]
    #[inline(always)]
    pub fn sz1(self) -> &'a mut crate::W<REG> {
        self.variant(Flashsz::Sz1)
    }
    #[doc = "Flash size set to level 0 (Min size)"]
    #[inline(always)]
    pub fn sz0(self) -> &'a mut crate::W<REG> {
        self.variant(Flashsz::Sz0)
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
System radio feature availability"]
    #[inline(always)]
    pub fn ropt(&self) -> RoptR {
        RoptR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:29 - 29:28\\]
System SRAM availability"]
    #[inline(always)]
    pub fn sramsz(&self) -> SramszR {
        SramszR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - 31:30\\]
System flash availability"]
    #[inline(always)]
    pub fn flashsz(&self) -> FlashszR {
        FlashszR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<Descex1Spec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
System radio feature availability"]
    #[inline(always)]
    #[must_use]
    pub fn ropt(&mut self) -> RoptW<Descex1Spec> {
        RoptW::new(self, 8)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<Descex1Spec> {
        Reserved16W::new(self, 16)
    }
    #[doc = "Bits 28:29 - 29:28\\]
System SRAM availability"]
    #[inline(always)]
    #[must_use]
    pub fn sramsz(&mut self) -> SramszW<Descex1Spec> {
        SramszW::new(self, 28)
    }
    #[doc = "Bits 30:31 - 31:30\\]
System flash availability"]
    #[inline(always)]
    #[must_use]
    pub fn flashsz(&mut self) -> FlashszW<Descex1Spec> {
        FlashszW::new(self, 30)
    }
}
#[doc = "Extended Description Register 1. This register shows SVT IP availability, HW features and memory size configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`descex1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`descex1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Descex1Spec;
impl crate::RegisterSpec for Descex1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`descex1::R`](R) reader structure"]
impl crate::Readable for Descex1Spec {}
#[doc = "`write(|w| ..)` method takes [`descex1::W`](W) writer structure"]
impl crate::Writable for Descex1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DESCEX1 to value 0xf000_ff00"]
impl crate::Resettable for Descex1Spec {
    const RESET_VALUE: u32 = 0xf000_ff00;
}
