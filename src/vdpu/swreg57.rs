#[doc = "Register `SWREG57` reader"]
pub type R = crate::R<Swreg57Spec>;
#[doc = "Register `SWREG57` writer"]
pub type W = crate::W<Swreg57Spec>;
#[doc = "Field `SW_DEC_ST_WORK` reader - enable flag for decoder to start working\n\nhw will auto reset this be after a frame be decodered no matter it\n\nright or have some error"]
pub type SwDecStWorkR = crate::BitReader;
#[doc = "Field `SW_DEC_ST_WORK` writer - enable flag for decoder to start working\n\nhw will auto reset this be after a frame be decodered no matter it\n\nright or have some error"]
pub type SwDecStWorkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "enable flag for Refer picture buffer 2\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwRefpicBuf2En {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable (should : pic size > QVGA)"]
    B1 = 1,
}
impl From<SwRefpicBuf2En> for bool {
    #[inline(always)]
    fn from(variant: SwRefpicBuf2En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_REFPIC_BUF2_EN` reader - enable flag for Refer picture buffer 2"]
pub type SwRefpicBuf2EnR = crate::BitReader<SwRefpicBuf2En>;
impl SwRefpicBuf2EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwRefpicBuf2En {
        match self.bits {
            false => SwRefpicBuf2En::B0,
            true => SwRefpicBuf2En::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwRefpicBuf2En::B0
    }
    #[doc = "enable (should : pic size > QVGA)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwRefpicBuf2En::B1
    }
}
#[doc = "Field `SW_REFPIC_BUF2_EN` writer - enable flag for Refer picture buffer 2"]
pub type SwRefpicBuf2EnW<'a, REG> = crate::BitWriter<'a, REG, SwRefpicBuf2En>;
impl<'a, REG> SwRefpicBuf2EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwRefpicBuf2En::B0)
    }
    #[doc = "enable (should : pic size > QVGA)"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwRefpicBuf2En::B1)
    }
}
#[doc = "disable flag for wiriting decoder output data to external memory\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwDecWrExtmemDis {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: disable(no write to external memory)"]
    B1 = 1,
}
impl From<SwDecWrExtmemDis> for bool {
    #[inline(always)]
    fn from(variant: SwDecWrExtmemDis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_DEC_WR_EXTMEM_DIS` reader - disable flag for wiriting decoder output data to external memory"]
pub type SwDecWrExtmemDisR = crate::BitReader<SwDecWrExtmemDis>;
impl SwDecWrExtmemDisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwDecWrExtmemDis {
        match self.bits {
            false => SwDecWrExtmemDis::B0,
            true => SwDecWrExtmemDis::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwDecWrExtmemDis::B0
    }
    #[doc = "disable(no write to external memory)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwDecWrExtmemDis::B1
    }
}
#[doc = "Field `SW_DEC_WR_EXTMEM_DIS` writer - disable flag for wiriting decoder output data to external memory"]
pub type SwDecWrExtmemDisW<'a, REG> = crate::BitWriter<'a, REG, SwDecWrExtmemDis>;
impl<'a, REG> SwDecWrExtmemDisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwDecWrExtmemDis::B0)
    }
    #[doc = "disable(no write to external memory)"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwDecWrExtmemDis::B1)
    }
}
#[doc = "the enable flag for Decoder auto clock gating\n\ndefault hw will reset to 1\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwDecClkgateEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<SwDecClkgateEn> for bool {
    #[inline(always)]
    fn from(variant: SwDecClkgateEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_DEC_CLKGATE_EN` reader - the enable flag for Decoder auto clock gating\n\ndefault hw will reset to 1"]
pub type SwDecClkgateEnR = crate::BitReader<SwDecClkgateEn>;
impl SwDecClkgateEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwDecClkgateEn {
        match self.bits {
            false => SwDecClkgateEn::B0,
            true => SwDecClkgateEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwDecClkgateEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwDecClkgateEn::B1
    }
}
#[doc = "Field `SW_DEC_CLKGATE_EN` writer - the enable flag for Decoder auto clock gating\n\ndefault hw will reset to 1"]
pub type SwDecClkgateEnW<'a, REG> = crate::BitWriter<'a, REG, SwDecClkgateEn>;
impl<'a, REG> SwDecClkgateEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwDecClkgateEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwDecClkgateEn::B1)
    }
}
#[doc = "the enable flag for Timeout interrupt\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwTimeoutStsEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable (if hw can be working status too long,you will get an timeout interrupt)"]
    B1 = 1,
}
impl From<SwTimeoutStsEn> for bool {
    #[inline(always)]
    fn from(variant: SwTimeoutStsEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_TIMEOUT_STS_EN` reader - the enable flag for Timeout interrupt"]
pub type SwTimeoutStsEnR = crate::BitReader<SwTimeoutStsEn>;
impl SwTimeoutStsEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwTimeoutStsEn {
        match self.bits {
            false => SwTimeoutStsEn::B0,
            true => SwTimeoutStsEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwTimeoutStsEn::B0
    }
    #[doc = "enable (if hw can be working status too long,you will get an timeout interrupt)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwTimeoutStsEn::B1
    }
}
#[doc = "Field `SW_TIMEOUT_STS_EN` writer - the enable flag for Timeout interrupt"]
pub type SwTimeoutStsEnW<'a, REG> = crate::BitWriter<'a, REG, SwTimeoutStsEn>;
impl<'a, REG> SwTimeoutStsEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwTimeoutStsEn::B0)
    }
    #[doc = "enable (if hw can be working status too long,you will get an timeout interrupt)"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwTimeoutStsEn::B1)
    }
}
#[doc = "the enable flag for reading Picture order count table\n\nread data from memory used\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwRdCntTabEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable (hw will read pic order counts)"]
    B1 = 1,
}
impl From<SwRdCntTabEn> for bool {
    #[inline(always)]
    fn from(variant: SwRdCntTabEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_RD_CNT_TAB_EN` reader - the enable flag for reading Picture order count table\n\nread data from memory used"]
pub type SwRdCntTabEnR = crate::BitReader<SwRdCntTabEn>;
impl SwRdCntTabEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwRdCntTabEn {
        match self.bits {
            false => SwRdCntTabEn::B0,
            true => SwRdCntTabEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwRdCntTabEn::B0
    }
    #[doc = "enable (hw will read pic order counts)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwRdCntTabEn::B1
    }
}
#[doc = "Field `SW_RD_CNT_TAB_EN` writer - the enable flag for reading Picture order count table\n\nread data from memory used"]
pub type SwRdCntTabEnW<'a, REG> = crate::BitWriter<'a, REG, SwRdCntTabEn>;
impl<'a, REG> SwRdCntTabEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwRdCntTabEn::B0)
    }
    #[doc = "enable (hw will read pic order counts)"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwRdCntTabEn::B1)
    }
}
#[doc = "the enable flag for Sequence includes MBAFF coded pictures\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwSequMbaffEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<SwSequMbaffEn> for bool {
    #[inline(always)]
    fn from(variant: SwSequMbaffEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_SEQU_MBAFF_EN` reader - the enable flag for Sequence includes MBAFF coded pictures"]
pub type SwSequMbaffEnR = crate::BitReader<SwSequMbaffEn>;
impl SwSequMbaffEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwSequMbaffEn {
        match self.bits {
            false => SwSequMbaffEn::B0,
            true => SwSequMbaffEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwSequMbaffEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwSequMbaffEn::B1
    }
}
#[doc = "Field `SW_SEQU_MBAFF_EN` writer - the enable flag for Sequence includes MBAFF coded pictures"]
pub type SwSequMbaffEnW<'a, REG> = crate::BitWriter<'a, REG, SwSequMbaffEn>;
impl<'a, REG> SwSequMbaffEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwSequMbaffEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwSequMbaffEn::B1)
    }
}
#[doc = "enable flag for FWD reference top field have been decoded first\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwFirstReftopEn {
    #[doc = "0: FWD reference bottom field"]
    B0 = 0,
    #[doc = "1: FWD reference top field"]
    B1 = 1,
}
impl From<SwFirstReftopEn> for bool {
    #[inline(always)]
    fn from(variant: SwFirstReftopEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_FIRST_REFTOP_EN` reader - enable flag for FWD reference top field have been decoded first"]
pub type SwFirstReftopEnR = crate::BitReader<SwFirstReftopEn>;
impl SwFirstReftopEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwFirstReftopEn {
        match self.bits {
            false => SwFirstReftopEn::B0,
            true => SwFirstReftopEn::B1,
        }
    }
    #[doc = "FWD reference bottom field"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwFirstReftopEn::B0
    }
    #[doc = "FWD reference top field"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwFirstReftopEn::B1
    }
}
#[doc = "Field `SW_FIRST_REFTOP_EN` writer - enable flag for FWD reference top field have been decoded first"]
pub type SwFirstReftopEnW<'a, REG> = crate::BitWriter<'a, REG, SwFirstReftopEn>;
impl<'a, REG> SwFirstReftopEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FWD reference bottom field"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwFirstReftopEn::B0)
    }
    #[doc = "FWD reference top field"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwFirstReftopEn::B1)
    }
}
#[doc = "Field `SW_REFTOP_EN` reader - enable flag for reference top field\n\n0 = bottom field\n\n1 = top field"]
pub type SwReftopEnR = crate::BitReader;
#[doc = "Field `SW_REFTOP_EN` writer - enable flag for reference top field\n\n0 = bottom field\n\n1 = top field"]
pub type SwReftopEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "enable flag for Direct mode motion vector write current picture\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwDmmvWrEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable this bit used in MPEG2 or VP6 is for the purpose error concealment case. this bit used in h264 is for the purpose write DPB case with the corresponding reference picture. this bit used in other decoder format is for the purpose writing to external memory starting from mv start address"]
    B1 = 1,
}
impl From<SwDmmvWrEn> for bool {
    #[inline(always)]
    fn from(variant: SwDmmvWrEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_DMMV_WR_EN` reader - enable flag for Direct mode motion vector write current picture"]
pub type SwDmmvWrEnR = crate::BitReader<SwDmmvWrEn>;
impl SwDmmvWrEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwDmmvWrEn {
        match self.bits {
            false => SwDmmvWrEn::B0,
            true => SwDmmvWrEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwDmmvWrEn::B0
    }
    #[doc = "enable this bit used in MPEG2 or VP6 is for the purpose error concealment case. this bit used in h264 is for the purpose write DPB case with the corresponding reference picture. this bit used in other decoder format is for the purpose writing to external memory starting from mv start address"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwDmmvWrEn::B1
    }
}
#[doc = "Field `SW_DMMV_WR_EN` writer - enable flag for Direct mode motion vector write current picture"]
pub type SwDmmvWrEnW<'a, REG> = crate::BitWriter<'a, REG, SwDmmvWrEn>;
impl<'a, REG> SwDmmvWrEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwDmmvWrEn::B0)
    }
    #[doc = "enable this bit used in MPEG2 or VP6 is for the purpose error concealment case. this bit used in h264 is for the purpose write DPB case with the corresponding reference picture. this bit used in other decoder format is for the purpose writing to external memory starting from mv start address"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwDmmvWrEn::B1)
    }
}
#[doc = "Field `SW_SORSPA_EN` reader - enable flag for Sorenson Sparc\n\nvalid when sw_dec_fmt_sel= MPEG- 4\n\n0 = disabled\n\n1 = enable\n\nif enable,will use Sorenson escape coding to compatible stream for\n\nh.263"]
pub type SwSorspaEnR = crate::BitReader;
#[doc = "Field `SW_SORSPA_EN` writer - enable flag for Sorenson Sparc\n\nvalid when sw_dec_fmt_sel= MPEG- 4\n\n0 = disabled\n\n1 = enable\n\nif enable,will use Sorenson escape coding to compatible stream for\n\nh.263"]
pub type SwSorspaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "progressive and interlaced for coding mode\n\nused for forward reference picture:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwFwdRefpicModeSel {
    #[doc = "0: progressive"]
    B0 = 0,
    #[doc = "1: interlaced the backward reference picture is the same as current picture"]
    B1 = 1,
}
impl From<SwFwdRefpicModeSel> for bool {
    #[inline(always)]
    fn from(variant: SwFwdRefpicModeSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_FWD_REFPIC_MODE_SEL` reader - progressive and interlaced for coding mode\n\nused for forward reference picture:"]
pub type SwFwdRefpicModeSelR = crate::BitReader<SwFwdRefpicModeSel>;
impl SwFwdRefpicModeSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwFwdRefpicModeSel {
        match self.bits {
            false => SwFwdRefpicModeSel::B0,
            true => SwFwdRefpicModeSel::B1,
        }
    }
    #[doc = "progressive"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwFwdRefpicModeSel::B0
    }
    #[doc = "interlaced the backward reference picture is the same as current picture"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwFwdRefpicModeSel::B1
    }
}
#[doc = "Field `SW_FWD_REFPIC_MODE_SEL` writer - progressive and interlaced for coding mode\n\nused for forward reference picture:"]
pub type SwFwdRefpicModeSelW<'a, REG> = crate::BitWriter<'a, REG, SwFwdRefpicModeSel>;
impl<'a, REG> SwFwdRefpicModeSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "progressive"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwFwdRefpicModeSel::B0)
    }
    #[doc = "interlaced the backward reference picture is the same as current picture"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwFwdRefpicModeSel::B1)
    }
}
#[doc = "select which field will be decodered\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwPicDecfieldSel {
    #[doc = "0: bottom field"]
    B0 = 0,
    #[doc = "1: top field"]
    B1 = 1,
}
impl From<SwPicDecfieldSel> for bool {
    #[inline(always)]
    fn from(variant: SwPicDecfieldSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_PIC_DECFIELD_SEL` reader - select which field will be decodered"]
pub type SwPicDecfieldSelR = crate::BitReader<SwPicDecfieldSel>;
impl SwPicDecfieldSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwPicDecfieldSel {
        match self.bits {
            false => SwPicDecfieldSel::B0,
            true => SwPicDecfieldSel::B1,
        }
    }
    #[doc = "bottom field"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwPicDecfieldSel::B0
    }
    #[doc = "top field"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwPicDecfieldSel::B1
    }
}
#[doc = "Field `SW_PIC_DECFIELD_SEL` writer - select which field will be decodered"]
pub type SwPicDecfieldSelW<'a, REG> = crate::BitWriter<'a, REG, SwPicDecfieldSel>;
impl<'a, REG> SwPicDecfieldSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "bottom field"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwPicDecfieldSel::B0)
    }
    #[doc = "top field"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwPicDecfieldSel::B1)
    }
}
#[doc = "pic type sel0 flag\n\nshould need sw_pic_type_sel1=0\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwPicTypeSel0 {
    #[doc = "0: Intra type (I)"]
    B0 = 0,
    #[doc = "1: Inter type (P)"]
    B1 = 1,
}
impl From<SwPicTypeSel0> for bool {
    #[inline(always)]
    fn from(variant: SwPicTypeSel0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_PIC_TYPE_SEL0` reader - pic type sel0 flag\n\nshould need sw_pic_type_sel1=0"]
pub type SwPicTypeSel0R = crate::BitReader<SwPicTypeSel0>;
impl SwPicTypeSel0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwPicTypeSel0 {
        match self.bits {
            false => SwPicTypeSel0::B0,
            true => SwPicTypeSel0::B1,
        }
    }
    #[doc = "Intra type (I)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwPicTypeSel0::B0
    }
    #[doc = "Inter type (P)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwPicTypeSel0::B1
    }
}
#[doc = "Field `SW_PIC_TYPE_SEL0` writer - pic type sel0 flag\n\nshould need sw_pic_type_sel1=0"]
pub type SwPicTypeSel0W<'a, REG> = crate::BitWriter<'a, REG, SwPicTypeSel0>;
impl<'a, REG> SwPicTypeSel0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Intra type (I)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwPicTypeSel0::B0)
    }
    #[doc = "Inter type (P)"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwPicTypeSel0::B1)
    }
}
#[doc = "pic type sel1 flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwPicTypeSel1 {
    #[doc = "0: desided by sw_pic_type_sel0"]
    B0 = 0,
    #[doc = "1: picture type is BI/D/B note: D is for mpeg1 B is for h264"]
    B1 = 1,
}
impl From<SwPicTypeSel1> for bool {
    #[inline(always)]
    fn from(variant: SwPicTypeSel1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_PIC_TYPE_SEL1` reader - pic type sel1 flag"]
pub type SwPicTypeSel1R = crate::BitReader<SwPicTypeSel1>;
impl SwPicTypeSel1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwPicTypeSel1 {
        match self.bits {
            false => SwPicTypeSel1::B0,
            true => SwPicTypeSel1::B1,
        }
    }
    #[doc = "desided by sw_pic_type_sel0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwPicTypeSel1::B0
    }
    #[doc = "picture type is BI/D/B note: D is for mpeg1 B is for h264"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwPicTypeSel1::B1
    }
}
#[doc = "Field `SW_PIC_TYPE_SEL1` writer - pic type sel1 flag"]
pub type SwPicTypeSel1W<'a, REG> = crate::BitWriter<'a, REG, SwPicTypeSel1>;
impl<'a, REG> SwPicTypeSel1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "desided by sw_pic_type_sel0"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwPicTypeSel1::B0)
    }
    #[doc = "picture type is BI/D/B note: D is for mpeg1 B is for h264"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwPicTypeSel1::B1)
    }
}
#[doc = "the current picture Structure selected\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwCurpicStruSel {
    #[doc = "0: frame structure, (that isMBAFF structured picture is interlaced)"]
    B0 = 0,
    #[doc = "1: field structure"]
    B1 = 1,
}
impl From<SwCurpicStruSel> for bool {
    #[inline(always)]
    fn from(variant: SwCurpicStruSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_CURPIC_STRU_SEL` reader - the current picture Structure selected"]
pub type SwCurpicStruSelR = crate::BitReader<SwCurpicStruSel>;
impl SwCurpicStruSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwCurpicStruSel {
        match self.bits {
            false => SwCurpicStruSel::B0,
            true => SwCurpicStruSel::B1,
        }
    }
    #[doc = "frame structure, (that isMBAFF structured picture is interlaced)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwCurpicStruSel::B0
    }
    #[doc = "field structure"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwCurpicStruSel::B1
    }
}
#[doc = "Field `SW_CURPIC_STRU_SEL` writer - the current picture Structure selected"]
pub type SwCurpicStruSelW<'a, REG> = crate::BitWriter<'a, REG, SwCurpicStruSel>;
impl<'a, REG> SwCurpicStruSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "frame structure, (that isMBAFF structured picture is interlaced)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwCurpicStruSel::B0)
    }
    #[doc = "field structure"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwCurpicStruSel::B1)
    }
}
#[doc = "the current picture coding mode select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwCurpicCodeSel {
    #[doc = "0: progressive"]
    B0 = 0,
    #[doc = "1: interlaced"]
    B1 = 1,
}
impl From<SwCurpicCodeSel> for bool {
    #[inline(always)]
    fn from(variant: SwCurpicCodeSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_CURPIC_CODE_SEL` reader - the current picture coding mode select"]
pub type SwCurpicCodeSelR = crate::BitReader<SwCurpicCodeSel>;
impl SwCurpicCodeSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwCurpicCodeSel {
        match self.bits {
            false => SwCurpicCodeSel::B0,
            true => SwCurpicCodeSel::B1,
        }
    }
    #[doc = "progressive"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwCurpicCodeSel::B0
    }
    #[doc = "interlaced"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwCurpicCodeSel::B1
    }
}
#[doc = "Field `SW_CURPIC_CODE_SEL` writer - the current picture coding mode select"]
pub type SwCurpicCodeSelW<'a, REG> = crate::BitWriter<'a, REG, SwCurpicCodeSel>;
impl<'a, REG> SwCurpicCodeSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "progressive"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwCurpicCodeSel::B0)
    }
    #[doc = "interlaced"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwCurpicCodeSel::B1)
    }
}
#[doc = "enable flag for Progressive JPEG\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwProgJpegEn {
    #[doc = "0: baseline JPEG"]
    B0 = 0,
    #[doc = "1: progressive JPEG"]
    B1 = 1,
}
impl From<SwProgJpegEn> for bool {
    #[inline(always)]
    fn from(variant: SwProgJpegEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_PROG_JPEG_EN` reader - enable flag for Progressive JPEG"]
pub type SwProgJpegEnR = crate::BitReader<SwProgJpegEn>;
impl SwProgJpegEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwProgJpegEn {
        match self.bits {
            false => SwProgJpegEn::B0,
            true => SwProgJpegEn::B1,
        }
    }
    #[doc = "baseline JPEG"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwProgJpegEn::B0
    }
    #[doc = "progressive JPEG"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwProgJpegEn::B1
    }
}
#[doc = "Field `SW_PROG_JPEG_EN` writer - enable flag for Progressive JPEG"]
pub type SwProgJpegEnW<'a, REG> = crate::BitWriter<'a, REG, SwProgJpegEn>;
impl<'a, REG> SwProgJpegEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "baseline JPEG"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwProgJpegEn::B0)
    }
    #[doc = "progressive JPEG"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwProgJpegEn::B1)
    }
}
#[doc = "enable for RLC mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwRlcModeEn {
    #[doc = "0: decoder data come from bit stream(VLC mode),side information"]
    B0 = 0,
    #[doc = "1: decoder data come from RLC input data,only h.264 and MPEG4 sp be valid"]
    B1 = 1,
}
impl From<SwRlcModeEn> for bool {
    #[inline(always)]
    fn from(variant: SwRlcModeEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_RLC_MODE_EN` reader - enable for RLC mode"]
pub type SwRlcModeEnR = crate::BitReader<SwRlcModeEn>;
impl SwRlcModeEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwRlcModeEn {
        match self.bits {
            false => SwRlcModeEn::B0,
            true => SwRlcModeEn::B1,
        }
    }
    #[doc = "decoder data come from bit stream(VLC mode),side information"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwRlcModeEn::B0
    }
    #[doc = "decoder data come from RLC input data,only h.264 and MPEG4 sp be valid"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwRlcModeEn::B1
    }
}
#[doc = "Field `SW_RLC_MODE_EN` writer - enable for RLC mode"]
pub type SwRlcModeEnW<'a, REG> = crate::BitWriter<'a, REG, SwRlcModeEn>;
impl<'a, REG> SwRlcModeEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "decoder data come from bit stream(VLC mode),side information"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwRlcModeEn::B0)
    }
    #[doc = "decoder data come from RLC input data,only h.264 and MPEG4 sp be valid"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwRlcModeEn::B1)
    }
}
#[doc = "Field `SW_ADDIT_CH_FMT_WEN` reader - Enable for additional chrominance data format writing\n\ntiled mode should be disable,when this bit be used\n\ndecoder writes chrominance: group of 8 pixels of Cb then\n\ncorresponding 8 pixels of Cr\n\nData is written to sw_dec_ch8pix_st_adr."]
pub type SwAdditChFmtWenR = crate::BitReader;
#[doc = "Field `SW_ADDIT_CH_FMT_WEN` writer - Enable for additional chrominance data format writing\n\ntiled mode should be disable,when this bit be used\n\ndecoder writes chrominance: group of 8 pixels of Cb then\n\ncorresponding 8 pixels of Cr\n\nData is written to sw_dec_ch8pix_st_adr."]
pub type SwAdditChFmtWenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "existence flag for stream start code\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwStCodeExist {
    #[doc = "0: not exist"]
    B0 = 0,
    #[doc = "1: exist"]
    B1 = 1,
}
impl From<SwStCodeExist> for bool {
    #[inline(always)]
    fn from(variant: SwStCodeExist) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_ST_CODE_EXIST` reader - existence flag for stream start code"]
pub type SwStCodeExistR = crate::BitReader<SwStCodeExist>;
impl SwStCodeExistR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwStCodeExist {
        match self.bits {
            false => SwStCodeExist::B0,
            true => SwStCodeExist::B1,
        }
    }
    #[doc = "not exist"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwStCodeExist::B0
    }
    #[doc = "exist"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwStCodeExist::B1
    }
}
#[doc = "Field `SW_ST_CODE_EXIST` writer - existence flag for stream start code"]
pub type SwStCodeExistW<'a, REG> = crate::BitWriter<'a, REG, SwStCodeExist>;
impl<'a, REG> SwStCodeExistW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "not exist"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwStCodeExist::B0)
    }
    #[doc = "exist"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwStCodeExist::B1)
    }
}
#[doc = "Field `SW_INTER_DBLSPEED` reader - inter double speed enable\n\nInter double speed enable"]
pub type SwInterDblspeedR = crate::BitReader;
#[doc = "Field `SW_INTER_DBLSPEED` writer - inter double speed enable\n\nInter double speed enable"]
pub type SwInterDblspeedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_INTRA_DBLSPEED` reader - intra double speed enable\n\nIntra double speed enable"]
pub type SwIntraDblspeedR = crate::BitReader;
#[doc = "Field `SW_INTRA_DBLSPEED` writer - intra double speed enable\n\nIntra double speed enable"]
pub type SwIntraDblspeedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_INTRA_DBL3T` reader - In chroma dc intra prediction, when this bit is enable, there will 3\n\ncycle enhance for every block"]
pub type SwIntraDbl3tR = crate::BitReader;
#[doc = "Field `SW_INTRA_DBL3T` writer - In chroma dc intra prediction, when this bit is enable, there will 3\n\ncycle enhance for every block"]
pub type SwIntraDbl3tW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_PREF_SIGCHAN` reader - prefetch single channel enable\n\n1'b1: prefetch single channel enable"]
pub type SwPrefSigchanR = crate::BitReader;
#[doc = "Field `SW_PREF_SIGCHAN` writer - prefetch single channel enable\n\n1'b1: prefetch single channel enable"]
pub type SwPrefSigchanW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "cache enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwCacheEn {
    #[doc = "1: cache enable"]
    B1 = 1,
    #[doc = "0: cache disable when sw_cache_en is 1'b1, sw_pref_sigchan should also be 1'b1"]
    B0 = 0,
}
impl From<SwCacheEn> for bool {
    #[inline(always)]
    fn from(variant: SwCacheEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_CACHE_EN` reader - cache enable"]
pub type SwCacheEnR = crate::BitReader<SwCacheEn>;
impl SwCacheEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwCacheEn {
        match self.bits {
            true => SwCacheEn::B1,
            false => SwCacheEn::B0,
        }
    }
    #[doc = "cache enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwCacheEn::B1
    }
    #[doc = "cache disable when sw_cache_en is 1'b1, sw_pref_sigchan should also be 1'b1"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwCacheEn::B0
    }
}
#[doc = "Field `SW_CACHE_EN` writer - cache enable"]
pub type SwCacheEnW<'a, REG> = crate::BitWriter<'a, REG, SwCacheEn>;
impl<'a, REG> SwCacheEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "cache enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwCacheEn::B1)
    }
    #[doc = "cache disable when sw_cache_en is 1'b1, sw_pref_sigchan should also be 1'b1"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwCacheEn::B0)
    }
}
#[doc = "Field `SW_DEC_TIMEOUT_MODE` reader - dec timeout mode selset\n\nwhen 1'b0 , timeout cycle is 181'b1\n\nwhen 1'b1, timeout cycle is 221'b1"]
pub type SwDecTimeoutModeR = crate::BitReader;
#[doc = "Field `SW_DEC_TIMEOUT_MODE` writer - dec timeout mode selset\n\nwhen 1'b0 , timeout cycle is 181'b1\n\nwhen 1'b1, timeout cycle is 221'b1"]
pub type SwDecTimeoutModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enable flag for decoder to start working\n\nhw will auto reset this be after a frame be decodered no matter it\n\nright or have some error"]
    #[inline(always)]
    pub fn sw_dec_st_work(&self) -> SwDecStWorkR {
        SwDecStWorkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable flag for Refer picture buffer 2"]
    #[inline(always)]
    pub fn sw_refpic_buf2_en(&self) -> SwRefpicBuf2EnR {
        SwRefpicBuf2EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - disable flag for wiriting decoder output data to external memory"]
    #[inline(always)]
    pub fn sw_dec_wr_extmem_dis(&self) -> SwDecWrExtmemDisR {
        SwDecWrExtmemDisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - the enable flag for Decoder auto clock gating\n\ndefault hw will reset to 1"]
    #[inline(always)]
    pub fn sw_dec_clkgate_en(&self) -> SwDecClkgateEnR {
        SwDecClkgateEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - the enable flag for Timeout interrupt"]
    #[inline(always)]
    pub fn sw_timeout_sts_en(&self) -> SwTimeoutStsEnR {
        SwTimeoutStsEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - the enable flag for reading Picture order count table\n\nread data from memory used"]
    #[inline(always)]
    pub fn sw_rd_cnt_tab_en(&self) -> SwRdCntTabEnR {
        SwRdCntTabEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - the enable flag for Sequence includes MBAFF coded pictures"]
    #[inline(always)]
    pub fn sw_sequ_mbaff_en(&self) -> SwSequMbaffEnR {
        SwSequMbaffEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - enable flag for FWD reference top field have been decoded first"]
    #[inline(always)]
    pub fn sw_first_reftop_en(&self) -> SwFirstReftopEnR {
        SwFirstReftopEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - enable flag for reference top field\n\n0 = bottom field\n\n1 = top field"]
    #[inline(always)]
    pub fn sw_reftop_en(&self) -> SwReftopEnR {
        SwReftopEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - enable flag for Direct mode motion vector write current picture"]
    #[inline(always)]
    pub fn sw_dmmv_wr_en(&self) -> SwDmmvWrEnR {
        SwDmmvWrEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - enable flag for Sorenson Sparc\n\nvalid when sw_dec_fmt_sel= MPEG- 4\n\n0 = disabled\n\n1 = enable\n\nif enable,will use Sorenson escape coding to compatible stream for\n\nh.263"]
    #[inline(always)]
    pub fn sw_sorspa_en(&self) -> SwSorspaEnR {
        SwSorspaEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - progressive and interlaced for coding mode\n\nused for forward reference picture:"]
    #[inline(always)]
    pub fn sw_fwd_refpic_mode_sel(&self) -> SwFwdRefpicModeSelR {
        SwFwdRefpicModeSelR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - select which field will be decodered"]
    #[inline(always)]
    pub fn sw_pic_decfield_sel(&self) -> SwPicDecfieldSelR {
        SwPicDecfieldSelR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - pic type sel0 flag\n\nshould need sw_pic_type_sel1=0"]
    #[inline(always)]
    pub fn sw_pic_type_sel0(&self) -> SwPicTypeSel0R {
        SwPicTypeSel0R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - pic type sel1 flag"]
    #[inline(always)]
    pub fn sw_pic_type_sel1(&self) -> SwPicTypeSel1R {
        SwPicTypeSel1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - the current picture Structure selected"]
    #[inline(always)]
    pub fn sw_curpic_stru_sel(&self) -> SwCurpicStruSelR {
        SwCurpicStruSelR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - the current picture coding mode select"]
    #[inline(always)]
    pub fn sw_curpic_code_sel(&self) -> SwCurpicCodeSelR {
        SwCurpicCodeSelR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - enable flag for Progressive JPEG"]
    #[inline(always)]
    pub fn sw_prog_jpeg_en(&self) -> SwProgJpegEnR {
        SwProgJpegEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - enable for RLC mode"]
    #[inline(always)]
    pub fn sw_rlc_mode_en(&self) -> SwRlcModeEnR {
        SwRlcModeEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable for additional chrominance data format writing\n\ntiled mode should be disable,when this bit be used\n\ndecoder writes chrominance: group of 8 pixels of Cb then\n\ncorresponding 8 pixels of Cr\n\nData is written to sw_dec_ch8pix_st_adr."]
    #[inline(always)]
    pub fn sw_addit_ch_fmt_wen(&self) -> SwAdditChFmtWenR {
        SwAdditChFmtWenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - existence flag for stream start code"]
    #[inline(always)]
    pub fn sw_st_code_exist(&self) -> SwStCodeExistR {
        SwStCodeExistR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 25 - inter double speed enable\n\nInter double speed enable"]
    #[inline(always)]
    pub fn sw_inter_dblspeed(&self) -> SwInterDblspeedR {
        SwInterDblspeedR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - intra double speed enable\n\nIntra double speed enable"]
    #[inline(always)]
    pub fn sw_intra_dblspeed(&self) -> SwIntraDblspeedR {
        SwIntraDblspeedR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - In chroma dc intra prediction, when this bit is enable, there will 3\n\ncycle enhance for every block"]
    #[inline(always)]
    pub fn sw_intra_dbl3t(&self) -> SwIntraDbl3tR {
        SwIntraDbl3tR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - prefetch single channel enable\n\n1'b1: prefetch single channel enable"]
    #[inline(always)]
    pub fn sw_pref_sigchan(&self) -> SwPrefSigchanR {
        SwPrefSigchanR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - cache enable"]
    #[inline(always)]
    pub fn sw_cache_en(&self) -> SwCacheEnR {
        SwCacheEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - dec timeout mode selset\n\nwhen 1'b0 , timeout cycle is 181'b1\n\nwhen 1'b1, timeout cycle is 221'b1"]
    #[inline(always)]
    pub fn sw_dec_timeout_mode(&self) -> SwDecTimeoutModeR {
        SwDecTimeoutModeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable flag for decoder to start working\n\nhw will auto reset this be after a frame be decodered no matter it\n\nright or have some error"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dec_st_work(&mut self) -> SwDecStWorkW<Swreg57Spec> {
        SwDecStWorkW::new(self, 0)
    }
    #[doc = "Bit 1 - enable flag for Refer picture buffer 2"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refpic_buf2_en(&mut self) -> SwRefpicBuf2EnW<Swreg57Spec> {
        SwRefpicBuf2EnW::new(self, 1)
    }
    #[doc = "Bit 2 - disable flag for wiriting decoder output data to external memory"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dec_wr_extmem_dis(&mut self) -> SwDecWrExtmemDisW<Swreg57Spec> {
        SwDecWrExtmemDisW::new(self, 2)
    }
    #[doc = "Bit 4 - the enable flag for Decoder auto clock gating\n\ndefault hw will reset to 1"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dec_clkgate_en(&mut self) -> SwDecClkgateEnW<Swreg57Spec> {
        SwDecClkgateEnW::new(self, 4)
    }
    #[doc = "Bit 5 - the enable flag for Timeout interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn sw_timeout_sts_en(&mut self) -> SwTimeoutStsEnW<Swreg57Spec> {
        SwTimeoutStsEnW::new(self, 5)
    }
    #[doc = "Bit 6 - the enable flag for reading Picture order count table\n\nread data from memory used"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rd_cnt_tab_en(&mut self) -> SwRdCntTabEnW<Swreg57Spec> {
        SwRdCntTabEnW::new(self, 6)
    }
    #[doc = "Bit 7 - the enable flag for Sequence includes MBAFF coded pictures"]
    #[inline(always)]
    #[must_use]
    pub fn sw_sequ_mbaff_en(&mut self) -> SwSequMbaffEnW<Swreg57Spec> {
        SwSequMbaffEnW::new(self, 7)
    }
    #[doc = "Bit 8 - enable flag for FWD reference top field have been decoded first"]
    #[inline(always)]
    #[must_use]
    pub fn sw_first_reftop_en(&mut self) -> SwFirstReftopEnW<Swreg57Spec> {
        SwFirstReftopEnW::new(self, 8)
    }
    #[doc = "Bit 9 - enable flag for reference top field\n\n0 = bottom field\n\n1 = top field"]
    #[inline(always)]
    #[must_use]
    pub fn sw_reftop_en(&mut self) -> SwReftopEnW<Swreg57Spec> {
        SwReftopEnW::new(self, 9)
    }
    #[doc = "Bit 10 - enable flag for Direct mode motion vector write current picture"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dmmv_wr_en(&mut self) -> SwDmmvWrEnW<Swreg57Spec> {
        SwDmmvWrEnW::new(self, 10)
    }
    #[doc = "Bit 11 - enable flag for Sorenson Sparc\n\nvalid when sw_dec_fmt_sel= MPEG- 4\n\n0 = disabled\n\n1 = enable\n\nif enable,will use Sorenson escape coding to compatible stream for\n\nh.263"]
    #[inline(always)]
    #[must_use]
    pub fn sw_sorspa_en(&mut self) -> SwSorspaEnW<Swreg57Spec> {
        SwSorspaEnW::new(self, 11)
    }
    #[doc = "Bit 12 - progressive and interlaced for coding mode\n\nused for forward reference picture:"]
    #[inline(always)]
    #[must_use]
    pub fn sw_fwd_refpic_mode_sel(&mut self) -> SwFwdRefpicModeSelW<Swreg57Spec> {
        SwFwdRefpicModeSelW::new(self, 12)
    }
    #[doc = "Bit 13 - select which field will be decodered"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pic_decfield_sel(&mut self) -> SwPicDecfieldSelW<Swreg57Spec> {
        SwPicDecfieldSelW::new(self, 13)
    }
    #[doc = "Bit 14 - pic type sel0 flag\n\nshould need sw_pic_type_sel1=0"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pic_type_sel0(&mut self) -> SwPicTypeSel0W<Swreg57Spec> {
        SwPicTypeSel0W::new(self, 14)
    }
    #[doc = "Bit 15 - pic type sel1 flag"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pic_type_sel1(&mut self) -> SwPicTypeSel1W<Swreg57Spec> {
        SwPicTypeSel1W::new(self, 15)
    }
    #[doc = "Bit 16 - the current picture Structure selected"]
    #[inline(always)]
    #[must_use]
    pub fn sw_curpic_stru_sel(&mut self) -> SwCurpicStruSelW<Swreg57Spec> {
        SwCurpicStruSelW::new(self, 16)
    }
    #[doc = "Bit 17 - the current picture coding mode select"]
    #[inline(always)]
    #[must_use]
    pub fn sw_curpic_code_sel(&mut self) -> SwCurpicCodeSelW<Swreg57Spec> {
        SwCurpicCodeSelW::new(self, 17)
    }
    #[doc = "Bit 18 - enable flag for Progressive JPEG"]
    #[inline(always)]
    #[must_use]
    pub fn sw_prog_jpeg_en(&mut self) -> SwProgJpegEnW<Swreg57Spec> {
        SwProgJpegEnW::new(self, 18)
    }
    #[doc = "Bit 20 - enable for RLC mode"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rlc_mode_en(&mut self) -> SwRlcModeEnW<Swreg57Spec> {
        SwRlcModeEnW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable for additional chrominance data format writing\n\ntiled mode should be disable,when this bit be used\n\ndecoder writes chrominance: group of 8 pixels of Cb then\n\ncorresponding 8 pixels of Cr\n\nData is written to sw_dec_ch8pix_st_adr."]
    #[inline(always)]
    #[must_use]
    pub fn sw_addit_ch_fmt_wen(&mut self) -> SwAdditChFmtWenW<Swreg57Spec> {
        SwAdditChFmtWenW::new(self, 21)
    }
    #[doc = "Bit 22 - existence flag for stream start code"]
    #[inline(always)]
    #[must_use]
    pub fn sw_st_code_exist(&mut self) -> SwStCodeExistW<Swreg57Spec> {
        SwStCodeExistW::new(self, 22)
    }
    #[doc = "Bit 25 - inter double speed enable\n\nInter double speed enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw_inter_dblspeed(&mut self) -> SwInterDblspeedW<Swreg57Spec> {
        SwInterDblspeedW::new(self, 25)
    }
    #[doc = "Bit 26 - intra double speed enable\n\nIntra double speed enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw_intra_dblspeed(&mut self) -> SwIntraDblspeedW<Swreg57Spec> {
        SwIntraDblspeedW::new(self, 26)
    }
    #[doc = "Bit 27 - In chroma dc intra prediction, when this bit is enable, there will 3\n\ncycle enhance for every block"]
    #[inline(always)]
    #[must_use]
    pub fn sw_intra_dbl3t(&mut self) -> SwIntraDbl3tW<Swreg57Spec> {
        SwIntraDbl3tW::new(self, 27)
    }
    #[doc = "Bit 28 - prefetch single channel enable\n\n1'b1: prefetch single channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pref_sigchan(&mut self) -> SwPrefSigchanW<Swreg57Spec> {
        SwPrefSigchanW::new(self, 28)
    }
    #[doc = "Bit 29 - cache enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw_cache_en(&mut self) -> SwCacheEnW<Swreg57Spec> {
        SwCacheEnW::new(self, 29)
    }
    #[doc = "Bit 31 - dec timeout mode selset\n\nwhen 1'b0 , timeout cycle is 181'b1\n\nwhen 1'b1, timeout cycle is 221'b1"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dec_timeout_mode(&mut self) -> SwDecTimeoutModeW<Swreg57Spec> {
        SwDecTimeoutModeW::new(self, 31)
    }
}
#[doc = "enable flag for decoder\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg57::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg57::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg57Spec;
impl crate::RegisterSpec for Swreg57Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg57::R`](R) reader structure"]
impl crate::Readable for Swreg57Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg57::W`](W) writer structure"]
impl crate::Writable for Swreg57Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG57 to value 0x10"]
impl crate::Resettable for Swreg57Spec {
    const RESET_VALUE: u32 = 0x10;
}
