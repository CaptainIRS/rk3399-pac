#[doc = "Register `PMU_GPIO0_POS_INT_ST` reader"]
pub type R = crate::R<PmuGpio0PosIntStSpec>;
#[doc = "Register `PMU_GPIO0_POS_INT_ST` writer"]
pub type W = crate::W<PmuGpio0PosIntStSpec>;
#[doc = "gpio0a posedge pulse interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0aPosIntStatus {
    #[doc = "0: wakeup by gpio0a posedge pulse"]
    B0 = 0,
    #[doc = "1: wakeup by gpio0a posedge pulse"]
    B1 = 1,
}
impl From<Gpio0aPosIntStatus> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0aPosIntStatus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0aPosIntStatus {
    type Ux = u8;
}
#[doc = "Field `GPIO0A_POS_INT_STATUS` reader - gpio0a posedge pulse interrupt status"]
pub type Gpio0aPosIntStatusR = crate::FieldReader<Gpio0aPosIntStatus>;
impl Gpio0aPosIntStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0aPosIntStatus> {
        match self.bits {
            0 => Some(Gpio0aPosIntStatus::B0),
            1 => Some(Gpio0aPosIntStatus::B1),
            _ => None,
        }
    }
    #[doc = "wakeup by gpio0a posedge pulse"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio0aPosIntStatus::B0
    }
    #[doc = "wakeup by gpio0a posedge pulse"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio0aPosIntStatus::B1
    }
}
#[doc = "Field `GPIO0A_POS_INT_STATUS` writer - gpio0a posedge pulse interrupt status"]
pub type Gpio0aPosIntStatusW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio0aPosIntStatus>;
impl<'a, REG> Gpio0aPosIntStatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "wakeup by gpio0a posedge pulse"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0aPosIntStatus::B0)
    }
    #[doc = "wakeup by gpio0a posedge pulse"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0aPosIntStatus::B1)
    }
}
#[doc = "gpio0b posedge pulse interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0bPosIntStatus {
    #[doc = "0: wakeup by gpio0b posedge pulse"]
    B0 = 0,
    #[doc = "1: wakeup by gpio0b posedge pulse"]
    B1 = 1,
}
impl From<Gpio0bPosIntStatus> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0bPosIntStatus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0bPosIntStatus {
    type Ux = u8;
}
#[doc = "Field `GPIO0B_POS_INT_STATUS` reader - gpio0b posedge pulse interrupt status"]
pub type Gpio0bPosIntStatusR = crate::FieldReader<Gpio0bPosIntStatus>;
impl Gpio0bPosIntStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0bPosIntStatus> {
        match self.bits {
            0 => Some(Gpio0bPosIntStatus::B0),
            1 => Some(Gpio0bPosIntStatus::B1),
            _ => None,
        }
    }
    #[doc = "wakeup by gpio0b posedge pulse"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio0bPosIntStatus::B0
    }
    #[doc = "wakeup by gpio0b posedge pulse"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio0bPosIntStatus::B1
    }
}
#[doc = "Field `GPIO0B_POS_INT_STATUS` writer - gpio0b posedge pulse interrupt status"]
pub type Gpio0bPosIntStatusW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio0bPosIntStatus>;
impl<'a, REG> Gpio0bPosIntStatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "wakeup by gpio0b posedge pulse"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0bPosIntStatus::B0)
    }
    #[doc = "wakeup by gpio0b posedge pulse"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0bPosIntStatus::B1)
    }
}
#[doc = "gpio0c posedge pulse interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0cPosIntStatus {
    #[doc = "0: wakeup by gpio0c posedge pulse"]
    B0 = 0,
    #[doc = "1: wakeup by gpio0c posedge pulse"]
    B1 = 1,
}
impl From<Gpio0cPosIntStatus> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0cPosIntStatus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0cPosIntStatus {
    type Ux = u8;
}
#[doc = "Field `GPIO0C_POS_INT_STATUS` reader - gpio0c posedge pulse interrupt status"]
pub type Gpio0cPosIntStatusR = crate::FieldReader<Gpio0cPosIntStatus>;
impl Gpio0cPosIntStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0cPosIntStatus> {
        match self.bits {
            0 => Some(Gpio0cPosIntStatus::B0),
            1 => Some(Gpio0cPosIntStatus::B1),
            _ => None,
        }
    }
    #[doc = "wakeup by gpio0c posedge pulse"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio0cPosIntStatus::B0
    }
    #[doc = "wakeup by gpio0c posedge pulse"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio0cPosIntStatus::B1
    }
}
#[doc = "Field `GPIO0C_POS_INT_STATUS` writer - gpio0c posedge pulse interrupt status"]
pub type Gpio0cPosIntStatusW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio0cPosIntStatus>;
impl<'a, REG> Gpio0cPosIntStatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "wakeup by gpio0c posedge pulse"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0cPosIntStatus::B0)
    }
    #[doc = "wakeup by gpio0c posedge pulse"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0cPosIntStatus::B1)
    }
}
#[doc = "gpio0d posedge pulse interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0dPosIntStatus {
    #[doc = "0: wakeup by gpio0d posedge pulse"]
    B0 = 0,
    #[doc = "1: wakeup by gpio0d posedge pulse"]
    B1 = 1,
}
impl From<Gpio0dPosIntStatus> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0dPosIntStatus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0dPosIntStatus {
    type Ux = u8;
}
#[doc = "Field `GPIO0D_POS_INT_STATUS` reader - gpio0d posedge pulse interrupt status"]
pub type Gpio0dPosIntStatusR = crate::FieldReader<Gpio0dPosIntStatus>;
impl Gpio0dPosIntStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0dPosIntStatus> {
        match self.bits {
            0 => Some(Gpio0dPosIntStatus::B0),
            1 => Some(Gpio0dPosIntStatus::B1),
            _ => None,
        }
    }
    #[doc = "wakeup by gpio0d posedge pulse"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio0dPosIntStatus::B0
    }
    #[doc = "wakeup by gpio0d posedge pulse"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio0dPosIntStatus::B1
    }
}
#[doc = "Field `GPIO0D_POS_INT_STATUS` writer - gpio0d posedge pulse interrupt status"]
pub type Gpio0dPosIntStatusW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio0dPosIntStatus>;
impl<'a, REG> Gpio0dPosIntStatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "wakeup by gpio0d posedge pulse"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0dPosIntStatus::B0)
    }
    #[doc = "wakeup by gpio0d posedge pulse"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0dPosIntStatus::B1)
    }
}
impl R {
    #[doc = "Bits 0:7 - gpio0a posedge pulse interrupt status"]
    #[inline(always)]
    pub fn gpio0a_pos_int_status(&self) -> Gpio0aPosIntStatusR {
        Gpio0aPosIntStatusR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - gpio0b posedge pulse interrupt status"]
    #[inline(always)]
    pub fn gpio0b_pos_int_status(&self) -> Gpio0bPosIntStatusR {
        Gpio0bPosIntStatusR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - gpio0c posedge pulse interrupt status"]
    #[inline(always)]
    pub fn gpio0c_pos_int_status(&self) -> Gpio0cPosIntStatusR {
        Gpio0cPosIntStatusR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - gpio0d posedge pulse interrupt status"]
    #[inline(always)]
    pub fn gpio0d_pos_int_status(&self) -> Gpio0dPosIntStatusR {
        Gpio0dPosIntStatusR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - gpio0a posedge pulse interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0a_pos_int_status(&mut self) -> Gpio0aPosIntStatusW<PmuGpio0PosIntStSpec> {
        Gpio0aPosIntStatusW::new(self, 0)
    }
    #[doc = "Bits 8:15 - gpio0b posedge pulse interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0b_pos_int_status(&mut self) -> Gpio0bPosIntStatusW<PmuGpio0PosIntStSpec> {
        Gpio0bPosIntStatusW::new(self, 8)
    }
    #[doc = "Bits 16:23 - gpio0c posedge pulse interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0c_pos_int_status(&mut self) -> Gpio0cPosIntStatusW<PmuGpio0PosIntStSpec> {
        Gpio0cPosIntStatusW::new(self, 16)
    }
    #[doc = "Bits 24:31 - gpio0d posedge pulse interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0d_pos_int_status(&mut self) -> Gpio0dPosIntStatusW<PmuGpio0PosIntStSpec> {
        Gpio0dPosIntStatusW::new(self, 24)
    }
}
#[doc = "pmu gpio0 posedge interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_gpio0_pos_int_st::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_gpio0_pos_int_st::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuGpio0PosIntStSpec;
impl crate::RegisterSpec for PmuGpio0PosIntStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmu_gpio0_pos_int_st::R`](R) reader structure"]
impl crate::Readable for PmuGpio0PosIntStSpec {}
#[doc = "`write(|w| ..)` method takes [`pmu_gpio0_pos_int_st::W`](W) writer structure"]
impl crate::Writable for PmuGpio0PosIntStSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMU_GPIO0_POS_INT_ST to value 0"]
impl crate::Resettable for PmuGpio0PosIntStSpec {
    const RESET_VALUE: u32 = 0;
}
