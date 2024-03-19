#[doc = "Register `PMU_NOC_AUTO_ENA` reader"]
pub type R = crate::R<PmuNocAutoEnaSpec>;
#[doc = "Register `PMU_NOC_AUTO_ENA` writer"]
pub type W = crate::W<PmuNocAutoEnaSpec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pmum0GatingDisable {
    #[doc = "0: noting ;"]
    B0 = 0,
    #[doc = "1: clock gating disable."]
    B1 = 1,
}
impl From<Pmum0GatingDisable> for bool {
    #[inline(always)]
    fn from(variant: Pmum0GatingDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMUM0_GATING_DISABLE` reader - "]
pub type Pmum0GatingDisableR = crate::BitReader<Pmum0GatingDisable>;
impl Pmum0GatingDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pmum0GatingDisable {
        match self.bits {
            false => Pmum0GatingDisable::B0,
            true => Pmum0GatingDisable::B1,
        }
    }
    #[doc = "noting ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Pmum0GatingDisable::B0
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Pmum0GatingDisable::B1
    }
}
#[doc = "Field `PMUM0_GATING_DISABLE` writer - "]
pub type Pmum0GatingDisableW<'a, REG> = crate::BitWriter<'a, REG, Pmum0GatingDisable>;
impl<'a, REG> Pmum0GatingDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "noting ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Pmum0GatingDisable::B0)
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Pmum0GatingDisable::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Center1GatingDisable {
    #[doc = "0: nothing"]
    B0 = 0,
    #[doc = "1: clock gating disable."]
    B1 = 1,
}
impl From<Center1GatingDisable> for bool {
    #[inline(always)]
    fn from(variant: Center1GatingDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CENTER1_GATING_DISABLE` reader - "]
pub type Center1GatingDisableR = crate::BitReader<Center1GatingDisable>;
impl Center1GatingDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Center1GatingDisable {
        match self.bits {
            false => Center1GatingDisable::B0,
            true => Center1GatingDisable::B1,
        }
    }
    #[doc = "nothing"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Center1GatingDisable::B0
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Center1GatingDisable::B1
    }
}
#[doc = "Field `CENTER1_GATING_DISABLE` writer - "]
pub type Center1GatingDisableW<'a, REG> = crate::BitWriter<'a, REG, Center1GatingDisable>;
impl<'a, REG> Center1GatingDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "nothing"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Center1GatingDisable::B0)
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Center1GatingDisable::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmmcGatingDisable {
    #[doc = "0: nothing ;"]
    B0 = 0,
    #[doc = "1: clock gating disable."]
    B1 = 1,
}
impl From<EmmcGatingDisable> for bool {
    #[inline(always)]
    fn from(variant: EmmcGatingDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EMMC_GATING_DISABLE` reader - "]
pub type EmmcGatingDisableR = crate::BitReader<EmmcGatingDisable>;
impl EmmcGatingDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EmmcGatingDisable {
        match self.bits {
            false => EmmcGatingDisable::B0,
            true => EmmcGatingDisable::B1,
        }
    }
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == EmmcGatingDisable::B0
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == EmmcGatingDisable::B1
    }
}
#[doc = "Field `EMMC_GATING_DISABLE` writer - "]
pub type EmmcGatingDisableW<'a, REG> = crate::BitWriter<'a, REG, EmmcGatingDisable>;
impl<'a, REG> EmmcGatingDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(EmmcGatingDisable::B0)
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(EmmcGatingDisable::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GmacGatingDisable {
    #[doc = "0: nothing ;"]
    B0 = 0,
    #[doc = "1: gmac clock gating disable."]
    B1 = 1,
}
impl From<GmacGatingDisable> for bool {
    #[inline(always)]
    fn from(variant: GmacGatingDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GMAC_GATING_DISABLE` reader - "]
pub type GmacGatingDisableR = crate::BitReader<GmacGatingDisable>;
impl GmacGatingDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GmacGatingDisable {
        match self.bits {
            false => GmacGatingDisable::B0,
            true => GmacGatingDisable::B1,
        }
    }
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == GmacGatingDisable::B0
    }
    #[doc = "gmac clock gating disable."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == GmacGatingDisable::B1
    }
}
#[doc = "Field `GMAC_GATING_DISABLE` writer - "]
pub type GmacGatingDisableW<'a, REG> = crate::BitWriter<'a, REG, GmacGatingDisable>;
impl<'a, REG> GmacGatingDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(GmacGatingDisable::B0)
    }
    #[doc = "gmac clock gating disable."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(GmacGatingDisable::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdpGatingDisable {
    #[doc = "0: nothing ;"]
    B0 = 0,
    #[doc = "1: clock gating disable."]
    B1 = 1,
}
impl From<EdpGatingDisable> for bool {
    #[inline(always)]
    fn from(variant: EdpGatingDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDP_GATING_DISABLE` reader - "]
pub type EdpGatingDisableR = crate::BitReader<EdpGatingDisable>;
impl EdpGatingDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EdpGatingDisable {
        match self.bits {
            false => EdpGatingDisable::B0,
            true => EdpGatingDisable::B1,
        }
    }
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == EdpGatingDisable::B0
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == EdpGatingDisable::B1
    }
}
#[doc = "Field `EDP_GATING_DISABLE` writer - "]
pub type EdpGatingDisableW<'a, REG> = crate::BitWriter<'a, REG, EdpGatingDisable>;
impl<'a, REG> EdpGatingDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(EdpGatingDisable::B0)
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(EdpGatingDisable::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PmuGatingDisable {
    #[doc = "0: nothing ;"]
    B0 = 0,
    #[doc = "1: clock gating disable."]
    B1 = 1,
}
impl From<PmuGatingDisable> for bool {
    #[inline(always)]
    fn from(variant: PmuGatingDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMU_GATING_DISABLE` reader - "]
pub type PmuGatingDisableR = crate::BitReader<PmuGatingDisable>;
impl PmuGatingDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PmuGatingDisable {
        match self.bits {
            false => PmuGatingDisable::B0,
            true => PmuGatingDisable::B1,
        }
    }
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PmuGatingDisable::B0
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PmuGatingDisable::B1
    }
}
#[doc = "Field `PMU_GATING_DISABLE` writer - "]
pub type PmuGatingDisableW<'a, REG> = crate::BitWriter<'a, REG, PmuGatingDisable>;
impl<'a, REG> PmuGatingDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PmuGatingDisable::B0)
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PmuGatingDisable::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AliveGatingDisable {
    #[doc = "0: nothing ;"]
    B0 = 0,
    #[doc = "1: clock gating disable."]
    B1 = 1,
}
impl From<AliveGatingDisable> for bool {
    #[inline(always)]
    fn from(variant: AliveGatingDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALIVE_GATING_DISABLE` reader - "]
pub type AliveGatingDisableR = crate::BitReader<AliveGatingDisable>;
impl AliveGatingDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AliveGatingDisable {
        match self.bits {
            false => AliveGatingDisable::B0,
            true => AliveGatingDisable::B1,
        }
    }
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AliveGatingDisable::B0
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AliveGatingDisable::B1
    }
}
#[doc = "Field `ALIVE_GATING_DISABLE` writer - "]
pub type AliveGatingDisableW<'a, REG> = crate::BitWriter<'a, REG, AliveGatingDisable>;
impl<'a, REG> AliveGatingDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(AliveGatingDisable::B0)
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(AliveGatingDisable::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msch1GatingDisable {
    #[doc = "0: nothing ;"]
    B0 = 0,
    #[doc = "1: clock gating disable."]
    B1 = 1,
}
impl From<Msch1GatingDisable> for bool {
    #[inline(always)]
    fn from(variant: Msch1GatingDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSCH1_GATING_DISABLE` reader - "]
pub type Msch1GatingDisableR = crate::BitReader<Msch1GatingDisable>;
impl Msch1GatingDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msch1GatingDisable {
        match self.bits {
            false => Msch1GatingDisable::B0,
            true => Msch1GatingDisable::B1,
        }
    }
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Msch1GatingDisable::B0
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Msch1GatingDisable::B1
    }
}
#[doc = "Field `MSCH1_GATING_DISABLE` writer - "]
pub type Msch1GatingDisableW<'a, REG> = crate::BitWriter<'a, REG, Msch1GatingDisable>;
impl<'a, REG> Msch1GatingDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Msch1GatingDisable::B0)
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Msch1GatingDisable::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msch0GatingDisable {
    #[doc = "0: nothing ;"]
    B0 = 0,
    #[doc = "1: clock gating disable."]
    B1 = 1,
}
impl From<Msch0GatingDisable> for bool {
    #[inline(always)]
    fn from(variant: Msch0GatingDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSCH0_GATING_DISABLE` reader - "]
pub type Msch0GatingDisableR = crate::BitReader<Msch0GatingDisable>;
impl Msch0GatingDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msch0GatingDisable {
        match self.bits {
            false => Msch0GatingDisable::B0,
            true => Msch0GatingDisable::B1,
        }
    }
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Msch0GatingDisable::B0
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Msch0GatingDisable::B1
    }
}
#[doc = "Field `MSCH0_GATING_DISABLE` writer - "]
pub type Msch0GatingDisableW<'a, REG> = crate::BitWriter<'a, REG, Msch0GatingDisable>;
impl<'a, REG> Msch0GatingDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Msch0GatingDisable::B0)
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Msch0GatingDisable::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VioGatingDisable {
    #[doc = "0: nothing ;"]
    B0 = 0,
    #[doc = "1: clock gating disable."]
    B1 = 1,
}
impl From<VioGatingDisable> for bool {
    #[inline(always)]
    fn from(variant: VioGatingDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VIO_GATING_DISABLE` reader - "]
pub type VioGatingDisableR = crate::BitReader<VioGatingDisable>;
impl VioGatingDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VioGatingDisable {
        match self.bits {
            false => VioGatingDisable::B0,
            true => VioGatingDisable::B1,
        }
    }
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VioGatingDisable::B0
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VioGatingDisable::B1
    }
}
#[doc = "Field `VIO_GATING_DISABLE` writer - "]
pub type VioGatingDisableW<'a, REG> = crate::BitWriter<'a, REG, VioGatingDisable>;
impl<'a, REG> VioGatingDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VioGatingDisable::B0)
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VioGatingDisable::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccim1GatingDisable {
    #[doc = "0: nothing ;"]
    B0 = 0,
    #[doc = "1: clock gating disable."]
    B1 = 1,
}
impl From<Ccim1GatingDisable> for bool {
    #[inline(always)]
    fn from(variant: Ccim1GatingDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCIM1_GATING_DISABLE` reader - "]
pub type Ccim1GatingDisableR = crate::BitReader<Ccim1GatingDisable>;
impl Ccim1GatingDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccim1GatingDisable {
        match self.bits {
            false => Ccim1GatingDisable::B0,
            true => Ccim1GatingDisable::B1,
        }
    }
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ccim1GatingDisable::B0
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ccim1GatingDisable::B1
    }
}
#[doc = "Field `CCIM1_GATING_DISABLE` writer - "]
pub type Ccim1GatingDisableW<'a, REG> = crate::BitWriter<'a, REG, Ccim1GatingDisable>;
impl<'a, REG> Ccim1GatingDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccim1GatingDisable::B0)
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccim1GatingDisable::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccim0GatingDisable {
    #[doc = "0: nothing ;"]
    B0 = 0,
    #[doc = "1: clock gating disable."]
    B1 = 1,
}
impl From<Ccim0GatingDisable> for bool {
    #[inline(always)]
    fn from(variant: Ccim0GatingDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCIM0_GATING_DISABLE` reader - "]
pub type Ccim0GatingDisableR = crate::BitReader<Ccim0GatingDisable>;
impl Ccim0GatingDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccim0GatingDisable {
        match self.bits {
            false => Ccim0GatingDisable::B0,
            true => Ccim0GatingDisable::B1,
        }
    }
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ccim0GatingDisable::B0
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ccim0GatingDisable::B1
    }
}
#[doc = "Field `CCIM0_GATING_DISABLE` writer - "]
pub type Ccim0GatingDisableW<'a, REG> = crate::BitWriter<'a, REG, Ccim0GatingDisable>;
impl<'a, REG> Ccim0GatingDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccim0GatingDisable::B0)
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccim0GatingDisable::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CenterGatingDisable {
    #[doc = "0: noting"]
    B0 = 0,
    #[doc = "1: clock gating disable."]
    B1 = 1,
}
impl From<CenterGatingDisable> for bool {
    #[inline(always)]
    fn from(variant: CenterGatingDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CENTER_GATING_DISABLE` reader - "]
pub type CenterGatingDisableR = crate::BitReader<CenterGatingDisable>;
impl CenterGatingDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CenterGatingDisable {
        match self.bits {
            false => CenterGatingDisable::B0,
            true => CenterGatingDisable::B1,
        }
    }
    #[doc = "noting"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CenterGatingDisable::B0
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CenterGatingDisable::B1
    }
}
#[doc = "Field `CENTER_GATING_DISABLE` writer - "]
pub type CenterGatingDisableW<'a, REG> = crate::BitWriter<'a, REG, CenterGatingDisable>;
impl<'a, REG> CenterGatingDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "noting"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CenterGatingDisable::B0)
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CenterGatingDisable::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perilpm0GatingDisable {
    #[doc = "0: nothing ;"]
    B0 = 0,
    #[doc = "1: clock gating disable."]
    B1 = 1,
}
impl From<Perilpm0GatingDisable> for bool {
    #[inline(always)]
    fn from(variant: Perilpm0GatingDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERILPM0_GATING_DISABLE` reader - "]
pub type Perilpm0GatingDisableR = crate::BitReader<Perilpm0GatingDisable>;
impl Perilpm0GatingDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perilpm0GatingDisable {
        match self.bits {
            false => Perilpm0GatingDisable::B0,
            true => Perilpm0GatingDisable::B1,
        }
    }
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Perilpm0GatingDisable::B0
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Perilpm0GatingDisable::B1
    }
}
#[doc = "Field `PERILPM0_GATING_DISABLE` writer - "]
pub type Perilpm0GatingDisableW<'a, REG> = crate::BitWriter<'a, REG, Perilpm0GatingDisable>;
impl<'a, REG> Perilpm0GatingDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Perilpm0GatingDisable::B0)
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Perilpm0GatingDisable::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usb3GatingDisable {
    #[doc = "0: nothing"]
    B0 = 0,
    #[doc = "1: clock gating disable."]
    B1 = 1,
}
impl From<Usb3GatingDisable> for bool {
    #[inline(always)]
    fn from(variant: Usb3GatingDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB3_GATING_DISABLE` reader - "]
pub type Usb3GatingDisableR = crate::BitReader<Usb3GatingDisable>;
impl Usb3GatingDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usb3GatingDisable {
        match self.bits {
            false => Usb3GatingDisable::B0,
            true => Usb3GatingDisable::B1,
        }
    }
    #[doc = "nothing"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Usb3GatingDisable::B0
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Usb3GatingDisable::B1
    }
}
#[doc = "Field `USB3_GATING_DISABLE` writer - "]
pub type Usb3GatingDisableW<'a, REG> = crate::BitWriter<'a, REG, Usb3GatingDisable>;
impl<'a, REG> Usb3GatingDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "nothing"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Usb3GatingDisable::B0)
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Usb3GatingDisable::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HdcpGatingDisable {
    #[doc = "0: nothing ;"]
    B0 = 0,
    #[doc = "1: clock gating disable."]
    B1 = 1,
}
impl From<HdcpGatingDisable> for bool {
    #[inline(always)]
    fn from(variant: HdcpGatingDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCP_GATING_DISABLE` reader - "]
pub type HdcpGatingDisableR = crate::BitReader<HdcpGatingDisable>;
impl HdcpGatingDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HdcpGatingDisable {
        match self.bits {
            false => HdcpGatingDisable::B0,
            true => HdcpGatingDisable::B1,
        }
    }
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HdcpGatingDisable::B0
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HdcpGatingDisable::B1
    }
}
#[doc = "Field `HDCP_GATING_DISABLE` writer - "]
pub type HdcpGatingDisableW<'a, REG> = crate::BitWriter<'a, REG, HdcpGatingDisable>;
impl<'a, REG> HdcpGatingDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HdcpGatingDisable::B0)
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HdcpGatingDisable::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isp1GatingDisable {
    #[doc = "0: nothing ;"]
    B0 = 0,
    #[doc = "1: clock gating disable."]
    B1 = 1,
}
impl From<Isp1GatingDisable> for bool {
    #[inline(always)]
    fn from(variant: Isp1GatingDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISP1_GATING_DISABLE` reader - "]
pub type Isp1GatingDisableR = crate::BitReader<Isp1GatingDisable>;
impl Isp1GatingDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isp1GatingDisable {
        match self.bits {
            false => Isp1GatingDisable::B0,
            true => Isp1GatingDisable::B1,
        }
    }
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Isp1GatingDisable::B0
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Isp1GatingDisable::B1
    }
}
#[doc = "Field `ISP1_GATING_DISABLE` writer - "]
pub type Isp1GatingDisableW<'a, REG> = crate::BitWriter<'a, REG, Isp1GatingDisable>;
impl<'a, REG> Isp1GatingDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Isp1GatingDisable::B0)
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Isp1GatingDisable::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isp0GatingDisable {
    #[doc = "0: nothing ;"]
    B0 = 0,
    #[doc = "1: clock gating disable."]
    B1 = 1,
}
impl From<Isp0GatingDisable> for bool {
    #[inline(always)]
    fn from(variant: Isp0GatingDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISP0_GATING_DISABLE` reader - "]
pub type Isp0GatingDisableR = crate::BitReader<Isp0GatingDisable>;
impl Isp0GatingDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isp0GatingDisable {
        match self.bits {
            false => Isp0GatingDisable::B0,
            true => Isp0GatingDisable::B1,
        }
    }
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Isp0GatingDisable::B0
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Isp0GatingDisable::B1
    }
}
#[doc = "Field `ISP0_GATING_DISABLE` writer - "]
pub type Isp0GatingDisableW<'a, REG> = crate::BitWriter<'a, REG, Isp0GatingDisable>;
impl<'a, REG> Isp0GatingDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Isp0GatingDisable::B0)
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Isp0GatingDisable::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VoplGatingDisable {
    #[doc = "0: nothing ;"]
    B0 = 0,
    #[doc = "1: clock gating disable."]
    B1 = 1,
}
impl From<VoplGatingDisable> for bool {
    #[inline(always)]
    fn from(variant: VoplGatingDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VOPL_GATING_DISABLE` reader - "]
pub type VoplGatingDisableR = crate::BitReader<VoplGatingDisable>;
impl VoplGatingDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VoplGatingDisable {
        match self.bits {
            false => VoplGatingDisable::B0,
            true => VoplGatingDisable::B1,
        }
    }
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VoplGatingDisable::B0
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VoplGatingDisable::B1
    }
}
#[doc = "Field `VOPL_GATING_DISABLE` writer - "]
pub type VoplGatingDisableW<'a, REG> = crate::BitWriter<'a, REG, VoplGatingDisable>;
impl<'a, REG> VoplGatingDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VoplGatingDisable::B0)
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VoplGatingDisable::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VopbGatingDisable {
    #[doc = "0: nothing ;"]
    B0 = 0,
    #[doc = "1: clock gating disable."]
    B1 = 1,
}
impl From<VopbGatingDisable> for bool {
    #[inline(always)]
    fn from(variant: VopbGatingDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VOPB_GATING_DISABLE` reader - "]
pub type VopbGatingDisableR = crate::BitReader<VopbGatingDisable>;
impl VopbGatingDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VopbGatingDisable {
        match self.bits {
            false => VopbGatingDisable::B0,
            true => VopbGatingDisable::B1,
        }
    }
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VopbGatingDisable::B0
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VopbGatingDisable::B1
    }
}
#[doc = "Field `VOPB_GATING_DISABLE` writer - "]
pub type VopbGatingDisableW<'a, REG> = crate::BitWriter<'a, REG, VopbGatingDisable>;
impl<'a, REG> VopbGatingDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VopbGatingDisable::B0)
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VopbGatingDisable::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IepGatingDisable {
    #[doc = "0: nothing ;"]
    B0 = 0,
    #[doc = "1: clock gating disable."]
    B1 = 1,
}
impl From<IepGatingDisable> for bool {
    #[inline(always)]
    fn from(variant: IepGatingDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IEP_GATING_DISABLE` reader - "]
pub type IepGatingDisableR = crate::BitReader<IepGatingDisable>;
impl IepGatingDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IepGatingDisable {
        match self.bits {
            false => IepGatingDisable::B0,
            true => IepGatingDisable::B1,
        }
    }
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IepGatingDisable::B0
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IepGatingDisable::B1
    }
}
#[doc = "Field `IEP_GATING_DISABLE` writer - "]
pub type IepGatingDisableW<'a, REG> = crate::BitWriter<'a, REG, IepGatingDisable>;
impl<'a, REG> IepGatingDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IepGatingDisable::B0)
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IepGatingDisable::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RgaGatingDisable {
    #[doc = "0: nothing ;"]
    B0 = 0,
    #[doc = "1: clock gating disable."]
    B1 = 1,
}
impl From<RgaGatingDisable> for bool {
    #[inline(always)]
    fn from(variant: RgaGatingDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGA_GATING_DISABLE` reader - "]
pub type RgaGatingDisableR = crate::BitReader<RgaGatingDisable>;
impl RgaGatingDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RgaGatingDisable {
        match self.bits {
            false => RgaGatingDisable::B0,
            true => RgaGatingDisable::B1,
        }
    }
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == RgaGatingDisable::B0
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == RgaGatingDisable::B1
    }
}
#[doc = "Field `RGA_GATING_DISABLE` writer - "]
pub type RgaGatingDisableW<'a, REG> = crate::BitWriter<'a, REG, RgaGatingDisable>;
impl<'a, REG> RgaGatingDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(RgaGatingDisable::B0)
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(RgaGatingDisable::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VduGatingDisable {
    #[doc = "0: nothing ;"]
    B0 = 0,
    #[doc = "1: clock gating disable."]
    B1 = 1,
}
impl From<VduGatingDisable> for bool {
    #[inline(always)]
    fn from(variant: VduGatingDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDU_GATING_DISABLE` reader - "]
pub type VduGatingDisableR = crate::BitReader<VduGatingDisable>;
impl VduGatingDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VduGatingDisable {
        match self.bits {
            false => VduGatingDisable::B0,
            true => VduGatingDisable::B1,
        }
    }
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VduGatingDisable::B0
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VduGatingDisable::B1
    }
}
#[doc = "Field `VDU_GATING_DISABLE` writer - "]
pub type VduGatingDisableW<'a, REG> = crate::BitWriter<'a, REG, VduGatingDisable>;
impl<'a, REG> VduGatingDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VduGatingDisable::B0)
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VduGatingDisable::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VcodecGatingDisable {
    #[doc = "0: nothing"]
    B0 = 0,
    #[doc = "1: clock gating disable."]
    B1 = 1,
}
impl From<VcodecGatingDisable> for bool {
    #[inline(always)]
    fn from(variant: VcodecGatingDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCODEC_GATING_DISABLE` reader - "]
pub type VcodecGatingDisableR = crate::BitReader<VcodecGatingDisable>;
impl VcodecGatingDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VcodecGatingDisable {
        match self.bits {
            false => VcodecGatingDisable::B0,
            true => VcodecGatingDisable::B1,
        }
    }
    #[doc = "nothing"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VcodecGatingDisable::B0
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VcodecGatingDisable::B1
    }
}
#[doc = "Field `VCODEC_GATING_DISABLE` writer - "]
pub type VcodecGatingDisableW<'a, REG> = crate::BitWriter<'a, REG, VcodecGatingDisable>;
impl<'a, REG> VcodecGatingDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "nothing"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VcodecGatingDisable::B0)
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VcodecGatingDisable::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PerihpGatingDisable {
    #[doc = "0: nothing ;"]
    B0 = 0,
    #[doc = "1: clock gating disable."]
    B1 = 1,
}
impl From<PerihpGatingDisable> for bool {
    #[inline(always)]
    fn from(variant: PerihpGatingDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERIHP_GATING_DISABLE` reader - "]
pub type PerihpGatingDisableR = crate::BitReader<PerihpGatingDisable>;
impl PerihpGatingDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PerihpGatingDisable {
        match self.bits {
            false => PerihpGatingDisable::B0,
            true => PerihpGatingDisable::B1,
        }
    }
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PerihpGatingDisable::B0
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PerihpGatingDisable::B1
    }
}
#[doc = "Field `PERIHP_GATING_DISABLE` writer - "]
pub type PerihpGatingDisableW<'a, REG> = crate::BitWriter<'a, REG, PerihpGatingDisable>;
impl<'a, REG> PerihpGatingDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PerihpGatingDisable::B0)
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PerihpGatingDisable::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PerilpGatingDisable {
    #[doc = "0: nothing ;"]
    B0 = 0,
    #[doc = "1: clock gating disable."]
    B1 = 1,
}
impl From<PerilpGatingDisable> for bool {
    #[inline(always)]
    fn from(variant: PerilpGatingDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERILP_GATING_DISABLE` reader - "]
pub type PerilpGatingDisableR = crate::BitReader<PerilpGatingDisable>;
impl PerilpGatingDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PerilpGatingDisable {
        match self.bits {
            false => PerilpGatingDisable::B0,
            true => PerilpGatingDisable::B1,
        }
    }
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PerilpGatingDisable::B0
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PerilpGatingDisable::B1
    }
}
#[doc = "Field `PERILP_GATING_DISABLE` writer - "]
pub type PerilpGatingDisableW<'a, REG> = crate::BitWriter<'a, REG, PerilpGatingDisable>;
impl<'a, REG> PerilpGatingDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PerilpGatingDisable::B0)
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PerilpGatingDisable::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpuGatingDisable {
    #[doc = "0: nothing ;"]
    B0 = 0,
    #[doc = "1: clock gating disable."]
    B1 = 1,
}
impl From<GpuGatingDisable> for bool {
    #[inline(always)]
    fn from(variant: GpuGatingDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPU_GATING_DISABLE` reader - "]
pub type GpuGatingDisableR = crate::BitReader<GpuGatingDisable>;
impl GpuGatingDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpuGatingDisable {
        match self.bits {
            false => GpuGatingDisable::B0,
            true => GpuGatingDisable::B1,
        }
    }
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == GpuGatingDisable::B0
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == GpuGatingDisable::B1
    }
}
#[doc = "Field `GPU_GATING_DISABLE` writer - "]
pub type GpuGatingDisableW<'a, REG> = crate::BitWriter<'a, REG, GpuGatingDisable>;
impl<'a, REG> GpuGatingDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(GpuGatingDisable::B0)
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(GpuGatingDisable::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GicGatingDisable {
    #[doc = "0: nothing ;"]
    B0 = 0,
    #[doc = "1: clock gating disable."]
    B1 = 1,
}
impl From<GicGatingDisable> for bool {
    #[inline(always)]
    fn from(variant: GicGatingDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GIC_GATING_DISABLE` reader - "]
pub type GicGatingDisableR = crate::BitReader<GicGatingDisable>;
impl GicGatingDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GicGatingDisable {
        match self.bits {
            false => GicGatingDisable::B0,
            true => GicGatingDisable::B1,
        }
    }
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == GicGatingDisable::B0
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == GicGatingDisable::B1
    }
}
#[doc = "Field `GIC_GATING_DISABLE` writer - "]
pub type GicGatingDisableW<'a, REG> = crate::BitWriter<'a, REG, GicGatingDisable>;
impl<'a, REG> GicGatingDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "nothing ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(GicGatingDisable::B0)
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(GicGatingDisable::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SdGatingDisable {
    #[doc = "0: nothing"]
    B0 = 0,
    #[doc = "1: clock gating disable."]
    B1 = 1,
}
impl From<SdGatingDisable> for bool {
    #[inline(always)]
    fn from(variant: SdGatingDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SD_GATING_DISABLE` reader - "]
pub type SdGatingDisableR = crate::BitReader<SdGatingDisable>;
impl SdGatingDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SdGatingDisable {
        match self.bits {
            false => SdGatingDisable::B0,
            true => SdGatingDisable::B1,
        }
    }
    #[doc = "nothing"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SdGatingDisable::B0
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SdGatingDisable::B1
    }
}
#[doc = "Field `SD_GATING_DISABLE` writer - "]
pub type SdGatingDisableW<'a, REG> = crate::BitWriter<'a, REG, SdGatingDisable>;
impl<'a, REG> SdGatingDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "nothing"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SdGatingDisable::B0)
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SdGatingDisable::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SdioaudioGatingDisable {
    #[doc = "0: nothing"]
    B0 = 0,
    #[doc = "1: clock gating disable."]
    B1 = 1,
}
impl From<SdioaudioGatingDisable> for bool {
    #[inline(always)]
    fn from(variant: SdioaudioGatingDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIOAUDIO_GATING_DISABLE` reader - "]
pub type SdioaudioGatingDisableR = crate::BitReader<SdioaudioGatingDisable>;
impl SdioaudioGatingDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SdioaudioGatingDisable {
        match self.bits {
            false => SdioaudioGatingDisable::B0,
            true => SdioaudioGatingDisable::B1,
        }
    }
    #[doc = "nothing"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SdioaudioGatingDisable::B0
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SdioaudioGatingDisable::B1
    }
}
#[doc = "Field `SDIOAUDIO_GATING_DISABLE` writer - "]
pub type SdioaudioGatingDisableW<'a, REG> = crate::BitWriter<'a, REG, SdioaudioGatingDisable>;
impl<'a, REG> SdioaudioGatingDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "nothing"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SdioaudioGatingDisable::B0)
    }
    #[doc = "clock gating disable."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SdioaudioGatingDisable::B1)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pmum0_gating_disable(&self) -> Pmum0GatingDisableR {
        Pmum0GatingDisableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn center1_gating_disable(&self) -> Center1GatingDisableR {
        Center1GatingDisableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn emmc_gating_disable(&self) -> EmmcGatingDisableR {
        EmmcGatingDisableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gmac_gating_disable(&self) -> GmacGatingDisableR {
        GmacGatingDisableR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn edp_gating_disable(&self) -> EdpGatingDisableR {
        EdpGatingDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pmu_gating_disable(&self) -> PmuGatingDisableR {
        PmuGatingDisableR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn alive_gating_disable(&self) -> AliveGatingDisableR {
        AliveGatingDisableR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn msch1_gating_disable(&self) -> Msch1GatingDisableR {
        Msch1GatingDisableR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn msch0_gating_disable(&self) -> Msch0GatingDisableR {
        Msch0GatingDisableR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn vio_gating_disable(&self) -> VioGatingDisableR {
        VioGatingDisableR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ccim1_gating_disable(&self) -> Ccim1GatingDisableR {
        Ccim1GatingDisableR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ccim0_gating_disable(&self) -> Ccim0GatingDisableR {
        Ccim0GatingDisableR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn center_gating_disable(&self) -> CenterGatingDisableR {
        CenterGatingDisableR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn perilpm0_gating_disable(&self) -> Perilpm0GatingDisableR {
        Perilpm0GatingDisableR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn usb3_gating_disable(&self) -> Usb3GatingDisableR {
        Usb3GatingDisableR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn hdcp_gating_disable(&self) -> HdcpGatingDisableR {
        HdcpGatingDisableR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn isp1_gating_disable(&self) -> Isp1GatingDisableR {
        Isp1GatingDisableR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn isp0_gating_disable(&self) -> Isp0GatingDisableR {
        Isp0GatingDisableR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn vopl_gating_disable(&self) -> VoplGatingDisableR {
        VoplGatingDisableR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn vopb_gating_disable(&self) -> VopbGatingDisableR {
        VopbGatingDisableR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn iep_gating_disable(&self) -> IepGatingDisableR {
        IepGatingDisableR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rga_gating_disable(&self) -> RgaGatingDisableR {
        RgaGatingDisableR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn vdu_gating_disable(&self) -> VduGatingDisableR {
        VduGatingDisableR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn vcodec_gating_disable(&self) -> VcodecGatingDisableR {
        VcodecGatingDisableR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn perihp_gating_disable(&self) -> PerihpGatingDisableR {
        PerihpGatingDisableR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn perilp_gating_disable(&self) -> PerilpGatingDisableR {
        PerilpGatingDisableR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn gpu_gating_disable(&self) -> GpuGatingDisableR {
        GpuGatingDisableR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn gic_gating_disable(&self) -> GicGatingDisableR {
        GicGatingDisableR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn sd_gating_disable(&self) -> SdGatingDisableR {
        SdGatingDisableR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn sdioaudio_gating_disable(&self) -> SdioaudioGatingDisableR {
        SdioaudioGatingDisableR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pmum0_gating_disable(&mut self) -> Pmum0GatingDisableW<PmuNocAutoEnaSpec> {
        Pmum0GatingDisableW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn center1_gating_disable(&mut self) -> Center1GatingDisableW<PmuNocAutoEnaSpec> {
        Center1GatingDisableW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn emmc_gating_disable(&mut self) -> EmmcGatingDisableW<PmuNocAutoEnaSpec> {
        EmmcGatingDisableW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn gmac_gating_disable(&mut self) -> GmacGatingDisableW<PmuNocAutoEnaSpec> {
        GmacGatingDisableW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn edp_gating_disable(&mut self) -> EdpGatingDisableW<PmuNocAutoEnaSpec> {
        EdpGatingDisableW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_gating_disable(&mut self) -> PmuGatingDisableW<PmuNocAutoEnaSpec> {
        PmuGatingDisableW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn alive_gating_disable(&mut self) -> AliveGatingDisableW<PmuNocAutoEnaSpec> {
        AliveGatingDisableW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn msch1_gating_disable(&mut self) -> Msch1GatingDisableW<PmuNocAutoEnaSpec> {
        Msch1GatingDisableW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn msch0_gating_disable(&mut self) -> Msch0GatingDisableW<PmuNocAutoEnaSpec> {
        Msch0GatingDisableW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn vio_gating_disable(&mut self) -> VioGatingDisableW<PmuNocAutoEnaSpec> {
        VioGatingDisableW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn ccim1_gating_disable(&mut self) -> Ccim1GatingDisableW<PmuNocAutoEnaSpec> {
        Ccim1GatingDisableW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn ccim0_gating_disable(&mut self) -> Ccim0GatingDisableW<PmuNocAutoEnaSpec> {
        Ccim0GatingDisableW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn center_gating_disable(&mut self) -> CenterGatingDisableW<PmuNocAutoEnaSpec> {
        CenterGatingDisableW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn perilpm0_gating_disable(&mut self) -> Perilpm0GatingDisableW<PmuNocAutoEnaSpec> {
        Perilpm0GatingDisableW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn usb3_gating_disable(&mut self) -> Usb3GatingDisableW<PmuNocAutoEnaSpec> {
        Usb3GatingDisableW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn hdcp_gating_disable(&mut self) -> HdcpGatingDisableW<PmuNocAutoEnaSpec> {
        HdcpGatingDisableW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn isp1_gating_disable(&mut self) -> Isp1GatingDisableW<PmuNocAutoEnaSpec> {
        Isp1GatingDisableW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn isp0_gating_disable(&mut self) -> Isp0GatingDisableW<PmuNocAutoEnaSpec> {
        Isp0GatingDisableW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn vopl_gating_disable(&mut self) -> VoplGatingDisableW<PmuNocAutoEnaSpec> {
        VoplGatingDisableW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn vopb_gating_disable(&mut self) -> VopbGatingDisableW<PmuNocAutoEnaSpec> {
        VopbGatingDisableW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn iep_gating_disable(&mut self) -> IepGatingDisableW<PmuNocAutoEnaSpec> {
        IepGatingDisableW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn rga_gating_disable(&mut self) -> RgaGatingDisableW<PmuNocAutoEnaSpec> {
        RgaGatingDisableW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn vdu_gating_disable(&mut self) -> VduGatingDisableW<PmuNocAutoEnaSpec> {
        VduGatingDisableW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn vcodec_gating_disable(&mut self) -> VcodecGatingDisableW<PmuNocAutoEnaSpec> {
        VcodecGatingDisableW::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn perihp_gating_disable(&mut self) -> PerihpGatingDisableW<PmuNocAutoEnaSpec> {
        PerihpGatingDisableW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn perilp_gating_disable(&mut self) -> PerilpGatingDisableW<PmuNocAutoEnaSpec> {
        PerilpGatingDisableW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn gpu_gating_disable(&mut self) -> GpuGatingDisableW<PmuNocAutoEnaSpec> {
        GpuGatingDisableW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn gic_gating_disable(&mut self) -> GicGatingDisableW<PmuNocAutoEnaSpec> {
        GicGatingDisableW::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn sd_gating_disable(&mut self) -> SdGatingDisableW<PmuNocAutoEnaSpec> {
        SdGatingDisableW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn sdioaudio_gating_disable(&mut self) -> SdioaudioGatingDisableW<PmuNocAutoEnaSpec> {
        SdioaudioGatingDisableW::new(self, 29)
    }
}
#[doc = "NOC auto domain clock gating disable enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_noc_auto_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_noc_auto_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuNocAutoEnaSpec;
impl crate::RegisterSpec for PmuNocAutoEnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmu_noc_auto_ena::R`](R) reader structure"]
impl crate::Readable for PmuNocAutoEnaSpec {}
#[doc = "`write(|w| ..)` method takes [`pmu_noc_auto_ena::W`](W) writer structure"]
impl crate::Writable for PmuNocAutoEnaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMU_NOC_AUTO_ENA to value 0"]
impl crate::Resettable for PmuNocAutoEnaSpec {
    const RESET_VALUE: u32 = 0;
}
