#[doc = "Register `INT_CTL` reader"]
pub type R = crate::R<IntCtlSpec>;
#[doc = "Register `INT_CTL` writer"]
pub type W = crate::W<IntCtlSpec>;
#[doc = "INT pin assertion polarity:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntPol {
    #[doc = "1: Assert low"]
    B1 = 1,
    #[doc = "0: Assert low"]
    B0 = 0,
}
impl From<IntPol> for bool {
    #[inline(always)]
    fn from(variant: IntPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_POL` reader - INT pin assertion polarity:"]
pub type IntPolR = crate::BitReader<IntPol>;
impl IntPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntPol {
        match self.bits {
            true => IntPol::B1,
            false => IntPol::B0,
        }
    }
    #[doc = "Assert low"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntPol::B1
    }
    #[doc = "Assert low"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntPol::B0
    }
}
#[doc = "Field `INT_POL` writer - INT pin assertion polarity:"]
pub type IntPolW<'a, REG> = crate::BitWriter<'a, REG, IntPol>;
impl<'a, REG> IntPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Assert low"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntPol::B1)
    }
    #[doc = "Assert low"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntPol::B0)
    }
}
#[doc = "Set Software Interrupt:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SoftIntCtrl {
    #[doc = "1: Do not set interrupt,"]
    B1 = 1,
    #[doc = "0: Do not set interrupt,"]
    B0 = 0,
}
impl From<SoftIntCtrl> for bool {
    #[inline(always)]
    fn from(variant: SoftIntCtrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOFT_INT_CTRL` reader - Set Software Interrupt:"]
pub type SoftIntCtrlR = crate::BitReader<SoftIntCtrl>;
impl SoftIntCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SoftIntCtrl {
        match self.bits {
            true => SoftIntCtrl::B1,
            false => SoftIntCtrl::B0,
        }
    }
    #[doc = "Do not set interrupt,"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SoftIntCtrl::B1
    }
    #[doc = "Do not set interrupt,"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SoftIntCtrl::B0
    }
}
#[doc = "Field `SOFT_INT_CTRL` writer - Set Software Interrupt:"]
pub type SoftIntCtrlW<'a, REG> = crate::BitWriter1C<'a, REG, SoftIntCtrl>;
impl<'a, REG> SoftIntCtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not set interrupt,"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SoftIntCtrl::B1)
    }
    #[doc = "Do not set interrupt,"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SoftIntCtrl::B0)
    }
}
#[doc = "Field `SERDES_UNDERFLOW_CLEAR` reader - 1: clear SerDes FIFO underflow flag"]
pub type SerdesUnderflowClearR = crate::BitReader;
#[doc = "Field `SERDES_UNDERFLOW_CLEAR` writer - 1: clear SerDes FIFO underflow flag"]
pub type SerdesUnderflowClearW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SERDES_OVERFLOW_CLEAR` reader - 1: clear SerDes FIFO overflow flag"]
pub type SerdesOverflowClearR = crate::BitReader;
#[doc = "Field `SERDES_OVERFLOW_CLEAR` writer - 1: clear SerDes FIFO overflow flag"]
pub type SerdesOverflowClearW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - INT pin assertion polarity:"]
    #[inline(always)]
    pub fn int_pol(&self) -> IntPolR {
        IntPolR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Set Software Interrupt:"]
    #[inline(always)]
    pub fn soft_int_ctrl(&self) -> SoftIntCtrlR {
        SoftIntCtrlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: clear SerDes FIFO underflow flag"]
    #[inline(always)]
    pub fn serdes_underflow_clear(&self) -> SerdesUnderflowClearR {
        SerdesUnderflowClearR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: clear SerDes FIFO overflow flag"]
    #[inline(always)]
    pub fn serdes_overflow_clear(&self) -> SerdesOverflowClearR {
        SerdesOverflowClearR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - INT pin assertion polarity:"]
    #[inline(always)]
    #[must_use]
    pub fn int_pol(&mut self) -> IntPolW<IntCtlSpec> {
        IntPolW::new(self, 0)
    }
    #[doc = "Bit 2 - Set Software Interrupt:"]
    #[inline(always)]
    #[must_use]
    pub fn soft_int_ctrl(&mut self) -> SoftIntCtrlW<IntCtlSpec> {
        SoftIntCtrlW::new(self, 2)
    }
    #[doc = "Bit 4 - 1: clear SerDes FIFO underflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn serdes_underflow_clear(&mut self) -> SerdesUnderflowClearW<IntCtlSpec> {
        SerdesUnderflowClearW::new(self, 4)
    }
    #[doc = "Bit 5 - 1: clear SerDes FIFO overflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn serdes_overflow_clear(&mut self) -> SerdesOverflowClearW<IntCtlSpec> {
        SerdesOverflowClearW::new(self, 5)
    }
}
#[doc = "Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntCtlSpec;
impl crate::RegisterSpec for IntCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ctl::R`](R) reader structure"]
impl crate::Readable for IntCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`int_ctl::W`](W) writer structure"]
impl crate::Writable for IntCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x34;
}
#[doc = "`reset()` method sets INT_CTL to value 0"]
impl crate::Resettable for IntCtlSpec {
    const RESET_VALUE: u32 = 0;
}
