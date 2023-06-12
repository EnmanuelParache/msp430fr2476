#[doc = "Register `SYSCFG3` reader"]
pub struct R(crate::R<SYSCFG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCFG3` writer"]
pub struct W(crate::W<SYSCFG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCFG3_SPEC>;
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
impl From<crate::W<SYSCFG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCFG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USCIA0RMP` reader - eUSCI_A0 remapping source selection"]
pub type USCIA0RMP_R = crate::BitReader<USCIA0RMP_A>;
#[doc = "eUSCI_A0 remapping source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USCIA0RMP_A {
    #[doc = "0: Default function. See the device-specific data sheet for details."]
    USCIA0RMP_0 = 0,
    #[doc = "1: Remapped function. See the device-specific data sheet for details."]
    USCIA0RMP_1 = 1,
}
impl From<USCIA0RMP_A> for bool {
    #[inline(always)]
    fn from(variant: USCIA0RMP_A) -> Self {
        variant as u8 != 0
    }
}
impl USCIA0RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USCIA0RMP_A {
        match self.bits {
            false => USCIA0RMP_A::USCIA0RMP_0,
            true => USCIA0RMP_A::USCIA0RMP_1,
        }
    }
    #[doc = "Checks if the value of the field is `USCIA0RMP_0`"]
    #[inline(always)]
    pub fn is_uscia0rmp_0(&self) -> bool {
        *self == USCIA0RMP_A::USCIA0RMP_0
    }
    #[doc = "Checks if the value of the field is `USCIA0RMP_1`"]
    #[inline(always)]
    pub fn is_uscia0rmp_1(&self) -> bool {
        *self == USCIA0RMP_A::USCIA0RMP_1
    }
}
#[doc = "Field `USCIA0RMP` writer - eUSCI_A0 remapping source selection"]
pub type USCIA0RMP_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG3_SPEC, USCIA0RMP_A, O>;
impl<'a, const O: u8> USCIA0RMP_W<'a, O> {
    #[doc = "Default function. See the device-specific data sheet for details."]
    #[inline(always)]
    pub fn uscia0rmp_0(self) -> &'a mut W {
        self.variant(USCIA0RMP_A::USCIA0RMP_0)
    }
    #[doc = "Remapped function. See the device-specific data sheet for details."]
    #[inline(always)]
    pub fn uscia0rmp_1(self) -> &'a mut W {
        self.variant(USCIA0RMP_A::USCIA0RMP_1)
    }
}
#[doc = "Field `TA2RMP` reader - Timer2_A3 remapping source selection"]
pub type TA2RMP_R = crate::BitReader<TA2RMP_A>;
#[doc = "Timer2_A3 remapping source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TA2RMP_A {
    #[doc = "0: Default function. See the device-specific data sheet for details."]
    TA2RMP_0 = 0,
    #[doc = "1: Remapped function. See the device-specific data sheet for details."]
    TA2RMP_1 = 1,
}
impl From<TA2RMP_A> for bool {
    #[inline(always)]
    fn from(variant: TA2RMP_A) -> Self {
        variant as u8 != 0
    }
}
impl TA2RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TA2RMP_A {
        match self.bits {
            false => TA2RMP_A::TA2RMP_0,
            true => TA2RMP_A::TA2RMP_1,
        }
    }
    #[doc = "Checks if the value of the field is `TA2RMP_0`"]
    #[inline(always)]
    pub fn is_ta2rmp_0(&self) -> bool {
        *self == TA2RMP_A::TA2RMP_0
    }
    #[doc = "Checks if the value of the field is `TA2RMP_1`"]
    #[inline(always)]
    pub fn is_ta2rmp_1(&self) -> bool {
        *self == TA2RMP_A::TA2RMP_1
    }
}
#[doc = "Field `TA2RMP` writer - Timer2_A3 remapping source selection"]
pub type TA2RMP_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG3_SPEC, TA2RMP_A, O>;
impl<'a, const O: u8> TA2RMP_W<'a, O> {
    #[doc = "Default function. See the device-specific data sheet for details."]
    #[inline(always)]
    pub fn ta2rmp_0(self) -> &'a mut W {
        self.variant(TA2RMP_A::TA2RMP_0)
    }
    #[doc = "Remapped function. See the device-specific data sheet for details."]
    #[inline(always)]
    pub fn ta2rmp_1(self) -> &'a mut W {
        self.variant(TA2RMP_A::TA2RMP_1)
    }
}
#[doc = "Field `TA3RMP` reader - Timer3_A3 remapping source selection"]
pub type TA3RMP_R = crate::BitReader<TA3RMP_A>;
#[doc = "Timer3_A3 remapping source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TA3RMP_A {
    #[doc = "0: Default function. See the device-specific data sheet for details."]
    TA3RMP_0 = 0,
    #[doc = "1: Remapped function. See the device-specific data sheet for details."]
    TA3RMP_1 = 1,
}
impl From<TA3RMP_A> for bool {
    #[inline(always)]
    fn from(variant: TA3RMP_A) -> Self {
        variant as u8 != 0
    }
}
impl TA3RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TA3RMP_A {
        match self.bits {
            false => TA3RMP_A::TA3RMP_0,
            true => TA3RMP_A::TA3RMP_1,
        }
    }
    #[doc = "Checks if the value of the field is `TA3RMP_0`"]
    #[inline(always)]
    pub fn is_ta3rmp_0(&self) -> bool {
        *self == TA3RMP_A::TA3RMP_0
    }
    #[doc = "Checks if the value of the field is `TA3RMP_1`"]
    #[inline(always)]
    pub fn is_ta3rmp_1(&self) -> bool {
        *self == TA3RMP_A::TA3RMP_1
    }
}
#[doc = "Field `TA3RMP` writer - Timer3_A3 remapping source selection"]
pub type TA3RMP_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG3_SPEC, TA3RMP_A, O>;
impl<'a, const O: u8> TA3RMP_W<'a, O> {
    #[doc = "Default function. See the device-specific data sheet for details."]
    #[inline(always)]
    pub fn ta3rmp_0(self) -> &'a mut W {
        self.variant(TA3RMP_A::TA3RMP_0)
    }
    #[doc = "Remapped function. See the device-specific data sheet for details."]
    #[inline(always)]
    pub fn ta3rmp_1(self) -> &'a mut W {
        self.variant(TA3RMP_A::TA3RMP_1)
    }
}
#[doc = "Field `USCIB1RMP` reader - eUSCI_B1 remapping source selection"]
pub type USCIB1RMP_R = crate::BitReader<USCIB1RMP_A>;
#[doc = "eUSCI_B1 remapping source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USCIB1RMP_A {
    #[doc = "0: Default function. See the device-specific data sheet for details."]
    USCIB1RMP_0 = 0,
    #[doc = "1: Remapped function. See the device-specific data sheet for details."]
    USCIB1RMP_1 = 1,
}
impl From<USCIB1RMP_A> for bool {
    #[inline(always)]
    fn from(variant: USCIB1RMP_A) -> Self {
        variant as u8 != 0
    }
}
impl USCIB1RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USCIB1RMP_A {
        match self.bits {
            false => USCIB1RMP_A::USCIB1RMP_0,
            true => USCIB1RMP_A::USCIB1RMP_1,
        }
    }
    #[doc = "Checks if the value of the field is `USCIB1RMP_0`"]
    #[inline(always)]
    pub fn is_uscib1rmp_0(&self) -> bool {
        *self == USCIB1RMP_A::USCIB1RMP_0
    }
    #[doc = "Checks if the value of the field is `USCIB1RMP_1`"]
    #[inline(always)]
    pub fn is_uscib1rmp_1(&self) -> bool {
        *self == USCIB1RMP_A::USCIB1RMP_1
    }
}
#[doc = "Field `USCIB1RMP` writer - eUSCI_B1 remapping source selection"]
pub type USCIB1RMP_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG3_SPEC, USCIB1RMP_A, O>;
impl<'a, const O: u8> USCIB1RMP_W<'a, O> {
    #[doc = "Default function. See the device-specific data sheet for details."]
    #[inline(always)]
    pub fn uscib1rmp_0(self) -> &'a mut W {
        self.variant(USCIB1RMP_A::USCIB1RMP_0)
    }
    #[doc = "Remapped function. See the device-specific data sheet for details."]
    #[inline(always)]
    pub fn uscib1rmp_1(self) -> &'a mut W {
        self.variant(USCIB1RMP_A::USCIB1RMP_1)
    }
}
impl R {
    #[doc = "Bit 0 - eUSCI_A0 remapping source selection"]
    #[inline(always)]
    pub fn uscia0rmp(&self) -> USCIA0RMP_R {
        USCIA0RMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Timer2_A3 remapping source selection"]
    #[inline(always)]
    pub fn ta2rmp(&self) -> TA2RMP_R {
        TA2RMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer3_A3 remapping source selection"]
    #[inline(always)]
    pub fn ta3rmp(&self) -> TA3RMP_R {
        TA3RMP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - eUSCI_B1 remapping source selection"]
    #[inline(always)]
    pub fn uscib1rmp(&self) -> USCIB1RMP_R {
        USCIB1RMP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - eUSCI_A0 remapping source selection"]
    #[inline(always)]
    #[must_use]
    pub fn uscia0rmp(&mut self) -> USCIA0RMP_W<0> {
        USCIA0RMP_W::new(self)
    }
    #[doc = "Bit 2 - Timer2_A3 remapping source selection"]
    #[inline(always)]
    #[must_use]
    pub fn ta2rmp(&mut self) -> TA2RMP_W<2> {
        TA2RMP_W::new(self)
    }
    #[doc = "Bit 3 - Timer3_A3 remapping source selection"]
    #[inline(always)]
    #[must_use]
    pub fn ta3rmp(&mut self) -> TA3RMP_W<3> {
        TA3RMP_W::new(self)
    }
    #[doc = "Bit 4 - eUSCI_B1 remapping source selection"]
    #[inline(always)]
    #[must_use]
    pub fn uscib1rmp(&mut self) -> USCIB1RMP_W<4> {
        USCIB1RMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Configuration 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg3](index.html) module"]
pub struct SYSCFG3_SPEC;
impl crate::RegisterSpec for SYSCFG3_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [syscfg3::R](R) reader structure"]
impl crate::Readable for SYSCFG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syscfg3::W](W) writer structure"]
impl crate::Writable for SYSCFG3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSCFG3 to value 0"]
impl crate::Resettable for SYSCFG3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
