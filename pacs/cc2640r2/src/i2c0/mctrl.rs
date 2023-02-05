#[doc = "Register `MCTRL` reader"]
pub struct R(crate::R<MCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCTRL` writer"]
pub struct W(crate::W<MCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCTRL_SPEC>;
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
impl From<crate::W<MCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RUN` reader - 0:0\\]
I2C master enable 0: The master is disabled. 1: The master is enabled to transmit or receive data."]
pub type RUN_R = crate::BitReader<RUN_A>;
#[doc = "0:0\\]
I2C master enable 0: The master is disabled. 1: The master is enabled to transmit or receive data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RUN_A {
    #[doc = "1: Enable Master"]
    EN = 1,
    #[doc = "0: Disable Master"]
    DIS = 0,
}
impl From<RUN_A> for bool {
    #[inline(always)]
    fn from(variant: RUN_A) -> Self {
        variant as u8 != 0
    }
}
impl RUN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RUN_A {
        match self.bits {
            true => RUN_A::EN,
            false => RUN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RUN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RUN_A::DIS
    }
}
#[doc = "Field `RUN` writer - 0:0\\]
I2C master enable 0: The master is disabled. 1: The master is enabled to transmit or receive data."]
pub type RUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCTRL_SPEC, RUN_A, O>;
impl<'a, const O: u8> RUN_W<'a, O> {
    #[doc = "Enable Master"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RUN_A::EN)
    }
    #[doc = "Disable Master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RUN_A::DIS)
    }
}
#[doc = "Field `START` reader - 1:1\\]
This bit-field generates the Start or Repeated Start condition. 0: The controller does not generate the Start condition. 1: The controller generates the Start condition."]
pub type START_R = crate::BitReader<START_A>;
#[doc = "1:1\\]
This bit-field generates the Start or Repeated Start condition. 0: The controller does not generate the Start condition. 1: The controller generates the Start condition.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum START_A {
    #[doc = "1: Enable START"]
    EN = 1,
    #[doc = "0: Disable START"]
    DIS = 0,
}
impl From<START_A> for bool {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as u8 != 0
    }
}
impl START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> START_A {
        match self.bits {
            true => START_A::EN,
            false => START_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == START_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == START_A::DIS
    }
}
#[doc = "Field `START` writer - 1:1\\]
This bit-field generates the Start or Repeated Start condition. 0: The controller does not generate the Start condition. 1: The controller generates the Start condition."]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCTRL_SPEC, START_A, O>;
impl<'a, const O: u8> START_W<'a, O> {
    #[doc = "Enable START"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(START_A::EN)
    }
    #[doc = "Disable START"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(START_A::DIS)
    }
}
#[doc = "Field `STOP` reader - 2:2\\]
This bit-field determines if the cycle stops at the end of the data cycle or continues on to a repeated START condition. 0: The controller does not generate the Stop condition. 1: The controller generates the Stop condition."]
pub type STOP_R = crate::BitReader<STOP_A>;
#[doc = "2:2\\]
This bit-field determines if the cycle stops at the end of the data cycle or continues on to a repeated START condition. 0: The controller does not generate the Stop condition. 1: The controller generates the Stop condition.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOP_A {
    #[doc = "1: Enable STOP"]
    EN = 1,
    #[doc = "0: Disable STOP"]
    DIS = 0,
}
impl From<STOP_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOP_A {
        match self.bits {
            true => STOP_A::EN,
            false => STOP_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == STOP_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == STOP_A::DIS
    }
}
#[doc = "Field `STOP` writer - 2:2\\]
This bit-field determines if the cycle stops at the end of the data cycle or continues on to a repeated START condition. 0: The controller does not generate the Stop condition. 1: The controller generates the Stop condition."]
pub type STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCTRL_SPEC, STOP_A, O>;
impl<'a, const O: u8> STOP_W<'a, O> {
    #[doc = "Enable STOP"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(STOP_A::EN)
    }
    #[doc = "Disable STOP"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(STOP_A::DIS)
    }
}
#[doc = "Field `ACK` reader - 3:3\\]
Data acknowledge enable 0: The received data byte is not acknowledged automatically by the master. 1: The received data byte is acknowledged automatically by the master. This bit-field must be cleared when the I2C bus controller requires no further data to be transmitted from the slave transmitter."]
pub type ACK_R = crate::BitReader<ACK_A>;
#[doc = "3:3\\]
Data acknowledge enable 0: The received data byte is not acknowledged automatically by the master. 1: The received data byte is acknowledged automatically by the master. This bit-field must be cleared when the I2C bus controller requires no further data to be transmitted from the slave transmitter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACK_A {
    #[doc = "1: Enable acknowledge"]
    EN = 1,
    #[doc = "0: Disable acknowledge"]
    DIS = 0,
}
impl From<ACK_A> for bool {
    #[inline(always)]
    fn from(variant: ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACK_A {
        match self.bits {
            true => ACK_A::EN,
            false => ACK_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ACK_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ACK_A::DIS
    }
}
#[doc = "Field `ACK` writer - 3:3\\]
Data acknowledge enable 0: The received data byte is not acknowledged automatically by the master. 1: The received data byte is acknowledged automatically by the master. This bit-field must be cleared when the I2C bus controller requires no further data to be transmitted from the slave transmitter."]
pub type ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCTRL_SPEC, ACK_A, O>;
impl<'a, const O: u8> ACK_W<'a, O> {
    #[doc = "Enable acknowledge"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ACK_A::EN)
    }
    #[doc = "Disable acknowledge"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ACK_A::DIS)
    }
}
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCTRL_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
I2C master enable 0: The master is disabled. 1: The master is enabled to transmit or receive data."]
    #[inline(always)]
    pub fn run(&self) -> RUN_R {
        RUN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit-field generates the Start or Repeated Start condition. 0: The controller does not generate the Start condition. 1: The controller generates the Start condition."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
This bit-field determines if the cycle stops at the end of the data cycle or continues on to a repeated START condition. 0: The controller does not generate the Stop condition. 1: The controller generates the Stop condition."]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Data acknowledge enable 0: The received data byte is not acknowledged automatically by the master. 1: The received data byte is acknowledged automatically by the master. This bit-field must be cleared when the I2C bus controller requires no further data to be transmitted from the slave transmitter."]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
I2C master enable 0: The master is disabled. 1: The master is enabled to transmit or receive data."]
    #[inline(always)]
    #[must_use]
    pub fn run(&mut self) -> RUN_W<0> {
        RUN_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit-field generates the Start or Repeated Start condition. 0: The controller does not generate the Start condition. 1: The controller generates the Start condition."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<1> {
        START_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
This bit-field determines if the cycle stops at the end of the data cycle or continues on to a repeated START condition. 0: The controller does not generate the Stop condition. 1: The controller generates the Stop condition."]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<2> {
        STOP_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Data acknowledge enable 0: The received data byte is not acknowledged automatically by the master. 1: The received data byte is acknowledged automatically by the master. This bit-field must be cleared when the I2C bus controller requires no further data to be transmitted from the slave transmitter."]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> ACK_W<3> {
        ACK_W::new(self)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> RESERVED4_W<4> {
        RESERVED4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Control This register accesses status bits when read and control bits when written. When read, the status register indicates the state of the I2C bus controller as stated in MSTAT. When written, the control register configures the I2C controller operation. To generate a single transmit cycle, the I2C Master Slave Address (MSA) register is written with the desired address, the MSA.RS bit is cleared, and this register is written with * ACK=X (0 or 1), * STOP=1, * START=1, * RUN=1 to perform the operation and stop. When the operation is completed (or aborted due an error), an interrupt becomes active and the data may be read from the MDR register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctrl](index.html) module"]
pub struct MCTRL_SPEC;
impl crate::RegisterSpec for MCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mctrl::R](R) reader structure"]
impl crate::Readable for MCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mctrl::W](W) writer structure"]
impl crate::Writable for MCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCTRL to value 0"]
impl crate::Resettable for MCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
