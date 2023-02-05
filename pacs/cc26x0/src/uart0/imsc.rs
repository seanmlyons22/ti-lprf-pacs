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
#[doc = "Field `RESERVED0` reader - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED0` writer - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMSC_SPEC, bool, O>;
#[doc = "Field `CTSMIM` reader - 1:1\\]
Clear to Send (CTS) modem interrupt mask. A read returns the current mask for UART's clear to send interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.CTSMMIS. A write of 0 clears the mask which means MIS.CTSMMIS will not reflect the interrupt."]
pub type CTSMIM_R = crate::BitReader<bool>;
#[doc = "Field `CTSMIM` writer - 1:1\\]
Clear to Send (CTS) modem interrupt mask. A read returns the current mask for UART's clear to send interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.CTSMMIS. A write of 0 clears the mask which means MIS.CTSMMIS will not reflect the interrupt."]
pub type CTSMIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMSC_SPEC, bool, O>;
#[doc = "Field `RESERVED2` reader - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED2` writer - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IMSC_SPEC, u8, u8, 2, O>;
#[doc = "Field `RXIM` reader - 4:4\\]
Receive interrupt mask. A read returns the current mask for UART's receive interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.RXMIS. A write of 0 clears the mask which means MIS.RXMIS will not reflect the interrupt."]
pub type RXIM_R = crate::BitReader<bool>;
#[doc = "Field `RXIM` writer - 4:4\\]
Receive interrupt mask. A read returns the current mask for UART's receive interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.RXMIS. A write of 0 clears the mask which means MIS.RXMIS will not reflect the interrupt."]
pub type RXIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMSC_SPEC, bool, O>;
#[doc = "Field `TXIM` reader - 5:5\\]
Transmit interrupt mask. A read returns the current mask for UART's transmit interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.TXMIS. A write of 0 clears the mask which means MIS.TXMIS will not reflect the interrupt."]
pub type TXIM_R = crate::BitReader<bool>;
#[doc = "Field `TXIM` writer - 5:5\\]
Transmit interrupt mask. A read returns the current mask for UART's transmit interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.TXMIS. A write of 0 clears the mask which means MIS.TXMIS will not reflect the interrupt."]
pub type TXIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMSC_SPEC, bool, O>;
#[doc = "Field `RTIM` reader - 6:6\\]
Receive timeout interrupt mask. A read returns the current mask for UART's receive timeout interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.RTMIS. A write of 0 clears the mask which means this bitfield will not reflect the interrupt. The raw interrupt for receive timeout RIS.RTRIS cannot be set unless the mask is set (RTIM = 1). This is because the mask acts as an enable for power saving. That is, the same status can be read from MIS.RTMIS and RIS.RTRIS."]
pub type RTIM_R = crate::BitReader<bool>;
#[doc = "Field `RTIM` writer - 6:6\\]
Receive timeout interrupt mask. A read returns the current mask for UART's receive timeout interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.RTMIS. A write of 0 clears the mask which means this bitfield will not reflect the interrupt. The raw interrupt for receive timeout RIS.RTRIS cannot be set unless the mask is set (RTIM = 1). This is because the mask acts as an enable for power saving. That is, the same status can be read from MIS.RTMIS and RIS.RTRIS."]
pub type RTIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMSC_SPEC, bool, O>;
#[doc = "Field `FEIM` reader - 7:7\\]
Framing error interrupt mask. A read returns the current mask for UART's framing error interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.FEMIS. A write of 0 clears the mask which means MIS.FEMIS will not reflect the interrupt."]
pub type FEIM_R = crate::BitReader<bool>;
#[doc = "Field `FEIM` writer - 7:7\\]
Framing error interrupt mask. A read returns the current mask for UART's framing error interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.FEMIS. A write of 0 clears the mask which means MIS.FEMIS will not reflect the interrupt."]
pub type FEIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMSC_SPEC, bool, O>;
#[doc = "Field `PEIM` reader - 8:8\\]
Parity error interrupt mask. A read returns the current mask for UART's parity error interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.PEMIS. A write of 0 clears the mask which means MIS.PEMIS will not reflect the interrupt."]
pub type PEIM_R = crate::BitReader<bool>;
#[doc = "Field `PEIM` writer - 8:8\\]
Parity error interrupt mask. A read returns the current mask for UART's parity error interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.PEMIS. A write of 0 clears the mask which means MIS.PEMIS will not reflect the interrupt."]
pub type PEIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMSC_SPEC, bool, O>;
#[doc = "Field `BEIM` reader - 9:9\\]
Break error interrupt mask. A read returns the current mask for UART's break error interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.BEMIS. A write of 0 clears the mask which means MIS.BEMIS will not reflect the interrupt."]
pub type BEIM_R = crate::BitReader<bool>;
#[doc = "Field `BEIM` writer - 9:9\\]
Break error interrupt mask. A read returns the current mask for UART's break error interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.BEMIS. A write of 0 clears the mask which means MIS.BEMIS will not reflect the interrupt."]
pub type BEIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMSC_SPEC, bool, O>;
#[doc = "Field `OEIM` reader - 10:10\\]
Overrun error interrupt mask. A read returns the current mask for UART's overrun error interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.OEMIS. A write of 0 clears the mask which means MIS.OEMIS will not reflect the interrupt."]
pub type OEIM_R = crate::BitReader<bool>;
#[doc = "Field `OEIM` writer - 10:10\\]
Overrun error interrupt mask. A read returns the current mask for UART's overrun error interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.OEMIS. A write of 0 clears the mask which means MIS.OEMIS will not reflect the interrupt."]
pub type OEIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMSC_SPEC, bool, O>;
#[doc = "Field `RESERVED11` reader - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED11_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED11` writer - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IMSC_SPEC, u32, u32, 21, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clear to Send (CTS) modem interrupt mask. A read returns the current mask for UART's clear to send interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.CTSMMIS. A write of 0 clears the mask which means MIS.CTSMMIS will not reflect the interrupt."]
    #[inline(always)]
    pub fn ctsmim(&self) -> CTSMIM_R {
        CTSMIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Receive interrupt mask. A read returns the current mask for UART's receive interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.RXMIS. A write of 0 clears the mask which means MIS.RXMIS will not reflect the interrupt."]
    #[inline(always)]
    pub fn rxim(&self) -> RXIM_R {
        RXIM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Transmit interrupt mask. A read returns the current mask for UART's transmit interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.TXMIS. A write of 0 clears the mask which means MIS.TXMIS will not reflect the interrupt."]
    #[inline(always)]
    pub fn txim(&self) -> TXIM_R {
        TXIM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Receive timeout interrupt mask. A read returns the current mask for UART's receive timeout interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.RTMIS. A write of 0 clears the mask which means this bitfield will not reflect the interrupt. The raw interrupt for receive timeout RIS.RTRIS cannot be set unless the mask is set (RTIM = 1). This is because the mask acts as an enable for power saving. That is, the same status can be read from MIS.RTMIS and RIS.RTRIS."]
    #[inline(always)]
    pub fn rtim(&self) -> RTIM_R {
        RTIM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Framing error interrupt mask. A read returns the current mask for UART's framing error interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.FEMIS. A write of 0 clears the mask which means MIS.FEMIS will not reflect the interrupt."]
    #[inline(always)]
    pub fn feim(&self) -> FEIM_R {
        FEIM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Parity error interrupt mask. A read returns the current mask for UART's parity error interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.PEMIS. A write of 0 clears the mask which means MIS.PEMIS will not reflect the interrupt."]
    #[inline(always)]
    pub fn peim(&self) -> PEIM_R {
        PEIM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Break error interrupt mask. A read returns the current mask for UART's break error interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.BEMIS. A write of 0 clears the mask which means MIS.BEMIS will not reflect the interrupt."]
    #[inline(always)]
    pub fn beim(&self) -> BEIM_R {
        BEIM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Overrun error interrupt mask. A read returns the current mask for UART's overrun error interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.OEMIS. A write of 0 clears the mask which means MIS.OEMIS will not reflect the interrupt."]
    #[inline(always)]
    pub fn oeim(&self) -> OEIM_R {
        OEIM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> RESERVED11_R {
        RESERVED11_R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<0> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Clear to Send (CTS) modem interrupt mask. A read returns the current mask for UART's clear to send interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.CTSMMIS. A write of 0 clears the mask which means MIS.CTSMMIS will not reflect the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ctsmim(&mut self) -> CTSMIM_W<1> {
        CTSMIM_W::new(self)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Receive interrupt mask. A read returns the current mask for UART's receive interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.RXMIS. A write of 0 clears the mask which means MIS.RXMIS will not reflect the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rxim(&mut self) -> RXIM_W<4> {
        RXIM_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Transmit interrupt mask. A read returns the current mask for UART's transmit interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.TXMIS. A write of 0 clears the mask which means MIS.TXMIS will not reflect the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn txim(&mut self) -> TXIM_W<5> {
        TXIM_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Receive timeout interrupt mask. A read returns the current mask for UART's receive timeout interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.RTMIS. A write of 0 clears the mask which means this bitfield will not reflect the interrupt. The raw interrupt for receive timeout RIS.RTRIS cannot be set unless the mask is set (RTIM = 1). This is because the mask acts as an enable for power saving. That is, the same status can be read from MIS.RTMIS and RIS.RTRIS."]
    #[inline(always)]
    #[must_use]
    pub fn rtim(&mut self) -> RTIM_W<6> {
        RTIM_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Framing error interrupt mask. A read returns the current mask for UART's framing error interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.FEMIS. A write of 0 clears the mask which means MIS.FEMIS will not reflect the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn feim(&mut self) -> FEIM_W<7> {
        FEIM_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Parity error interrupt mask. A read returns the current mask for UART's parity error interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.PEMIS. A write of 0 clears the mask which means MIS.PEMIS will not reflect the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn peim(&mut self) -> PEIM_W<8> {
        PEIM_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Break error interrupt mask. A read returns the current mask for UART's break error interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.BEMIS. A write of 0 clears the mask which means MIS.BEMIS will not reflect the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn beim(&mut self) -> BEIM_W<9> {
        BEIM_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
Overrun error interrupt mask. A read returns the current mask for UART's overrun error interrupt. On a write of 1, the mask of the overrun error interrupt is set which means the interrupt state will be reflected in MIS.OEMIS. A write of 0 clears the mask which means MIS.OEMIS will not reflect the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn oeim(&mut self) -> OEIM_W<10> {
        OEIM_W::new(self)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved11(&mut self) -> RESERVED11_W<11> {
        RESERVED11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Mask Set/Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imsc](index.html) module"]
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
