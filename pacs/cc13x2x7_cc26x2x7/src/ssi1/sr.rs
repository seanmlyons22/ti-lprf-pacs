#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `TFE` reader - 0:0\\]
Transmit FIFO empty: 0: Transmit FIFO is not empty. 1: Transmit FIFO is empty."]
pub type TfeR = crate::BitReader;
#[doc = "Field `TFE` writer - 0:0\\]
Transmit FIFO empty: 0: Transmit FIFO is not empty. 1: Transmit FIFO is empty."]
pub type TfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TNF` reader - 1:1\\]
Transmit FIFO not full: 0: Transmit FIFO is full. 1: Transmit FIFO is not full."]
pub type TnfR = crate::BitReader;
#[doc = "Field `TNF` writer - 1:1\\]
Transmit FIFO not full: 0: Transmit FIFO is full. 1: Transmit FIFO is not full."]
pub type TnfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNE` reader - 2:2\\]
Receive FIFO not empty 0: Receive FIFO is empty. 1: Receive FIFO is not empty."]
pub type RneR = crate::BitReader;
#[doc = "Field `RNE` writer - 2:2\\]
Receive FIFO not empty 0: Receive FIFO is empty. 1: Receive FIFO is not empty."]
pub type RneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFF` reader - 3:3\\]
Receive FIFO full: 0: Receive FIFO is not full. 1: Receive FIFO is full."]
pub type RffR = crate::BitReader;
#[doc = "Field `RFF` writer - 3:3\\]
Receive FIFO full: 0: Receive FIFO is not full. 1: Receive FIFO is full."]
pub type RffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSY` reader - 4:4\\]
Serial interface busy: 0: SSI is idle 1: SSI is currently transmitting and/or receiving a frame or the transmit FIFO is not empty."]
pub type BsyR = crate::BitReader;
#[doc = "Field `BSY` writer - 4:4\\]
Serial interface busy: 0: SSI is idle 1: SSI is currently transmitting and/or receiving a frame or the transmit FIFO is not empty."]
pub type BsyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Transmit FIFO empty: 0: Transmit FIFO is not empty. 1: Transmit FIFO is empty."]
    #[inline(always)]
    pub fn tfe(&self) -> TfeR {
        TfeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Transmit FIFO not full: 0: Transmit FIFO is full. 1: Transmit FIFO is not full."]
    #[inline(always)]
    pub fn tnf(&self) -> TnfR {
        TnfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Receive FIFO not empty 0: Receive FIFO is empty. 1: Receive FIFO is not empty."]
    #[inline(always)]
    pub fn rne(&self) -> RneR {
        RneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Receive FIFO full: 0: Receive FIFO is not full. 1: Receive FIFO is full."]
    #[inline(always)]
    pub fn rff(&self) -> RffR {
        RffR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Serial interface busy: 0: SSI is idle 1: SSI is currently transmitting and/or receiving a frame or the transmit FIFO is not empty."]
    #[inline(always)]
    pub fn bsy(&self) -> BsyR {
        BsyR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Transmit FIFO empty: 0: Transmit FIFO is not empty. 1: Transmit FIFO is empty."]
    #[inline(always)]
    #[must_use]
    pub fn tfe(&mut self) -> TfeW<SrSpec> {
        TfeW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Transmit FIFO not full: 0: Transmit FIFO is full. 1: Transmit FIFO is not full."]
    #[inline(always)]
    #[must_use]
    pub fn tnf(&mut self) -> TnfW<SrSpec> {
        TnfW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Receive FIFO not empty 0: Receive FIFO is empty. 1: Receive FIFO is not empty."]
    #[inline(always)]
    #[must_use]
    pub fn rne(&mut self) -> RneW<SrSpec> {
        RneW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Receive FIFO full: 0: Receive FIFO is not full. 1: Receive FIFO is full."]
    #[inline(always)]
    #[must_use]
    pub fn rff(&mut self) -> RffW<SrSpec> {
        RffW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Serial interface busy: 0: SSI is idle 1: SSI is currently transmitting and/or receiving a frame or the transmit FIFO is not empty."]
    #[inline(always)]
    #[must_use]
    pub fn bsy(&mut self) -> BsyW<SrSpec> {
        BsyW::new(self, 4)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0x03"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0x03;
}
