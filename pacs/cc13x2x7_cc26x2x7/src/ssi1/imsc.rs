#[doc = "Register `IMSC` reader"]
pub struct R(crate::R<IMSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMSC` writer"]
pub struct W(crate::W<IMSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMSC_SPEC>;
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
impl From<crate::W<IMSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RORIM` reader - 0:0\\]
Receive overrun interrupt mask: A read returns the current mask for receive overrun interrupt. On a write of 1, the mask for receive overrun interrupt is set which means the interrupt state will be reflected in MIS.RORMIS. A write of 0 clears the mask which means MIS.RORMIS will not reflect the interrupt."]
pub type RORIM_R = crate::BitReader<bool>;
#[doc = "Field `RORIM` writer - 0:0\\]
Receive overrun interrupt mask: A read returns the current mask for receive overrun interrupt. On a write of 1, the mask for receive overrun interrupt is set which means the interrupt state will be reflected in MIS.RORMIS. A write of 0 clears the mask which means MIS.RORMIS will not reflect the interrupt."]
pub type RORIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMSC_SPEC, bool, O>;
#[doc = "Field `RTIM` reader - 1:1\\]
Receive timeout interrupt mask: A read returns the current mask for receive timeout interrupt. On a write of 1, the mask for receive timeout interrupt is set which means the interrupt state will be reflected in MIS.RTMIS. A write of 0 clears the mask which means MIS.RTMIS will not reflect the interrupt."]
pub type RTIM_R = crate::BitReader<bool>;
#[doc = "Field `RTIM` writer - 1:1\\]
Receive timeout interrupt mask: A read returns the current mask for receive timeout interrupt. On a write of 1, the mask for receive timeout interrupt is set which means the interrupt state will be reflected in MIS.RTMIS. A write of 0 clears the mask which means MIS.RTMIS will not reflect the interrupt."]
pub type RTIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMSC_SPEC, bool, O>;
#[doc = "Field `RXIM` reader - 2:2\\]
Receive FIFO interrupt mask: A read returns the current mask for receive FIFO interrupt. On a write of 1, the mask for receive FIFO interrupt is set which means the interrupt state will be reflected in MIS.RXMIS. A write of 0 clears the mask which means MIS.RXMIS will not reflect the interrupt."]
pub type RXIM_R = crate::BitReader<bool>;
#[doc = "Field `RXIM` writer - 2:2\\]
Receive FIFO interrupt mask: A read returns the current mask for receive FIFO interrupt. On a write of 1, the mask for receive FIFO interrupt is set which means the interrupt state will be reflected in MIS.RXMIS. A write of 0 clears the mask which means MIS.RXMIS will not reflect the interrupt."]
pub type RXIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMSC_SPEC, bool, O>;
#[doc = "Field `TXIM` reader - 3:3\\]
Transmit FIFO interrupt mask: A read returns the current mask for transmit FIFO interrupt. On a write of 1, the mask for transmit FIFO interrupt is set which means the interrupt state will be reflected in MIS.TXMIS. A write of 0 clears the mask which means MIS.TXMIS will not reflect the interrupt."]
pub type TXIM_R = crate::BitReader<bool>;
#[doc = "Field `TXIM` writer - 3:3\\]
Transmit FIFO interrupt mask: A read returns the current mask for transmit FIFO interrupt. On a write of 1, the mask for transmit FIFO interrupt is set which means the interrupt state will be reflected in MIS.TXMIS. A write of 0 clears the mask which means MIS.TXMIS will not reflect the interrupt."]
pub type TXIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMSC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Receive overrun interrupt mask: A read returns the current mask for receive overrun interrupt. On a write of 1, the mask for receive overrun interrupt is set which means the interrupt state will be reflected in MIS.RORMIS. A write of 0 clears the mask which means MIS.RORMIS will not reflect the interrupt."]
    #[inline(always)]
    pub fn rorim(&self) -> RORIM_R {
        RORIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Receive timeout interrupt mask: A read returns the current mask for receive timeout interrupt. On a write of 1, the mask for receive timeout interrupt is set which means the interrupt state will be reflected in MIS.RTMIS. A write of 0 clears the mask which means MIS.RTMIS will not reflect the interrupt."]
    #[inline(always)]
    pub fn rtim(&self) -> RTIM_R {
        RTIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Receive FIFO interrupt mask: A read returns the current mask for receive FIFO interrupt. On a write of 1, the mask for receive FIFO interrupt is set which means the interrupt state will be reflected in MIS.RXMIS. A write of 0 clears the mask which means MIS.RXMIS will not reflect the interrupt."]
    #[inline(always)]
    pub fn rxim(&self) -> RXIM_R {
        RXIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Transmit FIFO interrupt mask: A read returns the current mask for transmit FIFO interrupt. On a write of 1, the mask for transmit FIFO interrupt is set which means the interrupt state will be reflected in MIS.TXMIS. A write of 0 clears the mask which means MIS.TXMIS will not reflect the interrupt."]
    #[inline(always)]
    pub fn txim(&self) -> TXIM_R {
        TXIM_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Receive overrun interrupt mask: A read returns the current mask for receive overrun interrupt. On a write of 1, the mask for receive overrun interrupt is set which means the interrupt state will be reflected in MIS.RORMIS. A write of 0 clears the mask which means MIS.RORMIS will not reflect the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rorim(&mut self) -> RORIM_W<0> {
        RORIM_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Receive timeout interrupt mask: A read returns the current mask for receive timeout interrupt. On a write of 1, the mask for receive timeout interrupt is set which means the interrupt state will be reflected in MIS.RTMIS. A write of 0 clears the mask which means MIS.RTMIS will not reflect the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rtim(&mut self) -> RTIM_W<1> {
        RTIM_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Receive FIFO interrupt mask: A read returns the current mask for receive FIFO interrupt. On a write of 1, the mask for receive FIFO interrupt is set which means the interrupt state will be reflected in MIS.RXMIS. A write of 0 clears the mask which means MIS.RXMIS will not reflect the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rxim(&mut self) -> RXIM_W<2> {
        RXIM_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Transmit FIFO interrupt mask: A read returns the current mask for transmit FIFO interrupt. On a write of 1, the mask for transmit FIFO interrupt is set which means the interrupt state will be reflected in MIS.TXMIS. A write of 0 clears the mask which means MIS.TXMIS will not reflect the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn txim(&mut self) -> TXIM_W<3> {
        TXIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Mask Set and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imsc](index.html) module"]
pub struct IMSC_SPEC;
impl crate::RegisterSpec for IMSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imsc::R](R) reader structure"]
impl crate::Readable for IMSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imsc::W](W) writer structure"]
impl crate::Writable for IMSC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMSC to value 0"]
impl crate::Resettable for IMSC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
