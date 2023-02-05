#[doc = "Register `MSTAT` reader"]
pub struct R(crate::R<MSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSTAT` writer"]
pub struct W(crate::W<MSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSTAT_SPEC>;
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
impl From<crate::W<MSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUSY` reader - 0:0\\]
I2C busy 0: The controller is idle. 1: The controller is busy. When this bit-field is set, the other status bits are not valid. Note: The I2C controller requires four SYSBUS clock cycles to assert the BUSY status after I2C master operation has been initiated through MCTRL register. Hence after programming MCTRL register, application is requested to wait for four SYSBUS clock cycles before issuing a controller status inquiry through MSTAT register. Any prior inquiry would result in wrong status being reported."]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` writer - 0:0\\]
I2C busy 0: The controller is idle. 1: The controller is busy. When this bit-field is set, the other status bits are not valid. Note: The I2C controller requires four SYSBUS clock cycles to assert the BUSY status after I2C master operation has been initiated through MCTRL register. Hence after programming MCTRL register, application is requested to wait for four SYSBUS clock cycles before issuing a controller status inquiry through MSTAT register. Any prior inquiry would result in wrong status being reported."]
pub type BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTAT_SPEC, bool, O>;
#[doc = "Field `ERR` reader - 1:1\\]
Error 0: No error was detected on the last operation. 1: An error occurred on the last operation."]
pub type ERR_R = crate::BitReader<bool>;
#[doc = "Field `ERR` writer - 1:1\\]
Error 0: No error was detected on the last operation. 1: An error occurred on the last operation."]
pub type ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTAT_SPEC, bool, O>;
#[doc = "Field `ADRACK_N` reader - 2:2\\]
Address Was Not Acknowledge 0: The transmitted address was acknowledged. 1: The transmitted address was not acknowledged."]
pub type ADRACK_N_R = crate::BitReader<bool>;
#[doc = "Field `ADRACK_N` writer - 2:2\\]
Address Was Not Acknowledge 0: The transmitted address was acknowledged. 1: The transmitted address was not acknowledged."]
pub type ADRACK_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTAT_SPEC, bool, O>;
#[doc = "Field `DATACK_N` reader - 3:3\\]
Data Was Not Acknowledge 0: The transmitted data was acknowledged. 1: The transmitted data was not acknowledged."]
pub type DATACK_N_R = crate::BitReader<bool>;
#[doc = "Field `DATACK_N` writer - 3:3\\]
Data Was Not Acknowledge 0: The transmitted data was acknowledged. 1: The transmitted data was not acknowledged."]
pub type DATACK_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTAT_SPEC, bool, O>;
#[doc = "Field `ARBLST` reader - 4:4\\]
Arbitration lost 0: The I2C controller won arbitration. 1: The I2C controller lost arbitration."]
pub type ARBLST_R = crate::BitReader<bool>;
#[doc = "Field `ARBLST` writer - 4:4\\]
Arbitration lost 0: The I2C controller won arbitration. 1: The I2C controller lost arbitration."]
pub type ARBLST_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTAT_SPEC, bool, O>;
#[doc = "Field `IDLE` reader - 5:5\\]
I2C idle 0: The I2C controller is not idle. 1: The I2C controller is idle."]
pub type IDLE_R = crate::BitReader<bool>;
#[doc = "Field `IDLE` writer - 5:5\\]
I2C idle 0: The I2C controller is not idle. 1: The I2C controller is idle."]
pub type IDLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTAT_SPEC, bool, O>;
#[doc = "Field `BUSBSY` reader - 6:6\\]
Bus busy 0: The I2C bus is idle. 1: The I2C bus is busy. The bit changes based on the MCTRL.START and MCTRL.STOP conditions."]
pub type BUSBSY_R = crate::BitReader<bool>;
#[doc = "Field `BUSBSY` writer - 6:6\\]
Bus busy 0: The I2C bus is idle. 1: The I2C bus is busy. The bit changes based on the MCTRL.START and MCTRL.STOP conditions."]
pub type BUSBSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTAT_SPEC, bool, O>;
#[doc = "Field `RESERVED7` reader - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED7_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED7` writer - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MSTAT_SPEC, u32, u32, 25, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
I2C busy 0: The controller is idle. 1: The controller is busy. When this bit-field is set, the other status bits are not valid. Note: The I2C controller requires four SYSBUS clock cycles to assert the BUSY status after I2C master operation has been initiated through MCTRL register. Hence after programming MCTRL register, application is requested to wait for four SYSBUS clock cycles before issuing a controller status inquiry through MSTAT register. Any prior inquiry would result in wrong status being reported."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Error 0: No error was detected on the last operation. 1: An error occurred on the last operation."]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Address Was Not Acknowledge 0: The transmitted address was acknowledged. 1: The transmitted address was not acknowledged."]
    #[inline(always)]
    pub fn adrack_n(&self) -> ADRACK_N_R {
        ADRACK_N_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Data Was Not Acknowledge 0: The transmitted data was acknowledged. 1: The transmitted data was not acknowledged."]
    #[inline(always)]
    pub fn datack_n(&self) -> DATACK_N_R {
        DATACK_N_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Arbitration lost 0: The I2C controller won arbitration. 1: The I2C controller lost arbitration."]
    #[inline(always)]
    pub fn arblst(&self) -> ARBLST_R {
        ARBLST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
I2C idle 0: The I2C controller is not idle. 1: The I2C controller is idle."]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Bus busy 0: The I2C bus is idle. 1: The I2C bus is busy. The bit changes based on the MCTRL.START and MCTRL.STOP conditions."]
    #[inline(always)]
    pub fn busbsy(&self) -> BUSBSY_R {
        BUSBSY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
I2C busy 0: The controller is idle. 1: The controller is busy. When this bit-field is set, the other status bits are not valid. Note: The I2C controller requires four SYSBUS clock cycles to assert the BUSY status after I2C master operation has been initiated through MCTRL register. Hence after programming MCTRL register, application is requested to wait for four SYSBUS clock cycles before issuing a controller status inquiry through MSTAT register. Any prior inquiry would result in wrong status being reported."]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BUSY_W<0> {
        BUSY_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Error 0: No error was detected on the last operation. 1: An error occurred on the last operation."]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ERR_W<1> {
        ERR_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Address Was Not Acknowledge 0: The transmitted address was acknowledged. 1: The transmitted address was not acknowledged."]
    #[inline(always)]
    #[must_use]
    pub fn adrack_n(&mut self) -> ADRACK_N_W<2> {
        ADRACK_N_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Data Was Not Acknowledge 0: The transmitted data was acknowledged. 1: The transmitted data was not acknowledged."]
    #[inline(always)]
    #[must_use]
    pub fn datack_n(&mut self) -> DATACK_N_W<3> {
        DATACK_N_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Arbitration lost 0: The I2C controller won arbitration. 1: The I2C controller lost arbitration."]
    #[inline(always)]
    #[must_use]
    pub fn arblst(&mut self) -> ARBLST_W<4> {
        ARBLST_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
I2C idle 0: The I2C controller is not idle. 1: The I2C controller is idle."]
    #[inline(always)]
    #[must_use]
    pub fn idle(&mut self) -> IDLE_W<5> {
        IDLE_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Bus busy 0: The I2C bus is idle. 1: The I2C bus is busy. The bit changes based on the MCTRL.START and MCTRL.STOP conditions."]
    #[inline(always)]
    #[must_use]
    pub fn busbsy(&mut self) -> BUSBSY_W<6> {
        BUSBSY_W::new(self)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> RESERVED7_W<7> {
        RESERVED7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mstat](index.html) module"]
pub struct MSTAT_SPEC;
impl crate::RegisterSpec for MSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mstat::R](R) reader structure"]
impl crate::Readable for MSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mstat::W](W) writer structure"]
impl crate::Writable for MSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSTAT to value 0x20"]
impl crate::Resettable for MSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
