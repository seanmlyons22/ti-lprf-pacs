#[doc = "Register `MSTAT` reader"]
pub type R = crate::R<MstatSpec>;
#[doc = "Register `MSTAT` writer"]
pub type W = crate::W<MstatSpec>;
#[doc = "Field `BUSY` reader - 0:0\\]
I2C busy 0: The controller is idle. 1: The controller is busy. When this bit-field is set, the other status bits are not valid. Note: The I2C controller requires four SYSBUS clock cycles to assert the BUSY status after I2C master operation has been initiated through MCTRL register. Hence after programming MCTRL register, application is requested to wait for four SYSBUS clock cycles before issuing a controller status inquiry through MSTAT register. Any prior inquiry would result in wrong status being reported."]
pub type BusyR = crate::BitReader;
#[doc = "Field `BUSY` writer - 0:0\\]
I2C busy 0: The controller is idle. 1: The controller is busy. When this bit-field is set, the other status bits are not valid. Note: The I2C controller requires four SYSBUS clock cycles to assert the BUSY status after I2C master operation has been initiated through MCTRL register. Hence after programming MCTRL register, application is requested to wait for four SYSBUS clock cycles before issuing a controller status inquiry through MSTAT register. Any prior inquiry would result in wrong status being reported."]
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR` reader - 1:1\\]
Error 0: No error was detected on the last operation. 1: An error occurred on the last operation."]
pub type ErrR = crate::BitReader;
#[doc = "Field `ERR` writer - 1:1\\]
Error 0: No error was detected on the last operation. 1: An error occurred on the last operation."]
pub type ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADRACK_N` reader - 2:2\\]
Address Was Not Acknowledge 0: The transmitted address was acknowledged. 1: The transmitted address was not acknowledged."]
pub type AdrackNR = crate::BitReader;
#[doc = "Field `ADRACK_N` writer - 2:2\\]
Address Was Not Acknowledge 0: The transmitted address was acknowledged. 1: The transmitted address was not acknowledged."]
pub type AdrackNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATACK_N` reader - 3:3\\]
Data Was Not Acknowledge 0: The transmitted data was acknowledged. 1: The transmitted data was not acknowledged."]
pub type DatackNR = crate::BitReader;
#[doc = "Field `DATACK_N` writer - 3:3\\]
Data Was Not Acknowledge 0: The transmitted data was acknowledged. 1: The transmitted data was not acknowledged."]
pub type DatackNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBLST` reader - 4:4\\]
Arbitration lost 0: The I2C controller won arbitration. 1: The I2C controller lost arbitration."]
pub type ArblstR = crate::BitReader;
#[doc = "Field `ARBLST` writer - 4:4\\]
Arbitration lost 0: The I2C controller won arbitration. 1: The I2C controller lost arbitration."]
pub type ArblstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLE` reader - 5:5\\]
I2C idle 0: The I2C controller is not idle. 1: The I2C controller is idle."]
pub type IdleR = crate::BitReader;
#[doc = "Field `IDLE` writer - 5:5\\]
I2C idle 0: The I2C controller is not idle. 1: The I2C controller is idle."]
pub type IdleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSBSY` reader - 6:6\\]
Bus busy 0: The I2C bus is idle. 1: The I2C bus is busy. The bit changes based on the MCTRL.START and MCTRL.STOP conditions."]
pub type BusbsyR = crate::BitReader;
#[doc = "Field `BUSBSY` writer - 6:6\\]
Bus busy 0: The I2C bus is idle. 1: The I2C bus is busy. The bit changes based on the MCTRL.START and MCTRL.STOP conditions."]
pub type BusbsyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED7` reader - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED7` writer - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
I2C busy 0: The controller is idle. 1: The controller is busy. When this bit-field is set, the other status bits are not valid. Note: The I2C controller requires four SYSBUS clock cycles to assert the BUSY status after I2C master operation has been initiated through MCTRL register. Hence after programming MCTRL register, application is requested to wait for four SYSBUS clock cycles before issuing a controller status inquiry through MSTAT register. Any prior inquiry would result in wrong status being reported."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Error 0: No error was detected on the last operation. 1: An error occurred on the last operation."]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Address Was Not Acknowledge 0: The transmitted address was acknowledged. 1: The transmitted address was not acknowledged."]
    #[inline(always)]
    pub fn adrack_n(&self) -> AdrackNR {
        AdrackNR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Data Was Not Acknowledge 0: The transmitted data was acknowledged. 1: The transmitted data was not acknowledged."]
    #[inline(always)]
    pub fn datack_n(&self) -> DatackNR {
        DatackNR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Arbitration lost 0: The I2C controller won arbitration. 1: The I2C controller lost arbitration."]
    #[inline(always)]
    pub fn arblst(&self) -> ArblstR {
        ArblstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
I2C idle 0: The I2C controller is not idle. 1: The I2C controller is idle."]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Bus busy 0: The I2C bus is idle. 1: The I2C bus is busy. The bit changes based on the MCTRL.START and MCTRL.STOP conditions."]
    #[inline(always)]
    pub fn busbsy(&self) -> BusbsyR {
        BusbsyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
I2C busy 0: The controller is idle. 1: The controller is busy. When this bit-field is set, the other status bits are not valid. Note: The I2C controller requires four SYSBUS clock cycles to assert the BUSY status after I2C master operation has been initiated through MCTRL register. Hence after programming MCTRL register, application is requested to wait for four SYSBUS clock cycles before issuing a controller status inquiry through MSTAT register. Any prior inquiry would result in wrong status being reported."]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BusyW<MstatSpec> {
        BusyW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Error 0: No error was detected on the last operation. 1: An error occurred on the last operation."]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ErrW<MstatSpec> {
        ErrW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Address Was Not Acknowledge 0: The transmitted address was acknowledged. 1: The transmitted address was not acknowledged."]
    #[inline(always)]
    #[must_use]
    pub fn adrack_n(&mut self) -> AdrackNW<MstatSpec> {
        AdrackNW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Data Was Not Acknowledge 0: The transmitted data was acknowledged. 1: The transmitted data was not acknowledged."]
    #[inline(always)]
    #[must_use]
    pub fn datack_n(&mut self) -> DatackNW<MstatSpec> {
        DatackNW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Arbitration lost 0: The I2C controller won arbitration. 1: The I2C controller lost arbitration."]
    #[inline(always)]
    #[must_use]
    pub fn arblst(&mut self) -> ArblstW<MstatSpec> {
        ArblstW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
I2C idle 0: The I2C controller is not idle. 1: The I2C controller is idle."]
    #[inline(always)]
    #[must_use]
    pub fn idle(&mut self) -> IdleW<MstatSpec> {
        IdleW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Bus busy 0: The I2C bus is idle. 1: The I2C bus is busy. The bit changes based on the MCTRL.START and MCTRL.STOP conditions."]
    #[inline(always)]
    #[must_use]
    pub fn busbsy(&mut self) -> BusbsyW<MstatSpec> {
        BusbsyW::new(self, 6)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> Reserved7W<MstatSpec> {
        Reserved7W::new(self, 7)
    }
}
#[doc = "Master Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MstatSpec;
impl crate::RegisterSpec for MstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mstat::R`](R) reader structure"]
impl crate::Readable for MstatSpec {}
#[doc = "`write(|w| ..)` method takes [`mstat::W`](W) writer structure"]
impl crate::Writable for MstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSTAT to value 0x20"]
impl crate::Resettable for MstatSpec {
    const RESET_VALUE: u32 = 0x20;
}
