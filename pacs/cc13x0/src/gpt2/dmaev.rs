#[doc = "Register `DMAEV` reader"]
pub type R = crate::R<DmaevSpec>;
#[doc = "Register `DMAEV` writer"]
pub type W = crate::W<DmaevSpec>;
#[doc = "Field `TATODMAEN` reader - 0:0\\]
GPT Timer A Time-Out DMA Trigger Enable"]
pub type TatodmaenR = crate::BitReader;
#[doc = "Field `TATODMAEN` writer - 0:0\\]
GPT Timer A Time-Out DMA Trigger Enable"]
pub type TatodmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAMDMAEN` reader - 1:1\\]
GPT Timer A Capture Match DMA Trigger Enable"]
pub type CamdmaenR = crate::BitReader;
#[doc = "Field `CAMDMAEN` writer - 1:1\\]
GPT Timer A Capture Match DMA Trigger Enable"]
pub type CamdmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAEDMAEN` reader - 2:2\\]
GPT Timer A Capture Event DMA Trigger Enable"]
pub type CaedmaenR = crate::BitReader;
#[doc = "Field `CAEDMAEN` writer - 2:2\\]
GPT Timer A Capture Event DMA Trigger Enable"]
pub type CaedmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `RESERVED3` writer - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMDMAEN` reader - 4:4\\]
GPT Timer A Match DMA Trigger Enable"]
pub type TamdmaenR = crate::BitReader;
#[doc = "Field `TAMDMAEN` writer - 4:4\\]
GPT Timer A Match DMA Trigger Enable"]
pub type TamdmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED5` reader - 7:5\\]
Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
pub type Reserved5R = crate::FieldReader;
#[doc = "Field `RESERVED5` writer - 7:5\\]
Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
pub type Reserved5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TBTODMAEN` reader - 8:8\\]
GPT Timer B Time-Out DMA Trigger Enable"]
pub type TbtodmaenR = crate::BitReader;
#[doc = "Field `TBTODMAEN` writer - 8:8\\]
GPT Timer B Time-Out DMA Trigger Enable"]
pub type TbtodmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBMDMAEN` reader - 9:9\\]
GPT Timer B Capture Match DMA Trigger Enable"]
pub type CbmdmaenR = crate::BitReader;
#[doc = "Field `CBMDMAEN` writer - 9:9\\]
GPT Timer B Capture Match DMA Trigger Enable"]
pub type CbmdmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBEDMAEN` reader - 10:10\\]
GPT Timer B Capture Event DMA Trigger Enable"]
pub type CbedmaenR = crate::BitReader;
#[doc = "Field `CBEDMAEN` writer - 10:10\\]
GPT Timer B Capture Event DMA Trigger Enable"]
pub type CbedmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBMDMAEN` reader - 11:11\\]
GPT Timer B Match DMA Trigger Enable"]
pub type TbmdmaenR = crate::BitReader;
#[doc = "Field `TBMDMAEN` writer - 11:11\\]
GPT Timer B Match DMA Trigger Enable"]
pub type TbmdmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED12` reader - 31:12\\]
Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
pub type Reserved12R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED12` writer - 31:12\\]
Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
pub type Reserved12W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
GPT Timer A Time-Out DMA Trigger Enable"]
    #[inline(always)]
    pub fn tatodmaen(&self) -> TatodmaenR {
        TatodmaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
GPT Timer A Capture Match DMA Trigger Enable"]
    #[inline(always)]
    pub fn camdmaen(&self) -> CamdmaenR {
        CamdmaenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
GPT Timer A Capture Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn caedmaen(&self) -> CaedmaenR {
        CaedmaenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
GPT Timer A Match DMA Trigger Enable"]
    #[inline(always)]
    pub fn tamdmaen(&self) -> TamdmaenR {
        TamdmaenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
GPT Timer B Time-Out DMA Trigger Enable"]
    #[inline(always)]
    pub fn tbtodmaen(&self) -> TbtodmaenR {
        TbtodmaenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
GPT Timer B Capture Match DMA Trigger Enable"]
    #[inline(always)]
    pub fn cbmdmaen(&self) -> CbmdmaenR {
        CbmdmaenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
GPT Timer B Capture Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn cbedmaen(&self) -> CbedmaenR {
        CbedmaenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
GPT Timer B Match DMA Trigger Enable"]
    #[inline(always)]
    pub fn tbmdmaen(&self) -> TbmdmaenR {
        TbmdmaenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
GPT Timer A Time-Out DMA Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tatodmaen(&mut self) -> TatodmaenW<DmaevSpec> {
        TatodmaenW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
GPT Timer A Capture Match DMA Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn camdmaen(&mut self) -> CamdmaenW<DmaevSpec> {
        CamdmaenW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
GPT Timer A Capture Event DMA Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn caedmaen(&mut self) -> CaedmaenW<DmaevSpec> {
        CaedmaenW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<DmaevSpec> {
        Reserved3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
GPT Timer A Match DMA Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tamdmaen(&mut self) -> TamdmaenW<DmaevSpec> {
        TamdmaenW::new(self, 4)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> Reserved5W<DmaevSpec> {
        Reserved5W::new(self, 5)
    }
    #[doc = "Bit 8 - 8:8\\]
GPT Timer B Time-Out DMA Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tbtodmaen(&mut self) -> TbtodmaenW<DmaevSpec> {
        TbtodmaenW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
GPT Timer B Capture Match DMA Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cbmdmaen(&mut self) -> CbmdmaenW<DmaevSpec> {
        CbmdmaenW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
GPT Timer B Capture Event DMA Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cbedmaen(&mut self) -> CbedmaenW<DmaevSpec> {
        CbedmaenW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
GPT Timer B Match DMA Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tbmdmaen(&mut self) -> TbmdmaenW<DmaevSpec> {
        TbmdmaenW::new(self, 11)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> Reserved12W<DmaevSpec> {
        Reserved12W::new(self, 12)
    }
}
#[doc = "DMA Event This register allows software to enable/disable GPT DMA trigger events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaev::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaev::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaevSpec;
impl crate::RegisterSpec for DmaevSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaev::R`](R) reader structure"]
impl crate::Readable for DmaevSpec {}
#[doc = "`write(|w| ..)` method takes [`dmaev::W`](W) writer structure"]
impl crate::Writable for DmaevSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAEV to value 0"]
impl crate::Resettable for DmaevSpec {
    const RESET_VALUE: u32 = 0;
}
