#[doc = "Register `DESCEX` reader"]
pub type R = crate::R<DescexSpec>;
#[doc = "Register `DESCEX` writer"]
pub type W = crate::W<DescexSpec>;
#[doc = "Field `NCH` reader - 3:0\\]
Number of channels."]
pub type NchR = crate::FieldReader;
#[doc = "Field `NCH` writer - 3:0\\]
Number of channels."]
pub type NchW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "5:4\\]
Counter bit-width. The maximum counter value is equal to 2^CNTRW-1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cntrw {
    #[doc = "2: 32-bit counter."]
    Cntr32 = 2,
    #[doc = "1: 24-bit counter."]
    Cntr24 = 1,
    #[doc = "0: 16-bit counter."]
    Cntr16 = 0,
}
impl From<Cntrw> for u8 {
    #[inline(always)]
    fn from(variant: Cntrw) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cntrw {
    type Ux = u8;
}
impl crate::IsEnum for Cntrw {}
#[doc = "Field `CNTRW` reader - 5:4\\]
Counter bit-width. The maximum counter value is equal to 2^CNTRW-1."]
pub type CntrwR = crate::FieldReader<Cntrw>;
impl CntrwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cntrw {
        match self.bits {
            2 => Cntrw::Cntr32,
            1 => Cntrw::Cntr24,
            0 => Cntrw::Cntr16,
            _ => unreachable!(),
        }
    }
    #[doc = "32-bit counter."]
    #[inline(always)]
    pub fn is_cntr32(&self) -> bool {
        *self == Cntrw::Cntr32
    }
    #[doc = "24-bit counter."]
    #[inline(always)]
    pub fn is_cntr24(&self) -> bool {
        *self == Cntrw::Cntr24
    }
    #[doc = "16-bit counter."]
    #[inline(always)]
    pub fn is_cntr16(&self) -> bool {
        *self == Cntrw::Cntr16
    }
}
#[doc = "Field `CNTRW` writer - 5:4\\]
Counter bit-width. The maximum counter value is equal to 2^CNTRW-1."]
pub type CntrwW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cntrw>;
impl<'a, REG> CntrwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32-bit counter."]
    #[inline(always)]
    pub fn cntr32(self) -> &'a mut crate::W<REG> {
        self.variant(Cntrw::Cntr32)
    }
    #[doc = "24-bit counter."]
    #[inline(always)]
    pub fn cntr24(self) -> &'a mut crate::W<REG> {
        self.variant(Cntrw::Cntr24)
    }
    #[doc = "16-bit counter."]
    #[inline(always)]
    pub fn cntr16(self) -> &'a mut crate::W<REG> {
        self.variant(Cntrw::Cntr16)
    }
}
#[doc = "Field `HINT` reader - 6:6\\]
Has interrupt output and logic."]
pub type HintR = crate::BitReader;
#[doc = "Field `HINT` writer - 6:6\\]
Has interrupt output and logic."]
pub type HintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDMA` reader - 7:7\\]
Has uDMA output and logic."]
pub type HdmaR = crate::BitReader;
#[doc = "Field `HDMA` writer - 7:7\\]
Has uDMA output and logic."]
pub type HdmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIFS` reader - 11:8\\]
Channel input filter size. The prevailing state filter can maximum be configured to 2^CIFS-1."]
pub type CifsR = crate::FieldReader;
#[doc = "Field `CIFS` writer - 11:8\\]
Channel input filter size. The prevailing state filter can maximum be configured to 2^CIFS-1."]
pub type CifsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HCIF` reader - 12:12\\]
Has channel input filter."]
pub type HcifR = crate::BitReader;
#[doc = "Field `HCIF` writer - 12:12\\]
Has channel input filter."]
pub type HcifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HQDEC` reader - 13:13\\]
Has Quadrature Decoder."]
pub type HqdecR = crate::BitReader;
#[doc = "Field `HQDEC` writer - 13:13\\]
Has Quadrature Decoder."]
pub type HqdecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PREW` reader - 17:14\\]
Prescale width. The prescaler can maximum be configured to 2^PREW-1."]
pub type PrewR = crate::FieldReader;
#[doc = "Field `PREW` writer - 17:14\\]
Prescale width. The prescaler can maximum be configured to 2^PREW-1."]
pub type PrewW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HDBF` reader - 18:18\\]
Has Dead-Band, Fault, and Park logic."]
pub type HdbfR = crate::BitReader;
#[doc = "Field `HDBF` writer - 18:18\\]
Has Dead-Band, Fault, and Park logic."]
pub type HdbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HIR` reader - 19:19\\]
Has IR logic."]
pub type HirR = crate::BitReader;
#[doc = "Field `HIR` writer - 19:19\\]
Has IR logic."]
pub type HirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED20` reader - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved20R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED20` writer - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved20W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Number of channels."]
    #[inline(always)]
    pub fn nch(&self) -> NchR {
        NchR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Counter bit-width. The maximum counter value is equal to 2^CNTRW-1."]
    #[inline(always)]
    pub fn cntrw(&self) -> CntrwR {
        CntrwR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
Has interrupt output and logic."]
    #[inline(always)]
    pub fn hint(&self) -> HintR {
        HintR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Has uDMA output and logic."]
    #[inline(always)]
    pub fn hdma(&self) -> HdmaR {
        HdmaR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Channel input filter size. The prevailing state filter can maximum be configured to 2^CIFS-1."]
    #[inline(always)]
    pub fn cifs(&self) -> CifsR {
        CifsR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - 12:12\\]
Has channel input filter."]
    #[inline(always)]
    pub fn hcif(&self) -> HcifR {
        HcifR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Has Quadrature Decoder."]
    #[inline(always)]
    pub fn hqdec(&self) -> HqdecR {
        HqdecR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:17 - 17:14\\]
Prescale width. The prescaler can maximum be configured to 2^PREW-1."]
    #[inline(always)]
    pub fn prew(&self) -> PrewR {
        PrewR::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - 18:18\\]
Has Dead-Band, Fault, and Park logic."]
    #[inline(always)]
    pub fn hdbf(&self) -> HdbfR {
        HdbfR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Has IR logic."]
    #[inline(always)]
    pub fn hir(&self) -> HirR {
        HirR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved20(&self) -> Reserved20R {
        Reserved20R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Number of channels."]
    #[inline(always)]
    #[must_use]
    pub fn nch(&mut self) -> NchW<DescexSpec> {
        NchW::new(self, 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Counter bit-width. The maximum counter value is equal to 2^CNTRW-1."]
    #[inline(always)]
    #[must_use]
    pub fn cntrw(&mut self) -> CntrwW<DescexSpec> {
        CntrwW::new(self, 4)
    }
    #[doc = "Bit 6 - 6:6\\]
Has interrupt output and logic."]
    #[inline(always)]
    #[must_use]
    pub fn hint(&mut self) -> HintW<DescexSpec> {
        HintW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Has uDMA output and logic."]
    #[inline(always)]
    #[must_use]
    pub fn hdma(&mut self) -> HdmaW<DescexSpec> {
        HdmaW::new(self, 7)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Channel input filter size. The prevailing state filter can maximum be configured to 2^CIFS-1."]
    #[inline(always)]
    #[must_use]
    pub fn cifs(&mut self) -> CifsW<DescexSpec> {
        CifsW::new(self, 8)
    }
    #[doc = "Bit 12 - 12:12\\]
Has channel input filter."]
    #[inline(always)]
    #[must_use]
    pub fn hcif(&mut self) -> HcifW<DescexSpec> {
        HcifW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Has Quadrature Decoder."]
    #[inline(always)]
    #[must_use]
    pub fn hqdec(&mut self) -> HqdecW<DescexSpec> {
        HqdecW::new(self, 13)
    }
    #[doc = "Bits 14:17 - 17:14\\]
Prescale width. The prescaler can maximum be configured to 2^PREW-1."]
    #[inline(always)]
    #[must_use]
    pub fn prew(&mut self) -> PrewW<DescexSpec> {
        PrewW::new(self, 14)
    }
    #[doc = "Bit 18 - 18:18\\]
Has Dead-Band, Fault, and Park logic."]
    #[inline(always)]
    #[must_use]
    pub fn hdbf(&mut self) -> HdbfW<DescexSpec> {
        HdbfW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Has IR logic."]
    #[inline(always)]
    #[must_use]
    pub fn hir(&mut self) -> HirW<DescexSpec> {
        HirW::new(self, 19)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved20(&mut self) -> Reserved20W<DescexSpec> {
        Reserved20W::new(self, 20)
    }
}
#[doc = "Description Extended This register describes the parameters of the LGPT.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`descex::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`descex::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets DESCEX to value 0x0002_18d3"]
impl crate::Resettable for DescexSpec {
    const RESET_VALUE: u32 = 0x0002_18d3;
}
