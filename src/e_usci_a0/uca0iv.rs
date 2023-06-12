#[doc = "Register `UCA0IV` reader"]
pub struct R(crate::R<UCA0IV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA0IV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCA0IV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCA0IV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA0IV` writer"]
pub struct W(crate::W<UCA0IV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA0IV_SPEC>;
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
impl From<crate::W<UCA0IV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCA0IV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCIV` reader - eUSCI_A interrupt vector value"]
pub type UCIV_R = crate::FieldReader<u16, UCIV_A>;
#[doc = "eUSCI_A interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum UCIV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: Interrupt Source: Receive buffer full; Interrupt Flag: UCRXIFG; Interrupt Priority: Highest"]
    UCRXIFG = 2,
    #[doc = "4: Interrupt Source: Transmit buffer empty; Interrupt Flag: UCTXIFG"]
    UCTXIFG = 4,
    #[doc = "6: Interrupt Source: Start bit received; Interrupt Flag: UCSTTIFG"]
    UCSTTIFG = 6,
    #[doc = "8: Interrupt Source: Transmit complete; Interrupt Flag: UCTXCPTIFG; Interrupt Priority: Lowest"]
    UCTXCPTIFG = 8,
}
impl From<UCIV_A> for u16 {
    #[inline(always)]
    fn from(variant: UCIV_A) -> Self {
        variant as _
    }
}
impl UCIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UCIV_A> {
        match self.bits {
            0 => Some(UCIV_A::NONE),
            2 => Some(UCIV_A::UCRXIFG),
            4 => Some(UCIV_A::UCTXIFG),
            6 => Some(UCIV_A::UCSTTIFG),
            8 => Some(UCIV_A::UCTXCPTIFG),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == UCIV_A::NONE
    }
    #[doc = "Checks if the value of the field is `UCRXIFG`"]
    #[inline(always)]
    pub fn is_ucrxifg(&self) -> bool {
        *self == UCIV_A::UCRXIFG
    }
    #[doc = "Checks if the value of the field is `UCTXIFG`"]
    #[inline(always)]
    pub fn is_uctxifg(&self) -> bool {
        *self == UCIV_A::UCTXIFG
    }
    #[doc = "Checks if the value of the field is `UCSTTIFG`"]
    #[inline(always)]
    pub fn is_ucsttifg(&self) -> bool {
        *self == UCIV_A::UCSTTIFG
    }
    #[doc = "Checks if the value of the field is `UCTXCPTIFG`"]
    #[inline(always)]
    pub fn is_uctxcptifg(&self) -> bool {
        *self == UCIV_A::UCTXCPTIFG
    }
}
impl R {
    #[doc = "Bits 0:15 - eUSCI_A interrupt vector value"]
    #[inline(always)]
    pub fn uciv(&self) -> UCIV_R {
        UCIV_R::new(self.bits)
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
#[doc = "eUSCI_Ax Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca0iv](index.html) module"]
pub struct UCA0IV_SPEC;
impl crate::RegisterSpec for UCA0IV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uca0iv::R](R) reader structure"]
impl crate::Readable for UCA0IV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca0iv::W](W) writer structure"]
impl crate::Writable for UCA0IV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCA0IV to value 0"]
impl crate::Resettable for UCA0IV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
