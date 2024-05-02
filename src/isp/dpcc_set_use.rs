#[doc = "Register `DPCC_SET_USE` reader"]
pub type R = crate::R<DpccSetUseSpec>;
#[doc = "Register `DPCC_SET_USE` writer"]
pub type W = crate::W<DpccSetUseSpec>;
#[doc = "\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage1UseSet1 {
    #[doc = "1: stage1 use methods set 1 *Default*"]
    B1 = 1,
    #[doc = "0: stage1 do not use methods set 1"]
    B0 = 0,
}
impl From<Stage1UseSet1> for bool {
    #[inline(always)]
    fn from(variant: Stage1UseSet1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STAGE1_USE_SET_1` reader - "]
pub type Stage1UseSet1R = crate::BitReader<Stage1UseSet1>;
impl Stage1UseSet1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stage1UseSet1 {
        match self.bits {
            true => Stage1UseSet1::B1,
            false => Stage1UseSet1::B0,
        }
    }
    #[doc = "stage1 use methods set 1 *Default*"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Stage1UseSet1::B1
    }
    #[doc = "stage1 do not use methods set 1"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Stage1UseSet1::B0
    }
}
#[doc = "Field `STAGE1_USE_SET_1` writer - "]
pub type Stage1UseSet1W<'a, REG> = crate::BitWriter<'a, REG, Stage1UseSet1>;
impl<'a, REG> Stage1UseSet1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "stage1 use methods set 1 *Default*"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Stage1UseSet1::B1)
    }
    #[doc = "stage1 do not use methods set 1"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Stage1UseSet1::B0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage1UseSet2 {
    #[doc = "1: stage1 use methods set 2"]
    B1 = 1,
    #[doc = "0: stage1 do not use methods set 2 *Default*"]
    B0 = 0,
}
impl From<Stage1UseSet2> for bool {
    #[inline(always)]
    fn from(variant: Stage1UseSet2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STAGE1_USE_SET_2` reader - "]
pub type Stage1UseSet2R = crate::BitReader<Stage1UseSet2>;
impl Stage1UseSet2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stage1UseSet2 {
        match self.bits {
            true => Stage1UseSet2::B1,
            false => Stage1UseSet2::B0,
        }
    }
    #[doc = "stage1 use methods set 2"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Stage1UseSet2::B1
    }
    #[doc = "stage1 do not use methods set 2 *Default*"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Stage1UseSet2::B0
    }
}
#[doc = "Field `STAGE1_USE_SET_2` writer - "]
pub type Stage1UseSet2W<'a, REG> = crate::BitWriter<'a, REG, Stage1UseSet2>;
impl<'a, REG> Stage1UseSet2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "stage1 use methods set 2"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Stage1UseSet2::B1)
    }
    #[doc = "stage1 do not use methods set 2 *Default*"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Stage1UseSet2::B0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage1UseSet3 {
    #[doc = "1: stage1 use methods set 3"]
    B1 = 1,
    #[doc = "0: stage1 do not use methods set 3 *Default*"]
    B0 = 0,
}
impl From<Stage1UseSet3> for bool {
    #[inline(always)]
    fn from(variant: Stage1UseSet3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STAGE1_USE_SET_3` reader - "]
pub type Stage1UseSet3R = crate::BitReader<Stage1UseSet3>;
impl Stage1UseSet3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stage1UseSet3 {
        match self.bits {
            true => Stage1UseSet3::B1,
            false => Stage1UseSet3::B0,
        }
    }
    #[doc = "stage1 use methods set 3"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Stage1UseSet3::B1
    }
    #[doc = "stage1 do not use methods set 3 *Default*"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Stage1UseSet3::B0
    }
}
#[doc = "Field `STAGE1_USE_SET_3` writer - "]
pub type Stage1UseSet3W<'a, REG> = crate::BitWriter<'a, REG, Stage1UseSet3>;
impl<'a, REG> Stage1UseSet3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "stage1 use methods set 3"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Stage1UseSet3::B1)
    }
    #[doc = "stage1 do not use methods set 3 *Default*"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Stage1UseSet3::B0)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn stage1_use_set_1(&self) -> Stage1UseSet1R {
        Stage1UseSet1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn stage1_use_set_2(&self) -> Stage1UseSet2R {
        Stage1UseSet2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn stage1_use_set_3(&self) -> Stage1UseSet3R {
        Stage1UseSet3R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn stage1_use_set_1(&mut self) -> Stage1UseSet1W<DpccSetUseSpec> {
        Stage1UseSet1W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn stage1_use_set_2(&mut self) -> Stage1UseSet2W<DpccSetUseSpec> {
        Stage1UseSet2W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn stage1_use_set_3(&mut self) -> Stage1UseSet3W<DpccSetUseSpec> {
        Stage1UseSet3W::new(self, 2)
    }
}
#[doc = "DPCC methods set usage for detection\n\nNote: methods sets can be used in parallel for each stage and the result is the logical OR \n\n\n\nof all selected sets \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_set_use::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_set_use::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpccSetUseSpec;
impl crate::RegisterSpec for DpccSetUseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpcc_set_use::R`](R) reader structure"]
impl crate::Readable for DpccSetUseSpec {}
#[doc = "`write(|w| ..)` method takes [`dpcc_set_use::W`](W) writer structure"]
impl crate::Writable for DpccSetUseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPCC_SET_USE to value 0x01"]
impl crate::Resettable for DpccSetUseSpec {
    const RESET_VALUE: u32 = 0x01;
}
