#[doc = "Register `TA3CTL` reader"]
pub struct R(crate::R<TA3CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TA3CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TA3CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TA3CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TA3CTL` writer"]
pub struct W(crate::W<TA3CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TA3CTL_SPEC>;
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
impl From<crate::W<TA3CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TA3CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAIFG` reader - TimerA interrupt flag"]
pub type TAIFG_R = crate::BitReader<TAIFG_A>;
#[doc = "TimerA interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAIFG_A {
    #[doc = "0: No interrupt pending"]
    TAIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    TAIFG_1 = 1,
}
impl From<TAIFG_A> for bool {
    #[inline(always)]
    fn from(variant: TAIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl TAIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAIFG_A {
        match self.bits {
            false => TAIFG_A::TAIFG_0,
            true => TAIFG_A::TAIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `TAIFG_0`"]
    #[inline(always)]
    pub fn is_taifg_0(&self) -> bool {
        *self == TAIFG_A::TAIFG_0
    }
    #[doc = "Checks if the value of the field is `TAIFG_1`"]
    #[inline(always)]
    pub fn is_taifg_1(&self) -> bool {
        *self == TAIFG_A::TAIFG_1
    }
}
#[doc = "Field `TAIFG` writer - TimerA interrupt flag"]
pub type TAIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, TA3CTL_SPEC, TAIFG_A, O>;
impl<'a, const O: u8> TAIFG_W<'a, O> {
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn taifg_0(self) -> &'a mut W {
        self.variant(TAIFG_A::TAIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn taifg_1(self) -> &'a mut W {
        self.variant(TAIFG_A::TAIFG_1)
    }
}
#[doc = "Field `TAIE` reader - TimerA interrupt enable"]
pub type TAIE_R = crate::BitReader<TAIE_A>;
#[doc = "TimerA interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAIE_A {
    #[doc = "0: Interrupt disabled"]
    TAIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    TAIE_1 = 1,
}
impl From<TAIE_A> for bool {
    #[inline(always)]
    fn from(variant: TAIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TAIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAIE_A {
        match self.bits {
            false => TAIE_A::TAIE_0,
            true => TAIE_A::TAIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TAIE_0`"]
    #[inline(always)]
    pub fn is_taie_0(&self) -> bool {
        *self == TAIE_A::TAIE_0
    }
    #[doc = "Checks if the value of the field is `TAIE_1`"]
    #[inline(always)]
    pub fn is_taie_1(&self) -> bool {
        *self == TAIE_A::TAIE_1
    }
}
#[doc = "Field `TAIE` writer - TimerA interrupt enable"]
pub type TAIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, TA3CTL_SPEC, TAIE_A, O>;
impl<'a, const O: u8> TAIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn taie_0(self) -> &'a mut W {
        self.variant(TAIE_A::TAIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn taie_1(self) -> &'a mut W {
        self.variant(TAIE_A::TAIE_1)
    }
}
#[doc = "Field `TACLR` reader - TimerA clear"]
pub type TACLR_R = crate::BitReader<bool>;
#[doc = "Field `TACLR` writer - TimerA clear"]
pub type TACLR_W<'a, const O: u8> = crate::BitWriter<'a, u16, TA3CTL_SPEC, bool, O>;
#[doc = "Field `MC` reader - Mode control"]
pub type MC_R = crate::FieldReader<u8, MC_A>;
#[doc = "Mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MC_A {
    #[doc = "0: Stop mode: Timer is halted"]
    STOP = 0,
    #[doc = "1: Up mode: Timer counts up to TAxCCR0"]
    UP = 1,
    #[doc = "2: Continuous mode: Timer counts up to 0FFFFh"]
    CONTINUOUS = 2,
    #[doc = "3: Up/down mode: Timer counts up to TAxCCR0 then down to 0000h"]
    UPDOWN = 3,
}
impl From<MC_A> for u8 {
    #[inline(always)]
    fn from(variant: MC_A) -> Self {
        variant as _
    }
}
impl MC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MC_A {
        match self.bits {
            0 => MC_A::STOP,
            1 => MC_A::UP,
            2 => MC_A::CONTINUOUS,
            3 => MC_A::UPDOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == MC_A::STOP
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == MC_A::UP
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == MC_A::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `UPDOWN`"]
    #[inline(always)]
    pub fn is_updown(&self) -> bool {
        *self == MC_A::UPDOWN
    }
}
#[doc = "Field `MC` writer - Mode control"]
pub type MC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, TA3CTL_SPEC, u8, MC_A, 2, O>;
impl<'a, const O: u8> MC_W<'a, O> {
    #[doc = "Stop mode: Timer is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(MC_A::STOP)
    }
    #[doc = "Up mode: Timer counts up to TAxCCR0"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(MC_A::UP)
    }
    #[doc = "Continuous mode: Timer counts up to 0FFFFh"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(MC_A::CONTINUOUS)
    }
    #[doc = "Up/down mode: Timer counts up to TAxCCR0 then down to 0000h"]
    #[inline(always)]
    pub fn updown(self) -> &'a mut W {
        self.variant(MC_A::UPDOWN)
    }
}
#[doc = "Field `ID` reader - Input divider"]
pub type ID_R = crate::FieldReader<u8, ID_A>;
#[doc = "Input divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ID_A {
    #[doc = "0: /1"]
    _1 = 0,
    #[doc = "1: /2"]
    _2 = 1,
    #[doc = "2: /4"]
    _4 = 2,
    #[doc = "3: /8"]
    _8 = 3,
}
impl From<ID_A> for u8 {
    #[inline(always)]
    fn from(variant: ID_A) -> Self {
        variant as _
    }
}
impl ID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ID_A {
        match self.bits {
            0 => ID_A::_1,
            1 => ID_A::_2,
            2 => ID_A::_4,
            3 => ID_A::_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ID_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == ID_A::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == ID_A::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == ID_A::_8
    }
}
#[doc = "Field `ID` writer - Input divider"]
pub type ID_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, TA3CTL_SPEC, u8, ID_A, 2, O>;
impl<'a, const O: u8> ID_W<'a, O> {
    #[doc = "/1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ID_A::_1)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(ID_A::_2)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(ID_A::_4)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(ID_A::_8)
    }
}
#[doc = "Field `TASSEL` reader - TimerA clock source select"]
pub type TASSEL_R = crate::FieldReader<u8, TASSEL_A>;
#[doc = "TimerA clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TASSEL_A {
    #[doc = "0: TAxCLK"]
    TACLK = 0,
    #[doc = "1: ACLK"]
    ACLK = 1,
    #[doc = "2: SMCLK"]
    SMCLK = 2,
    #[doc = "3: INCLK"]
    INCLK = 3,
}
impl From<TASSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TASSEL_A) -> Self {
        variant as _
    }
}
impl TASSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TASSEL_A {
        match self.bits {
            0 => TASSEL_A::TACLK,
            1 => TASSEL_A::ACLK,
            2 => TASSEL_A::SMCLK,
            3 => TASSEL_A::INCLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TACLK`"]
    #[inline(always)]
    pub fn is_taclk(&self) -> bool {
        *self == TASSEL_A::TACLK
    }
    #[doc = "Checks if the value of the field is `ACLK`"]
    #[inline(always)]
    pub fn is_aclk(&self) -> bool {
        *self == TASSEL_A::ACLK
    }
    #[doc = "Checks if the value of the field is `SMCLK`"]
    #[inline(always)]
    pub fn is_smclk(&self) -> bool {
        *self == TASSEL_A::SMCLK
    }
    #[doc = "Checks if the value of the field is `INCLK`"]
    #[inline(always)]
    pub fn is_inclk(&self) -> bool {
        *self == TASSEL_A::INCLK
    }
}
#[doc = "Field `TASSEL` writer - TimerA clock source select"]
pub type TASSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, TA3CTL_SPEC, u8, TASSEL_A, 2, O>;
impl<'a, const O: u8> TASSEL_W<'a, O> {
    #[doc = "TAxCLK"]
    #[inline(always)]
    pub fn taclk(self) -> &'a mut W {
        self.variant(TASSEL_A::TACLK)
    }
    #[doc = "ACLK"]
    #[inline(always)]
    pub fn aclk(self) -> &'a mut W {
        self.variant(TASSEL_A::ACLK)
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn smclk(self) -> &'a mut W {
        self.variant(TASSEL_A::SMCLK)
    }
    #[doc = "INCLK"]
    #[inline(always)]
    pub fn inclk(self) -> &'a mut W {
        self.variant(TASSEL_A::INCLK)
    }
}
impl R {
    #[doc = "Bit 0 - TimerA interrupt flag"]
    #[inline(always)]
    pub fn taifg(&self) -> TAIFG_R {
        TAIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TimerA interrupt enable"]
    #[inline(always)]
    pub fn taie(&self) -> TAIE_R {
        TAIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TimerA clear"]
    #[inline(always)]
    pub fn taclr(&self) -> TACLR_R {
        TACLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Mode control"]
    #[inline(always)]
    pub fn mc(&self) -> MC_R {
        MC_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Input divider"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - TimerA clock source select"]
    #[inline(always)]
    pub fn tassel(&self) -> TASSEL_R {
        TASSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TimerA interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn taifg(&mut self) -> TAIFG_W<0> {
        TAIFG_W::new(self)
    }
    #[doc = "Bit 1 - TimerA interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn taie(&mut self) -> TAIE_W<1> {
        TAIE_W::new(self)
    }
    #[doc = "Bit 2 - TimerA clear"]
    #[inline(always)]
    #[must_use]
    pub fn taclr(&mut self) -> TACLR_W<2> {
        TACLR_W::new(self)
    }
    #[doc = "Bits 4:5 - Mode control"]
    #[inline(always)]
    #[must_use]
    pub fn mc(&mut self) -> MC_W<4> {
        MC_W::new(self)
    }
    #[doc = "Bits 6:7 - Input divider"]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> ID_W<6> {
        ID_W::new(self)
    }
    #[doc = "Bits 8:9 - TimerA clock source select"]
    #[inline(always)]
    #[must_use]
    pub fn tassel(&mut self) -> TASSEL_W<8> {
        TASSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TimerAx Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta3ctl](index.html) module"]
pub struct TA3CTL_SPEC;
impl crate::RegisterSpec for TA3CTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ta3ctl::R](R) reader structure"]
impl crate::Readable for TA3CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ta3ctl::W](W) writer structure"]
impl crate::Writable for TA3CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TA3CTL to value 0"]
impl crate::Resettable for TA3CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
