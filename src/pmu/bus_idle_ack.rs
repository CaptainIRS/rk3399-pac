#[doc = "Register `BUS_IDLE_ACK` reader"]
pub type R = crate::R<BusIdleAckSpec>;
#[doc = "idle acknowledge status from gpu niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleAckGpu {
    #[doc = "0: idle acknowledge status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle acknowledge status of niu is 1"]
    B1 = 1,
}
impl From<IdleAckGpu> for bool {
    #[inline(always)]
    fn from(variant: IdleAckGpu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_ACK_GPU` reader - idle acknowledge status from gpu niu"]
pub type IdleAckGpuR = crate::BitReader<IdleAckGpu>;
impl IdleAckGpuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleAckGpu {
        match self.bits {
            false => IdleAckGpu::B0,
            true => IdleAckGpu::B1,
        }
    }
    #[doc = "idle acknowledge status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleAckGpu::B0
    }
    #[doc = "idle acknowledge status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleAckGpu::B1
    }
}
#[doc = "idle acknowledge status from perilp niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleAckPerilp {
    #[doc = "0: idle acknowledge status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle acknowledge status of niu is 1"]
    B1 = 1,
}
impl From<IdleAckPerilp> for bool {
    #[inline(always)]
    fn from(variant: IdleAckPerilp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_ACK_PERILP` reader - idle acknowledge status from perilp niu"]
pub type IdleAckPerilpR = crate::BitReader<IdleAckPerilp>;
impl IdleAckPerilpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleAckPerilp {
        match self.bits {
            false => IdleAckPerilp::B0,
            true => IdleAckPerilp::B1,
        }
    }
    #[doc = "idle acknowledge status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleAckPerilp::B0
    }
    #[doc = "idle acknowledge status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleAckPerilp::B1
    }
}
#[doc = "idle acknowledge status from perihp niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleAckPerihp {
    #[doc = "0: idle acknowledge status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle acknowledge status of niu is 1"]
    B1 = 1,
}
impl From<IdleAckPerihp> for bool {
    #[inline(always)]
    fn from(variant: IdleAckPerihp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_ACK_PERIHP` reader - idle acknowledge status from perihp niu"]
pub type IdleAckPerihpR = crate::BitReader<IdleAckPerihp>;
impl IdleAckPerihpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleAckPerihp {
        match self.bits {
            false => IdleAckPerihp::B0,
            true => IdleAckPerihp::B1,
        }
    }
    #[doc = "idle acknowledge status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleAckPerihp::B0
    }
    #[doc = "idle acknowledge status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleAckPerihp::B1
    }
}
#[doc = "idle acknowledge status from vcodec niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleAckVcodec {
    #[doc = "0: idle acknowledge status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle acknowledge status of niu is 1"]
    B1 = 1,
}
impl From<IdleAckVcodec> for bool {
    #[inline(always)]
    fn from(variant: IdleAckVcodec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_ACK_VCODEC` reader - idle acknowledge status from vcodec niu"]
pub type IdleAckVcodecR = crate::BitReader<IdleAckVcodec>;
impl IdleAckVcodecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleAckVcodec {
        match self.bits {
            false => IdleAckVcodec::B0,
            true => IdleAckVcodec::B1,
        }
    }
    #[doc = "idle acknowledge status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleAckVcodec::B0
    }
    #[doc = "idle acknowledge status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleAckVcodec::B1
    }
}
#[doc = "idle acknowledge status from vdu niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleAckVdu {
    #[doc = "0: idle acknowledge status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle acknowledge status of niu is 1"]
    B1 = 1,
}
impl From<IdleAckVdu> for bool {
    #[inline(always)]
    fn from(variant: IdleAckVdu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_ACK_VDU` reader - idle acknowledge status from vdu niu"]
pub type IdleAckVduR = crate::BitReader<IdleAckVdu>;
impl IdleAckVduR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleAckVdu {
        match self.bits {
            false => IdleAckVdu::B0,
            true => IdleAckVdu::B1,
        }
    }
    #[doc = "idle acknowledge status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleAckVdu::B0
    }
    #[doc = "idle acknowledge status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleAckVdu::B1
    }
}
#[doc = "idle acknowledge status from rga niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleAckRga {
    #[doc = "0: idle acknowledge status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle acknowledge status of niu is 1"]
    B1 = 1,
}
impl From<IdleAckRga> for bool {
    #[inline(always)]
    fn from(variant: IdleAckRga) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_ACK_RGA` reader - idle acknowledge status from rga niu"]
pub type IdleAckRgaR = crate::BitReader<IdleAckRga>;
impl IdleAckRgaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleAckRga {
        match self.bits {
            false => IdleAckRga::B0,
            true => IdleAckRga::B1,
        }
    }
    #[doc = "idle acknowledge status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleAckRga::B0
    }
    #[doc = "idle acknowledge status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleAckRga::B1
    }
}
#[doc = "idle acknowledge status from iep niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleAckIep {
    #[doc = "0: idle acknowledge status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle acknowledge status of niu is 1"]
    B1 = 1,
}
impl From<IdleAckIep> for bool {
    #[inline(always)]
    fn from(variant: IdleAckIep) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_ACK_IEP` reader - idle acknowledge status from iep niu"]
pub type IdleAckIepR = crate::BitReader<IdleAckIep>;
impl IdleAckIepR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleAckIep {
        match self.bits {
            false => IdleAckIep::B0,
            true => IdleAckIep::B1,
        }
    }
    #[doc = "idle acknowledge status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleAckIep::B0
    }
    #[doc = "idle acknowledge status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleAckIep::B1
    }
}
#[doc = "idle acknowledge status from vopb niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleAckVopb {
    #[doc = "0: idle acknowledge status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle acknowledge status of niu is 1"]
    B1 = 1,
}
impl From<IdleAckVopb> for bool {
    #[inline(always)]
    fn from(variant: IdleAckVopb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_ACK_VOPB` reader - idle acknowledge status from vopb niu"]
pub type IdleAckVopbR = crate::BitReader<IdleAckVopb>;
impl IdleAckVopbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleAckVopb {
        match self.bits {
            false => IdleAckVopb::B0,
            true => IdleAckVopb::B1,
        }
    }
    #[doc = "idle acknowledge status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleAckVopb::B0
    }
    #[doc = "idle acknowledge status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleAckVopb::B1
    }
}
#[doc = "idle acknowledge status from vopl niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleAckVopl {
    #[doc = "0: idle acknowledge status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle acknowledge status of niu is 1"]
    B1 = 1,
}
impl From<IdleAckVopl> for bool {
    #[inline(always)]
    fn from(variant: IdleAckVopl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_ACK_VOPL` reader - idle acknowledge status from vopl niu"]
pub type IdleAckVoplR = crate::BitReader<IdleAckVopl>;
impl IdleAckVoplR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleAckVopl {
        match self.bits {
            false => IdleAckVopl::B0,
            true => IdleAckVopl::B1,
        }
    }
    #[doc = "idle acknowledge status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleAckVopl::B0
    }
    #[doc = "idle acknowledge status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleAckVopl::B1
    }
}
#[doc = "idle acknowledge status from isp0 niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleAckIsp0 {
    #[doc = "0: idle acknowledge status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle acknowledge status of niu is 1"]
    B1 = 1,
}
impl From<IdleAckIsp0> for bool {
    #[inline(always)]
    fn from(variant: IdleAckIsp0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_ACK_ISP0` reader - idle acknowledge status from isp0 niu"]
pub type IdleAckIsp0R = crate::BitReader<IdleAckIsp0>;
impl IdleAckIsp0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleAckIsp0 {
        match self.bits {
            false => IdleAckIsp0::B0,
            true => IdleAckIsp0::B1,
        }
    }
    #[doc = "idle acknowledge status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleAckIsp0::B0
    }
    #[doc = "idle acknowledge status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleAckIsp0::B1
    }
}
#[doc = "idle acknowledge status from isp1 niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleAckIsp1 {
    #[doc = "0: idle acknowledge status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle acknowledge status of niu is 1"]
    B1 = 1,
}
impl From<IdleAckIsp1> for bool {
    #[inline(always)]
    fn from(variant: IdleAckIsp1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_ACK_ISP1` reader - idle acknowledge status from isp1 niu"]
pub type IdleAckIsp1R = crate::BitReader<IdleAckIsp1>;
impl IdleAckIsp1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleAckIsp1 {
        match self.bits {
            false => IdleAckIsp1::B0,
            true => IdleAckIsp1::B1,
        }
    }
    #[doc = "idle acknowledge status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleAckIsp1::B0
    }
    #[doc = "idle acknowledge status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleAckIsp1::B1
    }
}
#[doc = "idle acknowledge status from hdcp niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleAckHdcp {
    #[doc = "0: idle acknowledge status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle acknowledge status of niu is 1"]
    B1 = 1,
}
impl From<IdleAckHdcp> for bool {
    #[inline(always)]
    fn from(variant: IdleAckHdcp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_ACK_HDCP` reader - idle acknowledge status from hdcp niu"]
pub type IdleAckHdcpR = crate::BitReader<IdleAckHdcp>;
impl IdleAckHdcpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleAckHdcp {
        match self.bits {
            false => IdleAckHdcp::B0,
            true => IdleAckHdcp::B1,
        }
    }
    #[doc = "idle acknowledge status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleAckHdcp::B0
    }
    #[doc = "idle acknowledge status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleAckHdcp::B1
    }
}
#[doc = "idle acknowledge status from usb3 niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleAckUsb3 {
    #[doc = "0: idle acknowledge status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle acknowledge status of niu is 1"]
    B1 = 1,
}
impl From<IdleAckUsb3> for bool {
    #[inline(always)]
    fn from(variant: IdleAckUsb3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_ACK_USB3` reader - idle acknowledge status from usb3 niu"]
pub type IdleAckUsb3R = crate::BitReader<IdleAckUsb3>;
impl IdleAckUsb3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleAckUsb3 {
        match self.bits {
            false => IdleAckUsb3::B0,
            true => IdleAckUsb3::B1,
        }
    }
    #[doc = "idle acknowledge status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleAckUsb3::B0
    }
    #[doc = "idle acknowledge status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleAckUsb3::B1
    }
}
#[doc = "idle acknowledge status from perilp m0 niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleAckPerilpm0 {
    #[doc = "0: idle acknowledge status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle acknowledge status of niu is 1"]
    B1 = 1,
}
impl From<IdleAckPerilpm0> for bool {
    #[inline(always)]
    fn from(variant: IdleAckPerilpm0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_ACK_PERILPM0` reader - idle acknowledge status from perilp m0 niu"]
pub type IdleAckPerilpm0R = crate::BitReader<IdleAckPerilpm0>;
impl IdleAckPerilpm0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleAckPerilpm0 {
        match self.bits {
            false => IdleAckPerilpm0::B0,
            true => IdleAckPerilpm0::B1,
        }
    }
    #[doc = "idle acknowledge status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleAckPerilpm0::B0
    }
    #[doc = "idle acknowledge status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleAckPerilpm0::B1
    }
}
#[doc = "idle acknowledge status from center niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleAckCenter {
    #[doc = "0: idle acknowledge status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle acknowledge status of niu is 1"]
    B1 = 1,
}
impl From<IdleAckCenter> for bool {
    #[inline(always)]
    fn from(variant: IdleAckCenter) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_ACK_CENTER` reader - idle acknowledge status from center niu"]
pub type IdleAckCenterR = crate::BitReader<IdleAckCenter>;
impl IdleAckCenterR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleAckCenter {
        match self.bits {
            false => IdleAckCenter::B0,
            true => IdleAckCenter::B1,
        }
    }
    #[doc = "idle acknowledge status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleAckCenter::B0
    }
    #[doc = "idle acknowledge status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleAckCenter::B1
    }
}
#[doc = "idle acknowledge status from ccim0 niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleAckCcim0 {
    #[doc = "0: idle acknowledge status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle acknowledge status of niu is 1"]
    B1 = 1,
}
impl From<IdleAckCcim0> for bool {
    #[inline(always)]
    fn from(variant: IdleAckCcim0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_ACK_CCIM0` reader - idle acknowledge status from ccim0 niu"]
pub type IdleAckCcim0R = crate::BitReader<IdleAckCcim0>;
impl IdleAckCcim0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleAckCcim0 {
        match self.bits {
            false => IdleAckCcim0::B0,
            true => IdleAckCcim0::B1,
        }
    }
    #[doc = "idle acknowledge status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleAckCcim0::B0
    }
    #[doc = "idle acknowledge status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleAckCcim0::B1
    }
}
#[doc = "idle acknowledge status from ccim1 niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleAckCcim1 {
    #[doc = "0: idle acknowledge status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle acknowledge status of niu is 1"]
    B1 = 1,
}
impl From<IdleAckCcim1> for bool {
    #[inline(always)]
    fn from(variant: IdleAckCcim1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_ACK_CCIM1` reader - idle acknowledge status from ccim1 niu"]
pub type IdleAckCcim1R = crate::BitReader<IdleAckCcim1>;
impl IdleAckCcim1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleAckCcim1 {
        match self.bits {
            false => IdleAckCcim1::B0,
            true => IdleAckCcim1::B1,
        }
    }
    #[doc = "idle acknowledge status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleAckCcim1::B0
    }
    #[doc = "idle acknowledge status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleAckCcim1::B1
    }
}
#[doc = "idle acknowledge status from vio niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleAckVio {
    #[doc = "0: idle acknowledge status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle acknowledge status of niu is 1"]
    B1 = 1,
}
impl From<IdleAckVio> for bool {
    #[inline(always)]
    fn from(variant: IdleAckVio) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_ACK_VIO` reader - idle acknowledge status from vio niu"]
pub type IdleAckVioR = crate::BitReader<IdleAckVio>;
impl IdleAckVioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleAckVio {
        match self.bits {
            false => IdleAckVio::B0,
            true => IdleAckVio::B1,
        }
    }
    #[doc = "idle acknowledge status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleAckVio::B0
    }
    #[doc = "idle acknowledge status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleAckVio::B1
    }
}
#[doc = "idle acknowledge status from msch0 niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleAckMsch0 {
    #[doc = "0: idle acknowledge status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle acknowledge status of niu is 1"]
    B1 = 1,
}
impl From<IdleAckMsch0> for bool {
    #[inline(always)]
    fn from(variant: IdleAckMsch0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_ACK_MSCH0` reader - idle acknowledge status from msch0 niu"]
pub type IdleAckMsch0R = crate::BitReader<IdleAckMsch0>;
impl IdleAckMsch0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleAckMsch0 {
        match self.bits {
            false => IdleAckMsch0::B0,
            true => IdleAckMsch0::B1,
        }
    }
    #[doc = "idle acknowledge status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleAckMsch0::B0
    }
    #[doc = "idle acknowledge status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleAckMsch0::B1
    }
}
#[doc = "idle acknowledge status from msch1 niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleAckMsch1 {
    #[doc = "0: idle acknowledge status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle acknowledge status of niu is 1"]
    B1 = 1,
}
impl From<IdleAckMsch1> for bool {
    #[inline(always)]
    fn from(variant: IdleAckMsch1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_ACK_MSCH1` reader - idle acknowledge status from msch1 niu"]
pub type IdleAckMsch1R = crate::BitReader<IdleAckMsch1>;
impl IdleAckMsch1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleAckMsch1 {
        match self.bits {
            false => IdleAckMsch1::B0,
            true => IdleAckMsch1::B1,
        }
    }
    #[doc = "idle acknowledge status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleAckMsch1::B0
    }
    #[doc = "idle acknowledge status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleAckMsch1::B1
    }
}
#[doc = "idle acknowledge status from alive niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleAckAlive {
    #[doc = "0: idle acknowledge status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle acknowledge status of niu is 1"]
    B1 = 1,
}
impl From<IdleAckAlive> for bool {
    #[inline(always)]
    fn from(variant: IdleAckAlive) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_ACK_ALIVE` reader - idle acknowledge status from alive niu"]
pub type IdleAckAliveR = crate::BitReader<IdleAckAlive>;
impl IdleAckAliveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleAckAlive {
        match self.bits {
            false => IdleAckAlive::B0,
            true => IdleAckAlive::B1,
        }
    }
    #[doc = "idle acknowledge status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleAckAlive::B0
    }
    #[doc = "idle acknowledge status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleAckAlive::B1
    }
}
#[doc = "idle acknowledge status from pmu niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleAckPmu {
    #[doc = "0: idle acknowledge status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle acknowledge status of niu is 1"]
    B1 = 1,
}
impl From<IdleAckPmu> for bool {
    #[inline(always)]
    fn from(variant: IdleAckPmu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_ACK_PMU` reader - idle acknowledge status from pmu niu"]
pub type IdleAckPmuR = crate::BitReader<IdleAckPmu>;
impl IdleAckPmuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleAckPmu {
        match self.bits {
            false => IdleAckPmu::B0,
            true => IdleAckPmu::B1,
        }
    }
    #[doc = "idle acknowledge status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleAckPmu::B0
    }
    #[doc = "idle acknowledge status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleAckPmu::B1
    }
}
#[doc = "idle acknowledge status from edp niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleAckEdp {
    #[doc = "0: idle acknowledge status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle acknowledge status of niu is 1"]
    B1 = 1,
}
impl From<IdleAckEdp> for bool {
    #[inline(always)]
    fn from(variant: IdleAckEdp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_ACK_EDP` reader - idle acknowledge status from edp niu"]
pub type IdleAckEdpR = crate::BitReader<IdleAckEdp>;
impl IdleAckEdpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleAckEdp {
        match self.bits {
            false => IdleAckEdp::B0,
            true => IdleAckEdp::B1,
        }
    }
    #[doc = "idle acknowledge status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleAckEdp::B0
    }
    #[doc = "idle acknowledge status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleAckEdp::B1
    }
}
#[doc = "idle acknowledge status from gmac niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleAckGmac {
    #[doc = "0: idle acknowledge status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle acknowledge status of niu is 1"]
    B1 = 1,
}
impl From<IdleAckGmac> for bool {
    #[inline(always)]
    fn from(variant: IdleAckGmac) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_ACK_GMAC` reader - idle acknowledge status from gmac niu"]
pub type IdleAckGmacR = crate::BitReader<IdleAckGmac>;
impl IdleAckGmacR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleAckGmac {
        match self.bits {
            false => IdleAckGmac::B0,
            true => IdleAckGmac::B1,
        }
    }
    #[doc = "idle acknowledge status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleAckGmac::B0
    }
    #[doc = "idle acknowledge status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleAckGmac::B1
    }
}
#[doc = "idle acknowledge status from emmc niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleAckEmmc {
    #[doc = "0: idle acknowledge status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle acknowledge status of niu is 1"]
    B1 = 1,
}
impl From<IdleAckEmmc> for bool {
    #[inline(always)]
    fn from(variant: IdleAckEmmc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_ACK_EMMC` reader - idle acknowledge status from emmc niu"]
pub type IdleAckEmmcR = crate::BitReader<IdleAckEmmc>;
impl IdleAckEmmcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleAckEmmc {
        match self.bits {
            false => IdleAckEmmc::B0,
            true => IdleAckEmmc::B1,
        }
    }
    #[doc = "idle acknowledge status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleAckEmmc::B0
    }
    #[doc = "idle acknowledge status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleAckEmmc::B1
    }
}
#[doc = "idle acknowledge status from center1 niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleAckCenter1 {
    #[doc = "0: idle acknowledge status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle acknowledge status of niu is 1"]
    B1 = 1,
}
impl From<IdleAckCenter1> for bool {
    #[inline(always)]
    fn from(variant: IdleAckCenter1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_ACK_CENTER1` reader - idle acknowledge status from center1 niu"]
pub type IdleAckCenter1R = crate::BitReader<IdleAckCenter1>;
impl IdleAckCenter1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleAckCenter1 {
        match self.bits {
            false => IdleAckCenter1::B0,
            true => IdleAckCenter1::B1,
        }
    }
    #[doc = "idle acknowledge status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleAckCenter1::B0
    }
    #[doc = "idle acknowledge status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleAckCenter1::B1
    }
}
#[doc = "idle acknowledge status from pmu m0 niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleAckPmum0 {
    #[doc = "0: idle acknowledge status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle acknowledge status of niu is 1"]
    B1 = 1,
}
impl From<IdleAckPmum0> for bool {
    #[inline(always)]
    fn from(variant: IdleAckPmum0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_ACK_PMUM0` reader - idle acknowledge status from pmu m0 niu"]
pub type IdleAckPmum0R = crate::BitReader<IdleAckPmum0>;
impl IdleAckPmum0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleAckPmum0 {
        match self.bits {
            false => IdleAckPmum0::B0,
            true => IdleAckPmum0::B1,
        }
    }
    #[doc = "idle acknowledge status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleAckPmum0::B0
    }
    #[doc = "idle acknowledge status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleAckPmum0::B1
    }
}
#[doc = "idle acknowledge status from gic niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleAckGic {
    #[doc = "0: idle acknowledge status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle acknowledge status of niu is 1"]
    B1 = 1,
}
impl From<IdleAckGic> for bool {
    #[inline(always)]
    fn from(variant: IdleAckGic) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_ACK_GIC` reader - idle acknowledge status from gic niu"]
pub type IdleAckGicR = crate::BitReader<IdleAckGic>;
impl IdleAckGicR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleAckGic {
        match self.bits {
            false => IdleAckGic::B0,
            true => IdleAckGic::B1,
        }
    }
    #[doc = "idle acknowledge status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleAckGic::B0
    }
    #[doc = "idle acknowledge status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleAckGic::B1
    }
}
#[doc = "idle acknowledge status from sd niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleAckSd {
    #[doc = "0: idle acknowledge status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle acknowledge status of niu is 1"]
    B1 = 1,
}
impl From<IdleAckSd> for bool {
    #[inline(always)]
    fn from(variant: IdleAckSd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_ACK_SD` reader - idle acknowledge status from sd niu"]
pub type IdleAckSdR = crate::BitReader<IdleAckSd>;
impl IdleAckSdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleAckSd {
        match self.bits {
            false => IdleAckSd::B0,
            true => IdleAckSd::B1,
        }
    }
    #[doc = "idle acknowledge status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleAckSd::B0
    }
    #[doc = "idle acknowledge status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleAckSd::B1
    }
}
#[doc = "idle acknowledge status from sdioaudio niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleAckSdioaudio {
    #[doc = "0: idle acknowledge status of niu is 0"]
    B0 = 0,
    #[doc = "1: idle acknowledge status of niu is 1"]
    B1 = 1,
}
impl From<IdleAckSdioaudio> for bool {
    #[inline(always)]
    fn from(variant: IdleAckSdioaudio) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_ACK_SDIOAUDIO` reader - idle acknowledge status from sdioaudio niu"]
pub type IdleAckSdioaudioR = crate::BitReader<IdleAckSdioaudio>;
impl IdleAckSdioaudioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleAckSdioaudio {
        match self.bits {
            false => IdleAckSdioaudio::B0,
            true => IdleAckSdioaudio::B1,
        }
    }
    #[doc = "idle acknowledge status of niu is 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleAckSdioaudio::B0
    }
    #[doc = "idle acknowledge status of niu is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleAckSdioaudio::B1
    }
}
impl R {
    #[doc = "Bit 0 - idle acknowledge status from gpu niu"]
    #[inline(always)]
    pub fn idle_ack_gpu(&self) -> IdleAckGpuR {
        IdleAckGpuR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - idle acknowledge status from perilp niu"]
    #[inline(always)]
    pub fn idle_ack_perilp(&self) -> IdleAckPerilpR {
        IdleAckPerilpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - idle acknowledge status from perihp niu"]
    #[inline(always)]
    pub fn idle_ack_perihp(&self) -> IdleAckPerihpR {
        IdleAckPerihpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - idle acknowledge status from vcodec niu"]
    #[inline(always)]
    pub fn idle_ack_vcodec(&self) -> IdleAckVcodecR {
        IdleAckVcodecR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - idle acknowledge status from vdu niu"]
    #[inline(always)]
    pub fn idle_ack_vdu(&self) -> IdleAckVduR {
        IdleAckVduR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - idle acknowledge status from rga niu"]
    #[inline(always)]
    pub fn idle_ack_rga(&self) -> IdleAckRgaR {
        IdleAckRgaR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - idle acknowledge status from iep niu"]
    #[inline(always)]
    pub fn idle_ack_iep(&self) -> IdleAckIepR {
        IdleAckIepR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - idle acknowledge status from vopb niu"]
    #[inline(always)]
    pub fn idle_ack_vopb(&self) -> IdleAckVopbR {
        IdleAckVopbR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - idle acknowledge status from vopl niu"]
    #[inline(always)]
    pub fn idle_ack_vopl(&self) -> IdleAckVoplR {
        IdleAckVoplR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - idle acknowledge status from isp0 niu"]
    #[inline(always)]
    pub fn idle_ack_isp0(&self) -> IdleAckIsp0R {
        IdleAckIsp0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - idle acknowledge status from isp1 niu"]
    #[inline(always)]
    pub fn idle_ack_isp1(&self) -> IdleAckIsp1R {
        IdleAckIsp1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - idle acknowledge status from hdcp niu"]
    #[inline(always)]
    pub fn idle_ack_hdcp(&self) -> IdleAckHdcpR {
        IdleAckHdcpR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - idle acknowledge status from usb3 niu"]
    #[inline(always)]
    pub fn idle_ack_usb3(&self) -> IdleAckUsb3R {
        IdleAckUsb3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - idle acknowledge status from perilp m0 niu"]
    #[inline(always)]
    pub fn idle_ack_perilpm0(&self) -> IdleAckPerilpm0R {
        IdleAckPerilpm0R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - idle acknowledge status from center niu"]
    #[inline(always)]
    pub fn idle_ack_center(&self) -> IdleAckCenterR {
        IdleAckCenterR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - idle acknowledge status from ccim0 niu"]
    #[inline(always)]
    pub fn idle_ack_ccim0(&self) -> IdleAckCcim0R {
        IdleAckCcim0R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - idle acknowledge status from ccim1 niu"]
    #[inline(always)]
    pub fn idle_ack_ccim1(&self) -> IdleAckCcim1R {
        IdleAckCcim1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - idle acknowledge status from vio niu"]
    #[inline(always)]
    pub fn idle_ack_vio(&self) -> IdleAckVioR {
        IdleAckVioR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - idle acknowledge status from msch0 niu"]
    #[inline(always)]
    pub fn idle_ack_msch0(&self) -> IdleAckMsch0R {
        IdleAckMsch0R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - idle acknowledge status from msch1 niu"]
    #[inline(always)]
    pub fn idle_ack_msch1(&self) -> IdleAckMsch1R {
        IdleAckMsch1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - idle acknowledge status from alive niu"]
    #[inline(always)]
    pub fn idle_ack_alive(&self) -> IdleAckAliveR {
        IdleAckAliveR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - idle acknowledge status from pmu niu"]
    #[inline(always)]
    pub fn idle_ack_pmu(&self) -> IdleAckPmuR {
        IdleAckPmuR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - idle acknowledge status from edp niu"]
    #[inline(always)]
    pub fn idle_ack_edp(&self) -> IdleAckEdpR {
        IdleAckEdpR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - idle acknowledge status from gmac niu"]
    #[inline(always)]
    pub fn idle_ack_gmac(&self) -> IdleAckGmacR {
        IdleAckGmacR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - idle acknowledge status from emmc niu"]
    #[inline(always)]
    pub fn idle_ack_emmc(&self) -> IdleAckEmmcR {
        IdleAckEmmcR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - idle acknowledge status from center1 niu"]
    #[inline(always)]
    pub fn idle_ack_center1(&self) -> IdleAckCenter1R {
        IdleAckCenter1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - idle acknowledge status from pmu m0 niu"]
    #[inline(always)]
    pub fn idle_ack_pmum0(&self) -> IdleAckPmum0R {
        IdleAckPmum0R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - idle acknowledge status from gic niu"]
    #[inline(always)]
    pub fn idle_ack_gic(&self) -> IdleAckGicR {
        IdleAckGicR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - idle acknowledge status from sd niu"]
    #[inline(always)]
    pub fn idle_ack_sd(&self) -> IdleAckSdR {
        IdleAckSdR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - idle acknowledge status from sdioaudio niu"]
    #[inline(always)]
    pub fn idle_ack_sdioaudio(&self) -> IdleAckSdioaudioR {
        IdleAckSdioaudioR::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "pmu bus idle ack status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus_idle_ack::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BusIdleAckSpec;
impl crate::RegisterSpec for BusIdleAckSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus_idle_ack::R`](R) reader structure"]
impl crate::Readable for BusIdleAckSpec {}
#[doc = "`reset()` method sets BUS_IDLE_ACK to value 0"]
impl crate::Resettable for BusIdleAckSpec {
    const RESET_VALUE: u32 = 0;
}
