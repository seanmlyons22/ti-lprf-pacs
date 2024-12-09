#[doc = "Register `MCTRL` reader"]
pub type R = crate::R<MctrlSpec>;
#[doc = "Register `MCTRL` writer"]
pub type W = crate::W<MctrlSpec>;
#[doc = "0:0\\]
I2C master enable 0: The master is disabled. 1: The master is enabled to transmit or receive data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Run {
    #[doc = "1: Enable Master"]
    En = 1,
    #[doc = "0: Disable Master"]
    Dis = 0,
}
impl From<Run> for bool {
    #[inline(always)]
    fn from(variant: Run) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RUN` writer - 0:0\\]
I2C master enable 0: The master is disabled. 1: The master is enabled to transmit or receive data."]
pub type RunW<'a, REG> = crate::BitWriter<'a, REG, Run>;
impl<'a, REG> RunW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Master"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Run::En)
    }
    #[doc = "Disable Master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Run::Dis)
    }
}
#[doc = "1:1\\]
This bit-field generates the Start or Repeated Start condition. 0: The controller does not generate the Start condition. 1: The controller generates the Start condition.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Start {
    #[doc = "1: Enable START"]
    En = 1,
    #[doc = "0: Disable START"]
    Dis = 0,
}
impl From<Start> for bool {
    #[inline(always)]
    fn from(variant: Start) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` writer - 1:1\\]
This bit-field generates the Start or Repeated Start condition. 0: The controller does not generate the Start condition. 1: The controller generates the Start condition."]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG, Start>;
impl<'a, REG> StartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable START"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Start::En)
    }
    #[doc = "Disable START"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Dis)
    }
}
#[doc = "2:2\\]
This bit-field determines if the cycle stops at the end of the data cycle or continues on to a repeated START condition. 0: The controller does not generate the Stop condition. 1: The controller generates the Stop condition.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop {
    #[doc = "1: Enable STOP"]
    En = 1,
    #[doc = "0: Disable STOP"]
    Dis = 0,
}
impl From<Stop> for bool {
    #[inline(always)]
    fn from(variant: Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP` writer - 2:2\\]
This bit-field determines if the cycle stops at the end of the data cycle or continues on to a repeated START condition. 0: The controller does not generate the Stop condition. 1: The controller generates the Stop condition."]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG, Stop>;
impl<'a, REG> StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable STOP"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::En)
    }
    #[doc = "Disable STOP"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::Dis)
    }
}
#[doc = "3:3\\]
Data acknowledge enable 0: The received data byte is not acknowledged automatically by the master. 1: The received data byte is acknowledged automatically by the master. This bit-field must be cleared when the I2C bus controller requires no further data to be transmitted from the slave transmitter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ack {
    #[doc = "1: Enable acknowledge"]
    En = 1,
    #[doc = "0: Disable acknowledge"]
    Dis = 0,
}
impl From<Ack> for bool {
    #[inline(always)]
    fn from(variant: Ack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACK` writer - 3:3\\]
Data acknowledge enable 0: The received data byte is not acknowledged automatically by the master. 1: The received data byte is acknowledged automatically by the master. This bit-field must be cleared when the I2C bus controller requires no further data to be transmitted from the slave transmitter."]
pub type AckW<'a, REG> = crate::BitWriter<'a, REG, Ack>;
impl<'a, REG> AckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable acknowledge"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Ack::En)
    }
    #[doc = "Disable acknowledge"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ack::Dis)
    }
}
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl W {
    #[doc = "Bit 0 - 0:0\\]
I2C master enable 0: The master is disabled. 1: The master is enabled to transmit or receive data."]
    #[inline(always)]
    #[must_use]
    pub fn run(&mut self) -> RunW<MctrlSpec> {
        RunW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit-field generates the Start or Repeated Start condition. 0: The controller does not generate the Start condition. 1: The controller generates the Start condition."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<MctrlSpec> {
        StartW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
This bit-field determines if the cycle stops at the end of the data cycle or continues on to a repeated START condition. 0: The controller does not generate the Stop condition. 1: The controller generates the Stop condition."]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> StopW<MctrlSpec> {
        StopW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Data acknowledge enable 0: The received data byte is not acknowledged automatically by the master. 1: The received data byte is acknowledged automatically by the master. This bit-field must be cleared when the I2C bus controller requires no further data to be transmitted from the slave transmitter."]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> AckW<MctrlSpec> {
        AckW::new(self, 3)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<MctrlSpec> {
        Reserved4W::new(self, 4)
    }
}
#[doc = "Master Control This register accesses status bits when read and control bits when written. When read, the status register indicates the state of the I2C bus controller as stated in MSTAT. When written, the control register configures the I2C controller operation. To generate a single transmit cycle, the I2C Master Slave Address (MSA) register is written with the desired address, the MSA.RS bit is cleared, and this register is written with * ACK=X (0 or 1), * STOP=1, * START=1, * RUN=1 to perform the operation and stop. When the operation is completed (or aborted due an error), an interrupt becomes active and the data may be read from the MDR register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MctrlSpec;
impl crate::RegisterSpec for MctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mctrl::R`](R) reader structure"]
impl crate::Readable for MctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mctrl::W`](W) writer structure"]
impl crate::Writable for MctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCTRL to value 0"]
impl crate::Resettable for MctrlSpec {
    const RESET_VALUE: u32 = 0;
}
