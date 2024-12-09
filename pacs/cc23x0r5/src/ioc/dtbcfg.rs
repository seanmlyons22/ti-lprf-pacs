#[doc = "Register `DTBCFG` reader"]
pub type R = crate::R<DtbcfgSpec>;
#[doc = "Register `DTBCFG` writer"]
pub type W = crate::W<DtbcfgSpec>;
#[doc = "4:0\\]
SVT DTB Mux selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Svtsel {
    #[doc = "31: Maximum value"]
    Maximum = 31,
    #[doc = "0: Minimum value"]
    Minimum = 0,
}
impl From<Svtsel> for u8 {
    #[inline(always)]
    fn from(variant: Svtsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Svtsel {
    type Ux = u8;
}
impl crate::IsEnum for Svtsel {}
#[doc = "Field `SVTSEL` reader - 4:0\\]
SVT DTB Mux selection"]
pub type SvtselR = crate::FieldReader<Svtsel>;
impl SvtselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Svtsel> {
        match self.bits {
            31 => Some(Svtsel::Maximum),
            0 => Some(Svtsel::Minimum),
            _ => None,
        }
    }
    #[doc = "Maximum value"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == Svtsel::Maximum
    }
    #[doc = "Minimum value"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == Svtsel::Minimum
    }
}
#[doc = "Field `SVTSEL` writer - 4:0\\]
SVT DTB Mux selection"]
pub type SvtselW<'a, REG> = crate::FieldWriter<'a, REG, 5, Svtsel>;
impl<'a, REG> SvtselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Maximum value"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut crate::W<REG> {
        self.variant(Svtsel::Maximum)
    }
    #[doc = "Minimum value"]
    #[inline(always)]
    pub fn minimum(self) -> &'a mut crate::W<REG> {
        self.variant(Svtsel::Minimum)
    }
}
#[doc = "Field `RESERVED5` reader - 7:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved5R = crate::FieldReader;
#[doc = "12:8\\]
ULL DTB Mux selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ullsel {
    #[doc = "31: Maximum value"]
    Maximum = 31,
    #[doc = "0: Minimum value"]
    Minimum = 0,
}
impl From<Ullsel> for u8 {
    #[inline(always)]
    fn from(variant: Ullsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ullsel {
    type Ux = u8;
}
impl crate::IsEnum for Ullsel {}
#[doc = "Field `ULLSEL` reader - 12:8\\]
ULL DTB Mux selection"]
pub type UllselR = crate::FieldReader<Ullsel>;
impl UllselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ullsel> {
        match self.bits {
            31 => Some(Ullsel::Maximum),
            0 => Some(Ullsel::Minimum),
            _ => None,
        }
    }
    #[doc = "Maximum value"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == Ullsel::Maximum
    }
    #[doc = "Minimum value"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == Ullsel::Minimum
    }
}
#[doc = "Field `ULLSEL` writer - 12:8\\]
ULL DTB Mux selection"]
pub type UllselW<'a, REG> = crate::FieldWriter<'a, REG, 5, Ullsel>;
impl<'a, REG> UllselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Maximum value"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut crate::W<REG> {
        self.variant(Ullsel::Maximum)
    }
    #[doc = "Minimum value"]
    #[inline(always)]
    pub fn minimum(self) -> &'a mut crate::W<REG> {
        self.variant(Ullsel::Minimum)
    }
}
#[doc = "Field `RESERVED13` reader - 15:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved13R = crate::FieldReader;
#[doc = "18:16\\]
Selects which 3 DTB lines out of total 16 are routed to DTB pins 15 to 13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Padsel {
    #[doc = "5: DTB\\[2:0\\]
selected"]
    Dtb2to0 = 5,
    #[doc = "4: DTB\\[5:3\\]
selected"]
    Dtb5to3 = 4,
    #[doc = "3: DTB\\[8:6\\]
selected"]
    Dtb8to6 = 3,
    #[doc = "2: DTB\\[11:9\\]
selected"]
    Dtb11to9 = 2,
    #[doc = "1: DTB\\[14:12\\]
selected"]
    Dtb14to12 = 1,
    #[doc = "0: DTB\\[15:13\\]
selected"]
    Dtb15to13 = 0,
}
impl From<Padsel> for u8 {
    #[inline(always)]
    fn from(variant: Padsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Padsel {
    type Ux = u8;
}
impl crate::IsEnum for Padsel {}
#[doc = "Field `PADSEL` reader - 18:16\\]
Selects which 3 DTB lines out of total 16 are routed to DTB pins 15 to 13."]
pub type PadselR = crate::FieldReader<Padsel>;
impl PadselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Padsel> {
        match self.bits {
            5 => Some(Padsel::Dtb2to0),
            4 => Some(Padsel::Dtb5to3),
            3 => Some(Padsel::Dtb8to6),
            2 => Some(Padsel::Dtb11to9),
            1 => Some(Padsel::Dtb14to12),
            0 => Some(Padsel::Dtb15to13),
            _ => None,
        }
    }
    #[doc = "DTB\\[2:0\\]
selected"]
    #[inline(always)]
    pub fn is_dtb2to0(&self) -> bool {
        *self == Padsel::Dtb2to0
    }
    #[doc = "DTB\\[5:3\\]
selected"]
    #[inline(always)]
    pub fn is_dtb5to3(&self) -> bool {
        *self == Padsel::Dtb5to3
    }
    #[doc = "DTB\\[8:6\\]
selected"]
    #[inline(always)]
    pub fn is_dtb8to6(&self) -> bool {
        *self == Padsel::Dtb8to6
    }
    #[doc = "DTB\\[11:9\\]
selected"]
    #[inline(always)]
    pub fn is_dtb11to9(&self) -> bool {
        *self == Padsel::Dtb11to9
    }
    #[doc = "DTB\\[14:12\\]
selected"]
    #[inline(always)]
    pub fn is_dtb14to12(&self) -> bool {
        *self == Padsel::Dtb14to12
    }
    #[doc = "DTB\\[15:13\\]
selected"]
    #[inline(always)]
    pub fn is_dtb15to13(&self) -> bool {
        *self == Padsel::Dtb15to13
    }
}
#[doc = "Field `PADSEL` writer - 18:16\\]
Selects which 3 DTB lines out of total 16 are routed to DTB pins 15 to 13."]
pub type PadselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Padsel>;
impl<'a, REG> PadselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DTB\\[2:0\\]
selected"]
    #[inline(always)]
    pub fn dtb2to0(self) -> &'a mut crate::W<REG> {
        self.variant(Padsel::Dtb2to0)
    }
    #[doc = "DTB\\[5:3\\]
selected"]
    #[inline(always)]
    pub fn dtb5to3(self) -> &'a mut crate::W<REG> {
        self.variant(Padsel::Dtb5to3)
    }
    #[doc = "DTB\\[8:6\\]
selected"]
    #[inline(always)]
    pub fn dtb8to6(self) -> &'a mut crate::W<REG> {
        self.variant(Padsel::Dtb8to6)
    }
    #[doc = "DTB\\[11:9\\]
selected"]
    #[inline(always)]
    pub fn dtb11to9(self) -> &'a mut crate::W<REG> {
        self.variant(Padsel::Dtb11to9)
    }
    #[doc = "DTB\\[14:12\\]
selected"]
    #[inline(always)]
    pub fn dtb14to12(self) -> &'a mut crate::W<REG> {
        self.variant(Padsel::Dtb14to12)
    }
    #[doc = "DTB\\[15:13\\]
selected"]
    #[inline(always)]
    pub fn dtb15to13(self) -> &'a mut crate::W<REG> {
        self.variant(Padsel::Dtb15to13)
    }
}
#[doc = "Field `RESERVED19` reader - 22:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved19R = crate::FieldReader;
#[doc = "23:23\\]
This bit is used to divide DTB\\[0\\]
output by 8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dtb0div {
    #[doc = "1: Divide DTB\\[0\\]
output by 8"]
    En = 1,
    #[doc = "0: No divide"]
    Dis = 0,
}
impl From<Dtb0div> for bool {
    #[inline(always)]
    fn from(variant: Dtb0div) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTB0DIV` reader - 23:23\\]
This bit is used to divide DTB\\[0\\]
output by 8."]
pub type Dtb0divR = crate::BitReader<Dtb0div>;
impl Dtb0divR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dtb0div {
        match self.bits {
            true => Dtb0div::En,
            false => Dtb0div::Dis,
        }
    }
    #[doc = "Divide DTB\\[0\\]
output by 8"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Dtb0div::En
    }
    #[doc = "No divide"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Dtb0div::Dis
    }
}
#[doc = "Field `DTB0DIV` writer - 23:23\\]
This bit is used to divide DTB\\[0\\]
output by 8."]
pub type Dtb0divW<'a, REG> = crate::BitWriter<'a, REG, Dtb0div>;
impl<'a, REG> Dtb0divW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Divide DTB\\[0\\]
output by 8"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Dtb0div::En)
    }
    #[doc = "No divide"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Dtb0div::Dis)
    }
}
#[doc = "Field `RESERVED24` reader - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
SVT DTB Mux selection"]
    #[inline(always)]
    pub fn svtsel(&self) -> SvtselR {
        SvtselR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
ULL DTB Mux selection"]
    #[inline(always)]
    pub fn ullsel(&self) -> UllselR {
        UllselR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved13(&self) -> Reserved13R {
        Reserved13R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Selects which 3 DTB lines out of total 16 are routed to DTB pins 15 to 13."]
    #[inline(always)]
    pub fn padsel(&self) -> PadselR {
        PadselR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:22 - 22:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved19(&self) -> Reserved19R {
        Reserved19R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - 23:23\\]
This bit is used to divide DTB\\[0\\]
output by 8."]
    #[inline(always)]
    pub fn dtb0div(&self) -> Dtb0divR {
        Dtb0divR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> Reserved24R {
        Reserved24R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
SVT DTB Mux selection"]
    #[inline(always)]
    #[must_use]
    pub fn svtsel(&mut self) -> SvtselW<DtbcfgSpec> {
        SvtselW::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
ULL DTB Mux selection"]
    #[inline(always)]
    #[must_use]
    pub fn ullsel(&mut self) -> UllselW<DtbcfgSpec> {
        UllselW::new(self, 8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Selects which 3 DTB lines out of total 16 are routed to DTB pins 15 to 13."]
    #[inline(always)]
    #[must_use]
    pub fn padsel(&mut self) -> PadselW<DtbcfgSpec> {
        PadselW::new(self, 16)
    }
    #[doc = "Bit 23 - 23:23\\]
This bit is used to divide DTB\\[0\\]
output by 8."]
    #[inline(always)]
    #[must_use]
    pub fn dtb0div(&mut self) -> Dtb0divW<DtbcfgSpec> {
        Dtb0divW::new(self, 23)
    }
}
#[doc = "DTB configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtbcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtbcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtbcfgSpec;
impl crate::RegisterSpec for DtbcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtbcfg::R`](R) reader structure"]
impl crate::Readable for DtbcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dtbcfg::W`](W) writer structure"]
impl crate::Writable for DtbcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DTBCFG to value 0"]
impl crate::Resettable for DtbcfgSpec {
    const RESET_VALUE: u32 = 0;
}
