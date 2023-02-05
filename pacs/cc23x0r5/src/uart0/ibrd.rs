#[doc = "Register `IBRD` reader"]
pub type R = crate::R<IbrdSpec>;
#[doc = "Register `IBRD` writer"]
pub type W = crate::W<IbrdSpec>;
#[doc = "Field `DIVINT` reader - 15:0\\]
The integer baud rate divisor: The baud rate divisor is calculated using the formula below: Baud rate divisor = (UART reference clock frequency) / (16 * Baud rate) Baud rate divisor must be minimum 1 and maximum 65535. That is, DIVINT=0 does not give a valid baud rate. Similarly, if DIVINT=0xFFFF, any non-zero values in FBRD.DIVFRAC will be illegal. A valid value must be written to this field before the UART can be used for RX or TX operations."]
pub type DivintR = crate::FieldReader<u16>;
#[doc = "Field `DIVINT` writer - 15:0\\]
The integer baud rate divisor: The baud rate divisor is calculated using the formula below: Baud rate divisor = (UART reference clock frequency) / (16 * Baud rate) Baud rate divisor must be minimum 1 and maximum 65535. That is, DIVINT=0 does not give a valid baud rate. Similarly, if DIVINT=0xFFFF, any non-zero values in FBRD.DIVFRAC will be illegal. A valid value must be written to this field before the UART can be used for RX or TX operations."]
pub type DivintW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Reads to this field return zero, writes to this field are ignored."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Reads to this field return zero, writes to this field are ignored."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
The integer baud rate divisor: The baud rate divisor is calculated using the formula below: Baud rate divisor = (UART reference clock frequency) / (16 * Baud rate) Baud rate divisor must be minimum 1 and maximum 65535. That is, DIVINT=0 does not give a valid baud rate. Similarly, if DIVINT=0xFFFF, any non-zero values in FBRD.DIVFRAC will be illegal. A valid value must be written to this field before the UART can be used for RX or TX operations."]
    #[inline(always)]
    pub fn divint(&self) -> DivintR {
        DivintR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reads to this field return zero, writes to this field are ignored."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
The integer baud rate divisor: The baud rate divisor is calculated using the formula below: Baud rate divisor = (UART reference clock frequency) / (16 * Baud rate) Baud rate divisor must be minimum 1 and maximum 65535. That is, DIVINT=0 does not give a valid baud rate. Similarly, if DIVINT=0xFFFF, any non-zero values in FBRD.DIVFRAC will be illegal. A valid value must be written to this field before the UART can be used for RX or TX operations."]
    #[inline(always)]
    #[must_use]
    pub fn divint(&mut self) -> DivintW<IbrdSpec> {
        DivintW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reads to this field return zero, writes to this field are ignored."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<IbrdSpec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Integer Baud-Rate Divisor If this register is modified while trasmission or reception is on-going, the baudrate will not be updated until transmission or reception of the current character is complete.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ibrd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ibrd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IbrdSpec;
impl crate::RegisterSpec for IbrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ibrd::R`](R) reader structure"]
impl crate::Readable for IbrdSpec {}
#[doc = "`write(|w| ..)` method takes [`ibrd::W`](W) writer structure"]
impl crate::Writable for IbrdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IBRD to value 0"]
impl crate::Resettable for IbrdSpec {
    const RESET_VALUE: u32 = 0;
}
