#[doc = "Register `PMU_BUS_IDLE_REQ` reader"]
pub type R = crate::R<PmuBusIdleReqSpec>;
#[doc = "Register `PMU_BUS_IDLE_REQ` writer"]
pub type W = crate::W<PmuBusIdleReqSpec>;
#[doc = "send idle request to gpu niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleReqGpu {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IdleReqGpu> for bool {
    #[inline(always)]
    fn from(variant: IdleReqGpu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_REQ_GPU` reader - send idle request to gpu niu"]
pub type IdleReqGpuR = crate::BitReader<IdleReqGpu>;
impl IdleReqGpuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleReqGpu {
        match self.bits {
            false => IdleReqGpu::B0,
            true => IdleReqGpu::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleReqGpu::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleReqGpu::B1
    }
}
#[doc = "Field `IDLE_REQ_GPU` writer - send idle request to gpu niu"]
pub type IdleReqGpuW<'a, REG> = crate::BitWriter<'a, REG, IdleReqGpu>;
impl<'a, REG> IdleReqGpuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqGpu::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqGpu::B1)
    }
}
#[doc = "send idle request to perilp niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleReqPerilp {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IdleReqPerilp> for bool {
    #[inline(always)]
    fn from(variant: IdleReqPerilp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_REQ_PERILP` reader - send idle request to perilp niu"]
pub type IdleReqPerilpR = crate::BitReader<IdleReqPerilp>;
impl IdleReqPerilpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleReqPerilp {
        match self.bits {
            false => IdleReqPerilp::B0,
            true => IdleReqPerilp::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleReqPerilp::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleReqPerilp::B1
    }
}
#[doc = "Field `IDLE_REQ_PERILP` writer - send idle request to perilp niu"]
pub type IdleReqPerilpW<'a, REG> = crate::BitWriter<'a, REG, IdleReqPerilp>;
impl<'a, REG> IdleReqPerilpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqPerilp::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqPerilp::B1)
    }
}
#[doc = "send idle request to perihp niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleReqPerihp {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IdleReqPerihp> for bool {
    #[inline(always)]
    fn from(variant: IdleReqPerihp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_REQ_PERIHP` reader - send idle request to perihp niu"]
pub type IdleReqPerihpR = crate::BitReader<IdleReqPerihp>;
impl IdleReqPerihpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleReqPerihp {
        match self.bits {
            false => IdleReqPerihp::B0,
            true => IdleReqPerihp::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleReqPerihp::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleReqPerihp::B1
    }
}
#[doc = "Field `IDLE_REQ_PERIHP` writer - send idle request to perihp niu"]
pub type IdleReqPerihpW<'a, REG> = crate::BitWriter<'a, REG, IdleReqPerihp>;
impl<'a, REG> IdleReqPerihpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqPerihp::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqPerihp::B1)
    }
}
#[doc = "send idle request to vcodec niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleReqVcodec {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IdleReqVcodec> for bool {
    #[inline(always)]
    fn from(variant: IdleReqVcodec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_REQ_VCODEC` reader - send idle request to vcodec niu"]
pub type IdleReqVcodecR = crate::BitReader<IdleReqVcodec>;
impl IdleReqVcodecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleReqVcodec {
        match self.bits {
            false => IdleReqVcodec::B0,
            true => IdleReqVcodec::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleReqVcodec::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleReqVcodec::B1
    }
}
#[doc = "Field `IDLE_REQ_VCODEC` writer - send idle request to vcodec niu"]
pub type IdleReqVcodecW<'a, REG> = crate::BitWriter<'a, REG, IdleReqVcodec>;
impl<'a, REG> IdleReqVcodecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqVcodec::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqVcodec::B1)
    }
}
#[doc = "send idle request to vdu niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleReqVdu {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IdleReqVdu> for bool {
    #[inline(always)]
    fn from(variant: IdleReqVdu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_REQ_VDU` reader - send idle request to vdu niu"]
pub type IdleReqVduR = crate::BitReader<IdleReqVdu>;
impl IdleReqVduR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleReqVdu {
        match self.bits {
            false => IdleReqVdu::B0,
            true => IdleReqVdu::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleReqVdu::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleReqVdu::B1
    }
}
#[doc = "Field `IDLE_REQ_VDU` writer - send idle request to vdu niu"]
pub type IdleReqVduW<'a, REG> = crate::BitWriter<'a, REG, IdleReqVdu>;
impl<'a, REG> IdleReqVduW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqVdu::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqVdu::B1)
    }
}
#[doc = "send idle request to rga niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleReqRga {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IdleReqRga> for bool {
    #[inline(always)]
    fn from(variant: IdleReqRga) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_REQ_RGA` reader - send idle request to rga niu"]
pub type IdleReqRgaR = crate::BitReader<IdleReqRga>;
impl IdleReqRgaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleReqRga {
        match self.bits {
            false => IdleReqRga::B0,
            true => IdleReqRga::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleReqRga::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleReqRga::B1
    }
}
#[doc = "Field `IDLE_REQ_RGA` writer - send idle request to rga niu"]
pub type IdleReqRgaW<'a, REG> = crate::BitWriter<'a, REG, IdleReqRga>;
impl<'a, REG> IdleReqRgaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqRga::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqRga::B1)
    }
}
#[doc = "send idle request to iep niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleReqIep {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IdleReqIep> for bool {
    #[inline(always)]
    fn from(variant: IdleReqIep) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_REQ_IEP` reader - send idle request to iep niu"]
pub type IdleReqIepR = crate::BitReader<IdleReqIep>;
impl IdleReqIepR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleReqIep {
        match self.bits {
            false => IdleReqIep::B0,
            true => IdleReqIep::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleReqIep::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleReqIep::B1
    }
}
#[doc = "Field `IDLE_REQ_IEP` writer - send idle request to iep niu"]
pub type IdleReqIepW<'a, REG> = crate::BitWriter<'a, REG, IdleReqIep>;
impl<'a, REG> IdleReqIepW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqIep::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqIep::B1)
    }
}
#[doc = "send idle request to vopb niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleReqVopb {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IdleReqVopb> for bool {
    #[inline(always)]
    fn from(variant: IdleReqVopb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_REQ_VOPB` reader - send idle request to vopb niu"]
pub type IdleReqVopbR = crate::BitReader<IdleReqVopb>;
impl IdleReqVopbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleReqVopb {
        match self.bits {
            false => IdleReqVopb::B0,
            true => IdleReqVopb::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleReqVopb::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleReqVopb::B1
    }
}
#[doc = "Field `IDLE_REQ_VOPB` writer - send idle request to vopb niu"]
pub type IdleReqVopbW<'a, REG> = crate::BitWriter<'a, REG, IdleReqVopb>;
impl<'a, REG> IdleReqVopbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqVopb::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqVopb::B1)
    }
}
#[doc = "send idle request to vopl niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleReqVopl {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IdleReqVopl> for bool {
    #[inline(always)]
    fn from(variant: IdleReqVopl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_REQ_VOPL` reader - send idle request to vopl niu"]
pub type IdleReqVoplR = crate::BitReader<IdleReqVopl>;
impl IdleReqVoplR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleReqVopl {
        match self.bits {
            false => IdleReqVopl::B0,
            true => IdleReqVopl::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleReqVopl::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleReqVopl::B1
    }
}
#[doc = "Field `IDLE_REQ_VOPL` writer - send idle request to vopl niu"]
pub type IdleReqVoplW<'a, REG> = crate::BitWriter<'a, REG, IdleReqVopl>;
impl<'a, REG> IdleReqVoplW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqVopl::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqVopl::B1)
    }
}
#[doc = "send idle request to isp0 niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleReqIsp0 {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IdleReqIsp0> for bool {
    #[inline(always)]
    fn from(variant: IdleReqIsp0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_REQ_ISP0` reader - send idle request to isp0 niu"]
pub type IdleReqIsp0R = crate::BitReader<IdleReqIsp0>;
impl IdleReqIsp0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleReqIsp0 {
        match self.bits {
            false => IdleReqIsp0::B0,
            true => IdleReqIsp0::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleReqIsp0::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleReqIsp0::B1
    }
}
#[doc = "Field `IDLE_REQ_ISP0` writer - send idle request to isp0 niu"]
pub type IdleReqIsp0W<'a, REG> = crate::BitWriter<'a, REG, IdleReqIsp0>;
impl<'a, REG> IdleReqIsp0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqIsp0::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqIsp0::B1)
    }
}
#[doc = "send idle request to isp1 niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleReqIsp1 {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IdleReqIsp1> for bool {
    #[inline(always)]
    fn from(variant: IdleReqIsp1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_REQ_ISP1` reader - send idle request to isp1 niu"]
pub type IdleReqIsp1R = crate::BitReader<IdleReqIsp1>;
impl IdleReqIsp1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleReqIsp1 {
        match self.bits {
            false => IdleReqIsp1::B0,
            true => IdleReqIsp1::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleReqIsp1::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleReqIsp1::B1
    }
}
#[doc = "Field `IDLE_REQ_ISP1` writer - send idle request to isp1 niu"]
pub type IdleReqIsp1W<'a, REG> = crate::BitWriter<'a, REG, IdleReqIsp1>;
impl<'a, REG> IdleReqIsp1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqIsp1::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqIsp1::B1)
    }
}
#[doc = "send idle request to hdcp niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleReqHdcp {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IdleReqHdcp> for bool {
    #[inline(always)]
    fn from(variant: IdleReqHdcp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_REQ_HDCP` reader - send idle request to hdcp niu"]
pub type IdleReqHdcpR = crate::BitReader<IdleReqHdcp>;
impl IdleReqHdcpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleReqHdcp {
        match self.bits {
            false => IdleReqHdcp::B0,
            true => IdleReqHdcp::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleReqHdcp::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleReqHdcp::B1
    }
}
#[doc = "Field `IDLE_REQ_HDCP` writer - send idle request to hdcp niu"]
pub type IdleReqHdcpW<'a, REG> = crate::BitWriter<'a, REG, IdleReqHdcp>;
impl<'a, REG> IdleReqHdcpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqHdcp::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqHdcp::B1)
    }
}
#[doc = "send idle request to usb3 niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleReqUsb3 {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IdleReqUsb3> for bool {
    #[inline(always)]
    fn from(variant: IdleReqUsb3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_REQ_USB3` reader - send idle request to usb3 niu"]
pub type IdleReqUsb3R = crate::BitReader<IdleReqUsb3>;
impl IdleReqUsb3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleReqUsb3 {
        match self.bits {
            false => IdleReqUsb3::B0,
            true => IdleReqUsb3::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleReqUsb3::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleReqUsb3::B1
    }
}
#[doc = "Field `IDLE_REQ_USB3` writer - send idle request to usb3 niu"]
pub type IdleReqUsb3W<'a, REG> = crate::BitWriter<'a, REG, IdleReqUsb3>;
impl<'a, REG> IdleReqUsb3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqUsb3::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqUsb3::B1)
    }
}
#[doc = "send idle request to perilp m0 niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleReqPerilpm0 {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IdleReqPerilpm0> for bool {
    #[inline(always)]
    fn from(variant: IdleReqPerilpm0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_REQ_PERILPM0` reader - send idle request to perilp m0 niu"]
pub type IdleReqPerilpm0R = crate::BitReader<IdleReqPerilpm0>;
impl IdleReqPerilpm0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleReqPerilpm0 {
        match self.bits {
            false => IdleReqPerilpm0::B0,
            true => IdleReqPerilpm0::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleReqPerilpm0::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleReqPerilpm0::B1
    }
}
#[doc = "Field `IDLE_REQ_PERILPM0` writer - send idle request to perilp m0 niu"]
pub type IdleReqPerilpm0W<'a, REG> = crate::BitWriter<'a, REG, IdleReqPerilpm0>;
impl<'a, REG> IdleReqPerilpm0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqPerilpm0::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqPerilpm0::B1)
    }
}
#[doc = "send idle request to center niu\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleReqCenter {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IdleReqCenter> for bool {
    #[inline(always)]
    fn from(variant: IdleReqCenter) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_REQ_CENTER` reader - send idle request to center niu"]
pub type IdleReqCenterR = crate::BitReader<IdleReqCenter>;
impl IdleReqCenterR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleReqCenter {
        match self.bits {
            false => IdleReqCenter::B0,
            true => IdleReqCenter::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleReqCenter::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleReqCenter::B1
    }
}
#[doc = "Field `IDLE_REQ_CENTER` writer - send idle request to center niu"]
pub type IdleReqCenterW<'a, REG> = crate::BitWriter<'a, REG, IdleReqCenter>;
impl<'a, REG> IdleReqCenterW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqCenter::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqCenter::B1)
    }
}
#[doc = "send idle request to ccim0 low power interface\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleReqCcim0 {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IdleReqCcim0> for bool {
    #[inline(always)]
    fn from(variant: IdleReqCcim0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_REQ_CCIM0` reader - send idle request to ccim0 low power interface"]
pub type IdleReqCcim0R = crate::BitReader<IdleReqCcim0>;
impl IdleReqCcim0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleReqCcim0 {
        match self.bits {
            false => IdleReqCcim0::B0,
            true => IdleReqCcim0::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleReqCcim0::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleReqCcim0::B1
    }
}
#[doc = "Field `IDLE_REQ_CCIM0` writer - send idle request to ccim0 low power interface"]
pub type IdleReqCcim0W<'a, REG> = crate::BitWriter<'a, REG, IdleReqCcim0>;
impl<'a, REG> IdleReqCcim0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqCcim0::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqCcim0::B1)
    }
}
#[doc = "send idle request to ccim1 low power interface\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleReqCcim1 {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IdleReqCcim1> for bool {
    #[inline(always)]
    fn from(variant: IdleReqCcim1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_REQ_CCIM1` reader - send idle request to ccim1 low power interface"]
pub type IdleReqCcim1R = crate::BitReader<IdleReqCcim1>;
impl IdleReqCcim1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleReqCcim1 {
        match self.bits {
            false => IdleReqCcim1::B0,
            true => IdleReqCcim1::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleReqCcim1::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleReqCcim1::B1
    }
}
#[doc = "Field `IDLE_REQ_CCIM1` writer - send idle request to ccim1 low power interface"]
pub type IdleReqCcim1W<'a, REG> = crate::BitWriter<'a, REG, IdleReqCcim1>;
impl<'a, REG> IdleReqCcim1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqCcim1::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqCcim1::B1)
    }
}
#[doc = "send idle request to vio low power interface\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleReqVio {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IdleReqVio> for bool {
    #[inline(always)]
    fn from(variant: IdleReqVio) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_REQ_VIO` reader - send idle request to vio low power interface"]
pub type IdleReqVioR = crate::BitReader<IdleReqVio>;
impl IdleReqVioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleReqVio {
        match self.bits {
            false => IdleReqVio::B0,
            true => IdleReqVio::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleReqVio::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleReqVio::B1
    }
}
#[doc = "Field `IDLE_REQ_VIO` writer - send idle request to vio low power interface"]
pub type IdleReqVioW<'a, REG> = crate::BitWriter<'a, REG, IdleReqVio>;
impl<'a, REG> IdleReqVioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqVio::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqVio::B1)
    }
}
#[doc = "send idle request to msch0 low power interface\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleReqMsch0 {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IdleReqMsch0> for bool {
    #[inline(always)]
    fn from(variant: IdleReqMsch0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_REQ_MSCH0` reader - send idle request to msch0 low power interface"]
pub type IdleReqMsch0R = crate::BitReader<IdleReqMsch0>;
impl IdleReqMsch0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleReqMsch0 {
        match self.bits {
            false => IdleReqMsch0::B0,
            true => IdleReqMsch0::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleReqMsch0::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleReqMsch0::B1
    }
}
#[doc = "Field `IDLE_REQ_MSCH0` writer - send idle request to msch0 low power interface"]
pub type IdleReqMsch0W<'a, REG> = crate::BitWriter<'a, REG, IdleReqMsch0>;
impl<'a, REG> IdleReqMsch0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqMsch0::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqMsch0::B1)
    }
}
#[doc = "send idle request to msch1 low power interface\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleReqMsch1 {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IdleReqMsch1> for bool {
    #[inline(always)]
    fn from(variant: IdleReqMsch1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_REQ_MSCH1` reader - send idle request to msch1 low power interface"]
pub type IdleReqMsch1R = crate::BitReader<IdleReqMsch1>;
impl IdleReqMsch1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleReqMsch1 {
        match self.bits {
            false => IdleReqMsch1::B0,
            true => IdleReqMsch1::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleReqMsch1::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleReqMsch1::B1
    }
}
#[doc = "Field `IDLE_REQ_MSCH1` writer - send idle request to msch1 low power interface"]
pub type IdleReqMsch1W<'a, REG> = crate::BitWriter<'a, REG, IdleReqMsch1>;
impl<'a, REG> IdleReqMsch1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqMsch1::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqMsch1::B1)
    }
}
#[doc = "send idle request to alive low power interface\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleReqAlive {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IdleReqAlive> for bool {
    #[inline(always)]
    fn from(variant: IdleReqAlive) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_REQ_ALIVE` reader - send idle request to alive low power interface"]
pub type IdleReqAliveR = crate::BitReader<IdleReqAlive>;
impl IdleReqAliveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleReqAlive {
        match self.bits {
            false => IdleReqAlive::B0,
            true => IdleReqAlive::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleReqAlive::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleReqAlive::B1
    }
}
#[doc = "Field `IDLE_REQ_ALIVE` writer - send idle request to alive low power interface"]
pub type IdleReqAliveW<'a, REG> = crate::BitWriter<'a, REG, IdleReqAlive>;
impl<'a, REG> IdleReqAliveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqAlive::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqAlive::B1)
    }
}
#[doc = "send idle request to pmu low power interface\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleReqPmu {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IdleReqPmu> for bool {
    #[inline(always)]
    fn from(variant: IdleReqPmu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_REQ_PMU` reader - send idle request to pmu low power interface"]
pub type IdleReqPmuR = crate::BitReader<IdleReqPmu>;
impl IdleReqPmuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleReqPmu {
        match self.bits {
            false => IdleReqPmu::B0,
            true => IdleReqPmu::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleReqPmu::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleReqPmu::B1
    }
}
#[doc = "Field `IDLE_REQ_PMU` writer - send idle request to pmu low power interface"]
pub type IdleReqPmuW<'a, REG> = crate::BitWriter<'a, REG, IdleReqPmu>;
impl<'a, REG> IdleReqPmuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqPmu::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqPmu::B1)
    }
}
#[doc = "send idle request to edp low power interface\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleReqEdp {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IdleReqEdp> for bool {
    #[inline(always)]
    fn from(variant: IdleReqEdp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_REQ_EDP` reader - send idle request to edp low power interface"]
pub type IdleReqEdpR = crate::BitReader<IdleReqEdp>;
impl IdleReqEdpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleReqEdp {
        match self.bits {
            false => IdleReqEdp::B0,
            true => IdleReqEdp::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleReqEdp::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleReqEdp::B1
    }
}
#[doc = "Field `IDLE_REQ_EDP` writer - send idle request to edp low power interface"]
pub type IdleReqEdpW<'a, REG> = crate::BitWriter<'a, REG, IdleReqEdp>;
impl<'a, REG> IdleReqEdpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqEdp::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqEdp::B1)
    }
}
#[doc = "send idle request to gmac low power interface\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleReqGmac {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IdleReqGmac> for bool {
    #[inline(always)]
    fn from(variant: IdleReqGmac) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_REQ_GMAC` reader - send idle request to gmac low power interface"]
pub type IdleReqGmacR = crate::BitReader<IdleReqGmac>;
impl IdleReqGmacR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleReqGmac {
        match self.bits {
            false => IdleReqGmac::B0,
            true => IdleReqGmac::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleReqGmac::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleReqGmac::B1
    }
}
#[doc = "Field `IDLE_REQ_GMAC` writer - send idle request to gmac low power interface"]
pub type IdleReqGmacW<'a, REG> = crate::BitWriter<'a, REG, IdleReqGmac>;
impl<'a, REG> IdleReqGmacW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqGmac::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqGmac::B1)
    }
}
#[doc = "send idle request to emmc low power interface\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleReqEmmc {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IdleReqEmmc> for bool {
    #[inline(always)]
    fn from(variant: IdleReqEmmc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_REQ_EMMC` reader - send idle request to emmc low power interface"]
pub type IdleReqEmmcR = crate::BitReader<IdleReqEmmc>;
impl IdleReqEmmcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleReqEmmc {
        match self.bits {
            false => IdleReqEmmc::B0,
            true => IdleReqEmmc::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleReqEmmc::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleReqEmmc::B1
    }
}
#[doc = "Field `IDLE_REQ_EMMC` writer - send idle request to emmc low power interface"]
pub type IdleReqEmmcW<'a, REG> = crate::BitWriter<'a, REG, IdleReqEmmc>;
impl<'a, REG> IdleReqEmmcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqEmmc::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqEmmc::B1)
    }
}
#[doc = "send idle request to center1 low power interface\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleReqCenter1 {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IdleReqCenter1> for bool {
    #[inline(always)]
    fn from(variant: IdleReqCenter1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_REQ_CENTER1` reader - send idle request to center1 low power interface"]
pub type IdleReqCenter1R = crate::BitReader<IdleReqCenter1>;
impl IdleReqCenter1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleReqCenter1 {
        match self.bits {
            false => IdleReqCenter1::B0,
            true => IdleReqCenter1::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleReqCenter1::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleReqCenter1::B1
    }
}
#[doc = "Field `IDLE_REQ_CENTER1` writer - send idle request to center1 low power interface"]
pub type IdleReqCenter1W<'a, REG> = crate::BitWriter<'a, REG, IdleReqCenter1>;
impl<'a, REG> IdleReqCenter1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqCenter1::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqCenter1::B1)
    }
}
#[doc = "send idle request to pmu m0 low power interface\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleReqPmum0 {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IdleReqPmum0> for bool {
    #[inline(always)]
    fn from(variant: IdleReqPmum0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_REQ_PMUM0` reader - send idle request to pmu m0 low power interface"]
pub type IdleReqPmum0R = crate::BitReader<IdleReqPmum0>;
impl IdleReqPmum0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleReqPmum0 {
        match self.bits {
            false => IdleReqPmum0::B0,
            true => IdleReqPmum0::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleReqPmum0::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleReqPmum0::B1
    }
}
#[doc = "Field `IDLE_REQ_PMUM0` writer - send idle request to pmu m0 low power interface"]
pub type IdleReqPmum0W<'a, REG> = crate::BitWriter<'a, REG, IdleReqPmum0>;
impl<'a, REG> IdleReqPmum0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqPmum0::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqPmum0::B1)
    }
}
#[doc = "send idle request to gic low power interface\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleReqGic {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IdleReqGic> for bool {
    #[inline(always)]
    fn from(variant: IdleReqGic) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_REQ_GIC` reader - send idle request to gic low power interface"]
pub type IdleReqGicR = crate::BitReader<IdleReqGic>;
impl IdleReqGicR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleReqGic {
        match self.bits {
            false => IdleReqGic::B0,
            true => IdleReqGic::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleReqGic::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleReqGic::B1
    }
}
#[doc = "Field `IDLE_REQ_GIC` writer - send idle request to gic low power interface"]
pub type IdleReqGicW<'a, REG> = crate::BitWriter<'a, REG, IdleReqGic>;
impl<'a, REG> IdleReqGicW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqGic::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqGic::B1)
    }
}
#[doc = "send idle request to sd low power interface\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleReqSd {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IdleReqSd> for bool {
    #[inline(always)]
    fn from(variant: IdleReqSd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_REQ_SD` reader - send idle request to sd low power interface"]
pub type IdleReqSdR = crate::BitReader<IdleReqSd>;
impl IdleReqSdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleReqSd {
        match self.bits {
            false => IdleReqSd::B0,
            true => IdleReqSd::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleReqSd::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleReqSd::B1
    }
}
#[doc = "Field `IDLE_REQ_SD` writer - send idle request to sd low power interface"]
pub type IdleReqSdW<'a, REG> = crate::BitWriter<'a, REG, IdleReqSd>;
impl<'a, REG> IdleReqSdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqSd::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqSd::B1)
    }
}
#[doc = "send idle request to sdioaudio low power interface\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleReqSdioaudio {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<IdleReqSdioaudio> for bool {
    #[inline(always)]
    fn from(variant: IdleReqSdioaudio) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_REQ_SDIOAUDIO` reader - send idle request to sdioaudio low power interface"]
pub type IdleReqSdioaudioR = crate::BitReader<IdleReqSdioaudio>;
impl IdleReqSdioaudioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleReqSdioaudio {
        match self.bits {
            false => IdleReqSdioaudio::B0,
            true => IdleReqSdioaudio::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleReqSdioaudio::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleReqSdioaudio::B1
    }
}
#[doc = "Field `IDLE_REQ_SDIOAUDIO` writer - send idle request to sdioaudio low power interface"]
pub type IdleReqSdioaudioW<'a, REG> = crate::BitWriter<'a, REG, IdleReqSdioaudio>;
impl<'a, REG> IdleReqSdioaudioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqSdioaudio::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IdleReqSdioaudio::B1)
    }
}
impl R {
    #[doc = "Bit 0 - send idle request to gpu niu"]
    #[inline(always)]
    pub fn idle_req_gpu(&self) -> IdleReqGpuR {
        IdleReqGpuR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - send idle request to perilp niu"]
    #[inline(always)]
    pub fn idle_req_perilp(&self) -> IdleReqPerilpR {
        IdleReqPerilpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - send idle request to perihp niu"]
    #[inline(always)]
    pub fn idle_req_perihp(&self) -> IdleReqPerihpR {
        IdleReqPerihpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - send idle request to vcodec niu"]
    #[inline(always)]
    pub fn idle_req_vcodec(&self) -> IdleReqVcodecR {
        IdleReqVcodecR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - send idle request to vdu niu"]
    #[inline(always)]
    pub fn idle_req_vdu(&self) -> IdleReqVduR {
        IdleReqVduR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - send idle request to rga niu"]
    #[inline(always)]
    pub fn idle_req_rga(&self) -> IdleReqRgaR {
        IdleReqRgaR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - send idle request to iep niu"]
    #[inline(always)]
    pub fn idle_req_iep(&self) -> IdleReqIepR {
        IdleReqIepR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - send idle request to vopb niu"]
    #[inline(always)]
    pub fn idle_req_vopb(&self) -> IdleReqVopbR {
        IdleReqVopbR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - send idle request to vopl niu"]
    #[inline(always)]
    pub fn idle_req_vopl(&self) -> IdleReqVoplR {
        IdleReqVoplR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - send idle request to isp0 niu"]
    #[inline(always)]
    pub fn idle_req_isp0(&self) -> IdleReqIsp0R {
        IdleReqIsp0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - send idle request to isp1 niu"]
    #[inline(always)]
    pub fn idle_req_isp1(&self) -> IdleReqIsp1R {
        IdleReqIsp1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - send idle request to hdcp niu"]
    #[inline(always)]
    pub fn idle_req_hdcp(&self) -> IdleReqHdcpR {
        IdleReqHdcpR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - send idle request to usb3 niu"]
    #[inline(always)]
    pub fn idle_req_usb3(&self) -> IdleReqUsb3R {
        IdleReqUsb3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - send idle request to perilp m0 niu"]
    #[inline(always)]
    pub fn idle_req_perilpm0(&self) -> IdleReqPerilpm0R {
        IdleReqPerilpm0R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - send idle request to center niu"]
    #[inline(always)]
    pub fn idle_req_center(&self) -> IdleReqCenterR {
        IdleReqCenterR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - send idle request to ccim0 low power interface"]
    #[inline(always)]
    pub fn idle_req_ccim0(&self) -> IdleReqCcim0R {
        IdleReqCcim0R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - send idle request to ccim1 low power interface"]
    #[inline(always)]
    pub fn idle_req_ccim1(&self) -> IdleReqCcim1R {
        IdleReqCcim1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - send idle request to vio low power interface"]
    #[inline(always)]
    pub fn idle_req_vio(&self) -> IdleReqVioR {
        IdleReqVioR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - send idle request to msch0 low power interface"]
    #[inline(always)]
    pub fn idle_req_msch0(&self) -> IdleReqMsch0R {
        IdleReqMsch0R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - send idle request to msch1 low power interface"]
    #[inline(always)]
    pub fn idle_req_msch1(&self) -> IdleReqMsch1R {
        IdleReqMsch1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - send idle request to alive low power interface"]
    #[inline(always)]
    pub fn idle_req_alive(&self) -> IdleReqAliveR {
        IdleReqAliveR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - send idle request to pmu low power interface"]
    #[inline(always)]
    pub fn idle_req_pmu(&self) -> IdleReqPmuR {
        IdleReqPmuR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - send idle request to edp low power interface"]
    #[inline(always)]
    pub fn idle_req_edp(&self) -> IdleReqEdpR {
        IdleReqEdpR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - send idle request to gmac low power interface"]
    #[inline(always)]
    pub fn idle_req_gmac(&self) -> IdleReqGmacR {
        IdleReqGmacR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - send idle request to emmc low power interface"]
    #[inline(always)]
    pub fn idle_req_emmc(&self) -> IdleReqEmmcR {
        IdleReqEmmcR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - send idle request to center1 low power interface"]
    #[inline(always)]
    pub fn idle_req_center1(&self) -> IdleReqCenter1R {
        IdleReqCenter1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - send idle request to pmu m0 low power interface"]
    #[inline(always)]
    pub fn idle_req_pmum0(&self) -> IdleReqPmum0R {
        IdleReqPmum0R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - send idle request to gic low power interface"]
    #[inline(always)]
    pub fn idle_req_gic(&self) -> IdleReqGicR {
        IdleReqGicR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - send idle request to sd low power interface"]
    #[inline(always)]
    pub fn idle_req_sd(&self) -> IdleReqSdR {
        IdleReqSdR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - send idle request to sdioaudio low power interface"]
    #[inline(always)]
    pub fn idle_req_sdioaudio(&self) -> IdleReqSdioaudioR {
        IdleReqSdioaudioR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - send idle request to gpu niu"]
    #[inline(always)]
    #[must_use]
    pub fn idle_req_gpu(&mut self) -> IdleReqGpuW<PmuBusIdleReqSpec> {
        IdleReqGpuW::new(self, 0)
    }
    #[doc = "Bit 1 - send idle request to perilp niu"]
    #[inline(always)]
    #[must_use]
    pub fn idle_req_perilp(&mut self) -> IdleReqPerilpW<PmuBusIdleReqSpec> {
        IdleReqPerilpW::new(self, 1)
    }
    #[doc = "Bit 2 - send idle request to perihp niu"]
    #[inline(always)]
    #[must_use]
    pub fn idle_req_perihp(&mut self) -> IdleReqPerihpW<PmuBusIdleReqSpec> {
        IdleReqPerihpW::new(self, 2)
    }
    #[doc = "Bit 3 - send idle request to vcodec niu"]
    #[inline(always)]
    #[must_use]
    pub fn idle_req_vcodec(&mut self) -> IdleReqVcodecW<PmuBusIdleReqSpec> {
        IdleReqVcodecW::new(self, 3)
    }
    #[doc = "Bit 4 - send idle request to vdu niu"]
    #[inline(always)]
    #[must_use]
    pub fn idle_req_vdu(&mut self) -> IdleReqVduW<PmuBusIdleReqSpec> {
        IdleReqVduW::new(self, 4)
    }
    #[doc = "Bit 5 - send idle request to rga niu"]
    #[inline(always)]
    #[must_use]
    pub fn idle_req_rga(&mut self) -> IdleReqRgaW<PmuBusIdleReqSpec> {
        IdleReqRgaW::new(self, 5)
    }
    #[doc = "Bit 6 - send idle request to iep niu"]
    #[inline(always)]
    #[must_use]
    pub fn idle_req_iep(&mut self) -> IdleReqIepW<PmuBusIdleReqSpec> {
        IdleReqIepW::new(self, 6)
    }
    #[doc = "Bit 7 - send idle request to vopb niu"]
    #[inline(always)]
    #[must_use]
    pub fn idle_req_vopb(&mut self) -> IdleReqVopbW<PmuBusIdleReqSpec> {
        IdleReqVopbW::new(self, 7)
    }
    #[doc = "Bit 8 - send idle request to vopl niu"]
    #[inline(always)]
    #[must_use]
    pub fn idle_req_vopl(&mut self) -> IdleReqVoplW<PmuBusIdleReqSpec> {
        IdleReqVoplW::new(self, 8)
    }
    #[doc = "Bit 9 - send idle request to isp0 niu"]
    #[inline(always)]
    #[must_use]
    pub fn idle_req_isp0(&mut self) -> IdleReqIsp0W<PmuBusIdleReqSpec> {
        IdleReqIsp0W::new(self, 9)
    }
    #[doc = "Bit 10 - send idle request to isp1 niu"]
    #[inline(always)]
    #[must_use]
    pub fn idle_req_isp1(&mut self) -> IdleReqIsp1W<PmuBusIdleReqSpec> {
        IdleReqIsp1W::new(self, 10)
    }
    #[doc = "Bit 11 - send idle request to hdcp niu"]
    #[inline(always)]
    #[must_use]
    pub fn idle_req_hdcp(&mut self) -> IdleReqHdcpW<PmuBusIdleReqSpec> {
        IdleReqHdcpW::new(self, 11)
    }
    #[doc = "Bit 12 - send idle request to usb3 niu"]
    #[inline(always)]
    #[must_use]
    pub fn idle_req_usb3(&mut self) -> IdleReqUsb3W<PmuBusIdleReqSpec> {
        IdleReqUsb3W::new(self, 12)
    }
    #[doc = "Bit 13 - send idle request to perilp m0 niu"]
    #[inline(always)]
    #[must_use]
    pub fn idle_req_perilpm0(&mut self) -> IdleReqPerilpm0W<PmuBusIdleReqSpec> {
        IdleReqPerilpm0W::new(self, 13)
    }
    #[doc = "Bit 14 - send idle request to center niu"]
    #[inline(always)]
    #[must_use]
    pub fn idle_req_center(&mut self) -> IdleReqCenterW<PmuBusIdleReqSpec> {
        IdleReqCenterW::new(self, 14)
    }
    #[doc = "Bit 15 - send idle request to ccim0 low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn idle_req_ccim0(&mut self) -> IdleReqCcim0W<PmuBusIdleReqSpec> {
        IdleReqCcim0W::new(self, 15)
    }
    #[doc = "Bit 16 - send idle request to ccim1 low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn idle_req_ccim1(&mut self) -> IdleReqCcim1W<PmuBusIdleReqSpec> {
        IdleReqCcim1W::new(self, 16)
    }
    #[doc = "Bit 17 - send idle request to vio low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn idle_req_vio(&mut self) -> IdleReqVioW<PmuBusIdleReqSpec> {
        IdleReqVioW::new(self, 17)
    }
    #[doc = "Bit 18 - send idle request to msch0 low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn idle_req_msch0(&mut self) -> IdleReqMsch0W<PmuBusIdleReqSpec> {
        IdleReqMsch0W::new(self, 18)
    }
    #[doc = "Bit 19 - send idle request to msch1 low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn idle_req_msch1(&mut self) -> IdleReqMsch1W<PmuBusIdleReqSpec> {
        IdleReqMsch1W::new(self, 19)
    }
    #[doc = "Bit 20 - send idle request to alive low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn idle_req_alive(&mut self) -> IdleReqAliveW<PmuBusIdleReqSpec> {
        IdleReqAliveW::new(self, 20)
    }
    #[doc = "Bit 21 - send idle request to pmu low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn idle_req_pmu(&mut self) -> IdleReqPmuW<PmuBusIdleReqSpec> {
        IdleReqPmuW::new(self, 21)
    }
    #[doc = "Bit 22 - send idle request to edp low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn idle_req_edp(&mut self) -> IdleReqEdpW<PmuBusIdleReqSpec> {
        IdleReqEdpW::new(self, 22)
    }
    #[doc = "Bit 23 - send idle request to gmac low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn idle_req_gmac(&mut self) -> IdleReqGmacW<PmuBusIdleReqSpec> {
        IdleReqGmacW::new(self, 23)
    }
    #[doc = "Bit 24 - send idle request to emmc low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn idle_req_emmc(&mut self) -> IdleReqEmmcW<PmuBusIdleReqSpec> {
        IdleReqEmmcW::new(self, 24)
    }
    #[doc = "Bit 25 - send idle request to center1 low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn idle_req_center1(&mut self) -> IdleReqCenter1W<PmuBusIdleReqSpec> {
        IdleReqCenter1W::new(self, 25)
    }
    #[doc = "Bit 26 - send idle request to pmu m0 low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn idle_req_pmum0(&mut self) -> IdleReqPmum0W<PmuBusIdleReqSpec> {
        IdleReqPmum0W::new(self, 26)
    }
    #[doc = "Bit 27 - send idle request to gic low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn idle_req_gic(&mut self) -> IdleReqGicW<PmuBusIdleReqSpec> {
        IdleReqGicW::new(self, 27)
    }
    #[doc = "Bit 28 - send idle request to sd low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn idle_req_sd(&mut self) -> IdleReqSdW<PmuBusIdleReqSpec> {
        IdleReqSdW::new(self, 28)
    }
    #[doc = "Bit 29 - send idle request to sdioaudio low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn idle_req_sdioaudio(&mut self) -> IdleReqSdioaudioW<PmuBusIdleReqSpec> {
        IdleReqSdioaudioW::new(self, 29)
    }
}
#[doc = "pmu bus idle request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_bus_idle_req::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_bus_idle_req::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuBusIdleReqSpec;
impl crate::RegisterSpec for PmuBusIdleReqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmu_bus_idle_req::R`](R) reader structure"]
impl crate::Readable for PmuBusIdleReqSpec {}
#[doc = "`write(|w| ..)` method takes [`pmu_bus_idle_req::W`](W) writer structure"]
impl crate::Writable for PmuBusIdleReqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMU_BUS_IDLE_REQ to value 0"]
impl crate::Resettable for PmuBusIdleReqSpec {
    const RESET_VALUE: u32 = 0;
}
