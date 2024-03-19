#[doc = "Register `GRF_SOC_CON2` reader"]
pub type R = crate::R<GrfSocCon2Spec>;
#[doc = "Register `GRF_SOC_CON2` writer"]
pub type W = crate::W<GrfSocCon2Spec>;
#[doc = "noc_perilp_fwd_center_rsp_err_stall bit\n\ncontrol\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PerilpFwdCenterPwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<PerilpFwdCenterPwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: PerilpFwdCenterPwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERILP_FWD_CENTER_PWRDISCTARGPWRSTALL` reader - noc_perilp_fwd_center_rsp_err_stall bit\n\ncontrol"]
pub type PerilpFwdCenterPwrdisctargpwrstallR = crate::BitReader<PerilpFwdCenterPwrdisctargpwrstall>;
impl PerilpFwdCenterPwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PerilpFwdCenterPwrdisctargpwrstall {
        match self.bits {
            false => PerilpFwdCenterPwrdisctargpwrstall::B0,
            true => PerilpFwdCenterPwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PerilpFwdCenterPwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PerilpFwdCenterPwrdisctargpwrstall::B1
    }
}
#[doc = "Field `PERILP_FWD_CENTER_PWRDISCTARGPWRSTALL` writer - noc_perilp_fwd_center_rsp_err_stall bit\n\ncontrol"]
pub type PerilpFwdCenterPwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, PerilpFwdCenterPwrdisctargpwrstall>;
impl<'a, REG> PerilpFwdCenterPwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PerilpFwdCenterPwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PerilpFwdCenterPwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_perilp_fwd_pmu_rsp_err_stall bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PerilpFwdPmuPwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<PerilpFwdPmuPwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: PerilpFwdPmuPwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERILP_FWD_PMU_PWRDISCTARGPWRSTALL` reader - noc_perilp_fwd_pmu_rsp_err_stall bit control"]
pub type PerilpFwdPmuPwrdisctargpwrstallR = crate::BitReader<PerilpFwdPmuPwrdisctargpwrstall>;
impl PerilpFwdPmuPwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PerilpFwdPmuPwrdisctargpwrstall {
        match self.bits {
            false => PerilpFwdPmuPwrdisctargpwrstall::B0,
            true => PerilpFwdPmuPwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PerilpFwdPmuPwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PerilpFwdPmuPwrdisctargpwrstall::B1
    }
}
#[doc = "Field `PERILP_FWD_PMU_PWRDISCTARGPWRSTALL` writer - noc_perilp_fwd_pmu_rsp_err_stall bit control"]
pub type PerilpFwdPmuPwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, PerilpFwdPmuPwrdisctargpwrstall>;
impl<'a, REG> PerilpFwdPmuPwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PerilpFwdPmuPwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PerilpFwdPmuPwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_perilp_req_msch0_rsp_err_stall bit\n\ncontrol\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PerilpReqMsch0Pwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<PerilpReqMsch0Pwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: PerilpReqMsch0Pwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERILP_REQ_MSCH0_PWRDISCTARGPWRSTALL` reader - noc_perilp_req_msch0_rsp_err_stall bit\n\ncontrol"]
pub type PerilpReqMsch0PwrdisctargpwrstallR = crate::BitReader<PerilpReqMsch0Pwrdisctargpwrstall>;
impl PerilpReqMsch0PwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PerilpReqMsch0Pwrdisctargpwrstall {
        match self.bits {
            false => PerilpReqMsch0Pwrdisctargpwrstall::B0,
            true => PerilpReqMsch0Pwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PerilpReqMsch0Pwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PerilpReqMsch0Pwrdisctargpwrstall::B1
    }
}
#[doc = "Field `PERILP_REQ_MSCH0_PWRDISCTARGPWRSTALL` writer - noc_perilp_req_msch0_rsp_err_stall bit\n\ncontrol"]
pub type PerilpReqMsch0PwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, PerilpReqMsch0Pwrdisctargpwrstall>;
impl<'a, REG> PerilpReqMsch0PwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PerilpReqMsch0Pwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PerilpReqMsch0Pwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_perilp_req_msch1_rsp_err_stall bit\n\ncontrol\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PerilpReqMsch1Pwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<PerilpReqMsch1Pwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: PerilpReqMsch1Pwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERILP_REQ_MSCH1_PWRDISCTARGPWRSTALL` reader - noc_perilp_req_msch1_rsp_err_stall bit\n\ncontrol"]
pub type PerilpReqMsch1PwrdisctargpwrstallR = crate::BitReader<PerilpReqMsch1Pwrdisctargpwrstall>;
impl PerilpReqMsch1PwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PerilpReqMsch1Pwrdisctargpwrstall {
        match self.bits {
            false => PerilpReqMsch1Pwrdisctargpwrstall::B0,
            true => PerilpReqMsch1Pwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PerilpReqMsch1Pwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PerilpReqMsch1Pwrdisctargpwrstall::B1
    }
}
#[doc = "Field `PERILP_REQ_MSCH1_PWRDISCTARGPWRSTALL` writer - noc_perilp_req_msch1_rsp_err_stall bit\n\ncontrol"]
pub type PerilpReqMsch1PwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, PerilpReqMsch1Pwrdisctargpwrstall>;
impl<'a, REG> PerilpReqMsch1PwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PerilpReqMsch1Pwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PerilpReqMsch1Pwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_perilpsrv_fwd_cm0_rsp_err_stall bit\n\ncontrol\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PerilpsrvFwdCm0Pwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<PerilpsrvFwdCm0Pwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: PerilpsrvFwdCm0Pwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERILPSRV_FWD_CM0_PWRDISCTARGPWRSTALL` reader - noc_perilpsrv_fwd_cm0_rsp_err_stall bit\n\ncontrol"]
pub type PerilpsrvFwdCm0PwrdisctargpwrstallR = crate::BitReader<PerilpsrvFwdCm0Pwrdisctargpwrstall>;
impl PerilpsrvFwdCm0PwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PerilpsrvFwdCm0Pwrdisctargpwrstall {
        match self.bits {
            false => PerilpsrvFwdCm0Pwrdisctargpwrstall::B0,
            true => PerilpsrvFwdCm0Pwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PerilpsrvFwdCm0Pwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PerilpsrvFwdCm0Pwrdisctargpwrstall::B1
    }
}
#[doc = "Field `PERILPSRV_FWD_CM0_PWRDISCTARGPWRSTALL` writer - noc_perilpsrv_fwd_cm0_rsp_err_stall bit\n\ncontrol"]
pub type PerilpsrvFwdCm0PwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, PerilpsrvFwdCm0Pwrdisctargpwrstall>;
impl<'a, REG> PerilpsrvFwdCm0PwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PerilpsrvFwdCm0Pwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PerilpsrvFwdCm0Pwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_gmac_fwd_perihp_rsp_err_stall bit\n\ncontrol\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GmacFwdPerihpPwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<GmacFwdPerihpPwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: GmacFwdPerihpPwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GMAC_FWD_PERIHP_PWRDISCTARGPWRSTALL` reader - noc_gmac_fwd_perihp_rsp_err_stall bit\n\ncontrol"]
pub type GmacFwdPerihpPwrdisctargpwrstallR = crate::BitReader<GmacFwdPerihpPwrdisctargpwrstall>;
impl GmacFwdPerihpPwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GmacFwdPerihpPwrdisctargpwrstall {
        match self.bits {
            false => GmacFwdPerihpPwrdisctargpwrstall::B0,
            true => GmacFwdPerihpPwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == GmacFwdPerihpPwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == GmacFwdPerihpPwrdisctargpwrstall::B1
    }
}
#[doc = "Field `GMAC_FWD_PERIHP_PWRDISCTARGPWRSTALL` writer - noc_gmac_fwd_perihp_rsp_err_stall bit\n\ncontrol"]
pub type GmacFwdPerihpPwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, GmacFwdPerihpPwrdisctargpwrstall>;
impl<'a, REG> GmacFwdPerihpPwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(GmacFwdPerihpPwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(GmacFwdPerihpPwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_pmu_fwd_perilp_rsp_err_stall bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SdioaudioFwdPerilpPwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<SdioaudioFwdPerilpPwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: SdioaudioFwdPerilpPwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIOAUDIO_FWD_PERILP_PWRDISCTARGPWRSTALL` reader - noc_pmu_fwd_perilp_rsp_err_stall bit control"]
pub type SdioaudioFwdPerilpPwrdisctargpwrstallR =
    crate::BitReader<SdioaudioFwdPerilpPwrdisctargpwrstall>;
impl SdioaudioFwdPerilpPwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SdioaudioFwdPerilpPwrdisctargpwrstall {
        match self.bits {
            false => SdioaudioFwdPerilpPwrdisctargpwrstall::B0,
            true => SdioaudioFwdPerilpPwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SdioaudioFwdPerilpPwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SdioaudioFwdPerilpPwrdisctargpwrstall::B1
    }
}
#[doc = "Field `SDIOAUDIO_FWD_PERILP_PWRDISCTARGPWRSTALL` writer - noc_pmu_fwd_perilp_rsp_err_stall bit control"]
pub type SdioaudioFwdPerilpPwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, SdioaudioFwdPerilpPwrdisctargpwrstall>;
impl<'a, REG> SdioaudioFwdPerilpPwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SdioaudioFwdPerilpPwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SdioaudioFwdPerilpPwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_rga_req_msch0_rsp_err_stall bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RgaReqMsch0Pwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<RgaReqMsch0Pwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: RgaReqMsch0Pwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGA_REQ_MSCH0_PWRDISCTARGPWRSTALL` reader - noc_rga_req_msch0_rsp_err_stall bit control"]
pub type RgaReqMsch0PwrdisctargpwrstallR = crate::BitReader<RgaReqMsch0Pwrdisctargpwrstall>;
impl RgaReqMsch0PwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RgaReqMsch0Pwrdisctargpwrstall {
        match self.bits {
            false => RgaReqMsch0Pwrdisctargpwrstall::B0,
            true => RgaReqMsch0Pwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == RgaReqMsch0Pwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == RgaReqMsch0Pwrdisctargpwrstall::B1
    }
}
#[doc = "Field `RGA_REQ_MSCH0_PWRDISCTARGPWRSTALL` writer - noc_rga_req_msch0_rsp_err_stall bit control"]
pub type RgaReqMsch0PwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, RgaReqMsch0Pwrdisctargpwrstall>;
impl<'a, REG> RgaReqMsch0PwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(RgaReqMsch0Pwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(RgaReqMsch0Pwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_rga_req_msch1_rsp_err_stall bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RgaReqMsch1Pwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<RgaReqMsch1Pwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: RgaReqMsch1Pwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGA_REQ_MSCH1_PWRDISCTARGPWRSTALL` reader - noc_rga_req_msch1_rsp_err_stall bit control"]
pub type RgaReqMsch1PwrdisctargpwrstallR = crate::BitReader<RgaReqMsch1Pwrdisctargpwrstall>;
impl RgaReqMsch1PwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RgaReqMsch1Pwrdisctargpwrstall {
        match self.bits {
            false => RgaReqMsch1Pwrdisctargpwrstall::B0,
            true => RgaReqMsch1Pwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == RgaReqMsch1Pwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == RgaReqMsch1Pwrdisctargpwrstall::B1
    }
}
#[doc = "Field `RGA_REQ_MSCH1_PWRDISCTARGPWRSTALL` writer - noc_rga_req_msch1_rsp_err_stall bit control"]
pub type RgaReqMsch1PwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, RgaReqMsch1Pwrdisctargpwrstall>;
impl<'a, REG> RgaReqMsch1PwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(RgaReqMsch1Pwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(RgaReqMsch1Pwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_usb3_req_msch0_rsp_err_stall bit\n\ncontrol\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usb3ReqMsch0Pwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<Usb3ReqMsch0Pwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: Usb3ReqMsch0Pwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB3_REQ_MSCH0_PWRDISCTARGPWRSTALL` reader - noc_usb3_req_msch0_rsp_err_stall bit\n\ncontrol"]
pub type Usb3ReqMsch0PwrdisctargpwrstallR = crate::BitReader<Usb3ReqMsch0Pwrdisctargpwrstall>;
impl Usb3ReqMsch0PwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usb3ReqMsch0Pwrdisctargpwrstall {
        match self.bits {
            false => Usb3ReqMsch0Pwrdisctargpwrstall::B0,
            true => Usb3ReqMsch0Pwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Usb3ReqMsch0Pwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Usb3ReqMsch0Pwrdisctargpwrstall::B1
    }
}
#[doc = "Field `USB3_REQ_MSCH0_PWRDISCTARGPWRSTALL` writer - noc_usb3_req_msch0_rsp_err_stall bit\n\ncontrol"]
pub type Usb3ReqMsch0PwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, Usb3ReqMsch0Pwrdisctargpwrstall>;
impl<'a, REG> Usb3ReqMsch0PwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Usb3ReqMsch0Pwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Usb3ReqMsch0Pwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_usb3_req_msch1_rsp_err_stall bit\n\ncontrol\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usb3ReqMsch1Pwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<Usb3ReqMsch1Pwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: Usb3ReqMsch1Pwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB3_REQ_MSCH1_PWRDISCTARGPWRSTALL` reader - noc_usb3_req_msch1_rsp_err_stall bit\n\ncontrol"]
pub type Usb3ReqMsch1PwrdisctargpwrstallR = crate::BitReader<Usb3ReqMsch1Pwrdisctargpwrstall>;
impl Usb3ReqMsch1PwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usb3ReqMsch1Pwrdisctargpwrstall {
        match self.bits {
            false => Usb3ReqMsch1Pwrdisctargpwrstall::B0,
            true => Usb3ReqMsch1Pwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Usb3ReqMsch1Pwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Usb3ReqMsch1Pwrdisctargpwrstall::B1
    }
}
#[doc = "Field `USB3_REQ_MSCH1_PWRDISCTARGPWRSTALL` writer - noc_usb3_req_msch1_rsp_err_stall bit\n\ncontrol"]
pub type Usb3ReqMsch1PwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, Usb3ReqMsch1Pwrdisctargpwrstall>;
impl<'a, REG> Usb3ReqMsch1PwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Usb3ReqMsch1Pwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Usb3ReqMsch1Pwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_vcodec_req_msch0_rsp_err_stall bit\n\ncontrol\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VcodecReqMsch0Pwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<VcodecReqMsch0Pwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: VcodecReqMsch0Pwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCODEC_REQ_MSCH0_PWRDISCTARGPWRSTALL` reader - noc_vcodec_req_msch0_rsp_err_stall bit\n\ncontrol"]
pub type VcodecReqMsch0PwrdisctargpwrstallR = crate::BitReader<VcodecReqMsch0Pwrdisctargpwrstall>;
impl VcodecReqMsch0PwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VcodecReqMsch0Pwrdisctargpwrstall {
        match self.bits {
            false => VcodecReqMsch0Pwrdisctargpwrstall::B0,
            true => VcodecReqMsch0Pwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VcodecReqMsch0Pwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VcodecReqMsch0Pwrdisctargpwrstall::B1
    }
}
#[doc = "Field `VCODEC_REQ_MSCH0_PWRDISCTARGPWRSTALL` writer - noc_vcodec_req_msch0_rsp_err_stall bit\n\ncontrol"]
pub type VcodecReqMsch0PwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, VcodecReqMsch0Pwrdisctargpwrstall>;
impl<'a, REG> VcodecReqMsch0PwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VcodecReqMsch0Pwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VcodecReqMsch0Pwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_vcodec_req_msch1_rsp_err_stall bit\n\ncontrol\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VcodecReqMsch1Pwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<VcodecReqMsch1Pwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: VcodecReqMsch1Pwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCODEC_REQ_MSCH1_PWRDISCTARGPWRSTALL` reader - noc_vcodec_req_msch1_rsp_err_stall bit\n\ncontrol"]
pub type VcodecReqMsch1PwrdisctargpwrstallR = crate::BitReader<VcodecReqMsch1Pwrdisctargpwrstall>;
impl VcodecReqMsch1PwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VcodecReqMsch1Pwrdisctargpwrstall {
        match self.bits {
            false => VcodecReqMsch1Pwrdisctargpwrstall::B0,
            true => VcodecReqMsch1Pwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VcodecReqMsch1Pwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VcodecReqMsch1Pwrdisctargpwrstall::B1
    }
}
#[doc = "Field `VCODEC_REQ_MSCH1_PWRDISCTARGPWRSTALL` writer - noc_vcodec_req_msch1_rsp_err_stall bit\n\ncontrol"]
pub type VcodecReqMsch1PwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, VcodecReqMsch1Pwrdisctargpwrstall>;
impl<'a, REG> VcodecReqMsch1PwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VcodecReqMsch1Pwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VcodecReqMsch1Pwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_vdu_req_msch0_rsp_err_stall bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VduReqMsch0Pwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<VduReqMsch0Pwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: VduReqMsch0Pwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDU_REQ_MSCH0_PWRDISCTARGPWRSTALL` reader - noc_vdu_req_msch0_rsp_err_stall bit control"]
pub type VduReqMsch0PwrdisctargpwrstallR = crate::BitReader<VduReqMsch0Pwrdisctargpwrstall>;
impl VduReqMsch0PwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VduReqMsch0Pwrdisctargpwrstall {
        match self.bits {
            false => VduReqMsch0Pwrdisctargpwrstall::B0,
            true => VduReqMsch0Pwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VduReqMsch0Pwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VduReqMsch0Pwrdisctargpwrstall::B1
    }
}
#[doc = "Field `VDU_REQ_MSCH0_PWRDISCTARGPWRSTALL` writer - noc_vdu_req_msch0_rsp_err_stall bit control"]
pub type VduReqMsch0PwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, VduReqMsch0Pwrdisctargpwrstall>;
impl<'a, REG> VduReqMsch0PwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VduReqMsch0Pwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VduReqMsch0Pwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_vdu_req_msch1_rsp_err_stall bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VduReqMsch1Pwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<VduReqMsch1Pwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: VduReqMsch1Pwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDU_REQ_MSCH1_PWRDISCTARGPWRSTALL` reader - noc_vdu_req_msch1_rsp_err_stall bit control"]
pub type VduReqMsch1PwrdisctargpwrstallR = crate::BitReader<VduReqMsch1Pwrdisctargpwrstall>;
impl VduReqMsch1PwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VduReqMsch1Pwrdisctargpwrstall {
        match self.bits {
            false => VduReqMsch1Pwrdisctargpwrstall::B0,
            true => VduReqMsch1Pwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VduReqMsch1Pwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VduReqMsch1Pwrdisctargpwrstall::B1
    }
}
#[doc = "Field `VDU_REQ_MSCH1_PWRDISCTARGPWRSTALL` writer - noc_vdu_req_msch1_rsp_err_stall bit control"]
pub type VduReqMsch1PwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, VduReqMsch1Pwrdisctargpwrstall>;
impl<'a, REG> VduReqMsch1PwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VduReqMsch1Pwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VduReqMsch1Pwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_vio0_req_msch0_rsp_err_stall bit\n\ncontrol\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vio0ReqMsch0Pwrdisctargpwrstall {
    #[doc = "0: error response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<Vio0ReqMsch0Pwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: Vio0ReqMsch0Pwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VIO0_REQ_MSCH0_PWRDISCTARGPWRSTALL` reader - noc_vio0_req_msch0_rsp_err_stall bit\n\ncontrol"]
pub type Vio0ReqMsch0PwrdisctargpwrstallR = crate::BitReader<Vio0ReqMsch0Pwrdisctargpwrstall>;
impl Vio0ReqMsch0PwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vio0ReqMsch0Pwrdisctargpwrstall {
        match self.bits {
            false => Vio0ReqMsch0Pwrdisctargpwrstall::B0,
            true => Vio0ReqMsch0Pwrdisctargpwrstall::B1,
        }
    }
    #[doc = "error response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Vio0ReqMsch0Pwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Vio0ReqMsch0Pwrdisctargpwrstall::B1
    }
}
#[doc = "Field `VIO0_REQ_MSCH0_PWRDISCTARGPWRSTALL` writer - noc_vio0_req_msch0_rsp_err_stall bit\n\ncontrol"]
pub type Vio0ReqMsch0PwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, Vio0ReqMsch0Pwrdisctargpwrstall>;
impl<'a, REG> Vio0ReqMsch0PwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Vio0ReqMsch0Pwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Vio0ReqMsch0Pwrdisctargpwrstall::B1)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - Fbit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - Fbit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - noc_perilp_fwd_center_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    pub fn perilp_fwd_center_pwrdisctargpwrstall(&self) -> PerilpFwdCenterPwrdisctargpwrstallR {
        PerilpFwdCenterPwrdisctargpwrstallR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - noc_perilp_fwd_pmu_rsp_err_stall bit control"]
    #[inline(always)]
    pub fn perilp_fwd_pmu_pwrdisctargpwrstall(&self) -> PerilpFwdPmuPwrdisctargpwrstallR {
        PerilpFwdPmuPwrdisctargpwrstallR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - noc_perilp_req_msch0_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    pub fn perilp_req_msch0_pwrdisctargpwrstall(&self) -> PerilpReqMsch0PwrdisctargpwrstallR {
        PerilpReqMsch0PwrdisctargpwrstallR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - noc_perilp_req_msch1_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    pub fn perilp_req_msch1_pwrdisctargpwrstall(&self) -> PerilpReqMsch1PwrdisctargpwrstallR {
        PerilpReqMsch1PwrdisctargpwrstallR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - noc_perilpsrv_fwd_cm0_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    pub fn perilpsrv_fwd_cm0_pwrdisctargpwrstall(&self) -> PerilpsrvFwdCm0PwrdisctargpwrstallR {
        PerilpsrvFwdCm0PwrdisctargpwrstallR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - noc_gmac_fwd_perihp_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    pub fn gmac_fwd_perihp_pwrdisctargpwrstall(&self) -> GmacFwdPerihpPwrdisctargpwrstallR {
        GmacFwdPerihpPwrdisctargpwrstallR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - noc_pmu_fwd_perilp_rsp_err_stall bit control"]
    #[inline(always)]
    pub fn sdioaudio_fwd_perilp_pwrdisctargpwrstall(
        &self,
    ) -> SdioaudioFwdPerilpPwrdisctargpwrstallR {
        SdioaudioFwdPerilpPwrdisctargpwrstallR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - noc_rga_req_msch0_rsp_err_stall bit control"]
    #[inline(always)]
    pub fn rga_req_msch0_pwrdisctargpwrstall(&self) -> RgaReqMsch0PwrdisctargpwrstallR {
        RgaReqMsch0PwrdisctargpwrstallR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - noc_rga_req_msch1_rsp_err_stall bit control"]
    #[inline(always)]
    pub fn rga_req_msch1_pwrdisctargpwrstall(&self) -> RgaReqMsch1PwrdisctargpwrstallR {
        RgaReqMsch1PwrdisctargpwrstallR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - noc_usb3_req_msch0_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    pub fn usb3_req_msch0_pwrdisctargpwrstall(&self) -> Usb3ReqMsch0PwrdisctargpwrstallR {
        Usb3ReqMsch0PwrdisctargpwrstallR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - noc_usb3_req_msch1_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    pub fn usb3_req_msch1_pwrdisctargpwrstall(&self) -> Usb3ReqMsch1PwrdisctargpwrstallR {
        Usb3ReqMsch1PwrdisctargpwrstallR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - noc_vcodec_req_msch0_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    pub fn vcodec_req_msch0_pwrdisctargpwrstall(&self) -> VcodecReqMsch0PwrdisctargpwrstallR {
        VcodecReqMsch0PwrdisctargpwrstallR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - noc_vcodec_req_msch1_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    pub fn vcodec_req_msch1_pwrdisctargpwrstall(&self) -> VcodecReqMsch1PwrdisctargpwrstallR {
        VcodecReqMsch1PwrdisctargpwrstallR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - noc_vdu_req_msch0_rsp_err_stall bit control"]
    #[inline(always)]
    pub fn vdu_req_msch0_pwrdisctargpwrstall(&self) -> VduReqMsch0PwrdisctargpwrstallR {
        VduReqMsch0PwrdisctargpwrstallR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - noc_vdu_req_msch1_rsp_err_stall bit control"]
    #[inline(always)]
    pub fn vdu_req_msch1_pwrdisctargpwrstall(&self) -> VduReqMsch1PwrdisctargpwrstallR {
        VduReqMsch1PwrdisctargpwrstallR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - noc_vio0_req_msch0_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    pub fn vio0_req_msch0_pwrdisctargpwrstall(&self) -> Vio0ReqMsch0PwrdisctargpwrstallR {
        Vio0ReqMsch0PwrdisctargpwrstallR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Fbit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - noc_perilp_fwd_center_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    #[must_use]
    pub fn perilp_fwd_center_pwrdisctargpwrstall(
        &mut self,
    ) -> PerilpFwdCenterPwrdisctargpwrstallW<GrfSocCon2Spec> {
        PerilpFwdCenterPwrdisctargpwrstallW::new(self, 0)
    }
    #[doc = "Bit 1 - noc_perilp_fwd_pmu_rsp_err_stall bit control"]
    #[inline(always)]
    #[must_use]
    pub fn perilp_fwd_pmu_pwrdisctargpwrstall(
        &mut self,
    ) -> PerilpFwdPmuPwrdisctargpwrstallW<GrfSocCon2Spec> {
        PerilpFwdPmuPwrdisctargpwrstallW::new(self, 1)
    }
    #[doc = "Bit 2 - noc_perilp_req_msch0_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    #[must_use]
    pub fn perilp_req_msch0_pwrdisctargpwrstall(
        &mut self,
    ) -> PerilpReqMsch0PwrdisctargpwrstallW<GrfSocCon2Spec> {
        PerilpReqMsch0PwrdisctargpwrstallW::new(self, 2)
    }
    #[doc = "Bit 3 - noc_perilp_req_msch1_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    #[must_use]
    pub fn perilp_req_msch1_pwrdisctargpwrstall(
        &mut self,
    ) -> PerilpReqMsch1PwrdisctargpwrstallW<GrfSocCon2Spec> {
        PerilpReqMsch1PwrdisctargpwrstallW::new(self, 3)
    }
    #[doc = "Bit 4 - noc_perilpsrv_fwd_cm0_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    #[must_use]
    pub fn perilpsrv_fwd_cm0_pwrdisctargpwrstall(
        &mut self,
    ) -> PerilpsrvFwdCm0PwrdisctargpwrstallW<GrfSocCon2Spec> {
        PerilpsrvFwdCm0PwrdisctargpwrstallW::new(self, 4)
    }
    #[doc = "Bit 5 - noc_gmac_fwd_perihp_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    #[must_use]
    pub fn gmac_fwd_perihp_pwrdisctargpwrstall(
        &mut self,
    ) -> GmacFwdPerihpPwrdisctargpwrstallW<GrfSocCon2Spec> {
        GmacFwdPerihpPwrdisctargpwrstallW::new(self, 5)
    }
    #[doc = "Bit 6 - noc_pmu_fwd_perilp_rsp_err_stall bit control"]
    #[inline(always)]
    #[must_use]
    pub fn sdioaudio_fwd_perilp_pwrdisctargpwrstall(
        &mut self,
    ) -> SdioaudioFwdPerilpPwrdisctargpwrstallW<GrfSocCon2Spec> {
        SdioaudioFwdPerilpPwrdisctargpwrstallW::new(self, 6)
    }
    #[doc = "Bit 7 - noc_rga_req_msch0_rsp_err_stall bit control"]
    #[inline(always)]
    #[must_use]
    pub fn rga_req_msch0_pwrdisctargpwrstall(
        &mut self,
    ) -> RgaReqMsch0PwrdisctargpwrstallW<GrfSocCon2Spec> {
        RgaReqMsch0PwrdisctargpwrstallW::new(self, 7)
    }
    #[doc = "Bit 8 - noc_rga_req_msch1_rsp_err_stall bit control"]
    #[inline(always)]
    #[must_use]
    pub fn rga_req_msch1_pwrdisctargpwrstall(
        &mut self,
    ) -> RgaReqMsch1PwrdisctargpwrstallW<GrfSocCon2Spec> {
        RgaReqMsch1PwrdisctargpwrstallW::new(self, 8)
    }
    #[doc = "Bit 9 - noc_usb3_req_msch0_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    #[must_use]
    pub fn usb3_req_msch0_pwrdisctargpwrstall(
        &mut self,
    ) -> Usb3ReqMsch0PwrdisctargpwrstallW<GrfSocCon2Spec> {
        Usb3ReqMsch0PwrdisctargpwrstallW::new(self, 9)
    }
    #[doc = "Bit 10 - noc_usb3_req_msch1_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    #[must_use]
    pub fn usb3_req_msch1_pwrdisctargpwrstall(
        &mut self,
    ) -> Usb3ReqMsch1PwrdisctargpwrstallW<GrfSocCon2Spec> {
        Usb3ReqMsch1PwrdisctargpwrstallW::new(self, 10)
    }
    #[doc = "Bit 11 - noc_vcodec_req_msch0_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    #[must_use]
    pub fn vcodec_req_msch0_pwrdisctargpwrstall(
        &mut self,
    ) -> VcodecReqMsch0PwrdisctargpwrstallW<GrfSocCon2Spec> {
        VcodecReqMsch0PwrdisctargpwrstallW::new(self, 11)
    }
    #[doc = "Bit 12 - noc_vcodec_req_msch1_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    #[must_use]
    pub fn vcodec_req_msch1_pwrdisctargpwrstall(
        &mut self,
    ) -> VcodecReqMsch1PwrdisctargpwrstallW<GrfSocCon2Spec> {
        VcodecReqMsch1PwrdisctargpwrstallW::new(self, 12)
    }
    #[doc = "Bit 13 - noc_vdu_req_msch0_rsp_err_stall bit control"]
    #[inline(always)]
    #[must_use]
    pub fn vdu_req_msch0_pwrdisctargpwrstall(
        &mut self,
    ) -> VduReqMsch0PwrdisctargpwrstallW<GrfSocCon2Spec> {
        VduReqMsch0PwrdisctargpwrstallW::new(self, 13)
    }
    #[doc = "Bit 14 - noc_vdu_req_msch1_rsp_err_stall bit control"]
    #[inline(always)]
    #[must_use]
    pub fn vdu_req_msch1_pwrdisctargpwrstall(
        &mut self,
    ) -> VduReqMsch1PwrdisctargpwrstallW<GrfSocCon2Spec> {
        VduReqMsch1PwrdisctargpwrstallW::new(self, 14)
    }
    #[doc = "Bit 15 - noc_vio0_req_msch0_rsp_err_stall bit\n\ncontrol"]
    #[inline(always)]
    #[must_use]
    pub fn vio0_req_msch0_pwrdisctargpwrstall(
        &mut self,
    ) -> Vio0ReqMsch0PwrdisctargpwrstallW<GrfSocCon2Spec> {
        Vio0ReqMsch0PwrdisctargpwrstallW::new(self, 15)
    }
    #[doc = "Bits 16:31 - Fbit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfSocCon2Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "SoC control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_soc_con2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_soc_con2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfSocCon2Spec;
impl crate::RegisterSpec for GrfSocCon2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_soc_con2::R`](R) reader structure"]
impl crate::Readable for GrfSocCon2Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_soc_con2::W`](W) writer structure"]
impl crate::Writable for GrfSocCon2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_SOC_CON2 to value 0"]
impl crate::Resettable for GrfSocCon2Spec {
    const RESET_VALUE: u32 = 0;
}
