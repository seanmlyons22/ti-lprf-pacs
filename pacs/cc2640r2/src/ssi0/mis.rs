#[doc = "Register `MIS` reader"]
pub struct R(crate::R<MIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIS` writer"]
pub struct W(crate::W<MIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIS_SPEC>;
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
impl From<crate::W<MIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RORMIS` reader - 0:0\\]
Masked interrupt state of receive overrun interrupt: This field returns the masked interrupt state of receive overrun interrupt which is the AND product of raw interrupt state RIS.RORRIS and the mask setting IMSC.RORIM."]
pub type RORMIS_R = crate::BitReader<bool>;
#[doc = "Field `RORMIS` writer - 0:0\\]
Masked interrupt state of receive overrun interrupt: This field returns the masked interrupt state of receive overrun interrupt which is the AND product of raw interrupt state RIS.RORRIS and the mask setting IMSC.RORIM."]
pub type RORMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIS_SPEC, bool, O>;
#[doc = "Field `RTMIS` reader - 1:1\\]
Masked interrupt state of receive timeout interrupt: This field returns the masked interrupt state of receive timeout interrupt which is the AND product of raw interrupt state RIS.RTRIS and the mask setting IMSC.RTIM."]
pub type RTMIS_R = crate::BitReader<bool>;
#[doc = "Field `RTMIS` writer - 1:1\\]
Masked interrupt state of receive timeout interrupt: This field returns the masked interrupt state of receive timeout interrupt which is the AND product of raw interrupt state RIS.RTRIS and the mask setting IMSC.RTIM."]
pub type RTMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIS_SPEC, bool, O>;
#[doc = "Field `RXMIS` reader - 2:2\\]
Masked interrupt state of receive FIFO interrupt: This field returns the masked interrupt state of receive FIFO interrupt which is the AND product of raw interrupt state RIS.RXRIS and the mask setting IMSC.RXIM."]
pub type RXMIS_R = crate::BitReader<bool>;
#[doc = "Field `RXMIS` writer - 2:2\\]
Masked interrupt state of receive FIFO interrupt: This field returns the masked interrupt state of receive FIFO interrupt which is the AND product of raw interrupt state RIS.RXRIS and the mask setting IMSC.RXIM."]
pub type RXMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIS_SPEC, bool, O>;
#[doc = "Field `TXMIS` reader - 3:3\\]
Masked interrupt state of transmit FIFO interrupt: This field returns the masked interrupt state of transmit FIFO interrupt which is the AND product of raw interrupt state RIS.TXRIS and the mask setting IMSC.TXIM."]
pub type TXMIS_R = crate::BitReader<bool>;
#[doc = "Field `TXMIS` writer - 3:3\\]
Masked interrupt state of transmit FIFO interrupt: This field returns the masked interrupt state of transmit FIFO interrupt which is the AND product of raw interrupt state RIS.TXRIS and the mask setting IMSC.TXIM."]
pub type TXMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Masked interrupt state of receive overrun interrupt: This field returns the masked interrupt state of receive overrun interrupt which is the AND product of raw interrupt state RIS.RORRIS and the mask setting IMSC.RORIM."]
    #[inline(always)]
    pub fn rormis(&self) -> RORMIS_R {
        RORMIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Masked interrupt state of receive timeout interrupt: This field returns the masked interrupt state of receive timeout interrupt which is the AND product of raw interrupt state RIS.RTRIS and the mask setting IMSC.RTIM."]
    #[inline(always)]
    pub fn rtmis(&self) -> RTMIS_R {
        RTMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Masked interrupt state of receive FIFO interrupt: This field returns the masked interrupt state of receive FIFO interrupt which is the AND product of raw interrupt state RIS.RXRIS and the mask setting IMSC.RXIM."]
    #[inline(always)]
    pub fn rxmis(&self) -> RXMIS_R {
        RXMIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Masked interrupt state of transmit FIFO interrupt: This field returns the masked interrupt state of transmit FIFO interrupt which is the AND product of raw interrupt state RIS.TXRIS and the mask setting IMSC.TXIM."]
    #[inline(always)]
    pub fn txmis(&self) -> TXMIS_R {
        TXMIS_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Masked interrupt state of receive overrun interrupt: This field returns the masked interrupt state of receive overrun interrupt which is the AND product of raw interrupt state RIS.RORRIS and the mask setting IMSC.RORIM."]
    #[inline(always)]
    #[must_use]
    pub fn rormis(&mut self) -> RORMIS_W<0> {
        RORMIS_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Masked interrupt state of receive timeout interrupt: This field returns the masked interrupt state of receive timeout interrupt which is the AND product of raw interrupt state RIS.RTRIS and the mask setting IMSC.RTIM."]
    #[inline(always)]
    #[must_use]
    pub fn rtmis(&mut self) -> RTMIS_W<1> {
        RTMIS_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Masked interrupt state of receive FIFO interrupt: This field returns the masked interrupt state of receive FIFO interrupt which is the AND product of raw interrupt state RIS.RXRIS and the mask setting IMSC.RXIM."]
    #[inline(always)]
    #[must_use]
    pub fn rxmis(&mut self) -> RXMIS_W<2> {
        RXMIS_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Masked interrupt state of transmit FIFO interrupt: This field returns the masked interrupt state of transmit FIFO interrupt which is the AND product of raw interrupt state RIS.TXRIS and the mask setting IMSC.TXIM."]
    #[inline(always)]
    #[must_use]
    pub fn txmis(&mut self) -> TXMIS_W<3> {
        TXMIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Masked Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mis](index.html) module"]
pub struct MIS_SPEC;
impl crate::RegisterSpec for MIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mis::R](R) reader structure"]
impl crate::Readable for MIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mis::W](W) writer structure"]
impl crate::Writable for MIS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
