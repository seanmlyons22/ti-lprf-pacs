#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "0:0\\]
Shows the enable status of the controller as configured by CFG.MASTERENABLE:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Masterenable {
    #[doc = "1: Controller is enabled"]
    En = 1,
    #[doc = "0: Controller is disabled"]
    Dis = 0,
}
impl From<Masterenable> for bool {
    #[inline(always)]
    fn from(variant: Masterenable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASTERENABLE` reader - 0:0\\]
Shows the enable status of the controller as configured by CFG.MASTERENABLE:"]
pub type MasterenableR = crate::BitReader<Masterenable>;
impl MasterenableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Masterenable {
        match self.bits {
            true => Masterenable::En,
            false => Masterenable::Dis,
        }
    }
    #[doc = "Controller is enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Masterenable::En
    }
    #[doc = "Controller is disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Masterenable::Dis
    }
}
#[doc = "Field `RESERVED1` reader - 3:1\\]
Reads to this field return zero, writes to this field are ignored."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `STATE` reader - 7:4\\]
Current state of the control state machine. State can be one of the following: 0x0: Idle 0x1: Reading channel controller data 0x2: Reading source data end pointer 0x3: Reading destination data end pointer 0x4: Reading source data 0x5: Writing destination data 0x6: Waiting for uDMA request to clear 0x7: Writing channel controller data 0x8: Stalled 0x9: Done 0xA: Peripheral scatter-gather transition 0xB: Undefined ... 0xF: Undefined."]
pub type StateR = crate::FieldReader;
#[doc = "Field `RESERVED8` reader - 15:8\\]
Reads to this field return zero, writes to this field are ignored."]
pub type Reserved8R = crate::FieldReader;
#[doc = "Field `TOTALCHANNELS` reader - 20:16\\]
Register value returns number of available uDMA channels minus one. For example a read out value of: 0x00: Show that the controller is configured to use 1 uDMA channel 0x01: Shows that the controller is configured to use 2 uDMA channels ... 0x1F: Shows that the controller is configured to use 32 uDMA channels (32-1=31=0x1F)"]
pub type TotalchannelsR = crate::FieldReader;
#[doc = "Field `RESERVED21` reader - 27:21\\]
Reads to this field return zero, writes to this field are ignored."]
pub type Reserved21R = crate::FieldReader;
#[doc = "Field `TEST` reader - 31:28\\]
0x0: Controller does not include the integration test logic 0x1: Controller includes the integration test logic 0x2: Undefined ... 0xF: Undefined"]
pub type TestR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Shows the enable status of the controller as configured by CFG.MASTERENABLE:"]
    #[inline(always)]
    pub fn masterenable(&self) -> MasterenableR {
        MasterenableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Reads to this field return zero, writes to this field are ignored."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Current state of the control state machine. State can be one of the following: 0x0: Idle 0x1: Reading channel controller data 0x2: Reading source data end pointer 0x3: Reading destination data end pointer 0x4: Reading source data 0x5: Writing destination data 0x6: Waiting for uDMA request to clear 0x7: Writing channel controller data 0x8: Stalled 0x9: Done 0xA: Peripheral scatter-gather transition 0xB: Undefined ... 0xF: Undefined."]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Reads to this field return zero, writes to this field are ignored."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Register value returns number of available uDMA channels minus one. For example a read out value of: 0x00: Show that the controller is configured to use 1 uDMA channel 0x01: Shows that the controller is configured to use 2 uDMA channels ... 0x1F: Shows that the controller is configured to use 32 uDMA channels (32-1=31=0x1F)"]
    #[inline(always)]
    pub fn totalchannels(&self) -> TotalchannelsR {
        TotalchannelsR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:27 - 27:21\\]
Reads to this field return zero, writes to this field are ignored."]
    #[inline(always)]
    pub fn reserved21(&self) -> Reserved21R {
        Reserved21R::new(((self.bits >> 21) & 0x7f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
0x0: Controller does not include the integration test logic 0x1: Controller includes the integration test logic 0x2: Undefined ... 0xF: Undefined"]
    #[inline(always)]
    pub fn test(&self) -> TestR {
        TestR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Status Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0x0007_0000"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0x0007_0000;
}
