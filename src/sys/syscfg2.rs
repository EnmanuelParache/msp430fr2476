#[doc = "Register `SYSCFG2` reader"]
pub struct R(crate::R<SYSCFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCFG2` writer"]
pub struct W(crate::W<SYSCFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCFG2_SPEC>;
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
impl From<crate::W<SYSCFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTCCKSEL` reader - RTC clock selection"]
pub type RTCCKSEL_R = crate::BitReader<RTCCKSEL_A>;
#[doc = "RTC clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCCKSEL_A {
    #[doc = "0: SMCLK is selected"]
    RTCCKSEL_0 = 0,
    #[doc = "1: ACLK is selected"]
    RTCCKSEL_1 = 1,
}
impl From<RTCCKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: RTCCKSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl RTCCKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCCKSEL_A {
        match self.bits {
            false => RTCCKSEL_A::RTCCKSEL_0,
            true => RTCCKSEL_A::RTCCKSEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTCCKSEL_0`"]
    #[inline(always)]
    pub fn is_rtccksel_0(&self) -> bool {
        *self == RTCCKSEL_A::RTCCKSEL_0
    }
    #[doc = "Checks if the value of the field is `RTCCKSEL_1`"]
    #[inline(always)]
    pub fn is_rtccksel_1(&self) -> bool {
        *self == RTCCKSEL_A::RTCCKSEL_1
    }
}
#[doc = "Field `RTCCKSEL` writer - RTC clock selection"]
pub type RTCCKSEL_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG2_SPEC, RTCCKSEL_A, O>;
impl<'a, const O: u8> RTCCKSEL_W<'a, O> {
    #[doc = "SMCLK is selected"]
    #[inline(always)]
    pub fn rtccksel_0(self) -> &'a mut W {
        self.variant(RTCCKSEL_A::RTCCKSEL_0)
    }
    #[doc = "ACLK is selected"]
    #[inline(always)]
    pub fn rtccksel_1(self) -> &'a mut W {
        self.variant(RTCCKSEL_A::RTCCKSEL_1)
    }
}
#[doc = "Field `USCIB0RMP` reader - eUSCI_B0 remapping source selection"]
pub type USCIB0RMP_R = crate::BitReader<USCIB0RMP_A>;
#[doc = "eUSCI_B0 remapping source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USCIB0RMP_A {
    #[doc = "0: Default function. See the device-specific data sheet for details."]
    USCIB0RMP_0 = 0,
    #[doc = "1: Remapped function. See the device-specific data sheet for details."]
    USCIB0RMP_1 = 1,
}
impl From<USCIB0RMP_A> for bool {
    #[inline(always)]
    fn from(variant: USCIB0RMP_A) -> Self {
        variant as u8 != 0
    }
}
impl USCIB0RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USCIB0RMP_A {
        match self.bits {
            false => USCIB0RMP_A::USCIB0RMP_0,
            true => USCIB0RMP_A::USCIB0RMP_1,
        }
    }
    #[doc = "Checks if the value of the field is `USCIB0RMP_0`"]
    #[inline(always)]
    pub fn is_uscib0rmp_0(&self) -> bool {
        *self == USCIB0RMP_A::USCIB0RMP_0
    }
    #[doc = "Checks if the value of the field is `USCIB0RMP_1`"]
    #[inline(always)]
    pub fn is_uscib0rmp_1(&self) -> bool {
        *self == USCIB0RMP_A::USCIB0RMP_1
    }
}
#[doc = "Field `USCIB0RMP` writer - eUSCI_B0 remapping source selection"]
pub type USCIB0RMP_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG2_SPEC, USCIB0RMP_A, O>;
impl<'a, const O: u8> USCIB0RMP_W<'a, O> {
    #[doc = "Default function. See the device-specific data sheet for details."]
    #[inline(always)]
    pub fn uscib0rmp_0(self) -> &'a mut W {
        self.variant(USCIB0RMP_A::USCIB0RMP_0)
    }
    #[doc = "Remapped function. See the device-specific data sheet for details."]
    #[inline(always)]
    pub fn uscib0rmp_1(self) -> &'a mut W {
        self.variant(USCIB0RMP_A::USCIB0RMP_1)
    }
}
#[doc = "Field `TB0TRGSEL` reader - TB0OUTH trigger source selection"]
pub type TB0TRGSEL_R = crate::BitReader<TB0TRGSEL_A>;
#[doc = "TB0OUTH trigger source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TB0TRGSEL_A {
    #[doc = "0: Internal source is selected"]
    TB0TRGSEL_0 = 0,
    #[doc = "1: External source is selected"]
    TB0TRGSEL_1 = 1,
}
impl From<TB0TRGSEL_A> for bool {
    #[inline(always)]
    fn from(variant: TB0TRGSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl TB0TRGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TB0TRGSEL_A {
        match self.bits {
            false => TB0TRGSEL_A::TB0TRGSEL_0,
            true => TB0TRGSEL_A::TB0TRGSEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `TB0TRGSEL_0`"]
    #[inline(always)]
    pub fn is_tb0trgsel_0(&self) -> bool {
        *self == TB0TRGSEL_A::TB0TRGSEL_0
    }
    #[doc = "Checks if the value of the field is `TB0TRGSEL_1`"]
    #[inline(always)]
    pub fn is_tb0trgsel_1(&self) -> bool {
        *self == TB0TRGSEL_A::TB0TRGSEL_1
    }
}
#[doc = "Field `TB0TRGSEL` writer - TB0OUTH trigger source selection"]
pub type TB0TRGSEL_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG2_SPEC, TB0TRGSEL_A, O>;
impl<'a, const O: u8> TB0TRGSEL_W<'a, O> {
    #[doc = "Internal source is selected"]
    #[inline(always)]
    pub fn tb0trgsel_0(self) -> &'a mut W {
        self.variant(TB0TRGSEL_A::TB0TRGSEL_0)
    }
    #[doc = "External source is selected"]
    #[inline(always)]
    pub fn tb0trgsel_1(self) -> &'a mut W {
        self.variant(TB0TRGSEL_A::TB0TRGSEL_1)
    }
}
impl R {
    #[doc = "Bit 10 - RTC clock selection"]
    #[inline(always)]
    pub fn rtccksel(&self) -> RTCCKSEL_R {
        RTCCKSEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - eUSCI_B0 remapping source selection"]
    #[inline(always)]
    pub fn uscib0rmp(&self) -> USCIB0RMP_R {
        USCIB0RMP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - TB0OUTH trigger source selection"]
    #[inline(always)]
    pub fn tb0trgsel(&self) -> TB0TRGSEL_R {
        TB0TRGSEL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - RTC clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn rtccksel(&mut self) -> RTCCKSEL_W<10> {
        RTCCKSEL_W::new(self)
    }
    #[doc = "Bit 11 - eUSCI_B0 remapping source selection"]
    #[inline(always)]
    #[must_use]
    pub fn uscib0rmp(&mut self) -> USCIB0RMP_W<11> {
        USCIB0RMP_W::new(self)
    }
    #[doc = "Bit 15 - TB0OUTH trigger source selection"]
    #[inline(always)]
    #[must_use]
    pub fn tb0trgsel(&mut self) -> TB0TRGSEL_W<15> {
        TB0TRGSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Configuration 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg2](index.html) module"]
pub struct SYSCFG2_SPEC;
impl crate::RegisterSpec for SYSCFG2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [syscfg2::R](R) reader structure"]
impl crate::Readable for SYSCFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syscfg2::W](W) writer structure"]
impl crate::Writable for SYSCFG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSCFG2 to value 0"]
impl crate::Resettable for SYSCFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
