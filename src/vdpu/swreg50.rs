#[doc = "Register `SWREG50` reader"]
pub type R = crate::R<Swreg50Spec>;
#[doc = "Register `SWREG50` writer"]
pub type W = crate::W<Swreg50Spec>;
#[doc = "the enable for msb tiled mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwDecTiledMsb {
    #[doc = "0: Tiled mode disable"]
    B0 = 0,
    #[doc = "1: Tiled mode enabled for 8x4 tile size"]
    B1 = 1,
}
impl From<SwDecTiledMsb> for bool {
    #[inline(always)]
    fn from(variant: SwDecTiledMsb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_DEC_TILED_MSB` reader - the enable for msb tiled mode"]
pub type SwDecTiledMsbR = crate::BitReader<SwDecTiledMsb>;
impl SwDecTiledMsbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwDecTiledMsb {
        match self.bits {
            false => SwDecTiledMsb::B0,
            true => SwDecTiledMsb::B1,
        }
    }
    #[doc = "Tiled mode disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwDecTiledMsb::B0
    }
    #[doc = "Tiled mode enabled for 8x4 tile size"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwDecTiledMsb::B1
    }
}
#[doc = "Field `SW_DEC_TILED_MSB` writer - the enable for msb tiled mode"]
pub type SwDecTiledMsbW<'a, REG> = crate::BitWriter<'a, REG, SwDecTiledMsb>;
impl<'a, REG> SwDecTiledMsbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tiled mode disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwDecTiledMsb::B0)
    }
    #[doc = "Tiled mode enabled for 8x4 tile size"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwDecTiledMsb::B1)
    }
}
#[doc = "Field `SW_ADTION_LATENCY` reader - the additional latency for decoder master interface\n\nCan be used to slow down 8*sw_dec_latency cycles of IDLE\n\nbetween services,so if sw_dec_latency =0,that is no latency"]
pub type SwAdtionLatencyR = crate::FieldReader;
#[doc = "Field `SW_ADTION_LATENCY` writer - the additional latency for decoder master interface\n\nCan be used to slow down 8*sw_dec_latency cycles of IDLE\n\nbetween services,so if sw_dec_latency =0,that is no latency"]
pub type SwAdtionLatencyW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "h.264:\n\nthis bit is for the enable of multi view coding\n\nother format\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwDecFixedQuant {
    #[doc = "0: it can be different inside pic for Quantization parameter"]
    B0 = 0,
    #[doc = "1: it is fixed for Quantization parameter"]
    B1 = 1,
}
impl From<SwDecFixedQuant> for bool {
    #[inline(always)]
    fn from(variant: SwDecFixedQuant) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_DEC_FIXED_QUANT` reader - h.264:\n\nthis bit is for the enable of multi view coding\n\nother format"]
pub type SwDecFixedQuantR = crate::BitReader<SwDecFixedQuant>;
impl SwDecFixedQuantR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwDecFixedQuant {
        match self.bits {
            false => SwDecFixedQuant::B0,
            true => SwDecFixedQuant::B1,
        }
    }
    #[doc = "it can be different inside pic for Quantization parameter"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwDecFixedQuant::B0
    }
    #[doc = "it is fixed for Quantization parameter"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwDecFixedQuant::B1
    }
}
#[doc = "Field `SW_DEC_FIXED_QUANT` writer - h.264:\n\nthis bit is for the enable of multi view coding\n\nother format"]
pub type SwDecFixedQuantW<'a, REG> = crate::BitWriter<'a, REG, SwDecFixedQuant>;
impl<'a, REG> SwDecFixedQuantW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "it can be different inside pic for Quantization parameter"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwDecFixedQuant::B0)
    }
    #[doc = "it is fixed for Quantization parameter"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwDecFixedQuant::B1)
    }
}
#[doc = "the disable for current pic deblock filtering\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwDblkFltDis {
    #[doc = "1: disable"]
    B1 = 1,
    #[doc = "0: enable"]
    B0 = 0,
}
impl From<SwDblkFltDis> for bool {
    #[inline(always)]
    fn from(variant: SwDblkFltDis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_DBLK_FLT_DIS` reader - the disable for current pic deblock filtering"]
pub type SwDblkFltDisR = crate::BitReader<SwDblkFltDis>;
impl SwDblkFltDisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwDblkFltDis {
        match self.bits {
            true => SwDblkFltDis::B1,
            false => SwDblkFltDis::B0,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwDblkFltDis::B1
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwDblkFltDis::B0
    }
}
#[doc = "Field `SW_DBLK_FLT_DIS` writer - the disable for current pic deblock filtering"]
pub type SwDblkFltDisW<'a, REG> = crate::BitWriter<'a, REG, SwDblkFltDis>;
impl<'a, REG> SwDblkFltDisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwDblkFltDis::B1)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwDblkFltDis::B0)
    }
}
#[doc = "AVS format:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwSkipSel {
    #[doc = "0: skip mbs use special MB type"]
    B0 = 0,
    #[doc = "1: avs skip mbs have the same skip run syntax element as h264"]
    B1 = 1,
}
impl From<SwSkipSel> for bool {
    #[inline(always)]
    fn from(variant: SwSkipSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_SKIP_SEL` reader - AVS format:"]
pub type SwSkipSelR = crate::BitReader<SwSkipSel>;
impl SwSkipSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwSkipSel {
        match self.bits {
            false => SwSkipSel::B0,
            true => SwSkipSel::B1,
        }
    }
    #[doc = "skip mbs use special MB type"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwSkipSel::B0
    }
    #[doc = "avs skip mbs have the same skip run syntax element as h264"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwSkipSel::B1
    }
}
#[doc = "Field `SW_SKIP_SEL` writer - AVS format:"]
pub type SwSkipSelW<'a, REG> = crate::BitWriter<'a, REG, SwSkipSel>;
impl<'a, REG> SwSkipSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "skip mbs use special MB type"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwSkipSel::B0)
    }
    #[doc = "avs skip mbs have the same skip run syntax element as h264"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwSkipSel::B1)
    }
}
#[doc = "Field `SW_DEC_ASCMD0_DIS` reader - the disable for AXI Single Command Multiple Data0"]
pub type SwDecAscmd0DisR = crate::BitReader;
#[doc = "Field `SW_DEC_ASCMD0_DIS` writer - the disable for AXI Single Command Multiple Data0"]
pub type SwDecAscmd0DisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_ADV_PREF_DIS` reader - disable for Advanced PREFETCH mode"]
pub type SwAdvPrefDisR = crate::BitReader;
#[doc = "Field `SW_ADV_PREF_DIS` writer - disable for Advanced PREFETCH mode"]
pub type SwAdvPrefDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "the enable for lsb tiled mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwDecTiledLsb {
    #[doc = "0: Tiled mode disable"]
    B0 = 0,
    #[doc = "1: Tiled mode enabled for 8x4 tile size"]
    B1 = 1,
}
impl From<SwDecTiledLsb> for bool {
    #[inline(always)]
    fn from(variant: SwDecTiledLsb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_DEC_TILED_LSB` reader - the enable for lsb tiled mode"]
pub type SwDecTiledLsbR = crate::BitReader<SwDecTiledLsb>;
impl SwDecTiledLsbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwDecTiledLsb {
        match self.bits {
            false => SwDecTiledLsb::B0,
            true => SwDecTiledLsb::B1,
        }
    }
    #[doc = "Tiled mode disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwDecTiledLsb::B0
    }
    #[doc = "Tiled mode enabled for 8x4 tile size"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwDecTiledLsb::B1
    }
}
#[doc = "Field `SW_DEC_TILED_LSB` writer - the enable for lsb tiled mode"]
pub type SwDecTiledLsbW<'a, REG> = crate::BitWriter<'a, REG, SwDecTiledLsb>;
impl<'a, REG> SwDecTiledLsbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tiled mode disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwDecTiledLsb::B0)
    }
    #[doc = "Tiled mode enabled for 8x4 tile size"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwDecTiledLsb::B1)
    }
}
#[doc = "Field `SW_REFBUF_THRD` reader - Reference buffer threshold value\n\nUsed shut down buffer"]
pub type SwRefbufThrdR = crate::FieldReader<u16>;
#[doc = "Field `SW_REFBUF_THRD` writer - Reference buffer threshold value\n\nUsed shut down buffer"]
pub type SwRefbufThrdW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SW_REFBUF_PID` reader - the pic id for reference buffer\n\nThe used reference picture ID for reference buffer usage"]
pub type SwRefbufPidR = crate::FieldReader;
#[doc = "Field `SW_REFBUF_PID` writer - the pic id for reference buffer\n\nThe used reference picture ID for reference buffer usage"]
pub type SwRefbufPidW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - the enable for msb tiled mode"]
    #[inline(always)]
    pub fn sw_dec_tiled_msb(&self) -> SwDecTiledMsbR {
        SwDecTiledMsbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:6 - the additional latency for decoder master interface\n\nCan be used to slow down 8*sw_dec_latency cycles of IDLE\n\nbetween services,so if sw_dec_latency =0,that is no latency"]
    #[inline(always)]
    pub fn sw_adtion_latency(&self) -> SwAdtionLatencyR {
        SwAdtionLatencyR::new(((self.bits >> 1) & 0x3f) as u8)
    }
    #[doc = "Bit 7 - h.264:\n\nthis bit is for the enable of multi view coding\n\nother format"]
    #[inline(always)]
    pub fn sw_dec_fixed_quant(&self) -> SwDecFixedQuantR {
        SwDecFixedQuantR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - the disable for current pic deblock filtering"]
    #[inline(always)]
    pub fn sw_dblk_flt_dis(&self) -> SwDblkFltDisR {
        SwDblkFltDisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AVS format:"]
    #[inline(always)]
    pub fn sw_skip_sel(&self) -> SwSkipSelR {
        SwSkipSelR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - the disable for AXI Single Command Multiple Data0"]
    #[inline(always)]
    pub fn sw_dec_ascmd0_dis(&self) -> SwDecAscmd0DisR {
        SwDecAscmd0DisR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - disable for Advanced PREFETCH mode"]
    #[inline(always)]
    pub fn sw_adv_pref_dis(&self) -> SwAdvPrefDisR {
        SwAdvPrefDisR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - the enable for lsb tiled mode"]
    #[inline(always)]
    pub fn sw_dec_tiled_lsb(&self) -> SwDecTiledLsbR {
        SwDecTiledLsbR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:24 - Reference buffer threshold value\n\nUsed shut down buffer"]
    #[inline(always)]
    pub fn sw_refbuf_thrd(&self) -> SwRefbufThrdR {
        SwRefbufThrdR::new(((self.bits >> 13) & 0x0fff) as u16)
    }
    #[doc = "Bits 25:29 - the pic id for reference buffer\n\nThe used reference picture ID for reference buffer usage"]
    #[inline(always)]
    pub fn sw_refbuf_pid(&self) -> SwRefbufPidR {
        SwRefbufPidR::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - the enable for msb tiled mode"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dec_tiled_msb(&mut self) -> SwDecTiledMsbW<Swreg50Spec> {
        SwDecTiledMsbW::new(self, 0)
    }
    #[doc = "Bits 1:6 - the additional latency for decoder master interface\n\nCan be used to slow down 8*sw_dec_latency cycles of IDLE\n\nbetween services,so if sw_dec_latency =0,that is no latency"]
    #[inline(always)]
    #[must_use]
    pub fn sw_adtion_latency(&mut self) -> SwAdtionLatencyW<Swreg50Spec> {
        SwAdtionLatencyW::new(self, 1)
    }
    #[doc = "Bit 7 - h.264:\n\nthis bit is for the enable of multi view coding\n\nother format"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dec_fixed_quant(&mut self) -> SwDecFixedQuantW<Swreg50Spec> {
        SwDecFixedQuantW::new(self, 7)
    }
    #[doc = "Bit 8 - the disable for current pic deblock filtering"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dblk_flt_dis(&mut self) -> SwDblkFltDisW<Swreg50Spec> {
        SwDblkFltDisW::new(self, 8)
    }
    #[doc = "Bit 9 - AVS format:"]
    #[inline(always)]
    #[must_use]
    pub fn sw_skip_sel(&mut self) -> SwSkipSelW<Swreg50Spec> {
        SwSkipSelW::new(self, 9)
    }
    #[doc = "Bit 10 - the disable for AXI Single Command Multiple Data0"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dec_ascmd0_dis(&mut self) -> SwDecAscmd0DisW<Swreg50Spec> {
        SwDecAscmd0DisW::new(self, 10)
    }
    #[doc = "Bit 11 - disable for Advanced PREFETCH mode"]
    #[inline(always)]
    #[must_use]
    pub fn sw_adv_pref_dis(&mut self) -> SwAdvPrefDisW<Swreg50Spec> {
        SwAdvPrefDisW::new(self, 11)
    }
    #[doc = "Bit 12 - the enable for lsb tiled mode"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dec_tiled_lsb(&mut self) -> SwDecTiledLsbW<Swreg50Spec> {
        SwDecTiledLsbW::new(self, 12)
    }
    #[doc = "Bits 13:24 - Reference buffer threshold value\n\nUsed shut down buffer"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refbuf_thrd(&mut self) -> SwRefbufThrdW<Swreg50Spec> {
        SwRefbufThrdW::new(self, 13)
    }
    #[doc = "Bits 25:29 - the pic id for reference buffer\n\nThe used reference picture ID for reference buffer usage"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refbuf_pid(&mut self) -> SwRefbufPidW<Swreg50Spec> {
        SwRefbufPidW::new(self, 25)
    }
}
#[doc = "video decoder ctrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg50::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg50::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg50Spec;
impl crate::RegisterSpec for Swreg50Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg50::R`](R) reader structure"]
impl crate::Readable for Swreg50Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg50::W`](W) writer structure"]
impl crate::Writable for Swreg50Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG50 to value 0"]
impl crate::Resettable for Swreg50Spec {
    const RESET_VALUE: u32 = 0;
}
