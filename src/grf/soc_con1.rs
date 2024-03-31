#[doc = "Register `SOC_CON1` reader"]
pub type R = crate::R<SocCon1Spec>;
#[doc = "Register `SOC_CON1` writer"]
pub type W = crate::W<SocCon1Spec>;
#[doc = "noc_gpu_req_msch0_rsp_err_stall bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpuReqMsch0Pwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<GpuReqMsch0Pwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: GpuReqMsch0Pwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPU_REQ_MSCH0_PWRDISCTARGPWRSTALL` reader - noc_gpu_req_msch0_rsp_err_stall bit control"]
pub type GpuReqMsch0PwrdisctargpwrstallR = crate::BitReader<GpuReqMsch0Pwrdisctargpwrstall>;
impl GpuReqMsch0PwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpuReqMsch0Pwrdisctargpwrstall {
        match self.bits {
            false => GpuReqMsch0Pwrdisctargpwrstall::B0,
            true => GpuReqMsch0Pwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == GpuReqMsch0Pwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == GpuReqMsch0Pwrdisctargpwrstall::B1
    }
}
#[doc = "Field `GPU_REQ_MSCH0_PWRDISCTARGPWRSTALL` writer - noc_gpu_req_msch0_rsp_err_stall bit control"]
pub type GpuReqMsch0PwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, GpuReqMsch0Pwrdisctargpwrstall>;
impl<'a, REG> GpuReqMsch0PwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(GpuReqMsch0Pwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(GpuReqMsch0Pwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_gpu_req_msch1_rsp_err_stall bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpuReqMsch1Pwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<GpuReqMsch1Pwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: GpuReqMsch1Pwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPU_REQ_MSCH1_PWRDISCTARGPWRSTALL` reader - noc_gpu_req_msch1_rsp_err_stall bit control"]
pub type GpuReqMsch1PwrdisctargpwrstallR = crate::BitReader<GpuReqMsch1Pwrdisctargpwrstall>;
impl GpuReqMsch1PwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpuReqMsch1Pwrdisctargpwrstall {
        match self.bits {
            false => GpuReqMsch1Pwrdisctargpwrstall::B0,
            true => GpuReqMsch1Pwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == GpuReqMsch1Pwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == GpuReqMsch1Pwrdisctargpwrstall::B1
    }
}
#[doc = "Field `GPU_REQ_MSCH1_PWRDISCTARGPWRSTALL` writer - noc_gpu_req_msch1_rsp_err_stall bit control"]
pub type GpuReqMsch1PwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, GpuReqMsch1Pwrdisctargpwrstall>;
impl<'a, REG> GpuReqMsch1PwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(GpuReqMsch1Pwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(GpuReqMsch1Pwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_hdcp_req_msch01_rsp_err_stall bit\n\ncontrol\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HdcpReqMsch01Pwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<HdcpReqMsch01Pwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: HdcpReqMsch01Pwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCP_REQ_MSCH01_PWRDISCTARGPWRSTALL` reader - noc_hdcp_req_msch01_rsp_err_stall bit\n\ncontrol"]
pub type HdcpReqMsch01PwrdisctargpwrstallR = crate::BitReader<HdcpReqMsch01Pwrdisctargpwrstall>;
impl HdcpReqMsch01PwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HdcpReqMsch01Pwrdisctargpwrstall {
        match self.bits {
            false => HdcpReqMsch01Pwrdisctargpwrstall::B0,
            true => HdcpReqMsch01Pwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HdcpReqMsch01Pwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HdcpReqMsch01Pwrdisctargpwrstall::B1
    }
}
#[doc = "Field `HDCP_REQ_MSCH01_PWRDISCTARGPWRSTALL` writer - noc_hdcp_req_msch01_rsp_err_stall bit\n\ncontrol"]
pub type HdcpReqMsch01PwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, HdcpReqMsch01Pwrdisctargpwrstall>;
impl<'a, REG> HdcpReqMsch01PwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HdcpReqMsch01Pwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HdcpReqMsch01Pwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_iep_req_msch0_rsp_err_stall bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IepReqMsch0Pwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<IepReqMsch0Pwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: IepReqMsch0Pwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IEP_REQ_MSCH0_PWRDISCTARGPWRSTALL` reader - noc_iep_req_msch0_rsp_err_stall bit control"]
pub type IepReqMsch0PwrdisctargpwrstallR = crate::BitReader<IepReqMsch0Pwrdisctargpwrstall>;
impl IepReqMsch0PwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IepReqMsch0Pwrdisctargpwrstall {
        match self.bits {
            false => IepReqMsch0Pwrdisctargpwrstall::B0,
            true => IepReqMsch0Pwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IepReqMsch0Pwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IepReqMsch0Pwrdisctargpwrstall::B1
    }
}
#[doc = "Field `IEP_REQ_MSCH0_PWRDISCTARGPWRSTALL` writer - noc_iep_req_msch0_rsp_err_stall bit control"]
pub type IepReqMsch0PwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, IepReqMsch0Pwrdisctargpwrstall>;
impl<'a, REG> IepReqMsch0PwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IepReqMsch0Pwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IepReqMsch0Pwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_iep_req_msch1_rsp_err_stall bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IepReqMsch1Pwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<IepReqMsch1Pwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: IepReqMsch1Pwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IEP_REQ_MSCH1_PWRDISCTARGPWRSTALL` reader - noc_iep_req_msch1_rsp_err_stall bit control"]
pub type IepReqMsch1PwrdisctargpwrstallR = crate::BitReader<IepReqMsch1Pwrdisctargpwrstall>;
impl IepReqMsch1PwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IepReqMsch1Pwrdisctargpwrstall {
        match self.bits {
            false => IepReqMsch1Pwrdisctargpwrstall::B0,
            true => IepReqMsch1Pwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IepReqMsch1Pwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IepReqMsch1Pwrdisctargpwrstall::B1
    }
}
#[doc = "Field `IEP_REQ_MSCH1_PWRDISCTARGPWRSTALL` writer - noc_iep_req_msch1_rsp_err_stall bit control"]
pub type IepReqMsch1PwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, IepReqMsch1Pwrdisctargpwrstall>;
impl<'a, REG> IepReqMsch1PwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IepReqMsch1Pwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IepReqMsch1Pwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_isp0_req_msch01_rsp_err_stall bit\n\ncontrol\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isp0ReqMsch01Pwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<Isp0ReqMsch01Pwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: Isp0ReqMsch01Pwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISP0_REQ_MSCH01_PWRDISCTARGPWRSTALL` reader - noc_isp0_req_msch01_rsp_err_stall bit\n\ncontrol"]
pub type Isp0ReqMsch01PwrdisctargpwrstallR = crate::BitReader<Isp0ReqMsch01Pwrdisctargpwrstall>;
impl Isp0ReqMsch01PwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isp0ReqMsch01Pwrdisctargpwrstall {
        match self.bits {
            false => Isp0ReqMsch01Pwrdisctargpwrstall::B0,
            true => Isp0ReqMsch01Pwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Isp0ReqMsch01Pwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Isp0ReqMsch01Pwrdisctargpwrstall::B1
    }
}
#[doc = "Field `ISP0_REQ_MSCH01_PWRDISCTARGPWRSTALL` writer - noc_isp0_req_msch01_rsp_err_stall bit\n\ncontrol"]
pub type Isp0ReqMsch01PwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, Isp0ReqMsch01Pwrdisctargpwrstall>;
impl<'a, REG> Isp0ReqMsch01PwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Isp0ReqMsch01Pwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Isp0ReqMsch01Pwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_isp1_req_msch01_rsp_err_stall bit\n\ncontrol\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isp1ReqMsch01Pwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<Isp1ReqMsch01Pwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: Isp1ReqMsch01Pwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISP1_REQ_MSCH01_PWRDISCTARGPWRSTALL` reader - noc_isp1_req_msch01_rsp_err_stall bit\n\ncontrol"]
pub type Isp1ReqMsch01PwrdisctargpwrstallR = crate::BitReader<Isp1ReqMsch01Pwrdisctargpwrstall>;
impl Isp1ReqMsch01PwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isp1ReqMsch01Pwrdisctargpwrstall {
        match self.bits {
            false => Isp1ReqMsch01Pwrdisctargpwrstall::B0,
            true => Isp1ReqMsch01Pwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Isp1ReqMsch01Pwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Isp1ReqMsch01Pwrdisctargpwrstall::B1
    }
}
#[doc = "Field `ISP1_REQ_MSCH01_PWRDISCTARGPWRSTALL` writer - noc_isp1_req_msch01_rsp_err_stall bit\n\ncontrol"]
pub type Isp1ReqMsch01PwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, Isp1ReqMsch01Pwrdisctargpwrstall>;
impl<'a, REG> Isp1ReqMsch01PwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Isp1ReqMsch01Pwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Isp1ReqMsch01Pwrdisctargpwrstall::B1)
    }
}
#[doc = "all\n\nnoc_msch0regsrv_fwd_msch0_rsp_err_stall\n\nbit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msch0regsrvFwdMsch0Pwrdisctargpwrst {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<Msch0regsrvFwdMsch0Pwrdisctargpwrst> for bool {
    #[inline(always)]
    fn from(variant: Msch0regsrvFwdMsch0Pwrdisctargpwrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSCH0REGSRV_FWD_MSCH0_PWRDISCTARGPWRST` reader - all\n\nnoc_msch0regsrv_fwd_msch0_rsp_err_stall\n\nbit control"]
pub type Msch0regsrvFwdMsch0PwrdisctargpwrstR =
    crate::BitReader<Msch0regsrvFwdMsch0Pwrdisctargpwrst>;
impl Msch0regsrvFwdMsch0PwrdisctargpwrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msch0regsrvFwdMsch0Pwrdisctargpwrst {
        match self.bits {
            false => Msch0regsrvFwdMsch0Pwrdisctargpwrst::B0,
            true => Msch0regsrvFwdMsch0Pwrdisctargpwrst::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Msch0regsrvFwdMsch0Pwrdisctargpwrst::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Msch0regsrvFwdMsch0Pwrdisctargpwrst::B1
    }
}
#[doc = "Field `MSCH0REGSRV_FWD_MSCH0_PWRDISCTARGPWRST` writer - all\n\nnoc_msch0regsrv_fwd_msch0_rsp_err_stall\n\nbit control"]
pub type Msch0regsrvFwdMsch0PwrdisctargpwrstW<'a, REG> =
    crate::BitWriter<'a, REG, Msch0regsrvFwdMsch0Pwrdisctargpwrst>;
impl<'a, REG> Msch0regsrvFwdMsch0PwrdisctargpwrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Msch0regsrvFwdMsch0Pwrdisctargpwrst::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Msch0regsrvFwdMsch0Pwrdisctargpwrst::B1)
    }
}
#[doc = "all\n\nnoc_msch1regsrv_fwd_msch1_rsp_err_stall\n\nbit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msch1regsrvFwdMsch1Pwrdisctargpwrst {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<Msch1regsrvFwdMsch1Pwrdisctargpwrst> for bool {
    #[inline(always)]
    fn from(variant: Msch1regsrvFwdMsch1Pwrdisctargpwrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSCH1REGSRV_FWD_MSCH1_PWRDISCTARGPWRST` reader - all\n\nnoc_msch1regsrv_fwd_msch1_rsp_err_stall\n\nbit control"]
pub type Msch1regsrvFwdMsch1PwrdisctargpwrstR =
    crate::BitReader<Msch1regsrvFwdMsch1Pwrdisctargpwrst>;
impl Msch1regsrvFwdMsch1PwrdisctargpwrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msch1regsrvFwdMsch1Pwrdisctargpwrst {
        match self.bits {
            false => Msch1regsrvFwdMsch1Pwrdisctargpwrst::B0,
            true => Msch1regsrvFwdMsch1Pwrdisctargpwrst::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Msch1regsrvFwdMsch1Pwrdisctargpwrst::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Msch1regsrvFwdMsch1Pwrdisctargpwrst::B1
    }
}
#[doc = "Field `MSCH1REGSRV_FWD_MSCH1_PWRDISCTARGPWRST` writer - all\n\nnoc_msch1regsrv_fwd_msch1_rsp_err_stall\n\nbit control"]
pub type Msch1regsrvFwdMsch1PwrdisctargpwrstW<'a, REG> =
    crate::BitWriter<'a, REG, Msch1regsrvFwdMsch1Pwrdisctargpwrst>;
impl<'a, REG> Msch1regsrvFwdMsch1PwrdisctargpwrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Msch1regsrvFwdMsch1Pwrdisctargpwrst::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Msch1regsrvFwdMsch1Pwrdisctargpwrst::B1)
    }
}
#[doc = "noc_pcie_fwd_perihp_rsp_err_stall bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PcieFwdPerihpPwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<PcieFwdPerihpPwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: PcieFwdPerihpPwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCIE_FWD_PERIHP_PWRDISCTARGPWRSTALL` reader - noc_pcie_fwd_perihp_rsp_err_stall bit control"]
pub type PcieFwdPerihpPwrdisctargpwrstallR = crate::BitReader<PcieFwdPerihpPwrdisctargpwrstall>;
impl PcieFwdPerihpPwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PcieFwdPerihpPwrdisctargpwrstall {
        match self.bits {
            false => PcieFwdPerihpPwrdisctargpwrstall::B0,
            true => PcieFwdPerihpPwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PcieFwdPerihpPwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PcieFwdPerihpPwrdisctargpwrstall::B1
    }
}
#[doc = "Field `PCIE_FWD_PERIHP_PWRDISCTARGPWRSTALL` writer - noc_pcie_fwd_perihp_rsp_err_stall bit control"]
pub type PcieFwdPerihpPwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, PcieFwdPerihpPwrdisctargpwrstall>;
impl<'a, REG> PcieFwdPerihpPwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PcieFwdPerihpPwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PcieFwdPerihpPwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_perihp_fwd_center_rsp_err_stall bit\n\ncontrol\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PerihpFwdCenterPwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<PerihpFwdCenterPwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: PerihpFwdCenterPwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERIHP_FWD_CENTER_PWRDISCTARGPWRSTALL` reader - noc_perihp_fwd_center_rsp_err_stall bit\n\ncontrol"]
pub type PerihpFwdCenterPwrdisctargpwrstallR = crate::BitReader<PerihpFwdCenterPwrdisctargpwrstall>;
impl PerihpFwdCenterPwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PerihpFwdCenterPwrdisctargpwrstall {
        match self.bits {
            false => PerihpFwdCenterPwrdisctargpwrstall::B0,
            true => PerihpFwdCenterPwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PerihpFwdCenterPwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PerihpFwdCenterPwrdisctargpwrstall::B1
    }
}
#[doc = "Field `PERIHP_FWD_CENTER_PWRDISCTARGPWRSTALL` writer - noc_perihp_fwd_center_rsp_err_stall bit\n\ncontrol"]
pub type PerihpFwdCenterPwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, PerihpFwdCenterPwrdisctargpwrstall>;
impl<'a, REG> PerihpFwdCenterPwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PerihpFwdCenterPwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PerihpFwdCenterPwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_perihp_req_msch0_rsp_err_stall bit\n\ncontrol\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PerihpReqMsch0Pwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<PerihpReqMsch0Pwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: PerihpReqMsch0Pwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERIHP_REQ_MSCH0_PWRDISCTARGPWRSTALL` reader - noc_perihp_req_msch0_rsp_err_stall bit\n\ncontrol"]
pub type PerihpReqMsch0PwrdisctargpwrstallR = crate::BitReader<PerihpReqMsch0Pwrdisctargpwrstall>;
impl PerihpReqMsch0PwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PerihpReqMsch0Pwrdisctargpwrstall {
        match self.bits {
            false => PerihpReqMsch0Pwrdisctargpwrstall::B0,
            true => PerihpReqMsch0Pwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PerihpReqMsch0Pwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PerihpReqMsch0Pwrdisctargpwrstall::B1
    }
}
#[doc = "Field `PERIHP_REQ_MSCH0_PWRDISCTARGPWRSTALL` writer - noc_perihp_req_msch0_rsp_err_stall bit\n\ncontrol"]
pub type PerihpReqMsch0PwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, PerihpReqMsch0Pwrdisctargpwrstall>;
impl<'a, REG> PerihpReqMsch0PwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PerihpReqMsch0Pwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PerihpReqMsch0Pwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_perihp_req_msch1_rsp_err_stall bit\n\ncontrol\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PerihpReqMsch1Pwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<PerihpReqMsch1Pwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: PerihpReqMsch1Pwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERIHP_REQ_MSCH1_PWRDISCTARGPWRSTALL` reader - noc_perihp_req_msch1_rsp_err_stall bit\n\ncontrol"]
pub type PerihpReqMsch1PwrdisctargpwrstallR = crate::BitReader<PerihpReqMsch1Pwrdisctargpwrstall>;
impl PerihpReqMsch1PwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PerihpReqMsch1Pwrdisctargpwrstall {
        match self.bits {
            false => PerihpReqMsch1Pwrdisctargpwrstall::B0,
            true => PerihpReqMsch1Pwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PerihpReqMsch1Pwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PerihpReqMsch1Pwrdisctargpwrstall::B1
    }
}
#[doc = "Field `PERIHP_REQ_MSCH1_PWRDISCTARGPWRSTALL` writer - noc_perihp_req_msch1_rsp_err_stall bit\n\ncontrol"]
pub type PerihpReqMsch1PwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, PerihpReqMsch1Pwrdisctargpwrstall>;
impl<'a, REG> PerihpReqMsch1PwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PerihpReqMsch1Pwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PerihpReqMsch1Pwrdisctargpwrstall::B1)
    }
}
#[doc = "l\n\nnoc_perihp_cm0_fwd_perihp_rsp_err_stall\n\nbit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PerihpCm0FwdPerihpPwrdisctargpwrstal {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<PerihpCm0FwdPerihpPwrdisctargpwrstal> for bool {
    #[inline(always)]
    fn from(variant: PerihpCm0FwdPerihpPwrdisctargpwrstal) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERIHP_CM0_FWD_PERIHP_PWRDISCTARGPWRSTAL` reader - l\n\nnoc_perihp_cm0_fwd_perihp_rsp_err_stall\n\nbit control"]
pub type PerihpCm0FwdPerihpPwrdisctargpwrstalR =
    crate::BitReader<PerihpCm0FwdPerihpPwrdisctargpwrstal>;
impl PerihpCm0FwdPerihpPwrdisctargpwrstalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PerihpCm0FwdPerihpPwrdisctargpwrstal {
        match self.bits {
            false => PerihpCm0FwdPerihpPwrdisctargpwrstal::B0,
            true => PerihpCm0FwdPerihpPwrdisctargpwrstal::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PerihpCm0FwdPerihpPwrdisctargpwrstal::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PerihpCm0FwdPerihpPwrdisctargpwrstal::B1
    }
}
#[doc = "Field `PERIHP_CM0_FWD_PERIHP_PWRDISCTARGPWRSTAL` writer - l\n\nnoc_perihp_cm0_fwd_perihp_rsp_err_stall\n\nbit control"]
pub type PerihpCm0FwdPerihpPwrdisctargpwrstalW<'a, REG> =
    crate::BitWriter<'a, REG, PerihpCm0FwdPerihpPwrdisctargpwrstal>;
impl<'a, REG> PerihpCm0FwdPerihpPwrdisctargpwrstalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PerihpCm0FwdPerihpPwrdisctargpwrstal::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PerihpCm0FwdPerihpPwrdisctargpwrstal::B1)
    }
}
#[doc = "noc_perihp_fwd_alive_rsp_err_stall bit\n\ncontrol\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PerihpFwdAlivePwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<PerihpFwdAlivePwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: PerihpFwdAlivePwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERIHP_FWD_ALIVE_PWRDISCTARGPWRSTALL` reader - noc_perihp_fwd_alive_rsp_err_stall bit\n\ncontrol"]
pub type PerihpFwdAlivePwrdisctargpwrstallR = crate::BitReader<PerihpFwdAlivePwrdisctargpwrstall>;
impl PerihpFwdAlivePwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PerihpFwdAlivePwrdisctargpwrstall {
        match self.bits {
            false => PerihpFwdAlivePwrdisctargpwrstall::B0,
            true => PerihpFwdAlivePwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PerihpFwdAlivePwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PerihpFwdAlivePwrdisctargpwrstall::B1
    }
}
#[doc = "Field `PERIHP_FWD_ALIVE_PWRDISCTARGPWRSTALL` writer - noc_perihp_fwd_alive_rsp_err_stall bit\n\ncontrol"]
pub type PerihpFwdAlivePwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, PerihpFwdAlivePwrdisctargpwrstall>;
impl<'a, REG> PerihpFwdAlivePwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PerihpFwdAlivePwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PerihpFwdAlivePwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_perihp_fwd_cci_rsp_err_stall bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PerihpFwdCciPwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<PerihpFwdCciPwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: PerihpFwdCciPwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERIHP_FWD_CCI_PWRDISCTARGPWRSTALL` reader - noc_perihp_fwd_cci_rsp_err_stall bit control"]
pub type PerihpFwdCciPwrdisctargpwrstallR = crate::BitReader<PerihpFwdCciPwrdisctargpwrstall>;
impl PerihpFwdCciPwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PerihpFwdCciPwrdisctargpwrstall {
        match self.bits {
            false => PerihpFwdCciPwrdisctargpwrstall::B0,
            true => PerihpFwdCciPwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PerihpFwdCciPwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PerihpFwdCciPwrdisctargpwrstall::B1
    }
}
#[doc = "Field `PERIHP_FWD_CCI_PWRDISCTARGPWRSTALL` writer - noc_perihp_fwd_cci_rsp_err_stall bit control"]
pub type PerihpFwdCciPwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, PerihpFwdCciPwrdisctargpwrstall>;
impl<'a, REG> PerihpFwdCciPwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PerihpFwdCciPwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PerihpFwdCciPwrdisctargpwrstall::B1)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - noc_gpu_req_msch0_rsp_err_stall bit control"]
    #[inline(always)]
    pub fn gpu_req_msch0_pwrdisctargpwrstall(&self) -> GpuReqMsch0PwrdisctargpwrstallR {
        GpuReqMsch0PwrdisctargpwrstallR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - noc_gpu_req_msch1_rsp_err_stall bit control"]
    #[inline(always)]
    pub fn gpu_req_msch1_pwrdisctargpwrstall(&self) -> GpuReqMsch1PwrdisctargpwrstallR {
        GpuReqMsch1PwrdisctargpwrstallR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - noc_hdcp_req_msch01_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    pub fn hdcp_req_msch01_pwrdisctargpwrstall(&self) -> HdcpReqMsch01PwrdisctargpwrstallR {
        HdcpReqMsch01PwrdisctargpwrstallR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - noc_iep_req_msch0_rsp_err_stall bit control"]
    #[inline(always)]
    pub fn iep_req_msch0_pwrdisctargpwrstall(&self) -> IepReqMsch0PwrdisctargpwrstallR {
        IepReqMsch0PwrdisctargpwrstallR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - noc_iep_req_msch1_rsp_err_stall bit control"]
    #[inline(always)]
    pub fn iep_req_msch1_pwrdisctargpwrstall(&self) -> IepReqMsch1PwrdisctargpwrstallR {
        IepReqMsch1PwrdisctargpwrstallR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - noc_isp0_req_msch01_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    pub fn isp0_req_msch01_pwrdisctargpwrstall(&self) -> Isp0ReqMsch01PwrdisctargpwrstallR {
        Isp0ReqMsch01PwrdisctargpwrstallR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - noc_isp1_req_msch01_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    pub fn isp1_req_msch01_pwrdisctargpwrstall(&self) -> Isp1ReqMsch01PwrdisctargpwrstallR {
        Isp1ReqMsch01PwrdisctargpwrstallR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - all\n\nnoc_msch0regsrv_fwd_msch0_rsp_err_stall\n\nbit control"]
    #[inline(always)]
    pub fn msch0regsrv_fwd_msch0_pwrdisctargpwrst(&self) -> Msch0regsrvFwdMsch0PwrdisctargpwrstR {
        Msch0regsrvFwdMsch0PwrdisctargpwrstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - all\n\nnoc_msch1regsrv_fwd_msch1_rsp_err_stall\n\nbit control"]
    #[inline(always)]
    pub fn msch1regsrv_fwd_msch1_pwrdisctargpwrst(&self) -> Msch1regsrvFwdMsch1PwrdisctargpwrstR {
        Msch1regsrvFwdMsch1PwrdisctargpwrstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - noc_pcie_fwd_perihp_rsp_err_stall bit control"]
    #[inline(always)]
    pub fn pcie_fwd_perihp_pwrdisctargpwrstall(&self) -> PcieFwdPerihpPwrdisctargpwrstallR {
        PcieFwdPerihpPwrdisctargpwrstallR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - noc_perihp_fwd_center_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    pub fn perihp_fwd_center_pwrdisctargpwrstall(&self) -> PerihpFwdCenterPwrdisctargpwrstallR {
        PerihpFwdCenterPwrdisctargpwrstallR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - noc_perihp_req_msch0_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    pub fn perihp_req_msch0_pwrdisctargpwrstall(&self) -> PerihpReqMsch0PwrdisctargpwrstallR {
        PerihpReqMsch0PwrdisctargpwrstallR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - noc_perihp_req_msch1_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    pub fn perihp_req_msch1_pwrdisctargpwrstall(&self) -> PerihpReqMsch1PwrdisctargpwrstallR {
        PerihpReqMsch1PwrdisctargpwrstallR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - l\n\nnoc_perihp_cm0_fwd_perihp_rsp_err_stall\n\nbit control"]
    #[inline(always)]
    pub fn perihp_cm0_fwd_perihp_pwrdisctargpwrstal(
        &self,
    ) -> PerihpCm0FwdPerihpPwrdisctargpwrstalR {
        PerihpCm0FwdPerihpPwrdisctargpwrstalR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - noc_perihp_fwd_alive_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    pub fn perihp_fwd_alive_pwrdisctargpwrstall(&self) -> PerihpFwdAlivePwrdisctargpwrstallR {
        PerihpFwdAlivePwrdisctargpwrstallR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - noc_perihp_fwd_cci_rsp_err_stall bit control"]
    #[inline(always)]
    pub fn perihp_fwd_cci_pwrdisctargpwrstall(&self) -> PerihpFwdCciPwrdisctargpwrstallR {
        PerihpFwdCciPwrdisctargpwrstallR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - noc_gpu_req_msch0_rsp_err_stall bit control"]
    #[inline(always)]
    #[must_use]
    pub fn gpu_req_msch0_pwrdisctargpwrstall(
        &mut self,
    ) -> GpuReqMsch0PwrdisctargpwrstallW<SocCon1Spec> {
        GpuReqMsch0PwrdisctargpwrstallW::new(self, 0)
    }
    #[doc = "Bit 1 - noc_gpu_req_msch1_rsp_err_stall bit control"]
    #[inline(always)]
    #[must_use]
    pub fn gpu_req_msch1_pwrdisctargpwrstall(
        &mut self,
    ) -> GpuReqMsch1PwrdisctargpwrstallW<SocCon1Spec> {
        GpuReqMsch1PwrdisctargpwrstallW::new(self, 1)
    }
    #[doc = "Bit 2 - noc_hdcp_req_msch01_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    #[must_use]
    pub fn hdcp_req_msch01_pwrdisctargpwrstall(
        &mut self,
    ) -> HdcpReqMsch01PwrdisctargpwrstallW<SocCon1Spec> {
        HdcpReqMsch01PwrdisctargpwrstallW::new(self, 2)
    }
    #[doc = "Bit 3 - noc_iep_req_msch0_rsp_err_stall bit control"]
    #[inline(always)]
    #[must_use]
    pub fn iep_req_msch0_pwrdisctargpwrstall(
        &mut self,
    ) -> IepReqMsch0PwrdisctargpwrstallW<SocCon1Spec> {
        IepReqMsch0PwrdisctargpwrstallW::new(self, 3)
    }
    #[doc = "Bit 4 - noc_iep_req_msch1_rsp_err_stall bit control"]
    #[inline(always)]
    #[must_use]
    pub fn iep_req_msch1_pwrdisctargpwrstall(
        &mut self,
    ) -> IepReqMsch1PwrdisctargpwrstallW<SocCon1Spec> {
        IepReqMsch1PwrdisctargpwrstallW::new(self, 4)
    }
    #[doc = "Bit 5 - noc_isp0_req_msch01_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    #[must_use]
    pub fn isp0_req_msch01_pwrdisctargpwrstall(
        &mut self,
    ) -> Isp0ReqMsch01PwrdisctargpwrstallW<SocCon1Spec> {
        Isp0ReqMsch01PwrdisctargpwrstallW::new(self, 5)
    }
    #[doc = "Bit 6 - noc_isp1_req_msch01_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    #[must_use]
    pub fn isp1_req_msch01_pwrdisctargpwrstall(
        &mut self,
    ) -> Isp1ReqMsch01PwrdisctargpwrstallW<SocCon1Spec> {
        Isp1ReqMsch01PwrdisctargpwrstallW::new(self, 6)
    }
    #[doc = "Bit 7 - all\n\nnoc_msch0regsrv_fwd_msch0_rsp_err_stall\n\nbit control"]
    #[inline(always)]
    #[must_use]
    pub fn msch0regsrv_fwd_msch0_pwrdisctargpwrst(
        &mut self,
    ) -> Msch0regsrvFwdMsch0PwrdisctargpwrstW<SocCon1Spec> {
        Msch0regsrvFwdMsch0PwrdisctargpwrstW::new(self, 7)
    }
    #[doc = "Bit 8 - all\n\nnoc_msch1regsrv_fwd_msch1_rsp_err_stall\n\nbit control"]
    #[inline(always)]
    #[must_use]
    pub fn msch1regsrv_fwd_msch1_pwrdisctargpwrst(
        &mut self,
    ) -> Msch1regsrvFwdMsch1PwrdisctargpwrstW<SocCon1Spec> {
        Msch1regsrvFwdMsch1PwrdisctargpwrstW::new(self, 8)
    }
    #[doc = "Bit 9 - noc_pcie_fwd_perihp_rsp_err_stall bit control"]
    #[inline(always)]
    #[must_use]
    pub fn pcie_fwd_perihp_pwrdisctargpwrstall(
        &mut self,
    ) -> PcieFwdPerihpPwrdisctargpwrstallW<SocCon1Spec> {
        PcieFwdPerihpPwrdisctargpwrstallW::new(self, 9)
    }
    #[doc = "Bit 10 - noc_perihp_fwd_center_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    #[must_use]
    pub fn perihp_fwd_center_pwrdisctargpwrstall(
        &mut self,
    ) -> PerihpFwdCenterPwrdisctargpwrstallW<SocCon1Spec> {
        PerihpFwdCenterPwrdisctargpwrstallW::new(self, 10)
    }
    #[doc = "Bit 11 - noc_perihp_req_msch0_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    #[must_use]
    pub fn perihp_req_msch0_pwrdisctargpwrstall(
        &mut self,
    ) -> PerihpReqMsch0PwrdisctargpwrstallW<SocCon1Spec> {
        PerihpReqMsch0PwrdisctargpwrstallW::new(self, 11)
    }
    #[doc = "Bit 12 - noc_perihp_req_msch1_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    #[must_use]
    pub fn perihp_req_msch1_pwrdisctargpwrstall(
        &mut self,
    ) -> PerihpReqMsch1PwrdisctargpwrstallW<SocCon1Spec> {
        PerihpReqMsch1PwrdisctargpwrstallW::new(self, 12)
    }
    #[doc = "Bit 13 - l\n\nnoc_perihp_cm0_fwd_perihp_rsp_err_stall\n\nbit control"]
    #[inline(always)]
    #[must_use]
    pub fn perihp_cm0_fwd_perihp_pwrdisctargpwrstal(
        &mut self,
    ) -> PerihpCm0FwdPerihpPwrdisctargpwrstalW<SocCon1Spec> {
        PerihpCm0FwdPerihpPwrdisctargpwrstalW::new(self, 13)
    }
    #[doc = "Bit 14 - noc_perihp_fwd_alive_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    #[must_use]
    pub fn perihp_fwd_alive_pwrdisctargpwrstall(
        &mut self,
    ) -> PerihpFwdAlivePwrdisctargpwrstallW<SocCon1Spec> {
        PerihpFwdAlivePwrdisctargpwrstallW::new(self, 14)
    }
    #[doc = "Bit 15 - noc_perihp_fwd_cci_rsp_err_stall bit control"]
    #[inline(always)]
    #[must_use]
    pub fn perihp_fwd_cci_pwrdisctargpwrstall(
        &mut self,
    ) -> PerihpFwdCciPwrdisctargpwrstallW<SocCon1Spec> {
        PerihpFwdCciPwrdisctargpwrstallW::new(self, 15)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<SocCon1Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "SoC control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_con1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_con1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SocCon1Spec;
impl crate::RegisterSpec for SocCon1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soc_con1::R`](R) reader structure"]
impl crate::Readable for SocCon1Spec {}
#[doc = "`write(|w| ..)` method takes [`soc_con1::W`](W) writer structure"]
impl crate::Writable for SocCon1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOC_CON1 to value 0"]
impl crate::Resettable for SocCon1Spec {
    const RESET_VALUE: u32 = 0;
}
