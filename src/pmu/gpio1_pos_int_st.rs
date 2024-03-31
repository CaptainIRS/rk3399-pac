#[doc = "Register `GPIO1_POS_INT_ST` reader"]
pub type R = crate::R<Gpio1PosIntStSpec>;
#[doc = "Register `GPIO1_POS_INT_ST` writer"]
pub type W = crate::W<Gpio1PosIntStSpec>;
#[doc = "gpio1a posedge pulse interrupt status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1aPosIntStatus {
    #[doc = "0: not wakeup by gpio1a posedge pulse"]
    B0 = 0,
    #[doc = "1: wakeup by gpio1a posedge pulse"]
    B1 = 1,
}
impl From<Gpio1aPosIntStatus> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1aPosIntStatus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1aPosIntStatus {
    type Ux = u8;
}
#[doc = "Field `GPIO1A_POS_INT_STATUS` reader - gpio1a posedge pulse interrupt status"]
pub type Gpio1aPosIntStatusR = crate::FieldReader<Gpio1aPosIntStatus>;
impl Gpio1aPosIntStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1aPosIntStatus> {
        match self.bits {
            0 => Some(Gpio1aPosIntStatus::B0),
            1 => Some(Gpio1aPosIntStatus::B1),
            _ => None,
        }
    }
    #[doc = "not wakeup by gpio1a posedge pulse"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio1aPosIntStatus::B0
    }
    #[doc = "wakeup by gpio1a posedge pulse"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio1aPosIntStatus::B1
    }
}
#[doc = "Field `GPIO1A_POS_INT_STATUS` writer - gpio1a posedge pulse interrupt status"]
pub type Gpio1aPosIntStatusW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio1aPosIntStatus>;
impl<'a, REG> Gpio1aPosIntStatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "not wakeup by gpio1a posedge pulse"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1aPosIntStatus::B0)
    }
    #[doc = "wakeup by gpio1a posedge pulse"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1aPosIntStatus::B1)
    }
}
#[doc = "gpio1b posedge pulse interrupt status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1bPosIntStatus {
    #[doc = "0: not wakeup by gpio1b posedge pulse"]
    B0 = 0,
    #[doc = "1: wakeup by gpio1b posedge pulse"]
    B1 = 1,
}
impl From<Gpio1bPosIntStatus> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1bPosIntStatus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1bPosIntStatus {
    type Ux = u8;
}
#[doc = "Field `GPIO1B_POS_INT_STATUS` reader - gpio1b posedge pulse interrupt status"]
pub type Gpio1bPosIntStatusR = crate::FieldReader<Gpio1bPosIntStatus>;
impl Gpio1bPosIntStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1bPosIntStatus> {
        match self.bits {
            0 => Some(Gpio1bPosIntStatus::B0),
            1 => Some(Gpio1bPosIntStatus::B1),
            _ => None,
        }
    }
    #[doc = "not wakeup by gpio1b posedge pulse"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio1bPosIntStatus::B0
    }
    #[doc = "wakeup by gpio1b posedge pulse"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio1bPosIntStatus::B1
    }
}
#[doc = "Field `GPIO1B_POS_INT_STATUS` writer - gpio1b posedge pulse interrupt status"]
pub type Gpio1bPosIntStatusW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio1bPosIntStatus>;
impl<'a, REG> Gpio1bPosIntStatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "not wakeup by gpio1b posedge pulse"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1bPosIntStatus::B0)
    }
    #[doc = "wakeup by gpio1b posedge pulse"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1bPosIntStatus::B1)
    }
}
#[doc = "gpio1c posedge pulse interrupt status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1cPosIntStatus {
    #[doc = "0: not wakeup by gpio1c posedge pulse"]
    B0 = 0,
    #[doc = "1: wakeup by gpio1c posedge pulse"]
    B1 = 1,
}
impl From<Gpio1cPosIntStatus> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1cPosIntStatus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1cPosIntStatus {
    type Ux = u8;
}
#[doc = "Field `GPIO1C_POS_INT_STATUS` reader - gpio1c posedge pulse interrupt status"]
pub type Gpio1cPosIntStatusR = crate::FieldReader<Gpio1cPosIntStatus>;
impl Gpio1cPosIntStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1cPosIntStatus> {
        match self.bits {
            0 => Some(Gpio1cPosIntStatus::B0),
            1 => Some(Gpio1cPosIntStatus::B1),
            _ => None,
        }
    }
    #[doc = "not wakeup by gpio1c posedge pulse"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio1cPosIntStatus::B0
    }
    #[doc = "wakeup by gpio1c posedge pulse"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio1cPosIntStatus::B1
    }
}
#[doc = "Field `GPIO1C_POS_INT_STATUS` writer - gpio1c posedge pulse interrupt status"]
pub type Gpio1cPosIntStatusW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio1cPosIntStatus>;
impl<'a, REG> Gpio1cPosIntStatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "not wakeup by gpio1c posedge pulse"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1cPosIntStatus::B0)
    }
    #[doc = "wakeup by gpio1c posedge pulse"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1cPosIntStatus::B1)
    }
}
#[doc = "gpio1d posedge pulse interrupt status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1dPosIntStatus {
    #[doc = "0: not wakeup by gpio1d posedge pulse"]
    B0 = 0,
    #[doc = "1: wakeup by gpio1d posedge pulse"]
    B1 = 1,
}
impl From<Gpio1dPosIntStatus> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1dPosIntStatus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1dPosIntStatus {
    type Ux = u8;
}
#[doc = "Field `GPIO1D_POS_INT_STATUS` reader - gpio1d posedge pulse interrupt status"]
pub type Gpio1dPosIntStatusR = crate::FieldReader<Gpio1dPosIntStatus>;
impl Gpio1dPosIntStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1dPosIntStatus> {
        match self.bits {
            0 => Some(Gpio1dPosIntStatus::B0),
            1 => Some(Gpio1dPosIntStatus::B1),
            _ => None,
        }
    }
    #[doc = "not wakeup by gpio1d posedge pulse"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio1dPosIntStatus::B0
    }
    #[doc = "wakeup by gpio1d posedge pulse"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio1dPosIntStatus::B1
    }
}
#[doc = "Field `GPIO1D_POS_INT_STATUS` writer - gpio1d posedge pulse interrupt status"]
pub type Gpio1dPosIntStatusW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio1dPosIntStatus>;
impl<'a, REG> Gpio1dPosIntStatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "not wakeup by gpio1d posedge pulse"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1dPosIntStatus::B0)
    }
    #[doc = "wakeup by gpio1d posedge pulse"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1dPosIntStatus::B1)
    }
}
impl R {
    #[doc = "Bits 0:7 - gpio1a posedge pulse interrupt status"]
    #[inline(always)]
    pub fn gpio1a_pos_int_status(&self) -> Gpio1aPosIntStatusR {
        Gpio1aPosIntStatusR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - gpio1b posedge pulse interrupt status"]
    #[inline(always)]
    pub fn gpio1b_pos_int_status(&self) -> Gpio1bPosIntStatusR {
        Gpio1bPosIntStatusR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - gpio1c posedge pulse interrupt status"]
    #[inline(always)]
    pub fn gpio1c_pos_int_status(&self) -> Gpio1cPosIntStatusR {
        Gpio1cPosIntStatusR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - gpio1d posedge pulse interrupt status"]
    #[inline(always)]
    pub fn gpio1d_pos_int_status(&self) -> Gpio1dPosIntStatusR {
        Gpio1dPosIntStatusR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - gpio1a posedge pulse interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1a_pos_int_status(&mut self) -> Gpio1aPosIntStatusW<Gpio1PosIntStSpec> {
        Gpio1aPosIntStatusW::new(self, 0)
    }
    #[doc = "Bits 8:15 - gpio1b posedge pulse interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1b_pos_int_status(&mut self) -> Gpio1bPosIntStatusW<Gpio1PosIntStSpec> {
        Gpio1bPosIntStatusW::new(self, 8)
    }
    #[doc = "Bits 16:23 - gpio1c posedge pulse interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1c_pos_int_status(&mut self) -> Gpio1cPosIntStatusW<Gpio1PosIntStSpec> {
        Gpio1cPosIntStatusW::new(self, 16)
    }
    #[doc = "Bits 24:31 - gpio1d posedge pulse interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1d_pos_int_status(&mut self) -> Gpio1dPosIntStatusW<Gpio1PosIntStSpec> {
        Gpio1dPosIntStatusW::new(self, 24)
    }
}
#[doc = "pmu gpio1 posedge interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio1_pos_int_st::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio1_pos_int_st::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio1PosIntStSpec;
impl crate::RegisterSpec for Gpio1PosIntStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio1_pos_int_st::R`](R) reader structure"]
impl crate::Readable for Gpio1PosIntStSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio1_pos_int_st::W`](W) writer structure"]
impl crate::Writable for Gpio1PosIntStSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO1_POS_INT_ST to value 0"]
impl crate::Resettable for Gpio1PosIntStSpec {
    const RESET_VALUE: u32 = 0;
}
