#[doc = "Register `TA3R` reader"]
pub struct R(crate::R<TA3R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TA3R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TA3R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TA3R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TA3R` writer"]
pub struct W(crate::W<TA3R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TA3R_SPEC>;
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
impl From<crate::W<TA3R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TA3R_SPEC>) -> Self {
        W(writer)
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
#[doc = "TimerA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta3r](index.html) module"]
pub struct TA3R_SPEC;
impl crate::RegisterSpec for TA3R_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ta3r::R](R) reader structure"]
impl crate::Readable for TA3R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ta3r::W](W) writer structure"]
impl crate::Writable for TA3R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TA3R to value 0"]
impl crate::Resettable for TA3R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
