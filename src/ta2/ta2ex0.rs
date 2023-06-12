#[doc = "Register `TA2EX0` reader"]
pub struct R(crate::R<TA2EX0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TA2EX0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TA2EX0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TA2EX0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TA2EX0` writer"]
pub struct W(crate::W<TA2EX0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TA2EX0_SPEC>;
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
impl From<crate::W<TA2EX0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TA2EX0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAIDEX` reader - Input divider expansion"]
pub type TAIDEX_R = crate::FieldReader<u8, TAIDEX_A>;
#[doc = "Input divider expansion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TAIDEX_A {
    #[doc = "0: Divide by 1"]
    _1 = 0,
    #[doc = "1: Divide by 2"]
    _2 = 1,
    #[doc = "2: Divide by 3"]
    _3 = 2,
    #[doc = "3: Divide by 4"]
    _4 = 3,
    #[doc = "4: Divide by 5"]
    _5 = 4,
    #[doc = "5: Divide by 6"]
    _6 = 5,
    #[doc = "6: Divide by 7"]
    _7 = 6,
    #[doc = "7: Divide by 8"]
    _8 = 7,
}
impl From<TAIDEX_A> for u8 {
    #[inline(always)]
    fn from(variant: TAIDEX_A) -> Self {
        variant as _
    }
}
impl TAIDEX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAIDEX_A {
        match self.bits {
            0 => TAIDEX_A::_1,
            1 => TAIDEX_A::_2,
            2 => TAIDEX_A::_3,
            3 => TAIDEX_A::_4,
            4 => TAIDEX_A::_5,
            5 => TAIDEX_A::_6,
            6 => TAIDEX_A::_7,
            7 => TAIDEX_A::_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TAIDEX_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == TAIDEX_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == TAIDEX_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == TAIDEX_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        *self == TAIDEX_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        *self == TAIDEX_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        *self == TAIDEX_A::_7
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == TAIDEX_A::_8
    }
}
#[doc = "Field `TAIDEX` writer - Input divider expansion"]
pub type TAIDEX_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, TA2EX0_SPEC, u8, TAIDEX_A, 3, O>;
impl<'a, const O: u8> TAIDEX_W<'a, O> {
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TAIDEX_A::_1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(TAIDEX_A::_2)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(TAIDEX_A::_3)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(TAIDEX_A::_4)
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(TAIDEX_A::_5)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(TAIDEX_A::_6)
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(TAIDEX_A::_7)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(TAIDEX_A::_8)
    }
}
impl R {
    #[doc = "Bits 0:2 - Input divider expansion"]
    #[inline(always)]
    pub fn taidex(&self) -> TAIDEX_R {
        TAIDEX_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Input divider expansion"]
    #[inline(always)]
    #[must_use]
    pub fn taidex(&mut self) -> TAIDEX_W<0> {
        TAIDEX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TimerAx Expansion 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta2ex0](index.html) module"]
pub struct TA2EX0_SPEC;
impl crate::RegisterSpec for TA2EX0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ta2ex0::R](R) reader structure"]
impl crate::Readable for TA2EX0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ta2ex0::W](W) writer structure"]
impl crate::Writable for TA2EX0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TA2EX0 to value 0"]
impl crate::Resettable for TA2EX0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
