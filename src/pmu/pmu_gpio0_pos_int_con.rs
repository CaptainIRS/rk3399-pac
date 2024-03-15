#[doc = "Register `PMU_GPIO0_POS_INT_CON` reader"]
pub type R = crate::R<PmuGpio0PosIntConSpec>;
#[doc = "Register `PMU_GPIO0_POS_INT_CON` writer"]
pub type W = crate::W<PmuGpio0PosIntConSpec>;
#[doc = "gpio0a posedge pulse interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0aPosIntEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Gpio0aPosIntEn> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0aPosIntEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0aPosIntEn {
    type Ux = u8;
}
#[doc = "Field `GPIO0A_POS_INT_EN` reader - gpio0a posedge pulse interrupt enable"]
pub type Gpio0aPosIntEnR = crate::FieldReader<Gpio0aPosIntEn>;
impl Gpio0aPosIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0aPosIntEn> {
        match self.bits {
            0 => Some(Gpio0aPosIntEn::B0),
            1 => Some(Gpio0aPosIntEn::B1),
            _ => None,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio0aPosIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio0aPosIntEn::B1
    }
}
#[doc = "Field `GPIO0A_POS_INT_EN` writer - gpio0a posedge pulse interrupt enable"]
pub type Gpio0aPosIntEnW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio0aPosIntEn>;
impl<'a, REG> Gpio0aPosIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0aPosIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0aPosIntEn::B1)
    }
}
#[doc = "gpio0b posedge pulse interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0bPosIntEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Gpio0bPosIntEn> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0bPosIntEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0bPosIntEn {
    type Ux = u8;
}
#[doc = "Field `GPIO0B_POS_INT_EN` reader - gpio0b posedge pulse interrupt enable"]
pub type Gpio0bPosIntEnR = crate::FieldReader<Gpio0bPosIntEn>;
impl Gpio0bPosIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0bPosIntEn> {
        match self.bits {
            0 => Some(Gpio0bPosIntEn::B0),
            1 => Some(Gpio0bPosIntEn::B1),
            _ => None,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio0bPosIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio0bPosIntEn::B1
    }
}
#[doc = "Field `GPIO0B_POS_INT_EN` writer - gpio0b posedge pulse interrupt enable"]
pub type Gpio0bPosIntEnW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio0bPosIntEn>;
impl<'a, REG> Gpio0bPosIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0bPosIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0bPosIntEn::B1)
    }
}
#[doc = "gpio0c posedge pulse interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0cPosIntEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Gpio0cPosIntEn> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0cPosIntEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0cPosIntEn {
    type Ux = u8;
}
#[doc = "Field `GPIO0C_POS_INT_EN` reader - gpio0c posedge pulse interrupt enable"]
pub type Gpio0cPosIntEnR = crate::FieldReader<Gpio0cPosIntEn>;
impl Gpio0cPosIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0cPosIntEn> {
        match self.bits {
            0 => Some(Gpio0cPosIntEn::B0),
            1 => Some(Gpio0cPosIntEn::B1),
            _ => None,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio0cPosIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio0cPosIntEn::B1
    }
}
#[doc = "Field `GPIO0C_POS_INT_EN` writer - gpio0c posedge pulse interrupt enable"]
pub type Gpio0cPosIntEnW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio0cPosIntEn>;
impl<'a, REG> Gpio0cPosIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0cPosIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0cPosIntEn::B1)
    }
}
#[doc = "gpio0d posedge pulse interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0dPosIntEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Gpio0dPosIntEn> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0dPosIntEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0dPosIntEn {
    type Ux = u8;
}
#[doc = "Field `GPIO0D_POS_INT_EN` reader - gpio0d posedge pulse interrupt enable"]
pub type Gpio0dPosIntEnR = crate::FieldReader<Gpio0dPosIntEn>;
impl Gpio0dPosIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0dPosIntEn> {
        match self.bits {
            0 => Some(Gpio0dPosIntEn::B0),
            1 => Some(Gpio0dPosIntEn::B1),
            _ => None,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio0dPosIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio0dPosIntEn::B1
    }
}
#[doc = "Field `GPIO0D_POS_INT_EN` writer - gpio0d posedge pulse interrupt enable"]
pub type Gpio0dPosIntEnW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio0dPosIntEn>;
impl<'a, REG> Gpio0dPosIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0dPosIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0dPosIntEn::B1)
    }
}
impl R {
    #[doc = "Bits 0:7 - gpio0a posedge pulse interrupt enable"]
    #[inline(always)]
    pub fn gpio0a_pos_int_en(&self) -> Gpio0aPosIntEnR {
        Gpio0aPosIntEnR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - gpio0b posedge pulse interrupt enable"]
    #[inline(always)]
    pub fn gpio0b_pos_int_en(&self) -> Gpio0bPosIntEnR {
        Gpio0bPosIntEnR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - gpio0c posedge pulse interrupt enable"]
    #[inline(always)]
    pub fn gpio0c_pos_int_en(&self) -> Gpio0cPosIntEnR {
        Gpio0cPosIntEnR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - gpio0d posedge pulse interrupt enable"]
    #[inline(always)]
    pub fn gpio0d_pos_int_en(&self) -> Gpio0dPosIntEnR {
        Gpio0dPosIntEnR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - gpio0a posedge pulse interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0a_pos_int_en(&mut self) -> Gpio0aPosIntEnW<PmuGpio0PosIntConSpec> {
        Gpio0aPosIntEnW::new(self, 0)
    }
    #[doc = "Bits 8:15 - gpio0b posedge pulse interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0b_pos_int_en(&mut self) -> Gpio0bPosIntEnW<PmuGpio0PosIntConSpec> {
        Gpio0bPosIntEnW::new(self, 8)
    }
    #[doc = "Bits 16:23 - gpio0c posedge pulse interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0c_pos_int_en(&mut self) -> Gpio0cPosIntEnW<PmuGpio0PosIntConSpec> {
        Gpio0cPosIntEnW::new(self, 16)
    }
    #[doc = "Bits 24:31 - gpio0d posedge pulse interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0d_pos_int_en(&mut self) -> Gpio0dPosIntEnW<PmuGpio0PosIntConSpec> {
        Gpio0dPosIntEnW::new(self, 24)
    }
}
#[doc = "pmu gpio0 posedge interrupt configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_gpio0_pos_int_con::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_gpio0_pos_int_con::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuGpio0PosIntConSpec;
impl crate::RegisterSpec for PmuGpio0PosIntConSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmu_gpio0_pos_int_con::R`](R) reader structure"]
impl crate::Readable for PmuGpio0PosIntConSpec {}
#[doc = "`write(|w| ..)` method takes [`pmu_gpio0_pos_int_con::W`](W) writer structure"]
impl crate::Writable for PmuGpio0PosIntConSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMU_GPIO0_POS_INT_CON to value 0"]
impl crate::Resettable for PmuGpio0PosIntConSpec {
    const RESET_VALUE: u32 = 0;
}
