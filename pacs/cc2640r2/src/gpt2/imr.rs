#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMR` writer"]
pub struct W(crate::W<IMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMR_SPEC>;
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
impl From<crate::W<IMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TATOIM` reader - 0:0\\]
Enabling this bit will make the RIS.TATORIS interrupt propagate to MIS.TATOMIS"]
pub type TATOIM_R = crate::BitReader<TATOIM_A>;
#[doc = "0:0\\]
Enabling this bit will make the RIS.TATORIS interrupt propagate to MIS.TATOMIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TATOIM_A {
    #[doc = "1: Enable Interrupt"]
    EN = 1,
    #[doc = "0: Disable Interrupt"]
    DIS = 0,
}
impl From<TATOIM_A> for bool {
    #[inline(always)]
    fn from(variant: TATOIM_A) -> Self {
        variant as u8 != 0
    }
}
impl TATOIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TATOIM_A {
        match self.bits {
            true => TATOIM_A::EN,
            false => TATOIM_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TATOIM_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TATOIM_A::DIS
    }
}
#[doc = "Field `TATOIM` writer - 0:0\\]
Enabling this bit will make the RIS.TATORIS interrupt propagate to MIS.TATOMIS"]
pub type TATOIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, TATOIM_A, O>;
impl<'a, const O: u8> TATOIM_W<'a, O> {
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TATOIM_A::EN)
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TATOIM_A::DIS)
    }
}
#[doc = "Field `CAMIM` reader - 1:1\\]
Enabling this bit will make the RIS.CAMRIS interrupt propagate to MIS.CAMMIS"]
pub type CAMIM_R = crate::BitReader<CAMIM_A>;
#[doc = "1:1\\]
Enabling this bit will make the RIS.CAMRIS interrupt propagate to MIS.CAMMIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAMIM_A {
    #[doc = "1: Enable Interrupt"]
    EN = 1,
    #[doc = "0: Disable Interrupt"]
    DIS = 0,
}
impl From<CAMIM_A> for bool {
    #[inline(always)]
    fn from(variant: CAMIM_A) -> Self {
        variant as u8 != 0
    }
}
impl CAMIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAMIM_A {
        match self.bits {
            true => CAMIM_A::EN,
            false => CAMIM_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CAMIM_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CAMIM_A::DIS
    }
}
#[doc = "Field `CAMIM` writer - 1:1\\]
Enabling this bit will make the RIS.CAMRIS interrupt propagate to MIS.CAMMIS"]
pub type CAMIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, CAMIM_A, O>;
impl<'a, const O: u8> CAMIM_W<'a, O> {
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CAMIM_A::EN)
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CAMIM_A::DIS)
    }
}
#[doc = "Field `CAEIM` reader - 2:2\\]
Enabling this bit will make the RIS.CAERIS interrupt propagate to MIS.CAEMIS"]
pub type CAEIM_R = crate::BitReader<CAEIM_A>;
#[doc = "2:2\\]
Enabling this bit will make the RIS.CAERIS interrupt propagate to MIS.CAEMIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAEIM_A {
    #[doc = "1: Enable Interrupt"]
    EN = 1,
    #[doc = "0: Disable Interrupt"]
    DIS = 0,
}
impl From<CAEIM_A> for bool {
    #[inline(always)]
    fn from(variant: CAEIM_A) -> Self {
        variant as u8 != 0
    }
}
impl CAEIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAEIM_A {
        match self.bits {
            true => CAEIM_A::EN,
            false => CAEIM_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CAEIM_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CAEIM_A::DIS
    }
}
#[doc = "Field `CAEIM` writer - 2:2\\]
Enabling this bit will make the RIS.CAERIS interrupt propagate to MIS.CAEMIS"]
pub type CAEIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, CAEIM_A, O>;
impl<'a, const O: u8> CAEIM_W<'a, O> {
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CAEIM_A::EN)
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CAEIM_A::DIS)
    }
}
#[doc = "Field `RESERVED3` reader - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED3` writer - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `TAMIM` reader - 4:4\\]
Enabling this bit will make the RIS.TAMRIS interrupt propagate to MIS.TAMMIS"]
pub type TAMIM_R = crate::BitReader<TAMIM_A>;
#[doc = "4:4\\]
Enabling this bit will make the RIS.TAMRIS interrupt propagate to MIS.TAMMIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMIM_A {
    #[doc = "1: Enable Interrupt"]
    EN = 1,
    #[doc = "0: Disable Interrupt"]
    DIS = 0,
}
impl From<TAMIM_A> for bool {
    #[inline(always)]
    fn from(variant: TAMIM_A) -> Self {
        variant as u8 != 0
    }
}
impl TAMIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMIM_A {
        match self.bits {
            true => TAMIM_A::EN,
            false => TAMIM_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TAMIM_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TAMIM_A::DIS
    }
}
#[doc = "Field `TAMIM` writer - 4:4\\]
Enabling this bit will make the RIS.TAMRIS interrupt propagate to MIS.TAMMIS"]
pub type TAMIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, TAMIM_A, O>;
impl<'a, const O: u8> TAMIM_W<'a, O> {
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TAMIM_A::EN)
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TAMIM_A::DIS)
    }
}
#[doc = "Field `DMAAIM` reader - 5:5\\]
Enabling this bit will make the RIS.DMAARIS interrupt propagate to MIS.DMAAMIS"]
pub type DMAAIM_R = crate::BitReader<DMAAIM_A>;
#[doc = "5:5\\]
Enabling this bit will make the RIS.DMAARIS interrupt propagate to MIS.DMAAMIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAAIM_A {
    #[doc = "1: Enable Interrupt"]
    EN = 1,
    #[doc = "0: Disable Interrupt"]
    DIS = 0,
}
impl From<DMAAIM_A> for bool {
    #[inline(always)]
    fn from(variant: DMAAIM_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAAIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAAIM_A {
        match self.bits {
            true => DMAAIM_A::EN,
            false => DMAAIM_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == DMAAIM_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == DMAAIM_A::DIS
    }
}
#[doc = "Field `DMAAIM` writer - 5:5\\]
Enabling this bit will make the RIS.DMAARIS interrupt propagate to MIS.DMAAMIS"]
pub type DMAAIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, DMAAIM_A, O>;
impl<'a, const O: u8> DMAAIM_W<'a, O> {
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(DMAAIM_A::EN)
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(DMAAIM_A::DIS)
    }
}
#[doc = "Field `RESERVED6` reader - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED6` writer - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IMR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TBTOIM` reader - 8:8\\]
Enabling this bit will make the RIS.TBTORIS interrupt propagate to MIS.TBTOMIS"]
pub type TBTOIM_R = crate::BitReader<TBTOIM_A>;
#[doc = "8:8\\]
Enabling this bit will make the RIS.TBTORIS interrupt propagate to MIS.TBTOMIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBTOIM_A {
    #[doc = "1: Enable Interrupt"]
    EN = 1,
    #[doc = "0: Disable Interrupt"]
    DIS = 0,
}
impl From<TBTOIM_A> for bool {
    #[inline(always)]
    fn from(variant: TBTOIM_A) -> Self {
        variant as u8 != 0
    }
}
impl TBTOIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBTOIM_A {
        match self.bits {
            true => TBTOIM_A::EN,
            false => TBTOIM_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TBTOIM_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TBTOIM_A::DIS
    }
}
#[doc = "Field `TBTOIM` writer - 8:8\\]
Enabling this bit will make the RIS.TBTORIS interrupt propagate to MIS.TBTOMIS"]
pub type TBTOIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, TBTOIM_A, O>;
impl<'a, const O: u8> TBTOIM_W<'a, O> {
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TBTOIM_A::EN)
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TBTOIM_A::DIS)
    }
}
#[doc = "Field `CBMIM` reader - 9:9\\]
Enabling this bit will make the RIS.CBMRIS interrupt propagate to MIS.CBMMIS"]
pub type CBMIM_R = crate::BitReader<CBMIM_A>;
#[doc = "9:9\\]
Enabling this bit will make the RIS.CBMRIS interrupt propagate to MIS.CBMMIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CBMIM_A {
    #[doc = "1: Enable Interrupt"]
    EN = 1,
    #[doc = "0: Disable Interrupt"]
    DIS = 0,
}
impl From<CBMIM_A> for bool {
    #[inline(always)]
    fn from(variant: CBMIM_A) -> Self {
        variant as u8 != 0
    }
}
impl CBMIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CBMIM_A {
        match self.bits {
            true => CBMIM_A::EN,
            false => CBMIM_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CBMIM_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CBMIM_A::DIS
    }
}
#[doc = "Field `CBMIM` writer - 9:9\\]
Enabling this bit will make the RIS.CBMRIS interrupt propagate to MIS.CBMMIS"]
pub type CBMIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, CBMIM_A, O>;
impl<'a, const O: u8> CBMIM_W<'a, O> {
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CBMIM_A::EN)
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CBMIM_A::DIS)
    }
}
#[doc = "Field `CBEIM` reader - 10:10\\]
Enabling this bit will make the RIS.CBERIS interrupt propagate to MIS.CBEMIS"]
pub type CBEIM_R = crate::BitReader<CBEIM_A>;
#[doc = "10:10\\]
Enabling this bit will make the RIS.CBERIS interrupt propagate to MIS.CBEMIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CBEIM_A {
    #[doc = "1: Enable Interrupt"]
    EN = 1,
    #[doc = "0: Disable Interrupt"]
    DIS = 0,
}
impl From<CBEIM_A> for bool {
    #[inline(always)]
    fn from(variant: CBEIM_A) -> Self {
        variant as u8 != 0
    }
}
impl CBEIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CBEIM_A {
        match self.bits {
            true => CBEIM_A::EN,
            false => CBEIM_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CBEIM_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CBEIM_A::DIS
    }
}
#[doc = "Field `CBEIM` writer - 10:10\\]
Enabling this bit will make the RIS.CBERIS interrupt propagate to MIS.CBEMIS"]
pub type CBEIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, CBEIM_A, O>;
impl<'a, const O: u8> CBEIM_W<'a, O> {
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CBEIM_A::EN)
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CBEIM_A::DIS)
    }
}
#[doc = "Field `TBMIM` reader - 11:11\\]
Enabling this bit will make the RIS.TBMRIS interrupt propagate to MIS.TBMMIS"]
pub type TBMIM_R = crate::BitReader<TBMIM_A>;
#[doc = "11:11\\]
Enabling this bit will make the RIS.TBMRIS interrupt propagate to MIS.TBMMIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBMIM_A {
    #[doc = "1: Enable Interrupt"]
    EN = 1,
    #[doc = "0: Disable Interrupt"]
    DIS = 0,
}
impl From<TBMIM_A> for bool {
    #[inline(always)]
    fn from(variant: TBMIM_A) -> Self {
        variant as u8 != 0
    }
}
impl TBMIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBMIM_A {
        match self.bits {
            true => TBMIM_A::EN,
            false => TBMIM_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TBMIM_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TBMIM_A::DIS
    }
}
#[doc = "Field `TBMIM` writer - 11:11\\]
Enabling this bit will make the RIS.TBMRIS interrupt propagate to MIS.TBMMIS"]
pub type TBMIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, TBMIM_A, O>;
impl<'a, const O: u8> TBMIM_W<'a, O> {
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TBMIM_A::EN)
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TBMIM_A::DIS)
    }
}
#[doc = "Field `RESERVED12` reader - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED12_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED12` writer - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED12_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `DMABIM` reader - 13:13\\]
Enabling this bit will make the RIS.DMABRIS interrupt propagate to MIS.DMABMIS"]
pub type DMABIM_R = crate::BitReader<DMABIM_A>;
#[doc = "13:13\\]
Enabling this bit will make the RIS.DMABRIS interrupt propagate to MIS.DMABMIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMABIM_A {
    #[doc = "1: Enable Interrupt"]
    EN = 1,
    #[doc = "0: Disable Interrupt"]
    DIS = 0,
}
impl From<DMABIM_A> for bool {
    #[inline(always)]
    fn from(variant: DMABIM_A) -> Self {
        variant as u8 != 0
    }
}
impl DMABIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMABIM_A {
        match self.bits {
            true => DMABIM_A::EN,
            false => DMABIM_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == DMABIM_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == DMABIM_A::DIS
    }
}
#[doc = "Field `DMABIM` writer - 13:13\\]
Enabling this bit will make the RIS.DMABRIS interrupt propagate to MIS.DMABMIS"]
pub type DMABIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, DMABIM_A, O>;
impl<'a, const O: u8> DMABIM_W<'a, O> {
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(DMABIM_A::EN)
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(DMABIM_A::DIS)
    }
}
#[doc = "Field `RESERVED14` reader - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED14_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED14` writer - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IMR_SPEC, u32, u32, 18, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enabling this bit will make the RIS.TATORIS interrupt propagate to MIS.TATOMIS"]
    #[inline(always)]
    pub fn tatoim(&self) -> TATOIM_R {
        TATOIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enabling this bit will make the RIS.CAMRIS interrupt propagate to MIS.CAMMIS"]
    #[inline(always)]
    pub fn camim(&self) -> CAMIM_R {
        CAMIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enabling this bit will make the RIS.CAERIS interrupt propagate to MIS.CAEMIS"]
    #[inline(always)]
    pub fn caeim(&self) -> CAEIM_R {
        CAEIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Enabling this bit will make the RIS.TAMRIS interrupt propagate to MIS.TAMMIS"]
    #[inline(always)]
    pub fn tamim(&self) -> TAMIM_R {
        TAMIM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Enabling this bit will make the RIS.DMAARIS interrupt propagate to MIS.DMAAMIS"]
    #[inline(always)]
    pub fn dmaaim(&self) -> DMAAIM_R {
        DMAAIM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Enabling this bit will make the RIS.TBTORIS interrupt propagate to MIS.TBTOMIS"]
    #[inline(always)]
    pub fn tbtoim(&self) -> TBTOIM_R {
        TBTOIM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Enabling this bit will make the RIS.CBMRIS interrupt propagate to MIS.CBMMIS"]
    #[inline(always)]
    pub fn cbmim(&self) -> CBMIM_R {
        CBMIM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Enabling this bit will make the RIS.CBERIS interrupt propagate to MIS.CBEMIS"]
    #[inline(always)]
    pub fn cbeim(&self) -> CBEIM_R {
        CBEIM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Enabling this bit will make the RIS.TBMRIS interrupt propagate to MIS.TBMMIS"]
    #[inline(always)]
    pub fn tbmim(&self) -> TBMIM_R {
        TBMIM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Enabling this bit will make the RIS.DMABRIS interrupt propagate to MIS.DMABMIS"]
    #[inline(always)]
    pub fn dmabim(&self) -> DMABIM_R {
        DMABIM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&self) -> RESERVED14_R {
        RESERVED14_R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enabling this bit will make the RIS.TATORIS interrupt propagate to MIS.TATOMIS"]
    #[inline(always)]
    #[must_use]
    pub fn tatoim(&mut self) -> TATOIM_W<0> {
        TATOIM_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Enabling this bit will make the RIS.CAMRIS interrupt propagate to MIS.CAMMIS"]
    #[inline(always)]
    #[must_use]
    pub fn camim(&mut self) -> CAMIM_W<1> {
        CAMIM_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Enabling this bit will make the RIS.CAERIS interrupt propagate to MIS.CAEMIS"]
    #[inline(always)]
    #[must_use]
    pub fn caeim(&mut self) -> CAEIM_W<2> {
        CAEIM_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<3> {
        RESERVED3_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Enabling this bit will make the RIS.TAMRIS interrupt propagate to MIS.TAMMIS"]
    #[inline(always)]
    #[must_use]
    pub fn tamim(&mut self) -> TAMIM_W<4> {
        TAMIM_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Enabling this bit will make the RIS.DMAARIS interrupt propagate to MIS.DMAAMIS"]
    #[inline(always)]
    #[must_use]
    pub fn dmaaim(&mut self) -> DMAAIM_W<5> {
        DMAAIM_W::new(self)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<6> {
        RESERVED6_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Enabling this bit will make the RIS.TBTORIS interrupt propagate to MIS.TBTOMIS"]
    #[inline(always)]
    #[must_use]
    pub fn tbtoim(&mut self) -> TBTOIM_W<8> {
        TBTOIM_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Enabling this bit will make the RIS.CBMRIS interrupt propagate to MIS.CBMMIS"]
    #[inline(always)]
    #[must_use]
    pub fn cbmim(&mut self) -> CBMIM_W<9> {
        CBMIM_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
Enabling this bit will make the RIS.CBERIS interrupt propagate to MIS.CBEMIS"]
    #[inline(always)]
    #[must_use]
    pub fn cbeim(&mut self) -> CBEIM_W<10> {
        CBEIM_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
Enabling this bit will make the RIS.TBMRIS interrupt propagate to MIS.TBMMIS"]
    #[inline(always)]
    #[must_use]
    pub fn tbmim(&mut self) -> TBMIM_W<11> {
        TBMIM_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> RESERVED12_W<12> {
        RESERVED12_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
Enabling this bit will make the RIS.DMABRIS interrupt propagate to MIS.DMABMIS"]
    #[inline(always)]
    #[must_use]
    pub fn dmabim(&mut self) -> DMABIM_W<13> {
        DMABIM_W::new(self)
    }
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved14(&mut self) -> RESERVED14_W<14> {
        RESERVED14_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Mask This register is used to enable the interrupts. Associated registers: RIS, MIS, ICLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imr::W](W) writer structure"]
impl crate::Writable for IMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
