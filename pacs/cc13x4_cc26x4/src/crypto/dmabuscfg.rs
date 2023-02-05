#[doc = "Register `DMABUSCFG` reader"]
pub struct R(crate::R<DMABUSCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMABUSCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMABUSCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMABUSCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMABUSCFG` writer"]
pub struct W(crate::W<DMABUSCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMABUSCFG_SPEC>;
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
impl From<crate::W<DMABUSCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMABUSCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED0` writer - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMABUSCFG_SPEC, u8, u8, 8, O>;
#[doc = "Field `AHB_MST1_BIGEND` reader - 8:8\\]
Endianess for the AHB master"]
pub type AHB_MST1_BIGEND_R = crate::BitReader<AHB_MST1_BIGEND_A>;
#[doc = "8:8\\]
Endianess for the AHB master\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHB_MST1_BIGEND_A {
    #[doc = "1: Big Endian"]
    BIG_ENDIAN = 1,
    #[doc = "0: Little Endian"]
    LITTLE_ENDIAN = 0,
}
impl From<AHB_MST1_BIGEND_A> for bool {
    #[inline(always)]
    fn from(variant: AHB_MST1_BIGEND_A) -> Self {
        variant as u8 != 0
    }
}
impl AHB_MST1_BIGEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHB_MST1_BIGEND_A {
        match self.bits {
            true => AHB_MST1_BIGEND_A::BIG_ENDIAN,
            false => AHB_MST1_BIGEND_A::LITTLE_ENDIAN,
        }
    }
    #[doc = "Checks if the value of the field is `BIG_ENDIAN`"]
    #[inline(always)]
    pub fn is_big_endian(&self) -> bool {
        *self == AHB_MST1_BIGEND_A::BIG_ENDIAN
    }
    #[doc = "Checks if the value of the field is `LITTLE_ENDIAN`"]
    #[inline(always)]
    pub fn is_little_endian(&self) -> bool {
        *self == AHB_MST1_BIGEND_A::LITTLE_ENDIAN
    }
}
#[doc = "Field `AHB_MST1_BIGEND` writer - 8:8\\]
Endianess for the AHB master"]
pub type AHB_MST1_BIGEND_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMABUSCFG_SPEC, AHB_MST1_BIGEND_A, O>;
impl<'a, const O: u8> AHB_MST1_BIGEND_W<'a, O> {
    #[doc = "Big Endian"]
    #[inline(always)]
    pub fn big_endian(self) -> &'a mut W {
        self.variant(AHB_MST1_BIGEND_A::BIG_ENDIAN)
    }
    #[doc = "Little Endian"]
    #[inline(always)]
    pub fn little_endian(self) -> &'a mut W {
        self.variant(AHB_MST1_BIGEND_A::LITTLE_ENDIAN)
    }
}
#[doc = "Field `AHB_MST1_LOCK_EN` reader - 9:9\\]
Locked transform on AHB"]
pub type AHB_MST1_LOCK_EN_R = crate::BitReader<AHB_MST1_LOCK_EN_A>;
#[doc = "9:9\\]
Locked transform on AHB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHB_MST1_LOCK_EN_A {
    #[doc = "1: Transfers are locked"]
    LOCKED = 1,
    #[doc = "0: Transfers are not locked"]
    NOT_LOCKED = 0,
}
impl From<AHB_MST1_LOCK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: AHB_MST1_LOCK_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl AHB_MST1_LOCK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHB_MST1_LOCK_EN_A {
        match self.bits {
            true => AHB_MST1_LOCK_EN_A::LOCKED,
            false => AHB_MST1_LOCK_EN_A::NOT_LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == AHB_MST1_LOCK_EN_A::LOCKED
    }
    #[doc = "Checks if the value of the field is `NOT_LOCKED`"]
    #[inline(always)]
    pub fn is_not_locked(&self) -> bool {
        *self == AHB_MST1_LOCK_EN_A::NOT_LOCKED
    }
}
#[doc = "Field `AHB_MST1_LOCK_EN` writer - 9:9\\]
Locked transform on AHB"]
pub type AHB_MST1_LOCK_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMABUSCFG_SPEC, AHB_MST1_LOCK_EN_A, O>;
impl<'a, const O: u8> AHB_MST1_LOCK_EN_W<'a, O> {
    #[doc = "Transfers are locked"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(AHB_MST1_LOCK_EN_A::LOCKED)
    }
    #[doc = "Transfers are not locked"]
    #[inline(always)]
    pub fn not_locked(self) -> &'a mut W {
        self.variant(AHB_MST1_LOCK_EN_A::NOT_LOCKED)
    }
}
#[doc = "Field `AHB_MST1_INCR_EN` reader - 10:10\\]
Burst length type of AHB transfer"]
pub type AHB_MST1_INCR_EN_R = crate::BitReader<AHB_MST1_INCR_EN_A>;
#[doc = "10:10\\]
Burst length type of AHB transfer\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHB_MST1_INCR_EN_A {
    #[doc = "1: Fixed length bursts or single transfers"]
    SPECIFIED = 1,
    #[doc = "0: Unspecified length burst transfers"]
    UNSPECIFIED = 0,
}
impl From<AHB_MST1_INCR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: AHB_MST1_INCR_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl AHB_MST1_INCR_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHB_MST1_INCR_EN_A {
        match self.bits {
            true => AHB_MST1_INCR_EN_A::SPECIFIED,
            false => AHB_MST1_INCR_EN_A::UNSPECIFIED,
        }
    }
    #[doc = "Checks if the value of the field is `SPECIFIED`"]
    #[inline(always)]
    pub fn is_specified(&self) -> bool {
        *self == AHB_MST1_INCR_EN_A::SPECIFIED
    }
    #[doc = "Checks if the value of the field is `UNSPECIFIED`"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        *self == AHB_MST1_INCR_EN_A::UNSPECIFIED
    }
}
#[doc = "Field `AHB_MST1_INCR_EN` writer - 10:10\\]
Burst length type of AHB transfer"]
pub type AHB_MST1_INCR_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMABUSCFG_SPEC, AHB_MST1_INCR_EN_A, O>;
impl<'a, const O: u8> AHB_MST1_INCR_EN_W<'a, O> {
    #[doc = "Fixed length bursts or single transfers"]
    #[inline(always)]
    pub fn specified(self) -> &'a mut W {
        self.variant(AHB_MST1_INCR_EN_A::SPECIFIED)
    }
    #[doc = "Unspecified length burst transfers"]
    #[inline(always)]
    pub fn unspecified(self) -> &'a mut W {
        self.variant(AHB_MST1_INCR_EN_A::UNSPECIFIED)
    }
}
#[doc = "Field `AHB_MST1_IDLE_EN` reader - 11:11\\]
Idle insertion between consecutive burst transfers on AHB"]
pub type AHB_MST1_IDLE_EN_R = crate::BitReader<AHB_MST1_IDLE_EN_A>;
#[doc = "11:11\\]
Idle insertion between consecutive burst transfers on AHB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHB_MST1_IDLE_EN_A {
    #[doc = "1: Idle transfer insertion enabled"]
    IDLE = 1,
    #[doc = "0: Do not insert idle transfers."]
    NO_IDLE = 0,
}
impl From<AHB_MST1_IDLE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: AHB_MST1_IDLE_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl AHB_MST1_IDLE_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHB_MST1_IDLE_EN_A {
        match self.bits {
            true => AHB_MST1_IDLE_EN_A::IDLE,
            false => AHB_MST1_IDLE_EN_A::NO_IDLE,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == AHB_MST1_IDLE_EN_A::IDLE
    }
    #[doc = "Checks if the value of the field is `NO_IDLE`"]
    #[inline(always)]
    pub fn is_no_idle(&self) -> bool {
        *self == AHB_MST1_IDLE_EN_A::NO_IDLE
    }
}
#[doc = "Field `AHB_MST1_IDLE_EN` writer - 11:11\\]
Idle insertion between consecutive burst transfers on AHB"]
pub type AHB_MST1_IDLE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMABUSCFG_SPEC, AHB_MST1_IDLE_EN_A, O>;
impl<'a, const O: u8> AHB_MST1_IDLE_EN_W<'a, O> {
    #[doc = "Idle transfer insertion enabled"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(AHB_MST1_IDLE_EN_A::IDLE)
    }
    #[doc = "Do not insert idle transfers."]
    #[inline(always)]
    pub fn no_idle(self) -> &'a mut W {
        self.variant(AHB_MST1_IDLE_EN_A::NO_IDLE)
    }
}
#[doc = "Field `AHB_MST1_BURST_SIZE` reader - 15:12\\]
Maximum burst size that can be performed on the AHB bus"]
pub type AHB_MST1_BURST_SIZE_R = crate::FieldReader<u8, AHB_MST1_BURST_SIZE_A>;
#[doc = "15:12\\]
Maximum burst size that can be performed on the AHB bus\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AHB_MST1_BURST_SIZE_A {
    #[doc = "6: 64 bytes"]
    _64_BYTE = 6,
    #[doc = "5: 32 bytes"]
    _32_BYTE = 5,
    #[doc = "4: 16 bytes"]
    _16_BYTE = 4,
    #[doc = "3: 8 bytes"]
    _8_BYTE = 3,
    #[doc = "2: 4 bytes"]
    _4_BYTE = 2,
}
impl From<AHB_MST1_BURST_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: AHB_MST1_BURST_SIZE_A) -> Self {
        variant as _
    }
}
impl AHB_MST1_BURST_SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AHB_MST1_BURST_SIZE_A> {
        match self.bits {
            6 => Some(AHB_MST1_BURST_SIZE_A::_64_BYTE),
            5 => Some(AHB_MST1_BURST_SIZE_A::_32_BYTE),
            4 => Some(AHB_MST1_BURST_SIZE_A::_16_BYTE),
            3 => Some(AHB_MST1_BURST_SIZE_A::_8_BYTE),
            2 => Some(AHB_MST1_BURST_SIZE_A::_4_BYTE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_64_BYTE`"]
    #[inline(always)]
    pub fn is_64_byte(&self) -> bool {
        *self == AHB_MST1_BURST_SIZE_A::_64_BYTE
    }
    #[doc = "Checks if the value of the field is `_32_BYTE`"]
    #[inline(always)]
    pub fn is_32_byte(&self) -> bool {
        *self == AHB_MST1_BURST_SIZE_A::_32_BYTE
    }
    #[doc = "Checks if the value of the field is `_16_BYTE`"]
    #[inline(always)]
    pub fn is_16_byte(&self) -> bool {
        *self == AHB_MST1_BURST_SIZE_A::_16_BYTE
    }
    #[doc = "Checks if the value of the field is `_8_BYTE`"]
    #[inline(always)]
    pub fn is_8_byte(&self) -> bool {
        *self == AHB_MST1_BURST_SIZE_A::_8_BYTE
    }
    #[doc = "Checks if the value of the field is `_4_BYTE`"]
    #[inline(always)]
    pub fn is_4_byte(&self) -> bool {
        *self == AHB_MST1_BURST_SIZE_A::_4_BYTE
    }
}
#[doc = "Field `AHB_MST1_BURST_SIZE` writer - 15:12\\]
Maximum burst size that can be performed on the AHB bus"]
pub type AHB_MST1_BURST_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMABUSCFG_SPEC, u8, AHB_MST1_BURST_SIZE_A, 4, O>;
impl<'a, const O: u8> AHB_MST1_BURST_SIZE_W<'a, O> {
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn _64_byte(self) -> &'a mut W {
        self.variant(AHB_MST1_BURST_SIZE_A::_64_BYTE)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn _32_byte(self) -> &'a mut W {
        self.variant(AHB_MST1_BURST_SIZE_A::_32_BYTE)
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn _16_byte(self) -> &'a mut W {
        self.variant(AHB_MST1_BURST_SIZE_A::_16_BYTE)
    }
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn _8_byte(self) -> &'a mut W {
        self.variant(AHB_MST1_BURST_SIZE_A::_8_BYTE)
    }
    #[doc = "4 bytes"]
    #[inline(always)]
    pub fn _4_byte(self) -> &'a mut W {
        self.variant(AHB_MST1_BURST_SIZE_A::_4_BYTE)
    }
}
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMABUSCFG_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Endianess for the AHB master"]
    #[inline(always)]
    pub fn ahb_mst1_bigend(&self) -> AHB_MST1_BIGEND_R {
        AHB_MST1_BIGEND_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Locked transform on AHB"]
    #[inline(always)]
    pub fn ahb_mst1_lock_en(&self) -> AHB_MST1_LOCK_EN_R {
        AHB_MST1_LOCK_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Burst length type of AHB transfer"]
    #[inline(always)]
    pub fn ahb_mst1_incr_en(&self) -> AHB_MST1_INCR_EN_R {
        AHB_MST1_INCR_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Idle insertion between consecutive burst transfers on AHB"]
    #[inline(always)]
    pub fn ahb_mst1_idle_en(&self) -> AHB_MST1_IDLE_EN_R {
        AHB_MST1_IDLE_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Maximum burst size that can be performed on the AHB bus"]
    #[inline(always)]
    pub fn ahb_mst1_burst_size(&self) -> AHB_MST1_BURST_SIZE_R {
        AHB_MST1_BURST_SIZE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<0> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Endianess for the AHB master"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_mst1_bigend(&mut self) -> AHB_MST1_BIGEND_W<8> {
        AHB_MST1_BIGEND_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Locked transform on AHB"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_mst1_lock_en(&mut self) -> AHB_MST1_LOCK_EN_W<9> {
        AHB_MST1_LOCK_EN_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
Burst length type of AHB transfer"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_mst1_incr_en(&mut self) -> AHB_MST1_INCR_EN_W<10> {
        AHB_MST1_INCR_EN_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
Idle insertion between consecutive burst transfers on AHB"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_mst1_idle_en(&mut self) -> AHB_MST1_IDLE_EN_W<11> {
        AHB_MST1_IDLE_EN_W::new(self)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Maximum burst size that can be performed on the AHB bus"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_mst1_burst_size(&mut self) -> AHB_MST1_BURST_SIZE_W<12> {
        AHB_MST1_BURST_SIZE_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> RESERVED16_W<16> {
        RESERVED16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Master Run-time Parameters This register defines all the run-time parameters for the AHB master interface port. These parameters are required for the proper functioning of the EIP-101m AHB master adapter.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmabuscfg](index.html) module"]
pub struct DMABUSCFG_SPEC;
impl crate::RegisterSpec for DMABUSCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmabuscfg::R](R) reader structure"]
impl crate::Readable for DMABUSCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmabuscfg::W](W) writer structure"]
impl crate::Writable for DMABUSCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMABUSCFG to value 0x2400"]
impl crate::Resettable for DMABUSCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x2400;
}
