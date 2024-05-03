#[doc = "Register `MCUSRAMCFG` reader"]
pub type R = crate::R<McusramcfgSpec>;
#[doc = "Register `MCUSRAMCFG` writer"]
pub type W = crate::W<McusramcfgSpec>;
#[doc = "Field `PCH_L` reader - 0:0\\]
NOT in use. Writing any other value than the reset value may result in undefined behavior."]
pub type PchLR = crate::BitReader;
#[doc = "Field `PCH_L` writer - 0:0\\]
NOT in use. Writing any other value than the reset value may result in undefined behavior."]
pub type PchLW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCH_F` reader - 1:1\\]
NOT in use. Writing any other value than the reset value may result in undefined behavior."]
pub type PchFR = crate::BitReader;
#[doc = "Field `PCH_F` writer - 1:1\\]
NOT in use. Writing any other value than the reset value may result in undefined behavior."]
pub type PchFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BM` reader - 2:2\\]
NOT in use. Writing any other value than the reset value may result in undefined behavior."]
pub type BmR = crate::BitReader;
#[doc = "Field `BM` writer - 2:2\\]
NOT in use. Writing any other value than the reset value may result in undefined behavior."]
pub type BmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGS` reader - 3:3\\]
NOT in use. Writing any other value than the reset value may result in undefined behavior."]
pub type PgsR = crate::BitReader;
#[doc = "Field `PGS` writer - 3:3\\]
NOT in use. Writing any other value than the reset value may result in undefined behavior."]
pub type PgsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAGE` reader - 4:4\\]
NOT in use. Writing any other value than the reset value may result in undefined behavior."]
pub type PageR = crate::BitReader;
#[doc = "Field `PAGE` writer - 4:4\\]
NOT in use. Writing any other value than the reset value may result in undefined behavior."]
pub type PageW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BM_OFF` reader - 5:5\\]
NOT in use. Writing any other value than the reset value may result in undefined behavior."]
pub type BmOffR = crate::BitReader;
#[doc = "Field `BM_OFF` writer - 5:5\\]
NOT in use. Writing any other value than the reset value may result in undefined behavior."]
pub type BmOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARITY_EN` reader - 6:6\\]
Parity enable 0: Parity disabled Parity section available as GPRAM 1: Parity enabled"]
pub type ParityEnR = crate::BitReader;
#[doc = "Field `PARITY_EN` writer - 6:6\\]
Parity enable 0: Parity disabled Parity section available as GPRAM 1: Parity enabled"]
pub type ParityEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED7` reader - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED7` writer - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
NOT in use. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn pch_l(&self) -> PchLR {
        PchLR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
NOT in use. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn pch_f(&self) -> PchFR {
        PchFR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
NOT in use. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn bm(&self) -> BmR {
        BmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
NOT in use. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn pgs(&self) -> PgsR {
        PgsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
NOT in use. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn page(&self) -> PageR {
        PageR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
NOT in use. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn bm_off(&self) -> BmOffR {
        BmOffR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Parity enable 0: Parity disabled Parity section available as GPRAM 1: Parity enabled"]
    #[inline(always)]
    pub fn parity_en(&self) -> ParityEnR {
        ParityEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
NOT in use. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn pch_l(&mut self) -> PchLW<McusramcfgSpec> {
        PchLW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
NOT in use. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn pch_f(&mut self) -> PchFW<McusramcfgSpec> {
        PchFW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
NOT in use. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn bm(&mut self) -> BmW<McusramcfgSpec> {
        BmW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
NOT in use. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn pgs(&mut self) -> PgsW<McusramcfgSpec> {
        PgsW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
NOT in use. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn page(&mut self) -> PageW<McusramcfgSpec> {
        PageW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
NOT in use. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn bm_off(&mut self) -> BmOffW<McusramcfgSpec> {
        BmOffW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Parity enable 0: Parity disabled Parity section available as GPRAM 1: Parity enabled"]
    #[inline(always)]
    #[must_use]
    pub fn parity_en(&mut self) -> ParityEnW<McusramcfgSpec> {
        ParityEnW::new(self, 6)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> Reserved7W<McusramcfgSpec> {
        Reserved7W::new(self, 7)
    }
}
#[doc = "MCU SRAM configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcusramcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcusramcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McusramcfgSpec;
impl crate::RegisterSpec for McusramcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcusramcfg::R`](R) reader structure"]
impl crate::Readable for McusramcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`mcusramcfg::W`](W) writer structure"]
impl crate::Writable for McusramcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCUSRAMCFG to value 0x20"]
impl crate::Resettable for McusramcfgSpec {
    const RESET_VALUE: u32 = 0x20;
}
