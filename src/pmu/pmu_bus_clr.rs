#[doc = "Register `PMU_BUS_CLR` reader"]
pub type R = crate::R<PmuBusClrSpec>;
#[doc = "Register `PMU_BUS_CLR` writer"]
pub type W = crate::W<PmuBusClrSpec>;
#[doc = "send idle request to gpu niu\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrGpu {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClrGpu> for bool {
    #[inline(always)]
    fn from(variant: ClrGpu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_GPU` reader - send idle request to gpu niu"]
pub type ClrGpuR = crate::BitReader<ClrGpu>;
impl ClrGpuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrGpu {
        match self.bits {
            false => ClrGpu::B0,
            true => ClrGpu::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClrGpu::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClrGpu::B1
    }
}
#[doc = "Field `CLR_GPU` writer - send idle request to gpu niu"]
pub type ClrGpuW<'a, REG> = crate::BitWriter<'a, REG, ClrGpu>;
impl<'a, REG> ClrGpuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrGpu::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrGpu::B1)
    }
}
#[doc = "send idle request to perilp niu\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrPerilp {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClrPerilp> for bool {
    #[inline(always)]
    fn from(variant: ClrPerilp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_PERILP` reader - send idle request to perilp niu"]
pub type ClrPerilpR = crate::BitReader<ClrPerilp>;
impl ClrPerilpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrPerilp {
        match self.bits {
            false => ClrPerilp::B0,
            true => ClrPerilp::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClrPerilp::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClrPerilp::B1
    }
}
#[doc = "Field `CLR_PERILP` writer - send idle request to perilp niu"]
pub type ClrPerilpW<'a, REG> = crate::BitWriter<'a, REG, ClrPerilp>;
impl<'a, REG> ClrPerilpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrPerilp::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrPerilp::B1)
    }
}
#[doc = "send idle request to perihp niu\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrPerihp {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClrPerihp> for bool {
    #[inline(always)]
    fn from(variant: ClrPerihp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_PERIHP` reader - send idle request to perihp niu"]
pub type ClrPerihpR = crate::BitReader<ClrPerihp>;
impl ClrPerihpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrPerihp {
        match self.bits {
            false => ClrPerihp::B0,
            true => ClrPerihp::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClrPerihp::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClrPerihp::B1
    }
}
#[doc = "Field `CLR_PERIHP` writer - send idle request to perihp niu"]
pub type ClrPerihpW<'a, REG> = crate::BitWriter<'a, REG, ClrPerihp>;
impl<'a, REG> ClrPerihpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrPerihp::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrPerihp::B1)
    }
}
#[doc = "send idle request to vcodec niu\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrVcodec {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClrVcodec> for bool {
    #[inline(always)]
    fn from(variant: ClrVcodec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_VCODEC` reader - send idle request to vcodec niu"]
pub type ClrVcodecR = crate::BitReader<ClrVcodec>;
impl ClrVcodecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrVcodec {
        match self.bits {
            false => ClrVcodec::B0,
            true => ClrVcodec::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClrVcodec::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClrVcodec::B1
    }
}
#[doc = "Field `CLR_VCODEC` writer - send idle request to vcodec niu"]
pub type ClrVcodecW<'a, REG> = crate::BitWriter<'a, REG, ClrVcodec>;
impl<'a, REG> ClrVcodecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrVcodec::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrVcodec::B1)
    }
}
#[doc = "send idle request to vdu niu\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrVdu {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClrVdu> for bool {
    #[inline(always)]
    fn from(variant: ClrVdu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_VDU` reader - send idle request to vdu niu"]
pub type ClrVduR = crate::BitReader<ClrVdu>;
impl ClrVduR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrVdu {
        match self.bits {
            false => ClrVdu::B0,
            true => ClrVdu::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClrVdu::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClrVdu::B1
    }
}
#[doc = "Field `CLR_VDU` writer - send idle request to vdu niu"]
pub type ClrVduW<'a, REG> = crate::BitWriter<'a, REG, ClrVdu>;
impl<'a, REG> ClrVduW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrVdu::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrVdu::B1)
    }
}
#[doc = "send idle request to rga niu\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrRga {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClrRga> for bool {
    #[inline(always)]
    fn from(variant: ClrRga) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_RGA` reader - send idle request to rga niu"]
pub type ClrRgaR = crate::BitReader<ClrRga>;
impl ClrRgaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrRga {
        match self.bits {
            false => ClrRga::B0,
            true => ClrRga::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClrRga::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClrRga::B1
    }
}
#[doc = "Field `CLR_RGA` writer - send idle request to rga niu"]
pub type ClrRgaW<'a, REG> = crate::BitWriter<'a, REG, ClrRga>;
impl<'a, REG> ClrRgaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrRga::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrRga::B1)
    }
}
#[doc = "send idle request to iep niu\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrIep {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClrIep> for bool {
    #[inline(always)]
    fn from(variant: ClrIep) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_IEP` reader - send idle request to iep niu"]
pub type ClrIepR = crate::BitReader<ClrIep>;
impl ClrIepR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrIep {
        match self.bits {
            false => ClrIep::B0,
            true => ClrIep::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClrIep::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClrIep::B1
    }
}
#[doc = "Field `CLR_IEP` writer - send idle request to iep niu"]
pub type ClrIepW<'a, REG> = crate::BitWriter<'a, REG, ClrIep>;
impl<'a, REG> ClrIepW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrIep::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrIep::B1)
    }
}
#[doc = "send idle request to vopb niu\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrVopb {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClrVopb> for bool {
    #[inline(always)]
    fn from(variant: ClrVopb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_VOPB` reader - send idle request to vopb niu"]
pub type ClrVopbR = crate::BitReader<ClrVopb>;
impl ClrVopbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrVopb {
        match self.bits {
            false => ClrVopb::B0,
            true => ClrVopb::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClrVopb::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClrVopb::B1
    }
}
#[doc = "Field `CLR_VOPB` writer - send idle request to vopb niu"]
pub type ClrVopbW<'a, REG> = crate::BitWriter<'a, REG, ClrVopb>;
impl<'a, REG> ClrVopbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrVopb::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrVopb::B1)
    }
}
#[doc = "send idle request to vopl niu\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrVopl {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClrVopl> for bool {
    #[inline(always)]
    fn from(variant: ClrVopl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_VOPL` reader - send idle request to vopl niu"]
pub type ClrVoplR = crate::BitReader<ClrVopl>;
impl ClrVoplR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrVopl {
        match self.bits {
            false => ClrVopl::B0,
            true => ClrVopl::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClrVopl::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClrVopl::B1
    }
}
#[doc = "Field `CLR_VOPL` writer - send idle request to vopl niu"]
pub type ClrVoplW<'a, REG> = crate::BitWriter<'a, REG, ClrVopl>;
impl<'a, REG> ClrVoplW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrVopl::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrVopl::B1)
    }
}
#[doc = "send idle request to isp0 niu\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrIsp0 {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClrIsp0> for bool {
    #[inline(always)]
    fn from(variant: ClrIsp0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_ISP0` reader - send idle request to isp0 niu"]
pub type ClrIsp0R = crate::BitReader<ClrIsp0>;
impl ClrIsp0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrIsp0 {
        match self.bits {
            false => ClrIsp0::B0,
            true => ClrIsp0::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClrIsp0::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClrIsp0::B1
    }
}
#[doc = "Field `CLR_ISP0` writer - send idle request to isp0 niu"]
pub type ClrIsp0W<'a, REG> = crate::BitWriter<'a, REG, ClrIsp0>;
impl<'a, REG> ClrIsp0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrIsp0::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrIsp0::B1)
    }
}
#[doc = "send idle request to isp1 niu\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrIsp1 {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClrIsp1> for bool {
    #[inline(always)]
    fn from(variant: ClrIsp1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_ISP1` reader - send idle request to isp1 niu"]
pub type ClrIsp1R = crate::BitReader<ClrIsp1>;
impl ClrIsp1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrIsp1 {
        match self.bits {
            false => ClrIsp1::B0,
            true => ClrIsp1::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClrIsp1::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClrIsp1::B1
    }
}
#[doc = "Field `CLR_ISP1` writer - send idle request to isp1 niu"]
pub type ClrIsp1W<'a, REG> = crate::BitWriter<'a, REG, ClrIsp1>;
impl<'a, REG> ClrIsp1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrIsp1::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrIsp1::B1)
    }
}
#[doc = "send idle request to hdcp niu\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrHdcp {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClrHdcp> for bool {
    #[inline(always)]
    fn from(variant: ClrHdcp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_HDCP` reader - send idle request to hdcp niu"]
pub type ClrHdcpR = crate::BitReader<ClrHdcp>;
impl ClrHdcpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrHdcp {
        match self.bits {
            false => ClrHdcp::B0,
            true => ClrHdcp::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClrHdcp::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClrHdcp::B1
    }
}
#[doc = "Field `CLR_HDCP` writer - send idle request to hdcp niu"]
pub type ClrHdcpW<'a, REG> = crate::BitWriter<'a, REG, ClrHdcp>;
impl<'a, REG> ClrHdcpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrHdcp::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrHdcp::B1)
    }
}
#[doc = "send idle request to usb3 niu\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrUsb3 {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClrUsb3> for bool {
    #[inline(always)]
    fn from(variant: ClrUsb3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_USB3` reader - send idle request to usb3 niu"]
pub type ClrUsb3R = crate::BitReader<ClrUsb3>;
impl ClrUsb3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrUsb3 {
        match self.bits {
            false => ClrUsb3::B0,
            true => ClrUsb3::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClrUsb3::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClrUsb3::B1
    }
}
#[doc = "Field `CLR_USB3` writer - send idle request to usb3 niu"]
pub type ClrUsb3W<'a, REG> = crate::BitWriter<'a, REG, ClrUsb3>;
impl<'a, REG> ClrUsb3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrUsb3::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrUsb3::B1)
    }
}
#[doc = "send idle request to perilp m0 niu\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrPerilpm0 {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClrPerilpm0> for bool {
    #[inline(always)]
    fn from(variant: ClrPerilpm0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_PERILPM0` reader - send idle request to perilp m0 niu"]
pub type ClrPerilpm0R = crate::BitReader<ClrPerilpm0>;
impl ClrPerilpm0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrPerilpm0 {
        match self.bits {
            false => ClrPerilpm0::B0,
            true => ClrPerilpm0::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClrPerilpm0::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClrPerilpm0::B1
    }
}
#[doc = "Field `CLR_PERILPM0` writer - send idle request to perilp m0 niu"]
pub type ClrPerilpm0W<'a, REG> = crate::BitWriter<'a, REG, ClrPerilpm0>;
impl<'a, REG> ClrPerilpm0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrPerilpm0::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrPerilpm0::B1)
    }
}
#[doc = "send idle request to center niu\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrCenter {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClrCenter> for bool {
    #[inline(always)]
    fn from(variant: ClrCenter) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_CENTER` reader - send idle request to center niu"]
pub type ClrCenterR = crate::BitReader<ClrCenter>;
impl ClrCenterR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrCenter {
        match self.bits {
            false => ClrCenter::B0,
            true => ClrCenter::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClrCenter::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClrCenter::B1
    }
}
#[doc = "Field `CLR_CENTER` writer - send idle request to center niu"]
pub type ClrCenterW<'a, REG> = crate::BitWriter<'a, REG, ClrCenter>;
impl<'a, REG> ClrCenterW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrCenter::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrCenter::B1)
    }
}
#[doc = "send idle request to ccim1 low power interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrCcim1 {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClrCcim1> for bool {
    #[inline(always)]
    fn from(variant: ClrCcim1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_CCIM1` reader - send idle request to ccim1 low power interface"]
pub type ClrCcim1R = crate::BitReader<ClrCcim1>;
impl ClrCcim1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrCcim1 {
        match self.bits {
            false => ClrCcim1::B0,
            true => ClrCcim1::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClrCcim1::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClrCcim1::B1
    }
}
#[doc = "Field `CLR_CCIM1` writer - send idle request to ccim1 low power interface"]
pub type ClrCcim1W<'a, REG> = crate::BitWriter<'a, REG, ClrCcim1>;
impl<'a, REG> ClrCcim1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrCcim1::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrCcim1::B1)
    }
}
#[doc = "send idle request to ccim0 low power interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrCcim0 {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClrCcim0> for bool {
    #[inline(always)]
    fn from(variant: ClrCcim0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_CCIM0` reader - send idle request to ccim0 low power interface"]
pub type ClrCcim0R = crate::BitReader<ClrCcim0>;
impl ClrCcim0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrCcim0 {
        match self.bits {
            false => ClrCcim0::B0,
            true => ClrCcim0::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClrCcim0::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClrCcim0::B1
    }
}
#[doc = "Field `CLR_CCIM0` writer - send idle request to ccim0 low power interface"]
pub type ClrCcim0W<'a, REG> = crate::BitWriter<'a, REG, ClrCcim0>;
impl<'a, REG> ClrCcim0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrCcim0::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrCcim0::B1)
    }
}
#[doc = "send idle request to vio low power interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrVio {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClrVio> for bool {
    #[inline(always)]
    fn from(variant: ClrVio) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_VIO` reader - send idle request to vio low power interface"]
pub type ClrVioR = crate::BitReader<ClrVio>;
impl ClrVioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrVio {
        match self.bits {
            false => ClrVio::B0,
            true => ClrVio::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClrVio::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClrVio::B1
    }
}
#[doc = "Field `CLR_VIO` writer - send idle request to vio low power interface"]
pub type ClrVioW<'a, REG> = crate::BitWriter<'a, REG, ClrVio>;
impl<'a, REG> ClrVioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrVio::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrVio::B1)
    }
}
#[doc = "send idle request to msch0 low power interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrMsch0 {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClrMsch0> for bool {
    #[inline(always)]
    fn from(variant: ClrMsch0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_MSCH0` reader - send idle request to msch0 low power interface"]
pub type ClrMsch0R = crate::BitReader<ClrMsch0>;
impl ClrMsch0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrMsch0 {
        match self.bits {
            false => ClrMsch0::B0,
            true => ClrMsch0::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClrMsch0::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClrMsch0::B1
    }
}
#[doc = "Field `CLR_MSCH0` writer - send idle request to msch0 low power interface"]
pub type ClrMsch0W<'a, REG> = crate::BitWriter<'a, REG, ClrMsch0>;
impl<'a, REG> ClrMsch0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrMsch0::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrMsch0::B1)
    }
}
#[doc = "send idle request to msch1 low power interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrMsch1 {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClrMsch1> for bool {
    #[inline(always)]
    fn from(variant: ClrMsch1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_MSCH1` reader - send idle request to msch1 low power interface"]
pub type ClrMsch1R = crate::BitReader<ClrMsch1>;
impl ClrMsch1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrMsch1 {
        match self.bits {
            false => ClrMsch1::B0,
            true => ClrMsch1::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClrMsch1::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClrMsch1::B1
    }
}
#[doc = "Field `CLR_MSCH1` writer - send idle request to msch1 low power interface"]
pub type ClrMsch1W<'a, REG> = crate::BitWriter<'a, REG, ClrMsch1>;
impl<'a, REG> ClrMsch1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrMsch1::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrMsch1::B1)
    }
}
#[doc = "send idle request to alive low power interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrAlive {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClrAlive> for bool {
    #[inline(always)]
    fn from(variant: ClrAlive) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_ALIVE` reader - send idle request to alive low power interface"]
pub type ClrAliveR = crate::BitReader<ClrAlive>;
impl ClrAliveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrAlive {
        match self.bits {
            false => ClrAlive::B0,
            true => ClrAlive::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClrAlive::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClrAlive::B1
    }
}
#[doc = "Field `CLR_ALIVE` writer - send idle request to alive low power interface"]
pub type ClrAliveW<'a, REG> = crate::BitWriter<'a, REG, ClrAlive>;
impl<'a, REG> ClrAliveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrAlive::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrAlive::B1)
    }
}
#[doc = "send idle request to pmu low power interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrPmu {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClrPmu> for bool {
    #[inline(always)]
    fn from(variant: ClrPmu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_PMU` reader - send idle request to pmu low power interface"]
pub type ClrPmuR = crate::BitReader<ClrPmu>;
impl ClrPmuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrPmu {
        match self.bits {
            false => ClrPmu::B0,
            true => ClrPmu::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClrPmu::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClrPmu::B1
    }
}
#[doc = "Field `CLR_PMU` writer - send idle request to pmu low power interface"]
pub type ClrPmuW<'a, REG> = crate::BitWriter<'a, REG, ClrPmu>;
impl<'a, REG> ClrPmuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrPmu::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrPmu::B1)
    }
}
#[doc = "send idle request to edp low power interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrEdp {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClrEdp> for bool {
    #[inline(always)]
    fn from(variant: ClrEdp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_EDP` reader - send idle request to edp low power interface"]
pub type ClrEdpR = crate::BitReader<ClrEdp>;
impl ClrEdpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrEdp {
        match self.bits {
            false => ClrEdp::B0,
            true => ClrEdp::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClrEdp::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClrEdp::B1
    }
}
#[doc = "Field `CLR_EDP` writer - send idle request to edp low power interface"]
pub type ClrEdpW<'a, REG> = crate::BitWriter<'a, REG, ClrEdp>;
impl<'a, REG> ClrEdpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrEdp::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrEdp::B1)
    }
}
#[doc = "send idle request to gmac low power interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrGmac {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClrGmac> for bool {
    #[inline(always)]
    fn from(variant: ClrGmac) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_GMAC` reader - send idle request to gmac low power interface"]
pub type ClrGmacR = crate::BitReader<ClrGmac>;
impl ClrGmacR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrGmac {
        match self.bits {
            false => ClrGmac::B0,
            true => ClrGmac::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClrGmac::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClrGmac::B1
    }
}
#[doc = "Field `CLR_GMAC` writer - send idle request to gmac low power interface"]
pub type ClrGmacW<'a, REG> = crate::BitWriter<'a, REG, ClrGmac>;
impl<'a, REG> ClrGmacW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrGmac::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrGmac::B1)
    }
}
#[doc = "send idle request to emmc low power interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrEmmc {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClrEmmc> for bool {
    #[inline(always)]
    fn from(variant: ClrEmmc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_EMMC` reader - send idle request to emmc low power interface"]
pub type ClrEmmcR = crate::BitReader<ClrEmmc>;
impl ClrEmmcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrEmmc {
        match self.bits {
            false => ClrEmmc::B0,
            true => ClrEmmc::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClrEmmc::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClrEmmc::B1
    }
}
#[doc = "Field `CLR_EMMC` writer - send idle request to emmc low power interface"]
pub type ClrEmmcW<'a, REG> = crate::BitWriter<'a, REG, ClrEmmc>;
impl<'a, REG> ClrEmmcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrEmmc::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrEmmc::B1)
    }
}
#[doc = "send idle request to center1 low power interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrCenter1 {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClrCenter1> for bool {
    #[inline(always)]
    fn from(variant: ClrCenter1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_CENTER1` reader - send idle request to center1 low power interface"]
pub type ClrCenter1R = crate::BitReader<ClrCenter1>;
impl ClrCenter1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrCenter1 {
        match self.bits {
            false => ClrCenter1::B0,
            true => ClrCenter1::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClrCenter1::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClrCenter1::B1
    }
}
#[doc = "Field `CLR_CENTER1` writer - send idle request to center1 low power interface"]
pub type ClrCenter1W<'a, REG> = crate::BitWriter<'a, REG, ClrCenter1>;
impl<'a, REG> ClrCenter1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrCenter1::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrCenter1::B1)
    }
}
#[doc = "send idle request to pmu m0 low power interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrPmum0 {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClrPmum0> for bool {
    #[inline(always)]
    fn from(variant: ClrPmum0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_PMUM0` reader - send idle request to pmu m0 low power interface"]
pub type ClrPmum0R = crate::BitReader<ClrPmum0>;
impl ClrPmum0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrPmum0 {
        match self.bits {
            false => ClrPmum0::B0,
            true => ClrPmum0::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClrPmum0::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClrPmum0::B1
    }
}
#[doc = "Field `CLR_PMUM0` writer - send idle request to pmu m0 low power interface"]
pub type ClrPmum0W<'a, REG> = crate::BitWriter<'a, REG, ClrPmum0>;
impl<'a, REG> ClrPmum0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrPmum0::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrPmum0::B1)
    }
}
#[doc = "send idle request to gic low power interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrGic {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClrGic> for bool {
    #[inline(always)]
    fn from(variant: ClrGic) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_GIC` reader - send idle request to gic low power interface"]
pub type ClrGicR = crate::BitReader<ClrGic>;
impl ClrGicR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrGic {
        match self.bits {
            false => ClrGic::B0,
            true => ClrGic::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClrGic::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClrGic::B1
    }
}
#[doc = "Field `CLR_GIC` writer - send idle request to gic low power interface"]
pub type ClrGicW<'a, REG> = crate::BitWriter<'a, REG, ClrGic>;
impl<'a, REG> ClrGicW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrGic::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrGic::B1)
    }
}
#[doc = "send idle request to sd low power interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrSd {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClrSd> for bool {
    #[inline(always)]
    fn from(variant: ClrSd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_SD` reader - send idle request to sd low power interface"]
pub type ClrSdR = crate::BitReader<ClrSd>;
impl ClrSdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrSd {
        match self.bits {
            false => ClrSd::B0,
            true => ClrSd::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClrSd::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClrSd::B1
    }
}
#[doc = "Field `CLR_SD` writer - send idle request to sd low power interface"]
pub type ClrSdW<'a, REG> = crate::BitWriter<'a, REG, ClrSd>;
impl<'a, REG> ClrSdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrSd::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrSd::B1)
    }
}
#[doc = "send idle request to sdioaudio low power interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrSdioaudio {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClrSdioaudio> for bool {
    #[inline(always)]
    fn from(variant: ClrSdioaudio) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_SDIOAUDIO` reader - send idle request to sdioaudio low power interface"]
pub type ClrSdioaudioR = crate::BitReader<ClrSdioaudio>;
impl ClrSdioaudioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrSdioaudio {
        match self.bits {
            false => ClrSdioaudio::B0,
            true => ClrSdioaudio::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClrSdioaudio::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClrSdioaudio::B1
    }
}
#[doc = "Field `CLR_SDIOAUDIO` writer - send idle request to sdioaudio low power interface"]
pub type ClrSdioaudioW<'a, REG> = crate::BitWriter<'a, REG, ClrSdioaudio>;
impl<'a, REG> ClrSdioaudioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrSdioaudio::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrSdioaudio::B1)
    }
}
impl R {
    #[doc = "Bit 0 - send idle request to gpu niu"]
    #[inline(always)]
    pub fn clr_gpu(&self) -> ClrGpuR {
        ClrGpuR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - send idle request to perilp niu"]
    #[inline(always)]
    pub fn clr_perilp(&self) -> ClrPerilpR {
        ClrPerilpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - send idle request to perihp niu"]
    #[inline(always)]
    pub fn clr_perihp(&self) -> ClrPerihpR {
        ClrPerihpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - send idle request to vcodec niu"]
    #[inline(always)]
    pub fn clr_vcodec(&self) -> ClrVcodecR {
        ClrVcodecR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - send idle request to vdu niu"]
    #[inline(always)]
    pub fn clr_vdu(&self) -> ClrVduR {
        ClrVduR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - send idle request to rga niu"]
    #[inline(always)]
    pub fn clr_rga(&self) -> ClrRgaR {
        ClrRgaR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - send idle request to iep niu"]
    #[inline(always)]
    pub fn clr_iep(&self) -> ClrIepR {
        ClrIepR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - send idle request to vopb niu"]
    #[inline(always)]
    pub fn clr_vopb(&self) -> ClrVopbR {
        ClrVopbR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - send idle request to vopl niu"]
    #[inline(always)]
    pub fn clr_vopl(&self) -> ClrVoplR {
        ClrVoplR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - send idle request to isp0 niu"]
    #[inline(always)]
    pub fn clr_isp0(&self) -> ClrIsp0R {
        ClrIsp0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - send idle request to isp1 niu"]
    #[inline(always)]
    pub fn clr_isp1(&self) -> ClrIsp1R {
        ClrIsp1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - send idle request to hdcp niu"]
    #[inline(always)]
    pub fn clr_hdcp(&self) -> ClrHdcpR {
        ClrHdcpR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - send idle request to usb3 niu"]
    #[inline(always)]
    pub fn clr_usb3(&self) -> ClrUsb3R {
        ClrUsb3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - send idle request to perilp m0 niu"]
    #[inline(always)]
    pub fn clr_perilpm0(&self) -> ClrPerilpm0R {
        ClrPerilpm0R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - send idle request to center niu"]
    #[inline(always)]
    pub fn clr_center(&self) -> ClrCenterR {
        ClrCenterR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - send idle request to ccim1 low power interface"]
    #[inline(always)]
    pub fn clr_ccim1(&self) -> ClrCcim1R {
        ClrCcim1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - send idle request to ccim0 low power interface"]
    #[inline(always)]
    pub fn clr_ccim0(&self) -> ClrCcim0R {
        ClrCcim0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - send idle request to vio low power interface"]
    #[inline(always)]
    pub fn clr_vio(&self) -> ClrVioR {
        ClrVioR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - send idle request to msch0 low power interface"]
    #[inline(always)]
    pub fn clr_msch0(&self) -> ClrMsch0R {
        ClrMsch0R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - send idle request to msch1 low power interface"]
    #[inline(always)]
    pub fn clr_msch1(&self) -> ClrMsch1R {
        ClrMsch1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - send idle request to alive low power interface"]
    #[inline(always)]
    pub fn clr_alive(&self) -> ClrAliveR {
        ClrAliveR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - send idle request to pmu low power interface"]
    #[inline(always)]
    pub fn clr_pmu(&self) -> ClrPmuR {
        ClrPmuR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - send idle request to edp low power interface"]
    #[inline(always)]
    pub fn clr_edp(&self) -> ClrEdpR {
        ClrEdpR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - send idle request to gmac low power interface"]
    #[inline(always)]
    pub fn clr_gmac(&self) -> ClrGmacR {
        ClrGmacR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - send idle request to emmc low power interface"]
    #[inline(always)]
    pub fn clr_emmc(&self) -> ClrEmmcR {
        ClrEmmcR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - send idle request to center1 low power interface"]
    #[inline(always)]
    pub fn clr_center1(&self) -> ClrCenter1R {
        ClrCenter1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - send idle request to pmu m0 low power interface"]
    #[inline(always)]
    pub fn clr_pmum0(&self) -> ClrPmum0R {
        ClrPmum0R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - send idle request to gic low power interface"]
    #[inline(always)]
    pub fn clr_gic(&self) -> ClrGicR {
        ClrGicR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - send idle request to sd low power interface"]
    #[inline(always)]
    pub fn clr_sd(&self) -> ClrSdR {
        ClrSdR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - send idle request to sdioaudio low power interface"]
    #[inline(always)]
    pub fn clr_sdioaudio(&self) -> ClrSdioaudioR {
        ClrSdioaudioR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - send idle request to gpu niu"]
    #[inline(always)]
    #[must_use]
    pub fn clr_gpu(&mut self) -> ClrGpuW<PmuBusClrSpec> {
        ClrGpuW::new(self, 0)
    }
    #[doc = "Bit 1 - send idle request to perilp niu"]
    #[inline(always)]
    #[must_use]
    pub fn clr_perilp(&mut self) -> ClrPerilpW<PmuBusClrSpec> {
        ClrPerilpW::new(self, 1)
    }
    #[doc = "Bit 2 - send idle request to perihp niu"]
    #[inline(always)]
    #[must_use]
    pub fn clr_perihp(&mut self) -> ClrPerihpW<PmuBusClrSpec> {
        ClrPerihpW::new(self, 2)
    }
    #[doc = "Bit 3 - send idle request to vcodec niu"]
    #[inline(always)]
    #[must_use]
    pub fn clr_vcodec(&mut self) -> ClrVcodecW<PmuBusClrSpec> {
        ClrVcodecW::new(self, 3)
    }
    #[doc = "Bit 4 - send idle request to vdu niu"]
    #[inline(always)]
    #[must_use]
    pub fn clr_vdu(&mut self) -> ClrVduW<PmuBusClrSpec> {
        ClrVduW::new(self, 4)
    }
    #[doc = "Bit 5 - send idle request to rga niu"]
    #[inline(always)]
    #[must_use]
    pub fn clr_rga(&mut self) -> ClrRgaW<PmuBusClrSpec> {
        ClrRgaW::new(self, 5)
    }
    #[doc = "Bit 6 - send idle request to iep niu"]
    #[inline(always)]
    #[must_use]
    pub fn clr_iep(&mut self) -> ClrIepW<PmuBusClrSpec> {
        ClrIepW::new(self, 6)
    }
    #[doc = "Bit 7 - send idle request to vopb niu"]
    #[inline(always)]
    #[must_use]
    pub fn clr_vopb(&mut self) -> ClrVopbW<PmuBusClrSpec> {
        ClrVopbW::new(self, 7)
    }
    #[doc = "Bit 8 - send idle request to vopl niu"]
    #[inline(always)]
    #[must_use]
    pub fn clr_vopl(&mut self) -> ClrVoplW<PmuBusClrSpec> {
        ClrVoplW::new(self, 8)
    }
    #[doc = "Bit 9 - send idle request to isp0 niu"]
    #[inline(always)]
    #[must_use]
    pub fn clr_isp0(&mut self) -> ClrIsp0W<PmuBusClrSpec> {
        ClrIsp0W::new(self, 9)
    }
    #[doc = "Bit 10 - send idle request to isp1 niu"]
    #[inline(always)]
    #[must_use]
    pub fn clr_isp1(&mut self) -> ClrIsp1W<PmuBusClrSpec> {
        ClrIsp1W::new(self, 10)
    }
    #[doc = "Bit 11 - send idle request to hdcp niu"]
    #[inline(always)]
    #[must_use]
    pub fn clr_hdcp(&mut self) -> ClrHdcpW<PmuBusClrSpec> {
        ClrHdcpW::new(self, 11)
    }
    #[doc = "Bit 12 - send idle request to usb3 niu"]
    #[inline(always)]
    #[must_use]
    pub fn clr_usb3(&mut self) -> ClrUsb3W<PmuBusClrSpec> {
        ClrUsb3W::new(self, 12)
    }
    #[doc = "Bit 13 - send idle request to perilp m0 niu"]
    #[inline(always)]
    #[must_use]
    pub fn clr_perilpm0(&mut self) -> ClrPerilpm0W<PmuBusClrSpec> {
        ClrPerilpm0W::new(self, 13)
    }
    #[doc = "Bit 14 - send idle request to center niu"]
    #[inline(always)]
    #[must_use]
    pub fn clr_center(&mut self) -> ClrCenterW<PmuBusClrSpec> {
        ClrCenterW::new(self, 14)
    }
    #[doc = "Bit 15 - send idle request to ccim1 low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn clr_ccim1(&mut self) -> ClrCcim1W<PmuBusClrSpec> {
        ClrCcim1W::new(self, 15)
    }
    #[doc = "Bit 16 - send idle request to ccim0 low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn clr_ccim0(&mut self) -> ClrCcim0W<PmuBusClrSpec> {
        ClrCcim0W::new(self, 16)
    }
    #[doc = "Bit 17 - send idle request to vio low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn clr_vio(&mut self) -> ClrVioW<PmuBusClrSpec> {
        ClrVioW::new(self, 17)
    }
    #[doc = "Bit 18 - send idle request to msch0 low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn clr_msch0(&mut self) -> ClrMsch0W<PmuBusClrSpec> {
        ClrMsch0W::new(self, 18)
    }
    #[doc = "Bit 19 - send idle request to msch1 low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn clr_msch1(&mut self) -> ClrMsch1W<PmuBusClrSpec> {
        ClrMsch1W::new(self, 19)
    }
    #[doc = "Bit 20 - send idle request to alive low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn clr_alive(&mut self) -> ClrAliveW<PmuBusClrSpec> {
        ClrAliveW::new(self, 20)
    }
    #[doc = "Bit 21 - send idle request to pmu low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn clr_pmu(&mut self) -> ClrPmuW<PmuBusClrSpec> {
        ClrPmuW::new(self, 21)
    }
    #[doc = "Bit 22 - send idle request to edp low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn clr_edp(&mut self) -> ClrEdpW<PmuBusClrSpec> {
        ClrEdpW::new(self, 22)
    }
    #[doc = "Bit 23 - send idle request to gmac low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn clr_gmac(&mut self) -> ClrGmacW<PmuBusClrSpec> {
        ClrGmacW::new(self, 23)
    }
    #[doc = "Bit 24 - send idle request to emmc low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn clr_emmc(&mut self) -> ClrEmmcW<PmuBusClrSpec> {
        ClrEmmcW::new(self, 24)
    }
    #[doc = "Bit 25 - send idle request to center1 low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn clr_center1(&mut self) -> ClrCenter1W<PmuBusClrSpec> {
        ClrCenter1W::new(self, 25)
    }
    #[doc = "Bit 26 - send idle request to pmu m0 low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn clr_pmum0(&mut self) -> ClrPmum0W<PmuBusClrSpec> {
        ClrPmum0W::new(self, 26)
    }
    #[doc = "Bit 27 - send idle request to gic low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn clr_gic(&mut self) -> ClrGicW<PmuBusClrSpec> {
        ClrGicW::new(self, 27)
    }
    #[doc = "Bit 28 - send idle request to sd low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn clr_sd(&mut self) -> ClrSdW<PmuBusClrSpec> {
        ClrSdW::new(self, 28)
    }
    #[doc = "Bit 29 - send idle request to sdioaudio low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn clr_sdioaudio(&mut self) -> ClrSdioaudioW<PmuBusClrSpec> {
        ClrSdioaudioW::new(self, 29)
    }
}
#[doc = "pmu bus clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_bus_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_bus_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuBusClrSpec;
impl crate::RegisterSpec for PmuBusClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmu_bus_clr::R`](R) reader structure"]
impl crate::Readable for PmuBusClrSpec {}
#[doc = "`write(|w| ..)` method takes [`pmu_bus_clr::W`](W) writer structure"]
impl crate::Writable for PmuBusClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMU_BUS_CLR to value 0"]
impl crate::Resettable for PmuBusClrSpec {
    const RESET_VALUE: u32 = 0;
}
