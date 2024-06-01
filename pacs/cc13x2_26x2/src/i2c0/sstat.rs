#[doc = "Register `SSTAT` reader"]
pub type R = crate::R<SstatSpec>;
#[doc = "Register `SSTAT` writer"]
pub type W = crate::W<SstatSpec>;
#[doc = "Field `RREQ` reader - 0:0\\]
Receive request 0: No outstanding receive data 1: The I2C controller has outstanding receive data from the I2C master and is using clock stretching to delay the master until data has been read from the SDR register."]
pub type RreqR = crate::BitReader;
#[doc = "Field `RREQ` writer - 0:0\\]
Receive request 0: No outstanding receive data 1: The I2C controller has outstanding receive data from the I2C master and is using clock stretching to delay the master until data has been read from the SDR register."]
pub type RreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TREQ` reader - 1:1\\]
Transmit request 0: No outstanding transmit request. 1: The I2C controller has been addressed as a slave transmitter and is using clock stretching to delay the master until data has been written to the SDR register."]
pub type TreqR = crate::BitReader;
#[doc = "Field `TREQ` writer - 1:1\\]
Transmit request 0: No outstanding transmit request. 1: The I2C controller has been addressed as a slave transmitter and is using clock stretching to delay the master until data has been written to the SDR register."]
pub type TreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBR` reader - 2:2\\]
First byte received 0: The first byte has not been received. 1: The first byte following the slave's own address has been received. This bit is only valid when the RREQ bit is set and is automatically cleared when data has been read from the SDR register. Note: This bit is not used for slave transmit operations."]
pub type FbrR = crate::BitReader;
#[doc = "Field `FBR` writer - 2:2\\]
First byte received 0: The first byte has not been received. 1: The first byte following the slave's own address has been received. This bit is only valid when the RREQ bit is set and is automatically cleared when data has been read from the SDR register. Note: This bit is not used for slave transmit operations."]
pub type FbrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Receive request 0: No outstanding receive data 1: The I2C controller has outstanding receive data from the I2C master and is using clock stretching to delay the master until data has been read from the SDR register."]
    #[inline(always)]
    pub fn rreq(&self) -> RreqR {
        RreqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Transmit request 0: No outstanding transmit request. 1: The I2C controller has been addressed as a slave transmitter and is using clock stretching to delay the master until data has been written to the SDR register."]
    #[inline(always)]
    pub fn treq(&self) -> TreqR {
        TreqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
First byte received 0: The first byte has not been received. 1: The first byte following the slave's own address has been received. This bit is only valid when the RREQ bit is set and is automatically cleared when data has been read from the SDR register. Note: This bit is not used for slave transmit operations."]
    #[inline(always)]
    pub fn fbr(&self) -> FbrR {
        FbrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Receive request 0: No outstanding receive data 1: The I2C controller has outstanding receive data from the I2C master and is using clock stretching to delay the master until data has been read from the SDR register."]
    #[inline(always)]
    #[must_use]
    pub fn rreq(&mut self) -> RreqW<SstatSpec> {
        RreqW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Transmit request 0: No outstanding transmit request. 1: The I2C controller has been addressed as a slave transmitter and is using clock stretching to delay the master until data has been written to the SDR register."]
    #[inline(always)]
    #[must_use]
    pub fn treq(&mut self) -> TreqW<SstatSpec> {
        TreqW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
First byte received 0: The first byte has not been received. 1: The first byte following the slave's own address has been received. This bit is only valid when the RREQ bit is set and is automatically cleared when data has been read from the SDR register. Note: This bit is not used for slave transmit operations."]
    #[inline(always)]
    #[must_use]
    pub fn fbr(&mut self) -> FbrW<SstatSpec> {
        FbrW::new(self, 2)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<SstatSpec> {
        Reserved3W::new(self, 3)
    }
}
#[doc = "Slave Status Note: This register shares address with SCTL, meaning that this register functions as a control register when written, and a status register when read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SstatSpec;
impl crate::RegisterSpec for SstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sstat::R`](R) reader structure"]
impl crate::Readable for SstatSpec {}
#[doc = "`write(|w| ..)` method takes [`sstat::W`](W) writer structure"]
impl crate::Writable for SstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSTAT to value 0"]
impl crate::Resettable for SstatSpec {
    const RESET_VALUE: u32 = 0;
}
