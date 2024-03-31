#[doc = "Register `SIG_DETECT_CLR` reader"]
pub type R = crate::R<SigDetectClrSpec>;
#[doc = "Register `SIG_DETECT_CLR` writer"]
pub type W = crate::W<SigDetectClrSpec>;
#[doc = "sdmmc card rise edge detect control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SdmmcCardRiseEdge {
    #[doc = "0: disbale"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<SdmmcCardRiseEdge> for bool {
    #[inline(always)]
    fn from(variant: SdmmcCardRiseEdge) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDMMC_CARD_RISE_EDGE` reader - sdmmc card rise edge detect control"]
pub type SdmmcCardRiseEdgeR = crate::BitReader<SdmmcCardRiseEdge>;
impl SdmmcCardRiseEdgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SdmmcCardRiseEdge {
        match self.bits {
            false => SdmmcCardRiseEdge::B0,
            true => SdmmcCardRiseEdge::B1,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SdmmcCardRiseEdge::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SdmmcCardRiseEdge::B1
    }
}
#[doc = "Field `SDMMC_CARD_RISE_EDGE` writer - sdmmc card rise edge detect control"]
pub type SdmmcCardRiseEdgeW<'a, REG> = crate::BitWriter<'a, REG, SdmmcCardRiseEdge>;
impl<'a, REG> SdmmcCardRiseEdgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SdmmcCardRiseEdge::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SdmmcCardRiseEdge::B1)
    }
}
#[doc = "sdmmc card fall edge detect control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SdmmcCardFallEdge {
    #[doc = "0: disbale"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<SdmmcCardFallEdge> for bool {
    #[inline(always)]
    fn from(variant: SdmmcCardFallEdge) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDMMC_CARD_FALL_EDGE` reader - sdmmc card fall edge detect control"]
pub type SdmmcCardFallEdgeR = crate::BitReader<SdmmcCardFallEdge>;
impl SdmmcCardFallEdgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SdmmcCardFallEdge {
        match self.bits {
            false => SdmmcCardFallEdge::B0,
            true => SdmmcCardFallEdge::B1,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SdmmcCardFallEdge::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SdmmcCardFallEdge::B1
    }
}
#[doc = "Field `SDMMC_CARD_FALL_EDGE` writer - sdmmc card fall edge detect control"]
pub type SdmmcCardFallEdgeW<'a, REG> = crate::BitWriter<'a, REG, SdmmcCardFallEdge>;
impl<'a, REG> SdmmcCardFallEdgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SdmmcCardFallEdge::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SdmmcCardFallEdge::B1)
    }
}
#[doc = "cphy0_otg_linestate_change detect control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cphy0OtgLinestateChange {
    #[doc = "0: disbale"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Cphy0OtgLinestateChange> for bool {
    #[inline(always)]
    fn from(variant: Cphy0OtgLinestateChange) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPHY0_OTG_LINESTATE_CHANGE` reader - cphy0_otg_linestate_change detect control"]
pub type Cphy0OtgLinestateChangeR = crate::BitReader<Cphy0OtgLinestateChange>;
impl Cphy0OtgLinestateChangeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cphy0OtgLinestateChange {
        match self.bits {
            false => Cphy0OtgLinestateChange::B0,
            true => Cphy0OtgLinestateChange::B1,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cphy0OtgLinestateChange::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cphy0OtgLinestateChange::B1
    }
}
#[doc = "Field `CPHY0_OTG_LINESTATE_CHANGE` writer - cphy0_otg_linestate_change detect control"]
pub type Cphy0OtgLinestateChangeW<'a, REG> = crate::BitWriter<'a, REG, Cphy0OtgLinestateChange>;
impl<'a, REG> Cphy0OtgLinestateChangeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cphy0OtgLinestateChange::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cphy0OtgLinestateChange::B1)
    }
}
#[doc = "cphy0_otg_bvalid_rise detect control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cphy0OtgBvalidRise {
    #[doc = "0: disbale"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Cphy0OtgBvalidRise> for bool {
    #[inline(always)]
    fn from(variant: Cphy0OtgBvalidRise) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPHY0_OTG_BVALID_RISE` reader - cphy0_otg_bvalid_rise detect control"]
pub type Cphy0OtgBvalidRiseR = crate::BitReader<Cphy0OtgBvalidRise>;
impl Cphy0OtgBvalidRiseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cphy0OtgBvalidRise {
        match self.bits {
            false => Cphy0OtgBvalidRise::B0,
            true => Cphy0OtgBvalidRise::B1,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cphy0OtgBvalidRise::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cphy0OtgBvalidRise::B1
    }
}
#[doc = "Field `CPHY0_OTG_BVALID_RISE` writer - cphy0_otg_bvalid_rise detect control"]
pub type Cphy0OtgBvalidRiseW<'a, REG> = crate::BitWriter<'a, REG, Cphy0OtgBvalidRise>;
impl<'a, REG> Cphy0OtgBvalidRiseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cphy0OtgBvalidRise::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cphy0OtgBvalidRise::B1)
    }
}
#[doc = "cphy0_otg_id_rise detect control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cphy0OtgIdRise {
    #[doc = "0: disbale"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Cphy0OtgIdRise> for bool {
    #[inline(always)]
    fn from(variant: Cphy0OtgIdRise) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPHY0_OTG_ID_RISE` reader - cphy0_otg_id_rise detect control"]
pub type Cphy0OtgIdRiseR = crate::BitReader<Cphy0OtgIdRise>;
impl Cphy0OtgIdRiseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cphy0OtgIdRise {
        match self.bits {
            false => Cphy0OtgIdRise::B0,
            true => Cphy0OtgIdRise::B1,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cphy0OtgIdRise::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cphy0OtgIdRise::B1
    }
}
#[doc = "Field `CPHY0_OTG_ID_RISE` writer - cphy0_otg_id_rise detect control"]
pub type Cphy0OtgIdRiseW<'a, REG> = crate::BitWriter<'a, REG, Cphy0OtgIdRise>;
impl<'a, REG> Cphy0OtgIdRiseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cphy0OtgIdRise::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cphy0OtgIdRise::B1)
    }
}
#[doc = "cphy0_otg_id_fall detect control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cphy0OtgIdFall {
    #[doc = "0: disbale"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Cphy0OtgIdFall> for bool {
    #[inline(always)]
    fn from(variant: Cphy0OtgIdFall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPHY0_OTG_ID_FALL` reader - cphy0_otg_id_fall detect control"]
pub type Cphy0OtgIdFallR = crate::BitReader<Cphy0OtgIdFall>;
impl Cphy0OtgIdFallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cphy0OtgIdFall {
        match self.bits {
            false => Cphy0OtgIdFall::B0,
            true => Cphy0OtgIdFall::B1,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cphy0OtgIdFall::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cphy0OtgIdFall::B1
    }
}
#[doc = "Field `CPHY0_OTG_ID_FALL` writer - cphy0_otg_id_fall detect control"]
pub type Cphy0OtgIdFallW<'a, REG> = crate::BitWriter<'a, REG, Cphy0OtgIdFall>;
impl<'a, REG> Cphy0OtgIdFallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cphy0OtgIdFall::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cphy0OtgIdFall::B1)
    }
}
#[doc = "cphy0_host_linestate_change detect control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cphy0HostLinestateChange {
    #[doc = "0: disbale"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Cphy0HostLinestateChange> for bool {
    #[inline(always)]
    fn from(variant: Cphy0HostLinestateChange) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPHY0_HOST_LINESTATE_CHANGE` reader - cphy0_host_linestate_change detect control"]
pub type Cphy0HostLinestateChangeR = crate::BitReader<Cphy0HostLinestateChange>;
impl Cphy0HostLinestateChangeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cphy0HostLinestateChange {
        match self.bits {
            false => Cphy0HostLinestateChange::B0,
            true => Cphy0HostLinestateChange::B1,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cphy0HostLinestateChange::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cphy0HostLinestateChange::B1
    }
}
#[doc = "Field `CPHY0_HOST_LINESTATE_CHANGE` writer - cphy0_host_linestate_change detect control"]
pub type Cphy0HostLinestateChangeW<'a, REG> = crate::BitWriter<'a, REG, Cphy0HostLinestateChange>;
impl<'a, REG> Cphy0HostLinestateChangeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cphy0HostLinestateChange::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cphy0HostLinestateChange::B1)
    }
}
#[doc = "cphy1_otg_linestate_change detect control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cphy1OtgLinestateChange {
    #[doc = "0: disbale"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Cphy1OtgLinestateChange> for bool {
    #[inline(always)]
    fn from(variant: Cphy1OtgLinestateChange) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPHY1_OTG_LINESTATE_CHANGE` reader - cphy1_otg_linestate_change detect control"]
pub type Cphy1OtgLinestateChangeR = crate::BitReader<Cphy1OtgLinestateChange>;
impl Cphy1OtgLinestateChangeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cphy1OtgLinestateChange {
        match self.bits {
            false => Cphy1OtgLinestateChange::B0,
            true => Cphy1OtgLinestateChange::B1,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cphy1OtgLinestateChange::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cphy1OtgLinestateChange::B1
    }
}
#[doc = "Field `CPHY1_OTG_LINESTATE_CHANGE` writer - cphy1_otg_linestate_change detect control"]
pub type Cphy1OtgLinestateChangeW<'a, REG> = crate::BitWriter<'a, REG, Cphy1OtgLinestateChange>;
impl<'a, REG> Cphy1OtgLinestateChangeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cphy1OtgLinestateChange::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cphy1OtgLinestateChange::B1)
    }
}
#[doc = "cphy1_otg_bvalid_rise detect control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cphy1OtgBvalidRise {
    #[doc = "0: disbale"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Cphy1OtgBvalidRise> for bool {
    #[inline(always)]
    fn from(variant: Cphy1OtgBvalidRise) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPHY1_OTG_BVALID_RISE` reader - cphy1_otg_bvalid_rise detect control"]
pub type Cphy1OtgBvalidRiseR = crate::BitReader<Cphy1OtgBvalidRise>;
impl Cphy1OtgBvalidRiseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cphy1OtgBvalidRise {
        match self.bits {
            false => Cphy1OtgBvalidRise::B0,
            true => Cphy1OtgBvalidRise::B1,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cphy1OtgBvalidRise::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cphy1OtgBvalidRise::B1
    }
}
#[doc = "Field `CPHY1_OTG_BVALID_RISE` writer - cphy1_otg_bvalid_rise detect control"]
pub type Cphy1OtgBvalidRiseW<'a, REG> = crate::BitWriter<'a, REG, Cphy1OtgBvalidRise>;
impl<'a, REG> Cphy1OtgBvalidRiseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cphy1OtgBvalidRise::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cphy1OtgBvalidRise::B1)
    }
}
#[doc = "cphy1_otg_id_rise detect control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cphy1OtgIdRise {
    #[doc = "0: disbale"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Cphy1OtgIdRise> for bool {
    #[inline(always)]
    fn from(variant: Cphy1OtgIdRise) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPHY1_OTG_ID_RISE` reader - cphy1_otg_id_rise detect control"]
pub type Cphy1OtgIdRiseR = crate::BitReader<Cphy1OtgIdRise>;
impl Cphy1OtgIdRiseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cphy1OtgIdRise {
        match self.bits {
            false => Cphy1OtgIdRise::B0,
            true => Cphy1OtgIdRise::B1,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cphy1OtgIdRise::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cphy1OtgIdRise::B1
    }
}
#[doc = "Field `CPHY1_OTG_ID_RISE` writer - cphy1_otg_id_rise detect control"]
pub type Cphy1OtgIdRiseW<'a, REG> = crate::BitWriter<'a, REG, Cphy1OtgIdRise>;
impl<'a, REG> Cphy1OtgIdRiseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cphy1OtgIdRise::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cphy1OtgIdRise::B1)
    }
}
#[doc = "cphy1_otg_id_fall detect control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cphy1OtgIdFall {
    #[doc = "0: disbale"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Cphy1OtgIdFall> for bool {
    #[inline(always)]
    fn from(variant: Cphy1OtgIdFall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPHY1_OTG_ID_FALL` reader - cphy1_otg_id_fall detect control"]
pub type Cphy1OtgIdFallR = crate::BitReader<Cphy1OtgIdFall>;
impl Cphy1OtgIdFallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cphy1OtgIdFall {
        match self.bits {
            false => Cphy1OtgIdFall::B0,
            true => Cphy1OtgIdFall::B1,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cphy1OtgIdFall::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cphy1OtgIdFall::B1
    }
}
#[doc = "Field `CPHY1_OTG_ID_FALL` writer - cphy1_otg_id_fall detect control"]
pub type Cphy1OtgIdFallW<'a, REG> = crate::BitWriter<'a, REG, Cphy1OtgIdFall>;
impl<'a, REG> Cphy1OtgIdFallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cphy1OtgIdFall::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cphy1OtgIdFall::B1)
    }
}
#[doc = "cphy1_host_linestate_change detect control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cphy1HostLinestateChange {
    #[doc = "0: disbale"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Cphy1HostLinestateChange> for bool {
    #[inline(always)]
    fn from(variant: Cphy1HostLinestateChange) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPHY1_HOST_LINESTATE_CHANGE` reader - cphy1_host_linestate_change detect control"]
pub type Cphy1HostLinestateChangeR = crate::BitReader<Cphy1HostLinestateChange>;
impl Cphy1HostLinestateChangeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cphy1HostLinestateChange {
        match self.bits {
            false => Cphy1HostLinestateChange::B0,
            true => Cphy1HostLinestateChange::B1,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cphy1HostLinestateChange::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cphy1HostLinestateChange::B1
    }
}
#[doc = "Field `CPHY1_HOST_LINESTATE_CHANGE` writer - cphy1_host_linestate_change detect control"]
pub type Cphy1HostLinestateChangeW<'a, REG> = crate::BitWriter<'a, REG, Cphy1HostLinestateChange>;
impl<'a, REG> Cphy1HostLinestateChangeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cphy1HostLinestateChange::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cphy1HostLinestateChange::B1)
    }
}
#[doc = "uphy0_rxdet_change detect control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uphy0RxdetChange {
    #[doc = "0: disbale"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Uphy0RxdetChange> for bool {
    #[inline(always)]
    fn from(variant: Uphy0RxdetChange) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPHY0_RXDET_CHANGE` reader - uphy0_rxdet_change detect control"]
pub type Uphy0RxdetChangeR = crate::BitReader<Uphy0RxdetChange>;
impl Uphy0RxdetChangeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uphy0RxdetChange {
        match self.bits {
            false => Uphy0RxdetChange::B0,
            true => Uphy0RxdetChange::B1,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Uphy0RxdetChange::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Uphy0RxdetChange::B1
    }
}
#[doc = "Field `UPHY0_RXDET_CHANGE` writer - uphy0_rxdet_change detect control"]
pub type Uphy0RxdetChangeW<'a, REG> = crate::BitWriter<'a, REG, Uphy0RxdetChange>;
impl<'a, REG> Uphy0RxdetChangeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Uphy0RxdetChange::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Uphy0RxdetChange::B1)
    }
}
#[doc = "uphy1_rxdet_change detect control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uphy1RxdetChange {
    #[doc = "0: disbale"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Uphy1RxdetChange> for bool {
    #[inline(always)]
    fn from(variant: Uphy1RxdetChange) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPHY1_RXDET_CHANGE` reader - uphy1_rxdet_change detect control"]
pub type Uphy1RxdetChangeR = crate::BitReader<Uphy1RxdetChange>;
impl Uphy1RxdetChangeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uphy1RxdetChange {
        match self.bits {
            false => Uphy1RxdetChange::B0,
            true => Uphy1RxdetChange::B1,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Uphy1RxdetChange::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Uphy1RxdetChange::B1
    }
}
#[doc = "Field `UPHY1_RXDET_CHANGE` writer - uphy1_rxdet_change detect control"]
pub type Uphy1RxdetChangeW<'a, REG> = crate::BitWriter<'a, REG, Uphy1RxdetChange>;
impl<'a, REG> Uphy1RxdetChangeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Uphy1RxdetChange::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Uphy1RxdetChange::B1)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - sdmmc card rise edge detect control"]
    #[inline(always)]
    pub fn sdmmc_card_rise_edge(&self) -> SdmmcCardRiseEdgeR {
        SdmmcCardRiseEdgeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - sdmmc card fall edge detect control"]
    #[inline(always)]
    pub fn sdmmc_card_fall_edge(&self) -> SdmmcCardFallEdgeR {
        SdmmcCardFallEdgeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - cphy0_otg_linestate_change detect control"]
    #[inline(always)]
    pub fn cphy0_otg_linestate_change(&self) -> Cphy0OtgLinestateChangeR {
        Cphy0OtgLinestateChangeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - cphy0_otg_bvalid_rise detect control"]
    #[inline(always)]
    pub fn cphy0_otg_bvalid_rise(&self) -> Cphy0OtgBvalidRiseR {
        Cphy0OtgBvalidRiseR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - cphy0_otg_id_rise detect control"]
    #[inline(always)]
    pub fn cphy0_otg_id_rise(&self) -> Cphy0OtgIdRiseR {
        Cphy0OtgIdRiseR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - cphy0_otg_id_fall detect control"]
    #[inline(always)]
    pub fn cphy0_otg_id_fall(&self) -> Cphy0OtgIdFallR {
        Cphy0OtgIdFallR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - cphy0_host_linestate_change detect control"]
    #[inline(always)]
    pub fn cphy0_host_linestate_change(&self) -> Cphy0HostLinestateChangeR {
        Cphy0HostLinestateChangeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - cphy1_otg_linestate_change detect control"]
    #[inline(always)]
    pub fn cphy1_otg_linestate_change(&self) -> Cphy1OtgLinestateChangeR {
        Cphy1OtgLinestateChangeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - cphy1_otg_bvalid_rise detect control"]
    #[inline(always)]
    pub fn cphy1_otg_bvalid_rise(&self) -> Cphy1OtgBvalidRiseR {
        Cphy1OtgBvalidRiseR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - cphy1_otg_id_rise detect control"]
    #[inline(always)]
    pub fn cphy1_otg_id_rise(&self) -> Cphy1OtgIdRiseR {
        Cphy1OtgIdRiseR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - cphy1_otg_id_fall detect control"]
    #[inline(always)]
    pub fn cphy1_otg_id_fall(&self) -> Cphy1OtgIdFallR {
        Cphy1OtgIdFallR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - cphy1_host_linestate_change detect control"]
    #[inline(always)]
    pub fn cphy1_host_linestate_change(&self) -> Cphy1HostLinestateChangeR {
        Cphy1HostLinestateChangeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - uphy0_rxdet_change detect control"]
    #[inline(always)]
    pub fn uphy0_rxdet_change(&self) -> Uphy0RxdetChangeR {
        Uphy0RxdetChangeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - uphy1_rxdet_change detect control"]
    #[inline(always)]
    pub fn uphy1_rxdet_change(&self) -> Uphy1RxdetChangeR {
        Uphy1RxdetChangeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - sdmmc card rise edge detect control"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc_card_rise_edge(&mut self) -> SdmmcCardRiseEdgeW<SigDetectClrSpec> {
        SdmmcCardRiseEdgeW::new(self, 0)
    }
    #[doc = "Bit 1 - sdmmc card fall edge detect control"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc_card_fall_edge(&mut self) -> SdmmcCardFallEdgeW<SigDetectClrSpec> {
        SdmmcCardFallEdgeW::new(self, 1)
    }
    #[doc = "Bit 2 - cphy0_otg_linestate_change detect control"]
    #[inline(always)]
    #[must_use]
    pub fn cphy0_otg_linestate_change(&mut self) -> Cphy0OtgLinestateChangeW<SigDetectClrSpec> {
        Cphy0OtgLinestateChangeW::new(self, 2)
    }
    #[doc = "Bit 3 - cphy0_otg_bvalid_rise detect control"]
    #[inline(always)]
    #[must_use]
    pub fn cphy0_otg_bvalid_rise(&mut self) -> Cphy0OtgBvalidRiseW<SigDetectClrSpec> {
        Cphy0OtgBvalidRiseW::new(self, 3)
    }
    #[doc = "Bit 4 - cphy0_otg_id_rise detect control"]
    #[inline(always)]
    #[must_use]
    pub fn cphy0_otg_id_rise(&mut self) -> Cphy0OtgIdRiseW<SigDetectClrSpec> {
        Cphy0OtgIdRiseW::new(self, 4)
    }
    #[doc = "Bit 5 - cphy0_otg_id_fall detect control"]
    #[inline(always)]
    #[must_use]
    pub fn cphy0_otg_id_fall(&mut self) -> Cphy0OtgIdFallW<SigDetectClrSpec> {
        Cphy0OtgIdFallW::new(self, 5)
    }
    #[doc = "Bit 6 - cphy0_host_linestate_change detect control"]
    #[inline(always)]
    #[must_use]
    pub fn cphy0_host_linestate_change(&mut self) -> Cphy0HostLinestateChangeW<SigDetectClrSpec> {
        Cphy0HostLinestateChangeW::new(self, 6)
    }
    #[doc = "Bit 7 - cphy1_otg_linestate_change detect control"]
    #[inline(always)]
    #[must_use]
    pub fn cphy1_otg_linestate_change(&mut self) -> Cphy1OtgLinestateChangeW<SigDetectClrSpec> {
        Cphy1OtgLinestateChangeW::new(self, 7)
    }
    #[doc = "Bit 8 - cphy1_otg_bvalid_rise detect control"]
    #[inline(always)]
    #[must_use]
    pub fn cphy1_otg_bvalid_rise(&mut self) -> Cphy1OtgBvalidRiseW<SigDetectClrSpec> {
        Cphy1OtgBvalidRiseW::new(self, 8)
    }
    #[doc = "Bit 9 - cphy1_otg_id_rise detect control"]
    #[inline(always)]
    #[must_use]
    pub fn cphy1_otg_id_rise(&mut self) -> Cphy1OtgIdRiseW<SigDetectClrSpec> {
        Cphy1OtgIdRiseW::new(self, 9)
    }
    #[doc = "Bit 10 - cphy1_otg_id_fall detect control"]
    #[inline(always)]
    #[must_use]
    pub fn cphy1_otg_id_fall(&mut self) -> Cphy1OtgIdFallW<SigDetectClrSpec> {
        Cphy1OtgIdFallW::new(self, 10)
    }
    #[doc = "Bit 11 - cphy1_host_linestate_change detect control"]
    #[inline(always)]
    #[must_use]
    pub fn cphy1_host_linestate_change(&mut self) -> Cphy1HostLinestateChangeW<SigDetectClrSpec> {
        Cphy1HostLinestateChangeW::new(self, 11)
    }
    #[doc = "Bit 12 - uphy0_rxdet_change detect control"]
    #[inline(always)]
    #[must_use]
    pub fn uphy0_rxdet_change(&mut self) -> Uphy0RxdetChangeW<SigDetectClrSpec> {
        Uphy0RxdetChangeW::new(self, 12)
    }
    #[doc = "Bit 13 - uphy1_rxdet_change detect control"]
    #[inline(always)]
    #[must_use]
    pub fn uphy1_rxdet_change(&mut self) -> Uphy1RxdetChangeW<SigDetectClrSpec> {
        Uphy1RxdetChangeW::new(self, 13)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<SigDetectClrSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "Signal detect status clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sig_detect_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sig_detect_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SigDetectClrSpec;
impl crate::RegisterSpec for SigDetectClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sig_detect_clr::R`](R) reader structure"]
impl crate::Readable for SigDetectClrSpec {}
#[doc = "`write(|w| ..)` method takes [`sig_detect_clr::W`](W) writer structure"]
impl crate::Writable for SigDetectClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIG_DETECT_CLR to value 0"]
impl crate::Resettable for SigDetectClrSpec {
    const RESET_VALUE: u32 = 0;
}
