#[doc = "Register `TA2IV` reader"]
pub struct R(crate::R<TA2IV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TA2IV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TA2IV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TA2IV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TA2IV` writer"]
pub struct W(crate::W<TA2IV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TA2IV_SPEC>;
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
impl From<crate::W<TA2IV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TA2IV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAIV` reader - TimerA interrupt vector value"]
pub type TAIV_R = crate::FieldReader<u16, TAIV_A>;
#[doc = "TimerA interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum TAIV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: Interrupt Source: Capture/compare 1; Interrupt Flag: TAxCCR1 CCIFG; Interrupt Priority: Highest"]
    TACCR1 = 2,
    #[doc = "4: Interrupt Source: Capture/compare 2; Interrupt Flag: TAxCCR2 CCIFG"]
    TACCR2 = 4,
    #[doc = "6: Interrupt Source: Capture/compare 3; Interrupt Flag: TAxCCR3 CCIFG"]
    TACCR3 = 6,
    #[doc = "8: Interrupt Source: Capture/compare 4; Interrupt Flag: TAxCCR4 CCIFG"]
    TACCR4 = 8,
    #[doc = "10: Interrupt Source: Capture/compare 5; Interrupt Flag: TAxCCR5 CCIFG"]
    TACCR5 = 10,
    #[doc = "12: Interrupt Source: Capture/compare 6; Interrupt Flag: TAxCCR6 CCIFG"]
    TACCR6 = 12,
    #[doc = "14: Interrupt Source: Timer overflow; Interrupt Flag: TAxCTL TAIFG; Interrupt Priority: Lowest"]
    TAIFG = 14,
}
impl From<TAIV_A> for u16 {
    #[inline(always)]
    fn from(variant: TAIV_A) -> Self {
        variant as _
    }
}
impl TAIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TAIV_A> {
        match self.bits {
            0 => Some(TAIV_A::NONE),
            2 => Some(TAIV_A::TACCR1),
            4 => Some(TAIV_A::TACCR2),
            6 => Some(TAIV_A::TACCR3),
            8 => Some(TAIV_A::TACCR4),
            10 => Some(TAIV_A::TACCR5),
            12 => Some(TAIV_A::TACCR6),
            14 => Some(TAIV_A::TAIFG),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == TAIV_A::NONE
    }
    #[doc = "Checks if the value of the field is `TACCR1`"]
    #[inline(always)]
    pub fn is_taccr1(&self) -> bool {
        *self == TAIV_A::TACCR1
    }
    #[doc = "Checks if the value of the field is `TACCR2`"]
    #[inline(always)]
    pub fn is_taccr2(&self) -> bool {
        *self == TAIV_A::TACCR2
    }
    #[doc = "Checks if the value of the field is `TACCR3`"]
    #[inline(always)]
    pub fn is_taccr3(&self) -> bool {
        *self == TAIV_A::TACCR3
    }
    #[doc = "Checks if the value of the field is `TACCR4`"]
    #[inline(always)]
    pub fn is_taccr4(&self) -> bool {
        *self == TAIV_A::TACCR4
    }
    #[doc = "Checks if the value of the field is `TACCR5`"]
    #[inline(always)]
    pub fn is_taccr5(&self) -> bool {
        *self == TAIV_A::TACCR5
    }
    #[doc = "Checks if the value of the field is `TACCR6`"]
    #[inline(always)]
    pub fn is_taccr6(&self) -> bool {
        *self == TAIV_A::TACCR6
    }
    #[doc = "Checks if the value of the field is `TAIFG`"]
    #[inline(always)]
    pub fn is_taifg(&self) -> bool {
        *self == TAIV_A::TAIFG
    }
}
impl R {
    #[doc = "Bits 0:15 - TimerA interrupt vector value"]
    #[inline(always)]
    pub fn taiv(&self) -> TAIV_R {
        TAIV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TimerAx Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta2iv](index.html) module"]
pub struct TA2IV_SPEC;
impl crate::RegisterSpec for TA2IV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ta2iv::R](R) reader structure"]
impl crate::Readable for TA2IV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ta2iv::W](W) writer structure"]
impl crate::Writable for TA2IV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TA2IV to value 0"]
impl crate::Resettable for TA2IV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
