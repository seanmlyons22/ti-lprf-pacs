#[doc = "Register `SSTAT_SCTL` reader"]
pub type R = crate::R<SstatSctlSpec>;
#[doc = "Register `SSTAT_SCTL` writer"]
pub type W = crate::W<SstatSctlSpec>;
#[doc = "Field `RREQ_DA` reader - 0:0\\]
This field reflects the Receive Request status when read and sets the Device Active control when written. When read: 0 - No outstanding receive data 1 - The *I2C* controller has outstanding receive data from the *I2C* master and is using clock stretching to delay the master until data has been read from the SDR register When written: 0 - Disables the *I2C* slave operation 1 - Enables the *I2C* slave operation"]
pub type RreqDaR = crate::BitReader;
#[doc = "Field `RREQ_DA` writer - 0:0\\]
This field reflects the Receive Request status when read and sets the Device Active control when written. When read: 0 - No outstanding receive data 1 - The *I2C* controller has outstanding receive data from the *I2C* master and is using clock stretching to delay the master until data has been read from the SDR register When written: 0 - Disables the *I2C* slave operation 1 - Enables the *I2C* slave operation"]
pub type RreqDaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TREQ` reader - 1:1\\]
Transmit request 0 - No outstanding transmit request 1 - The *I2C* controller has been addressed as a slave transmitter and is using clock stretching to delay the master until data has been written to the SDR register."]
pub type TreqR = crate::BitReader;
#[doc = "Field `FBR` reader - 2:2\\]
First byte received. 0 - The first byte following the slave's own address has not been received. 1 - The first byte following the slave's own address has been received. This bit is only valide when the RREQ bit is set and is automatically cleared when data has been read from the SDR register. Note: This bit is not used for slave transmit operations."]
pub type FbrR = crate::BitReader;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Reads to this field return zero.Writes to this field are ignored."]
pub type Reserved3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This field reflects the Receive Request status when read and sets the Device Active control when written. When read: 0 - No outstanding receive data 1 - The *I2C* controller has outstanding receive data from the *I2C* master and is using clock stretching to delay the master until data has been read from the SDR register When written: 0 - Disables the *I2C* slave operation 1 - Enables the *I2C* slave operation"]
    #[inline(always)]
    pub fn rreq_da(&self) -> RreqDaR {
        RreqDaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Transmit request 0 - No outstanding transmit request 1 - The *I2C* controller has been addressed as a slave transmitter and is using clock stretching to delay the master until data has been written to the SDR register."]
    #[inline(always)]
    pub fn treq(&self) -> TreqR {
        TreqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
First byte received. 0 - The first byte following the slave's own address has not been received. 1 - The first byte following the slave's own address has been received. This bit is only valide when the RREQ bit is set and is automatically cleared when data has been read from the SDR register. Note: This bit is not used for slave transmit operations."]
    #[inline(always)]
    pub fn fbr(&self) -> FbrR {
        FbrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Reads to this field return zero.Writes to this field are ignored."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This field reflects the Receive Request status when read and sets the Device Active control when written. When read: 0 - No outstanding receive data 1 - The *I2C* controller has outstanding receive data from the *I2C* master and is using clock stretching to delay the master until data has been read from the SDR register When written: 0 - Disables the *I2C* slave operation 1 - Enables the *I2C* slave operation"]
    #[inline(always)]
    #[must_use]
    pub fn rreq_da(&mut self) -> RreqDaW<SstatSctlSpec> {
        RreqDaW::new(self, 0)
    }
}
#[doc = "Slave Control and Slave Status This register functions as a control register when written, and a status register when read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sstat_sctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sstat_sctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SstatSctlSpec;
impl crate::RegisterSpec for SstatSctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sstat_sctl::R`](R) reader structure"]
impl crate::Readable for SstatSctlSpec {}
#[doc = "`write(|w| ..)` method takes [`sstat_sctl::W`](W) writer structure"]
impl crate::Writable for SstatSctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSTAT_SCTL to value 0"]
impl crate::Resettable for SstatSctlSpec {
    const RESET_VALUE: u32 = 0;
}
