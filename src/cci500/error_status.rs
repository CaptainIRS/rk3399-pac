#[doc = "Register `ERROR_STATUS` reader"]
pub type R = crate::R<ErrorStatusSpec>;
#[doc = "Register `ERROR_STATUS` writer"]
pub type W = crate::W<ErrorStatusSpec>;
#[doc = "Imprecise error indicator for master interface 0\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImpreciseErrMst0 {
    #[doc = "0: No error from the time this bit was last reset."]
    B0 = 0,
    #[doc = "1: An error response has been received, but not signalled precisely."]
    B1 = 1,
}
impl From<ImpreciseErrMst0> for bool {
    #[inline(always)]
    fn from(variant: ImpreciseErrMst0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMPRECISE_ERR_MST0` reader - Imprecise error indicator for master interface 0"]
pub type ImpreciseErrMst0R = crate::BitReader<ImpreciseErrMst0>;
impl ImpreciseErrMst0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImpreciseErrMst0 {
        match self.bits {
            false => ImpreciseErrMst0::B0,
            true => ImpreciseErrMst0::B1,
        }
    }
    #[doc = "No error from the time this bit was last reset."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ImpreciseErrMst0::B0
    }
    #[doc = "An error response has been received, but not signalled precisely."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ImpreciseErrMst0::B1
    }
}
#[doc = "Field `IMPRECISE_ERR_MST0` writer - Imprecise error indicator for master interface 0"]
pub type ImpreciseErrMst0W<'a, REG> = crate::BitWriter<'a, REG, ImpreciseErrMst0>;
impl<'a, REG> ImpreciseErrMst0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error from the time this bit was last reset."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ImpreciseErrMst0::B0)
    }
    #[doc = "An error response has been received, but not signalled precisely."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ImpreciseErrMst0::B1)
    }
}
#[doc = "Imprecise error indicator for master interface 1\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImpreciseErrMst1 {
    #[doc = "0: No error from the time this bit was last reset."]
    B0 = 0,
    #[doc = "1: An error response has been received, but not signalled precisely."]
    B1 = 1,
}
impl From<ImpreciseErrMst1> for bool {
    #[inline(always)]
    fn from(variant: ImpreciseErrMst1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMPRECISE_ERR_MST1` reader - Imprecise error indicator for master interface 1"]
pub type ImpreciseErrMst1R = crate::BitReader<ImpreciseErrMst1>;
impl ImpreciseErrMst1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImpreciseErrMst1 {
        match self.bits {
            false => ImpreciseErrMst1::B0,
            true => ImpreciseErrMst1::B1,
        }
    }
    #[doc = "No error from the time this bit was last reset."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ImpreciseErrMst1::B0
    }
    #[doc = "An error response has been received, but not signalled precisely."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ImpreciseErrMst1::B1
    }
}
#[doc = "Field `IMPRECISE_ERR_MST1` writer - Imprecise error indicator for master interface 1"]
pub type ImpreciseErrMst1W<'a, REG> = crate::BitWriter<'a, REG, ImpreciseErrMst1>;
impl<'a, REG> ImpreciseErrMst1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error from the time this bit was last reset."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ImpreciseErrMst1::B0)
    }
    #[doc = "An error response has been received, but not signalled precisely."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ImpreciseErrMst1::B1)
    }
}
#[doc = "Imprecise error indicator for master interface 2\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImpreciseErrMst2 {
    #[doc = "0: No error from the time this bit was last reset."]
    B0 = 0,
    #[doc = "1: An error response has been received, but not signalled precisely."]
    B1 = 1,
}
impl From<ImpreciseErrMst2> for bool {
    #[inline(always)]
    fn from(variant: ImpreciseErrMst2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMPRECISE_ERR_MST2` reader - Imprecise error indicator for master interface 2"]
pub type ImpreciseErrMst2R = crate::BitReader<ImpreciseErrMst2>;
impl ImpreciseErrMst2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImpreciseErrMst2 {
        match self.bits {
            false => ImpreciseErrMst2::B0,
            true => ImpreciseErrMst2::B1,
        }
    }
    #[doc = "No error from the time this bit was last reset."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ImpreciseErrMst2::B0
    }
    #[doc = "An error response has been received, but not signalled precisely."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ImpreciseErrMst2::B1
    }
}
#[doc = "Field `IMPRECISE_ERR_MST2` writer - Imprecise error indicator for master interface 2"]
pub type ImpreciseErrMst2W<'a, REG> = crate::BitWriter<'a, REG, ImpreciseErrMst2>;
impl<'a, REG> ImpreciseErrMst2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error from the time this bit was last reset."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ImpreciseErrMst2::B0)
    }
    #[doc = "An error response has been received, but not signalled precisely."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ImpreciseErrMst2::B1)
    }
}
#[doc = "Imprecise error indicator for master interface 3\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImpreciseErrMst3 {
    #[doc = "0: No error from the time this bit was last reset."]
    B0 = 0,
    #[doc = "1: An error response has been received, but not signalled precisely."]
    B1 = 1,
}
impl From<ImpreciseErrMst3> for bool {
    #[inline(always)]
    fn from(variant: ImpreciseErrMst3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMPRECISE_ERR_MST3` reader - Imprecise error indicator for master interface 3"]
pub type ImpreciseErrMst3R = crate::BitReader<ImpreciseErrMst3>;
impl ImpreciseErrMst3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImpreciseErrMst3 {
        match self.bits {
            false => ImpreciseErrMst3::B0,
            true => ImpreciseErrMst3::B1,
        }
    }
    #[doc = "No error from the time this bit was last reset."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ImpreciseErrMst3::B0
    }
    #[doc = "An error response has been received, but not signalled precisely."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ImpreciseErrMst3::B1
    }
}
#[doc = "Field `IMPRECISE_ERR_MST3` writer - Imprecise error indicator for master interface 3"]
pub type ImpreciseErrMst3W<'a, REG> = crate::BitWriter<'a, REG, ImpreciseErrMst3>;
impl<'a, REG> ImpreciseErrMst3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error from the time this bit was last reset."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ImpreciseErrMst3::B0)
    }
    #[doc = "An error response has been received, but not signalled precisely."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ImpreciseErrMst3::B1)
    }
}
#[doc = "Imprecise error indicator for master interface 4\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImpreciseErrMst4 {
    #[doc = "0: No error from the time this bit was last reset."]
    B0 = 0,
    #[doc = "1: An error response has been received, but not signalled precisely."]
    B1 = 1,
}
impl From<ImpreciseErrMst4> for bool {
    #[inline(always)]
    fn from(variant: ImpreciseErrMst4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMPRECISE_ERR_MST4` reader - Imprecise error indicator for master interface 4"]
pub type ImpreciseErrMst4R = crate::BitReader<ImpreciseErrMst4>;
impl ImpreciseErrMst4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImpreciseErrMst4 {
        match self.bits {
            false => ImpreciseErrMst4::B0,
            true => ImpreciseErrMst4::B1,
        }
    }
    #[doc = "No error from the time this bit was last reset."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ImpreciseErrMst4::B0
    }
    #[doc = "An error response has been received, but not signalled precisely."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ImpreciseErrMst4::B1
    }
}
#[doc = "Field `IMPRECISE_ERR_MST4` writer - Imprecise error indicator for master interface 4"]
pub type ImpreciseErrMst4W<'a, REG> = crate::BitWriter<'a, REG, ImpreciseErrMst4>;
impl<'a, REG> ImpreciseErrMst4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error from the time this bit was last reset."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ImpreciseErrMst4::B0)
    }
    #[doc = "An error response has been received, but not signalled precisely."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ImpreciseErrMst4::B1)
    }
}
#[doc = "Imprecise error indicator for master interface 5\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImpreciseErrMst5 {
    #[doc = "0: No error from the time this bit was last reset."]
    B0 = 0,
    #[doc = "1: An error response has been received, but not signalled precisely."]
    B1 = 1,
}
impl From<ImpreciseErrMst5> for bool {
    #[inline(always)]
    fn from(variant: ImpreciseErrMst5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMPRECISE_ERR_MST5` reader - Imprecise error indicator for master interface 5"]
pub type ImpreciseErrMst5R = crate::BitReader<ImpreciseErrMst5>;
impl ImpreciseErrMst5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImpreciseErrMst5 {
        match self.bits {
            false => ImpreciseErrMst5::B0,
            true => ImpreciseErrMst5::B1,
        }
    }
    #[doc = "No error from the time this bit was last reset."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ImpreciseErrMst5::B0
    }
    #[doc = "An error response has been received, but not signalled precisely."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ImpreciseErrMst5::B1
    }
}
#[doc = "Field `IMPRECISE_ERR_MST5` writer - Imprecise error indicator for master interface 5"]
pub type ImpreciseErrMst5W<'a, REG> = crate::BitWriter<'a, REG, ImpreciseErrMst5>;
impl<'a, REG> ImpreciseErrMst5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error from the time this bit was last reset."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ImpreciseErrMst5::B0)
    }
    #[doc = "An error response has been received, but not signalled precisely."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ImpreciseErrMst5::B1)
    }
}
#[doc = "Imprecise error indicator for slave interface 0.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImpreciseErrSlv0 {
    #[doc = "0: No error from the time this bit was last reset."]
    B0 = 0,
    #[doc = "1: An error response has been received, but not signaled precisely."]
    B1 = 1,
}
impl From<ImpreciseErrSlv0> for bool {
    #[inline(always)]
    fn from(variant: ImpreciseErrSlv0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMPRECISE_ERR_SLV0` reader - Imprecise error indicator for slave interface 0."]
pub type ImpreciseErrSlv0R = crate::BitReader<ImpreciseErrSlv0>;
impl ImpreciseErrSlv0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImpreciseErrSlv0 {
        match self.bits {
            false => ImpreciseErrSlv0::B0,
            true => ImpreciseErrSlv0::B1,
        }
    }
    #[doc = "No error from the time this bit was last reset."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ImpreciseErrSlv0::B0
    }
    #[doc = "An error response has been received, but not signaled precisely."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ImpreciseErrSlv0::B1
    }
}
#[doc = "Field `IMPRECISE_ERR_SLV0` writer - Imprecise error indicator for slave interface 0."]
pub type ImpreciseErrSlv0W<'a, REG> = crate::BitWriter<'a, REG, ImpreciseErrSlv0>;
impl<'a, REG> ImpreciseErrSlv0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error from the time this bit was last reset."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ImpreciseErrSlv0::B0)
    }
    #[doc = "An error response has been received, but not signaled precisely."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ImpreciseErrSlv0::B1)
    }
}
#[doc = "Imprecise error indicator for slave interface 1\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImpreciseErrSlv1 {
    #[doc = "0: No error from the time this bit was last reset."]
    B0 = 0,
    #[doc = "1: An error response has been received, but not signaled precisely."]
    B1 = 1,
}
impl From<ImpreciseErrSlv1> for bool {
    #[inline(always)]
    fn from(variant: ImpreciseErrSlv1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMPRECISE_ERR_SLV1` reader - Imprecise error indicator for slave interface 1"]
pub type ImpreciseErrSlv1R = crate::BitReader<ImpreciseErrSlv1>;
impl ImpreciseErrSlv1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImpreciseErrSlv1 {
        match self.bits {
            false => ImpreciseErrSlv1::B0,
            true => ImpreciseErrSlv1::B1,
        }
    }
    #[doc = "No error from the time this bit was last reset."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ImpreciseErrSlv1::B0
    }
    #[doc = "An error response has been received, but not signaled precisely."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ImpreciseErrSlv1::B1
    }
}
#[doc = "Field `IMPRECISE_ERR_SLV1` writer - Imprecise error indicator for slave interface 1"]
pub type ImpreciseErrSlv1W<'a, REG> = crate::BitWriter<'a, REG, ImpreciseErrSlv1>;
impl<'a, REG> ImpreciseErrSlv1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error from the time this bit was last reset."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ImpreciseErrSlv1::B0)
    }
    #[doc = "An error response has been received, but not signaled precisely."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ImpreciseErrSlv1::B1)
    }
}
#[doc = "Imprecise error indicator for slave interface 2\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImpreciseErrSlv2 {
    #[doc = "0: No error from the time this bit was last reset."]
    B0 = 0,
    #[doc = "1: An error response has been received, but not signaled precisely."]
    B1 = 1,
}
impl From<ImpreciseErrSlv2> for bool {
    #[inline(always)]
    fn from(variant: ImpreciseErrSlv2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMPRECISE_ERR_SLV2` reader - Imprecise error indicator for slave interface 2"]
pub type ImpreciseErrSlv2R = crate::BitReader<ImpreciseErrSlv2>;
impl ImpreciseErrSlv2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImpreciseErrSlv2 {
        match self.bits {
            false => ImpreciseErrSlv2::B0,
            true => ImpreciseErrSlv2::B1,
        }
    }
    #[doc = "No error from the time this bit was last reset."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ImpreciseErrSlv2::B0
    }
    #[doc = "An error response has been received, but not signaled precisely."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ImpreciseErrSlv2::B1
    }
}
#[doc = "Field `IMPRECISE_ERR_SLV2` writer - Imprecise error indicator for slave interface 2"]
pub type ImpreciseErrSlv2W<'a, REG> = crate::BitWriter<'a, REG, ImpreciseErrSlv2>;
impl<'a, REG> ImpreciseErrSlv2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error from the time this bit was last reset."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ImpreciseErrSlv2::B0)
    }
    #[doc = "An error response has been received, but not signaled precisely."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ImpreciseErrSlv2::B1)
    }
}
#[doc = "Imprecise error indicator for slave interface 3\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImpreciseErrSlv3 {
    #[doc = "0: No error from the time this bit was last reset."]
    B0 = 0,
    #[doc = "1: An error response has been received, but not signaled precisely."]
    B1 = 1,
}
impl From<ImpreciseErrSlv3> for bool {
    #[inline(always)]
    fn from(variant: ImpreciseErrSlv3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMPRECISE_ERR_SLV3` reader - Imprecise error indicator for slave interface 3"]
pub type ImpreciseErrSlv3R = crate::BitReader<ImpreciseErrSlv3>;
impl ImpreciseErrSlv3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImpreciseErrSlv3 {
        match self.bits {
            false => ImpreciseErrSlv3::B0,
            true => ImpreciseErrSlv3::B1,
        }
    }
    #[doc = "No error from the time this bit was last reset."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ImpreciseErrSlv3::B0
    }
    #[doc = "An error response has been received, but not signaled precisely."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ImpreciseErrSlv3::B1
    }
}
#[doc = "Field `IMPRECISE_ERR_SLV3` writer - Imprecise error indicator for slave interface 3"]
pub type ImpreciseErrSlv3W<'a, REG> = crate::BitWriter<'a, REG, ImpreciseErrSlv3>;
impl<'a, REG> ImpreciseErrSlv3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error from the time this bit was last reset."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ImpreciseErrSlv3::B0)
    }
    #[doc = "An error response has been received, but not signaled precisely."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ImpreciseErrSlv3::B1)
    }
}
#[doc = "Imprecise error indicator for slave interface 4\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImpreciseErrSlv4 {
    #[doc = "0: No error from the time this bit was last reset."]
    B0 = 0,
    #[doc = "1: An error response has been received, but not signaled precisely."]
    B1 = 1,
}
impl From<ImpreciseErrSlv4> for bool {
    #[inline(always)]
    fn from(variant: ImpreciseErrSlv4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMPRECISE_ERR_SLV4` reader - Imprecise error indicator for slave interface 4"]
pub type ImpreciseErrSlv4R = crate::BitReader<ImpreciseErrSlv4>;
impl ImpreciseErrSlv4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImpreciseErrSlv4 {
        match self.bits {
            false => ImpreciseErrSlv4::B0,
            true => ImpreciseErrSlv4::B1,
        }
    }
    #[doc = "No error from the time this bit was last reset."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ImpreciseErrSlv4::B0
    }
    #[doc = "An error response has been received, but not signaled precisely."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ImpreciseErrSlv4::B1
    }
}
#[doc = "Field `IMPRECISE_ERR_SLV4` writer - Imprecise error indicator for slave interface 4"]
pub type ImpreciseErrSlv4W<'a, REG> = crate::BitWriter<'a, REG, ImpreciseErrSlv4>;
impl<'a, REG> ImpreciseErrSlv4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error from the time this bit was last reset."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ImpreciseErrSlv4::B0)
    }
    #[doc = "An error response has been received, but not signaled precisely."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ImpreciseErrSlv4::B1)
    }
}
#[doc = "Imprecise error indicator for slave interface 5\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImpreciseErrSlv5 {
    #[doc = "0: No error from the time this bit was last reset."]
    B0 = 0,
    #[doc = "1: An error response has been received, but not signaled precisely."]
    B1 = 1,
}
impl From<ImpreciseErrSlv5> for bool {
    #[inline(always)]
    fn from(variant: ImpreciseErrSlv5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMPRECISE_ERR_SLV5` reader - Imprecise error indicator for slave interface 5"]
pub type ImpreciseErrSlv5R = crate::BitReader<ImpreciseErrSlv5>;
impl ImpreciseErrSlv5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImpreciseErrSlv5 {
        match self.bits {
            false => ImpreciseErrSlv5::B0,
            true => ImpreciseErrSlv5::B1,
        }
    }
    #[doc = "No error from the time this bit was last reset."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ImpreciseErrSlv5::B0
    }
    #[doc = "An error response has been received, but not signaled precisely."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ImpreciseErrSlv5::B1
    }
}
#[doc = "Field `IMPRECISE_ERR_SLV5` writer - Imprecise error indicator for slave interface 5"]
pub type ImpreciseErrSlv5W<'a, REG> = crate::BitWriter<'a, REG, ImpreciseErrSlv5>;
impl<'a, REG> ImpreciseErrSlv5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error from the time this bit was last reset."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ImpreciseErrSlv5::B0)
    }
    #[doc = "An error response has been received, but not signaled precisely."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ImpreciseErrSlv5::B1)
    }
}
#[doc = "Imprecise error indicator for slave interface 6\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImpreciseErrSlv6 {
    #[doc = "0: No error from the time this bit was last reset."]
    B0 = 0,
    #[doc = "1: An error response has been received, but not signaled precisely."]
    B1 = 1,
}
impl From<ImpreciseErrSlv6> for bool {
    #[inline(always)]
    fn from(variant: ImpreciseErrSlv6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMPRECISE_ERR_SLV6` reader - Imprecise error indicator for slave interface 6"]
pub type ImpreciseErrSlv6R = crate::BitReader<ImpreciseErrSlv6>;
impl ImpreciseErrSlv6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImpreciseErrSlv6 {
        match self.bits {
            false => ImpreciseErrSlv6::B0,
            true => ImpreciseErrSlv6::B1,
        }
    }
    #[doc = "No error from the time this bit was last reset."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ImpreciseErrSlv6::B0
    }
    #[doc = "An error response has been received, but not signaled precisely."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ImpreciseErrSlv6::B1
    }
}
#[doc = "Field `IMPRECISE_ERR_SLV6` writer - Imprecise error indicator for slave interface 6"]
pub type ImpreciseErrSlv6W<'a, REG> = crate::BitWriter<'a, REG, ImpreciseErrSlv6>;
impl<'a, REG> ImpreciseErrSlv6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error from the time this bit was last reset."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ImpreciseErrSlv6::B0)
    }
    #[doc = "An error response has been received, but not signaled precisely."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ImpreciseErrSlv6::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Imprecise error indicator for master interface 0"]
    #[inline(always)]
    pub fn imprecise_err_mst0(&self) -> ImpreciseErrMst0R {
        ImpreciseErrMst0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Imprecise error indicator for master interface 1"]
    #[inline(always)]
    pub fn imprecise_err_mst1(&self) -> ImpreciseErrMst1R {
        ImpreciseErrMst1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Imprecise error indicator for master interface 2"]
    #[inline(always)]
    pub fn imprecise_err_mst2(&self) -> ImpreciseErrMst2R {
        ImpreciseErrMst2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Imprecise error indicator for master interface 3"]
    #[inline(always)]
    pub fn imprecise_err_mst3(&self) -> ImpreciseErrMst3R {
        ImpreciseErrMst3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Imprecise error indicator for master interface 4"]
    #[inline(always)]
    pub fn imprecise_err_mst4(&self) -> ImpreciseErrMst4R {
        ImpreciseErrMst4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Imprecise error indicator for master interface 5"]
    #[inline(always)]
    pub fn imprecise_err_mst5(&self) -> ImpreciseErrMst5R {
        ImpreciseErrMst5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - Imprecise error indicator for slave interface 0."]
    #[inline(always)]
    pub fn imprecise_err_slv0(&self) -> ImpreciseErrSlv0R {
        ImpreciseErrSlv0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Imprecise error indicator for slave interface 1"]
    #[inline(always)]
    pub fn imprecise_err_slv1(&self) -> ImpreciseErrSlv1R {
        ImpreciseErrSlv1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Imprecise error indicator for slave interface 2"]
    #[inline(always)]
    pub fn imprecise_err_slv2(&self) -> ImpreciseErrSlv2R {
        ImpreciseErrSlv2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Imprecise error indicator for slave interface 3"]
    #[inline(always)]
    pub fn imprecise_err_slv3(&self) -> ImpreciseErrSlv3R {
        ImpreciseErrSlv3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Imprecise error indicator for slave interface 4"]
    #[inline(always)]
    pub fn imprecise_err_slv4(&self) -> ImpreciseErrSlv4R {
        ImpreciseErrSlv4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Imprecise error indicator for slave interface 5"]
    #[inline(always)]
    pub fn imprecise_err_slv5(&self) -> ImpreciseErrSlv5R {
        ImpreciseErrSlv5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Imprecise error indicator for slave interface 6"]
    #[inline(always)]
    pub fn imprecise_err_slv6(&self) -> ImpreciseErrSlv6R {
        ImpreciseErrSlv6R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Imprecise error indicator for master interface 0"]
    #[inline(always)]
    #[must_use]
    pub fn imprecise_err_mst0(&mut self) -> ImpreciseErrMst0W<ErrorStatusSpec> {
        ImpreciseErrMst0W::new(self, 0)
    }
    #[doc = "Bit 1 - Imprecise error indicator for master interface 1"]
    #[inline(always)]
    #[must_use]
    pub fn imprecise_err_mst1(&mut self) -> ImpreciseErrMst1W<ErrorStatusSpec> {
        ImpreciseErrMst1W::new(self, 1)
    }
    #[doc = "Bit 2 - Imprecise error indicator for master interface 2"]
    #[inline(always)]
    #[must_use]
    pub fn imprecise_err_mst2(&mut self) -> ImpreciseErrMst2W<ErrorStatusSpec> {
        ImpreciseErrMst2W::new(self, 2)
    }
    #[doc = "Bit 3 - Imprecise error indicator for master interface 3"]
    #[inline(always)]
    #[must_use]
    pub fn imprecise_err_mst3(&mut self) -> ImpreciseErrMst3W<ErrorStatusSpec> {
        ImpreciseErrMst3W::new(self, 3)
    }
    #[doc = "Bit 4 - Imprecise error indicator for master interface 4"]
    #[inline(always)]
    #[must_use]
    pub fn imprecise_err_mst4(&mut self) -> ImpreciseErrMst4W<ErrorStatusSpec> {
        ImpreciseErrMst4W::new(self, 4)
    }
    #[doc = "Bit 5 - Imprecise error indicator for master interface 5"]
    #[inline(always)]
    #[must_use]
    pub fn imprecise_err_mst5(&mut self) -> ImpreciseErrMst5W<ErrorStatusSpec> {
        ImpreciseErrMst5W::new(self, 5)
    }
    #[doc = "Bit 16 - Imprecise error indicator for slave interface 0."]
    #[inline(always)]
    #[must_use]
    pub fn imprecise_err_slv0(&mut self) -> ImpreciseErrSlv0W<ErrorStatusSpec> {
        ImpreciseErrSlv0W::new(self, 16)
    }
    #[doc = "Bit 17 - Imprecise error indicator for slave interface 1"]
    #[inline(always)]
    #[must_use]
    pub fn imprecise_err_slv1(&mut self) -> ImpreciseErrSlv1W<ErrorStatusSpec> {
        ImpreciseErrSlv1W::new(self, 17)
    }
    #[doc = "Bit 18 - Imprecise error indicator for slave interface 2"]
    #[inline(always)]
    #[must_use]
    pub fn imprecise_err_slv2(&mut self) -> ImpreciseErrSlv2W<ErrorStatusSpec> {
        ImpreciseErrSlv2W::new(self, 18)
    }
    #[doc = "Bit 19 - Imprecise error indicator for slave interface 3"]
    #[inline(always)]
    #[must_use]
    pub fn imprecise_err_slv3(&mut self) -> ImpreciseErrSlv3W<ErrorStatusSpec> {
        ImpreciseErrSlv3W::new(self, 19)
    }
    #[doc = "Bit 20 - Imprecise error indicator for slave interface 4"]
    #[inline(always)]
    #[must_use]
    pub fn imprecise_err_slv4(&mut self) -> ImpreciseErrSlv4W<ErrorStatusSpec> {
        ImpreciseErrSlv4W::new(self, 20)
    }
    #[doc = "Bit 21 - Imprecise error indicator for slave interface 5"]
    #[inline(always)]
    #[must_use]
    pub fn imprecise_err_slv5(&mut self) -> ImpreciseErrSlv5W<ErrorStatusSpec> {
        ImpreciseErrSlv5W::new(self, 21)
    }
    #[doc = "Bit 22 - Imprecise error indicator for slave interface 6"]
    #[inline(always)]
    #[must_use]
    pub fn imprecise_err_slv6(&mut self) -> ImpreciseErrSlv6W<ErrorStatusSpec> {
        ImpreciseErrSlv6W::new(self, 22)
    }
}
#[doc = "Imprecise Error Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`error_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`error_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrorStatusSpec;
impl crate::RegisterSpec for ErrorStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`error_status::R`](R) reader structure"]
impl crate::Readable for ErrorStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`error_status::W`](W) writer structure"]
impl crate::Writable for ErrorStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERROR_STATUS to value 0"]
impl crate::Resettable for ErrorStatusSpec {
    const RESET_VALUE: u32 = 0;
}
