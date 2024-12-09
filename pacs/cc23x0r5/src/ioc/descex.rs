#[doc = "Register `DESCEX` reader"]
pub type R = crate::R<DescexSpec>;
#[doc = "Register `DESCEX` writer"]
pub type W = crate::W<DescexSpec>;
#[doc = "5:0\\]
Number of DIOs supported. Total DIOs supported is NUMDIO value +1.\n\nValue on reset: 25"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Numdio {
    #[doc = "63: Highest possible value"]
    Maximum = 63,
    #[doc = "0: Smallest value"]
    Minimum = 0,
}
impl From<Numdio> for u8 {
    #[inline(always)]
    fn from(variant: Numdio) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Numdio {
    type Ux = u8;
}
impl crate::IsEnum for Numdio {}
#[doc = "Field `NUMDIO` reader - 5:0\\]
Number of DIOs supported. Total DIOs supported is NUMDIO value +1."]
pub type NumdioR = crate::FieldReader<Numdio>;
impl NumdioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Numdio> {
        match self.bits {
            63 => Some(Numdio::Maximum),
            0 => Some(Numdio::Minimum),
            _ => None,
        }
    }
    #[doc = "Highest possible value"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == Numdio::Maximum
    }
    #[doc = "Smallest value"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == Numdio::Minimum
    }
}
#[doc = "Field `NUMDIO` writer - 5:0\\]
Number of DIOs supported. Total DIOs supported is NUMDIO value +1."]
pub type NumdioW<'a, REG> = crate::FieldWriter<'a, REG, 6, Numdio>;
impl<'a, REG> NumdioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Highest possible value"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut crate::W<REG> {
        self.variant(Numdio::Maximum)
    }
    #[doc = "Smallest value"]
    #[inline(always)]
    pub fn minimum(self) -> &'a mut crate::W<REG> {
        self.variant(Numdio::Minimum)
    }
}
#[doc = "6:6\\]
High drive IO supported by IOC.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hdio {
    #[doc = "1: HD IO supported by IOC"]
    Present = 1,
    #[doc = "0: HD IO not supported by IOC"]
    Absent = 0,
}
impl From<Hdio> for bool {
    #[inline(always)]
    fn from(variant: Hdio) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDIO` reader - 6:6\\]
High drive IO supported by IOC."]
pub type HdioR = crate::BitReader<Hdio>;
impl HdioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hdio {
        match self.bits {
            true => Hdio::Present,
            false => Hdio::Absent,
        }
    }
    #[doc = "HD IO supported by IOC"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == Hdio::Present
    }
    #[doc = "HD IO not supported by IOC"]
    #[inline(always)]
    pub fn is_absent(&self) -> bool {
        *self == Hdio::Absent
    }
}
#[doc = "Field `HDIO` writer - 6:6\\]
High drive IO supported by IOC."]
pub type HdioW<'a, REG> = crate::BitWriter<'a, REG, Hdio>;
impl<'a, REG> HdioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HD IO supported by IOC"]
    #[inline(always)]
    pub fn present(self) -> &'a mut crate::W<REG> {
        self.variant(Hdio::Present)
    }
    #[doc = "HD IO not supported by IOC"]
    #[inline(always)]
    pub fn absent(self) -> &'a mut crate::W<REG> {
        self.variant(Hdio::Absent)
    }
}
#[doc = "11:7\\]
Number of HD IOs supported. Total HD IOs supported is NUMHDIO value +1.\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Numhdio {
    #[doc = "31: Highest possible value"]
    Maximum = 31,
    #[doc = "0: Smallest value"]
    Minimum = 0,
}
impl From<Numhdio> for u8 {
    #[inline(always)]
    fn from(variant: Numhdio) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Numhdio {
    type Ux = u8;
}
impl crate::IsEnum for Numhdio {}
#[doc = "Field `NUMHDIO` reader - 11:7\\]
Number of HD IOs supported. Total HD IOs supported is NUMHDIO value +1."]
pub type NumhdioR = crate::FieldReader<Numhdio>;
impl NumhdioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Numhdio> {
        match self.bits {
            31 => Some(Numhdio::Maximum),
            0 => Some(Numhdio::Minimum),
            _ => None,
        }
    }
    #[doc = "Highest possible value"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == Numhdio::Maximum
    }
    #[doc = "Smallest value"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == Numhdio::Minimum
    }
}
#[doc = "Field `NUMHDIO` writer - 11:7\\]
Number of HD IOs supported. Total HD IOs supported is NUMHDIO value +1."]
pub type NumhdioW<'a, REG> = crate::FieldWriter<'a, REG, 5, Numhdio>;
impl<'a, REG> NumhdioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Highest possible value"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut crate::W<REG> {
        self.variant(Numhdio::Maximum)
    }
    #[doc = "Smallest value"]
    #[inline(always)]
    pub fn minimum(self) -> &'a mut crate::W<REG> {
        self.variant(Numhdio::Minimum)
    }
}
#[doc = "15:12\\]
Number of DTB IOs supported. Total DTB IOs supported is NUMDTBIO value +1.\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Numdtbio {
    #[doc = "15: Highest possible value"]
    Maximum = 15,
    #[doc = "0: Smallest value"]
    Minimum = 0,
}
impl From<Numdtbio> for u8 {
    #[inline(always)]
    fn from(variant: Numdtbio) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Numdtbio {
    type Ux = u8;
}
impl crate::IsEnum for Numdtbio {}
#[doc = "Field `NUMDTBIO` reader - 15:12\\]
Number of DTB IOs supported. Total DTB IOs supported is NUMDTBIO value +1."]
pub type NumdtbioR = crate::FieldReader<Numdtbio>;
impl NumdtbioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Numdtbio> {
        match self.bits {
            15 => Some(Numdtbio::Maximum),
            0 => Some(Numdtbio::Minimum),
            _ => None,
        }
    }
    #[doc = "Highest possible value"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == Numdtbio::Maximum
    }
    #[doc = "Smallest value"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == Numdtbio::Minimum
    }
}
#[doc = "Field `NUMDTBIO` writer - 15:12\\]
Number of DTB IOs supported. Total DTB IOs supported is NUMDTBIO value +1."]
pub type NumdtbioW<'a, REG> = crate::FieldWriter<'a, REG, 4, Numdtbio>;
impl<'a, REG> NumdtbioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Highest possible value"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut crate::W<REG> {
        self.variant(Numdtbio::Maximum)
    }
    #[doc = "Smallest value"]
    #[inline(always)]
    pub fn minimum(self) -> &'a mut crate::W<REG> {
        self.variant(Numdtbio::Minimum)
    }
}
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Number of DIOs supported. Total DIOs supported is NUMDIO value +1."]
    #[inline(always)]
    pub fn numdio(&self) -> NumdioR {
        NumdioR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
High drive IO supported by IOC."]
    #[inline(always)]
    pub fn hdio(&self) -> HdioR {
        HdioR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:11 - 11:7\\]
Number of HD IOs supported. Total HD IOs supported is NUMHDIO value +1."]
    #[inline(always)]
    pub fn numhdio(&self) -> NumhdioR {
        NumhdioR::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Number of DTB IOs supported. Total DTB IOs supported is NUMDTBIO value +1."]
    #[inline(always)]
    pub fn numdtbio(&self) -> NumdtbioR {
        NumdtbioR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Number of DIOs supported. Total DIOs supported is NUMDIO value +1."]
    #[inline(always)]
    #[must_use]
    pub fn numdio(&mut self) -> NumdioW<DescexSpec> {
        NumdioW::new(self, 0)
    }
    #[doc = "Bit 6 - 6:6\\]
High drive IO supported by IOC."]
    #[inline(always)]
    #[must_use]
    pub fn hdio(&mut self) -> HdioW<DescexSpec> {
        HdioW::new(self, 6)
    }
    #[doc = "Bits 7:11 - 11:7\\]
Number of HD IOs supported. Total HD IOs supported is NUMHDIO value +1."]
    #[inline(always)]
    #[must_use]
    pub fn numhdio(&mut self) -> NumhdioW<DescexSpec> {
        NumhdioW::new(self, 7)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Number of DTB IOs supported. Total DTB IOs supported is NUMDTBIO value +1."]
    #[inline(always)]
    #[must_use]
    pub fn numdtbio(&mut self) -> NumdtbioW<DescexSpec> {
        NumdtbioW::new(self, 12)
    }
}
#[doc = "Extended Description Register. This register provides configuration details of the IP to software drivers and end users.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`descex::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`descex::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DescexSpec;
impl crate::RegisterSpec for DescexSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`descex::R`](R) reader structure"]
impl crate::Readable for DescexSpec {}
#[doc = "`write(|w| ..)` method takes [`descex::W`](W) writer structure"]
impl crate::Writable for DescexSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DESCEX to value 0xf2d9"]
impl crate::Resettable for DescexSpec {
    const RESET_VALUE: u32 = 0xf2d9;
}
