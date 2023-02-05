#[doc = "Register `MSTAT_MCTL` reader"]
pub type R = crate::R<MstatMctlSpec>;
#[doc = "Register `MSTAT_MCTL` writer"]
pub type W = crate::W<MstatMctlSpec>;
#[doc = "Field `BUSY_RUN` reader - 0:0\\]
This field reflects the *I2C* busy status when read and sets *I2C* master enable when written. When Read 0 - The controller is idle. 1 - The controller is busy. When this bit-field is set, the other status bits are not valid. Note: The *I2C* controller requires four SYSBUS clock cycles to assert the BUSY status after *I2C* master operation has been initiated through a write into MSTAT_MCTL register. Hence after programming MCTRL register, application is requested to wait for four SYSBUS clock cycles before issuing a controller status inquiry through a read from MSTAT_MCTL register. Any prior inquiry would result in wrong status being reported. When written: 0 - The master is disabled. 1 - The master is enabled to transmit or receive data."]
pub type BusyRunR = crate::BitReader;
#[doc = "Field `BUSY_RUN` writer - 0:0\\]
This field reflects the *I2C* busy status when read and sets *I2C* master enable when written. When Read 0 - The controller is idle. 1 - The controller is busy. When this bit-field is set, the other status bits are not valid. Note: The *I2C* controller requires four SYSBUS clock cycles to assert the BUSY status after *I2C* master operation has been initiated through a write into MSTAT_MCTL register. Hence after programming MCTRL register, application is requested to wait for four SYSBUS clock cycles before issuing a controller status inquiry through a read from MSTAT_MCTL register. Any prior inquiry would result in wrong status being reported. When written: 0 - The master is disabled. 1 - The master is enabled to transmit or receive data."]
pub type BusyRunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_START` reader - 1:1\\]
This field reflect the error status when read and sets start or repeated start condition when written. When read: 0 - No error was detected on the last operation. 1 - An error occurred on the last operation. When written: 0 - The controller does not generate the Start condition. 1 - The controller generates the Start condition."]
pub type ErrStartR = crate::BitReader;
#[doc = "Field `ERR_START` writer - 1:1\\]
This field reflect the error status when read and sets start or repeated start condition when written. When read: 0 - No error was detected on the last operation. 1 - An error occurred on the last operation. When written: 0 - The controller does not generate the Start condition. 1 - The controller generates the Start condition."]
pub type ErrStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADRACKN_STOP` reader - 2:2\\]
This field reflects the address acknowledge status when read and sets stop condition when written. When read: 0 - The transmitted address was acknowledged. 1 - The transmitted address was not acknowledged. When written Note:This bit-field determines if the cycle stops at the end of the data cycle or continues on to a repeated START condition. 0 - The controller does not generate the Stop condition. 1 - The controller generates the Stop condition."]
pub type AdracknStopR = crate::BitReader;
#[doc = "Field `ADRACKN_STOP` writer - 2:2\\]
This field reflects the address acknowledge status when read and sets stop condition when written. When read: 0 - The transmitted address was acknowledged. 1 - The transmitted address was not acknowledged. When written Note:This bit-field determines if the cycle stops at the end of the data cycle or continues on to a repeated START condition. 0 - The controller does not generate the Stop condition. 1 - The controller generates the Stop condition."]
pub type AdracknStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATACKN_ACK` reader - 3:3\\]
This field contains Data acknowledge in status read and Data acknowledge enable in control write. Status Read: Data Acknowledge 0 - The transmitted data was acknowledged. 1 - The transmitted data was not acknowledged. Control write: Data acknowledge enable 0 - The received data byte is not acknowledged automatically by the master. 1 - The received data byte is acknowledged automatically by the master. Note:This bit-field must be cleared when the I2C bus controller requires no further data to be transmitted from the slave transmitter."]
pub type DatacknAckR = crate::BitReader;
#[doc = "Field `DATACKN_ACK` writer - 3:3\\]
This field contains Data acknowledge in status read and Data acknowledge enable in control write. Status Read: Data Acknowledge 0 - The transmitted data was acknowledged. 1 - The transmitted data was not acknowledged. Control write: Data acknowledge enable 0 - The received data byte is not acknowledged automatically by the master. 1 - The received data byte is acknowledged automatically by the master. Note:This bit-field must be cleared when the I2C bus controller requires no further data to be transmitted from the slave transmitter."]
pub type DatacknAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBLST` reader - 4:4\\]
Arbitration status 0 - The *I2C* controller won arbitration. 1 - The *I2C* controller lost arbitration."]
pub type ArblstR = crate::BitReader;
#[doc = "Field `IDLE` reader - 5:5\\]
*I2C* idle 0 - The *I2C* controller is not idle. 1 - The *I2C* controller is idle."]
pub type IdleR = crate::BitReader;
#[doc = "Field `BUSBSY` reader - 6:6\\]
Bus busy 0 - The *I2C* bus is idle. 1 - The *I2C* bus is busy. Note:The bit changes based on the MCTRL.START and MCTRL.STOP conditions."]
pub type BusbsyR = crate::BitReader;
#[doc = "Field `RESERVED7` reader - 31:7\\]
Reads to this field return zero.Writes to this field are ignored."]
pub type Reserved7R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This field reflects the *I2C* busy status when read and sets *I2C* master enable when written. When Read 0 - The controller is idle. 1 - The controller is busy. When this bit-field is set, the other status bits are not valid. Note: The *I2C* controller requires four SYSBUS clock cycles to assert the BUSY status after *I2C* master operation has been initiated through a write into MSTAT_MCTL register. Hence after programming MCTRL register, application is requested to wait for four SYSBUS clock cycles before issuing a controller status inquiry through a read from MSTAT_MCTL register. Any prior inquiry would result in wrong status being reported. When written: 0 - The master is disabled. 1 - The master is enabled to transmit or receive data."]
    #[inline(always)]
    pub fn busy_run(&self) -> BusyRunR {
        BusyRunR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This field reflect the error status when read and sets start or repeated start condition when written. When read: 0 - No error was detected on the last operation. 1 - An error occurred on the last operation. When written: 0 - The controller does not generate the Start condition. 1 - The controller generates the Start condition."]
    #[inline(always)]
    pub fn err_start(&self) -> ErrStartR {
        ErrStartR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
This field reflects the address acknowledge status when read and sets stop condition when written. When read: 0 - The transmitted address was acknowledged. 1 - The transmitted address was not acknowledged. When written Note:This bit-field determines if the cycle stops at the end of the data cycle or continues on to a repeated START condition. 0 - The controller does not generate the Stop condition. 1 - The controller generates the Stop condition."]
    #[inline(always)]
    pub fn adrackn_stop(&self) -> AdracknStopR {
        AdracknStopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
This field contains Data acknowledge in status read and Data acknowledge enable in control write. Status Read: Data Acknowledge 0 - The transmitted data was acknowledged. 1 - The transmitted data was not acknowledged. Control write: Data acknowledge enable 0 - The received data byte is not acknowledged automatically by the master. 1 - The received data byte is acknowledged automatically by the master. Note:This bit-field must be cleared when the I2C bus controller requires no further data to be transmitted from the slave transmitter."]
    #[inline(always)]
    pub fn datackn_ack(&self) -> DatacknAckR {
        DatacknAckR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Arbitration status 0 - The *I2C* controller won arbitration. 1 - The *I2C* controller lost arbitration."]
    #[inline(always)]
    pub fn arblst(&self) -> ArblstR {
        ArblstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
*I2C* idle 0 - The *I2C* controller is not idle. 1 - The *I2C* controller is idle."]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Bus busy 0 - The *I2C* bus is idle. 1 - The *I2C* bus is busy. Note:The bit changes based on the MCTRL.START and MCTRL.STOP conditions."]
    #[inline(always)]
    pub fn busbsy(&self) -> BusbsyR {
        BusbsyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Reads to this field return zero.Writes to this field are ignored."]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This field reflects the *I2C* busy status when read and sets *I2C* master enable when written. When Read 0 - The controller is idle. 1 - The controller is busy. When this bit-field is set, the other status bits are not valid. Note: The *I2C* controller requires four SYSBUS clock cycles to assert the BUSY status after *I2C* master operation has been initiated through a write into MSTAT_MCTL register. Hence after programming MCTRL register, application is requested to wait for four SYSBUS clock cycles before issuing a controller status inquiry through a read from MSTAT_MCTL register. Any prior inquiry would result in wrong status being reported. When written: 0 - The master is disabled. 1 - The master is enabled to transmit or receive data."]
    #[inline(always)]
    #[must_use]
    pub fn busy_run(&mut self) -> BusyRunW<MstatMctlSpec> {
        BusyRunW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This field reflect the error status when read and sets start or repeated start condition when written. When read: 0 - No error was detected on the last operation. 1 - An error occurred on the last operation. When written: 0 - The controller does not generate the Start condition. 1 - The controller generates the Start condition."]
    #[inline(always)]
    #[must_use]
    pub fn err_start(&mut self) -> ErrStartW<MstatMctlSpec> {
        ErrStartW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
This field reflects the address acknowledge status when read and sets stop condition when written. When read: 0 - The transmitted address was acknowledged. 1 - The transmitted address was not acknowledged. When written Note:This bit-field determines if the cycle stops at the end of the data cycle or continues on to a repeated START condition. 0 - The controller does not generate the Stop condition. 1 - The controller generates the Stop condition."]
    #[inline(always)]
    #[must_use]
    pub fn adrackn_stop(&mut self) -> AdracknStopW<MstatMctlSpec> {
        AdracknStopW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
This field contains Data acknowledge in status read and Data acknowledge enable in control write. Status Read: Data Acknowledge 0 - The transmitted data was acknowledged. 1 - The transmitted data was not acknowledged. Control write: Data acknowledge enable 0 - The received data byte is not acknowledged automatically by the master. 1 - The received data byte is acknowledged automatically by the master. Note:This bit-field must be cleared when the I2C bus controller requires no further data to be transmitted from the slave transmitter."]
    #[inline(always)]
    #[must_use]
    pub fn datackn_ack(&mut self) -> DatacknAckW<MstatMctlSpec> {
        DatacknAckW::new(self, 3)
    }
}
#[doc = "Master Control and Status This register functions as a control register when written, and a status register when read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mstat_mctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mstat_mctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MstatMctlSpec;
impl crate::RegisterSpec for MstatMctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mstat_mctl::R`](R) reader structure"]
impl crate::Readable for MstatMctlSpec {}
#[doc = "`write(|w| ..)` method takes [`mstat_mctl::W`](W) writer structure"]
impl crate::Writable for MstatMctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSTAT_MCTL to value 0x20"]
impl crate::Resettable for MstatMctlSpec {
    const RESET_VALUE: u32 = 0x20;
}
