#[doc = "Register `SWREG_59` reader"]
pub type R = crate::R<Swreg59Spec>;
#[doc = "Register `SWREG_59` writer"]
pub type W = crate::W<Swreg59Spec>;
#[doc = "Field `H264_SLICE_NUM` reader - the h264 slice numeber in on picture\n\n0=one slice in currect picture\n\n1=two slice in currect picture\n\n......"]
pub type H264SliceNumR = crate::FieldReader;
#[doc = "Field `H264_SLICE_NUM` writer - the h264 slice numeber in on picture\n\n0=one slice in currect picture\n\n1=two slice in currect picture\n\n......"]
pub type H264SliceNumW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "used to select stream mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum H264StrmModSel {
    #[doc = "0: NAL unit ;"]
    B0 = 0,
    #[doc = "1: BYTE"]
    B1 = 1,
}
impl From<H264StrmModSel> for bool {
    #[inline(always)]
    fn from(variant: H264StrmModSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `H264_STRM_MOD_SEL` reader - used to select stream mode"]
pub type H264StrmModSelR = crate::BitReader<H264StrmModSel>;
impl H264StrmModSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> H264StrmModSel {
        match self.bits {
            false => H264StrmModSel::B0,
            true => H264StrmModSel::B1,
        }
    }
    #[doc = "NAL unit ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == H264StrmModSel::B0
    }
    #[doc = "BYTE"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == H264StrmModSel::B1
    }
}
#[doc = "Field `H264_STRM_MOD_SEL` writer - used to select stream mode"]
pub type H264StrmModSelW<'a, REG> = crate::BitWriter<'a, REG, H264StrmModSel>;
impl<'a, REG> H264StrmModSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NAL unit ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(H264StrmModSel::B0)
    }
    #[doc = "BYTE"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(H264StrmModSel::B1)
    }
}
#[doc = "Field `H264_RES_INTERMOD_4X4` reader - the restriction inter mode selected in 4x4 block\n\nthe restriction inter mode selected in 4x4 block"]
pub type H264ResIntermod4x4R = crate::BitReader;
#[doc = "Field `H264_RES_INTERMOD_4X4` writer - the restriction inter mode selected in 4x4 block\n\nthe restriction inter mode selected in 4x4 block"]
pub type H264ResIntermod4x4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H264_TRFMOD_8X8` reader - on-off for 8x8 transform used in h264\n\non-off for 8x8 transform used in h264"]
pub type H264Trfmod8x8R = crate::BitReader;
#[doc = "Field `H264_TRFMOD_8X8` writer - on-off for 8x8 transform used in h264\n\non-off for 8x8 transform used in h264"]
pub type H264Trfmod8x8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "the format of stream entropy coding\n\nh.264:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntryCodeFmt {
    #[doc = "0: cavlc"]
    B0 = 0,
    #[doc = "1: cabac"]
    B1 = 1,
}
impl From<EntryCodeFmt> for bool {
    #[inline(always)]
    fn from(variant: EntryCodeFmt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENTRY_CODE_FMT` reader - the format of stream entropy coding\n\nh.264:"]
pub type EntryCodeFmtR = crate::BitReader<EntryCodeFmt>;
impl EntryCodeFmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EntryCodeFmt {
        match self.bits {
            false => EntryCodeFmt::B0,
            true => EntryCodeFmt::B1,
        }
    }
    #[doc = "cavlc"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == EntryCodeFmt::B0
    }
    #[doc = "cabac"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == EntryCodeFmt::B1
    }
}
#[doc = "Field `ENTRY_CODE_FMT` writer - the format of stream entropy coding\n\nh.264:"]
pub type EntryCodeFmtW<'a, REG> = crate::BitWriter<'a, REG, EntryCodeFmt>;
impl<'a, REG> EntryCodeFmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "cavlc"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(EntryCodeFmt::B0)
    }
    #[doc = "cabac"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(EntryCodeFmt::B1)
    }
}
#[doc = "Field `H264_CABAC_IDC` reader - the initial idc for cabac used in h264\n\n0,1,2: used\n\n3:no use"]
pub type H264CabacIdcR = crate::FieldReader;
#[doc = "Field `H264_CABAC_IDC` writer - the initial idc for cabac used in h264\n\n0,1,2: used\n\n3:no use"]
pub type H264CabacIdcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "deblocking filter mode select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DblkingFltMode {
    #[doc = "0: enabled"]
    D0 = 0,
    #[doc = "1: disabled"]
    D1 = 1,
    #[doc = "2: disabled on slice"]
    D2 = 2,
}
impl From<DblkingFltMode> for u8 {
    #[inline(always)]
    fn from(variant: DblkingFltMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DblkingFltMode {
    type Ux = u8;
}
#[doc = "Field `DBLKING_FLT_MODE` reader - deblocking filter mode select"]
pub type DblkingFltModeR = crate::FieldReader<DblkingFltMode>;
impl DblkingFltModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DblkingFltMode> {
        match self.bits {
            0 => Some(DblkingFltMode::D0),
            1 => Some(DblkingFltMode::D1),
            2 => Some(DblkingFltMode::D2),
            _ => None,
        }
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == DblkingFltMode::D0
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == DblkingFltMode::D1
    }
    #[doc = "disabled on slice"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == DblkingFltMode::D2
    }
}
#[doc = "Field `DBLKING_FLT_MODE` writer - deblocking filter mode select"]
pub type DblkingFltModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, DblkingFltMode>;
impl<'a, REG> DblkingFltModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "enabled"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(DblkingFltMode::D0)
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(DblkingFltMode::D1)
    }
    #[doc = "disabled on slice"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(DblkingFltMode::D2)
    }
}
#[doc = "disable the function of quarter pixel MVs used in h264\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum H264QurtPixmvDis {
    #[doc = "1: disable"]
    B1 = 1,
    #[doc = "0: default,enable"]
    B0 = 0,
}
impl From<H264QurtPixmvDis> for bool {
    #[inline(always)]
    fn from(variant: H264QurtPixmvDis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `H264_QURT_PIXMV_DIS` reader - disable the function of quarter pixel MVs used in h264"]
pub type H264QurtPixmvDisR = crate::BitReader<H264QurtPixmvDis>;
impl H264QurtPixmvDisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> H264QurtPixmvDis {
        match self.bits {
            true => H264QurtPixmvDis::B1,
            false => H264QurtPixmvDis::B0,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == H264QurtPixmvDis::B1
    }
    #[doc = "default,enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == H264QurtPixmvDis::B0
    }
}
#[doc = "Field `H264_QURT_PIXMV_DIS` writer - disable the function of quarter pixel MVs used in h264"]
pub type H264QurtPixmvDisW<'a, REG> = crate::BitWriter<'a, REG, H264QurtPixmvDis>;
impl<'a, REG> H264QurtPixmvDisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(H264QurtPixmvDis::B1)
    }
    #[doc = "default,enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(H264QurtPixmvDis::B0)
    }
}
impl R {
    #[doc = "Bits 8:14 - the h264 slice numeber in on picture\n\n0=one slice in currect picture\n\n1=two slice in currect picture\n\n......"]
    #[inline(always)]
    pub fn h264_slice_num(&self) -> H264SliceNumR {
        H264SliceNumR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - used to select stream mode"]
    #[inline(always)]
    pub fn h264_strm_mod_sel(&self) -> H264StrmModSelR {
        H264StrmModSelR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - the restriction inter mode selected in 4x4 block\n\nthe restriction inter mode selected in 4x4 block"]
    #[inline(always)]
    pub fn h264_res_intermod_4x4(&self) -> H264ResIntermod4x4R {
        H264ResIntermod4x4R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - on-off for 8x8 transform used in h264\n\non-off for 8x8 transform used in h264"]
    #[inline(always)]
    pub fn h264_trfmod_8x8(&self) -> H264Trfmod8x8R {
        H264Trfmod8x8R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - the format of stream entropy coding\n\nh.264:"]
    #[inline(always)]
    pub fn entry_code_fmt(&self) -> EntryCodeFmtR {
        EntryCodeFmtR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - the initial idc for cabac used in h264\n\n0,1,2: used\n\n3:no use"]
    #[inline(always)]
    pub fn h264_cabac_idc(&self) -> H264CabacIdcR {
        H264CabacIdcR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 24:25 - deblocking filter mode select"]
    #[inline(always)]
    pub fn dblking_flt_mode(&self) -> DblkingFltModeR {
        DblkingFltModeR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28 - disable the function of quarter pixel MVs used in h264"]
    #[inline(always)]
    pub fn h264_qurt_pixmv_dis(&self) -> H264QurtPixmvDisR {
        H264QurtPixmvDisR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:14 - the h264 slice numeber in on picture\n\n0=one slice in currect picture\n\n1=two slice in currect picture\n\n......"]
    #[inline(always)]
    #[must_use]
    pub fn h264_slice_num(&mut self) -> H264SliceNumW<Swreg59Spec> {
        H264SliceNumW::new(self, 8)
    }
    #[doc = "Bit 15 - used to select stream mode"]
    #[inline(always)]
    #[must_use]
    pub fn h264_strm_mod_sel(&mut self) -> H264StrmModSelW<Swreg59Spec> {
        H264StrmModSelW::new(self, 15)
    }
    #[doc = "Bit 16 - the restriction inter mode selected in 4x4 block\n\nthe restriction inter mode selected in 4x4 block"]
    #[inline(always)]
    #[must_use]
    pub fn h264_res_intermod_4x4(&mut self) -> H264ResIntermod4x4W<Swreg59Spec> {
        H264ResIntermod4x4W::new(self, 16)
    }
    #[doc = "Bit 17 - on-off for 8x8 transform used in h264\n\non-off for 8x8 transform used in h264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_trfmod_8x8(&mut self) -> H264Trfmod8x8W<Swreg59Spec> {
        H264Trfmod8x8W::new(self, 17)
    }
    #[doc = "Bit 20 - the format of stream entropy coding\n\nh.264:"]
    #[inline(always)]
    #[must_use]
    pub fn entry_code_fmt(&mut self) -> EntryCodeFmtW<Swreg59Spec> {
        EntryCodeFmtW::new(self, 20)
    }
    #[doc = "Bits 21:22 - the initial idc for cabac used in h264\n\n0,1,2: used\n\n3:no use"]
    #[inline(always)]
    #[must_use]
    pub fn h264_cabac_idc(&mut self) -> H264CabacIdcW<Swreg59Spec> {
        H264CabacIdcW::new(self, 21)
    }
    #[doc = "Bits 24:25 - deblocking filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn dblking_flt_mode(&mut self) -> DblkingFltModeW<Swreg59Spec> {
        DblkingFltModeW::new(self, 24)
    }
    #[doc = "Bit 28 - disable the function of quarter pixel MVs used in h264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_qurt_pixmv_dis(&mut self) -> H264QurtPixmvDisW<Swreg59Spec> {
        H264QurtPixmvDisW::new(self, 28)
    }
}
#[doc = "Register0000 Abstract\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_59::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_59::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg59Spec;
impl crate::RegisterSpec for Swreg59Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_59::R`](R) reader structure"]
impl crate::Readable for Swreg59Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_59::W`](W) writer structure"]
impl crate::Writable for Swreg59Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_59 to value 0"]
impl crate::Resettable for Swreg59Spec {
    const RESET_VALUE: u32 = 0;
}
