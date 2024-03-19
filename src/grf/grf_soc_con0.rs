#[doc = "Register `GRF_SOC_CON0` reader"]
pub type R = crate::R<GrfSocCon0Spec>;
#[doc = "Register `GRF_SOC_CON0` writer"]
pub type W = crate::W<GrfSocCon0Spec>;
#[doc = "noc_cci_fwd_perilp_rsp_err_stall bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CciFwdPerilpPwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<CciFwdPerilpPwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: CciFwdPerilpPwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCI_FWD_PERILP_PWRDISCTARGPWRSTALL` reader - noc_cci_fwd_perilp_rsp_err_stall bit control"]
pub type CciFwdPerilpPwrdisctargpwrstallR = crate::BitReader<CciFwdPerilpPwrdisctargpwrstall>;
impl CciFwdPerilpPwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CciFwdPerilpPwrdisctargpwrstall {
        match self.bits {
            false => CciFwdPerilpPwrdisctargpwrstall::B0,
            true => CciFwdPerilpPwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CciFwdPerilpPwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CciFwdPerilpPwrdisctargpwrstall::B1
    }
}
#[doc = "Field `CCI_FWD_PERILP_PWRDISCTARGPWRSTALL` writer - noc_cci_fwd_perilp_rsp_err_stall bit control"]
pub type CciFwdPerilpPwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, CciFwdPerilpPwrdisctargpwrstall>;
impl<'a, REG> CciFwdPerilpPwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CciFwdPerilpPwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CciFwdPerilpPwrdisctargpwrstall::B1)
    }
}
#[doc = "noc__rsp_err_stall bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CciReqMsch0Pwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<CciReqMsch0Pwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: CciReqMsch0Pwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCI_REQ_MSCH0_PWRDISCTARGPWRSTALL` reader - noc__rsp_err_stall bit control"]
pub type CciReqMsch0PwrdisctargpwrstallR = crate::BitReader<CciReqMsch0Pwrdisctargpwrstall>;
impl CciReqMsch0PwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CciReqMsch0Pwrdisctargpwrstall {
        match self.bits {
            false => CciReqMsch0Pwrdisctargpwrstall::B0,
            true => CciReqMsch0Pwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CciReqMsch0Pwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CciReqMsch0Pwrdisctargpwrstall::B1
    }
}
#[doc = "Field `CCI_REQ_MSCH0_PWRDISCTARGPWRSTALL` writer - noc__rsp_err_stall bit control"]
pub type CciReqMsch0PwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, CciReqMsch0Pwrdisctargpwrstall>;
impl<'a, REG> CciReqMsch0PwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CciReqMsch0Pwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CciReqMsch0Pwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_cci_req_msch1_rsp_err_stall bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CciReqMsch1Pwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<CciReqMsch1Pwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: CciReqMsch1Pwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCI_REQ_MSCH1_PWRDISCTARGPWRSTALL` reader - noc_cci_req_msch1_rsp_err_stall bit control"]
pub type CciReqMsch1PwrdisctargpwrstallR = crate::BitReader<CciReqMsch1Pwrdisctargpwrstall>;
impl CciReqMsch1PwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CciReqMsch1Pwrdisctargpwrstall {
        match self.bits {
            false => CciReqMsch1Pwrdisctargpwrstall::B0,
            true => CciReqMsch1Pwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CciReqMsch1Pwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CciReqMsch1Pwrdisctargpwrstall::B1
    }
}
#[doc = "Field `CCI_REQ_MSCH1_PWRDISCTARGPWRSTALL` writer - noc_cci_req_msch1_rsp_err_stall bit control"]
pub type CciReqMsch1PwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, CciReqMsch1Pwrdisctargpwrstall>;
impl<'a, REG> CciReqMsch1PwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CciReqMsch1Pwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CciReqMsch1Pwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_center_fwd_edp_rsp_err_stall bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CenterFwdEdpPwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<CenterFwdEdpPwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: CenterFwdEdpPwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CENTER_FWD_EDP_PWRDISCTARGPWRSTALL` reader - noc_center_fwd_edp_rsp_err_stall bit control"]
pub type CenterFwdEdpPwrdisctargpwrstallR = crate::BitReader<CenterFwdEdpPwrdisctargpwrstall>;
impl CenterFwdEdpPwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CenterFwdEdpPwrdisctargpwrstall {
        match self.bits {
            false => CenterFwdEdpPwrdisctargpwrstall::B0,
            true => CenterFwdEdpPwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CenterFwdEdpPwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CenterFwdEdpPwrdisctargpwrstall::B1
    }
}
#[doc = "Field `CENTER_FWD_EDP_PWRDISCTARGPWRSTALL` writer - noc_center_fwd_edp_rsp_err_stall bit control"]
pub type CenterFwdEdpPwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, CenterFwdEdpPwrdisctargpwrstall>;
impl<'a, REG> CenterFwdEdpPwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CenterFwdEdpPwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CenterFwdEdpPwrdisctargpwrstall::B1)
    }
}
#[doc = "perilp_fwd_emmc_rsp_err_stall bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PerilpFwdEmmcPwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<PerilpFwdEmmcPwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: PerilpFwdEmmcPwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERILP_FWD_EMMC_PWRDISCTARGPWRSTALL` reader - perilp_fwd_emmc_rsp_err_stall bit control"]
pub type PerilpFwdEmmcPwrdisctargpwrstallR = crate::BitReader<PerilpFwdEmmcPwrdisctargpwrstall>;
impl PerilpFwdEmmcPwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PerilpFwdEmmcPwrdisctargpwrstall {
        match self.bits {
            false => PerilpFwdEmmcPwrdisctargpwrstall::B0,
            true => PerilpFwdEmmcPwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PerilpFwdEmmcPwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PerilpFwdEmmcPwrdisctargpwrstall::B1
    }
}
#[doc = "Field `PERILP_FWD_EMMC_PWRDISCTARGPWRSTALL` writer - perilp_fwd_emmc_rsp_err_stall bit control"]
pub type PerilpFwdEmmcPwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, PerilpFwdEmmcPwrdisctargpwrstall>;
impl<'a, REG> PerilpFwdEmmcPwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PerilpFwdEmmcPwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PerilpFwdEmmcPwrdisctargpwrstall::B1)
    }
}
#[doc = "perilp_fwd_gmac_rsp_err_stall bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PerilpFwdGmacPwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<PerilpFwdGmacPwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: PerilpFwdGmacPwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERILP_FWD_GMAC_PWRDISCTARGPWRSTALL` reader - perilp_fwd_gmac_rsp_err_stall bit control"]
pub type PerilpFwdGmacPwrdisctargpwrstallR = crate::BitReader<PerilpFwdGmacPwrdisctargpwrstall>;
impl PerilpFwdGmacPwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PerilpFwdGmacPwrdisctargpwrstall {
        match self.bits {
            false => PerilpFwdGmacPwrdisctargpwrstall::B0,
            true => PerilpFwdGmacPwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PerilpFwdGmacPwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PerilpFwdGmacPwrdisctargpwrstall::B1
    }
}
#[doc = "Field `PERILP_FWD_GMAC_PWRDISCTARGPWRSTALL` writer - perilp_fwd_gmac_rsp_err_stall bit control"]
pub type PerilpFwdGmacPwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, PerilpFwdGmacPwrdisctargpwrstall>;
impl<'a, REG> PerilpFwdGmacPwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PerilpFwdGmacPwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PerilpFwdGmacPwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_center_fwd_gpu_rsp_err_stall bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CenterFwdGpuPwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<CenterFwdGpuPwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: CenterFwdGpuPwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CENTER_FWD_GPU_PWRDISCTARGPWRSTALL` reader - noc_center_fwd_gpu_rsp_err_stall bit control"]
pub type CenterFwdGpuPwrdisctargpwrstallR = crate::BitReader<CenterFwdGpuPwrdisctargpwrstall>;
impl CenterFwdGpuPwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CenterFwdGpuPwrdisctargpwrstall {
        match self.bits {
            false => CenterFwdGpuPwrdisctargpwrstall::B0,
            true => CenterFwdGpuPwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CenterFwdGpuPwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CenterFwdGpuPwrdisctargpwrstall::B1
    }
}
#[doc = "Field `CENTER_FWD_GPU_PWRDISCTARGPWRSTALL` writer - noc_center_fwd_gpu_rsp_err_stall bit control"]
pub type CenterFwdGpuPwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, CenterFwdGpuPwrdisctargpwrstall>;
impl<'a, REG> CenterFwdGpuPwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CenterFwdGpuPwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CenterFwdGpuPwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_center_fwd_iep_rsp_err_stall bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CenterFwdIepPwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<CenterFwdIepPwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: CenterFwdIepPwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CENTER_FWD_IEP_PWRDISCTARGPWRSTALL` reader - noc_center_fwd_iep_rsp_err_stall bit control"]
pub type CenterFwdIepPwrdisctargpwrstallR = crate::BitReader<CenterFwdIepPwrdisctargpwrstall>;
impl CenterFwdIepPwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CenterFwdIepPwrdisctargpwrstall {
        match self.bits {
            false => CenterFwdIepPwrdisctargpwrstall::B0,
            true => CenterFwdIepPwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CenterFwdIepPwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CenterFwdIepPwrdisctargpwrstall::B1
    }
}
#[doc = "Field `CENTER_FWD_IEP_PWRDISCTARGPWRSTALL` writer - noc_center_fwd_iep_rsp_err_stall bit control"]
pub type CenterFwdIepPwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, CenterFwdIepPwrdisctargpwrstall>;
impl<'a, REG> CenterFwdIepPwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CenterFwdIepPwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CenterFwdIepPwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_center_fwd_perihp_rsp_err_stall bit\n\ncontrol\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CenterFwdPerihpPwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<CenterFwdPerihpPwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: CenterFwdPerihpPwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CENTER_FWD_PERIHP_PWRDISCTARGPWRSTALL` reader - noc_center_fwd_perihp_rsp_err_stall bit\n\ncontrol"]
pub type CenterFwdPerihpPwrdisctargpwrstallR = crate::BitReader<CenterFwdPerihpPwrdisctargpwrstall>;
impl CenterFwdPerihpPwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CenterFwdPerihpPwrdisctargpwrstall {
        match self.bits {
            false => CenterFwdPerihpPwrdisctargpwrstall::B0,
            true => CenterFwdPerihpPwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CenterFwdPerihpPwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CenterFwdPerihpPwrdisctargpwrstall::B1
    }
}
#[doc = "Field `CENTER_FWD_PERIHP_PWRDISCTARGPWRSTALL` writer - noc_center_fwd_perihp_rsp_err_stall bit\n\ncontrol"]
pub type CenterFwdPerihpPwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, CenterFwdPerihpPwrdisctargpwrstall>;
impl<'a, REG> CenterFwdPerihpPwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CenterFwdPerihpPwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CenterFwdPerihpPwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_center_fwd_rga_rsp_err_stall bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CenterFwdRgaPwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<CenterFwdRgaPwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: CenterFwdRgaPwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CENTER_FWD_RGA_PWRDISCTARGPWRSTALL` reader - noc_center_fwd_rga_rsp_err_stall bit control"]
pub type CenterFwdRgaPwrdisctargpwrstallR = crate::BitReader<CenterFwdRgaPwrdisctargpwrstall>;
impl CenterFwdRgaPwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CenterFwdRgaPwrdisctargpwrstall {
        match self.bits {
            false => CenterFwdRgaPwrdisctargpwrstall::B0,
            true => CenterFwdRgaPwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CenterFwdRgaPwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CenterFwdRgaPwrdisctargpwrstall::B1
    }
}
#[doc = "Field `CENTER_FWD_RGA_PWRDISCTARGPWRSTALL` writer - noc_center_fwd_rga_rsp_err_stall bit control"]
pub type CenterFwdRgaPwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, CenterFwdRgaPwrdisctargpwrstall>;
impl<'a, REG> CenterFwdRgaPwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CenterFwdRgaPwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CenterFwdRgaPwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_center_fwd_usb3_rsp_err_stall bit\n\ncontrol\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CenterFwdUsb3Pwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<CenterFwdUsb3Pwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: CenterFwdUsb3Pwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CENTER_FWD_USB3_PWRDISCTARGPWRSTALL` reader - noc_center_fwd_usb3_rsp_err_stall bit\n\ncontrol"]
pub type CenterFwdUsb3PwrdisctargpwrstallR = crate::BitReader<CenterFwdUsb3Pwrdisctargpwrstall>;
impl CenterFwdUsb3PwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CenterFwdUsb3Pwrdisctargpwrstall {
        match self.bits {
            false => CenterFwdUsb3Pwrdisctargpwrstall::B0,
            true => CenterFwdUsb3Pwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CenterFwdUsb3Pwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CenterFwdUsb3Pwrdisctargpwrstall::B1
    }
}
#[doc = "Field `CENTER_FWD_USB3_PWRDISCTARGPWRSTALL` writer - noc_center_fwd_usb3_rsp_err_stall bit\n\ncontrol"]
pub type CenterFwdUsb3PwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, CenterFwdUsb3Pwrdisctargpwrstall>;
impl<'a, REG> CenterFwdUsb3PwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CenterFwdUsb3Pwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CenterFwdUsb3Pwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_center_fwd_vcodec_rsp_err_stall bit\n\ncontrol\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CenterFwdVcodecPwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<CenterFwdVcodecPwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: CenterFwdVcodecPwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CENTER_FWD_VCODEC_PWRDISCTARGPWRSTALL` reader - noc_center_fwd_vcodec_rsp_err_stall bit\n\ncontrol"]
pub type CenterFwdVcodecPwrdisctargpwrstallR = crate::BitReader<CenterFwdVcodecPwrdisctargpwrstall>;
impl CenterFwdVcodecPwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CenterFwdVcodecPwrdisctargpwrstall {
        match self.bits {
            false => CenterFwdVcodecPwrdisctargpwrstall::B0,
            true => CenterFwdVcodecPwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CenterFwdVcodecPwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CenterFwdVcodecPwrdisctargpwrstall::B1
    }
}
#[doc = "Field `CENTER_FWD_VCODEC_PWRDISCTARGPWRSTALL` writer - noc_center_fwd_vcodec_rsp_err_stall bit\n\ncontrol"]
pub type CenterFwdVcodecPwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, CenterFwdVcodecPwrdisctargpwrstall>;
impl<'a, REG> CenterFwdVcodecPwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CenterFwdVcodecPwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CenterFwdVcodecPwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_center_fwd_vdu_rsp_err_stall bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CenterFwdVduPwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<CenterFwdVduPwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: CenterFwdVduPwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CENTER_FWD_VDU_PWRDISCTARGPWRSTALL` reader - noc_center_fwd_vdu_rsp_err_stall bit control"]
pub type CenterFwdVduPwrdisctargpwrstallR = crate::BitReader<CenterFwdVduPwrdisctargpwrstall>;
impl CenterFwdVduPwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CenterFwdVduPwrdisctargpwrstall {
        match self.bits {
            false => CenterFwdVduPwrdisctargpwrstall::B0,
            true => CenterFwdVduPwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CenterFwdVduPwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CenterFwdVduPwrdisctargpwrstall::B1
    }
}
#[doc = "Field `CENTER_FWD_VDU_PWRDISCTARGPWRSTALL` writer - noc_center_fwd_vdu_rsp_err_stall bit control"]
pub type CenterFwdVduPwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, CenterFwdVduPwrdisctargpwrstall>;
impl<'a, REG> CenterFwdVduPwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CenterFwdVduPwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CenterFwdVduPwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_center_fwd_vio_rsp_err_stall bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CenterFwdVioPwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<CenterFwdVioPwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: CenterFwdVioPwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CENTER_FWD_VIO_PWRDISCTARGPWRSTALL` reader - noc_center_fwd_vio_rsp_err_stall bit control"]
pub type CenterFwdVioPwrdisctargpwrstallR = crate::BitReader<CenterFwdVioPwrdisctargpwrstall>;
impl CenterFwdVioPwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CenterFwdVioPwrdisctargpwrstall {
        match self.bits {
            false => CenterFwdVioPwrdisctargpwrstall::B0,
            true => CenterFwdVioPwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CenterFwdVioPwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CenterFwdVioPwrdisctargpwrstall::B1
    }
}
#[doc = "Field `CENTER_FWD_VIO_PWRDISCTARGPWRSTALL` writer - noc_center_fwd_vio_rsp_err_stall bit control"]
pub type CenterFwdVioPwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, CenterFwdVioPwrdisctargpwrstall>;
impl<'a, REG> CenterFwdVioPwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CenterFwdVioPwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CenterFwdVioPwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_centersrv_fwd_ccim1_rsp_err_stall bit\n\ncontrol\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CentersrvFwdCcim1Pwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<CentersrvFwdCcim1Pwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: CentersrvFwdCcim1Pwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CENTERSRV_FWD_CCIM1_PWRDISCTARGPWRSTALL` reader - noc_centersrv_fwd_ccim1_rsp_err_stall bit\n\ncontrol"]
pub type CentersrvFwdCcim1PwrdisctargpwrstallR =
    crate::BitReader<CentersrvFwdCcim1Pwrdisctargpwrstall>;
impl CentersrvFwdCcim1PwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CentersrvFwdCcim1Pwrdisctargpwrstall {
        match self.bits {
            false => CentersrvFwdCcim1Pwrdisctargpwrstall::B0,
            true => CentersrvFwdCcim1Pwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CentersrvFwdCcim1Pwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CentersrvFwdCcim1Pwrdisctargpwrstall::B1
    }
}
#[doc = "Field `CENTERSRV_FWD_CCIM1_PWRDISCTARGPWRSTALL` writer - noc_centersrv_fwd_ccim1_rsp_err_stall bit\n\ncontrol"]
pub type CentersrvFwdCcim1PwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, CentersrvFwdCcim1Pwrdisctargpwrstall>;
impl<'a, REG> CentersrvFwdCcim1PwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CentersrvFwdCcim1Pwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CentersrvFwdCcim1Pwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_emmc_fwd_perihp_rsp_err_stall bit\n\ncontrol\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmmcFwdPerihpPwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<EmmcFwdPerihpPwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: EmmcFwdPerihpPwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EMMC_FWD_PERIHP_PWRDISCTARGPWRSTALL` reader - noc_emmc_fwd_perihp_rsp_err_stall bit\n\ncontrol"]
pub type EmmcFwdPerihpPwrdisctargpwrstallR = crate::BitReader<EmmcFwdPerihpPwrdisctargpwrstall>;
impl EmmcFwdPerihpPwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EmmcFwdPerihpPwrdisctargpwrstall {
        match self.bits {
            false => EmmcFwdPerihpPwrdisctargpwrstall::B0,
            true => EmmcFwdPerihpPwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == EmmcFwdPerihpPwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == EmmcFwdPerihpPwrdisctargpwrstall::B1
    }
}
#[doc = "Field `EMMC_FWD_PERIHP_PWRDISCTARGPWRSTALL` writer - noc_emmc_fwd_perihp_rsp_err_stall bit\n\ncontrol"]
pub type EmmcFwdPerihpPwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, EmmcFwdPerihpPwrdisctargpwrstall>;
impl<'a, REG> EmmcFwdPerihpPwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(EmmcFwdPerihpPwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(EmmcFwdPerihpPwrdisctargpwrstall::B1)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - noc_cci_fwd_perilp_rsp_err_stall bit control"]
    #[inline(always)]
    pub fn cci_fwd_perilp_pwrdisctargpwrstall(&self) -> CciFwdPerilpPwrdisctargpwrstallR {
        CciFwdPerilpPwrdisctargpwrstallR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - noc__rsp_err_stall bit control"]
    #[inline(always)]
    pub fn cci_req_msch0_pwrdisctargpwrstall(&self) -> CciReqMsch0PwrdisctargpwrstallR {
        CciReqMsch0PwrdisctargpwrstallR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - noc_cci_req_msch1_rsp_err_stall bit control"]
    #[inline(always)]
    pub fn cci_req_msch1_pwrdisctargpwrstall(&self) -> CciReqMsch1PwrdisctargpwrstallR {
        CciReqMsch1PwrdisctargpwrstallR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - noc_center_fwd_edp_rsp_err_stall bit control"]
    #[inline(always)]
    pub fn center_fwd_edp_pwrdisctargpwrstall(&self) -> CenterFwdEdpPwrdisctargpwrstallR {
        CenterFwdEdpPwrdisctargpwrstallR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - perilp_fwd_emmc_rsp_err_stall bit control"]
    #[inline(always)]
    pub fn perilp_fwd_emmc_pwrdisctargpwrstall(&self) -> PerilpFwdEmmcPwrdisctargpwrstallR {
        PerilpFwdEmmcPwrdisctargpwrstallR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - perilp_fwd_gmac_rsp_err_stall bit control"]
    #[inline(always)]
    pub fn perilp_fwd_gmac_pwrdisctargpwrstall(&self) -> PerilpFwdGmacPwrdisctargpwrstallR {
        PerilpFwdGmacPwrdisctargpwrstallR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - noc_center_fwd_gpu_rsp_err_stall bit control"]
    #[inline(always)]
    pub fn center_fwd_gpu_pwrdisctargpwrstall(&self) -> CenterFwdGpuPwrdisctargpwrstallR {
        CenterFwdGpuPwrdisctargpwrstallR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - noc_center_fwd_iep_rsp_err_stall bit control"]
    #[inline(always)]
    pub fn center_fwd_iep_pwrdisctargpwrstall(&self) -> CenterFwdIepPwrdisctargpwrstallR {
        CenterFwdIepPwrdisctargpwrstallR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - noc_center_fwd_perihp_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    pub fn center_fwd_perihp_pwrdisctargpwrstall(&self) -> CenterFwdPerihpPwrdisctargpwrstallR {
        CenterFwdPerihpPwrdisctargpwrstallR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - noc_center_fwd_rga_rsp_err_stall bit control"]
    #[inline(always)]
    pub fn center_fwd_rga_pwrdisctargpwrstall(&self) -> CenterFwdRgaPwrdisctargpwrstallR {
        CenterFwdRgaPwrdisctargpwrstallR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - noc_center_fwd_usb3_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    pub fn center_fwd_usb3_pwrdisctargpwrstall(&self) -> CenterFwdUsb3PwrdisctargpwrstallR {
        CenterFwdUsb3PwrdisctargpwrstallR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - noc_center_fwd_vcodec_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    pub fn center_fwd_vcodec_pwrdisctargpwrstall(&self) -> CenterFwdVcodecPwrdisctargpwrstallR {
        CenterFwdVcodecPwrdisctargpwrstallR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - noc_center_fwd_vdu_rsp_err_stall bit control"]
    #[inline(always)]
    pub fn center_fwd_vdu_pwrdisctargpwrstall(&self) -> CenterFwdVduPwrdisctargpwrstallR {
        CenterFwdVduPwrdisctargpwrstallR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - noc_center_fwd_vio_rsp_err_stall bit control"]
    #[inline(always)]
    pub fn center_fwd_vio_pwrdisctargpwrstall(&self) -> CenterFwdVioPwrdisctargpwrstallR {
        CenterFwdVioPwrdisctargpwrstallR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - noc_centersrv_fwd_ccim1_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    pub fn centersrv_fwd_ccim1_pwrdisctargpwrstall(&self) -> CentersrvFwdCcim1PwrdisctargpwrstallR {
        CentersrvFwdCcim1PwrdisctargpwrstallR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - noc_emmc_fwd_perihp_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    pub fn emmc_fwd_perihp_pwrdisctargpwrstall(&self) -> EmmcFwdPerihpPwrdisctargpwrstallR {
        EmmcFwdPerihpPwrdisctargpwrstallR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - noc_cci_fwd_perilp_rsp_err_stall bit control"]
    #[inline(always)]
    #[must_use]
    pub fn cci_fwd_perilp_pwrdisctargpwrstall(
        &mut self,
    ) -> CciFwdPerilpPwrdisctargpwrstallW<GrfSocCon0Spec> {
        CciFwdPerilpPwrdisctargpwrstallW::new(self, 0)
    }
    #[doc = "Bit 1 - noc__rsp_err_stall bit control"]
    #[inline(always)]
    #[must_use]
    pub fn cci_req_msch0_pwrdisctargpwrstall(
        &mut self,
    ) -> CciReqMsch0PwrdisctargpwrstallW<GrfSocCon0Spec> {
        CciReqMsch0PwrdisctargpwrstallW::new(self, 1)
    }
    #[doc = "Bit 2 - noc_cci_req_msch1_rsp_err_stall bit control"]
    #[inline(always)]
    #[must_use]
    pub fn cci_req_msch1_pwrdisctargpwrstall(
        &mut self,
    ) -> CciReqMsch1PwrdisctargpwrstallW<GrfSocCon0Spec> {
        CciReqMsch1PwrdisctargpwrstallW::new(self, 2)
    }
    #[doc = "Bit 3 - noc_center_fwd_edp_rsp_err_stall bit control"]
    #[inline(always)]
    #[must_use]
    pub fn center_fwd_edp_pwrdisctargpwrstall(
        &mut self,
    ) -> CenterFwdEdpPwrdisctargpwrstallW<GrfSocCon0Spec> {
        CenterFwdEdpPwrdisctargpwrstallW::new(self, 3)
    }
    #[doc = "Bit 4 - perilp_fwd_emmc_rsp_err_stall bit control"]
    #[inline(always)]
    #[must_use]
    pub fn perilp_fwd_emmc_pwrdisctargpwrstall(
        &mut self,
    ) -> PerilpFwdEmmcPwrdisctargpwrstallW<GrfSocCon0Spec> {
        PerilpFwdEmmcPwrdisctargpwrstallW::new(self, 4)
    }
    #[doc = "Bit 5 - perilp_fwd_gmac_rsp_err_stall bit control"]
    #[inline(always)]
    #[must_use]
    pub fn perilp_fwd_gmac_pwrdisctargpwrstall(
        &mut self,
    ) -> PerilpFwdGmacPwrdisctargpwrstallW<GrfSocCon0Spec> {
        PerilpFwdGmacPwrdisctargpwrstallW::new(self, 5)
    }
    #[doc = "Bit 6 - noc_center_fwd_gpu_rsp_err_stall bit control"]
    #[inline(always)]
    #[must_use]
    pub fn center_fwd_gpu_pwrdisctargpwrstall(
        &mut self,
    ) -> CenterFwdGpuPwrdisctargpwrstallW<GrfSocCon0Spec> {
        CenterFwdGpuPwrdisctargpwrstallW::new(self, 6)
    }
    #[doc = "Bit 7 - noc_center_fwd_iep_rsp_err_stall bit control"]
    #[inline(always)]
    #[must_use]
    pub fn center_fwd_iep_pwrdisctargpwrstall(
        &mut self,
    ) -> CenterFwdIepPwrdisctargpwrstallW<GrfSocCon0Spec> {
        CenterFwdIepPwrdisctargpwrstallW::new(self, 7)
    }
    #[doc = "Bit 8 - noc_center_fwd_perihp_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    #[must_use]
    pub fn center_fwd_perihp_pwrdisctargpwrstall(
        &mut self,
    ) -> CenterFwdPerihpPwrdisctargpwrstallW<GrfSocCon0Spec> {
        CenterFwdPerihpPwrdisctargpwrstallW::new(self, 8)
    }
    #[doc = "Bit 9 - noc_center_fwd_rga_rsp_err_stall bit control"]
    #[inline(always)]
    #[must_use]
    pub fn center_fwd_rga_pwrdisctargpwrstall(
        &mut self,
    ) -> CenterFwdRgaPwrdisctargpwrstallW<GrfSocCon0Spec> {
        CenterFwdRgaPwrdisctargpwrstallW::new(self, 9)
    }
    #[doc = "Bit 10 - noc_center_fwd_usb3_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    #[must_use]
    pub fn center_fwd_usb3_pwrdisctargpwrstall(
        &mut self,
    ) -> CenterFwdUsb3PwrdisctargpwrstallW<GrfSocCon0Spec> {
        CenterFwdUsb3PwrdisctargpwrstallW::new(self, 10)
    }
    #[doc = "Bit 11 - noc_center_fwd_vcodec_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    #[must_use]
    pub fn center_fwd_vcodec_pwrdisctargpwrstall(
        &mut self,
    ) -> CenterFwdVcodecPwrdisctargpwrstallW<GrfSocCon0Spec> {
        CenterFwdVcodecPwrdisctargpwrstallW::new(self, 11)
    }
    #[doc = "Bit 12 - noc_center_fwd_vdu_rsp_err_stall bit control"]
    #[inline(always)]
    #[must_use]
    pub fn center_fwd_vdu_pwrdisctargpwrstall(
        &mut self,
    ) -> CenterFwdVduPwrdisctargpwrstallW<GrfSocCon0Spec> {
        CenterFwdVduPwrdisctargpwrstallW::new(self, 12)
    }
    #[doc = "Bit 13 - noc_center_fwd_vio_rsp_err_stall bit control"]
    #[inline(always)]
    #[must_use]
    pub fn center_fwd_vio_pwrdisctargpwrstall(
        &mut self,
    ) -> CenterFwdVioPwrdisctargpwrstallW<GrfSocCon0Spec> {
        CenterFwdVioPwrdisctargpwrstallW::new(self, 13)
    }
    #[doc = "Bit 14 - noc_centersrv_fwd_ccim1_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    #[must_use]
    pub fn centersrv_fwd_ccim1_pwrdisctargpwrstall(
        &mut self,
    ) -> CentersrvFwdCcim1PwrdisctargpwrstallW<GrfSocCon0Spec> {
        CentersrvFwdCcim1PwrdisctargpwrstallW::new(self, 14)
    }
    #[doc = "Bit 15 - noc_emmc_fwd_perihp_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    #[must_use]
    pub fn emmc_fwd_perihp_pwrdisctargpwrstall(
        &mut self,
    ) -> EmmcFwdPerihpPwrdisctargpwrstallW<GrfSocCon0Spec> {
        EmmcFwdPerihpPwrdisctargpwrstallW::new(self, 15)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfSocCon0Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "SoC control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_soc_con0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_soc_con0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfSocCon0Spec;
impl crate::RegisterSpec for GrfSocCon0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_soc_con0::R`](R) reader structure"]
impl crate::Readable for GrfSocCon0Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_soc_con0::W`](W) writer structure"]
impl crate::Writable for GrfSocCon0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_SOC_CON0 to value 0"]
impl crate::Resettable for GrfSocCon0Spec {
    const RESET_VALUE: u32 = 0;
}
