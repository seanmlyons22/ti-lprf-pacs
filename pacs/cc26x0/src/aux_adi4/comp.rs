#[doc = "Register `COMP` reader"]
pub type R = crate::R<CompSpec>;
#[doc = "Register `COMP` writer"]
pub type W = crate::W<CompSpec>;
#[doc = "Field `COMPA_EN` reader - 0:0\\]
COMPA enable"]
pub type CompaEnR = crate::BitReader;
#[doc = "Field `COMPA_EN` writer - 0:0\\]
COMPA enable"]
pub type CompaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `RESERVED1` writer - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPB_EN` reader - 2:2\\]
COMPB enable"]
pub type CompbEnR = crate::BitReader;
#[doc = "Field `COMPB_EN` writer - 2:2\\]
COMPB enable"]
pub type CompbEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "5:3\\]
COMPB voltage reference trim temperature coded:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CompbTrim {
    #[doc = "7: Divide reference by 4"]
    Div4 = 7,
    #[doc = "3: Divide reference by 3"]
    Div3 = 3,
    #[doc = "1: Divide reference by 2"]
    Div2 = 1,
    #[doc = "0: No reference division"]
    Div1 = 0,
}
impl From<CompbTrim> for u8 {
    #[inline(always)]
    fn from(variant: CompbTrim) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CompbTrim {
    type Ux = u8;
}
impl crate::IsEnum for CompbTrim {}
#[doc = "Field `COMPB_TRIM` reader - 5:3\\]
COMPB voltage reference trim temperature coded:"]
pub type CompbTrimR = crate::FieldReader<CompbTrim>;
impl CompbTrimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CompbTrim> {
        match self.bits {
            7 => Some(CompbTrim::Div4),
            3 => Some(CompbTrim::Div3),
            1 => Some(CompbTrim::Div2),
            0 => Some(CompbTrim::Div1),
            _ => None,
        }
    }
    #[doc = "Divide reference by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CompbTrim::Div4
    }
    #[doc = "Divide reference by 3"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == CompbTrim::Div3
    }
    #[doc = "Divide reference by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == CompbTrim::Div2
    }
    #[doc = "No reference division"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == CompbTrim::Div1
    }
}
#[doc = "Field `COMPB_TRIM` writer - 5:3\\]
COMPB voltage reference trim temperature coded:"]
pub type CompbTrimW<'a, REG> = crate::FieldWriter<'a, REG, 3, CompbTrim>;
impl<'a, REG> CompbTrimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide reference by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(CompbTrim::Div4)
    }
    #[doc = "Divide reference by 3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(CompbTrim::Div3)
    }
    #[doc = "Divide reference by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(CompbTrim::Div2)
    }
    #[doc = "No reference division"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(CompbTrim::Div1)
    }
}
#[doc = "Field `COMPA_REF_CURR_EN` reader - 6:6\\]
Enables 2uA IPTAT current from ISRC to COMPA reference node. Requires ISRC.EN = 1. Used with COMPA_REF_RES_EN to generate voltage reference for cap-sense."]
pub type CompaRefCurrEnR = crate::BitReader;
#[doc = "Field `COMPA_REF_CURR_EN` writer - 6:6\\]
Enables 2uA IPTAT current from ISRC to COMPA reference node. Requires ISRC.EN = 1. Used with COMPA_REF_RES_EN to generate voltage reference for cap-sense."]
pub type CompaRefCurrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPA_REF_RES_EN` reader - 7:7\\]
Enables 400kohm resistance from COMPA reference node to ground. Used with COMPA_REF_CURR_EN to generate voltage reference for cap-sense."]
pub type CompaRefResEnR = crate::BitReader;
#[doc = "Field `COMPA_REF_RES_EN` writer - 7:7\\]
Enables 400kohm resistance from COMPA reference node to ground. Used with COMPA_REF_CURR_EN to generate voltage reference for cap-sense."]
pub type CompaRefResEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
COMPA enable"]
    #[inline(always)]
    pub fn compa_en(&self) -> CompaEnR {
        CompaEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
COMPB enable"]
    #[inline(always)]
    pub fn compb_en(&self) -> CompbEnR {
        CompbEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - 5:3\\]
COMPB voltage reference trim temperature coded:"]
    #[inline(always)]
    pub fn compb_trim(&self) -> CompbTrimR {
        CompbTrimR::new((self.bits >> 3) & 7)
    }
    #[doc = "Bit 6 - 6:6\\]
Enables 2uA IPTAT current from ISRC to COMPA reference node. Requires ISRC.EN = 1. Used with COMPA_REF_RES_EN to generate voltage reference for cap-sense."]
    #[inline(always)]
    pub fn compa_ref_curr_en(&self) -> CompaRefCurrEnR {
        CompaRefCurrEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Enables 400kohm resistance from COMPA reference node to ground. Used with COMPA_REF_CURR_EN to generate voltage reference for cap-sense."]
    #[inline(always)]
    pub fn compa_ref_res_en(&self) -> CompaRefResEnR {
        CompaRefResEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
COMPA enable"]
    #[inline(always)]
    #[must_use]
    pub fn compa_en(&mut self) -> CompaEnW<CompSpec> {
        CompaEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<CompSpec> {
        Reserved1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
COMPB enable"]
    #[inline(always)]
    #[must_use]
    pub fn compb_en(&mut self) -> CompbEnW<CompSpec> {
        CompbEnW::new(self, 2)
    }
    #[doc = "Bits 3:5 - 5:3\\]
COMPB voltage reference trim temperature coded:"]
    #[inline(always)]
    #[must_use]
    pub fn compb_trim(&mut self) -> CompbTrimW<CompSpec> {
        CompbTrimW::new(self, 3)
    }
    #[doc = "Bit 6 - 6:6\\]
Enables 2uA IPTAT current from ISRC to COMPA reference node. Requires ISRC.EN = 1. Used with COMPA_REF_RES_EN to generate voltage reference for cap-sense."]
    #[inline(always)]
    #[must_use]
    pub fn compa_ref_curr_en(&mut self) -> CompaRefCurrEnW<CompSpec> {
        CompaRefCurrEnW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Enables 400kohm resistance from COMPA reference node to ground. Used with COMPA_REF_CURR_EN to generate voltage reference for cap-sense."]
    #[inline(always)]
    #[must_use]
    pub fn compa_ref_res_en(&mut self) -> CompaRefResEnW<CompSpec> {
        CompaRefResEnW::new(self, 7)
    }
}
#[doc = "Comparator Control COMPA and COMPB comparators. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompSpec;
impl crate::RegisterSpec for CompSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`comp::R`](R) reader structure"]
impl crate::Readable for CompSpec {}
#[doc = "`write(|w| ..)` method takes [`comp::W`](W) writer structure"]
impl crate::Writable for CompSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets COMP to value 0"]
impl crate::Resettable for CompSpec {
    const RESET_VALUE: u8 = 0;
}
