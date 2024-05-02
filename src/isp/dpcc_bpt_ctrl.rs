#[doc = "Register `DPCC_BPT_CTRL` reader"]
pub type R = crate::R<DpccBptCtrlSpec>;
#[doc = "Register `DPCC_BPT_CTRL` writer"]
pub type W = crate::W<DpccBptCtrlSpec>;
#[doc = "Bad pixel detection write enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BptDetEn {
    #[doc = "1: bad pixel detection write to memory is enabled"]
    B1 = 1,
    #[doc = "0: bad pixel detection write to memory is disabled"]
    B0 = 0,
}
impl From<BptDetEn> for bool {
    #[inline(always)]
    fn from(variant: BptDetEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `bpt_det_en` reader - Bad pixel detection write enable"]
pub type BptDetEnR = crate::BitReader<BptDetEn>;
impl BptDetEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BptDetEn {
        match self.bits {
            true => BptDetEn::B1,
            false => BptDetEn::B0,
        }
    }
    #[doc = "bad pixel detection write to memory is enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BptDetEn::B1
    }
    #[doc = "bad pixel detection write to memory is disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BptDetEn::B0
    }
}
#[doc = "Field `bpt_det_en` writer - Bad pixel detection write enable"]
pub type BptDetEnW<'a, REG> = crate::BitWriter<'a, REG, BptDetEn>;
impl<'a, REG> BptDetEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "bad pixel detection write to memory is enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BptDetEn::B1)
    }
    #[doc = "bad pixel detection write to memory is disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BptDetEn::B0)
    }
}
#[doc = "table based correction enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BptCorEn {
    #[doc = "1: table based correction is enabled"]
    B1 = 1,
    #[doc = "0: table based correction is disabled"]
    B0 = 0,
}
impl From<BptCorEn> for bool {
    #[inline(always)]
    fn from(variant: BptCorEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `bpt_cor_en` reader - table based correction enable"]
pub type BptCorEnR = crate::BitReader<BptCorEn>;
impl BptCorEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BptCorEn {
        match self.bits {
            true => BptCorEn::B1,
            false => BptCorEn::B0,
        }
    }
    #[doc = "table based correction is enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BptCorEn::B1
    }
    #[doc = "table based correction is disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BptCorEn::B0
    }
}
#[doc = "Field `bpt_cor_en` writer - table based correction enable"]
pub type BptCorEnW<'a, REG> = crate::BitWriter<'a, REG, BptCorEn>;
impl<'a, REG> BptCorEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "table based correction is enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BptCorEn::B1)
    }
    #[doc = "table based correction is disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BptCorEn::B0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BptUseSet1 {
    #[doc = "1: for BPT write use methods set 1"]
    B1 = 1,
    #[doc = "0: for BPT write do not use methods set 1 *Default*"]
    B0 = 0,
}
impl From<BptUseSet1> for bool {
    #[inline(always)]
    fn from(variant: BptUseSet1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPT_USE_SET_1` reader - "]
pub type BptUseSet1R = crate::BitReader<BptUseSet1>;
impl BptUseSet1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BptUseSet1 {
        match self.bits {
            true => BptUseSet1::B1,
            false => BptUseSet1::B0,
        }
    }
    #[doc = "for BPT write use methods set 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BptUseSet1::B1
    }
    #[doc = "for BPT write do not use methods set 1 *Default*"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BptUseSet1::B0
    }
}
#[doc = "Field `BPT_USE_SET_1` writer - "]
pub type BptUseSet1W<'a, REG> = crate::BitWriter<'a, REG, BptUseSet1>;
impl<'a, REG> BptUseSet1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "for BPT write use methods set 1"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BptUseSet1::B1)
    }
    #[doc = "for BPT write do not use methods set 1 *Default*"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BptUseSet1::B0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BptUseSet2 {
    #[doc = "1: for BPT write use methods set 2"]
    B1 = 1,
    #[doc = "0: for BPT write do not use methods set 2 *Default*"]
    B0 = 0,
}
impl From<BptUseSet2> for bool {
    #[inline(always)]
    fn from(variant: BptUseSet2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPT_USE_SET_2` reader - "]
pub type BptUseSet2R = crate::BitReader<BptUseSet2>;
impl BptUseSet2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BptUseSet2 {
        match self.bits {
            true => BptUseSet2::B1,
            false => BptUseSet2::B0,
        }
    }
    #[doc = "for BPT write use methods set 2"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BptUseSet2::B1
    }
    #[doc = "for BPT write do not use methods set 2 *Default*"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BptUseSet2::B0
    }
}
#[doc = "Field `BPT_USE_SET_2` writer - "]
pub type BptUseSet2W<'a, REG> = crate::BitWriter<'a, REG, BptUseSet2>;
impl<'a, REG> BptUseSet2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "for BPT write use methods set 2"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BptUseSet2::B1)
    }
    #[doc = "for BPT write do not use methods set 2 *Default*"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BptUseSet2::B0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BptUseSet3 {
    #[doc = "1: for BPT write use methods set 3"]
    B1 = 1,
    #[doc = "0: for BPT write do not use methods set 3 *Default*"]
    B0 = 0,
}
impl From<BptUseSet3> for bool {
    #[inline(always)]
    fn from(variant: BptUseSet3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPT_USE_SET_3` reader - "]
pub type BptUseSet3R = crate::BitReader<BptUseSet3>;
impl BptUseSet3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BptUseSet3 {
        match self.bits {
            true => BptUseSet3::B1,
            false => BptUseSet3::B0,
        }
    }
    #[doc = "for BPT write use methods set 3"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BptUseSet3::B1
    }
    #[doc = "for BPT write do not use methods set 3 *Default*"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BptUseSet3::B0
    }
}
#[doc = "Field `BPT_USE_SET_3` writer - "]
pub type BptUseSet3W<'a, REG> = crate::BitWriter<'a, REG, BptUseSet3>;
impl<'a, REG> BptUseSet3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "for BPT write use methods set 3"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BptUseSet3::B1)
    }
    #[doc = "for BPT write do not use methods set 3 *Default*"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BptUseSet3::B0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BptUseFixSet {
    #[doc = "1: for BPT write use hard coded methods set"]
    B1 = 1,
    #[doc = "0: for BPT write do not use hard coded methods set *Default*"]
    B0 = 0,
}
impl From<BptUseFixSet> for bool {
    #[inline(always)]
    fn from(variant: BptUseFixSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPT_USE_FIX_SET` reader - "]
pub type BptUseFixSetR = crate::BitReader<BptUseFixSet>;
impl BptUseFixSetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BptUseFixSet {
        match self.bits {
            true => BptUseFixSet::B1,
            false => BptUseFixSet::B0,
        }
    }
    #[doc = "for BPT write use hard coded methods set"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BptUseFixSet::B1
    }
    #[doc = "for BPT write do not use hard coded methods set *Default*"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BptUseFixSet::B0
    }
}
#[doc = "Field `BPT_USE_FIX_SET` writer - "]
pub type BptUseFixSetW<'a, REG> = crate::BitWriter<'a, REG, BptUseFixSet>;
impl<'a, REG> BptUseFixSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "for BPT write use hard coded methods set"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BptUseFixSet::B1)
    }
    #[doc = "for BPT write do not use hard coded methods set *Default*"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BptUseFixSet::B0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BptInclRbCenter {
    #[doc = "1: if BPT active include center pixel for red/blue output median 2x2+1"]
    B1 = 1,
    #[doc = "0: if BPT active do not include center pixel for red/blue output median 2x2 *Default*"]
    B0 = 0,
}
impl From<BptInclRbCenter> for bool {
    #[inline(always)]
    fn from(variant: BptInclRbCenter) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPT_INCL_RB_CENTER` reader - "]
pub type BptInclRbCenterR = crate::BitReader<BptInclRbCenter>;
impl BptInclRbCenterR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BptInclRbCenter {
        match self.bits {
            true => BptInclRbCenter::B1,
            false => BptInclRbCenter::B0,
        }
    }
    #[doc = "if BPT active include center pixel for red/blue output median 2x2+1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BptInclRbCenter::B1
    }
    #[doc = "if BPT active do not include center pixel for red/blue output median 2x2 *Default*"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BptInclRbCenter::B0
    }
}
#[doc = "Field `BPT_INCL_RB_CENTER` writer - "]
pub type BptInclRbCenterW<'a, REG> = crate::BitWriter<'a, REG, BptInclRbCenter>;
impl<'a, REG> BptInclRbCenterW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "if BPT active include center pixel for red/blue output median 2x2+1"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BptInclRbCenter::B1)
    }
    #[doc = "if BPT active do not include center pixel for red/blue output median 2x2 *Default*"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BptInclRbCenter::B0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BptG3x3 {
    #[doc = "1: if BPT active green 9 pixel (3x3) output median"]
    B1 = 1,
    #[doc = "0: if BPT active green 4 or 5 pixel output median *Default*"]
    B0 = 0,
}
impl From<BptG3x3> for bool {
    #[inline(always)]
    fn from(variant: BptG3x3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPT_G_3x3` reader - "]
pub type BptG3x3R = crate::BitReader<BptG3x3>;
impl BptG3x3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BptG3x3 {
        match self.bits {
            true => BptG3x3::B1,
            false => BptG3x3::B0,
        }
    }
    #[doc = "if BPT active green 9 pixel (3x3) output median"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BptG3x3::B1
    }
    #[doc = "if BPT active green 4 or 5 pixel output median *Default*"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BptG3x3::B0
    }
}
#[doc = "Field `BPT_G_3x3` writer - "]
pub type BptG3x3W<'a, REG> = crate::BitWriter<'a, REG, BptG3x3>;
impl<'a, REG> BptG3x3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "if BPT active green 9 pixel (3x3) output median"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BptG3x3::B1)
    }
    #[doc = "if BPT active green 4 or 5 pixel output median *Default*"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BptG3x3::B0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BptRb3x3 {
    #[doc = "1: if BPT active red/blue 9 pixel (3x3) output median"]
    B1 = 1,
    #[doc = "0: if BPT active red/blue 4 or 5 pixel output median *Default*"]
    B0 = 0,
}
impl From<BptRb3x3> for bool {
    #[inline(always)]
    fn from(variant: BptRb3x3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPT_RB_3x3` reader - "]
pub type BptRb3x3R = crate::BitReader<BptRb3x3>;
impl BptRb3x3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BptRb3x3 {
        match self.bits {
            true => BptRb3x3::B1,
            false => BptRb3x3::B0,
        }
    }
    #[doc = "if BPT active red/blue 9 pixel (3x3) output median"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BptRb3x3::B1
    }
    #[doc = "if BPT active red/blue 4 or 5 pixel output median *Default*"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BptRb3x3::B0
    }
}
#[doc = "Field `BPT_RB_3x3` writer - "]
pub type BptRb3x3W<'a, REG> = crate::BitWriter<'a, REG, BptRb3x3>;
impl<'a, REG> BptRb3x3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "if BPT active red/blue 9 pixel (3x3) output median"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BptRb3x3::B1)
    }
    #[doc = "if BPT active red/blue 4 or 5 pixel output median *Default*"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BptRb3x3::B0)
    }
}
impl R {
    #[doc = "Bit 0 - Bad pixel detection write enable"]
    #[inline(always)]
    pub fn bpt_det_en(&self) -> BptDetEnR {
        BptDetEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - table based correction enable"]
    #[inline(always)]
    pub fn bpt_cor_en(&self) -> BptCorEnR {
        BptCorEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn bpt_use_set_1(&self) -> BptUseSet1R {
        BptUseSet1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn bpt_use_set_2(&self) -> BptUseSet2R {
        BptUseSet2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn bpt_use_set_3(&self) -> BptUseSet3R {
        BptUseSet3R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn bpt_use_fix_set(&self) -> BptUseFixSetR {
        BptUseFixSetR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bpt_incl_rb_center(&self) -> BptInclRbCenterR {
        BptInclRbCenterR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn bpt_g_3x3(&self) -> BptG3x3R {
        BptG3x3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn bpt_rb_3x3(&self) -> BptRb3x3R {
        BptRb3x3R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bad pixel detection write enable"]
    #[inline(always)]
    #[must_use]
    pub fn bpt_det_en(&mut self) -> BptDetEnW<DpccBptCtrlSpec> {
        BptDetEnW::new(self, 0)
    }
    #[doc = "Bit 1 - table based correction enable"]
    #[inline(always)]
    #[must_use]
    pub fn bpt_cor_en(&mut self) -> BptCorEnW<DpccBptCtrlSpec> {
        BptCorEnW::new(self, 1)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn bpt_use_set_1(&mut self) -> BptUseSet1W<DpccBptCtrlSpec> {
        BptUseSet1W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn bpt_use_set_2(&mut self) -> BptUseSet2W<DpccBptCtrlSpec> {
        BptUseSet2W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn bpt_use_set_3(&mut self) -> BptUseSet3W<DpccBptCtrlSpec> {
        BptUseSet3W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn bpt_use_fix_set(&mut self) -> BptUseFixSetW<DpccBptCtrlSpec> {
        BptUseFixSetW::new(self, 7)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn bpt_incl_rb_center(&mut self) -> BptInclRbCenterW<DpccBptCtrlSpec> {
        BptInclRbCenterW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn bpt_g_3x3(&mut self) -> BptG3x3W<DpccBptCtrlSpec> {
        BptG3x3W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn bpt_rb_3x3(&mut self) -> BptRb3x3W<DpccBptCtrlSpec> {
        BptRb3x3W::new(self, 11)
    }
}
#[doc = "bad pixel table settings\n\nNote: This register controls the behaviour of the table based bad pixel correction module. \n\nIt can be switched on and off independently of the DPCC detection and correction block. \n\nDifferent correction algorithms for the table based correction are available and are defined by \n\nthis register. The default setting after reset enables a correction algorithm with most accurate \n\ncorrelation to surrounding pixels. Detection for the table based correction can be configured \n\nindependently from the on-the-fly DPCC detection scheme. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_bpt_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_bpt_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpccBptCtrlSpec;
impl crate::RegisterSpec for DpccBptCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpcc_bpt_ctrl::R`](R) reader structure"]
impl crate::Readable for DpccBptCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dpcc_bpt_ctrl::W`](W) writer structure"]
impl crate::Writable for DpccBptCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPCC_BPT_CTRL to value 0"]
impl crate::Resettable for DpccBptCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
