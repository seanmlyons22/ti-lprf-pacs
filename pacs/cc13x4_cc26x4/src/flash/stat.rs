#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT` writer"]
pub struct W(crate::W<STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT_SPEC>;
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
impl From<crate::W<STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POWER_MODE` reader - 1:0\\]
Power state of each of the 2 flash arbiter FSM instances in the flash sub-system. For Thor, these bits should mostly be in the same state since both banks are in the same power mode. 0 : Active 1 : Ready for Low power (The 2T readiness has gone low or the flash_off_req has been set=1, and flash_off_ack is ready to be asserted). Bit 0 is for the power state for Bank0 which is at logical address 0x0 Bit 1 for Bank1"]
pub type POWER_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `POWER_MODE` writer - 1:0\\]
Power state of each of the 2 flash arbiter FSM instances in the flash sub-system. For Thor, these bits should mostly be in the same state since both banks are in the same power mode. 0 : Active 1 : Ready for Low power (The 2T readiness has gone low or the flash_off_req has been set=1, and flash_off_ack is ready to be asserted). Bit 0 is for the power state for Bank0 which is at logical address 0x0 Bit 1 for Bank1"]
pub type POWER_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STAT_SPEC, u8, u8, 2, O>;
#[doc = "Field `READY2T` reader - 2:2\\]
2T access readiness status indicator from NW 1: FLASH banks are ready for 2T accesses 0: FLASH banks are not ready for 2T accesses"]
pub type READY2T_R = crate::BitReader<bool>;
#[doc = "Field `READY2T` writer - 2:2\\]
2T access readiness status indicator from NW 1: FLASH banks are ready for 2T accesses 0: FLASH banks are not ready for 2T accesses"]
pub type READY2T_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `READY1T` reader - 3:3\\]
1T access readiness status indicator from NW. Comes later than 2T readiness. 1: FLASH banks are ready for 1T accesses 0: FLASH banks are not ready for 1T accesses"]
pub type READY1T_R = crate::BitReader<bool>;
#[doc = "Field `READY1T` writer - 3:3\\]
1T access readiness status indicator from NW. Comes later than 2T readiness. 1: FLASH banks are ready for 1T accesses 0: FLASH banks are not ready for 1T accesses"]
pub type READY1T_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `BUSY` reader - 5:4\\]
NW FW_SMSTAT.CMD_IN_PROGRESS bit. This flag is valid immediately after the operation setting it 0 : Not busy 1 : Busy Bit 4 is for the busy state for Bank0 which is at logical address 0x0 Bit 5 for Bank1."]
pub type BUSY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BUSY` writer - 5:4\\]
NW FW_SMSTAT.CMD_IN_PROGRESS bit. This flag is valid immediately after the operation setting it 0 : Not busy 1 : Busy Bit 4 is for the busy state for Bank0 which is at logical address 0x0 Bit 5 for Bank1."]
pub type BUSY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STAT_SPEC, u8, u8, 2, O>;
#[doc = "Field `RESERVED7` reader - 7:6\\]
Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation"]
pub type RESERVED7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED7` writer - 7:6\\]
Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation"]
pub type RESERVED7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STAT_SPEC, u8, u8, 2, O>;
#[doc = "Field `EFUSE_ERRCODE` reader - 12:8\\]
Same as EFUSEERROR.CODE"]
pub type EFUSE_ERRCODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EFUSE_ERRCODE` writer - 12:8\\]
Same as EFUSEERROR.CODE"]
pub type EFUSE_ERRCODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STAT_SPEC, u8, u8, 5, O>;
#[doc = "Field `SPRS_BYTE_NOT_OK` reader - 13:13\\]
Efuse scanning resulted in scan chain Sparse byte error. 0 : No Sparse error 1 : Sparse Error"]
pub type SPRS_BYTE_NOT_OK_R = crate::BitReader<bool>;
#[doc = "Field `SPRS_BYTE_NOT_OK` writer - 13:13\\]
Efuse scanning resulted in scan chain Sparse byte error. 0 : No Sparse error 1 : Sparse Error"]
pub type SPRS_BYTE_NOT_OK_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `EFUSE_TIMEOUT` reader - 14:14\\]
Efuse scanning resulted in timeout error. 0 : No Timeout error 1 : Timeout Error"]
pub type EFUSE_TIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `EFUSE_TIMEOUT` writer - 14:14\\]
Efuse scanning resulted in timeout error. 0 : No Timeout error 1 : Timeout Error"]
pub type EFUSE_TIMEOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `EFUSE_BLANK` reader - 15:15\\]
Efuse scanning detected if fuse ROM is blank: 0 : Not blank 1 : Blank"]
pub type EFUSE_BLANK_R = crate::BitReader<bool>;
#[doc = "Field `EFUSE_BLANK` writer - 15:15\\]
Efuse scanning detected if fuse ROM is blank: 0 : Not blank 1 : Blank"]
pub type EFUSE_BLANK_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `STALLSTAT` reader - 16:16\\]
An ocp1 or ocp3 read stall has occurred. 0 : No stall or stall acknowledged by writing a 1 1 : Stall condition occurred/occurring This is a read/write-clear status bit. It will reset to 0. It will be set when either an ocp1 or ocp3 read occurs to a bank that is presently undergoing a program or write operation. An ocp2 write of 1 to this bit will clear the bit. The ocp2 write will take highest priority in the event an ocp1/ocp3 read is occurring concurrently to the ocp2 write. Clearing the bit should be done only after the ongoing program/erase operation is complete indicating that both banks are free. If clearing occurs while the stall condition persists, the field may get set back to one."]
pub type STALLSTAT_R = crate::BitReader<bool>;
#[doc = "Field `STALLSTAT` writer - 16:16\\]
An ocp1 or ocp3 read stall has occurred. 0 : No stall or stall acknowledged by writing a 1 1 : Stall condition occurred/occurring This is a read/write-clear status bit. It will reset to 0. It will be set when either an ocp1 or ocp3 read occurs to a bank that is presently undergoing a program or write operation. An ocp2 write of 1 to this bit will clear the bit. The ocp2 write will take highest priority in the event an ocp1/ocp3 read is occurring concurrently to the ocp2 write. Clearing the bit should be done only after the ongoing program/erase operation is complete indicating that both banks are free. If clearing occurs while the stall condition persists, the field may get set back to one."]
pub type STALLSTAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `RESERVED15` reader - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED15_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED15` writer - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STAT_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Power state of each of the 2 flash arbiter FSM instances in the flash sub-system. For Thor, these bits should mostly be in the same state since both banks are in the same power mode. 0 : Active 1 : Ready for Low power (The 2T readiness has gone low or the flash_off_req has been set=1, and flash_off_ack is ready to be asserted). Bit 0 is for the power state for Bank0 which is at logical address 0x0 Bit 1 for Bank1"]
    #[inline(always)]
    pub fn power_mode(&self) -> POWER_MODE_R {
        POWER_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
2T access readiness status indicator from NW 1: FLASH banks are ready for 2T accesses 0: FLASH banks are not ready for 2T accesses"]
    #[inline(always)]
    pub fn ready2t(&self) -> READY2T_R {
        READY2T_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
1T access readiness status indicator from NW. Comes later than 2T readiness. 1: FLASH banks are ready for 1T accesses 0: FLASH banks are not ready for 1T accesses"]
    #[inline(always)]
    pub fn ready1t(&self) -> READY1T_R {
        READY1T_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
NW FW_SMSTAT.CMD_IN_PROGRESS bit. This flag is valid immediately after the operation setting it 0 : Not busy 1 : Busy Bit 4 is for the busy state for Bank0 which is at logical address 0x0 Bit 5 for Bank1."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation"]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Same as EFUSEERROR.CODE"]
    #[inline(always)]
    pub fn efuse_errcode(&self) -> EFUSE_ERRCODE_R {
        EFUSE_ERRCODE_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - 13:13\\]
Efuse scanning resulted in scan chain Sparse byte error. 0 : No Sparse error 1 : Sparse Error"]
    #[inline(always)]
    pub fn sprs_byte_not_ok(&self) -> SPRS_BYTE_NOT_OK_R {
        SPRS_BYTE_NOT_OK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Efuse scanning resulted in timeout error. 0 : No Timeout error 1 : Timeout Error"]
    #[inline(always)]
    pub fn efuse_timeout(&self) -> EFUSE_TIMEOUT_R {
        EFUSE_TIMEOUT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Efuse scanning detected if fuse ROM is blank: 0 : Not blank 1 : Blank"]
    #[inline(always)]
    pub fn efuse_blank(&self) -> EFUSE_BLANK_R {
        EFUSE_BLANK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
An ocp1 or ocp3 read stall has occurred. 0 : No stall or stall acknowledged by writing a 1 1 : Stall condition occurred/occurring This is a read/write-clear status bit. It will reset to 0. It will be set when either an ocp1 or ocp3 read occurs to a bank that is presently undergoing a program or write operation. An ocp2 write of 1 to this bit will clear the bit. The ocp2 write will take highest priority in the event an ocp1/ocp3 read is occurring concurrently to the ocp2 write. Clearing the bit should be done only after the ongoing program/erase operation is complete indicating that both banks are free. If clearing occurs while the stall condition persists, the field may get set back to one."]
    #[inline(always)]
    pub fn stallstat(&self) -> STALLSTAT_R {
        STALLSTAT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved15(&self) -> RESERVED15_R {
        RESERVED15_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Power state of each of the 2 flash arbiter FSM instances in the flash sub-system. For Thor, these bits should mostly be in the same state since both banks are in the same power mode. 0 : Active 1 : Ready for Low power (The 2T readiness has gone low or the flash_off_req has been set=1, and flash_off_ack is ready to be asserted). Bit 0 is for the power state for Bank0 which is at logical address 0x0 Bit 1 for Bank1"]
    #[inline(always)]
    #[must_use]
    pub fn power_mode(&mut self) -> POWER_MODE_W<0> {
        POWER_MODE_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
2T access readiness status indicator from NW 1: FLASH banks are ready for 2T accesses 0: FLASH banks are not ready for 2T accesses"]
    #[inline(always)]
    #[must_use]
    pub fn ready2t(&mut self) -> READY2T_W<2> {
        READY2T_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
1T access readiness status indicator from NW. Comes later than 2T readiness. 1: FLASH banks are ready for 1T accesses 0: FLASH banks are not ready for 1T accesses"]
    #[inline(always)]
    #[must_use]
    pub fn ready1t(&mut self) -> READY1T_W<3> {
        READY1T_W::new(self)
    }
    #[doc = "Bits 4:5 - 5:4\\]
NW FW_SMSTAT.CMD_IN_PROGRESS bit. This flag is valid immediately after the operation setting it 0 : Not busy 1 : Busy Bit 4 is for the busy state for Bank0 which is at logical address 0x0 Bit 5 for Bank1."]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BUSY_W<4> {
        BUSY_W::new(self)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation"]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> RESERVED7_W<6> {
        RESERVED7_W::new(self)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Same as EFUSEERROR.CODE"]
    #[inline(always)]
    #[must_use]
    pub fn efuse_errcode(&mut self) -> EFUSE_ERRCODE_W<8> {
        EFUSE_ERRCODE_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
Efuse scanning resulted in scan chain Sparse byte error. 0 : No Sparse error 1 : Sparse Error"]
    #[inline(always)]
    #[must_use]
    pub fn sprs_byte_not_ok(&mut self) -> SPRS_BYTE_NOT_OK_W<13> {
        SPRS_BYTE_NOT_OK_W::new(self)
    }
    #[doc = "Bit 14 - 14:14\\]
Efuse scanning resulted in timeout error. 0 : No Timeout error 1 : Timeout Error"]
    #[inline(always)]
    #[must_use]
    pub fn efuse_timeout(&mut self) -> EFUSE_TIMEOUT_W<14> {
        EFUSE_TIMEOUT_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
Efuse scanning detected if fuse ROM is blank: 0 : Not blank 1 : Blank"]
    #[inline(always)]
    #[must_use]
    pub fn efuse_blank(&mut self) -> EFUSE_BLANK_W<15> {
        EFUSE_BLANK_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
An ocp1 or ocp3 read stall has occurred. 0 : No stall or stall acknowledged by writing a 1 1 : Stall condition occurred/occurring This is a read/write-clear status bit. It will reset to 0. It will be set when either an ocp1 or ocp3 read occurs to a bank that is presently undergoing a program or write operation. An ocp2 write of 1 to this bit will clear the bit. The ocp2 write will take highest priority in the event an ocp1/ocp3 read is occurring concurrently to the ocp2 write. Clearing the bit should be done only after the ongoing program/erase operation is complete indicating that both banks are free. If clearing occurs while the stall condition persists, the field may get set back to one."]
    #[inline(always)]
    #[must_use]
    pub fn stallstat(&mut self) -> STALLSTAT_W<16> {
        STALLSTAT_W::new(self)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved15(&mut self) -> RESERVED15_W<17> {
        RESERVED15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NW and Efuse Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat::W](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
