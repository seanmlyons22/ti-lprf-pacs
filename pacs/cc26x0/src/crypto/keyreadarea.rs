#[doc = "Register `KEYREADAREA` reader"]
pub struct R(crate::R<KEYREADAREA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYREADAREA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYREADAREA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYREADAREA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEYREADAREA` writer"]
pub struct W(crate::W<KEYREADAREA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYREADAREA_SPEC>;
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
impl From<crate::W<KEYREADAREA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYREADAREA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAM_AREA` reader - 3:0\\]
Selects the area of the key store RAM from where the key needs to be read that will be written to the AES engine. Only RAM areas that contain valid written keys can be selected."]
pub type RAM_AREA_R = crate::FieldReader<u8, RAM_AREA_A>;
#[doc = "3:0\\]
Selects the area of the key store RAM from where the key needs to be read that will be written to the AES engine. Only RAM areas that contain valid written keys can be selected.\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RAM_AREA_A {
    #[doc = "8: No RAM"]
    NO_RAM = 8,
    #[doc = "7: RAM Area 7"]
    RAM_AREA7 = 7,
    #[doc = "6: RAM Area 6"]
    RAM_AREA6 = 6,
    #[doc = "5: RAM Area 5"]
    RAM_AREA5 = 5,
    #[doc = "4: RAM Area 4"]
    RAM_AREA4 = 4,
    #[doc = "3: RAM Area 3"]
    RAM_AREA3 = 3,
    #[doc = "2: RAM Area 2"]
    RAM_AREA2 = 2,
    #[doc = "1: RAM Area 1"]
    RAM_AREA1 = 1,
    #[doc = "0: RAM Area 0"]
    RAM_AREA0 = 0,
}
impl From<RAM_AREA_A> for u8 {
    #[inline(always)]
    fn from(variant: RAM_AREA_A) -> Self {
        variant as _
    }
}
impl RAM_AREA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RAM_AREA_A> {
        match self.bits {
            8 => Some(RAM_AREA_A::NO_RAM),
            7 => Some(RAM_AREA_A::RAM_AREA7),
            6 => Some(RAM_AREA_A::RAM_AREA6),
            5 => Some(RAM_AREA_A::RAM_AREA5),
            4 => Some(RAM_AREA_A::RAM_AREA4),
            3 => Some(RAM_AREA_A::RAM_AREA3),
            2 => Some(RAM_AREA_A::RAM_AREA2),
            1 => Some(RAM_AREA_A::RAM_AREA1),
            0 => Some(RAM_AREA_A::RAM_AREA0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_RAM`"]
    #[inline(always)]
    pub fn is_no_ram(&self) -> bool {
        *self == RAM_AREA_A::NO_RAM
    }
    #[doc = "Checks if the value of the field is `RAM_AREA7`"]
    #[inline(always)]
    pub fn is_ram_area7(&self) -> bool {
        *self == RAM_AREA_A::RAM_AREA7
    }
    #[doc = "Checks if the value of the field is `RAM_AREA6`"]
    #[inline(always)]
    pub fn is_ram_area6(&self) -> bool {
        *self == RAM_AREA_A::RAM_AREA6
    }
    #[doc = "Checks if the value of the field is `RAM_AREA5`"]
    #[inline(always)]
    pub fn is_ram_area5(&self) -> bool {
        *self == RAM_AREA_A::RAM_AREA5
    }
    #[doc = "Checks if the value of the field is `RAM_AREA4`"]
    #[inline(always)]
    pub fn is_ram_area4(&self) -> bool {
        *self == RAM_AREA_A::RAM_AREA4
    }
    #[doc = "Checks if the value of the field is `RAM_AREA3`"]
    #[inline(always)]
    pub fn is_ram_area3(&self) -> bool {
        *self == RAM_AREA_A::RAM_AREA3
    }
    #[doc = "Checks if the value of the field is `RAM_AREA2`"]
    #[inline(always)]
    pub fn is_ram_area2(&self) -> bool {
        *self == RAM_AREA_A::RAM_AREA2
    }
    #[doc = "Checks if the value of the field is `RAM_AREA1`"]
    #[inline(always)]
    pub fn is_ram_area1(&self) -> bool {
        *self == RAM_AREA_A::RAM_AREA1
    }
    #[doc = "Checks if the value of the field is `RAM_AREA0`"]
    #[inline(always)]
    pub fn is_ram_area0(&self) -> bool {
        *self == RAM_AREA_A::RAM_AREA0
    }
}
#[doc = "Field `RAM_AREA` writer - 3:0\\]
Selects the area of the key store RAM from where the key needs to be read that will be written to the AES engine. Only RAM areas that contain valid written keys can be selected."]
pub type RAM_AREA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, KEYREADAREA_SPEC, u8, RAM_AREA_A, 4, O>;
impl<'a, const O: u8> RAM_AREA_W<'a, O> {
    #[doc = "No RAM"]
    #[inline(always)]
    pub fn no_ram(self) -> &'a mut W {
        self.variant(RAM_AREA_A::NO_RAM)
    }
    #[doc = "RAM Area 7"]
    #[inline(always)]
    pub fn ram_area7(self) -> &'a mut W {
        self.variant(RAM_AREA_A::RAM_AREA7)
    }
    #[doc = "RAM Area 6"]
    #[inline(always)]
    pub fn ram_area6(self) -> &'a mut W {
        self.variant(RAM_AREA_A::RAM_AREA6)
    }
    #[doc = "RAM Area 5"]
    #[inline(always)]
    pub fn ram_area5(self) -> &'a mut W {
        self.variant(RAM_AREA_A::RAM_AREA5)
    }
    #[doc = "RAM Area 4"]
    #[inline(always)]
    pub fn ram_area4(self) -> &'a mut W {
        self.variant(RAM_AREA_A::RAM_AREA4)
    }
    #[doc = "RAM Area 3"]
    #[inline(always)]
    pub fn ram_area3(self) -> &'a mut W {
        self.variant(RAM_AREA_A::RAM_AREA3)
    }
    #[doc = "RAM Area 2"]
    #[inline(always)]
    pub fn ram_area2(self) -> &'a mut W {
        self.variant(RAM_AREA_A::RAM_AREA2)
    }
    #[doc = "RAM Area 1"]
    #[inline(always)]
    pub fn ram_area1(self) -> &'a mut W {
        self.variant(RAM_AREA_A::RAM_AREA1)
    }
    #[doc = "RAM Area 0"]
    #[inline(always)]
    pub fn ram_area0(self) -> &'a mut W {
        self.variant(RAM_AREA_A::RAM_AREA0)
    }
}
#[doc = "Field `RESERVED4` reader - 30:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED4` writer - 30:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, KEYREADAREA_SPEC, u32, u32, 27, O>;
#[doc = "Field `BUSY` reader - 31:31\\]
Key store operation busy status flag (read only) 0: operation is completed. 1: operation is not completed and the key store is busy."]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` writer - 31:31\\]
Key store operation busy status flag (read only) 0: operation is completed. 1: operation is not completed and the key store is busy."]
pub type BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, KEYREADAREA_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Selects the area of the key store RAM from where the key needs to be read that will be written to the AES engine. Only RAM areas that contain valid written keys can be selected."]
    #[inline(always)]
    pub fn ram_area(&self) -> RAM_AREA_R {
        RAM_AREA_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:30 - 30:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new((self.bits >> 4) & 0x07ff_ffff)
    }
    #[doc = "Bit 31 - 31:31\\]
Key store operation busy status flag (read only) 0: operation is completed. 1: operation is not completed and the key store is busy."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Selects the area of the key store RAM from where the key needs to be read that will be written to the AES engine. Only RAM areas that contain valid written keys can be selected."]
    #[inline(always)]
    #[must_use]
    pub fn ram_area(&mut self) -> RAM_AREA_W<0> {
        RAM_AREA_W::new(self)
    }
    #[doc = "Bits 4:30 - 30:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> RESERVED4_W<4> {
        RESERVED4_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
Key store operation busy status flag (read only) 0: operation is completed. 1: operation is not completed and the key store is busy."]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BUSY_W<31> {
        BUSY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key Read Area\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyreadarea](index.html) module"]
pub struct KEYREADAREA_SPEC;
impl crate::RegisterSpec for KEYREADAREA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keyreadarea::R](R) reader structure"]
impl crate::Readable for KEYREADAREA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keyreadarea::W](W) writer structure"]
impl crate::Writable for KEYREADAREA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEYREADAREA to value 0x08"]
impl crate::Resettable for KEYREADAREA_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
