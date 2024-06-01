#[doc = "Register `MSA` reader"]
pub type R = crate::R<MsaSpec>;
#[doc = "Register `MSA` writer"]
pub type W = crate::W<MsaSpec>;
#[doc = "0:0\\]
Receive or Send This bit-field specifies if the next operation is a receive (high) or a transmit/send (low) from the addressed slave SA.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rs {
    #[doc = "1: Receive data from slave"]
    Rx = 1,
    #[doc = "0: Transmit/send data to slave"]
    Tx = 0,
}
impl From<Rs> for bool {
    #[inline(always)]
    fn from(variant: Rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RS` reader - 0:0\\]
Receive or Send This bit-field specifies if the next operation is a receive (high) or a transmit/send (low) from the addressed slave SA."]
pub type RsR = crate::BitReader<Rs>;
impl RsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rs {
        match self.bits {
            true => Rs::Rx,
            false => Rs::Tx,
        }
    }
    #[doc = "Receive data from slave"]
    #[inline(always)]
    pub fn is_rx(&self) -> bool {
        *self == Rs::Rx
    }
    #[doc = "Transmit/send data to slave"]
    #[inline(always)]
    pub fn is_tx(&self) -> bool {
        *self == Rs::Tx
    }
}
#[doc = "Field `RS` writer - 0:0\\]
Receive or Send This bit-field specifies if the next operation is a receive (high) or a transmit/send (low) from the addressed slave SA."]
pub type RsW<'a, REG> = crate::BitWriter<'a, REG, Rs>;
impl<'a, REG> RsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive data from slave"]
    #[inline(always)]
    pub fn rx(self) -> &'a mut crate::W<REG> {
        self.variant(Rs::Rx)
    }
    #[doc = "Transmit/send data to slave"]
    #[inline(always)]
    pub fn tx(self) -> &'a mut crate::W<REG> {
        self.variant(Rs::Tx)
    }
}
#[doc = "Field `SA` reader - 7:1\\]
I2C master slave address Defines which slave is addressed for the transaction in master mode"]
pub type SaR = crate::FieldReader;
#[doc = "Field `SA` writer - 7:1\\]
I2C master slave address Defines which slave is addressed for the transaction in master mode"]
pub type SaW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Receive or Send This bit-field specifies if the next operation is a receive (high) or a transmit/send (low) from the addressed slave SA."]
    #[inline(always)]
    pub fn rs(&self) -> RsR {
        RsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
I2C master slave address Defines which slave is addressed for the transaction in master mode"]
    #[inline(always)]
    pub fn sa(&self) -> SaR {
        SaR::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Receive or Send This bit-field specifies if the next operation is a receive (high) or a transmit/send (low) from the addressed slave SA."]
    #[inline(always)]
    #[must_use]
    pub fn rs(&mut self) -> RsW<MsaSpec> {
        RsW::new(self, 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
I2C master slave address Defines which slave is addressed for the transaction in master mode"]
    #[inline(always)]
    #[must_use]
    pub fn sa(&mut self) -> SaW<MsaSpec> {
        SaW::new(self, 1)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<MsaSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Master Salve Address This register contains seven address bits of the slave to be accessed by the master (a6-a0), and an RS bit determining if the next operation is a receive or transmit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
