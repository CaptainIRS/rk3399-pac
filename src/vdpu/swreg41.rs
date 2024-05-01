#[doc = "Register `SWREG41` reader"]
pub type R = crate::R<Swreg41Spec>;
#[doc = "Register `SWREG41` writer"]
pub type W = crate::W<Swreg41Spec>;
#[doc = "Field `SW_PP_DEC_ST` reader - post-processing start flag\n\nafter config other register,write 1 to start post-processing\n\noperation,and hw will reset to 0 after it decodered a picture\n\nshould be under External mode."]
pub type SwPpDecStR = crate::BitReader;
#[doc = "Field `SW_PP_DEC_ST` writer - post-processing start flag\n\nafter config other register,write 1 to start post-processing\n\noperation,and hw will reset to 0 after it decodered a picture\n\nshould be under External mode."]
pub type SwPpDecStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_DEINT_BLD_EN` reader - on-off Blend for deinterlacing"]
pub type SwDeintBldEnR = crate::BitReader;
#[doc = "Field `SW_DEINT_BLD_EN` writer - on-off Blend for deinterlacing"]
pub type SwDeintBldEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_DEINT_EN` reader - Deinterlace enable flag\n\nthe input data should be interlaced format"]
pub type SwDeintEnR = crate::BitReader;
#[doc = "Field `SW_DEINT_EN` writer - Deinterlace enable flag\n\nthe input data should be interlaced format"]
pub type SwDeintEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "pp auto clock gating:\n\ndefault is 1\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwPpClkgateEn {
    #[doc = "0: don't auto gating"]
    B0 = 0,
    #[doc = "1: audo gating PP dynamic clock gating enable: 1 = Clock is gated from PP structures that are not used 0 = Clock is running for all PP structures Note: Clock gating value can be changed only when PP is not enabled"]
    B1 = 1,
}
impl From<SwPpClkgateEn> for bool {
    #[inline(always)]
    fn from(variant: SwPpClkgateEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_PP_CLKGATE_EN` reader - pp auto clock gating:\n\ndefault is 1"]
pub type SwPpClkgateEnR = crate::BitReader<SwPpClkgateEn>;
impl SwPpClkgateEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwPpClkgateEn {
        match self.bits {
            false => SwPpClkgateEn::B0,
            true => SwPpClkgateEn::B1,
        }
    }
    #[doc = "don't auto gating"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwPpClkgateEn::B0
    }
    #[doc = "audo gating PP dynamic clock gating enable: 1 = Clock is gated from PP structures that are not used 0 = Clock is running for all PP structures Note: Clock gating value can be changed only when PP is not enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwPpClkgateEn::B1
    }
}
#[doc = "Field `SW_PP_CLKGATE_EN` writer - pp auto clock gating:\n\ndefault is 1"]
pub type SwPpClkgateEnW<'a, REG> = crate::BitWriter<'a, REG, SwPpClkgateEn>;
impl<'a, REG> SwPpClkgateEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "don't auto gating"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpClkgateEn::B0)
    }
    #[doc = "audo gating PP dynamic clock gating enable: 1 = Clock is gated from PP structures that are not used 0 = Clock is running for all PP structures Note: Clock gating value can be changed only when PP is not enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpClkgateEn::B1)
    }
}
#[doc = "pp pipeline width Decoder enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwPpPiplEn {
    #[doc = "0: disable, external mode"]
    B0 = 0,
    #[doc = "1: enable, pipeline mode,Post-processing pipeline with decoder"]
    B1 = 1,
}
impl From<SwPpPiplEn> for bool {
    #[inline(always)]
    fn from(variant: SwPpPiplEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_PP_PIPL_EN` reader - pp pipeline width Decoder enable"]
pub type SwPpPiplEnR = crate::BitReader<SwPpPiplEn>;
impl SwPpPiplEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwPpPiplEn {
        match self.bits {
            false => SwPpPiplEn::B0,
            true => SwPpPiplEn::B1,
        }
    }
    #[doc = "disable, external mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwPpPiplEn::B0
    }
    #[doc = "enable, pipeline mode,Post-processing pipeline with decoder"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwPpPiplEn::B1
    }
}
#[doc = "Field `SW_PP_PIPL_EN` writer - pp pipeline width Decoder enable"]
pub type SwPpPiplEnW<'a, REG> = crate::BitWriter<'a, REG, SwPpPiplEn>;
impl<'a, REG> SwPpPiplEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable, external mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpPiplEn::B0)
    }
    #[doc = "enable, pipeline mode,Post-processing pipeline with decoder"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpPiplEn::B1)
    }
}
#[doc = "Field `SW_RANGEMAP_Y_EN` reader - the enable flag for Y component Range map"]
pub type SwRangemapYEnR = crate::BitReader;
#[doc = "Field `SW_RANGEMAP_Y_EN` writer - the enable flag for Y component Range map"]
pub type SwRangemapYEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_RANGEMAP_C_EN` reader - the enable flag for C component Range map"]
pub type SwRangemapCEnR = crate::BitReader;
#[doc = "Field `SW_RANGEMAP_C_EN` writer - the enable flag for C component Range map"]
pub type SwRangemapCEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "the enable flag for fast downscaling\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwPpFdsclEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enabled. it will inprove the performance but will decrease the quality of the pic"]
    B1 = 1,
}
impl From<SwPpFdsclEn> for bool {
    #[inline(always)]
    fn from(variant: SwPpFdsclEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_PP_FDSCL_EN` reader - the enable flag for fast downscaling"]
pub type SwPpFdsclEnR = crate::BitReader<SwPpFdsclEn>;
impl SwPpFdsclEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwPpFdsclEn {
        match self.bits {
            false => SwPpFdsclEn::B0,
            true => SwPpFdsclEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwPpFdsclEn::B0
    }
    #[doc = "enabled. it will inprove the performance but will decrease the quality of the pic"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwPpFdsclEn::B1
    }
}
#[doc = "Field `SW_PP_FDSCL_EN` writer - the enable flag for fast downscaling"]
pub type SwPpFdsclEnW<'a, REG> = crate::BitWriter<'a, REG, SwPpFdsclEn>;
impl<'a, REG> SwPpFdsclEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpFdsclEn::B0)
    }
    #[doc = "enabled. it will inprove the performance but will decrease the quality of the pic"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpFdsclEn::B1)
    }
}
#[doc = "Field `SW_PP_OUT_TILED_EN` reader - the enable flag for pp output tiled mode\n\nonly used in YCbYCr format .\n\nTile size : 4x4 pixels."]
pub type SwPpOutTiledEnR = crate::BitReader;
#[doc = "Field `SW_PP_OUT_TILED_EN` writer - the enable flag for pp output tiled mode\n\nonly used in YCbYCr format .\n\nTile size : 4x4 pixels."]
pub type SwPpOutTiledEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_PP_DISCD_EN` reader - the enable flag for PP data discard\n\nthe burst length will be fix after sw_pp_discd_en=1,and extra read\n\ndata will auto be discarded by HW"]
pub type SwPpDiscdEnR = crate::BitReader;
#[doc = "Field `SW_PP_DISCD_EN` writer - the enable flag for PP data discard\n\nthe burst length will be fix after sw_pp_discd_en=1,and extra read\n\ndata will auto be discarded by HW"]
pub type SwPpDiscdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "the enable flag for mask 1\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwMask1En {
    #[doc = "0: disable,"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<SwMask1En> for bool {
    #[inline(always)]
    fn from(variant: SwMask1En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_MASK1_EN` reader - the enable flag for mask 1"]
pub type SwMask1EnR = crate::BitReader<SwMask1En>;
impl SwMask1EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwMask1En {
        match self.bits {
            false => SwMask1En::B0,
            true => SwMask1En::B1,
        }
    }
    #[doc = "disable,"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwMask1En::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwMask1En::B1
    }
}
#[doc = "Field `SW_MASK1_EN` writer - the enable flag for mask 1"]
pub type SwMask1EnW<'a, REG> = crate::BitWriter<'a, REG, SwMask1En>;
impl<'a, REG> SwMask1EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable,"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwMask1En::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwMask1En::B1)
    }
}
#[doc = "the enable flag for mask 2\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwMask2En {
    #[doc = "0: disable,"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<SwMask2En> for bool {
    #[inline(always)]
    fn from(variant: SwMask2En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_MASK2_EN` reader - the enable flag for mask 2"]
pub type SwMask2EnR = crate::BitReader<SwMask2En>;
impl SwMask2EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwMask2En {
        match self.bits {
            false => SwMask2En::B0,
            true => SwMask2En::B1,
        }
    }
    #[doc = "disable,"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwMask2En::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwMask2En::B1
    }
}
#[doc = "Field `SW_MASK2_EN` writer - the enable flag for mask 2"]
pub type SwMask2EnW<'a, REG> = crate::BitWriter<'a, REG, SwMask2En>;
impl<'a, REG> SwMask2EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable,"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwMask2En::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwMask2En::B1)
    }
}
#[doc = "Field `SW_MASK1_ABLD_EN` reader - the enable flag for Mask 1 alpha blending\n\nalpha blending for the output picture , only be supported when\n\ndata format is RGB/YUYV422\n\nAlpha blending read data from alpha blend 1 base address"]
pub type SwMask1AbldEnR = crate::BitReader;
#[doc = "Field `SW_MASK1_ABLD_EN` writer - the enable flag for Mask 1 alpha blending\n\nalpha blending for the output picture , only be supported when\n\ndata format is RGB/YUYV422\n\nAlpha blending read data from alpha blend 1 base address"]
pub type SwMask1AbldEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_MASK2_ABLD_EN` reader - the enable flag for Mask 2 alpha blending\n\nalpha blending for the output picture , only be supported when\n\ndata format is RGB/YUYV422\n\nAlpha blending read data from alpha blend 2 base address"]
pub type SwMask2AbldEnR = crate::BitReader;
#[doc = "Field `SW_MASK2_ABLD_EN` writer - the enable flag for Mask 2 alpha blending\n\nalpha blending for the output picture , only be supported when\n\ndata format is RGB/YUYV422\n\nAlpha blending read data from alpha blend 2 base address"]
pub type SwMask2AbldEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "the enable flag for Upward overcross\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwUpwdCrossEn {
    #[doc = "0: disable,"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<SwUpwdCrossEn> for bool {
    #[inline(always)]
    fn from(variant: SwUpwdCrossEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_UPWD_CROSS_EN` reader - the enable flag for Upward overcross"]
pub type SwUpwdCrossEnR = crate::BitReader<SwUpwdCrossEn>;
impl SwUpwdCrossEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwUpwdCrossEn {
        match self.bits {
            false => SwUpwdCrossEn::B0,
            true => SwUpwdCrossEn::B1,
        }
    }
    #[doc = "disable,"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwUpwdCrossEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwUpwdCrossEn::B1
    }
}
#[doc = "Field `SW_UPWD_CROSS_EN` writer - the enable flag for Upward overcross"]
pub type SwUpwdCrossEnW<'a, REG> = crate::BitWriter<'a, REG, SwUpwdCrossEn>;
impl<'a, REG> SwUpwdCrossEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable,"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwUpwdCrossEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwUpwdCrossEn::B1)
    }
}
#[doc = "the enable flag for Downward overcross\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwDownwdCrossEn {
    #[doc = "0: disable,"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<SwDownwdCrossEn> for bool {
    #[inline(always)]
    fn from(variant: SwDownwdCrossEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_DOWNWD_CROSS_EN` reader - the enable flag for Downward overcross"]
pub type SwDownwdCrossEnR = crate::BitReader<SwDownwdCrossEn>;
impl SwDownwdCrossEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwDownwdCrossEn {
        match self.bits {
            false => SwDownwdCrossEn::B0,
            true => SwDownwdCrossEn::B1,
        }
    }
    #[doc = "disable,"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwDownwdCrossEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwDownwdCrossEn::B1
    }
}
#[doc = "Field `SW_DOWNWD_CROSS_EN` writer - the enable flag for Downward overcross"]
pub type SwDownwdCrossEnW<'a, REG> = crate::BitWriter<'a, REG, SwDownwdCrossEn>;
impl<'a, REG> SwDownwdCrossEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable,"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwDownwdCrossEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwDownwdCrossEn::B1)
    }
}
#[doc = "the enable flag for Left side overcross\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwLeftsdCrossEn {
    #[doc = "0: disable,"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<SwLeftsdCrossEn> for bool {
    #[inline(always)]
    fn from(variant: SwLeftsdCrossEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_LEFTSD_CROSS_EN` reader - the enable flag for Left side overcross"]
pub type SwLeftsdCrossEnR = crate::BitReader<SwLeftsdCrossEn>;
impl SwLeftsdCrossEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwLeftsdCrossEn {
        match self.bits {
            false => SwLeftsdCrossEn::B0,
            true => SwLeftsdCrossEn::B1,
        }
    }
    #[doc = "disable,"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwLeftsdCrossEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwLeftsdCrossEn::B1
    }
}
#[doc = "Field `SW_LEFTSD_CROSS_EN` writer - the enable flag for Left side overcross"]
pub type SwLeftsdCrossEnW<'a, REG> = crate::BitWriter<'a, REG, SwLeftsdCrossEn>;
impl<'a, REG> SwLeftsdCrossEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable,"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwLeftsdCrossEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwLeftsdCrossEn::B1)
    }
}
#[doc = "the enable flag for Right side overcross\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwRightwdCrossEn {
    #[doc = "0: disable,"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<SwRightwdCrossEn> for bool {
    #[inline(always)]
    fn from(variant: SwRightwdCrossEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_RIGHTWD_CROSS_EN` reader - the enable flag for Right side overcross"]
pub type SwRightwdCrossEnR = crate::BitReader<SwRightwdCrossEn>;
impl SwRightwdCrossEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwRightwdCrossEn {
        match self.bits {
            false => SwRightwdCrossEn::B0,
            true => SwRightwdCrossEn::B1,
        }
    }
    #[doc = "disable,"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwRightwdCrossEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwRightwdCrossEn::B1
    }
}
#[doc = "Field `SW_RIGHTWD_CROSS_EN` writer - the enable flag for Right side overcross"]
pub type SwRightwdCrossEnW<'a, REG> = crate::BitWriter<'a, REG, SwRightwdCrossEn>;
impl<'a, REG> SwRightwdCrossEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable,"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwRightwdCrossEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwRightwdCrossEn::B1)
    }
}
#[doc = "Field `SW_PP_AHB_HLOCK_EN` reader - the enable flag for AHB master HLOCK\n\nthe service is locked to pp as long as it needs the bus"]
pub type SwPpAhbHlockEnR = crate::BitReader;
#[doc = "Field `SW_PP_AHB_HLOCK_EN` writer - the enable flag for AHB master HLOCK\n\nthe service is locked to pp as long as it needs the bus"]
pub type SwPpAhbHlockEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - post-processing start flag\n\nafter config other register,write 1 to start post-processing\n\noperation,and hw will reset to 0 after it decodered a picture\n\nshould be under External mode."]
    #[inline(always)]
    pub fn sw_pp_dec_st(&self) -> SwPpDecStR {
        SwPpDecStR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - on-off Blend for deinterlacing"]
    #[inline(always)]
    pub fn sw_deint_bld_en(&self) -> SwDeintBldEnR {
        SwDeintBldEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Deinterlace enable flag\n\nthe input data should be interlaced format"]
    #[inline(always)]
    pub fn sw_deint_en(&self) -> SwDeintEnR {
        SwDeintEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - pp auto clock gating:\n\ndefault is 1"]
    #[inline(always)]
    pub fn sw_pp_clkgate_en(&self) -> SwPpClkgateEnR {
        SwPpClkgateEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - pp pipeline width Decoder enable"]
    #[inline(always)]
    pub fn sw_pp_pipl_en(&self) -> SwPpPiplEnR {
        SwPpPiplEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - the enable flag for Y component Range map"]
    #[inline(always)]
    pub fn sw_rangemap_y_en(&self) -> SwRangemapYEnR {
        SwRangemapYEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - the enable flag for C component Range map"]
    #[inline(always)]
    pub fn sw_rangemap_c_en(&self) -> SwRangemapCEnR {
        SwRangemapCEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - the enable flag for fast downscaling"]
    #[inline(always)]
    pub fn sw_pp_fdscl_en(&self) -> SwPpFdsclEnR {
        SwPpFdsclEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - the enable flag for pp output tiled mode\n\nonly used in YCbYCr format .\n\nTile size : 4x4 pixels."]
    #[inline(always)]
    pub fn sw_pp_out_tiled_en(&self) -> SwPpOutTiledEnR {
        SwPpOutTiledEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - the enable flag for PP data discard\n\nthe burst length will be fix after sw_pp_discd_en=1,and extra read\n\ndata will auto be discarded by HW"]
    #[inline(always)]
    pub fn sw_pp_discd_en(&self) -> SwPpDiscdEnR {
        SwPpDiscdEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - the enable flag for mask 1"]
    #[inline(always)]
    pub fn sw_mask1_en(&self) -> SwMask1EnR {
        SwMask1EnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - the enable flag for mask 2"]
    #[inline(always)]
    pub fn sw_mask2_en(&self) -> SwMask2EnR {
        SwMask2EnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - the enable flag for Mask 1 alpha blending\n\nalpha blending for the output picture , only be supported when\n\ndata format is RGB/YUYV422\n\nAlpha blending read data from alpha blend 1 base address"]
    #[inline(always)]
    pub fn sw_mask1_abld_en(&self) -> SwMask1AbldEnR {
        SwMask1AbldEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - the enable flag for Mask 2 alpha blending\n\nalpha blending for the output picture , only be supported when\n\ndata format is RGB/YUYV422\n\nAlpha blending read data from alpha blend 2 base address"]
    #[inline(always)]
    pub fn sw_mask2_abld_en(&self) -> SwMask2AbldEnR {
        SwMask2AbldEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - the enable flag for Upward overcross"]
    #[inline(always)]
    pub fn sw_upwd_cross_en(&self) -> SwUpwdCrossEnR {
        SwUpwdCrossEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - the enable flag for Downward overcross"]
    #[inline(always)]
    pub fn sw_downwd_cross_en(&self) -> SwDownwdCrossEnR {
        SwDownwdCrossEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - the enable flag for Left side overcross"]
    #[inline(always)]
    pub fn sw_leftsd_cross_en(&self) -> SwLeftsdCrossEnR {
        SwLeftsdCrossEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - the enable flag for Right side overcross"]
    #[inline(always)]
    pub fn sw_rightwd_cross_en(&self) -> SwRightwdCrossEnR {
        SwRightwdCrossEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - the enable flag for AHB master HLOCK\n\nthe service is locked to pp as long as it needs the bus"]
    #[inline(always)]
    pub fn sw_pp_ahb_hlock_en(&self) -> SwPpAhbHlockEnR {
        SwPpAhbHlockEnR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - post-processing start flag\n\nafter config other register,write 1 to start post-processing\n\noperation,and hw will reset to 0 after it decodered a picture\n\nshould be under External mode."]
    #[inline(always)]
    #[must_use]
    pub fn sw_pp_dec_st(&mut self) -> SwPpDecStW<Swreg41Spec> {
        SwPpDecStW::new(self, 0)
    }
    #[doc = "Bit 1 - on-off Blend for deinterlacing"]
    #[inline(always)]
    #[must_use]
    pub fn sw_deint_bld_en(&mut self) -> SwDeintBldEnW<Swreg41Spec> {
        SwDeintBldEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Deinterlace enable flag\n\nthe input data should be interlaced format"]
    #[inline(always)]
    #[must_use]
    pub fn sw_deint_en(&mut self) -> SwDeintEnW<Swreg41Spec> {
        SwDeintEnW::new(self, 2)
    }
    #[doc = "Bit 3 - pp auto clock gating:\n\ndefault is 1"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pp_clkgate_en(&mut self) -> SwPpClkgateEnW<Swreg41Spec> {
        SwPpClkgateEnW::new(self, 3)
    }
    #[doc = "Bit 4 - pp pipeline width Decoder enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pp_pipl_en(&mut self) -> SwPpPiplEnW<Swreg41Spec> {
        SwPpPiplEnW::new(self, 4)
    }
    #[doc = "Bit 8 - the enable flag for Y component Range map"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rangemap_y_en(&mut self) -> SwRangemapYEnW<Swreg41Spec> {
        SwRangemapYEnW::new(self, 8)
    }
    #[doc = "Bit 9 - the enable flag for C component Range map"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rangemap_c_en(&mut self) -> SwRangemapCEnW<Swreg41Spec> {
        SwRangemapCEnW::new(self, 9)
    }
    #[doc = "Bit 10 - the enable flag for fast downscaling"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pp_fdscl_en(&mut self) -> SwPpFdsclEnW<Swreg41Spec> {
        SwPpFdsclEnW::new(self, 10)
    }
    #[doc = "Bit 11 - the enable flag for pp output tiled mode\n\nonly used in YCbYCr format .\n\nTile size : 4x4 pixels."]
    #[inline(always)]
    #[must_use]
    pub fn sw_pp_out_tiled_en(&mut self) -> SwPpOutTiledEnW<Swreg41Spec> {
        SwPpOutTiledEnW::new(self, 11)
    }
    #[doc = "Bit 16 - the enable flag for PP data discard\n\nthe burst length will be fix after sw_pp_discd_en=1,and extra read\n\ndata will auto be discarded by HW"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pp_discd_en(&mut self) -> SwPpDiscdEnW<Swreg41Spec> {
        SwPpDiscdEnW::new(self, 16)
    }
    #[doc = "Bit 20 - the enable flag for mask 1"]
    #[inline(always)]
    #[must_use]
    pub fn sw_mask1_en(&mut self) -> SwMask1EnW<Swreg41Spec> {
        SwMask1EnW::new(self, 20)
    }
    #[doc = "Bit 21 - the enable flag for mask 2"]
    #[inline(always)]
    #[must_use]
    pub fn sw_mask2_en(&mut self) -> SwMask2EnW<Swreg41Spec> {
        SwMask2EnW::new(self, 21)
    }
    #[doc = "Bit 22 - the enable flag for Mask 1 alpha blending\n\nalpha blending for the output picture , only be supported when\n\ndata format is RGB/YUYV422\n\nAlpha blending read data from alpha blend 1 base address"]
    #[inline(always)]
    #[must_use]
    pub fn sw_mask1_abld_en(&mut self) -> SwMask1AbldEnW<Swreg41Spec> {
        SwMask1AbldEnW::new(self, 22)
    }
    #[doc = "Bit 23 - the enable flag for Mask 2 alpha blending\n\nalpha blending for the output picture , only be supported when\n\ndata format is RGB/YUYV422\n\nAlpha blending read data from alpha blend 2 base address"]
    #[inline(always)]
    #[must_use]
    pub fn sw_mask2_abld_en(&mut self) -> SwMask2AbldEnW<Swreg41Spec> {
        SwMask2AbldEnW::new(self, 23)
    }
    #[doc = "Bit 24 - the enable flag for Upward overcross"]
    #[inline(always)]
    #[must_use]
    pub fn sw_upwd_cross_en(&mut self) -> SwUpwdCrossEnW<Swreg41Spec> {
        SwUpwdCrossEnW::new(self, 24)
    }
    #[doc = "Bit 25 - the enable flag for Downward overcross"]
    #[inline(always)]
    #[must_use]
    pub fn sw_downwd_cross_en(&mut self) -> SwDownwdCrossEnW<Swreg41Spec> {
        SwDownwdCrossEnW::new(self, 25)
    }
    #[doc = "Bit 26 - the enable flag for Left side overcross"]
    #[inline(always)]
    #[must_use]
    pub fn sw_leftsd_cross_en(&mut self) -> SwLeftsdCrossEnW<Swreg41Spec> {
        SwLeftsdCrossEnW::new(self, 26)
    }
    #[doc = "Bit 27 - the enable flag for Right side overcross"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rightwd_cross_en(&mut self) -> SwRightwdCrossEnW<Swreg41Spec> {
        SwRightwdCrossEnW::new(self, 27)
    }
    #[doc = "Bit 28 - the enable flag for AHB master HLOCK\n\nthe service is locked to pp as long as it needs the bus"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pp_ahb_hlock_en(&mut self) -> SwPpAhbHlockEnW<Swreg41Spec> {
        SwPpAhbHlockEnW::new(self, 28)
    }
}
#[doc = "enable ctrl flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg41::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg41::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg41Spec;
impl crate::RegisterSpec for Swreg41Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg41::R`](R) reader structure"]
impl crate::Readable for Swreg41Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg41::W`](W) writer structure"]
impl crate::Writable for Swreg41Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG41 to value 0x08"]
impl crate::Resettable for Swreg41Spec {
    const RESET_VALUE: u32 = 0x08;
}
