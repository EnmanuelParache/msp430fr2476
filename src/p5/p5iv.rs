#[doc = "Register `P5IV` reader"]
pub struct R(crate::R<P5IV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P5IV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P5IV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P5IV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P5IV` writer"]
pub struct W(crate::W<P5IV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P5IV_SPEC>;
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
impl From<crate::W<P5IV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P5IV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P5IV` reader - Port 5 interrupt vector value"]
pub type P5IV_R = crate::FieldReader<u8, P5IV_A>;
#[doc = "Port 5 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P5IV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: Interrupt Source: Port 5.0 interrupt; Interrupt Flag: P5IFG0; Interrupt Priority: Highest"]
    P5IFG0 = 2,
    #[doc = "4: Interrupt Source: Port 5.1 interrupt; Interrupt Flag: P5IFG1"]
    P5IFG1 = 4,
    #[doc = "6: Interrupt Source: Port 5.2 interrupt; Interrupt Flag: P5IFG2"]
    P5IFG2 = 6,
    #[doc = "8: Interrupt Source: Port 5.3 interrupt; Interrupt Flag: P5IFG3"]
    P5IFG3 = 8,
    #[doc = "10: Interrupt Source: Port 5.4 interrupt; Interrupt Flag: P5IFG4"]
    P5IFG4 = 10,
    #[doc = "12: Interrupt Source: Port 5.5 interrupt; Interrupt Flag: P5IFG5"]
    P5IFG5 = 12,
    #[doc = "14: Interrupt Source: Port 5.6 interrupt; Interrupt Flag: P5IFG6"]
    P5IFG6 = 14,
    #[doc = "16: Interrupt Source: Port 5.7 interrupt; Interrupt Flag: P5IFG7; Interrupt Priority: Lowest"]
    P5IFG7 = 16,
}
impl From<P5IV_A> for u8 {
    #[inline(always)]
    fn from(variant: P5IV_A) -> Self {
        variant as _
    }
}
impl P5IV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<P5IV_A> {
        match self.bits {
            0 => Some(P5IV_A::NONE),
            2 => Some(P5IV_A::P5IFG0),
            4 => Some(P5IV_A::P5IFG1),
            6 => Some(P5IV_A::P5IFG2),
            8 => Some(P5IV_A::P5IFG3),
            10 => Some(P5IV_A::P5IFG4),
            12 => Some(P5IV_A::P5IFG5),
            14 => Some(P5IV_A::P5IFG6),
            16 => Some(P5IV_A::P5IFG7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == P5IV_A::NONE
    }
    #[doc = "Checks if the value of the field is `P5IFG0`"]
    #[inline(always)]
    pub fn is_p5ifg0(&self) -> bool {
        *self == P5IV_A::P5IFG0
    }
    #[doc = "Checks if the value of the field is `P5IFG1`"]
    #[inline(always)]
    pub fn is_p5ifg1(&self) -> bool {
        *self == P5IV_A::P5IFG1
    }
    #[doc = "Checks if the value of the field is `P5IFG2`"]
    #[inline(always)]
    pub fn is_p5ifg2(&self) -> bool {
        *self == P5IV_A::P5IFG2
    }
    #[doc = "Checks if the value of the field is `P5IFG3`"]
    #[inline(always)]
    pub fn is_p5ifg3(&self) -> bool {
        *self == P5IV_A::P5IFG3
    }
    #[doc = "Checks if the value of the field is `P5IFG4`"]
    #[inline(always)]
    pub fn is_p5ifg4(&self) -> bool {
        *self == P5IV_A::P5IFG4
    }
    #[doc = "Checks if the value of the field is `P5IFG5`"]
    #[inline(always)]
    pub fn is_p5ifg5(&self) -> bool {
        *self == P5IV_A::P5IFG5
    }
    #[doc = "Checks if the value of the field is `P5IFG6`"]
    #[inline(always)]
    pub fn is_p5ifg6(&self) -> bool {
        *self == P5IV_A::P5IFG6
    }
    #[doc = "Checks if the value of the field is `P5IFG7`"]
    #[inline(always)]
    pub fn is_p5ifg7(&self) -> bool {
        *self == P5IV_A::P5IFG7
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 5 interrupt vector value"]
    #[inline(always)]
    pub fn p5iv(&self) -> P5IV_R {
        P5IV_R::new((self.bits & 0x1f) as u8)
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
#[doc = "Port 5 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p5iv](index.html) module"]
pub struct P5IV_SPEC;
impl crate::RegisterSpec for P5IV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [p5iv::R](R) reader structure"]
impl crate::Readable for P5IV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p5iv::W](W) writer structure"]
impl crate::Writable for P5IV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P5IV to value 0"]
impl crate::Resettable for P5IV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
