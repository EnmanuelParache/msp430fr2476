#[doc = "Register `SYSJMBC` reader"]
pub struct R(crate::R<SYSJMBC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSJMBC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSJMBC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSJMBC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSJMBC` writer"]
pub struct W(crate::W<SYSJMBC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSJMBC_SPEC>;
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
impl From<crate::W<SYSJMBC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSJMBC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `JMBIN0FG` reader - Incoming JTAG Mailbox 0 flag"]
pub type JMBIN0FG_R = crate::BitReader<JMBIN0FG_A>;
#[doc = "Incoming JTAG Mailbox 0 flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JMBIN0FG_A {
    #[doc = "0: JMBI0 has no new data"]
    NODAT = 0,
    #[doc = "1: JMBI0 has new data available"]
    NEWDAT = 1,
}
impl From<JMBIN0FG_A> for bool {
    #[inline(always)]
    fn from(variant: JMBIN0FG_A) -> Self {
        variant as u8 != 0
    }
}
impl JMBIN0FG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JMBIN0FG_A {
        match self.bits {
            false => JMBIN0FG_A::NODAT,
            true => JMBIN0FG_A::NEWDAT,
        }
    }
    #[doc = "Checks if the value of the field is `NODAT`"]
    #[inline(always)]
    pub fn is_nodat(&self) -> bool {
        *self == JMBIN0FG_A::NODAT
    }
    #[doc = "Checks if the value of the field is `NEWDAT`"]
    #[inline(always)]
    pub fn is_newdat(&self) -> bool {
        *self == JMBIN0FG_A::NEWDAT
    }
}
#[doc = "Field `JMBIN0FG` writer - Incoming JTAG Mailbox 0 flag"]
pub type JMBIN0FG_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSJMBC_SPEC, JMBIN0FG_A, O>;
impl<'a, const O: u8> JMBIN0FG_W<'a, O> {
    #[doc = "JMBI0 has no new data"]
    #[inline(always)]
    pub fn nodat(self) -> &'a mut W {
        self.variant(JMBIN0FG_A::NODAT)
    }
    #[doc = "JMBI0 has new data available"]
    #[inline(always)]
    pub fn newdat(self) -> &'a mut W {
        self.variant(JMBIN0FG_A::NEWDAT)
    }
}
#[doc = "Field `JMBIN1FG` reader - Incoming JTAG Mailbox 1 flag"]
pub type JMBIN1FG_R = crate::BitReader<JMBIN1FG_A>;
#[doc = "Incoming JTAG Mailbox 1 flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JMBIN1FG_A {
    #[doc = "0: JMBI1 has no new data"]
    NODAT = 0,
    #[doc = "1: JMBI1 has new data available"]
    NEWDAT = 1,
}
impl From<JMBIN1FG_A> for bool {
    #[inline(always)]
    fn from(variant: JMBIN1FG_A) -> Self {
        variant as u8 != 0
    }
}
impl JMBIN1FG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JMBIN1FG_A {
        match self.bits {
            false => JMBIN1FG_A::NODAT,
            true => JMBIN1FG_A::NEWDAT,
        }
    }
    #[doc = "Checks if the value of the field is `NODAT`"]
    #[inline(always)]
    pub fn is_nodat(&self) -> bool {
        *self == JMBIN1FG_A::NODAT
    }
    #[doc = "Checks if the value of the field is `NEWDAT`"]
    #[inline(always)]
    pub fn is_newdat(&self) -> bool {
        *self == JMBIN1FG_A::NEWDAT
    }
}
#[doc = "Field `JMBIN1FG` writer - Incoming JTAG Mailbox 1 flag"]
pub type JMBIN1FG_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSJMBC_SPEC, JMBIN1FG_A, O>;
impl<'a, const O: u8> JMBIN1FG_W<'a, O> {
    #[doc = "JMBI1 has no new data"]
    #[inline(always)]
    pub fn nodat(self) -> &'a mut W {
        self.variant(JMBIN1FG_A::NODAT)
    }
    #[doc = "JMBI1 has new data available"]
    #[inline(always)]
    pub fn newdat(self) -> &'a mut W {
        self.variant(JMBIN1FG_A::NEWDAT)
    }
}
#[doc = "Field `JMBOUT0FG` reader - Outgoing JTAG Mailbox 0 flag"]
pub type JMBOUT0FG_R = crate::BitReader<JMBOUT0FG_A>;
#[doc = "Outgoing JTAG Mailbox 0 flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JMBOUT0FG_A {
    #[doc = "0: JMBO0 is not ready to receive new data"]
    BUSY = 0,
    #[doc = "1: JMBO0 is ready to receive new data"]
    READY = 1,
}
impl From<JMBOUT0FG_A> for bool {
    #[inline(always)]
    fn from(variant: JMBOUT0FG_A) -> Self {
        variant as u8 != 0
    }
}
impl JMBOUT0FG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JMBOUT0FG_A {
        match self.bits {
            false => JMBOUT0FG_A::BUSY,
            true => JMBOUT0FG_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == JMBOUT0FG_A::BUSY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == JMBOUT0FG_A::READY
    }
}
#[doc = "Field `JMBOUT1FG` reader - Outgoing JTAG Mailbox 1 flag"]
pub type JMBOUT1FG_R = crate::BitReader<JMBOUT1FG_A>;
#[doc = "Outgoing JTAG Mailbox 1 flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JMBOUT1FG_A {
    #[doc = "0: JMBO1 is not ready to receive new data"]
    BUSY = 0,
    #[doc = "1: JMBO1 is ready to receive new data"]
    READY = 1,
}
impl From<JMBOUT1FG_A> for bool {
    #[inline(always)]
    fn from(variant: JMBOUT1FG_A) -> Self {
        variant as u8 != 0
    }
}
impl JMBOUT1FG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JMBOUT1FG_A {
        match self.bits {
            false => JMBOUT1FG_A::BUSY,
            true => JMBOUT1FG_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == JMBOUT1FG_A::BUSY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == JMBOUT1FG_A::READY
    }
}
#[doc = "Field `JMBMODE` reader - Operation mode of JMB"]
pub type JMBMODE_R = crate::BitReader<JMBMODE_A>;
#[doc = "Operation mode of JMB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JMBMODE_A {
    #[doc = "0: 16-bit transfers using JMBO0 and JMBI0 only"]
    _16BIT = 0,
    #[doc = "1: 32-bit transfers using JMBO0 with JMBO1 and JMBI0 with JMBI1"]
    _32BIT = 1,
}
impl From<JMBMODE_A> for bool {
    #[inline(always)]
    fn from(variant: JMBMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl JMBMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JMBMODE_A {
        match self.bits {
            false => JMBMODE_A::_16BIT,
            true => JMBMODE_A::_32BIT,
        }
    }
    #[doc = "Checks if the value of the field is `_16BIT`"]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        *self == JMBMODE_A::_16BIT
    }
    #[doc = "Checks if the value of the field is `_32BIT`"]
    #[inline(always)]
    pub fn is_32bit(&self) -> bool {
        *self == JMBMODE_A::_32BIT
    }
}
#[doc = "Field `JMBMODE` writer - Operation mode of JMB"]
pub type JMBMODE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSJMBC_SPEC, JMBMODE_A, O>;
impl<'a, const O: u8> JMBMODE_W<'a, O> {
    #[doc = "16-bit transfers using JMBO0 and JMBI0 only"]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut W {
        self.variant(JMBMODE_A::_16BIT)
    }
    #[doc = "32-bit transfers using JMBO0 with JMBO1 and JMBI0 with JMBI1"]
    #[inline(always)]
    pub fn _32bit(self) -> &'a mut W {
        self.variant(JMBMODE_A::_32BIT)
    }
}
#[doc = "Field `JMBCLR0OFF` reader - Incoming JTAG Mailbox 0 flag auto-clear disable"]
pub type JMBCLR0OFF_R = crate::BitReader<JMBCLR0OFF_A>;
#[doc = "Incoming JTAG Mailbox 0 flag auto-clear disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JMBCLR0OFF_A {
    #[doc = "0: JMBIN0FG cleared on read of JMB0IN register"]
    CLRORD = 0,
    #[doc = "1: JMBIN0FG cleared by software"]
    CLRBSW = 1,
}
impl From<JMBCLR0OFF_A> for bool {
    #[inline(always)]
    fn from(variant: JMBCLR0OFF_A) -> Self {
        variant as u8 != 0
    }
}
impl JMBCLR0OFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JMBCLR0OFF_A {
        match self.bits {
            false => JMBCLR0OFF_A::CLRORD,
            true => JMBCLR0OFF_A::CLRBSW,
        }
    }
    #[doc = "Checks if the value of the field is `CLRORD`"]
    #[inline(always)]
    pub fn is_clrord(&self) -> bool {
        *self == JMBCLR0OFF_A::CLRORD
    }
    #[doc = "Checks if the value of the field is `CLRBSW`"]
    #[inline(always)]
    pub fn is_clrbsw(&self) -> bool {
        *self == JMBCLR0OFF_A::CLRBSW
    }
}
#[doc = "Field `JMBCLR0OFF` writer - Incoming JTAG Mailbox 0 flag auto-clear disable"]
pub type JMBCLR0OFF_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSJMBC_SPEC, JMBCLR0OFF_A, O>;
impl<'a, const O: u8> JMBCLR0OFF_W<'a, O> {
    #[doc = "JMBIN0FG cleared on read of JMB0IN register"]
    #[inline(always)]
    pub fn clrord(self) -> &'a mut W {
        self.variant(JMBCLR0OFF_A::CLRORD)
    }
    #[doc = "JMBIN0FG cleared by software"]
    #[inline(always)]
    pub fn clrbsw(self) -> &'a mut W {
        self.variant(JMBCLR0OFF_A::CLRBSW)
    }
}
#[doc = "Field `JMBCLR1OFF` reader - Incoming JTAG Mailbox 1 flag auto-clear disable"]
pub type JMBCLR1OFF_R = crate::BitReader<JMBCLR1OFF_A>;
#[doc = "Incoming JTAG Mailbox 1 flag auto-clear disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JMBCLR1OFF_A {
    #[doc = "0: JMBIN1FG cleared on read of JMB1IN register"]
    CLRORD = 0,
    #[doc = "1: JMBIN1FG cleared by software"]
    CLRBSW = 1,
}
impl From<JMBCLR1OFF_A> for bool {
    #[inline(always)]
    fn from(variant: JMBCLR1OFF_A) -> Self {
        variant as u8 != 0
    }
}
impl JMBCLR1OFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JMBCLR1OFF_A {
        match self.bits {
            false => JMBCLR1OFF_A::CLRORD,
            true => JMBCLR1OFF_A::CLRBSW,
        }
    }
    #[doc = "Checks if the value of the field is `CLRORD`"]
    #[inline(always)]
    pub fn is_clrord(&self) -> bool {
        *self == JMBCLR1OFF_A::CLRORD
    }
    #[doc = "Checks if the value of the field is `CLRBSW`"]
    #[inline(always)]
    pub fn is_clrbsw(&self) -> bool {
        *self == JMBCLR1OFF_A::CLRBSW
    }
}
#[doc = "Field `JMBCLR1OFF` writer - Incoming JTAG Mailbox 1 flag auto-clear disable"]
pub type JMBCLR1OFF_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSJMBC_SPEC, JMBCLR1OFF_A, O>;
impl<'a, const O: u8> JMBCLR1OFF_W<'a, O> {
    #[doc = "JMBIN1FG cleared on read of JMB1IN register"]
    #[inline(always)]
    pub fn clrord(self) -> &'a mut W {
        self.variant(JMBCLR1OFF_A::CLRORD)
    }
    #[doc = "JMBIN1FG cleared by software"]
    #[inline(always)]
    pub fn clrbsw(self) -> &'a mut W {
        self.variant(JMBCLR1OFF_A::CLRBSW)
    }
}
impl R {
    #[doc = "Bit 0 - Incoming JTAG Mailbox 0 flag"]
    #[inline(always)]
    pub fn jmbin0fg(&self) -> JMBIN0FG_R {
        JMBIN0FG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Incoming JTAG Mailbox 1 flag"]
    #[inline(always)]
    pub fn jmbin1fg(&self) -> JMBIN1FG_R {
        JMBIN1FG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Outgoing JTAG Mailbox 0 flag"]
    #[inline(always)]
    pub fn jmbout0fg(&self) -> JMBOUT0FG_R {
        JMBOUT0FG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Outgoing JTAG Mailbox 1 flag"]
    #[inline(always)]
    pub fn jmbout1fg(&self) -> JMBOUT1FG_R {
        JMBOUT1FG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Operation mode of JMB"]
    #[inline(always)]
    pub fn jmbmode(&self) -> JMBMODE_R {
        JMBMODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Incoming JTAG Mailbox 0 flag auto-clear disable"]
    #[inline(always)]
    pub fn jmbclr0off(&self) -> JMBCLR0OFF_R {
        JMBCLR0OFF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Incoming JTAG Mailbox 1 flag auto-clear disable"]
    #[inline(always)]
    pub fn jmbclr1off(&self) -> JMBCLR1OFF_R {
        JMBCLR1OFF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Incoming JTAG Mailbox 0 flag"]
    #[inline(always)]
    #[must_use]
    pub fn jmbin0fg(&mut self) -> JMBIN0FG_W<0> {
        JMBIN0FG_W::new(self)
    }
    #[doc = "Bit 1 - Incoming JTAG Mailbox 1 flag"]
    #[inline(always)]
    #[must_use]
    pub fn jmbin1fg(&mut self) -> JMBIN1FG_W<1> {
        JMBIN1FG_W::new(self)
    }
    #[doc = "Bit 4 - Operation mode of JMB"]
    #[inline(always)]
    #[must_use]
    pub fn jmbmode(&mut self) -> JMBMODE_W<4> {
        JMBMODE_W::new(self)
    }
    #[doc = "Bit 6 - Incoming JTAG Mailbox 0 flag auto-clear disable"]
    #[inline(always)]
    #[must_use]
    pub fn jmbclr0off(&mut self) -> JMBCLR0OFF_W<6> {
        JMBCLR0OFF_W::new(self)
    }
    #[doc = "Bit 7 - Incoming JTAG Mailbox 1 flag auto-clear disable"]
    #[inline(always)]
    #[must_use]
    pub fn jmbclr1off(&mut self) -> JMBCLR1OFF_W<7> {
        JMBCLR1OFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JTAG Mailbox Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysjmbc](index.html) module"]
pub struct SYSJMBC_SPEC;
impl crate::RegisterSpec for SYSJMBC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sysjmbc::R](R) reader structure"]
impl crate::Readable for SYSJMBC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysjmbc::W](W) writer structure"]
impl crate::Writable for SYSJMBC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSJMBC to value 0"]
impl crate::Resettable for SYSJMBC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
