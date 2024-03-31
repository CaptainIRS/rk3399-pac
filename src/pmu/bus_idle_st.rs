#[doc = "Register `BUS_IDLE_ST` reader"]
pub type R = crate::R<BusIdleStSpec>;
#[doc = "idle status of gpu niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleGpu {
    #[doc = "0: idle status of gpu_niu is 0"]
    B0 = 0,
    #[doc = "1: idle status of gpu_niu is 1"]
    B1 = 1,
}
impl From<IdleGpu> for bool {
    #[inline(always)]
    fn from(variant: IdleGpu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_GPU` reader - idle status of gpu niu"]
pub type IdleGpuR = crate::BitReader<IdleGpu>;
impl IdleGpuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleGpu {
        match self.bits {
            false => IdleGpu::B0,
            true => IdleGpu::B1,
        }
    }
    #[doc = "idle status of gpu_niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleGpu::B0
    }
    #[doc = "idle status of gpu_niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleGpu::B1
    }
}
#[doc = "idle status of perilp niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdlePerilp {
    #[doc = "0: idle status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle status of niu is 1"]
    B1 = 1,
}
impl From<IdlePerilp> for bool {
    #[inline(always)]
    fn from(variant: IdlePerilp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_PERILP` reader - idle status of perilp niu"]
pub type IdlePerilpR = crate::BitReader<IdlePerilp>;
impl IdlePerilpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdlePerilp {
        match self.bits {
            false => IdlePerilp::B0,
            true => IdlePerilp::B1,
        }
    }
    #[doc = "idle status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdlePerilp::B0
    }
    #[doc = "idle status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdlePerilp::B1
    }
}
#[doc = "idle status of perihp niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdlePerihp {
    #[doc = "0: idle status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle status of niu is 1"]
    B1 = 1,
}
impl From<IdlePerihp> for bool {
    #[inline(always)]
    fn from(variant: IdlePerihp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_PERIHP` reader - idle status of perihp niu"]
pub type IdlePerihpR = crate::BitReader<IdlePerihp>;
impl IdlePerihpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdlePerihp {
        match self.bits {
            false => IdlePerihp::B0,
            true => IdlePerihp::B1,
        }
    }
    #[doc = "idle status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdlePerihp::B0
    }
    #[doc = "idle status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdlePerihp::B1
    }
}
#[doc = "idle status of vcodec niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleVcodec {
    #[doc = "0: idle status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle status of niu is 1"]
    B1 = 1,
}
impl From<IdleVcodec> for bool {
    #[inline(always)]
    fn from(variant: IdleVcodec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_VCODEC` reader - idle status of vcodec niu"]
pub type IdleVcodecR = crate::BitReader<IdleVcodec>;
impl IdleVcodecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleVcodec {
        match self.bits {
            false => IdleVcodec::B0,
            true => IdleVcodec::B1,
        }
    }
    #[doc = "idle status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleVcodec::B0
    }
    #[doc = "idle status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleVcodec::B1
    }
}
#[doc = "idle status of vdu niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleVdu {
    #[doc = "0: idle status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle status of niu is 1"]
    B1 = 1,
}
impl From<IdleVdu> for bool {
    #[inline(always)]
    fn from(variant: IdleVdu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_VDU` reader - idle status of vdu niu"]
pub type IdleVduR = crate::BitReader<IdleVdu>;
impl IdleVduR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleVdu {
        match self.bits {
            false => IdleVdu::B0,
            true => IdleVdu::B1,
        }
    }
    #[doc = "idle status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleVdu::B0
    }
    #[doc = "idle status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleVdu::B1
    }
}
#[doc = "idle status of rga niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleRga {
    #[doc = "0: idle status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle status of niu is 1"]
    B1 = 1,
}
impl From<IdleRga> for bool {
    #[inline(always)]
    fn from(variant: IdleRga) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_RGA` reader - idle status of rga niu"]
pub type IdleRgaR = crate::BitReader<IdleRga>;
impl IdleRgaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleRga {
        match self.bits {
            false => IdleRga::B0,
            true => IdleRga::B1,
        }
    }
    #[doc = "idle status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleRga::B0
    }
    #[doc = "idle status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleRga::B1
    }
}
#[doc = "idle status of iep niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleIep {
    #[doc = "0: idle status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle status of niu is 1"]
    B1 = 1,
}
impl From<IdleIep> for bool {
    #[inline(always)]
    fn from(variant: IdleIep) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_IEP` reader - idle status of iep niu"]
pub type IdleIepR = crate::BitReader<IdleIep>;
impl IdleIepR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleIep {
        match self.bits {
            false => IdleIep::B0,
            true => IdleIep::B1,
        }
    }
    #[doc = "idle status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleIep::B0
    }
    #[doc = "idle status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleIep::B1
    }
}
#[doc = "idle status of vopb niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleVopb {
    #[doc = "0: idle status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle status of niu is 1"]
    B1 = 1,
}
impl From<IdleVopb> for bool {
    #[inline(always)]
    fn from(variant: IdleVopb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_VOPB` reader - idle status of vopb niu"]
pub type IdleVopbR = crate::BitReader<IdleVopb>;
impl IdleVopbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleVopb {
        match self.bits {
            false => IdleVopb::B0,
            true => IdleVopb::B1,
        }
    }
    #[doc = "idle status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleVopb::B0
    }
    #[doc = "idle status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleVopb::B1
    }
}
#[doc = "idle status of vopl niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleVopl {
    #[doc = "0: idle status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle status of niu is 1"]
    B1 = 1,
}
impl From<IdleVopl> for bool {
    #[inline(always)]
    fn from(variant: IdleVopl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_VOPL` reader - idle status of vopl niu"]
pub type IdleVoplR = crate::BitReader<IdleVopl>;
impl IdleVoplR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleVopl {
        match self.bits {
            false => IdleVopl::B0,
            true => IdleVopl::B1,
        }
    }
    #[doc = "idle status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleVopl::B0
    }
    #[doc = "idle status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleVopl::B1
    }
}
#[doc = "idle status of isp0 niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleIsp0 {
    #[doc = "0: idle status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle status of niu is 1"]
    B1 = 1,
}
impl From<IdleIsp0> for bool {
    #[inline(always)]
    fn from(variant: IdleIsp0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_ISP0` reader - idle status of isp0 niu"]
pub type IdleIsp0R = crate::BitReader<IdleIsp0>;
impl IdleIsp0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleIsp0 {
        match self.bits {
            false => IdleIsp0::B0,
            true => IdleIsp0::B1,
        }
    }
    #[doc = "idle status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleIsp0::B0
    }
    #[doc = "idle status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleIsp0::B1
    }
}
#[doc = "idle status of isp1 niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleIsp1 {
    #[doc = "0: idle status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle status of niu is 1"]
    B1 = 1,
}
impl From<IdleIsp1> for bool {
    #[inline(always)]
    fn from(variant: IdleIsp1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_ISP1` reader - idle status of isp1 niu"]
pub type IdleIsp1R = crate::BitReader<IdleIsp1>;
impl IdleIsp1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleIsp1 {
        match self.bits {
            false => IdleIsp1::B0,
            true => IdleIsp1::B1,
        }
    }
    #[doc = "idle status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleIsp1::B0
    }
    #[doc = "idle status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleIsp1::B1
    }
}
#[doc = "idle status of hdcp niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleHdcp {
    #[doc = "0: idle status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle status of niu is 1"]
    B1 = 1,
}
impl From<IdleHdcp> for bool {
    #[inline(always)]
    fn from(variant: IdleHdcp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_HDCP` reader - idle status of hdcp niu"]
pub type IdleHdcpR = crate::BitReader<IdleHdcp>;
impl IdleHdcpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleHdcp {
        match self.bits {
            false => IdleHdcp::B0,
            true => IdleHdcp::B1,
        }
    }
    #[doc = "idle status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleHdcp::B0
    }
    #[doc = "idle status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleHdcp::B1
    }
}
#[doc = "idle status of usb3 niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleUsb3 {
    #[doc = "0: idle status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle status of niu is 1"]
    B1 = 1,
}
impl From<IdleUsb3> for bool {
    #[inline(always)]
    fn from(variant: IdleUsb3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_USB3` reader - idle status of usb3 niu"]
pub type IdleUsb3R = crate::BitReader<IdleUsb3>;
impl IdleUsb3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleUsb3 {
        match self.bits {
            false => IdleUsb3::B0,
            true => IdleUsb3::B1,
        }
    }
    #[doc = "idle status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleUsb3::B0
    }
    #[doc = "idle status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleUsb3::B1
    }
}
#[doc = "idle status of perilpm0 niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdlePerilpm0 {
    #[doc = "0: idle status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle status of niu is 1"]
    B1 = 1,
}
impl From<IdlePerilpm0> for bool {
    #[inline(always)]
    fn from(variant: IdlePerilpm0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_PERILPM0` reader - idle status of perilpm0 niu"]
pub type IdlePerilpm0R = crate::BitReader<IdlePerilpm0>;
impl IdlePerilpm0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdlePerilpm0 {
        match self.bits {
            false => IdlePerilpm0::B0,
            true => IdlePerilpm0::B1,
        }
    }
    #[doc = "idle status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdlePerilpm0::B0
    }
    #[doc = "idle status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdlePerilpm0::B1
    }
}
#[doc = "idle status of center niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleCenter {
    #[doc = "0: idle status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle status of niu is 1"]
    B1 = 1,
}
impl From<IdleCenter> for bool {
    #[inline(always)]
    fn from(variant: IdleCenter) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_CENTER` reader - idle status of center niu"]
pub type IdleCenterR = crate::BitReader<IdleCenter>;
impl IdleCenterR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleCenter {
        match self.bits {
            false => IdleCenter::B0,
            true => IdleCenter::B1,
        }
    }
    #[doc = "idle status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleCenter::B0
    }
    #[doc = "idle status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleCenter::B1
    }
}
#[doc = "idle status of ccim0 niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleCcim0 {
    #[doc = "0: idle status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle status of niu is 1"]
    B1 = 1,
}
impl From<IdleCcim0> for bool {
    #[inline(always)]
    fn from(variant: IdleCcim0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_CCIM0` reader - idle status of ccim0 niu"]
pub type IdleCcim0R = crate::BitReader<IdleCcim0>;
impl IdleCcim0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleCcim0 {
        match self.bits {
            false => IdleCcim0::B0,
            true => IdleCcim0::B1,
        }
    }
    #[doc = "idle status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleCcim0::B0
    }
    #[doc = "idle status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleCcim0::B1
    }
}
#[doc = "idle status of ccim1 niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleCcim1 {
    #[doc = "0: idle status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle status of niu is 1"]
    B1 = 1,
}
impl From<IdleCcim1> for bool {
    #[inline(always)]
    fn from(variant: IdleCcim1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_CCIM1` reader - idle status of ccim1 niu"]
pub type IdleCcim1R = crate::BitReader<IdleCcim1>;
impl IdleCcim1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleCcim1 {
        match self.bits {
            false => IdleCcim1::B0,
            true => IdleCcim1::B1,
        }
    }
    #[doc = "idle status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleCcim1::B0
    }
    #[doc = "idle status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleCcim1::B1
    }
}
#[doc = "idle status of vio niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleVio {
    #[doc = "0: idle status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle status of niu is 1"]
    B1 = 1,
}
impl From<IdleVio> for bool {
    #[inline(always)]
    fn from(variant: IdleVio) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_VIO` reader - idle status of vio niu"]
pub type IdleVioR = crate::BitReader<IdleVio>;
impl IdleVioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleVio {
        match self.bits {
            false => IdleVio::B0,
            true => IdleVio::B1,
        }
    }
    #[doc = "idle status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleVio::B0
    }
    #[doc = "idle status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleVio::B1
    }
}
#[doc = "idle status of msch0 niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleMsch0 {
    #[doc = "0: idle status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle status of niu is 1"]
    B1 = 1,
}
impl From<IdleMsch0> for bool {
    #[inline(always)]
    fn from(variant: IdleMsch0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_MSCH0` reader - idle status of msch0 niu"]
pub type IdleMsch0R = crate::BitReader<IdleMsch0>;
impl IdleMsch0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleMsch0 {
        match self.bits {
            false => IdleMsch0::B0,
            true => IdleMsch0::B1,
        }
    }
    #[doc = "idle status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleMsch0::B0
    }
    #[doc = "idle status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleMsch0::B1
    }
}
#[doc = "idle status of msch1 niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleMsch1 {
    #[doc = "0: idle status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle status of niu is 1"]
    B1 = 1,
}
impl From<IdleMsch1> for bool {
    #[inline(always)]
    fn from(variant: IdleMsch1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_MSCH1` reader - idle status of msch1 niu"]
pub type IdleMsch1R = crate::BitReader<IdleMsch1>;
impl IdleMsch1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleMsch1 {
        match self.bits {
            false => IdleMsch1::B0,
            true => IdleMsch1::B1,
        }
    }
    #[doc = "idle status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleMsch1::B0
    }
    #[doc = "idle status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleMsch1::B1
    }
}
#[doc = "idle status of alive niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleAlive {
    #[doc = "0: idle status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle status of niu is 1"]
    B1 = 1,
}
impl From<IdleAlive> for bool {
    #[inline(always)]
    fn from(variant: IdleAlive) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_ALIVE` reader - idle status of alive niu"]
pub type IdleAliveR = crate::BitReader<IdleAlive>;
impl IdleAliveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleAlive {
        match self.bits {
            false => IdleAlive::B0,
            true => IdleAlive::B1,
        }
    }
    #[doc = "idle status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleAlive::B0
    }
    #[doc = "idle status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleAlive::B1
    }
}
#[doc = "idle status of pmu niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdlePmu {
    #[doc = "0: idle status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle status of niu is 1"]
    B1 = 1,
}
impl From<IdlePmu> for bool {
    #[inline(always)]
    fn from(variant: IdlePmu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_PMU` reader - idle status of pmu niu"]
pub type IdlePmuR = crate::BitReader<IdlePmu>;
impl IdlePmuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdlePmu {
        match self.bits {
            false => IdlePmu::B0,
            true => IdlePmu::B1,
        }
    }
    #[doc = "idle status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdlePmu::B0
    }
    #[doc = "idle status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdlePmu::B1
    }
}
#[doc = "idle status of edp niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleEdp {
    #[doc = "0: idle status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle status of niu is 1"]
    B1 = 1,
}
impl From<IdleEdp> for bool {
    #[inline(always)]
    fn from(variant: IdleEdp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_EDP` reader - idle status of edp niu"]
pub type IdleEdpR = crate::BitReader<IdleEdp>;
impl IdleEdpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleEdp {
        match self.bits {
            false => IdleEdp::B0,
            true => IdleEdp::B1,
        }
    }
    #[doc = "idle status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleEdp::B0
    }
    #[doc = "idle status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleEdp::B1
    }
}
#[doc = "idle status of gmac niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleGmac {
    #[doc = "0: idle status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle status of niu is 1"]
    B1 = 1,
}
impl From<IdleGmac> for bool {
    #[inline(always)]
    fn from(variant: IdleGmac) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_GMAC` reader - idle status of gmac niu"]
pub type IdleGmacR = crate::BitReader<IdleGmac>;
impl IdleGmacR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleGmac {
        match self.bits {
            false => IdleGmac::B0,
            true => IdleGmac::B1,
        }
    }
    #[doc = "idle status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleGmac::B0
    }
    #[doc = "idle status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleGmac::B1
    }
}
#[doc = "idle status of emmc niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleEmmc {
    #[doc = "0: idle status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle status of niu is 1"]
    B1 = 1,
}
impl From<IdleEmmc> for bool {
    #[inline(always)]
    fn from(variant: IdleEmmc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_EMMC` reader - idle status of emmc niu"]
pub type IdleEmmcR = crate::BitReader<IdleEmmc>;
impl IdleEmmcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleEmmc {
        match self.bits {
            false => IdleEmmc::B0,
            true => IdleEmmc::B1,
        }
    }
    #[doc = "idle status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleEmmc::B0
    }
    #[doc = "idle status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleEmmc::B1
    }
}
#[doc = "idle status of center1 niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleCenter1 {
    #[doc = "0: idle status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle status of niu is 1"]
    B1 = 1,
}
impl From<IdleCenter1> for bool {
    #[inline(always)]
    fn from(variant: IdleCenter1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_CENTER1` reader - idle status of center1 niu"]
pub type IdleCenter1R = crate::BitReader<IdleCenter1>;
impl IdleCenter1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleCenter1 {
        match self.bits {
            false => IdleCenter1::B0,
            true => IdleCenter1::B1,
        }
    }
    #[doc = "idle status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleCenter1::B0
    }
    #[doc = "idle status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleCenter1::B1
    }
}
#[doc = "idle status of pmu m0 niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdlePmum0 {
    #[doc = "0: idle status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle status of niu is 1"]
    B1 = 1,
}
impl From<IdlePmum0> for bool {
    #[inline(always)]
    fn from(variant: IdlePmum0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_PMUM0` reader - idle status of pmu m0 niu"]
pub type IdlePmum0R = crate::BitReader<IdlePmum0>;
impl IdlePmum0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdlePmum0 {
        match self.bits {
            false => IdlePmum0::B0,
            true => IdlePmum0::B1,
        }
    }
    #[doc = "idle status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdlePmum0::B0
    }
    #[doc = "idle status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdlePmum0::B1
    }
}
#[doc = "idle status of gic niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleGic {
    #[doc = "0: idle status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle status of niu is 1"]
    B1 = 1,
}
impl From<IdleGic> for bool {
    #[inline(always)]
    fn from(variant: IdleGic) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_GIC` reader - idle status of gic niu"]
pub type IdleGicR = crate::BitReader<IdleGic>;
impl IdleGicR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleGic {
        match self.bits {
            false => IdleGic::B0,
            true => IdleGic::B1,
        }
    }
    #[doc = "idle status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleGic::B0
    }
    #[doc = "idle status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleGic::B1
    }
}
#[doc = "send idle request to sd low power interface\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleSd {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IdleSd> for bool {
    #[inline(always)]
    fn from(variant: IdleSd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_SD` reader - send idle request to sd low power interface"]
pub type IdleSdR = crate::BitReader<IdleSd>;
impl IdleSdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleSd {
        match self.bits {
            false => IdleSd::B0,
            true => IdleSd::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleSd::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleSd::B1
    }
}
#[doc = "send idle request to sdioaudio low power interface\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleSdioaudio {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IdleSdioaudio> for bool {
    #[inline(always)]
    fn from(variant: IdleSdioaudio) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_SDIOAUDIO` reader - send idle request to sdioaudio low power interface"]
pub type IdleSdioaudioR = crate::BitReader<IdleSdioaudio>;
impl IdleSdioaudioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleSdioaudio {
        match self.bits {
            false => IdleSdioaudio::B0,
            true => IdleSdioaudio::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleSdioaudio::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleSdioaudio::B1
    }
}
impl R {
    #[doc = "Bit 0 - idle status of gpu niu"]
    #[inline(always)]
    pub fn idle_gpu(&self) -> IdleGpuR {
        IdleGpuR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - idle status of perilp niu"]
    #[inline(always)]
    pub fn idle_perilp(&self) -> IdlePerilpR {
        IdlePerilpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - idle status of perihp niu"]
    #[inline(always)]
    pub fn idle_perihp(&self) -> IdlePerihpR {
        IdlePerihpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - idle status of vcodec niu"]
    #[inline(always)]
    pub fn idle_vcodec(&self) -> IdleVcodecR {
        IdleVcodecR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - idle status of vdu niu"]
    #[inline(always)]
    pub fn idle_vdu(&self) -> IdleVduR {
        IdleVduR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - idle status of rga niu"]
    #[inline(always)]
    pub fn idle_rga(&self) -> IdleRgaR {
        IdleRgaR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - idle status of iep niu"]
    #[inline(always)]
    pub fn idle_iep(&self) -> IdleIepR {
        IdleIepR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - idle status of vopb niu"]
    #[inline(always)]
    pub fn idle_vopb(&self) -> IdleVopbR {
        IdleVopbR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - idle status of vopl niu"]
    #[inline(always)]
    pub fn idle_vopl(&self) -> IdleVoplR {
        IdleVoplR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - idle status of isp0 niu"]
    #[inline(always)]
    pub fn idle_isp0(&self) -> IdleIsp0R {
        IdleIsp0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - idle status of isp1 niu"]
    #[inline(always)]
    pub fn idle_isp1(&self) -> IdleIsp1R {
        IdleIsp1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - idle status of hdcp niu"]
    #[inline(always)]
    pub fn idle_hdcp(&self) -> IdleHdcpR {
        IdleHdcpR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - idle status of usb3 niu"]
    #[inline(always)]
    pub fn idle_usb3(&self) -> IdleUsb3R {
        IdleUsb3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - idle status of perilpm0 niu"]
    #[inline(always)]
    pub fn idle_perilpm0(&self) -> IdlePerilpm0R {
        IdlePerilpm0R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - idle status of center niu"]
    #[inline(always)]
    pub fn idle_center(&self) -> IdleCenterR {
        IdleCenterR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - idle status of ccim0 niu"]
    #[inline(always)]
    pub fn idle_ccim0(&self) -> IdleCcim0R {
        IdleCcim0R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - idle status of ccim1 niu"]
    #[inline(always)]
    pub fn idle_ccim1(&self) -> IdleCcim1R {
        IdleCcim1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - idle status of vio niu"]
    #[inline(always)]
    pub fn idle_vio(&self) -> IdleVioR {
        IdleVioR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - idle status of msch0 niu"]
    #[inline(always)]
    pub fn idle_msch0(&self) -> IdleMsch0R {
        IdleMsch0R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - idle status of msch1 niu"]
    #[inline(always)]
    pub fn idle_msch1(&self) -> IdleMsch1R {
        IdleMsch1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - idle status of alive niu"]
    #[inline(always)]
    pub fn idle_alive(&self) -> IdleAliveR {
        IdleAliveR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - idle status of pmu niu"]
    #[inline(always)]
    pub fn idle_pmu(&self) -> IdlePmuR {
        IdlePmuR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - idle status of edp niu"]
    #[inline(always)]
    pub fn idle_edp(&self) -> IdleEdpR {
        IdleEdpR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - idle status of gmac niu"]
    #[inline(always)]
    pub fn idle_gmac(&self) -> IdleGmacR {
        IdleGmacR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - idle status of emmc niu"]
    #[inline(always)]
    pub fn idle_emmc(&self) -> IdleEmmcR {
        IdleEmmcR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - idle status of center1 niu"]
    #[inline(always)]
    pub fn idle_center1(&self) -> IdleCenter1R {
        IdleCenter1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - idle status of pmu m0 niu"]
    #[inline(always)]
    pub fn idle_pmum0(&self) -> IdlePmum0R {
        IdlePmum0R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - idle status of gic niu"]
    #[inline(always)]
    pub fn idle_gic(&self) -> IdleGicR {
        IdleGicR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - send idle request to sd low power interface"]
    #[inline(always)]
    pub fn idle_sd(&self) -> IdleSdR {
        IdleSdR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - send idle request to sdioaudio low power interface"]
    #[inline(always)]
    pub fn idle_sdioaudio(&self) -> IdleSdioaudioR {
        IdleSdioaudioR::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "pmu bus idle status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus_idle_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BusIdleStSpec;
impl crate::RegisterSpec for BusIdleStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus_idle_st::R`](R) reader structure"]
impl crate::Readable for BusIdleStSpec {}
#[doc = "`reset()` method sets BUS_IDLE_ST to value 0"]
impl crate::Resettable for BusIdleStSpec {
    const RESET_VALUE: u32 = 0;
}
