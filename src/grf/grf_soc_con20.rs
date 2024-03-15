#[doc = "Register `GRF_SOC_CON20` reader"]
pub type R = crate::R<GrfSocCon20Spec>;
#[doc = "Register `GRF_SOC_CON20` writer"]
pub type W = crate::W<GrfSocCon20Spec>;
#[doc = "dsi0 vol select bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dsi0LcdcSel {
    #[doc = "0: vop little"]
    B0 = 0,
    #[doc = "1: vop little"]
    B1 = 1,
}
impl From<Dsi0LcdcSel> for bool {
    #[inline(always)]
    fn from(variant: Dsi0LcdcSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSI0_LCDC_SEL` reader - dsi0 vol select bit"]
pub type Dsi0LcdcSelR = crate::BitReader<Dsi0LcdcSel>;
impl Dsi0LcdcSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dsi0LcdcSel {
        match self.bits {
            false => Dsi0LcdcSel::B0,
            true => Dsi0LcdcSel::B1,
        }
    }
    #[doc = "vop little"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dsi0LcdcSel::B0
    }
    #[doc = "vop little"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dsi0LcdcSel::B1
    }
}
#[doc = "Field `DSI0_LCDC_SEL` writer - dsi0 vol select bit"]
pub type Dsi0LcdcSelW<'a, REG> = crate::BitWriter<'a, REG, Dsi0LcdcSel>;
impl<'a, REG> Dsi0LcdcSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "vop little"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dsi0LcdcSel::B0)
    }
    #[doc = "vop little"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dsi0LcdcSel::B1)
    }
}
#[doc = "Field `DSI1_DPICOLORM` reader - dsi1 dpicolorm bit control"]
pub type Dsi1DpicolormR = crate::BitReader;
#[doc = "Field `DSI1_DPICOLORM` writer - dsi1 dpicolorm bit control"]
pub type Dsi1DpicolormW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSI1_DPISHUTDN` reader - dsi1 dpishutdn bit control"]
pub type Dsi1DpishutdnR = crate::BitReader;
#[doc = "Field `DSI1_DPISHUTDN` writer - dsi1 dpishutdn bit control"]
pub type Dsi1DpishutdnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSI1_DPIUPDATECFG` reader - dsi1 dpiupdatecfg bit control"]
pub type Dsi1DpiupdatecfgR = crate::BitReader;
#[doc = "Field `DSI1_DPIUPDATECFG` writer - dsi1 dpiupdatecfg bit control"]
pub type Dsi1DpiupdatecfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "dsi1 lcdc select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dsi1LcdcSel {
    #[doc = "0: vop little"]
    B0 = 0,
    #[doc = "1: vop little"]
    B1 = 1,
}
impl From<Dsi1LcdcSel> for bool {
    #[inline(always)]
    fn from(variant: Dsi1LcdcSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSI1_LCDC_SEL` reader - dsi1 lcdc select"]
pub type Dsi1LcdcSelR = crate::BitReader<Dsi1LcdcSel>;
impl Dsi1LcdcSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dsi1LcdcSel {
        match self.bits {
            false => Dsi1LcdcSel::B0,
            true => Dsi1LcdcSel::B1,
        }
    }
    #[doc = "vop little"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dsi1LcdcSel::B0
    }
    #[doc = "vop little"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dsi1LcdcSel::B1
    }
}
#[doc = "Field `DSI1_LCDC_SEL` writer - dsi1 lcdc select"]
pub type Dsi1LcdcSelW<'a, REG> = crate::BitWriter<'a, REG, Dsi1LcdcSel>;
impl<'a, REG> Dsi1LcdcSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "vop little"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dsi1LcdcSel::B0)
    }
    #[doc = "vop little"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dsi1LcdcSel::B1)
    }
}
#[doc = "edp lcdc select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdpLcdcSel {
    #[doc = "0: vop little"]
    B0 = 0,
    #[doc = "1: vop little"]
    B1 = 1,
}
impl From<EdpLcdcSel> for bool {
    #[inline(always)]
    fn from(variant: EdpLcdcSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDP_LCDC_SEL` reader - edp lcdc select"]
pub type EdpLcdcSelR = crate::BitReader<EdpLcdcSel>;
impl EdpLcdcSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EdpLcdcSel {
        match self.bits {
            false => EdpLcdcSel::B0,
            true => EdpLcdcSel::B1,
        }
    }
    #[doc = "vop little"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == EdpLcdcSel::B0
    }
    #[doc = "vop little"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == EdpLcdcSel::B1
    }
}
#[doc = "Field `EDP_LCDC_SEL` writer - edp lcdc select"]
pub type EdpLcdcSelW<'a, REG> = crate::BitWriter<'a, REG, EdpLcdcSel>;
impl<'a, REG> EdpLcdcSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "vop little"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(EdpLcdcSel::B0)
    }
    #[doc = "vop little"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(EdpLcdcSel::B1)
    }
}
#[doc = "hdmi lcdc select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HdmiLcdcSel {
    #[doc = "0: vop little"]
    B0 = 0,
    #[doc = "1: vop little"]
    B1 = 1,
}
impl From<HdmiLcdcSel> for bool {
    #[inline(always)]
    fn from(variant: HdmiLcdcSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDMI_LCDC_SEL` reader - hdmi lcdc select"]
pub type HdmiLcdcSelR = crate::BitReader<HdmiLcdcSel>;
impl HdmiLcdcSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HdmiLcdcSel {
        match self.bits {
            false => HdmiLcdcSel::B0,
            true => HdmiLcdcSel::B1,
        }
    }
    #[doc = "vop little"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HdmiLcdcSel::B0
    }
    #[doc = "vop little"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HdmiLcdcSel::B1
    }
}
#[doc = "Field `HDMI_LCDC_SEL` writer - hdmi lcdc select"]
pub type HdmiLcdcSelW<'a, REG> = crate::BitWriter<'a, REG, HdmiLcdcSel>;
impl<'a, REG> HdmiLcdcSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "vop little"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HdmiLcdcSel::B0)
    }
    #[doc = "vop little"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HdmiLcdcSel::B1)
    }
}
#[doc = "vop finish select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VopFinishSel {
    #[doc = "0: vop little"]
    B0 = 0,
    #[doc = "1: vop little"]
    B1 = 1,
}
impl From<VopFinishSel> for bool {
    #[inline(always)]
    fn from(variant: VopFinishSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VOP_FINISH_SEL` reader - vop finish select"]
pub type VopFinishSelR = crate::BitReader<VopFinishSel>;
impl VopFinishSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VopFinishSel {
        match self.bits {
            false => VopFinishSel::B0,
            true => VopFinishSel::B1,
        }
    }
    #[doc = "vop little"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VopFinishSel::B0
    }
    #[doc = "vop little"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VopFinishSel::B1
    }
}
#[doc = "Field `VOP_FINISH_SEL` writer - vop finish select"]
pub type VopFinishSelW<'a, REG> = crate::BitWriter<'a, REG, VopFinishSel>;
impl<'a, REG> VopFinishSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "vop little"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VopFinishSel::B0)
    }
    #[doc = "vop little"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VopFinishSel::B1)
    }
}
#[doc = "edp video bist enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdpVideoBistEn {
    #[doc = "1: disable"]
    B1 = 1,
    #[doc = "0: disable"]
    B0 = 0,
}
impl From<EdpVideoBistEn> for bool {
    #[inline(always)]
    fn from(variant: EdpVideoBistEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDP_VIDEO_BIST_EN` reader - edp video bist enable"]
pub type EdpVideoBistEnR = crate::BitReader<EdpVideoBistEn>;
impl EdpVideoBistEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EdpVideoBistEn {
        match self.bits {
            true => EdpVideoBistEn::B1,
            false => EdpVideoBistEn::B0,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == EdpVideoBistEn::B1
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == EdpVideoBistEn::B0
    }
}
#[doc = "Field `EDP_VIDEO_BIST_EN` writer - edp video bist enable"]
pub type EdpVideoBistEnW<'a, REG> = crate::BitWriter<'a, REG, EdpVideoBistEn>;
impl<'a, REG> EdpVideoBistEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(EdpVideoBistEn::B1)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(EdpVideoBistEn::B0)
    }
}
#[doc = "pclkin dvp clock select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PclkinDvpRevSel {
    #[doc = "0: invert phase"]
    B0 = 0,
    #[doc = "1: invert phase"]
    B1 = 1,
}
impl From<PclkinDvpRevSel> for bool {
    #[inline(always)]
    fn from(variant: PclkinDvpRevSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCLKIN_DVP_REV_SEL` reader - pclkin dvp clock select"]
pub type PclkinDvpRevSelR = crate::BitReader<PclkinDvpRevSel>;
impl PclkinDvpRevSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PclkinDvpRevSel {
        match self.bits {
            false => PclkinDvpRevSel::B0,
            true => PclkinDvpRevSel::B1,
        }
    }
    #[doc = "invert phase"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PclkinDvpRevSel::B0
    }
    #[doc = "invert phase"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PclkinDvpRevSel::B1
    }
}
#[doc = "Field `PCLKIN_DVP_REV_SEL` writer - pclkin dvp clock select"]
pub type PclkinDvpRevSelW<'a, REG> = crate::BitWriter<'a, REG, PclkinDvpRevSel>;
impl<'a, REG> PclkinDvpRevSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "invert phase"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PclkinDvpRevSel::B0)
    }
    #[doc = "invert phase"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PclkinDvpRevSel::B1)
    }
}
#[doc = "vop select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GrfConRgbLcdcSel {
    #[doc = "0: vop little"]
    B0 = 0,
    #[doc = "1: vop little"]
    B1 = 1,
}
impl From<GrfConRgbLcdcSel> for bool {
    #[inline(always)]
    fn from(variant: GrfConRgbLcdcSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GRF_CON_RGB_LCDC_SEL` reader - vop select"]
pub type GrfConRgbLcdcSelR = crate::BitReader<GrfConRgbLcdcSel>;
impl GrfConRgbLcdcSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GrfConRgbLcdcSel {
        match self.bits {
            false => GrfConRgbLcdcSel::B0,
            true => GrfConRgbLcdcSel::B1,
        }
    }
    #[doc = "vop little"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == GrfConRgbLcdcSel::B0
    }
    #[doc = "vop little"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == GrfConRgbLcdcSel::B1
    }
}
#[doc = "Field `GRF_CON_RGB_LCDC_SEL` writer - vop select"]
pub type GrfConRgbLcdcSelW<'a, REG> = crate::BitWriter<'a, REG, GrfConRgbLcdcSel>;
impl<'a, REG> GrfConRgbLcdcSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "vop little"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(GrfConRgbLcdcSel::B0)
    }
    #[doc = "vop little"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(GrfConRgbLcdcSel::B1)
    }
}
#[doc = "dclk phase selct\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GrfVopRgbDclkRevSel {
    #[doc = "0: 180 degree"]
    B0 = 0,
    #[doc = "1: 180 degree"]
    B1 = 1,
}
impl From<GrfVopRgbDclkRevSel> for bool {
    #[inline(always)]
    fn from(variant: GrfVopRgbDclkRevSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GRF_VOP_RGB_DCLK_REV_SEL` reader - dclk phase selct"]
pub type GrfVopRgbDclkRevSelR = crate::BitReader<GrfVopRgbDclkRevSel>;
impl GrfVopRgbDclkRevSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GrfVopRgbDclkRevSel {
        match self.bits {
            false => GrfVopRgbDclkRevSel::B0,
            true => GrfVopRgbDclkRevSel::B1,
        }
    }
    #[doc = "180 degree"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == GrfVopRgbDclkRevSel::B0
    }
    #[doc = "180 degree"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == GrfVopRgbDclkRevSel::B1
    }
}
#[doc = "Field `GRF_VOP_RGB_DCLK_REV_SEL` writer - dclk phase selct"]
pub type GrfVopRgbDclkRevSelW<'a, REG> = crate::BitWriter<'a, REG, GrfVopRgbDclkRevSel>;
impl<'a, REG> GrfVopRgbDclkRevSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "180 degree"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(GrfVopRgbDclkRevSel::B0)
    }
    #[doc = "180 degree"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(GrfVopRgbDclkRevSel::B1)
    }
}
#[doc = "Field `HDCP_I2C_FORCE_SCL` reader - hdcp_i2c_force_scl control"]
pub type HdcpI2cForceSclR = crate::BitReader;
#[doc = "Field `HDCP_I2C_FORCE_SCL` writer - hdcp_i2c_force_scl control"]
pub type HdcpI2cForceSclW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDCP_I2C_FORCE_SDA` reader - hdcp_i2c_force_sda bit control"]
pub type HdcpI2cForceSdaR = crate::BitReader;
#[doc = "Field `HDCP_I2C_FORCE_SDA` writer - hdcp_i2c_force_sda bit control"]
pub type HdcpI2cForceSdaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - dsi0 vol select bit"]
    #[inline(always)]
    pub fn dsi0_lcdc_sel(&self) -> Dsi0LcdcSelR {
        Dsi0LcdcSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - dsi1 dpicolorm bit control"]
    #[inline(always)]
    pub fn dsi1_dpicolorm(&self) -> Dsi1DpicolormR {
        Dsi1DpicolormR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - dsi1 dpishutdn bit control"]
    #[inline(always)]
    pub fn dsi1_dpishutdn(&self) -> Dsi1DpishutdnR {
        Dsi1DpishutdnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - dsi1 dpiupdatecfg bit control"]
    #[inline(always)]
    pub fn dsi1_dpiupdatecfg(&self) -> Dsi1DpiupdatecfgR {
        Dsi1DpiupdatecfgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - dsi1 lcdc select"]
    #[inline(always)]
    pub fn dsi1_lcdc_sel(&self) -> Dsi1LcdcSelR {
        Dsi1LcdcSelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - edp lcdc select"]
    #[inline(always)]
    pub fn edp_lcdc_sel(&self) -> EdpLcdcSelR {
        EdpLcdcSelR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - hdmi lcdc select"]
    #[inline(always)]
    pub fn hdmi_lcdc_sel(&self) -> HdmiLcdcSelR {
        HdmiLcdcSelR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - vop finish select"]
    #[inline(always)]
    pub fn vop_finish_sel(&self) -> VopFinishSelR {
        VopFinishSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - edp video bist enable"]
    #[inline(always)]
    pub fn edp_video_bist_en(&self) -> EdpVideoBistEnR {
        EdpVideoBistEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - pclkin dvp clock select"]
    #[inline(always)]
    pub fn pclkin_dvp_rev_sel(&self) -> PclkinDvpRevSelR {
        PclkinDvpRevSelR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - vop select"]
    #[inline(always)]
    pub fn grf_con_rgb_lcdc_sel(&self) -> GrfConRgbLcdcSelR {
        GrfConRgbLcdcSelR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - dclk phase selct"]
    #[inline(always)]
    pub fn grf_vop_rgb_dclk_rev_sel(&self) -> GrfVopRgbDclkRevSelR {
        GrfVopRgbDclkRevSelR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - hdcp_i2c_force_scl control"]
    #[inline(always)]
    pub fn hdcp_i2c_force_scl(&self) -> HdcpI2cForceSclR {
        HdcpI2cForceSclR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - hdcp_i2c_force_sda bit control"]
    #[inline(always)]
    pub fn hdcp_i2c_force_sda(&self) -> HdcpI2cForceSdaR {
        HdcpI2cForceSdaR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - dsi0 vol select bit"]
    #[inline(always)]
    #[must_use]
    pub fn dsi0_lcdc_sel(&mut self) -> Dsi0LcdcSelW<GrfSocCon20Spec> {
        Dsi0LcdcSelW::new(self, 0)
    }
    #[doc = "Bit 1 - dsi1 dpicolorm bit control"]
    #[inline(always)]
    #[must_use]
    pub fn dsi1_dpicolorm(&mut self) -> Dsi1DpicolormW<GrfSocCon20Spec> {
        Dsi1DpicolormW::new(self, 1)
    }
    #[doc = "Bit 2 - dsi1 dpishutdn bit control"]
    #[inline(always)]
    #[must_use]
    pub fn dsi1_dpishutdn(&mut self) -> Dsi1DpishutdnW<GrfSocCon20Spec> {
        Dsi1DpishutdnW::new(self, 2)
    }
    #[doc = "Bit 3 - dsi1 dpiupdatecfg bit control"]
    #[inline(always)]
    #[must_use]
    pub fn dsi1_dpiupdatecfg(&mut self) -> Dsi1DpiupdatecfgW<GrfSocCon20Spec> {
        Dsi1DpiupdatecfgW::new(self, 3)
    }
    #[doc = "Bit 4 - dsi1 lcdc select"]
    #[inline(always)]
    #[must_use]
    pub fn dsi1_lcdc_sel(&mut self) -> Dsi1LcdcSelW<GrfSocCon20Spec> {
        Dsi1LcdcSelW::new(self, 4)
    }
    #[doc = "Bit 5 - edp lcdc select"]
    #[inline(always)]
    #[must_use]
    pub fn edp_lcdc_sel(&mut self) -> EdpLcdcSelW<GrfSocCon20Spec> {
        EdpLcdcSelW::new(self, 5)
    }
    #[doc = "Bit 6 - hdmi lcdc select"]
    #[inline(always)]
    #[must_use]
    pub fn hdmi_lcdc_sel(&mut self) -> HdmiLcdcSelW<GrfSocCon20Spec> {
        HdmiLcdcSelW::new(self, 6)
    }
    #[doc = "Bit 7 - vop finish select"]
    #[inline(always)]
    #[must_use]
    pub fn vop_finish_sel(&mut self) -> VopFinishSelW<GrfSocCon20Spec> {
        VopFinishSelW::new(self, 7)
    }
    #[doc = "Bit 8 - edp video bist enable"]
    #[inline(always)]
    #[must_use]
    pub fn edp_video_bist_en(&mut self) -> EdpVideoBistEnW<GrfSocCon20Spec> {
        EdpVideoBistEnW::new(self, 8)
    }
    #[doc = "Bit 9 - pclkin dvp clock select"]
    #[inline(always)]
    #[must_use]
    pub fn pclkin_dvp_rev_sel(&mut self) -> PclkinDvpRevSelW<GrfSocCon20Spec> {
        PclkinDvpRevSelW::new(self, 9)
    }
    #[doc = "Bit 11 - vop select"]
    #[inline(always)]
    #[must_use]
    pub fn grf_con_rgb_lcdc_sel(&mut self) -> GrfConRgbLcdcSelW<GrfSocCon20Spec> {
        GrfConRgbLcdcSelW::new(self, 11)
    }
    #[doc = "Bit 12 - dclk phase selct"]
    #[inline(always)]
    #[must_use]
    pub fn grf_vop_rgb_dclk_rev_sel(&mut self) -> GrfVopRgbDclkRevSelW<GrfSocCon20Spec> {
        GrfVopRgbDclkRevSelW::new(self, 12)
    }
    #[doc = "Bit 13 - hdcp_i2c_force_scl control"]
    #[inline(always)]
    #[must_use]
    pub fn hdcp_i2c_force_scl(&mut self) -> HdcpI2cForceSclW<GrfSocCon20Spec> {
        HdcpI2cForceSclW::new(self, 13)
    }
    #[doc = "Bit 14 - hdcp_i2c_force_sda bit control"]
    #[inline(always)]
    #[must_use]
    pub fn hdcp_i2c_force_sda(&mut self) -> HdcpI2cForceSdaW<GrfSocCon20Spec> {
        HdcpI2cForceSdaW::new(self, 14)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfSocCon20Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "SoC control register 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_soc_con20::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_soc_con20::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfSocCon20Spec;
impl crate::RegisterSpec for GrfSocCon20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_soc_con20::R`](R) reader structure"]
impl crate::Readable for GrfSocCon20Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_soc_con20::W`](W) writer structure"]
impl crate::Writable for GrfSocCon20Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_SOC_CON20 to value 0x0249"]
impl crate::Resettable for GrfSocCon20Spec {
    const RESET_VALUE: u32 = 0x0249;
}
