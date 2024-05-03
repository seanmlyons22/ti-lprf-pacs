#[doc = "Register `ICLR` reader"]
pub type R = crate::R<IclrSpec>;
#[doc = "Register `ICLR` writer"]
pub type W = crate::W<IclrSpec>;
#[doc = "Field `TATOCINT` reader - 0:0\\]
0: Do nothing. 1: Clear RIS.TATORIS and MIS.TATOMIS"]
pub type TatocintR = crate::BitReader;
#[doc = "Field `TATOCINT` writer - 0:0\\]
0: Do nothing. 1: Clear RIS.TATORIS and MIS.TATOMIS"]
pub type TatocintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAMCINT` reader - 1:1\\]
0: Do nothing. 1: Clear RIS.CAMRIS and MIS.CAMMIS"]
pub type CamcintR = crate::BitReader;
#[doc = "Field `CAMCINT` writer - 1:1\\]
0: Do nothing. 1: Clear RIS.CAMRIS and MIS.CAMMIS"]
pub type CamcintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAECINT` reader - 2:2\\]
0: Do nothing. 1: Clear RIS.CAERIS and MIS.CAEMIS"]
pub type CaecintR = crate::BitReader;
#[doc = "Field `CAECINT` writer - 2:2\\]
0: Do nothing. 1: Clear RIS.CAERIS and MIS.CAEMIS"]
pub type CaecintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `RESERVED3` writer - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMCINT` reader - 4:4\\]
0: Do nothing. 1: Clear RIS.TAMRIS and MIS.TAMMIS"]
pub type TamcintR = crate::BitReader;
#[doc = "Field `TAMCINT` writer - 4:4\\]
0: Do nothing. 1: Clear RIS.TAMRIS and MIS.TAMMIS"]
pub type TamcintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAAINT` reader - 5:5\\]
0: Do nothing. 1: Clear RIS.DMAARIS and MIS.DMAAMIS"]
pub type DmaaintR = crate::BitReader;
#[doc = "Field `DMAAINT` writer - 5:5\\]
0: Do nothing. 1: Clear RIS.DMAARIS and MIS.DMAAMIS"]
pub type DmaaintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED6` reader - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader;
#[doc = "Field `RESERVED6` writer - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TBTOCINT` reader - 8:8\\]
0: Do nothing. 1: Clear RIS.TBTORIS and MIS.TBTOMIS"]
pub type TbtocintR = crate::BitReader;
#[doc = "Field `TBTOCINT` writer - 8:8\\]
0: Do nothing. 1: Clear RIS.TBTORIS and MIS.TBTOMIS"]
pub type TbtocintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBMCINT` reader - 9:9\\]
0: Do nothing. 1: Clear RIS.CBMRIS and MIS.CBMMIS"]
pub type CbmcintR = crate::BitReader;
#[doc = "Field `CBMCINT` writer - 9:9\\]
0: Do nothing. 1: Clear RIS.CBMRIS and MIS.CBMMIS"]
pub type CbmcintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBECINT` reader - 10:10\\]
0: Do nothing. 1: Clear RIS.CBERIS and MIS.CBEMIS"]
pub type CbecintR = crate::BitReader;
#[doc = "Field `CBECINT` writer - 10:10\\]
0: Do nothing. 1: Clear RIS.CBERIS and MIS.CBEMIS"]
pub type CbecintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBMCINT` reader - 11:11\\]
0: Do nothing. 1: Clear RIS.TBMRIS and MIS.TBMMIS"]
pub type TbmcintR = crate::BitReader;
#[doc = "Field `TBMCINT` writer - 11:11\\]
0: Do nothing. 1: Clear RIS.TBMRIS and MIS.TBMMIS"]
pub type TbmcintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED12` reader - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12R = crate::BitReader;
#[doc = "Field `RESERVED12` writer - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMABINT` reader - 13:13\\]
0: Do nothing. 1: Clear RIS.DMABRIS and MIS.DMABMIS"]
pub type DmabintR = crate::BitReader;
#[doc = "Field `DMABINT` writer - 13:13\\]
0: Do nothing. 1: Clear RIS.DMABRIS and MIS.DMABMIS"]
pub type DmabintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED14` reader - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved14R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED14` writer - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved14W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0: Do nothing. 1: Clear RIS.TATORIS and MIS.TATOMIS"]
    #[inline(always)]
    pub fn tatocint(&self) -> TatocintR {
        TatocintR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Do nothing. 1: Clear RIS.CAMRIS and MIS.CAMMIS"]
    #[inline(always)]
    pub fn camcint(&self) -> CamcintR {
        CamcintR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
0: Do nothing. 1: Clear RIS.CAERIS and MIS.CAEMIS"]
    #[inline(always)]
    pub fn caecint(&self) -> CaecintR {
        CaecintR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
0: Do nothing. 1: Clear RIS.TAMRIS and MIS.TAMMIS"]
    #[inline(always)]
    pub fn tamcint(&self) -> TamcintR {
        TamcintR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
0: Do nothing. 1: Clear RIS.DMAARIS and MIS.DMAAMIS"]
    #[inline(always)]
    pub fn dmaaint(&self) -> DmaaintR {
        DmaaintR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
0: Do nothing. 1: Clear RIS.TBTORIS and MIS.TBTOMIS"]
    #[inline(always)]
    pub fn tbtocint(&self) -> TbtocintR {
        TbtocintR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
0: Do nothing. 1: Clear RIS.CBMRIS and MIS.CBMMIS"]
    #[inline(always)]
    pub fn cbmcint(&self) -> CbmcintR {
        CbmcintR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
0: Do nothing. 1: Clear RIS.CBERIS and MIS.CBEMIS"]
    #[inline(always)]
    pub fn cbecint(&self) -> CbecintR {
        CbecintR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
0: Do nothing. 1: Clear RIS.TBMRIS and MIS.TBMMIS"]
    #[inline(always)]
    pub fn tbmcint(&self) -> TbmcintR {
        TbmcintR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
0: Do nothing. 1: Clear RIS.DMABRIS and MIS.DMABMIS"]
    #[inline(always)]
    pub fn dmabint(&self) -> DmabintR {
        DmabintR::new(((self.bits >> 13) & 1) != 0)
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
0: Do nothing. 1: Clear RIS.TATORIS and MIS.TATOMIS"]
    #[inline(always)]
    #[must_use]
    pub fn tatocint(&mut self) -> TatocintW<IclrSpec> {
        TatocintW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Do nothing. 1: Clear RIS.CAMRIS and MIS.CAMMIS"]
    #[inline(always)]
    #[must_use]
    pub fn camcint(&mut self) -> CamcintW<IclrSpec> {
        CamcintW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
0: Do nothing. 1: Clear RIS.CAERIS and MIS.CAEMIS"]
    #[inline(always)]
    #[must_use]
    pub fn caecint(&mut self) -> CaecintW<IclrSpec> {
        CaecintW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<IclrSpec> {
        Reserved3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
0: Do nothing. 1: Clear RIS.TAMRIS and MIS.TAMMIS"]
    #[inline(always)]
    #[must_use]
    pub fn tamcint(&mut self) -> TamcintW<IclrSpec> {
        TamcintW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
0: Do nothing. 1: Clear RIS.DMAARIS and MIS.DMAAMIS"]
    #[inline(always)]
    #[must_use]
    pub fn dmaaint(&mut self) -> DmaaintW<IclrSpec> {
        DmaaintW::new(self, 5)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<IclrSpec> {
        Reserved6W::new(self, 6)
    }
    #[doc = "Bit 8 - 8:8\\]
0: Do nothing. 1: Clear RIS.TBTORIS and MIS.TBTOMIS"]
    #[inline(always)]
    #[must_use]
    pub fn tbtocint(&mut self) -> TbtocintW<IclrSpec> {
        TbtocintW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
0: Do nothing. 1: Clear RIS.CBMRIS and MIS.CBMMIS"]
    #[inline(always)]
    #[must_use]
    pub fn cbmcint(&mut self) -> CbmcintW<IclrSpec> {
        CbmcintW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
0: Do nothing. 1: Clear RIS.CBERIS and MIS.CBEMIS"]
    #[inline(always)]
    #[must_use]
    pub fn cbecint(&mut self) -> CbecintW<IclrSpec> {
        CbecintW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
0: Do nothing. 1: Clear RIS.TBMRIS and MIS.TBMMIS"]
    #[inline(always)]
    #[must_use]
    pub fn tbmcint(&mut self) -> TbmcintW<IclrSpec> {
        TbmcintW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> Reserved12W<IclrSpec> {
        Reserved12W::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
0: Do nothing. 1: Clear RIS.DMABRIS and MIS.DMABMIS"]
    #[inline(always)]
    #[must_use]
    pub fn dmabint(&mut self) -> DmabintW<IclrSpec> {
        DmabintW::new(self, 13)
    }
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved14(&mut self) -> Reserved14W<IclrSpec> {
        Reserved14W::new(self, 14)
    }
}
#[doc = "Interrupt Clear This register is used to clear status bits in the RIS and MIS registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IclrSpec;
impl crate::RegisterSpec for IclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iclr::R`](R) reader structure"]
impl crate::Readable for IclrSpec {}
#[doc = "`write(|w| ..)` method takes [`iclr::W`](W) writer structure"]
impl crate::Writable for IclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICLR to value 0"]
impl crate::Resettable for IclrSpec {
    const RESET_VALUE: u32 = 0;
}
