#[doc = "Register `FBRD` reader"]
pub type R = crate::R<FbrdSpec>;
#[doc = "Register `FBRD` writer"]
pub type W = crate::W<FbrdSpec>;
#[doc = "Field `DIVFRAC` reader - 5:0\\]
Fractional Baud-Rate Divisor: The baud rate divisor is calculated using the formula below: Baud rate divisor = (UART reference clock frequency) / (16 * Baud rate) Baud rate divisor must be minimum 1 and maximum 65535. That is, IBRD.DIVINT=0 does not give a valid baud rate. Similarly, if IBRD.DIVINT=0xFFFF, any non-zero values in DIVFRAC will be illegal. A valid value must be written to this field before the UART can be used for RX or TX operations."]
pub type DivfracR = crate::FieldReader;
#[doc = "Field `DIVFRAC` writer - 5:0\\]
Fractional Baud-Rate Divisor: The baud rate divisor is calculated using the formula below: Baud rate divisor = (UART reference clock frequency) / (16 * Baud rate) Baud rate divisor must be minimum 1 and maximum 65535. That is, IBRD.DIVINT=0 does not give a valid baud rate. Similarly, if IBRD.DIVINT=0xFFFF, any non-zero values in DIVFRAC will be illegal. A valid value must be written to this field before the UART can be used for RX or TX operations."]
pub type DivfracW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Fractional Baud-Rate Divisor: The baud rate divisor is calculated using the formula below: Baud rate divisor = (UART reference clock frequency) / (16 * Baud rate) Baud rate divisor must be minimum 1 and maximum 65535. That is, IBRD.DIVINT=0 does not give a valid baud rate. Similarly, if IBRD.DIVINT=0xFFFF, any non-zero values in DIVFRAC will be illegal. A valid value must be written to this field before the UART can be used for RX or TX operations."]
    #[inline(always)]
    pub fn divfrac(&self) -> DivfracR {
        DivfracR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Fractional Baud-Rate Divisor: The baud rate divisor is calculated using the formula below: Baud rate divisor = (UART reference clock frequency) / (16 * Baud rate) Baud rate divisor must be minimum 1 and maximum 65535. That is, IBRD.DIVINT=0 does not give a valid baud rate. Similarly, if IBRD.DIVINT=0xFFFF, any non-zero values in DIVFRAC will be illegal. A valid value must be written to this field before the UART can be used for RX or TX operations."]
    #[inline(always)]
    #[must_use]
    pub fn divfrac(&mut self) -> DivfracW<FbrdSpec> {
        DivfracW::new(self, 0)
    }
}
#[doc = "Fractional Baud-Rate Divisor If this register is modified while trasmission or reception is on-going, the baudrate will not be updated until transmission or reception of the current character is complete.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fbrd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fbrd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FbrdSpec;
impl crate::RegisterSpec for FbrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fbrd::R`](R) reader structure"]
impl crate::Readable for FbrdSpec {}
#[doc = "`write(|w| ..)` method takes [`fbrd::W`](W) writer structure"]
impl crate::Writable for FbrdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FBRD to value 0"]
impl crate::Resettable for FbrdSpec {
    const RESET_VALUE: u32 = 0;
}
