#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Register `MIS` writer"]
pub type W = crate::W<MisSpec>;
#[doc = "Field `TATOMIS` reader - 0:0\\]
0: No interrupt or interrupt not enabled 1: RIS.TATORIS = 1 &amp;&amp; IMR.TATOIM = 1"]
pub type TatomisR = crate::BitReader;
#[doc = "Field `TATOMIS` writer - 0:0\\]
0: No interrupt or interrupt not enabled 1: RIS.TATORIS = 1 &amp;&amp; IMR.TATOIM = 1"]
pub type TatomisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAMMIS` reader - 1:1\\]
0: No interrupt or interrupt not enabled 1: RIS.CAMRIS = 1 &amp;&amp; IMR.CAMIM = 1"]
pub type CammisR = crate::BitReader;
#[doc = "Field `CAMMIS` writer - 1:1\\]
0: No interrupt or interrupt not enabled 1: RIS.CAMRIS = 1 &amp;&amp; IMR.CAMIM = 1"]
pub type CammisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAEMIS` reader - 2:2\\]
0: No interrupt or interrupt not enabled 1: RIS.CAERIS = 1 &amp;&amp; IMR.CAEIM = 1"]
pub type CaemisR = crate::BitReader;
#[doc = "Field `CAEMIS` writer - 2:2\\]
0: No interrupt or interrupt not enabled 1: RIS.CAERIS = 1 &amp;&amp; IMR.CAEIM = 1"]
pub type CaemisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `RESERVED3` writer - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMMIS` reader - 4:4\\]
0: No interrupt or interrupt not enabled 1: RIS.TAMRIS = 1 &amp;&amp; IMR.TAMIM = 1"]
pub type TammisR = crate::BitReader;
#[doc = "Field `TAMMIS` writer - 4:4\\]
0: No interrupt or interrupt not enabled 1: RIS.TAMRIS = 1 &amp;&amp; IMR.TAMIM = 1"]
pub type TammisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAAMIS` reader - 5:5\\]
0: No interrupt or interrupt not enabled 1: RIS.DMAARIS = 1 &amp;&amp; IMR.DMAAIM = 1"]
pub type DmaamisR = crate::BitReader;
#[doc = "Field `DMAAMIS` writer - 5:5\\]
0: No interrupt or interrupt not enabled 1: RIS.DMAARIS = 1 &amp;&amp; IMR.DMAAIM = 1"]
pub type DmaamisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED6` reader - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader;
#[doc = "Field `RESERVED6` writer - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TBTOMIS` reader - 8:8\\]
0: No interrupt or interrupt not enabled 1: RIS.TBTORIS = 1 &amp;&amp; IMR.TBTOIM = 1"]
pub type TbtomisR = crate::BitReader;
#[doc = "Field `TBTOMIS` writer - 8:8\\]
0: No interrupt or interrupt not enabled 1: RIS.TBTORIS = 1 &amp;&amp; IMR.TBTOIM = 1"]
pub type TbtomisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBMMIS` reader - 9:9\\]
0: No interrupt or interrupt not enabled 1: RIS.CBMRIS = 1 &amp;&amp; IMR.CBMIM = 1"]
pub type CbmmisR = crate::BitReader;
#[doc = "Field `CBMMIS` writer - 9:9\\]
0: No interrupt or interrupt not enabled 1: RIS.CBMRIS = 1 &amp;&amp; IMR.CBMIM = 1"]
pub type CbmmisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBEMIS` reader - 10:10\\]
0: No interrupt or interrupt not enabled 1: RIS.CBERIS = 1 &amp;&amp; IMR.CBEIM = 1"]
pub type CbemisR = crate::BitReader;
#[doc = "Field `CBEMIS` writer - 10:10\\]
0: No interrupt or interrupt not enabled 1: RIS.CBERIS = 1 &amp;&amp; IMR.CBEIM = 1"]
pub type CbemisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBMMIS` reader - 11:11\\]
0: No interrupt or interrupt not enabled 1: RIS.TBMRIS = 1 &amp;&amp; IMR.TBMIM = 1"]
pub type TbmmisR = crate::BitReader;
#[doc = "Field `TBMMIS` writer - 11:11\\]
0: No interrupt or interrupt not enabled 1: RIS.TBMRIS = 1 &amp;&amp; IMR.TBMIM = 1"]
pub type TbmmisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED12` reader - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12R = crate::BitReader;
#[doc = "Field `RESERVED12` writer - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMABMIS` reader - 13:13\\]
0: No interrupt or interrupt not enabled 1: RIS.DMABRIS = 1 &amp;&amp; IMR.DMABIM = 1"]
pub type DmabmisR = crate::BitReader;
#[doc = "Field `DMABMIS` writer - 13:13\\]
0: No interrupt or interrupt not enabled 1: RIS.DMABRIS = 1 &amp;&amp; IMR.DMABIM = 1"]
pub type DmabmisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED14` reader - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved14R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED14` writer - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved14W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0: No interrupt or interrupt not enabled 1: RIS.TATORIS = 1 &amp;&amp; IMR.TATOIM = 1"]
    #[inline(always)]
    pub fn tatomis(&self) -> TatomisR {
        TatomisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: No interrupt or interrupt not enabled 1: RIS.CAMRIS = 1 &amp;&amp; IMR.CAMIM = 1"]
    #[inline(always)]
    pub fn cammis(&self) -> CammisR {
        CammisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
0: No interrupt or interrupt not enabled 1: RIS.CAERIS = 1 &amp;&amp; IMR.CAEIM = 1"]
    #[inline(always)]
    pub fn caemis(&self) -> CaemisR {
        CaemisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
0: No interrupt or interrupt not enabled 1: RIS.TAMRIS = 1 &amp;&amp; IMR.TAMIM = 1"]
    #[inline(always)]
    pub fn tammis(&self) -> TammisR {
        TammisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
0: No interrupt or interrupt not enabled 1: RIS.DMAARIS = 1 &amp;&amp; IMR.DMAAIM = 1"]
    #[inline(always)]
    pub fn dmaamis(&self) -> DmaamisR {
        DmaamisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
0: No interrupt or interrupt not enabled 1: RIS.TBTORIS = 1 &amp;&amp; IMR.TBTOIM = 1"]
    #[inline(always)]
    pub fn tbtomis(&self) -> TbtomisR {
        TbtomisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
0: No interrupt or interrupt not enabled 1: RIS.CBMRIS = 1 &amp;&amp; IMR.CBMIM = 1"]
    #[inline(always)]
    pub fn cbmmis(&self) -> CbmmisR {
        CbmmisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
0: No interrupt or interrupt not enabled 1: RIS.CBERIS = 1 &amp;&amp; IMR.CBEIM = 1"]
    #[inline(always)]
    pub fn cbemis(&self) -> CbemisR {
        CbemisR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
0: No interrupt or interrupt not enabled 1: RIS.TBMRIS = 1 &amp;&amp; IMR.TBMIM = 1"]
    #[inline(always)]
    pub fn tbmmis(&self) -> TbmmisR {
        TbmmisR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
0: No interrupt or interrupt not enabled 1: RIS.DMABRIS = 1 &amp;&amp; IMR.DMABIM = 1"]
    #[inline(always)]
    pub fn dmabmis(&self) -> DmabmisR {
        DmabmisR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&self) -> Reserved14R {
        Reserved14R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
0: No interrupt or interrupt not enabled 1: RIS.TATORIS = 1 &amp;&amp; IMR.TATOIM = 1"]
    #[inline(always)]
    #[must_use]
    pub fn tatomis(&mut self) -> TatomisW<MisSpec> {
        TatomisW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: No interrupt or interrupt not enabled 1: RIS.CAMRIS = 1 &amp;&amp; IMR.CAMIM = 1"]
    #[inline(always)]
    #[must_use]
    pub fn cammis(&mut self) -> CammisW<MisSpec> {
        CammisW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
0: No interrupt or interrupt not enabled 1: RIS.CAERIS = 1 &amp;&amp; IMR.CAEIM = 1"]
    #[inline(always)]
    #[must_use]
    pub fn caemis(&mut self) -> CaemisW<MisSpec> {
        CaemisW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<MisSpec> {
        Reserved3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
0: No interrupt or interrupt not enabled 1: RIS.TAMRIS = 1 &amp;&amp; IMR.TAMIM = 1"]
    #[inline(always)]
    #[must_use]
    pub fn tammis(&mut self) -> TammisW<MisSpec> {
        TammisW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
0: No interrupt or interrupt not enabled 1: RIS.DMAARIS = 1 &amp;&amp; IMR.DMAAIM = 1"]
    #[inline(always)]
    #[must_use]
    pub fn dmaamis(&mut self) -> DmaamisW<MisSpec> {
        DmaamisW::new(self, 5)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<MisSpec> {
        Reserved6W::new(self, 6)
    }
    #[doc = "Bit 8 - 8:8\\]
0: No interrupt or interrupt not enabled 1: RIS.TBTORIS = 1 &amp;&amp; IMR.TBTOIM = 1"]
    #[inline(always)]
    #[must_use]
    pub fn tbtomis(&mut self) -> TbtomisW<MisSpec> {
        TbtomisW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
0: No interrupt or interrupt not enabled 1: RIS.CBMRIS = 1 &amp;&amp; IMR.CBMIM = 1"]
    #[inline(always)]
    #[must_use]
    pub fn cbmmis(&mut self) -> CbmmisW<MisSpec> {
        CbmmisW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
0: No interrupt or interrupt not enabled 1: RIS.CBERIS = 1 &amp;&amp; IMR.CBEIM = 1"]
    #[inline(always)]
    #[must_use]
    pub fn cbemis(&mut self) -> CbemisW<MisSpec> {
        CbemisW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
0: No interrupt or interrupt not enabled 1: RIS.TBMRIS = 1 &amp;&amp; IMR.TBMIM = 1"]
    #[inline(always)]
    #[must_use]
    pub fn tbmmis(&mut self) -> TbmmisW<MisSpec> {
        TbmmisW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> Reserved12W<MisSpec> {
        Reserved12W::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
0: No interrupt or interrupt not enabled 1: RIS.DMABRIS = 1 &amp;&amp; IMR.DMABIM = 1"]
    #[inline(always)]
    #[must_use]
    pub fn dmabmis(&mut self) -> DmabmisW<MisSpec> {
        DmabmisW::new(self, 13)
    }
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved14(&mut self) -> Reserved14W<MisSpec> {
        Reserved14W::new(self, 14)
    }
}
#[doc = "Masked Interrupt Status Values are result of bitwise AND operation between RIS and IMR Assosciated clear register: ICLR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisSpec;
impl crate::RegisterSpec for MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MisSpec {}
#[doc = "`write(|w| ..)` method takes [`mis::W`](W) writer structure"]
impl crate::Writable for MisSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MisSpec {
    const RESET_VALUE: u32 = 0;
}
