#[doc = "Register `SYSBSLC` reader"]
pub struct R(crate::R<SYSBSLC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSBSLC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSBSLC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSBSLC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSBSLC` writer"]
pub struct W(crate::W<SYSBSLC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSBSLC_SPEC>;
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
impl From<crate::W<SYSBSLC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSBSLC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSBSLR` reader - RAM assigned to BSL"]
pub type SYSBSLR_R = crate::BitReader<SYSBSLR_A>;
#[doc = "RAM assigned to BSL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSBSLR_A {
    #[doc = "0: No RAM assigned to BSL area"]
    NORAM = 0,
    #[doc = "1: Lowest 16 bytes of RAM assigned to BSL"]
    RAM = 1,
}
impl From<SYSBSLR_A> for bool {
    #[inline(always)]
    fn from(variant: SYSBSLR_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSBSLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSBSLR_A {
        match self.bits {
            false => SYSBSLR_A::NORAM,
            true => SYSBSLR_A::RAM,
        }
    }
    #[doc = "Checks if the value of the field is `NORAM`"]
    #[inline(always)]
    pub fn is_noram(&self) -> bool {
        *self == SYSBSLR_A::NORAM
    }
    #[doc = "Checks if the value of the field is `RAM`"]
    #[inline(always)]
    pub fn is_ram(&self) -> bool {
        *self == SYSBSLR_A::RAM
    }
}
#[doc = "Field `SYSBSLR` writer - RAM assigned to BSL"]
pub type SYSBSLR_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSBSLC_SPEC, SYSBSLR_A, O>;
impl<'a, const O: u8> SYSBSLR_W<'a, O> {
    #[doc = "No RAM assigned to BSL area"]
    #[inline(always)]
    pub fn noram(self) -> &'a mut W {
        self.variant(SYSBSLR_A::NORAM)
    }
    #[doc = "Lowest 16 bytes of RAM assigned to BSL"]
    #[inline(always)]
    pub fn ram(self) -> &'a mut W {
        self.variant(SYSBSLR_A::RAM)
    }
}
#[doc = "Field `SYSBSLOFF` reader - Bootstrap loader memory disable for the size covered in SYSBSLSIZE"]
pub type SYSBSLOFF_R = crate::BitReader<SYSBSLOFF_A>;
#[doc = "Bootstrap loader memory disable for the size covered in SYSBSLSIZE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSBSLOFF_A {
    #[doc = "0: BSL memory is addressed when this area is read."]
    ON = 0,
    #[doc = "1: BSL memory behaves like vacant memory. Reads cause 3FFFh to be read. Fetches cause JMP $ to be executed."]
    OFF = 1,
}
impl From<SYSBSLOFF_A> for bool {
    #[inline(always)]
    fn from(variant: SYSBSLOFF_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSBSLOFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSBSLOFF_A {
        match self.bits {
            false => SYSBSLOFF_A::ON,
            true => SYSBSLOFF_A::OFF,
        }
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == SYSBSLOFF_A::ON
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == SYSBSLOFF_A::OFF
    }
}
#[doc = "Field `SYSBSLOFF` writer - Bootstrap loader memory disable for the size covered in SYSBSLSIZE"]
pub type SYSBSLOFF_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSBSLC_SPEC, SYSBSLOFF_A, O>;
impl<'a, const O: u8> SYSBSLOFF_W<'a, O> {
    #[doc = "BSL memory is addressed when this area is read."]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(SYSBSLOFF_A::ON)
    }
    #[doc = "BSL memory behaves like vacant memory. Reads cause 3FFFh to be read. Fetches cause JMP $ to be executed."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(SYSBSLOFF_A::OFF)
    }
}
#[doc = "Field `SYSBSLPE` reader - Bootstrap loader memory protection enable for the size covered in SYSBSLSIZE. By default, this bit is cleared by hardware with a BOR event (as indicated above); however, the boot code that checks for an available BSL may set this bit in software to protect the BSL. Because devices normally come with a TI BSL preprogrammed and protected, the boot code sets this bit."]
pub type SYSBSLPE_R = crate::BitReader<SYSBSLPE_A>;
#[doc = "Bootstrap loader memory protection enable for the size covered in SYSBSLSIZE. By default, this bit is cleared by hardware with a BOR event (as indicated above); however, the boot code that checks for an available BSL may set this bit in software to protect the BSL. Because devices normally come with a TI BSL preprogrammed and protected, the boot code sets this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSBSLPE_A {
    #[doc = "0: Area not protected. Read, program, and erase of BSL memory is possible."]
    NOTPROT = 0,
    #[doc = "1: Area protected"]
    PROT = 1,
}
impl From<SYSBSLPE_A> for bool {
    #[inline(always)]
    fn from(variant: SYSBSLPE_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSBSLPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSBSLPE_A {
        match self.bits {
            false => SYSBSLPE_A::NOTPROT,
            true => SYSBSLPE_A::PROT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPROT`"]
    #[inline(always)]
    pub fn is_notprot(&self) -> bool {
        *self == SYSBSLPE_A::NOTPROT
    }
    #[doc = "Checks if the value of the field is `PROT`"]
    #[inline(always)]
    pub fn is_prot(&self) -> bool {
        *self == SYSBSLPE_A::PROT
    }
}
#[doc = "Field `SYSBSLPE` writer - Bootstrap loader memory protection enable for the size covered in SYSBSLSIZE. By default, this bit is cleared by hardware with a BOR event (as indicated above); however, the boot code that checks for an available BSL may set this bit in software to protect the BSL. Because devices normally come with a TI BSL preprogrammed and protected, the boot code sets this bit."]
pub type SYSBSLPE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSBSLC_SPEC, SYSBSLPE_A, O>;
impl<'a, const O: u8> SYSBSLPE_W<'a, O> {
    #[doc = "Area not protected. Read, program, and erase of BSL memory is possible."]
    #[inline(always)]
    pub fn notprot(self) -> &'a mut W {
        self.variant(SYSBSLPE_A::NOTPROT)
    }
    #[doc = "Area protected"]
    #[inline(always)]
    pub fn prot(self) -> &'a mut W {
        self.variant(SYSBSLPE_A::PROT)
    }
}
impl R {
    #[doc = "Bit 2 - RAM assigned to BSL"]
    #[inline(always)]
    pub fn sysbslr(&self) -> SYSBSLR_R {
        SYSBSLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 14 - Bootstrap loader memory disable for the size covered in SYSBSLSIZE"]
    #[inline(always)]
    pub fn sysbsloff(&self) -> SYSBSLOFF_R {
        SYSBSLOFF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Bootstrap loader memory protection enable for the size covered in SYSBSLSIZE. By default, this bit is cleared by hardware with a BOR event (as indicated above); however, the boot code that checks for an available BSL may set this bit in software to protect the BSL. Because devices normally come with a TI BSL preprogrammed and protected, the boot code sets this bit."]
    #[inline(always)]
    pub fn sysbslpe(&self) -> SYSBSLPE_R {
        SYSBSLPE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - RAM assigned to BSL"]
    #[inline(always)]
    #[must_use]
    pub fn sysbslr(&mut self) -> SYSBSLR_W<2> {
        SYSBSLR_W::new(self)
    }
    #[doc = "Bit 14 - Bootstrap loader memory disable for the size covered in SYSBSLSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn sysbsloff(&mut self) -> SYSBSLOFF_W<14> {
        SYSBSLOFF_W::new(self)
    }
    #[doc = "Bit 15 - Bootstrap loader memory protection enable for the size covered in SYSBSLSIZE. By default, this bit is cleared by hardware with a BOR event (as indicated above); however, the boot code that checks for an available BSL may set this bit in software to protect the BSL. Because devices normally come with a TI BSL preprogrammed and protected, the boot code sets this bit."]
    #[inline(always)]
    #[must_use]
    pub fn sysbslpe(&mut self) -> SYSBSLPE_W<15> {
        SYSBSLPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bootloader Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysbslc](index.html) module"]
pub struct SYSBSLC_SPEC;
impl crate::RegisterSpec for SYSBSLC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sysbslc::R](R) reader structure"]
impl crate::Readable for SYSBSLC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysbslc::W](W) writer structure"]
impl crate::Writable for SYSBSLC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSBSLC to value 0"]
impl crate::Resettable for SYSBSLC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
