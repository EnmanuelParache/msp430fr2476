#[doc = "Register `PJIN` reader"]
pub struct R(crate::R<PJIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PJIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PJIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PJIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PJIN` writer"]
pub struct W(crate::W<PJIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PJIN_SPEC>;
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
impl From<crate::W<PJIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PJIN_SPEC>) -> Self {
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
#[doc = "Port J Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pjin](index.html) module"]
pub struct PJIN_SPEC;
impl crate::RegisterSpec for PJIN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pjin::R](R) reader structure"]
impl crate::Readable for PJIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pjin::W](W) writer structure"]
impl crate::Writable for PJIN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PJIN to value 0"]
impl crate::Resettable for PJIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
