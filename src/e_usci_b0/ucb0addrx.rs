#[doc = "Register `UCB0ADDRX` reader"]
pub struct R(crate::R<UCB0ADDRX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB0ADDRX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCB0ADDRX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCB0ADDRX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB0ADDRX` writer"]
pub struct W(crate::W<UCB0ADDRX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB0ADDRX_SPEC>;
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
impl From<crate::W<UCB0ADDRX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCB0ADDRX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRX` reader - Received Address Register"]
pub type ADDRX_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Received Address Register"]
    #[inline(always)]
    pub fn addrx(&self) -> ADDRX_R {
        ADDRX_R::new(self.bits & 0x03ff)
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
#[doc = "eUSCI_Bx I2C Received Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0addrx](index.html) module"]
pub struct UCB0ADDRX_SPEC;
impl crate::RegisterSpec for UCB0ADDRX_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucb0addrx::R](R) reader structure"]
impl crate::Readable for UCB0ADDRX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb0addrx::W](W) writer structure"]
impl crate::Writable for UCB0ADDRX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCB0ADDRX to value 0"]
impl crate::Resettable for UCB0ADDRX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
