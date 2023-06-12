#[doc = "Register `P6IV` reader"]
pub struct R(crate::R<P6IV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P6IV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P6IV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P6IV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P6IV` writer"]
pub struct W(crate::W<P6IV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P6IV_SPEC>;
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
impl From<crate::W<P6IV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P6IV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P6IV` reader - Port 6 interrupt vector value"]
pub type P6IV_R = crate::FieldReader<u8, P6IV_A>;
#[doc = "Port 6 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P6IV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: Interrupt Source: Port 6.0 interrupt; Interrupt Flag: P6IFG0; Interrupt Priority: Highest"]
    P6IFG0 = 2,
    #[doc = "4: Interrupt Source: Port 6.1 interrupt; Interrupt Flag: P6IFG1"]
    P6IFG1 = 4,
    #[doc = "6: Interrupt Source: Port 6.2 interrupt; Interrupt Flag: P6IFG2"]
    P6IFG2 = 6,
    #[doc = "8: Interrupt Source: Port 6.3 interrupt; Interrupt Flag: P6IFG3"]
    P6IFG3 = 8,
    #[doc = "10: Interrupt Source: Port 6.4 interrupt; Interrupt Flag: P6IFG4"]
    P6IFG4 = 10,
    #[doc = "12: Interrupt Source: Port 6.5 interrupt; Interrupt Flag: P6IFG5"]
    P6IFG5 = 12,
    #[doc = "14: Interrupt Source: Port 6.6 interrupt; Interrupt Flag: P6IFG6"]
    P6IFG6 = 14,
    #[doc = "16: Interrupt Source: Port 6.7 interrupt; Interrupt Flag: P6IFG7; Interrupt Priority: Lowest"]
    P6IFG7 = 16,
}
impl From<P6IV_A> for u8 {
    #[inline(always)]
    fn from(variant: P6IV_A) -> Self {
        variant as _
    }
}
impl P6IV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<P6IV_A> {
        match self.bits {
            0 => Some(P6IV_A::NONE),
            2 => Some(P6IV_A::P6IFG0),
            4 => Some(P6IV_A::P6IFG1),
            6 => Some(P6IV_A::P6IFG2),
            8 => Some(P6IV_A::P6IFG3),
            10 => Some(P6IV_A::P6IFG4),
            12 => Some(P6IV_A::P6IFG5),
            14 => Some(P6IV_A::P6IFG6),
            16 => Some(P6IV_A::P6IFG7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == P6IV_A::NONE
    }
    #[doc = "Checks if the value of the field is `P6IFG0`"]
    #[inline(always)]
    pub fn is_p6ifg0(&self) -> bool {
        *self == P6IV_A::P6IFG0
    }
    #[doc = "Checks if the value of the field is `P6IFG1`"]
    #[inline(always)]
    pub fn is_p6ifg1(&self) -> bool {
        *self == P6IV_A::P6IFG1
    }
    #[doc = "Checks if the value of the field is `P6IFG2`"]
    #[inline(always)]
    pub fn is_p6ifg2(&self) -> bool {
        *self == P6IV_A::P6IFG2
    }
    #[doc = "Checks if the value of the field is `P6IFG3`"]
    #[inline(always)]
    pub fn is_p6ifg3(&self) -> bool {
        *self == P6IV_A::P6IFG3
    }
    #[doc = "Checks if the value of the field is `P6IFG4`"]
    #[inline(always)]
    pub fn is_p6ifg4(&self) -> bool {
        *self == P6IV_A::P6IFG4
    }
    #[doc = "Checks if the value of the field is `P6IFG5`"]
    #[inline(always)]
    pub fn is_p6ifg5(&self) -> bool {
        *self == P6IV_A::P6IFG5
    }
    #[doc = "Checks if the value of the field is `P6IFG6`"]
    #[inline(always)]
    pub fn is_p6ifg6(&self) -> bool {
        *self == P6IV_A::P6IFG6
    }
    #[doc = "Checks if the value of the field is `P6IFG7`"]
    #[inline(always)]
    pub fn is_p6ifg7(&self) -> bool {
        *self == P6IV_A::P6IFG7
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 6 interrupt vector value"]
    #[inline(always)]
    pub fn p6iv(&self) -> P6IV_R {
        P6IV_R::new((self.bits & 0x1f) as u8)
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
#[doc = "Port 6 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p6iv](index.html) module"]
pub struct P6IV_SPEC;
impl crate::RegisterSpec for P6IV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [p6iv::R](R) reader structure"]
impl crate::Readable for P6IV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p6iv::W](W) writer structure"]
impl crate::Writable for P6IV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P6IV to value 0"]
impl crate::Resettable for P6IV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
