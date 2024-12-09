#[doc = "Register `MSA` reader"]
pub type R = crate::R<MsaSpec>;
#[doc = "Register `MSA` writer"]
pub type W = crate::W<MsaSpec>;
#[doc = "Field `RS` reader - 0:0\\]
Receive or Send This bit-field specifies the next operation with addressed slave SA. 0 - Transmit/send data to slave 1 - Receive data from slave"]
pub type RsR = crate::BitReader;
#[doc = "Field `RS` writer - 0:0\\]
Receive or Send This bit-field specifies the next operation with addressed slave SA. 0 - Transmit/send data to slave 1 - Receive data from slave"]
pub type RsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SA` reader - 7:1\\]
*I2C* master slave address Defines which slave is addressed for the transaction in master mode"]
pub type SaR = crate::FieldReader;
#[doc = "Field `SA` writer - 7:1\\]
*I2C* master slave address Defines which slave is addressed for the transaction in master mode"]
pub type SaW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Reads to this field return zero.Writes to this field are ignored."]
pub type Reserved8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Receive or Send This bit-field specifies the next operation with addressed slave SA. 0 - Transmit/send data to slave 1 - Receive data from slave"]
    #[inline(always)]
    pub fn rs(&self) -> RsR {
        RsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
*I2C* master slave address Defines which slave is addressed for the transaction in master mode"]
    #[inline(always)]
    pub fn sa(&self) -> SaR {
        SaR::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reads to this field return zero.Writes to this field are ignored."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Receive or Send This bit-field specifies the next operation with addressed slave SA. 0 - Transmit/send data to slave 1 - Receive data from slave"]
    #[inline(always)]
    #[must_use]
    pub fn rs(&mut self) -> RsW<MsaSpec> {
        RsW::new(self, 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
*I2C* master slave address Defines which slave is addressed for the transaction in master mode"]
    #[inline(always)]
    #[must_use]
    pub fn sa(&mut self) -> SaW<MsaSpec> {
        SaW::new(self, 1)
    }
}
#[doc = "Master Slave Address This register contains seven address bits of the slave to be accessed by the master (a6-a0), and an RS bit determining if the next operation is a receive or transmit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsaSpec;
impl crate::RegisterSpec for MsaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msa::R`](R) reader structure"]
impl crate::Readable for MsaSpec {}
#[doc = "`write(|w| ..)` method takes [`msa::W`](W) writer structure"]
impl crate::Writable for MsaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSA to value 0"]
impl crate::Resettable for MsaSpec {
    const RESET_VALUE: u32 = 0;
}
