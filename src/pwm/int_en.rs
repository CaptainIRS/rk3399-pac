#[doc = "Register `INT_EN` reader"]
pub type R = crate::R<IntEnSpec>;
#[doc = "Register `INT_EN` writer"]
pub type W = crate::W<IntEnSpec>;
#[doc = "Channel 0 Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0IntEn {
    #[doc = "0: Channel 0 Interrupt disabled"]
    B0 = 0,
    #[doc = "1: Channel 0 Interrupt enabled"]
    B1 = 1,
}
impl From<Ch0IntEn> for bool {
    #[inline(always)]
    fn from(variant: Ch0IntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0_INT_EN` reader - Channel 0 Interrupt Enable"]
pub type Ch0IntEnR = crate::BitReader<Ch0IntEn>;
impl Ch0IntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0IntEn {
        match self.bits {
            false => Ch0IntEn::B0,
            true => Ch0IntEn::B1,
        }
    }
    #[doc = "Channel 0 Interrupt disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ch0IntEn::B0
    }
    #[doc = "Channel 0 Interrupt enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ch0IntEn::B1
    }
}
#[doc = "Field `CH0_INT_EN` writer - Channel 0 Interrupt Enable"]
pub type Ch0IntEnW<'a, REG> = crate::BitWriter<'a, REG, Ch0IntEn>;
impl<'a, REG> Ch0IntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 0 Interrupt disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0IntEn::B0)
    }
    #[doc = "Channel 0 Interrupt enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0IntEn::B1)
    }
}
#[doc = "Channel 1 Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1IntEn {
    #[doc = "0: Channel 1 Interrupt disabled"]
    B0 = 0,
    #[doc = "1: Channel 1 Interrupt enabled"]
    B1 = 1,
}
impl From<Ch1IntEn> for bool {
    #[inline(always)]
    fn from(variant: Ch1IntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1_INT_EN` reader - Channel 1 Interrupt Enable"]
pub type Ch1IntEnR = crate::BitReader<Ch1IntEn>;
impl Ch1IntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1IntEn {
        match self.bits {
            false => Ch1IntEn::B0,
            true => Ch1IntEn::B1,
        }
    }
    #[doc = "Channel 1 Interrupt disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ch1IntEn::B0
    }
    #[doc = "Channel 1 Interrupt enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ch1IntEn::B1
    }
}
#[doc = "Field `CH1_INT_EN` writer - Channel 1 Interrupt Enable"]
pub type Ch1IntEnW<'a, REG> = crate::BitWriter<'a, REG, Ch1IntEn>;
impl<'a, REG> Ch1IntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 1 Interrupt disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1IntEn::B0)
    }
    #[doc = "Channel 1 Interrupt enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1IntEn::B1)
    }
}
#[doc = "Channel 2 Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch2IntEn {
    #[doc = "0: Channel 2 Interrupt disabled"]
    B0 = 0,
    #[doc = "1: Channel 2 Interrupt enabled"]
    B1 = 1,
}
impl From<Ch2IntEn> for bool {
    #[inline(always)]
    fn from(variant: Ch2IntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2_INT_EN` reader - Channel 2 Interrupt Enable"]
pub type Ch2IntEnR = crate::BitReader<Ch2IntEn>;
impl Ch2IntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch2IntEn {
        match self.bits {
            false => Ch2IntEn::B0,
            true => Ch2IntEn::B1,
        }
    }
    #[doc = "Channel 2 Interrupt disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ch2IntEn::B0
    }
    #[doc = "Channel 2 Interrupt enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ch2IntEn::B1
    }
}
#[doc = "Field `CH2_INT_EN` writer - Channel 2 Interrupt Enable"]
pub type Ch2IntEnW<'a, REG> = crate::BitWriter<'a, REG, Ch2IntEn>;
impl<'a, REG> Ch2IntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 2 Interrupt disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2IntEn::B0)
    }
    #[doc = "Channel 2 Interrupt enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2IntEn::B1)
    }
}
#[doc = "Channel 3 Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch3IntEn {
    #[doc = "0: Channel 3 Interrupt disabled"]
    B0 = 0,
    #[doc = "1: Channel 3 Interrupt enabled"]
    B1 = 1,
}
impl From<Ch3IntEn> for bool {
    #[inline(always)]
    fn from(variant: Ch3IntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3_INT_EN` reader - Channel 3 Interrupt Enable"]
pub type Ch3IntEnR = crate::BitReader<Ch3IntEn>;
impl Ch3IntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch3IntEn {
        match self.bits {
            false => Ch3IntEn::B0,
            true => Ch3IntEn::B1,
        }
    }
    #[doc = "Channel 3 Interrupt disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ch3IntEn::B0
    }
    #[doc = "Channel 3 Interrupt enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ch3IntEn::B1
    }
}
#[doc = "Field `CH3_INT_EN` writer - Channel 3 Interrupt Enable"]
pub type Ch3IntEnW<'a, REG> = crate::BitWriter<'a, REG, Ch3IntEn>;
impl<'a, REG> Ch3IntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 3 Interrupt disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3IntEn::B0)
    }
    #[doc = "Channel 3 Interrupt enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3IntEn::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 Interrupt Enable"]
    #[inline(always)]
    pub fn ch0_int_en(&self) -> Ch0IntEnR {
        Ch0IntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Interrupt Enable"]
    #[inline(always)]
    pub fn ch1_int_en(&self) -> Ch1IntEnR {
        Ch1IntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Interrupt Enable"]
    #[inline(always)]
    pub fn ch2_int_en(&self) -> Ch2IntEnR {
        Ch2IntEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Interrupt Enable"]
    #[inline(always)]
    pub fn ch3_int_en(&self) -> Ch3IntEnR {
        Ch3IntEnR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_int_en(&mut self) -> Ch0IntEnW<IntEnSpec> {
        Ch0IntEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_int_en(&mut self) -> Ch1IntEnW<IntEnSpec> {
        Ch1IntEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_int_en(&mut self) -> Ch2IntEnW<IntEnSpec> {
        Ch2IntEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_int_en(&mut self) -> Ch3IntEnW<IntEnSpec> {
        Ch3IntEnW::new(self, 3)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEnSpec;
impl crate::RegisterSpec for IntEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_en::R`](R) reader structure"]
impl crate::Readable for IntEnSpec {}
#[doc = "`write(|w| ..)` method takes [`int_en::W`](W) writer structure"]
impl crate::Writable for IntEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EN to value 0"]
impl crate::Resettable for IntEnSpec {
    const RESET_VALUE: u32 = 0;
}
