#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "Field `POWER_MODE` reader - 1:0\\]
Power state of each of the 2 flash arbiter FSM instances in the flash sub-system. For Thor, these bits should mostly be in the same state since both banks are in the same power mode. 0 : Active 1 : Ready for Low power (The 2T readiness has gone low or the flash_off_req has been set=1, and flash_off_ack is ready to be asserted). Bit 0 is for the power state for Bank0 which is at logical address 0x0 Bit 1 for Bank1"]
pub type PowerModeR = crate::FieldReader;
#[doc = "Field `POWER_MODE` writer - 1:0\\]
Power state of each of the 2 flash arbiter FSM instances in the flash sub-system. For Thor, these bits should mostly be in the same state since both banks are in the same power mode. 0 : Active 1 : Ready for Low power (The 2T readiness has gone low or the flash_off_req has been set=1, and flash_off_ack is ready to be asserted). Bit 0 is for the power state for Bank0 which is at logical address 0x0 Bit 1 for Bank1"]
pub type PowerModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `READY2T` reader - 2:2\\]
2T access readiness status indicator from NW 1: FLASH banks are ready for 2T accesses 0: FLASH banks are not ready for 2T accesses"]
pub type Ready2tR = crate::BitReader;
#[doc = "Field `READY2T` writer - 2:2\\]
2T access readiness status indicator from NW 1: FLASH banks are ready for 2T accesses 0: FLASH banks are not ready for 2T accesses"]
pub type Ready2tW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READY1T` reader - 3:3\\]
1T access readiness status indicator from NW. Comes later than 2T readiness. 1: FLASH banks are ready for 1T accesses 0: FLASH banks are not ready for 1T accesses"]
pub type Ready1tR = crate::BitReader;
#[doc = "Field `READY1T` writer - 3:3\\]
1T access readiness status indicator from NW. Comes later than 2T readiness. 1: FLASH banks are ready for 1T accesses 0: FLASH banks are not ready for 1T accesses"]
pub type Ready1tW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY` reader - 5:4\\]
NW FW_SMSTAT.CMD_IN_PROGRESS bit. This flag is valid immediately after the operation setting it 0 : Not busy 1 : Busy Bit 4 is for the busy state for Bank0 which is at logical address 0x0 Bit 5 for Bank1."]
pub type BusyR = crate::FieldReader;
#[doc = "Field `BUSY` writer - 5:4\\]
NW FW_SMSTAT.CMD_IN_PROGRESS bit. This flag is valid immediately after the operation setting it 0 : Not busy 1 : Busy Bit 4 is for the busy state for Bank0 which is at logical address 0x0 Bit 5 for Bank1."]
pub type BusyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESERVED7` reader - 7:6\\]
Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation"]
pub type Reserved7R = crate::FieldReader;
#[doc = "Field `RESERVED7` writer - 7:6\\]
Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation"]
pub type Reserved7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EFUSE_ERRCODE` reader - 12:8\\]
Same as EFUSEERROR.CODE"]
pub type EfuseErrcodeR = crate::FieldReader;
#[doc = "Field `EFUSE_ERRCODE` writer - 12:8\\]
Same as EFUSEERROR.CODE"]
pub type EfuseErrcodeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SPRS_BYTE_NOT_OK` reader - 13:13\\]
Efuse scanning resulted in scan chain Sparse byte error. 0 : No Sparse error 1 : Sparse Error"]
pub type SprsByteNotOkR = crate::BitReader;
#[doc = "Field `SPRS_BYTE_NOT_OK` writer - 13:13\\]
Efuse scanning resulted in scan chain Sparse byte error. 0 : No Sparse error 1 : Sparse Error"]
pub type SprsByteNotOkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFUSE_TIMEOUT` reader - 14:14\\]
Efuse scanning resulted in timeout error. 0 : No Timeout error 1 : Timeout Error"]
pub type EfuseTimeoutR = crate::BitReader;
#[doc = "Field `EFUSE_TIMEOUT` writer - 14:14\\]
Efuse scanning resulted in timeout error. 0 : No Timeout error 1 : Timeout Error"]
pub type EfuseTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFUSE_BLANK` reader - 15:15\\]
Efuse scanning detected if fuse ROM is blank: 0 : Not blank 1 : Blank"]
pub type EfuseBlankR = crate::BitReader;
#[doc = "Field `EFUSE_BLANK` writer - 15:15\\]
Efuse scanning detected if fuse ROM is blank: 0 : Not blank 1 : Blank"]
pub type EfuseBlankW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLSTAT` reader - 16:16\\]
An ocp1 or ocp3 read stall has occurred. 0 : No stall or stall acknowledged by writing a 1 1 : Stall condition occurred/occurring This is a read/write-clear status bit. It will reset to 0. It will be set when either an ocp1 or ocp3 read occurs to a bank that is presently undergoing a program or write operation. An ocp2 write of 1 to this bit will clear the bit. The ocp2 write will take highest priority in the event an ocp1/ocp3 read is occurring concurrently to the ocp2 write. Clearing the bit should be done only after the ongoing program/erase operation is complete indicating that both banks are free. If clearing occurs while the stall condition persists, the field may get set back to one."]
pub type StallstatR = crate::BitReader;
#[doc = "Field `STALLSTAT` writer - 16:16\\]
An ocp1 or ocp3 read stall has occurred. 0 : No stall or stall acknowledged by writing a 1 1 : Stall condition occurred/occurring This is a read/write-clear status bit. It will reset to 0. It will be set when either an ocp1 or ocp3 read occurs to a bank that is presently undergoing a program or write operation. An ocp2 write of 1 to this bit will clear the bit. The ocp2 write will take highest priority in the event an ocp1/ocp3 read is occurring concurrently to the ocp2 write. Clearing the bit should be done only after the ongoing program/erase operation is complete indicating that both banks are free. If clearing occurs while the stall condition persists, the field may get set back to one."]
pub type StallstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED15` reader - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved15R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED15` writer - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved15W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Power state of each of the 2 flash arbiter FSM instances in the flash sub-system. For Thor, these bits should mostly be in the same state since both banks are in the same power mode. 0 : Active 1 : Ready for Low power (The 2T readiness has gone low or the flash_off_req has been set=1, and flash_off_ack is ready to be asserted). Bit 0 is for the power state for Bank0 which is at logical address 0x0 Bit 1 for Bank1"]
    #[inline(always)]
    pub fn power_mode(&self) -> PowerModeR {
        PowerModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
2T access readiness status indicator from NW 1: FLASH banks are ready for 2T accesses 0: FLASH banks are not ready for 2T accesses"]
    #[inline(always)]
    pub fn ready2t(&self) -> Ready2tR {
        Ready2tR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
1T access readiness status indicator from NW. Comes later than 2T readiness. 1: FLASH banks are ready for 1T accesses 0: FLASH banks are not ready for 1T accesses"]
    #[inline(always)]
    pub fn ready1t(&self) -> Ready1tR {
        Ready1tR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
NW FW_SMSTAT.CMD_IN_PROGRESS bit. This flag is valid immediately after the operation setting it 0 : Not busy 1 : Busy Bit 4 is for the busy state for Bank0 which is at logical address 0x0 Bit 5 for Bank1."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Same as EFUSEERROR.CODE"]
    #[inline(always)]
    pub fn efuse_errcode(&self) -> EfuseErrcodeR {
        EfuseErrcodeR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - 13:13\\]
Efuse scanning resulted in scan chain Sparse byte error. 0 : No Sparse error 1 : Sparse Error"]
    #[inline(always)]
    pub fn sprs_byte_not_ok(&self) -> SprsByteNotOkR {
        SprsByteNotOkR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Efuse scanning resulted in timeout error. 0 : No Timeout error 1 : Timeout Error"]
    #[inline(always)]
    pub fn efuse_timeout(&self) -> EfuseTimeoutR {
        EfuseTimeoutR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Efuse scanning detected if fuse ROM is blank: 0 : Not blank 1 : Blank"]
    #[inline(always)]
    pub fn efuse_blank(&self) -> EfuseBlankR {
        EfuseBlankR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
An ocp1 or ocp3 read stall has occurred. 0 : No stall or stall acknowledged by writing a 1 1 : Stall condition occurred/occurring This is a read/write-clear status bit. It will reset to 0. It will be set when either an ocp1 or ocp3 read occurs to a bank that is presently undergoing a program or write operation. An ocp2 write of 1 to this bit will clear the bit. The ocp2 write will take highest priority in the event an ocp1/ocp3 read is occurring concurrently to the ocp2 write. Clearing the bit should be done only after the ongoing program/erase operation is complete indicating that both banks are free. If clearing occurs while the stall condition persists, the field may get set back to one."]
    #[inline(always)]
    pub fn stallstat(&self) -> StallstatR {
        StallstatR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved15(&self) -> Reserved15R {
        Reserved15R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Power state of each of the 2 flash arbiter FSM instances in the flash sub-system. For Thor, these bits should mostly be in the same state since both banks are in the same power mode. 0 : Active 1 : Ready for Low power (The 2T readiness has gone low or the flash_off_req has been set=1, and flash_off_ack is ready to be asserted). Bit 0 is for the power state for Bank0 which is at logical address 0x0 Bit 1 for Bank1"]
    #[inline(always)]
    #[must_use]
    pub fn power_mode(&mut self) -> PowerModeW<StatSpec> {
        PowerModeW::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
2T access readiness status indicator from NW 1: FLASH banks are ready for 2T accesses 0: FLASH banks are not ready for 2T accesses"]
    #[inline(always)]
    #[must_use]
    pub fn ready2t(&mut self) -> Ready2tW<StatSpec> {
        Ready2tW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
1T access readiness status indicator from NW. Comes later than 2T readiness. 1: FLASH banks are ready for 1T accesses 0: FLASH banks are not ready for 1T accesses"]
    #[inline(always)]
    #[must_use]
    pub fn ready1t(&mut self) -> Ready1tW<StatSpec> {
        Ready1tW::new(self, 3)
    }
    #[doc = "Bits 4:5 - 5:4\\]
NW FW_SMSTAT.CMD_IN_PROGRESS bit. This flag is valid immediately after the operation setting it 0 : Not busy 1 : Busy Bit 4 is for the busy state for Bank0 which is at logical address 0x0 Bit 5 for Bank1."]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BusyW<StatSpec> {
        BusyW::new(self, 4)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation"]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> Reserved7W<StatSpec> {
        Reserved7W::new(self, 6)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Same as EFUSEERROR.CODE"]
    #[inline(always)]
    #[must_use]
    pub fn efuse_errcode(&mut self) -> EfuseErrcodeW<StatSpec> {
        EfuseErrcodeW::new(self, 8)
    }
    #[doc = "Bit 13 - 13:13\\]
Efuse scanning resulted in scan chain Sparse byte error. 0 : No Sparse error 1 : Sparse Error"]
    #[inline(always)]
    #[must_use]
    pub fn sprs_byte_not_ok(&mut self) -> SprsByteNotOkW<StatSpec> {
        SprsByteNotOkW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Efuse scanning resulted in timeout error. 0 : No Timeout error 1 : Timeout Error"]
    #[inline(always)]
    #[must_use]
    pub fn efuse_timeout(&mut self) -> EfuseTimeoutW<StatSpec> {
        EfuseTimeoutW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Efuse scanning detected if fuse ROM is blank: 0 : Not blank 1 : Blank"]
    #[inline(always)]
    #[must_use]
    pub fn efuse_blank(&mut self) -> EfuseBlankW<StatSpec> {
        EfuseBlankW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
An ocp1 or ocp3 read stall has occurred. 0 : No stall or stall acknowledged by writing a 1 1 : Stall condition occurred/occurring This is a read/write-clear status bit. It will reset to 0. It will be set when either an ocp1 or ocp3 read occurs to a bank that is presently undergoing a program or write operation. An ocp2 write of 1 to this bit will clear the bit. The ocp2 write will take highest priority in the event an ocp1/ocp3 read is occurring concurrently to the ocp2 write. Clearing the bit should be done only after the ongoing program/erase operation is complete indicating that both banks are free. If clearing occurs while the stall condition persists, the field may get set back to one."]
    #[inline(always)]
    #[must_use]
    pub fn stallstat(&mut self) -> StallstatW<StatSpec> {
        StallstatW::new(self, 16)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved15(&mut self) -> Reserved15W<StatSpec> {
        Reserved15W::new(self, 17)
    }
}
#[doc = "NW and Efuse Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for StatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0;
}
