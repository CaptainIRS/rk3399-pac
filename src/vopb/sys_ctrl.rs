#[doc = "Register `SYS_CTRL` reader"]
pub type R = crate::R<SysCtrlSpec>;
#[doc = "Register `SYS_CTRL` writer"]
pub type W = crate::W<SysCtrlSpec>;
#[doc = "iep direct path enable signal\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirectPathEn {
    #[doc = "0: disable iep direct path"]
    B0 = 0,
    #[doc = "1: enable iep direct path"]
    B1 = 1,
}
impl From<DirectPathEn> for bool {
    #[inline(always)]
    fn from(variant: DirectPathEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRECT_PATH_EN` reader - iep direct path enable signal"]
pub type DirectPathEnR = crate::BitReader<DirectPathEn>;
impl DirectPathEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DirectPathEn {
        match self.bits {
            false => DirectPathEn::B0,
            true => DirectPathEn::B1,
        }
    }
    #[doc = "disable iep direct path"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DirectPathEn::B0
    }
    #[doc = "enable iep direct path"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DirectPathEn::B1
    }
}
#[doc = "Field `DIRECT_PATH_EN` writer - iep direct path enable signal"]
pub type DirectPathEnW<'a, REG> = crate::BitWriter<'a, REG, DirectPathEn>;
impl<'a, REG> DirectPathEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable iep direct path"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DirectPathEn::B0)
    }
    #[doc = "enable iep direct path"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DirectPathEn::B1)
    }
}
#[doc = "direct path layer select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DirectPathLayerSel {
    #[doc = "0: select win0"]
    B00 = 0,
    #[doc = "1: select win1"]
    B01 = 1,
    #[doc = "2: select win2"]
    B10 = 2,
    #[doc = "3: select win3"]
    B11 = 3,
}
impl From<DirectPathLayerSel> for u8 {
    #[inline(always)]
    fn from(variant: DirectPathLayerSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DirectPathLayerSel {
    type Ux = u8;
}
#[doc = "Field `DIRECT_PATH_LAYER_SEL` reader - direct path layer select"]
pub type DirectPathLayerSelR = crate::FieldReader<DirectPathLayerSel>;
impl DirectPathLayerSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DirectPathLayerSel {
        match self.bits {
            0 => DirectPathLayerSel::B00,
            1 => DirectPathLayerSel::B01,
            2 => DirectPathLayerSel::B10,
            3 => DirectPathLayerSel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "select win0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == DirectPathLayerSel::B00
    }
    #[doc = "select win1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == DirectPathLayerSel::B01
    }
    #[doc = "select win2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == DirectPathLayerSel::B10
    }
    #[doc = "select win3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == DirectPathLayerSel::B11
    }
}
#[doc = "Field `DIRECT_PATH_LAYER_SEL` writer - direct path layer select"]
pub type DirectPathLayerSelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DirectPathLayerSel>;
impl<'a, REG> DirectPathLayerSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "select win0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(DirectPathLayerSel::B00)
    }
    #[doc = "select win1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(DirectPathLayerSel::B01)
    }
    #[doc = "select win2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(DirectPathLayerSel::B10)
    }
    #[doc = "select win3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(DirectPathLayerSel::B11)
    }
}
#[doc = "Field `EDPI_HALT_EN` reader - mipi flow ctrl enable"]
pub type EdpiHaltEnR = crate::BitReader;
#[doc = "Field `EDPI_HALT_EN` writer - mipi flow ctrl enable"]
pub type EdpiHaltEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDPI_WMS_MODE` reader - 1'b1: mipi command mode"]
pub type EdpiWmsModeR = crate::BitReader;
#[doc = "Field `EDPI_WMS_MODE` writer - 1'b1: mipi command mode"]
pub type EdpiWmsModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDPI_WMS_FS` reader - edpi wms mode ,frame st signal\n\nwrite '1': edpi_wms_mode frame start (when other register is\n\nconfig done)\n\nread : wms mode hold status"]
pub type EdpiWmsFsR = crate::BitReader;
#[doc = "Field `EDPI_WMS_FS` writer - edpi wms mode ,frame st signal\n\nwrite '1': edpi_wms_mode frame start (when other register is\n\nconfig done)\n\nread : wms mode hold status"]
pub type EdpiWmsFsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DpOutEn {
    #[doc = "0: gating output clk ,data and control signal"]
    B0 = 0,
    #[doc = "1: dp interface enable"]
    B1 = 1,
}
impl From<DpOutEn> for bool {
    #[inline(always)]
    fn from(variant: DpOutEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DP_OUT_EN` reader - "]
pub type DpOutEnR = crate::BitReader<DpOutEn>;
impl DpOutEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DpOutEn {
        match self.bits {
            false => DpOutEn::B0,
            true => DpOutEn::B1,
        }
    }
    #[doc = "gating output clk ,data and control signal"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DpOutEn::B0
    }
    #[doc = "dp interface enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DpOutEn::B1
    }
}
#[doc = "Field `DP_OUT_EN` writer - "]
pub type DpOutEnW<'a, REG> = crate::BitWriter<'a, REG, DpOutEn>;
impl<'a, REG> DpOutEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "gating output clk ,data and control signal"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DpOutEn::B0)
    }
    #[doc = "dp interface enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DpOutEn::B1)
    }
}
#[doc = "\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RgbOutEn {
    #[doc = "0: gating output clk ,data and control signal"]
    B0 = 0,
    #[doc = "1: rgb/lvds interface enable"]
    B1 = 1,
}
impl From<RgbOutEn> for bool {
    #[inline(always)]
    fn from(variant: RgbOutEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGB_OUT_EN` reader - "]
pub type RgbOutEnR = crate::BitReader<RgbOutEn>;
impl RgbOutEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RgbOutEn {
        match self.bits {
            false => RgbOutEn::B0,
            true => RgbOutEn::B1,
        }
    }
    #[doc = "gating output clk ,data and control signal"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == RgbOutEn::B0
    }
    #[doc = "rgb/lvds interface enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == RgbOutEn::B1
    }
}
#[doc = "Field `RGB_OUT_EN` writer - "]
pub type RgbOutEnW<'a, REG> = crate::BitWriter<'a, REG, RgbOutEn>;
impl<'a, REG> RgbOutEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "gating output clk ,data and control signal"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(RgbOutEn::B0)
    }
    #[doc = "rgb/lvds interface enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(RgbOutEn::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HdmiOutEn {
    #[doc = "0: gating output clk ,data and control signal"]
    B0 = 0,
    #[doc = "1: hdmi interface enable"]
    B1 = 1,
}
impl From<HdmiOutEn> for bool {
    #[inline(always)]
    fn from(variant: HdmiOutEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDMI_OUT_EN` reader - "]
pub type HdmiOutEnR = crate::BitReader<HdmiOutEn>;
impl HdmiOutEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HdmiOutEn {
        match self.bits {
            false => HdmiOutEn::B0,
            true => HdmiOutEn::B1,
        }
    }
    #[doc = "gating output clk ,data and control signal"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HdmiOutEn::B0
    }
    #[doc = "hdmi interface enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HdmiOutEn::B1
    }
}
#[doc = "Field `HDMI_OUT_EN` writer - "]
pub type HdmiOutEnW<'a, REG> = crate::BitWriter<'a, REG, HdmiOutEn>;
impl<'a, REG> HdmiOutEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "gating output clk ,data and control signal"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HdmiOutEn::B0)
    }
    #[doc = "hdmi interface enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HdmiOutEn::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdpOutEn {
    #[doc = "0: gating output clk ,data and control signal"]
    B0 = 0,
    #[doc = "1: edp interface enable"]
    B1 = 1,
}
impl From<EdpOutEn> for bool {
    #[inline(always)]
    fn from(variant: EdpOutEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDP_OUT_EN` reader - "]
pub type EdpOutEnR = crate::BitReader<EdpOutEn>;
impl EdpOutEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EdpOutEn {
        match self.bits {
            false => EdpOutEn::B0,
            true => EdpOutEn::B1,
        }
    }
    #[doc = "gating output clk ,data and control signal"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == EdpOutEn::B0
    }
    #[doc = "edp interface enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == EdpOutEn::B1
    }
}
#[doc = "Field `EDP_OUT_EN` writer - "]
pub type EdpOutEnW<'a, REG> = crate::BitWriter<'a, REG, EdpOutEn>;
impl<'a, REG> EdpOutEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "gating output clk ,data and control signal"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(EdpOutEn::B0)
    }
    #[doc = "edp interface enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(EdpOutEn::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MipiOutEn {
    #[doc = "0: gating output clk ,data and control signal"]
    B0 = 0,
    #[doc = "1: mipi interface enable"]
    B1 = 1,
}
impl From<MipiOutEn> for bool {
    #[inline(always)]
    fn from(variant: MipiOutEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIPI_OUT_EN` reader - "]
pub type MipiOutEnR = crate::BitReader<MipiOutEn>;
impl MipiOutEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MipiOutEn {
        match self.bits {
            false => MipiOutEn::B0,
            true => MipiOutEn::B1,
        }
    }
    #[doc = "gating output clk ,data and control signal"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == MipiOutEn::B0
    }
    #[doc = "mipi interface enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == MipiOutEn::B1
    }
}
#[doc = "Field `MIPI_OUT_EN` writer - "]
pub type MipiOutEnW<'a, REG> = crate::BitWriter<'a, REG, MipiOutEn>;
impl<'a, REG> MipiOutEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "gating output clk ,data and control signal"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(MipiOutEn::B0)
    }
    #[doc = "mipi interface enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(MipiOutEn::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OverlayMode {
    #[doc = "0: RGB overlay"]
    B0 = 0,
    #[doc = "1: YUV overlay"]
    B1 = 1,
}
impl From<OverlayMode> for bool {
    #[inline(always)]
    fn from(variant: OverlayMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVERLAY_MODE` reader - "]
pub type OverlayModeR = crate::BitReader<OverlayMode>;
impl OverlayModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OverlayMode {
        match self.bits {
            false => OverlayMode::B0,
            true => OverlayMode::B1,
        }
    }
    #[doc = "RGB overlay"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == OverlayMode::B0
    }
    #[doc = "YUV overlay"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == OverlayMode::B1
    }
}
#[doc = "Field `OVERLAY_MODE` writer - "]
pub type OverlayModeW<'a, REG> = crate::BitWriter<'a, REG, OverlayMode>;
impl<'a, REG> OverlayModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RGB overlay"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(OverlayMode::B0)
    }
    #[doc = "YUV overlay"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(OverlayMode::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PostLbMode {
    #[doc = "0: 4x4096"]
    B0 = 0,
    #[doc = "1: 8x2048"]
    B1 = 1,
}
impl From<PostLbMode> for bool {
    #[inline(always)]
    fn from(variant: PostLbMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POST_LB_MODE` reader - "]
pub type PostLbModeR = crate::BitReader<PostLbMode>;
impl PostLbModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PostLbMode {
        match self.bits {
            false => PostLbMode::B0,
            true => PostLbMode::B1,
        }
    }
    #[doc = "4x4096"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PostLbMode::B0
    }
    #[doc = "8x2048"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PostLbMode::B1
    }
}
#[doc = "Field `POST_LB_MODE` writer - "]
pub type PostLbModeW<'a, REG> = crate::BitWriter<'a, REG, PostLbMode>;
impl<'a, REG> PostLbModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "4x4096"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PostLbMode::B0)
    }
    #[doc = "8x2048"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PostLbMode::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win23PriOptMode {
    #[doc = "0: win2 win3 dma priority enalbe"]
    B0 = 0,
    #[doc = "1: win2 win3 dma priority disable"]
    B1 = 1,
}
impl From<Win23PriOptMode> for bool {
    #[inline(always)]
    fn from(variant: Win23PriOptMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN23_PRI_OPT_MODE` reader - "]
pub type Win23PriOptModeR = crate::BitReader<Win23PriOptMode>;
impl Win23PriOptModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win23PriOptMode {
        match self.bits {
            false => Win23PriOptMode::B0,
            true => Win23PriOptMode::B1,
        }
    }
    #[doc = "win2 win3 dma priority enalbe"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win23PriOptMode::B0
    }
    #[doc = "win2 win3 dma priority disable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win23PriOptMode::B1
    }
}
#[doc = "Field `WIN23_PRI_OPT_MODE` writer - "]
pub type Win23PriOptModeW<'a, REG> = crate::BitWriter<'a, REG, Win23PriOptMode>;
impl<'a, REG> Win23PriOptModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "win2 win3 dma priority enalbe"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win23PriOptMode::B0)
    }
    #[doc = "win2 win3 dma priority disable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win23PriOptMode::B1)
    }
}
#[doc = "Field `VOPB_FIELD_TVE_TIMING_POL` reader - VOPB_field_tve_timing_pol"]
pub type VopbFieldTveTimingPolR = crate::BitReader;
#[doc = "Field `VOPB_FIELD_TVE_TIMING_POL` writer - VOPB_field_tve_timing_pol"]
pub type VopbFieldTveTimingPolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "VOP DMA stop mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VopbDmaStop {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable * If DMA is working, the stop mode would not be active until current bus transfer is finished."]
    B1 = 1,
}
impl From<VopbDmaStop> for bool {
    #[inline(always)]
    fn from(variant: VopbDmaStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VOPB_DMA_STOP` reader - VOP DMA stop mode"]
pub type VopbDmaStopR = crate::BitReader<VopbDmaStop>;
impl VopbDmaStopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VopbDmaStop {
        match self.bits {
            false => VopbDmaStop::B0,
            true => VopbDmaStop::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VopbDmaStop::B0
    }
    #[doc = "enable * If DMA is working, the stop mode would not be active until current bus transfer is finished."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VopbDmaStop::B1
    }
}
#[doc = "Field `VOPB_DMA_STOP` writer - VOP DMA stop mode"]
pub type VopbDmaStopW<'a, REG> = crate::BitWriter<'a, REG, VopbDmaStop>;
impl<'a, REG> VopbDmaStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VopbDmaStop::B0)
    }
    #[doc = "enable * If DMA is working, the stop mode would not be active until current bus transfer is finished."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VopbDmaStop::B1)
    }
}
#[doc = "LCDC standby mode\n\nWriting '1' to turn LCDC into standby mode, All the layer would\n\ndisable and the data transfer from frame buffer memory would stop\n\nat the end of current frame.\n\nThe output would be blank.\n\nWhen writing '0' to this bit, standby mode would disable and the\n\nLCDC go back to work immediately.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VopbStandbyEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable * Black display is recommended before setting standby mode enable."]
    B1 = 1,
}
impl From<VopbStandbyEn> for bool {
    #[inline(always)]
    fn from(variant: VopbStandbyEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VOPB_STANDBY_EN` reader - LCDC standby mode\n\nWriting '1' to turn LCDC into standby mode, All the layer would\n\ndisable and the data transfer from frame buffer memory would stop\n\nat the end of current frame.\n\nThe output would be blank.\n\nWhen writing '0' to this bit, standby mode would disable and the\n\nLCDC go back to work immediately."]
pub type VopbStandbyEnR = crate::BitReader<VopbStandbyEn>;
impl VopbStandbyEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VopbStandbyEn {
        match self.bits {
            false => VopbStandbyEn::B0,
            true => VopbStandbyEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VopbStandbyEn::B0
    }
    #[doc = "enable * Black display is recommended before setting standby mode enable."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VopbStandbyEn::B1
    }
}
#[doc = "Field `VOPB_STANDBY_EN` writer - LCDC standby mode\n\nWriting '1' to turn LCDC into standby mode, All the layer would\n\ndisable and the data transfer from frame buffer memory would stop\n\nat the end of current frame.\n\nThe output would be blank.\n\nWhen writing '0' to this bit, standby mode would disable and the\n\nLCDC go back to work immediately."]
pub type VopbStandbyEnW<'a, REG> = crate::BitWriter<'a, REG, VopbStandbyEn>;
impl<'a, REG> VopbStandbyEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VopbStandbyEn::B0)
    }
    #[doc = "enable * Black display is recommended before setting standby mode enable."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VopbStandbyEn::B1)
    }
}
#[doc = "LCDC layer axi-clk auto gating enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AutoGatingEn {
    #[doc = "0: disable auto gating"]
    B0 = 0,
    #[doc = "1: enable auto gating default auto gating enable"]
    B1 = 1,
}
impl From<AutoGatingEn> for bool {
    #[inline(always)]
    fn from(variant: AutoGatingEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTO_GATING_EN` reader - LCDC layer axi-clk auto gating enable"]
pub type AutoGatingEnR = crate::BitReader<AutoGatingEn>;
impl AutoGatingEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AutoGatingEn {
        match self.bits {
            false => AutoGatingEn::B0,
            true => AutoGatingEn::B1,
        }
    }
    #[doc = "disable auto gating"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AutoGatingEn::B0
    }
    #[doc = "enable auto gating default auto gating enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AutoGatingEn::B1
    }
}
#[doc = "Field `AUTO_GATING_EN` writer - LCDC layer axi-clk auto gating enable"]
pub type AutoGatingEnW<'a, REG> = crate::BitWriter<'a, REG, AutoGatingEn>;
impl<'a, REG> AutoGatingEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable auto gating"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(AutoGatingEn::B0)
    }
    #[doc = "enable auto gating default auto gating enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(AutoGatingEn::B1)
    }
}
#[doc = "Field `IMD_TVE_DCLK_EN` reader - tve dclk enable\n\ntve dclk enable"]
pub type ImdTveDclkEnR = crate::BitReader;
#[doc = "Field `IMD_TVE_DCLK_EN` writer - tve dclk enable\n\ntve dclk enable"]
pub type ImdTveDclkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMD_TVE_DCLK_POL` reader - tve dclk pol\n\ntve dclk pol"]
pub type ImdTveDclkPolR = crate::BitReader;
#[doc = "Field `IMD_TVE_DCLK_POL` writer - tve dclk pol\n\ntve dclk pol"]
pub type ImdTveDclkPolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "tve mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TveMode {
    #[doc = "0: NTSC"]
    B0 = 0,
    #[doc = "1: PAL"]
    B1 = 1,
}
impl From<TveMode> for bool {
    #[inline(always)]
    fn from(variant: TveMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TVE_MODE` reader - tve mode"]
pub type TveModeR = crate::BitReader<TveMode>;
impl TveModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TveMode {
        match self.bits {
            false => TveMode::B0,
            true => TveMode::B1,
        }
    }
    #[doc = "NTSC"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == TveMode::B0
    }
    #[doc = "PAL"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == TveMode::B1
    }
}
#[doc = "Field `TVE_MODE` writer - tve mode"]
pub type TveModeW<'a, REG> = crate::BitWriter<'a, REG, TveMode>;
impl<'a, REG> TveModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NTSC"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(TveMode::B0)
    }
    #[doc = "PAL"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(TveMode::B1)
    }
}
#[doc = "Field `UV_OFFSET_EN` reader - uv offset enable\n\nuv offset enable"]
pub type UvOffsetEnR = crate::BitReader;
#[doc = "Field `UV_OFFSET_EN` writer - uv offset enable\n\nuv offset enable"]
pub type UvOffsetEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "genlock for tve\n\ngenlock for tve in fpga\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Genlock {
    #[doc = "0: master mode"]
    B0 = 0,
    #[doc = "1: slave mode"]
    B1 = 1,
}
impl From<Genlock> for bool {
    #[inline(always)]
    fn from(variant: Genlock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GENLOCK` reader - genlock for tve\n\ngenlock for tve in fpga"]
pub type GenlockR = crate::BitReader<Genlock>;
impl GenlockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Genlock {
        match self.bits {
            false => Genlock::B0,
            true => Genlock::B1,
        }
    }
    #[doc = "master mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Genlock::B0
    }
    #[doc = "slave mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Genlock::B1
    }
}
#[doc = "Field `GENLOCK` writer - genlock for tve\n\ngenlock for tve in fpga"]
pub type GenlockW<'a, REG> = crate::BitWriter<'a, REG, Genlock>;
impl<'a, REG> GenlockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "master mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Genlock::B0)
    }
    #[doc = "slave mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Genlock::B1)
    }
}
#[doc = "dac output sel for tve in fpga\n\ndac output sel for tve in fpga\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DacSel {
    #[doc = "0: dac 3"]
    B0 = 0,
    #[doc = "1: dac 1"]
    B1 = 1,
}
impl From<DacSel> for bool {
    #[inline(always)]
    fn from(variant: DacSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC_SEL` reader - dac output sel for tve in fpga\n\ndac output sel for tve in fpga"]
pub type DacSelR = crate::BitReader<DacSel>;
impl DacSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DacSel {
        match self.bits {
            false => DacSel::B0,
            true => DacSel::B1,
        }
    }
    #[doc = "dac 3"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DacSel::B0
    }
    #[doc = "dac 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DacSel::B1
    }
}
#[doc = "Field `DAC_SEL` writer - dac output sel for tve in fpga\n\ndac output sel for tve in fpga"]
pub type DacSelW<'a, REG> = crate::BitWriter<'a, REG, DacSel>;
impl<'a, REG> DacSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "dac 3"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DacSel::B0)
    }
    #[doc = "dac 1"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DacSel::B1)
    }
}
#[doc = "Field `VOPB_FIELD_TVE_POL` reader - VOPB_field_tve_pol"]
pub type VopbFieldTvePolR = crate::BitReader;
#[doc = "Field `IO_PAD_CLK_SEL` reader - io_pad_clk_sel"]
pub type IoPadClkSelR = crate::BitReader;
#[doc = "Field `IO_PAD_CLK_SEL` writer - io_pad_clk_sel"]
pub type IoPadClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - iep direct path enable signal"]
    #[inline(always)]
    pub fn direct_path_en(&self) -> DirectPathEnR {
        DirectPathEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - direct path layer select"]
    #[inline(always)]
    pub fn direct_path_layer_sel(&self) -> DirectPathLayerSelR {
        DirectPathLayerSelR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 8 - mipi flow ctrl enable"]
    #[inline(always)]
    pub fn edpi_halt_en(&self) -> EdpiHaltEnR {
        EdpiHaltEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1'b1: mipi command mode"]
    #[inline(always)]
    pub fn edpi_wms_mode(&self) -> EdpiWmsModeR {
        EdpiWmsModeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - edpi wms mode ,frame st signal\n\nwrite '1': edpi_wms_mode frame start (when other register is\n\nconfig done)\n\nread : wms mode hold status"]
    #[inline(always)]
    pub fn edpi_wms_fs(&self) -> EdpiWmsFsR {
        EdpiWmsFsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dp_out_en(&self) -> DpOutEnR {
        DpOutEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rgb_out_en(&self) -> RgbOutEnR {
        RgbOutEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn hdmi_out_en(&self) -> HdmiOutEnR {
        HdmiOutEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn edp_out_en(&self) -> EdpOutEnR {
        EdpOutEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn mipi_out_en(&self) -> MipiOutEnR {
        MipiOutEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn overlay_mode(&self) -> OverlayModeR {
        OverlayModeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn post_lb_mode(&self) -> PostLbModeR {
        PostLbModeR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn win23_pri_opt_mode(&self) -> Win23PriOptModeR {
        Win23PriOptModeR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - VOPB_field_tve_timing_pol"]
    #[inline(always)]
    pub fn vopb_field_tve_timing_pol(&self) -> VopbFieldTveTimingPolR {
        VopbFieldTveTimingPolR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - VOP DMA stop mode"]
    #[inline(always)]
    pub fn vopb_dma_stop(&self) -> VopbDmaStopR {
        VopbDmaStopR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - LCDC standby mode\n\nWriting '1' to turn LCDC into standby mode, All the layer would\n\ndisable and the data transfer from frame buffer memory would stop\n\nat the end of current frame.\n\nThe output would be blank.\n\nWhen writing '0' to this bit, standby mode would disable and the\n\nLCDC go back to work immediately."]
    #[inline(always)]
    pub fn vopb_standby_en(&self) -> VopbStandbyEnR {
        VopbStandbyEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - LCDC layer axi-clk auto gating enable"]
    #[inline(always)]
    pub fn auto_gating_en(&self) -> AutoGatingEnR {
        AutoGatingEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - tve dclk enable\n\ntve dclk enable"]
    #[inline(always)]
    pub fn imd_tve_dclk_en(&self) -> ImdTveDclkEnR {
        ImdTveDclkEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - tve dclk pol\n\ntve dclk pol"]
    #[inline(always)]
    pub fn imd_tve_dclk_pol(&self) -> ImdTveDclkPolR {
        ImdTveDclkPolR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - tve mode"]
    #[inline(always)]
    pub fn tve_mode(&self) -> TveModeR {
        TveModeR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - uv offset enable\n\nuv offset enable"]
    #[inline(always)]
    pub fn uv_offset_en(&self) -> UvOffsetEnR {
        UvOffsetEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - genlock for tve\n\ngenlock for tve in fpga"]
    #[inline(always)]
    pub fn genlock(&self) -> GenlockR {
        GenlockR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - dac output sel for tve in fpga\n\ndac output sel for tve in fpga"]
    #[inline(always)]
    pub fn dac_sel(&self) -> DacSelR {
        DacSelR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - VOPB_field_tve_pol"]
    #[inline(always)]
    pub fn vopb_field_tve_pol(&self) -> VopbFieldTvePolR {
        VopbFieldTvePolR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - io_pad_clk_sel"]
    #[inline(always)]
    pub fn io_pad_clk_sel(&self) -> IoPadClkSelR {
        IoPadClkSelR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - iep direct path enable signal"]
    #[inline(always)]
    #[must_use]
    pub fn direct_path_en(&mut self) -> DirectPathEnW<SysCtrlSpec> {
        DirectPathEnW::new(self, 0)
    }
    #[doc = "Bits 1:2 - direct path layer select"]
    #[inline(always)]
    #[must_use]
    pub fn direct_path_layer_sel(&mut self) -> DirectPathLayerSelW<SysCtrlSpec> {
        DirectPathLayerSelW::new(self, 1)
    }
    #[doc = "Bit 8 - mipi flow ctrl enable"]
    #[inline(always)]
    #[must_use]
    pub fn edpi_halt_en(&mut self) -> EdpiHaltEnW<SysCtrlSpec> {
        EdpiHaltEnW::new(self, 8)
    }
    #[doc = "Bit 9 - 1'b1: mipi command mode"]
    #[inline(always)]
    #[must_use]
    pub fn edpi_wms_mode(&mut self) -> EdpiWmsModeW<SysCtrlSpec> {
        EdpiWmsModeW::new(self, 9)
    }
    #[doc = "Bit 10 - edpi wms mode ,frame st signal\n\nwrite '1': edpi_wms_mode frame start (when other register is\n\nconfig done)\n\nread : wms mode hold status"]
    #[inline(always)]
    #[must_use]
    pub fn edpi_wms_fs(&mut self) -> EdpiWmsFsW<SysCtrlSpec> {
        EdpiWmsFsW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn dp_out_en(&mut self) -> DpOutEnW<SysCtrlSpec> {
        DpOutEnW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn rgb_out_en(&mut self) -> RgbOutEnW<SysCtrlSpec> {
        RgbOutEnW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn hdmi_out_en(&mut self) -> HdmiOutEnW<SysCtrlSpec> {
        HdmiOutEnW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn edp_out_en(&mut self) -> EdpOutEnW<SysCtrlSpec> {
        EdpOutEnW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn mipi_out_en(&mut self) -> MipiOutEnW<SysCtrlSpec> {
        MipiOutEnW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn overlay_mode(&mut self) -> OverlayModeW<SysCtrlSpec> {
        OverlayModeW::new(self, 16)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn post_lb_mode(&mut self) -> PostLbModeW<SysCtrlSpec> {
        PostLbModeW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn win23_pri_opt_mode(&mut self) -> Win23PriOptModeW<SysCtrlSpec> {
        Win23PriOptModeW::new(self, 19)
    }
    #[doc = "Bit 20 - VOPB_field_tve_timing_pol"]
    #[inline(always)]
    #[must_use]
    pub fn vopb_field_tve_timing_pol(&mut self) -> VopbFieldTveTimingPolW<SysCtrlSpec> {
        VopbFieldTveTimingPolW::new(self, 20)
    }
    #[doc = "Bit 21 - VOP DMA stop mode"]
    #[inline(always)]
    #[must_use]
    pub fn vopb_dma_stop(&mut self) -> VopbDmaStopW<SysCtrlSpec> {
        VopbDmaStopW::new(self, 21)
    }
    #[doc = "Bit 22 - LCDC standby mode\n\nWriting '1' to turn LCDC into standby mode, All the layer would\n\ndisable and the data transfer from frame buffer memory would stop\n\nat the end of current frame.\n\nThe output would be blank.\n\nWhen writing '0' to this bit, standby mode would disable and the\n\nLCDC go back to work immediately."]
    #[inline(always)]
    #[must_use]
    pub fn vopb_standby_en(&mut self) -> VopbStandbyEnW<SysCtrlSpec> {
        VopbStandbyEnW::new(self, 22)
    }
    #[doc = "Bit 23 - LCDC layer axi-clk auto gating enable"]
    #[inline(always)]
    #[must_use]
    pub fn auto_gating_en(&mut self) -> AutoGatingEnW<SysCtrlSpec> {
        AutoGatingEnW::new(self, 23)
    }
    #[doc = "Bit 24 - tve dclk enable\n\ntve dclk enable"]
    #[inline(always)]
    #[must_use]
    pub fn imd_tve_dclk_en(&mut self) -> ImdTveDclkEnW<SysCtrlSpec> {
        ImdTveDclkEnW::new(self, 24)
    }
    #[doc = "Bit 25 - tve dclk pol\n\ntve dclk pol"]
    #[inline(always)]
    #[must_use]
    pub fn imd_tve_dclk_pol(&mut self) -> ImdTveDclkPolW<SysCtrlSpec> {
        ImdTveDclkPolW::new(self, 25)
    }
    #[doc = "Bit 26 - tve mode"]
    #[inline(always)]
    #[must_use]
    pub fn tve_mode(&mut self) -> TveModeW<SysCtrlSpec> {
        TveModeW::new(self, 26)
    }
    #[doc = "Bit 27 - uv offset enable\n\nuv offset enable"]
    #[inline(always)]
    #[must_use]
    pub fn uv_offset_en(&mut self) -> UvOffsetEnW<SysCtrlSpec> {
        UvOffsetEnW::new(self, 27)
    }
    #[doc = "Bit 28 - genlock for tve\n\ngenlock for tve in fpga"]
    #[inline(always)]
    #[must_use]
    pub fn genlock(&mut self) -> GenlockW<SysCtrlSpec> {
        GenlockW::new(self, 28)
    }
    #[doc = "Bit 29 - dac output sel for tve in fpga\n\ndac output sel for tve in fpga"]
    #[inline(always)]
    #[must_use]
    pub fn dac_sel(&mut self) -> DacSelW<SysCtrlSpec> {
        DacSelW::new(self, 29)
    }
    #[doc = "Bit 31 - io_pad_clk_sel"]
    #[inline(always)]
    #[must_use]
    pub fn io_pad_clk_sel(&mut self) -> IoPadClkSelW<SysCtrlSpec> {
        IoPadClkSelW::new(self, 31)
    }
}
#[doc = "System control register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysCtrlSpec;
impl crate::RegisterSpec for SysCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_ctrl::R`](R) reader structure"]
impl crate::Readable for SysCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`sys_ctrl::W`](W) writer structure"]
impl crate::Writable for SysCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYS_CTRL to value 0x0080_1800"]
impl crate::Resettable for SysCtrlSpec {
    const RESET_VALUE: u32 = 0x0080_1800;
}
