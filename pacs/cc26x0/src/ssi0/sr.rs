#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TFE` reader - 0:0\\]
Transmit FIFO empty: 0: Transmit FIFO is not empty. 1: Transmit FIFO is empty."]
pub type TFE_R = crate::BitReader<bool>;
#[doc = "Field `TFE` writer - 0:0\\]
Transmit FIFO empty: 0: Transmit FIFO is not empty. 1: Transmit FIFO is empty."]
pub type TFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `TNF` reader - 1:1\\]
Transmit FIFO not full: 0: Transmit FIFO is full. 1: Transmit FIFO is not full."]
pub type TNF_R = crate::BitReader<bool>;
#[doc = "Field `TNF` writer - 1:1\\]
Transmit FIFO not full: 0: Transmit FIFO is full. 1: Transmit FIFO is not full."]
pub type TNF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `RNE` reader - 2:2\\]
Receive FIFO not empty 0: Receive FIFO is empty. 1: Receive FIFO is not empty."]
pub type RNE_R = crate::BitReader<bool>;
#[doc = "Field `RNE` writer - 2:2\\]
Receive FIFO not empty 0: Receive FIFO is empty. 1: Receive FIFO is not empty."]
pub type RNE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `RFF` reader - 3:3\\]
Receive FIFO full: 0: Receive FIFO is not full. 1: Receive FIFO is full."]
pub type RFF_R = crate::BitReader<bool>;
#[doc = "Field `RFF` writer - 3:3\\]
Receive FIFO full: 0: Receive FIFO is not full. 1: Receive FIFO is full."]
pub type RFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `BSY` reader - 4:4\\]
Serial interface busy: 0: SSI is idle 1: SSI is currently transmitting and/or receiving a frame or the transmit FIFO is not empty."]
pub type BSY_R = crate::BitReader<bool>;
#[doc = "Field `BSY` writer - 4:4\\]
Serial interface busy: 0: SSI is idle 1: SSI is currently transmitting and/or receiving a frame or the transmit FIFO is not empty."]
pub type BSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Transmit FIFO empty: 0: Transmit FIFO is not empty. 1: Transmit FIFO is empty."]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Transmit FIFO not full: 0: Transmit FIFO is full. 1: Transmit FIFO is not full."]
    #[inline(always)]
    pub fn tnf(&self) -> TNF_R {
        TNF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Receive FIFO not empty 0: Receive FIFO is empty. 1: Receive FIFO is not empty."]
    #[inline(always)]
    pub fn rne(&self) -> RNE_R {
        RNE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Receive FIFO full: 0: Receive FIFO is not full. 1: Receive FIFO is full."]
    #[inline(always)]
    pub fn rff(&self) -> RFF_R {
        RFF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Serial interface busy: 0: SSI is idle 1: SSI is currently transmitting and/or receiving a frame or the transmit FIFO is not empty."]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Transmit FIFO empty: 0: Transmit FIFO is not empty. 1: Transmit FIFO is empty."]
    #[inline(always)]
    #[must_use]
    pub fn tfe(&mut self) -> TFE_W<0> {
        TFE_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Transmit FIFO not full: 0: Transmit FIFO is full. 1: Transmit FIFO is not full."]
    #[inline(always)]
    #[must_use]
    pub fn tnf(&mut self) -> TNF_W<1> {
        TNF_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Receive FIFO not empty 0: Receive FIFO is empty. 1: Receive FIFO is not empty."]
    #[inline(always)]
    #[must_use]
    pub fn rne(&mut self) -> RNE_W<2> {
        RNE_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Receive FIFO full: 0: Receive FIFO is not full. 1: Receive FIFO is full."]
    #[inline(always)]
    #[must_use]
    pub fn rff(&mut self) -> RFF_W<3> {
        RFF_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Serial interface busy: 0: SSI is idle 1: SSI is currently transmitting and/or receiving a frame or the transmit FIFO is not empty."]
    #[inline(always)]
    #[must_use]
    pub fn bsy(&mut self) -> BSY_W<4> {
        BSY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SR to value 0x03"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
