#[doc = "Register `SOC_CON3` reader"]
pub type R = crate::R<SocCon3Spec>;
#[doc = "Register `SOC_CON3` writer"]
pub type W = crate::W<SocCon3Spec>;
#[doc = "noc_vio0_req_msch1_rsp_err_stall bit\n\ncontrol\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vio0ReqMsch1Pwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<Vio0ReqMsch1Pwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: Vio0ReqMsch1Pwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VIO0_REQ_MSCH1_PWRDISCTARGPWRSTALL` reader - noc_vio0_req_msch1_rsp_err_stall bit\n\ncontrol"]
pub type Vio0ReqMsch1PwrdisctargpwrstallR = crate::BitReader<Vio0ReqMsch1Pwrdisctargpwrstall>;
impl Vio0ReqMsch1PwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vio0ReqMsch1Pwrdisctargpwrstall {
        match self.bits {
            false => Vio0ReqMsch1Pwrdisctargpwrstall::B0,
            true => Vio0ReqMsch1Pwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Vio0ReqMsch1Pwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Vio0ReqMsch1Pwrdisctargpwrstall::B1
    }
}
#[doc = "Field `VIO0_REQ_MSCH1_PWRDISCTARGPWRSTALL` writer - noc_vio0_req_msch1_rsp_err_stall bit\n\ncontrol"]
pub type Vio0ReqMsch1PwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, Vio0ReqMsch1Pwrdisctargpwrstall>;
impl<'a, REG> Vio0ReqMsch1PwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Vio0ReqMsch1Pwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Vio0ReqMsch1Pwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_vio1_req_msch0_rsp_err_stall bit\n\ncontrol\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vio1ReqMsch0Pwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<Vio1ReqMsch0Pwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: Vio1ReqMsch0Pwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VIO1_REQ_MSCH0_PWRDISCTARGPWRSTALL` reader - noc_vio1_req_msch0_rsp_err_stall bit\n\ncontrol"]
pub type Vio1ReqMsch0PwrdisctargpwrstallR = crate::BitReader<Vio1ReqMsch0Pwrdisctargpwrstall>;
impl Vio1ReqMsch0PwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vio1ReqMsch0Pwrdisctargpwrstall {
        match self.bits {
            false => Vio1ReqMsch0Pwrdisctargpwrstall::B0,
            true => Vio1ReqMsch0Pwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Vio1ReqMsch0Pwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Vio1ReqMsch0Pwrdisctargpwrstall::B1
    }
}
#[doc = "Field `VIO1_REQ_MSCH0_PWRDISCTARGPWRSTALL` writer - noc_vio1_req_msch0_rsp_err_stall bit\n\ncontrol"]
pub type Vio1ReqMsch0PwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, Vio1ReqMsch0Pwrdisctargpwrstall>;
impl<'a, REG> Vio1ReqMsch0PwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Vio1ReqMsch0Pwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Vio1ReqMsch0Pwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_vio1_req_msch1_rsp_err_stall bit\n\ncontrol\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vio1ReqMsch1Pwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<Vio1ReqMsch1Pwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: Vio1ReqMsch1Pwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VIO1_REQ_MSCH1_PWRDISCTARGPWRSTALL` reader - noc_vio1_req_msch1_rsp_err_stall bit\n\ncontrol"]
pub type Vio1ReqMsch1PwrdisctargpwrstallR = crate::BitReader<Vio1ReqMsch1Pwrdisctargpwrstall>;
impl Vio1ReqMsch1PwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vio1ReqMsch1Pwrdisctargpwrstall {
        match self.bits {
            false => Vio1ReqMsch1Pwrdisctargpwrstall::B0,
            true => Vio1ReqMsch1Pwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Vio1ReqMsch1Pwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Vio1ReqMsch1Pwrdisctargpwrstall::B1
    }
}
#[doc = "Field `VIO1_REQ_MSCH1_PWRDISCTARGPWRSTALL` writer - noc_vio1_req_msch1_rsp_err_stall bit\n\ncontrol"]
pub type Vio1ReqMsch1PwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, Vio1ReqMsch1Pwrdisctargpwrstall>;
impl<'a, REG> Vio1ReqMsch1PwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Vio1ReqMsch1Pwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Vio1ReqMsch1Pwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_viob_req_msch01_rsp_err_stall bit\n\ncontrol\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViobReqMsch01Pwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<ViobReqMsch01Pwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: ViobReqMsch01Pwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VIOB_REQ_MSCH01_PWRDISCTARGPWRSTALL` reader - noc_viob_req_msch01_rsp_err_stall bit\n\ncontrol"]
pub type ViobReqMsch01PwrdisctargpwrstallR = crate::BitReader<ViobReqMsch01Pwrdisctargpwrstall>;
impl ViobReqMsch01PwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ViobReqMsch01Pwrdisctargpwrstall {
        match self.bits {
            false => ViobReqMsch01Pwrdisctargpwrstall::B0,
            true => ViobReqMsch01Pwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ViobReqMsch01Pwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ViobReqMsch01Pwrdisctargpwrstall::B1
    }
}
#[doc = "Field `VIOB_REQ_MSCH01_PWRDISCTARGPWRSTALL` writer - noc_viob_req_msch01_rsp_err_stall bit\n\ncontrol"]
pub type ViobReqMsch01PwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, ViobReqMsch01Pwrdisctargpwrstall>;
impl<'a, REG> ViobReqMsch01PwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ViobReqMsch01Pwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ViobReqMsch01Pwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_viol_req_msch01_rsp_err_stall bit\n\ncontrol\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViolReqMsch01Pwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<ViolReqMsch01Pwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: ViolReqMsch01Pwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VIOL_REQ_MSCH01_PWRDISCTARGPWRSTALL` reader - noc_viol_req_msch01_rsp_err_stall bit\n\ncontrol"]
pub type ViolReqMsch01PwrdisctargpwrstallR = crate::BitReader<ViolReqMsch01Pwrdisctargpwrstall>;
impl ViolReqMsch01PwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ViolReqMsch01Pwrdisctargpwrstall {
        match self.bits {
            false => ViolReqMsch01Pwrdisctargpwrstall::B0,
            true => ViolReqMsch01Pwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ViolReqMsch01Pwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ViolReqMsch01Pwrdisctargpwrstall::B1
    }
}
#[doc = "Field `VIOL_REQ_MSCH01_PWRDISCTARGPWRSTALL` writer - noc_viol_req_msch01_rsp_err_stall bit\n\ncontrol"]
pub type ViolReqMsch01PwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, ViolReqMsch01Pwrdisctargpwrstall>;
impl<'a, REG> ViolReqMsch01PwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ViolReqMsch01Pwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ViolReqMsch01Pwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_usb3_fwd_perilp_rsp_err_stall bit\n\ncontrol\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usb3FwdPerilpPwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<Usb3FwdPerilpPwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: Usb3FwdPerilpPwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB3_FWD_PERILP_PWRDISCTARGPWRSTALL` reader - noc_usb3_fwd_perilp_rsp_err_stall bit\n\ncontrol"]
pub type Usb3FwdPerilpPwrdisctargpwrstallR = crate::BitReader<Usb3FwdPerilpPwrdisctargpwrstall>;
impl Usb3FwdPerilpPwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usb3FwdPerilpPwrdisctargpwrstall {
        match self.bits {
            false => Usb3FwdPerilpPwrdisctargpwrstall::B0,
            true => Usb3FwdPerilpPwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Usb3FwdPerilpPwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Usb3FwdPerilpPwrdisctargpwrstall::B1
    }
}
#[doc = "Field `USB3_FWD_PERILP_PWRDISCTARGPWRSTALL` writer - noc_usb3_fwd_perilp_rsp_err_stall bit\n\ncontrol"]
pub type Usb3FwdPerilpPwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, Usb3FwdPerilpPwrdisctargpwrstall>;
impl<'a, REG> Usb3FwdPerilpPwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Usb3FwdPerilpPwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Usb3FwdPerilpPwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_vio_fwd_isp0_rsp_err_stall bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VioFwdIsp0Pwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<VioFwdIsp0Pwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: VioFwdIsp0Pwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VIO_FWD_ISP0_PWRDISCTARGPWRSTALL` reader - noc_vio_fwd_isp0_rsp_err_stall bit control"]
pub type VioFwdIsp0PwrdisctargpwrstallR = crate::BitReader<VioFwdIsp0Pwrdisctargpwrstall>;
impl VioFwdIsp0PwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VioFwdIsp0Pwrdisctargpwrstall {
        match self.bits {
            false => VioFwdIsp0Pwrdisctargpwrstall::B0,
            true => VioFwdIsp0Pwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VioFwdIsp0Pwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VioFwdIsp0Pwrdisctargpwrstall::B1
    }
}
#[doc = "Field `VIO_FWD_ISP0_PWRDISCTARGPWRSTALL` writer - noc_vio_fwd_isp0_rsp_err_stall bit control"]
pub type VioFwdIsp0PwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, VioFwdIsp0Pwrdisctargpwrstall>;
impl<'a, REG> VioFwdIsp0PwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VioFwdIsp0Pwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VioFwdIsp0Pwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_vio_fwd_isp1_rsp_err_stall bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VioFwdIsp1Pwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<VioFwdIsp1Pwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: VioFwdIsp1Pwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VIO_FWD_ISP1_PWRDISCTARGPWRSTALL` reader - noc_vio_fwd_isp1_rsp_err_stall bit control"]
pub type VioFwdIsp1PwrdisctargpwrstallR = crate::BitReader<VioFwdIsp1Pwrdisctargpwrstall>;
impl VioFwdIsp1PwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VioFwdIsp1Pwrdisctargpwrstall {
        match self.bits {
            false => VioFwdIsp1Pwrdisctargpwrstall::B0,
            true => VioFwdIsp1Pwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VioFwdIsp1Pwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VioFwdIsp1Pwrdisctargpwrstall::B1
    }
}
#[doc = "Field `VIO_FWD_ISP1_PWRDISCTARGPWRSTALL` writer - noc_vio_fwd_isp1_rsp_err_stall bit control"]
pub type VioFwdIsp1PwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, VioFwdIsp1Pwrdisctargpwrstall>;
impl<'a, REG> VioFwdIsp1PwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VioFwdIsp1Pwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VioFwdIsp1Pwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_vio_fwd_vopb_rsp_err_stall bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VioFwdVopbPwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<VioFwdVopbPwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: VioFwdVopbPwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VIO_FWD_VOPB_PWRDISCTARGPWRSTALL` reader - noc_vio_fwd_vopb_rsp_err_stall bit control"]
pub type VioFwdVopbPwrdisctargpwrstallR = crate::BitReader<VioFwdVopbPwrdisctargpwrstall>;
impl VioFwdVopbPwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VioFwdVopbPwrdisctargpwrstall {
        match self.bits {
            false => VioFwdVopbPwrdisctargpwrstall::B0,
            true => VioFwdVopbPwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VioFwdVopbPwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VioFwdVopbPwrdisctargpwrstall::B1
    }
}
#[doc = "Field `VIO_FWD_VOPB_PWRDISCTARGPWRSTALL` writer - noc_vio_fwd_vopb_rsp_err_stall bit control"]
pub type VioFwdVopbPwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, VioFwdVopbPwrdisctargpwrstall>;
impl<'a, REG> VioFwdVopbPwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VioFwdVopbPwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VioFwdVopbPwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_vio_fwd_vopl_rsp_err_stall bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VioFwdVoplPwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<VioFwdVoplPwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: VioFwdVoplPwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VIO_FWD_VOPL_PWRDISCTARGPWRSTALL` reader - noc_vio_fwd_vopl_rsp_err_stall bit control"]
pub type VioFwdVoplPwrdisctargpwrstallR = crate::BitReader<VioFwdVoplPwrdisctargpwrstall>;
impl VioFwdVoplPwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VioFwdVoplPwrdisctargpwrstall {
        match self.bits {
            false => VioFwdVoplPwrdisctargpwrstall::B0,
            true => VioFwdVoplPwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VioFwdVoplPwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VioFwdVoplPwrdisctargpwrstall::B1
    }
}
#[doc = "Field `VIO_FWD_VOPL_PWRDISCTARGPWRSTALL` writer - noc_vio_fwd_vopl_rsp_err_stall bit control"]
pub type VioFwdVoplPwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, VioFwdVoplPwrdisctargpwrstall>;
impl<'a, REG> VioFwdVoplPwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VioFwdVoplPwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VioFwdVoplPwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_vio_fwd_hdcp_rsp_err_stall bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VioFwdHdcpPwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<VioFwdHdcpPwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: VioFwdHdcpPwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VIO_FWD_HDCP_PWRDISCTARGPWRSTALL` reader - noc_vio_fwd_hdcp_rsp_err_stall bit control"]
pub type VioFwdHdcpPwrdisctargpwrstallR = crate::BitReader<VioFwdHdcpPwrdisctargpwrstall>;
impl VioFwdHdcpPwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VioFwdHdcpPwrdisctargpwrstall {
        match self.bits {
            false => VioFwdHdcpPwrdisctargpwrstall::B0,
            true => VioFwdHdcpPwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VioFwdHdcpPwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VioFwdHdcpPwrdisctargpwrstall::B1
    }
}
#[doc = "Field `VIO_FWD_HDCP_PWRDISCTARGPWRSTALL` writer - noc_vio_fwd_hdcp_rsp_err_stall bit control"]
pub type VioFwdHdcpPwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, VioFwdHdcpPwrdisctargpwrstall>;
impl<'a, REG> VioFwdHdcpPwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VioFwdHdcpPwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VioFwdHdcpPwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_vopb_req_msch11_rsp_err_stall bit\n\ncontrol\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VopbReqMsch11Pwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<VopbReqMsch11Pwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: VopbReqMsch11Pwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VOPB_REQ_MSCH11_PWRDISCTARGPWRSTALL` reader - noc_vopb_req_msch11_rsp_err_stall bit\n\ncontrol"]
pub type VopbReqMsch11PwrdisctargpwrstallR = crate::BitReader<VopbReqMsch11Pwrdisctargpwrstall>;
impl VopbReqMsch11PwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VopbReqMsch11Pwrdisctargpwrstall {
        match self.bits {
            false => VopbReqMsch11Pwrdisctargpwrstall::B0,
            true => VopbReqMsch11Pwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VopbReqMsch11Pwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VopbReqMsch11Pwrdisctargpwrstall::B1
    }
}
#[doc = "Field `VOPB_REQ_MSCH11_PWRDISCTARGPWRSTALL` writer - noc_vopb_req_msch11_rsp_err_stall bit\n\ncontrol"]
pub type VopbReqMsch11PwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, VopbReqMsch11Pwrdisctargpwrstall>;
impl<'a, REG> VopbReqMsch11PwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VopbReqMsch11Pwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VopbReqMsch11Pwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_vopl_req_msch11_rsp_err_stall bit\n\ncontrol\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VoplReqMsch11Pwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<VoplReqMsch11Pwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: VoplReqMsch11Pwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VOPL_REQ_MSCH11_PWRDISCTARGPWRSTALL` reader - noc_vopl_req_msch11_rsp_err_stall bit\n\ncontrol"]
pub type VoplReqMsch11PwrdisctargpwrstallR = crate::BitReader<VoplReqMsch11Pwrdisctargpwrstall>;
impl VoplReqMsch11PwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VoplReqMsch11Pwrdisctargpwrstall {
        match self.bits {
            false => VoplReqMsch11Pwrdisctargpwrstall::B0,
            true => VoplReqMsch11Pwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VoplReqMsch11Pwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VoplReqMsch11Pwrdisctargpwrstall::B1
    }
}
#[doc = "Field `VOPL_REQ_MSCH11_PWRDISCTARGPWRSTALL` writer - noc_vopl_req_msch11_rsp_err_stall bit\n\ncontrol"]
pub type VoplReqMsch11PwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, VoplReqMsch11Pwrdisctargpwrstall>;
impl<'a, REG> VoplReqMsch11PwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VoplReqMsch11Pwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VoplReqMsch11Pwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_sd_fwd_perihp_rsp_err_stall bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SdFwdPerihpPwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<SdFwdPerihpPwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: SdFwdPerihpPwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SD_FWD_PERIHP_PWRDISCTARGPWRSTALL` reader - noc_sd_fwd_perihp_rsp_err_stall bit control"]
pub type SdFwdPerihpPwrdisctargpwrstallR = crate::BitReader<SdFwdPerihpPwrdisctargpwrstall>;
impl SdFwdPerihpPwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SdFwdPerihpPwrdisctargpwrstall {
        match self.bits {
            false => SdFwdPerihpPwrdisctargpwrstall::B0,
            true => SdFwdPerihpPwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SdFwdPerihpPwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SdFwdPerihpPwrdisctargpwrstall::B1
    }
}
#[doc = "Field `SD_FWD_PERIHP_PWRDISCTARGPWRSTALL` writer - noc_sd_fwd_perihp_rsp_err_stall bit control"]
pub type SdFwdPerihpPwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, SdFwdPerihpPwrdisctargpwrstall>;
impl<'a, REG> SdFwdPerihpPwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SdFwdPerihpPwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SdFwdPerihpPwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_gic_fwd_perilp_rsp_err_stall bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GicFwdPerilpPwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<GicFwdPerilpPwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: GicFwdPerilpPwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GIC_FWD_PERILP_PWRDISCTARGPWRSTALL` reader - noc_gic_fwd_perilp_rsp_err_stall bit control"]
pub type GicFwdPerilpPwrdisctargpwrstallR = crate::BitReader<GicFwdPerilpPwrdisctargpwrstall>;
impl GicFwdPerilpPwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GicFwdPerilpPwrdisctargpwrstall {
        match self.bits {
            false => GicFwdPerilpPwrdisctargpwrstall::B0,
            true => GicFwdPerilpPwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == GicFwdPerilpPwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == GicFwdPerilpPwrdisctargpwrstall::B1
    }
}
#[doc = "Field `GIC_FWD_PERILP_PWRDISCTARGPWRSTALL` writer - noc_gic_fwd_perilp_rsp_err_stall bit control"]
pub type GicFwdPerilpPwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, GicFwdPerilpPwrdisctargpwrstall>;
impl<'a, REG> GicFwdPerilpPwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(GicFwdPerilpPwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(GicFwdPerilpPwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_perihp_fwd_sd_rsp_err_stall bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PerihpFwdSdPwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<PerihpFwdSdPwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: PerihpFwdSdPwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERIHP_FWD_SD_PWRDISCTARGPWRSTALL` reader - noc_perihp_fwd_sd_rsp_err_stall bit control"]
pub type PerihpFwdSdPwrdisctargpwrstallR = crate::BitReader<PerihpFwdSdPwrdisctargpwrstall>;
impl PerihpFwdSdPwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PerihpFwdSdPwrdisctargpwrstall {
        match self.bits {
            false => PerihpFwdSdPwrdisctargpwrstall::B0,
            true => PerihpFwdSdPwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PerihpFwdSdPwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PerihpFwdSdPwrdisctargpwrstall::B1
    }
}
#[doc = "Field `PERIHP_FWD_SD_PWRDISCTARGPWRSTALL` writer - noc_perihp_fwd_sd_rsp_err_stall bit control"]
pub type PerihpFwdSdPwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, PerihpFwdSdPwrdisctargpwrstall>;
impl<'a, REG> PerihpFwdSdPwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PerihpFwdSdPwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PerihpFwdSdPwrdisctargpwrstall::B1)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - noc_vio0_req_msch1_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    pub fn vio0_req_msch1_pwrdisctargpwrstall(&self) -> Vio0ReqMsch1PwrdisctargpwrstallR {
        Vio0ReqMsch1PwrdisctargpwrstallR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - noc_vio1_req_msch0_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    pub fn vio1_req_msch0_pwrdisctargpwrstall(&self) -> Vio1ReqMsch0PwrdisctargpwrstallR {
        Vio1ReqMsch0PwrdisctargpwrstallR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - noc_vio1_req_msch1_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    pub fn vio1_req_msch1_pwrdisctargpwrstall(&self) -> Vio1ReqMsch1PwrdisctargpwrstallR {
        Vio1ReqMsch1PwrdisctargpwrstallR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - noc_viob_req_msch01_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    pub fn viob_req_msch01_pwrdisctargpwrstall(&self) -> ViobReqMsch01PwrdisctargpwrstallR {
        ViobReqMsch01PwrdisctargpwrstallR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - noc_viol_req_msch01_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    pub fn viol_req_msch01_pwrdisctargpwrstall(&self) -> ViolReqMsch01PwrdisctargpwrstallR {
        ViolReqMsch01PwrdisctargpwrstallR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - noc_usb3_fwd_perilp_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    pub fn usb3_fwd_perilp_pwrdisctargpwrstall(&self) -> Usb3FwdPerilpPwrdisctargpwrstallR {
        Usb3FwdPerilpPwrdisctargpwrstallR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - noc_vio_fwd_isp0_rsp_err_stall bit control"]
    #[inline(always)]
    pub fn vio_fwd_isp0_pwrdisctargpwrstall(&self) -> VioFwdIsp0PwrdisctargpwrstallR {
        VioFwdIsp0PwrdisctargpwrstallR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - noc_vio_fwd_isp1_rsp_err_stall bit control"]
    #[inline(always)]
    pub fn vio_fwd_isp1_pwrdisctargpwrstall(&self) -> VioFwdIsp1PwrdisctargpwrstallR {
        VioFwdIsp1PwrdisctargpwrstallR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - noc_vio_fwd_vopb_rsp_err_stall bit control"]
    #[inline(always)]
    pub fn vio_fwd_vopb_pwrdisctargpwrstall(&self) -> VioFwdVopbPwrdisctargpwrstallR {
        VioFwdVopbPwrdisctargpwrstallR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - noc_vio_fwd_vopl_rsp_err_stall bit control"]
    #[inline(always)]
    pub fn vio_fwd_vopl_pwrdisctargpwrstall(&self) -> VioFwdVoplPwrdisctargpwrstallR {
        VioFwdVoplPwrdisctargpwrstallR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - noc_vio_fwd_hdcp_rsp_err_stall bit control"]
    #[inline(always)]
    pub fn vio_fwd_hdcp_pwrdisctargpwrstall(&self) -> VioFwdHdcpPwrdisctargpwrstallR {
        VioFwdHdcpPwrdisctargpwrstallR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - noc_vopb_req_msch11_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    pub fn vopb_req_msch11_pwrdisctargpwrstall(&self) -> VopbReqMsch11PwrdisctargpwrstallR {
        VopbReqMsch11PwrdisctargpwrstallR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - noc_vopl_req_msch11_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    pub fn vopl_req_msch11_pwrdisctargpwrstall(&self) -> VoplReqMsch11PwrdisctargpwrstallR {
        VoplReqMsch11PwrdisctargpwrstallR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - noc_sd_fwd_perihp_rsp_err_stall bit control"]
    #[inline(always)]
    pub fn sd_fwd_perihp_pwrdisctargpwrstall(&self) -> SdFwdPerihpPwrdisctargpwrstallR {
        SdFwdPerihpPwrdisctargpwrstallR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - noc_gic_fwd_perilp_rsp_err_stall bit control"]
    #[inline(always)]
    pub fn gic_fwd_perilp_pwrdisctargpwrstall(&self) -> GicFwdPerilpPwrdisctargpwrstallR {
        GicFwdPerilpPwrdisctargpwrstallR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - noc_perihp_fwd_sd_rsp_err_stall bit control"]
    #[inline(always)]
    pub fn perihp_fwd_sd_pwrdisctargpwrstall(&self) -> PerihpFwdSdPwrdisctargpwrstallR {
        PerihpFwdSdPwrdisctargpwrstallR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - noc_vio0_req_msch1_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    #[must_use]
    pub fn vio0_req_msch1_pwrdisctargpwrstall(
        &mut self,
    ) -> Vio0ReqMsch1PwrdisctargpwrstallW<SocCon3Spec> {
        Vio0ReqMsch1PwrdisctargpwrstallW::new(self, 0)
    }
    #[doc = "Bit 1 - noc_vio1_req_msch0_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    #[must_use]
    pub fn vio1_req_msch0_pwrdisctargpwrstall(
        &mut self,
    ) -> Vio1ReqMsch0PwrdisctargpwrstallW<SocCon3Spec> {
        Vio1ReqMsch0PwrdisctargpwrstallW::new(self, 1)
    }
    #[doc = "Bit 2 - noc_vio1_req_msch1_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    #[must_use]
    pub fn vio1_req_msch1_pwrdisctargpwrstall(
        &mut self,
    ) -> Vio1ReqMsch1PwrdisctargpwrstallW<SocCon3Spec> {
        Vio1ReqMsch1PwrdisctargpwrstallW::new(self, 2)
    }
    #[doc = "Bit 3 - noc_viob_req_msch01_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    #[must_use]
    pub fn viob_req_msch01_pwrdisctargpwrstall(
        &mut self,
    ) -> ViobReqMsch01PwrdisctargpwrstallW<SocCon3Spec> {
        ViobReqMsch01PwrdisctargpwrstallW::new(self, 3)
    }
    #[doc = "Bit 4 - noc_viol_req_msch01_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    #[must_use]
    pub fn viol_req_msch01_pwrdisctargpwrstall(
        &mut self,
    ) -> ViolReqMsch01PwrdisctargpwrstallW<SocCon3Spec> {
        ViolReqMsch01PwrdisctargpwrstallW::new(self, 4)
    }
    #[doc = "Bit 5 - noc_usb3_fwd_perilp_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    #[must_use]
    pub fn usb3_fwd_perilp_pwrdisctargpwrstall(
        &mut self,
    ) -> Usb3FwdPerilpPwrdisctargpwrstallW<SocCon3Spec> {
        Usb3FwdPerilpPwrdisctargpwrstallW::new(self, 5)
    }
    #[doc = "Bit 6 - noc_vio_fwd_isp0_rsp_err_stall bit control"]
    #[inline(always)]
    #[must_use]
    pub fn vio_fwd_isp0_pwrdisctargpwrstall(
        &mut self,
    ) -> VioFwdIsp0PwrdisctargpwrstallW<SocCon3Spec> {
        VioFwdIsp0PwrdisctargpwrstallW::new(self, 6)
    }
    #[doc = "Bit 7 - noc_vio_fwd_isp1_rsp_err_stall bit control"]
    #[inline(always)]
    #[must_use]
    pub fn vio_fwd_isp1_pwrdisctargpwrstall(
        &mut self,
    ) -> VioFwdIsp1PwrdisctargpwrstallW<SocCon3Spec> {
        VioFwdIsp1PwrdisctargpwrstallW::new(self, 7)
    }
    #[doc = "Bit 8 - noc_vio_fwd_vopb_rsp_err_stall bit control"]
    #[inline(always)]
    #[must_use]
    pub fn vio_fwd_vopb_pwrdisctargpwrstall(
        &mut self,
    ) -> VioFwdVopbPwrdisctargpwrstallW<SocCon3Spec> {
        VioFwdVopbPwrdisctargpwrstallW::new(self, 8)
    }
    #[doc = "Bit 9 - noc_vio_fwd_vopl_rsp_err_stall bit control"]
    #[inline(always)]
    #[must_use]
    pub fn vio_fwd_vopl_pwrdisctargpwrstall(
        &mut self,
    ) -> VioFwdVoplPwrdisctargpwrstallW<SocCon3Spec> {
        VioFwdVoplPwrdisctargpwrstallW::new(self, 9)
    }
    #[doc = "Bit 10 - noc_vio_fwd_hdcp_rsp_err_stall bit control"]
    #[inline(always)]
    #[must_use]
    pub fn vio_fwd_hdcp_pwrdisctargpwrstall(
        &mut self,
    ) -> VioFwdHdcpPwrdisctargpwrstallW<SocCon3Spec> {
        VioFwdHdcpPwrdisctargpwrstallW::new(self, 10)
    }
    #[doc = "Bit 11 - noc_vopb_req_msch11_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    #[must_use]
    pub fn vopb_req_msch11_pwrdisctargpwrstall(
        &mut self,
    ) -> VopbReqMsch11PwrdisctargpwrstallW<SocCon3Spec> {
        VopbReqMsch11PwrdisctargpwrstallW::new(self, 11)
    }
    #[doc = "Bit 12 - noc_vopl_req_msch11_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    #[must_use]
    pub fn vopl_req_msch11_pwrdisctargpwrstall(
        &mut self,
    ) -> VoplReqMsch11PwrdisctargpwrstallW<SocCon3Spec> {
        VoplReqMsch11PwrdisctargpwrstallW::new(self, 12)
    }
    #[doc = "Bit 13 - noc_sd_fwd_perihp_rsp_err_stall bit control"]
    #[inline(always)]
    #[must_use]
    pub fn sd_fwd_perihp_pwrdisctargpwrstall(
        &mut self,
    ) -> SdFwdPerihpPwrdisctargpwrstallW<SocCon3Spec> {
        SdFwdPerihpPwrdisctargpwrstallW::new(self, 13)
    }
    #[doc = "Bit 14 - noc_gic_fwd_perilp_rsp_err_stall bit control"]
    #[inline(always)]
    #[must_use]
    pub fn gic_fwd_perilp_pwrdisctargpwrstall(
        &mut self,
    ) -> GicFwdPerilpPwrdisctargpwrstallW<SocCon3Spec> {
        GicFwdPerilpPwrdisctargpwrstallW::new(self, 14)
    }
    #[doc = "Bit 15 - noc_perihp_fwd_sd_rsp_err_stall bit control"]
    #[inline(always)]
    #[must_use]
    pub fn perihp_fwd_sd_pwrdisctargpwrstall(
        &mut self,
    ) -> PerihpFwdSdPwrdisctargpwrstallW<SocCon3Spec> {
        PerihpFwdSdPwrdisctargpwrstallW::new(self, 15)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<SocCon3Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "SoC control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_con3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_con3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SocCon3Spec;
impl crate::RegisterSpec for SocCon3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soc_con3::R`](R) reader structure"]
impl crate::Readable for SocCon3Spec {}
#[doc = "`write(|w| ..)` method takes [`soc_con3::W`](W) writer structure"]
impl crate::Writable for SocCon3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOC_CON3 to value 0"]
impl crate::Resettable for SocCon3Spec {
    const RESET_VALUE: u32 = 0;
}
